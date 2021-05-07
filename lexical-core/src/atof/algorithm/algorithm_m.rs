//! Algorithm M creates an exact representation from the digits of a float.
//!
//! Note: this algorithm exists entirely for historical purposes
//! and is currently not used. It exists primarily for a simple,
//! correct, reasonably-performance implementation.
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

#![cfg(feature = "algorithm_m")]

use crate::util::*;

use super::alias::*;
use super::bhcomp;
use super::bigcomp;
use super::bignum::*;
use super::format::*;

// ALGORITHM M

/// Scale the ratio by powers of 2 until the num/den will be set properly.
///
/// Returns the estimated binary exponent.
///
/// We want the returned quotient to have [64, 65) bits, IE, the bit
/// length of the numerator - denominator is 64.
fn scale_ratio<F>(num: &mut Bigint<F>, den: &mut Bigint<F>, mantissa_size: i32) -> i32
where
    F: FloatType,
{
    // We to scale the numerator and the denominator so there are
    // MANTISSA_SIZE extra bits in the numerator, so we can at
    // maximum overestimate the denominator by 1. This will mean
    // the resulting quotient will have `MANTISSA_SIZE` or
    // `MANTISSA_SIZE+1` bits. This is fine, since having 1 extra bit
    // is easy to deal with, especially with the remainder.
    let exp = num.bit_length() as i32 - den.bit_length() as i32;
    let shift = mantissa_size - exp;
    let nlz = den.leading_zeros();
    if shift > 0 {
        // Need to shift the numerator left, adjust the binary exponent.
        let shift = shift as usize;
        let s2 = nlz;
        let s1 = shift + s2;
        num.ishl(s1);
        den.ishl(s2);
    } else if shift < 0 {
        // Need to shift the denominator left, and ensure it's divisible by Limb::BITS.
        let shift = (-shift) as usize;
        let s2 = nlz + (shift % <Limb as Integer>::BITS);
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
pub(super) fn negative_exponent_atof<'a, F, Data>(
    data: Data,
    radix: u32,
    max_digits: usize,
    exponent: i32,
    kind: RoundingKind,
) -> F
where
    F: FloatType,
    Data: SlowDataInterface<'a>,
{
    // Calculate the numerator and denominator.
    let exponent = -exponent;
    let mut num = bhcomp::parse_mantissa::<F, _>(data, radix, max_digits);
    let mut den = Bigint::<F>::from_u32(1);
    den.imul_power(radix, exponent as u32);

    // Scale the numerator and denominator into range, using the bit-length.
    // This may be off by +1, however, that gives us 1 extra bit on the
    // mantissa, so we don't care.
    let mantissa_size = F::MANTISSA_SIZE;
    let mut exp = scale_ratio(&mut num, &mut den, mantissa_size);

    // Calculate in a single operation, without scaling,
    // the exponent, and then scale it to the mantissa bits.
    let (mut quo, mut rem) = num.div_large(&den);
    if quo.bit_length() as i32 == F::MANTISSA_SIZE {
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
    // Have at max mantissa_size bits with the hidden bit (+1),
    // we we can grab and lower bits and shift them in in case
    // we are using 32-bit limbs.
    debug_assert!(quo.bit_length() as i32 == F::MANTISSA_SIZE + 1);
    let shift = F::BITS - quo.bit_length();
    let mant = quo.himant().0 >> shift;

    // Get the exact representation of the float from the big integer.
    // Avoid rounding-up until we create the float, to avoid
    // overflowing the 53-bits of the mantissa in edge cases, these
    // should spill to the exponent using `next()`.
    // We also need to handle subnormal exponents.
    let mut fp = ExtendedFloat {
        mant,
        exp,
    };
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
            bhcomp::round_to_native::<F, _>(&mut fp, !rem.is_zero(), kind);
        } else {
            // We have a literal 0.
            fp.mant = F::MantissaType::ZERO;
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
        let order = rem.compare(&v);
        let f: F = into_float(fp);
        bigcomp::round_to_native(f, order, kind)
    }
}

/// Calculate the exact value of the float.
///
/// Notes:
///     The digits iterator must not have any trailing zeros (true for
///     `FloatSlice`).
///     sci_exponent and digits.size_hint() must not overflow i32.
#[allow(dead_code)]
pub(super) fn atof<'a, F, Data>(data: Data, radix: u32, f: F, kind: RoundingKind) -> F
where
    F: FloatType,
    Data: SlowDataInterface<'a>,
{
    // We have a finite conversions number of digits for base10.
    // In order for a float in radix `b` with a finite number of digits
    // to have a finite representation in radix `y`, `b` should divide
    // an integer power of `y`. This means for binary, all even radixes
    // have finite representations, and all odd ones do not.
    let max_digits = unwrap_or_max(F::max_correct_digits(radix));
    let count = max_digits.min(data.mantissa_digits());
    let exponent = data.scientific_exponent() + 1 - count as i32;

    if bhcomp::use_bigcomp(radix, count) {
        bigcomp::atof(data, radix, f, kind)
    } else if exponent >= 0 {
        bhcomp::large_atof(data, radix, max_digits, exponent, kind)
    } else {
        negative_exponent_atof(data, radix, max_digits, exponent, kind)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atof_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let kind = RoundingKind::NearestTieEven;

        let data: Data = (b!("8"), Some(b!("98846567431158")), Some(b!("+307")), 307).into();
        assert_eq!(
            8.98846567431158e+307,
            atof::<f64, _>(data.to_slow(0), 10, 8.98846567431158e+307, kind)
        );

        let data: Data = (b!("2"), Some(b!("47032822920623")), Some(b!("-324")), -324).into();
        assert_eq!(0.0, atof::<f64, _>(data.to_slow(0), 10, 0.0, kind));

        let data: Data = (b!("2"), Some(b!("47032822920625")), Some(b!("-324")), -324).into();
        assert_eq!(5e-324, atof::<f64, _>(data.to_slow(0), 10, 0.0, kind));

        let data: Data = (b!("2"), Some(b!("4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328124999")), Some(b!("-324")), -324).into();
        assert_eq!(0.0, atof::<f64, _>(data.to_slow(0), 10, 0.0, kind));

        let data: Data = (b!("2"), Some(b!("4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125")), Some(b!("-324")), -324).into();
        assert_eq!(0.0, atof::<f64, _>(data.to_slow(0), 10, 0.0, kind));

        let data: Data = (b!("2"), Some(b!("4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001")), Some(b!("-324")), -324).into();
        assert_eq!(5e-324, atof::<f64, _>(data.to_slow(0), 10, 0.0, kind));
    }
}
