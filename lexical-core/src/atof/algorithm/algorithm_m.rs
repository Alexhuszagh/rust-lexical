//! Algorithm M creates an exact representation from the digits of a float.
//!
//! Although this seems superficially similar to libcore's algorithm M, since
//! it represents the numerator and, if applicable, the denominator
//! exactly, it has a few significant modifications for speed.
//!
//! 1. It estimates, to within 1 bit, the size of the quotient, requiring
//!     only 2 big divisions at maximum.
//! 2. It uses a very fast division algorithm when the size of the two
//!     buffers is similar, meaning we only do 1-2 native divisions using
//!     Knuth's algorithm D (and ~N multiplications).
//!
//! That said, there's still room for optimization. According to
//! [Rick Regan](https://www.exploringbinary.com/how-glibc-strtod-works/),
//! glibc's strtod only requires 1-2 native integer divisions, and likely
//! few multiplications, and is still able to correctly round without
//! a remainder or other. I can estimate a quotient, very easily,
//! using the hi128 bits of the numerator and hi64 bits of the denominator,
//! giving me a mantissa within 1 bit of the actual value. However,
//! calculating the exact quotient, required for the mantissa, and for
//! the remainder, is a bit trickier, without using a full division
//! algorithm.

use lib::{cmp, iter};
use float::*;
use float::convert::*;
use util::*;
use super::bigcomp;
use super::bigint::*;
use super::math::*;

/// Scale the ratio by powers of 2 until the num/den will be set properly.
///
/// Returns the estimated binary exponent.
///
/// We want the returned quotient to have [64, 65) bits, IE, the bit
/// length of the numerator - denominator is 64.
unsafe fn scale_ratio(num: &mut Bigint, den: &mut Bigint, mantissa_size: i32)
    -> i32
{
    // We to scale the numerator and the denominator so there are
    // MANTISSA_SIZE extra bits in the numerator, so we can at
    // maximum overestimate the denominator by 1. This will mean
    // the resulting quotient will have `MANTISSA_SIZE` or
    // `MANTISSA_SIZE+1` bits. This is fine, since having 1 extra bit
    // is easy to deal with, especially with the remainder.
    let exp = num.bit_length().as_i32() - den.bit_length().as_i32();
    let shift = mantissa_size - exp;
    let nlz = den.leading_zeros();
    if shift > 0 {
        // Need to shift the numerator left, adjust the binary exponent.
        let shift = shift.as_usize();
        let s2 = nlz;
        let s1 = shift + s2;
        num.ishl(s1);
        den.ishl(s2);
    } else if shift < 0 {
        // Need to shift the denominator left, and ensure it's divisible by Limb::BITS.
        let shift = (-shift).as_usize();
        let s2 = nlz + (shift % Limb::BITS);
        let s1 = shift + s2;
        num.ishl(s2);
        den.ishl(s1)
    }

    exp - mantissa_size
}

/// Calculate the mantissa for a big integer with a negative exponent.
///
/// This invokes Algorithm M.
#[inline]
pub(super) unsafe fn negative_exponent_atof<F, Iter>(digits: Iter, radix: u32, max_digits: usize, exponent: i32)
    -> F
    where F: FloatRounding<u64>,
          F::Unsigned: Mantissa,
          Iter: iter::Iterator<Item=u8>
{
    // Calculate the numerator and denominator.
    let exponent = -exponent;
    let mut num = parse_mantissa(digits, radix, max_digits);
    let mut den = Bigint::from_u32(1);
    den.imul_power(radix, exponent.as_u32());

    // Scale the numerator and denominator into range, using the bit-length.
    // This may be off by +1, however, that gives us 1 extra bit on the
    // mantissa, so we don't care.
    let mantissa_size = F::MANTISSA_SIZE;
    let mut exp = scale_ratio(&mut num, &mut den, mantissa_size);

    // Calculate in a single operation, without scaling,
    // the exponent, and then scale it to the mantissa bits.
    let (mut quo, mut rem) = num.div_large(&den);
    if quo.bit_length().as_i32() == F::MANTISSA_SIZE {
        // Under-estimated the quotient by 1 bit, scale the numerator
        // up by 2, and then re-do.
        num.ishl(1);
        let t = num.div_large(&den);
        quo = t.0;
        rem = t.1;
        exp -= 1;
    }
    debug_assert!(rem.less(&den));

    // Calculate the mantissa_size+1 bit mantissa from the quotient.
    debug_assert!(quo.bit_length() == F::MANTISSA_SIZE.as_usize() + 1);
    let m0 = quo.data().get_unchecked(0).as_u64();
    let m1 = quo.data().get_unchecked(1).as_u64();
    let mant = m0 | (m1 << u32::BITS);

    // Get the exact representation of the float from the big integer.
    // Avoid rounding-up until we create the float, to avoid
    // overflowing the 53-bits of the mantissa in edge cases, these
    // should spill to the exponent using `next()`.
    // We also need to handle subnormal exponents.
    let mut fp = ExtendedFloat { mant: mant, exp: exp };
    if fp.exp < F::DENORMAL_EXPONENT {
        // With subnormal exponents, we don't care if the remainder
        // is greater than halfway, since we're shifting right further.
        let diff = F::DENORMAL_EXPONENT - fp.exp;
        if diff <= mantissa_size + 1 {
            // We can avoid underflow and get a valid representation.
            // If there are any remnant digits left, then we should
            // round up. We need to do some shift right. Since we're
            // shifting right, our halfway digit is internal, so just
            // check if we have any truncated digits.
            let cb = bigint_rounding(!rem.is_zero());
            cb(&mut fp, diff);
        } else {
            // We have a literal 0.
            fp.mant = 0;
        }
        // We already handled the roundup, so don't take the next float.
        into_float(fp)
    } else {
        // Round the mantissa based on the quotient and the remainder.
        // We already have a native-sized mantissa, so we just need to round
        // to nearest, tie-even. We can't use the default function here.
        // To check if we're greater than or at halfway, We need to check
        // if the  remainder is >= denominator/2. We do by this
        // `rem.compare(den - rem)`. If `rem > den - rem`, then `rem > den/2`
        // and we're greater than halfway. If `rem == den -rem`, we're
        // exactly halfway. Otherwise, we're below.
        let v = den.sub_large(&rem);
        let roundup = match rem.compare(&v) {
            cmp::Ordering::Greater  => true,
            cmp::Ordering::Less     => false,
            // Only roundup if the two are equal and the mantissa is odd.
            cmp::Ordering::Equal    => mant.is_odd(),
        };

        let f: F = into_float(fp);
        match roundup {
            true  => f.next(),
            false => f,
        }
    }
}

/// Calculate the exact value of the float.
///
/// Notes:
///     The digits iterator must not have any trailing zeros (true for
///     `FloatSlice`).
///     sci_exponent and digits.size_hint() must not overflow i32.
pub unsafe fn atof<F, Iter>(digits: Iter, radix: u32, sci_exponent: i32, f: F)
    -> F
    where F: FloatRounding<u64>,
          F::Unsigned: Mantissa,
          ExtendedFloat<F::Unsigned>: bigcomp::ToBigInt<F::Unsigned>,
          Iter: iter::Iterator<Item=u8>
{
    // We have a finite conversions number of digits for base10.
    // In order for a float in radix `b` with a finite number of digits
    // to have a finite representation in radix `y`, `b` should divide
    // an integer power of `y`. This means for binary, all even radixes
    // have finite representations, and all odd ones do not.
    let max_digits = unwrap_or_max(max_digits::<F>(radix));
    let count = max_digits.min(digits.size_hint().0);
    let exponent = sci_exponent + 1 - count.as_i32();

    if use_bigcomp(radix, count) {
        bigcomp_atof(digits, radix, sci_exponent, f)
    } else if exponent >= 0 {
        positive_exponent_atof(digits, radix, max_digits, exponent)
    } else if bigcomp::use_fast(radix, count) {
        // Use the bigcomp fast path when we have a small number of digits,
        // since the native division is a lot faster. This is slower
        // when the exponent is positive, not requiring division, but a lot
        // faster otherwise.
        return bigcomp::fast_atof(digits, radix, sci_exponent, f);
    } else {
        negative_exponent_atof(digits, radix, max_digits, exponent)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use util::test::*;
    use super::*;

    fn check_equal(x: &[Limb], y: &[Limb]) {
        assert_eq!(x, y);
    }

    #[test]
    fn parse_mantissa_test() {
        check_equal(&parse_mantissa("24703".bytes(), 10, usize::max_value()).data(), &from_u32(&[24703]));
        check_equal(&parse_mantissa("24703282292062327208828439643411068618252990130716".bytes(), 10, usize::max_value()).data(), &from_u32(&[1661053468, 4066317011, 983098885, 3087178645, 3876945216, 16]));
        check_equal(&parse_mantissa("2470328229206232720882843964341106861825299013071623822127928412503377536351043759326499181808179961".bytes(), 10, usize::max_value()).data(), &from_u32(&[343792377, 1257502172, 2486839483, 4245685716, 278612094, 1973156400, 2455112332, 951308983, 1039746225, 2266565483, 1156]));
        check_equal(&parse_mantissa("24703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435".bytes(), 10, usize::max_value()).data(), &from_u32(&[1695729331, 2456375028, 3202236074, 3827510783, 47077647, 1923930861, 3789246812, 1911374662, 2977998206, 2120411016, 2528099569, 141674960, 342998848, 2272761363, 1312604224, 282079522, 3157733715, 3055856552, 205697368, 2181386376, 5414488]));
        check_equal(&parse_mantissa("2470328229206232720882843964341106861825299013071623822127928412503377536351043759326499181808179961898982823477228588654633283551779698981993873980053909390631503565951557022639229085839244910518443593180284993653615250031937045767824921936562366986365848075700158576926990370631192827955855133292783433840935197801553124659726357957462276646527282722005637400648549997709659947045402082816622623785".bytes(), 10, usize::max_value()).data(), &from_u32(&[814251049, 2755077981, 4285001321, 2669351300, 2519242029, 3034311551, 342276798, 880308361, 1748718364, 3732949581, 781299344, 1214264338, 3185362616, 3233582652, 2112208418, 3414277503, 3219913079, 3380631325, 3580026062, 3444362784, 2165743130, 776645065, 2983620801, 3002608231, 144145998, 3282817425, 26054371, 3320616926, 2257167648, 3646068255, 3858222114, 4236286773, 2095261913, 1648760608, 75076342, 351411606, 4155499695, 644551068, 4154269317, 1622349521, 1023961221, 27631]));
        check_equal(&parse_mantissa("24703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328124999".bytes(), 10, usize::max_value()).data(), &from_u32(&[1727738439, 330069557, 3509095598, 686205316, 156923684, 750687444, 2688855918, 28211928, 1887482096, 3222998811, 913348873, 1652282845, 1600735541, 1664240266, 84454144, 1487769792, 1855966778, 2832488299, 507030148, 1410055467, 2513359584, 3453963205, 779237894, 3456088326, 3671009895, 3094451696, 1250165638, 2682979794, 357925323, 1713890438, 3271046672, 3485897285, 3934710962, 1813530592, 199705026, 976390839, 2805488572, 2194288220, 2094065006, 2592523639, 3798974617, 586957244, 1409218821, 3442050171, 3789534764, 1380190380, 2055222457, 3535299831, 429482276, 389342206, 133558576, 721875297, 3013586570, 540178306, 2389746866, 2313334501, 422440635, 1288499129, 864978311, 842263325, 3016323856, 2282442263, 1440906063, 3931458696, 3511314276, 1884879882, 946366824, 4260548261, 1073379659, 1732329252, 3828972211, 1915607049, 3665440937, 1844358779, 3735281178, 2646335050, 1457460927, 2940016422, 1051]));
    }

    #[test]
    fn atof_test() {
        unsafe {
            let f: f64 = atof("898846567431158".bytes(), 10, 307, 8.98846567431158e+307);
            assert_eq!(f, 8.98846567431158e+307);

            let f: f64 = atof("247032822920623".bytes(), 10, -324, 0f64);
            assert_eq!(f, 0.0);

            let f: f64 = atof("247032822920625".bytes(), 10, -324, 0f64);
            assert_eq!(f, 5e-324);

            let f: f64 = atof("24703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328124999".bytes(), 10, -324, 0f64);
            assert_eq!(f, 0.0);

            let f: f64 = atof("24703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125".bytes(), 10, -324, 0f64);
            assert_eq!(f, 0.0);

            let f: f64 = atof("24703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001".bytes(), 10, -324, 0f64);
            assert_eq!(f, 5e-324);
        }
    }
}
