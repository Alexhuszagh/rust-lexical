//! An implementation of bigcomp for Rust.
//!
//! Compares the known string to theoretical digits generated on the
//! fly for `b+h`, where a string representation of a float is between
//! `b` and `b+u`, where `b+u` is 1 unit in the least-precision. Therefore,
//! the string must be close to `b+h`.
//!
//! Adapted from:
//!     https://www.exploringbinary.com/bigcomp-deciding-truncated-near-halfway-conversions/

use stackvector;
use lib::{cmp, iter};
use float::*;
use table::*;
use util::*;
use super::cached::*;
use super::exponent::*;
use super::math::*;

// SHARED

/// Calculate `b` from a a representation of `b` as a float.
#[inline]
pub fn b<F: Float>(f: F)
    -> ExtendedFloat<F::Unsigned>
    where F::Unsigned: Mantissa
{
    f.into()
}

/// Calculate `b+h` from a a representation of `b` as a float.
#[inline]
pub fn bh<F: Float>(f: F)
    -> ExtendedFloat<F::Unsigned>
    where F::Unsigned: Mantissa
{
    // None of these can overflow.
    let mut b = b(f);
    b.mant <<= 1;
    b.mant += as_cast(1);
    b.exp -= 1;
    b
}

/// Determine whether we can use the fast path.
///
/// We can use a faster path, with 128-bit precision integers, for most
/// borderline cases, where we have <= a certain number of digits
/// that can be represented with a 128-bit numerator and denominator.
///
/// Since we calculate the numerator exactly (from a representation of `b`)
/// and the denominator with inexactly, the last digit may be inaccurate,
/// so for comfort, we only use this algorithm when the the mantissa digits
/// is less than or equal to the number of digits minus 2 that can be
/// exactly represented.
///
/// The number of digits that can be exactly represented, assuming no
/// rounding error, is: `(128 / log2(10)).floor()`.
///
/// * `base`            - Radix for the number parsing.
/// * `mantissa_digits` - Number of digits in the mantissa.
#[inline]
pub fn use_fast(base: u32, mantissa_digits: usize) -> bool {
    let exact_digits = match base {
        3  => 78,
        5  => 53,
        6  => 47,
        7  => 43,
        9  => 38,
        10 => 36,
        11 => 35,
        12 => 33,
        13 => 32,
        14 => 31,
        15 => 30,
        17 => 29,
        18 => 28,
        19 => 28,
        20 => 27,
        21 => 27,
        22 => 26,
        23 => 26,
        24 => 25,
        25 => 25,
        26 => 25,
        27 => 24,
        28 => 24,
        29 => 24,
        30 => 24,
        31 => 23,
        33 => 23,
        34 => 23,
        35 => 22,
        36 => 22,
        // Powers of 2, and others, should already be handled by now.
        _  => unreachable!(),
    };

    mantissa_digits <= exact_digits
}

// FAST PATH

/// Normalize the mantissa to 128 bits.
///
/// * `fp`      - Lower-precision floating-point number.
#[inline]
pub fn fast_normalize<M: Mantissa>(fp: ExtendedFloat<M>)
    -> ExtendedFloat160
{
    let mut fp = ExtendedFloat160 { mant: fp.mant.as_u128(), exp: fp.exp };
    fp.normalize();
    fp
}

/// Get the appropriate scaling factor from the digit count.
///
/// * `base`            - Radix for the number parsing.
/// * `sci_exponent`    - Exponent of basen string in scientific notation.
#[inline]
pub unsafe fn fast_scaling_factor(base: u32, sci_exponent: i32)
    -> ExtendedFloat160
{
    let powers = ExtendedFloat160::get_powers(base);
    let sci_exponent = sci_exponent + powers.bias;
    let small_index = sci_exponent % powers.step;
    let large_index = sci_exponent / powers.step;

    // We've already done bounds checking before, in `multiply_exponent_extended`.
    // Since the bounds are slightly excessive, we'll be safe regardless.
    let small = powers.get_small(small_index as usize);
    let large = powers.get_large(large_index as usize);

    // Widen to 160-bits and multiply and normalize, with enough space for
    // 1 operation before.
    let mut wide = large.mul(&small);
    wide.normalize_to(integral_binary_factor(base));
    wide
}

/// Make a ratio for the numerator and denominator.
///
/// * `base`            - Radix for the number parsing.
/// * `sci_exponent`    - Exponent of basen string in scientific notation.
/// * `f`               - Sub-halfway (`b`) float.
pub unsafe fn fast_ratio<F: Float>(base: u32, sci_exponent: i32, f: F)
    -> (u128, u128)
    where F::Unsigned: Mantissa
{
    let num = fast_normalize(bh(f));
    let den = fast_scaling_factor(base, sci_exponent);

    let diff = den.exp - num.exp;
    debug_assert!(diff <= integral_binary_factor(base).as_i32(), "make_ratio() improper scaling.");

    (num.mant >> diff, den.mant)
}

/// Compare digits between the generated values the ratio and the actual view.
///
/// * `digits`      - Actual digits from the mantissa.
/// * `base`        - Radix for the number parsing.
/// * `num`         - Numerator for the fraction.
/// * `denm`        - Denominator for the fraction.
pub unsafe fn fast_compare_digits<Iter>(mut digits: Iter, base: u32, mut num: u128, den: u128)
    -> cmp::Ordering
    where Iter: iter::Iterator<Item=u8>
{
    // Iterate until we get a difference in the generated digits.
    // If we run out,return Equal.
    let base: u128 = base.into();
    while !num.is_zero() {
        let actual = match digits.next() {
            Some(v) => v,
            None    => return cmp::Ordering::Less,
        };
        let expected = digit_to_char(num / den);
        num = base * (num % den);
        if actual < expected {
            return cmp::Ordering::Less;
        } else if actual > expected {
            return cmp::Ordering::Greater;
        }
    }

    // If there are remaining digits, and all are equal to 0, then we're
    // equal, otherwise, we produced all the digits we could and we're above.
    match digits.all(|v| v == b'0') {
        true  => cmp::Ordering::Equal,
        false => cmp::Ordering::Greater,
    }
}

/// Generate the correct representation from a halfway representation.
///
/// * `digits`          - Actual digits from the mantissa.
/// * `base`            - Radix for the number parsing.
/// * `sci_exponent`    - Exponent of basen string in scientific notation.
/// * `f`               - Sub-halfway (`b`) float.
pub unsafe fn fast_atof<F, Iter>(digits: Iter, base: u32, sci_exponent: i32, f: F)
    -> F
    where F: Float,
          F::Unsigned: Mantissa,
          Iter: iter::Iterator<Item=u8>
{
    let (num, den) = fast_ratio(base, sci_exponent, f);
    match fast_compare_digits(digits, base, num, den) {
        // Greater than representation, return `b+u`
        cmp::Ordering::Greater  => f.next(),
        // Less than representation, return `b`
        cmp::Ordering::Less     => f,
        // Exactly halfway, tie to even.
        cmp::Ordering::Equal    => if f.is_odd() { f.next() } else { f },
    }
}

// BIG INT

/// Storage for a big integer type.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bigint {
    /// Internal storage for the Bigint, in little-endian order.
    ///
    /// Enough storage for up to 10^345, which is 2^1146, or more than
    /// the max for f64.
    data: stackvector::StackVec<[u32; 36]>,
    /// It also makes sense to store an exponent, since this simplifies
    /// normalizing and powers of 2.
    exp: i32,
}

impl SharedOps for Bigint {
    type StorageType = stackvector::StackVec<[u32; 36]>;

    #[inline]
    fn data<'a>(&'a self) -> &'a Self::StorageType {
        &self.data
    }

    #[inline]
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType {
        &mut self.data
    }
}

impl SmallOps for Bigint {
    #[inline]
    fn imul_pow2(&mut self, n: u32) {
        // Increment exponent to simulate actual multiplication.
        self.exp += n.as_i32();
    }

    #[inline]
    fn idiv_pow2(&mut self, n: u32, _: bool) {
        // Decrement exponent to simulate actual division.
        self.exp -= n.as_i32();
    }
}

impl LargeOps for Bigint {
}

// TO BIG INT

/// Simple overloads to allow conversions of extended floats to big integers.
pub trait ToBigInt<M: Mantissa> {
    fn to_bigint(&self) -> Bigint;
}

impl ToBigInt<u32> for ExtendedFloat<u32> {
    #[inline]
    fn to_bigint(&self) -> Bigint {
        let mut bigint = Bigint::from_u32(self.mant);
        bigint.exp = self.exp;
        bigint
    }
}

impl ToBigInt<u64> for ExtendedFloat<u64> {
    #[inline]
    fn to_bigint(&self) -> Bigint {
        let mut bigint = Bigint::from_u64(self.mant);
        bigint.exp = self.exp;
        bigint
    }
}

impl ToBigInt<u128> for ExtendedFloat<u128> {
    #[inline]
    fn to_bigint(&self) -> Bigint {
        let mut bigint = Bigint::from_u128(self.mant);
        bigint.exp = self.exp;
        bigint
    }
}

// SLOW PATH

/// Get the appropriate scaling factor from the digit count.
///
/// * `base`            - Radix for the number parsing.
/// * `sci_exponent`    - Exponent of basen string in scientific notation.
#[inline]
pub unsafe fn slow_scaling_factor(base: u32, sci_exponent: u32)
    -> Bigint
{
    let mut factor = Bigint { data: stackvec![1], exp: 0 };
    factor.imul_power(sci_exponent, base);
    factor
}

/// Make a ratio for the numerator and denominator.
///
/// * `base`            - Radix for the number parsing.
/// * `sci_exponent`    - Exponent of basen string in scientific notation.
/// * `f`               - Sub-halfway (`b`) float.
pub unsafe fn slow_ratio<F: Float>(base: u32, sci_exponent: i32, f: F)
    -> (Bigint, Bigint)
    where F::Unsigned: Mantissa,
          ExtendedFloat<F::Unsigned>: ToBigInt<F::Unsigned>
{
    let bh = bh(f).to_bigint();
    let factor = slow_scaling_factor(base, sci_exponent.abs().as_u32());
    let mut num: Bigint;
    let mut den: Bigint;

    if sci_exponent < 0 {
        // Need to have the base10 factor be the numerator, and the fp
        // be the denominator. Since we assumed that bh was the numerator,
        // if it's the denominator, we need to multiply it into the numerator.
        num = factor;
        num.imul_large(&bh);
        den = Bigint { data: stackvec![1], exp: -bh.exp };
    } else {
        num = bh;
        den = factor;
    }

    // Scale the denominator so it has the number of bits
    // in the base as the number of leading zeros.
    let wlz = integral_binary_factor(base).as_usize();
    let nlz = den.leading_zeros().wrapping_sub(wlz) & (u32::BITS - 1);
    if nlz != 0 {
        small::ishl_bits(den.data_mut(), nlz.as_u32());
        den.exp -= nlz.as_i32();
    }

    // Need to scale the numerator or denominator to the same value.
    // We don't want to shift the denominator, so...
    let diff = den.exp - num.exp;
    let shift = diff.abs().as_usize();
    if diff < 0 {
        // Need to shift the numerator left.
        small::ishl(num.data_mut(), shift);
        num.exp -= shift.as_i32()
    } else if diff > 0 {
        // Need to shift denominator left, go by a power of 32.
        // After this, the numerator will be non-normalized, and the
        // denominator will be normalized.
        // We need to add one to the quotient,since we're calculating the
        // ceiling of the divmod.
        let (q, r) = shift.ceil_divmod(u32::BITS);
        if !r.is_zero() {
            // Since we're using a power from the denominator to the
            // numerator, we to invert r, not add u32::BITS.
            let r = -r;
            small::ishl_bits(num.data_mut(), r.as_u32());
            num.exp -= r;
        }
        if !q.is_zero() {
            den.pad_zeros(q);
            den.exp -= 32 * q.as_i32();
        }
    }

    (num, den)

//    // Normalize the denominator, so it has a leading-bit in the
//    // most-significant digit.
//    let nlz = den.leading_zeros();
//    if nlz != 0 {
//        small::ishl_bits(den.data_mut(), nlz.as_u32());
//        den.exp -= nlz.as_i32();
//    }
//
//    // Need to scale the numerator or denominator to the same value.
//    // We don't want to shift the denominator, so...
//    let diff = den.exp - num.exp;
//    let shift = diff.abs().as_usize();
//    if diff < 0 {
//        // Need to shift the numerator left.
//        small::ishl(num.data_mut(), shift);
//        num.exp -= shift.as_i32()
//    } else if diff > 0 {
//        // Need to shift denominator left, go by a power of 32.
//        // After this, the numerator will be non-normalized, and the
//        // denominator will be normalized.
//        let (q, r) = shift.ceil_divmod(32);
//        if !r.is_zero() {
//            small::ishl_bits(num.data_mut(), (32 + r).as_u32());
//            num.exp += r;
//        }
//        if !q.is_zero() {
//            den.pad_zeros(q);
//            den.exp -= 32 * q.as_i32();
//        }
//    }
//
//    (num, den)
}

/// Compare digits between the generated values the ratio and the actual view.
///
/// * `digits`      - Actual digits from the mantissa.
/// * `base`        - Radix for the number parsing.
/// * `num`         - Numerator for the fraction.
/// * `denm`        - Denominator for the fraction.
pub unsafe fn slow_compare_digits<Iter>(mut digits: Iter, base: u32, mut num: Bigint, den: Bigint)
    -> cmp::Ordering
    where Iter: iter::Iterator<Item=u8>
{
    // Iterate until we get a difference in the generated digits.
    // If we run out,return Equal.
    while !num.data.is_empty() {
        let actual = match digits.next() {
            Some(v) => v,
            None    => return cmp::Ordering::Less,
        };
        let expected = digit_to_char(num.quorem(&den));
        num.imul_small(base);
        if actual < expected {
            return cmp::Ordering::Less;
        } else if actual > expected {
            return cmp::Ordering::Greater;
        }
    }

    // If there are remaining digits, and all are equal to 0, then we're
    // equal, otherwise, we produced all the digits we could and we're above.
    match digits.all(|v| v == b'0') {
        true  => cmp::Ordering::Equal,
        false => cmp::Ordering::Greater,
    }
}

/// Generate the correct representation from a halfway representation.
///
/// * `digits`          - Actual digits from the mantissa.
/// * `base`            - Radix for the number parsing.
/// * `sci_exponent`    - Exponent of basen string in scientific notation.
/// * `f`               - Sub-halfway (`b`) float.
pub unsafe fn slow_atof<F, Iter>(digits: Iter, base: u32, sci_exponent: i32, f: F)
    -> F
    where F: Float,
          F::Unsigned: Mantissa,
          ExtendedFloat<F::Unsigned>: ToBigInt<F::Unsigned>,
          Iter: iter::Iterator<Item=u8>
{
    let (num, den) = slow_ratio(base, sci_exponent, f);
    match slow_compare_digits(digits, base, num, den) {
        // Greater than representation, return `b+u`
        cmp::Ordering::Greater  => f.next(),
        // Less than representation, return `b`
        cmp::Ordering::Less     => f,
        // Exactly halfway, tie to even.
        cmp::Ordering::Equal    => if f.is_odd() { f.next() } else { f },
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_test() {
        assert_eq!(b(1e-45_f32), (1, -149).into());
        assert_eq!(b(5e-324_f64), (1, -1074).into());
        assert_eq!(b(1e-323_f64), (2, -1074).into());
        assert_eq!(b(2e-323_f64), (4, -1074).into());
        assert_eq!(b(3e-323_f64), (6, -1074).into());
        assert_eq!(b(4e-323_f64), (8, -1074).into());
        assert_eq!(b(5e-323_f64), (10, -1074).into());
        assert_eq!(b(6e-323_f64), (12, -1074).into());
        assert_eq!(b(7e-323_f64), (14, -1074).into());
        assert_eq!(b(8e-323_f64), (16, -1074).into());
        assert_eq!(b(9e-323_f64), (18, -1074).into());
        assert_eq!(b(1_f32), (8388608, -23).into());
        assert_eq!(b(1_f64), (4503599627370496, -52).into());
        assert_eq!(b(1e38_f32), (9860761, 103).into());
        assert_eq!(b(1e308_f64), (5010420900022432, 971).into());
    }

    #[test]
    fn bh_test() {
        assert_eq!(bh(1e-45_f32), (3, -150).into());
        assert_eq!(bh(5e-324_f64), (3, -1075).into());
        assert_eq!(bh(1_f32), (16777217, -24).into());
        assert_eq!(bh(1_f64), (9007199254740993, -53).into());
        assert_eq!(bh(1e38_f32), (19721523, 102).into());
        assert_eq!(bh(1e308_f64), (10020841800044865, 970).into());
    }

    // FAST PATH

    #[test]
    fn fast_scaling_factor_test() {
        unsafe {
            assert_eq!(fast_scaling_factor(10, -324), (17218479456385750618067377696052635483, -1200).into());
            assert_eq!(fast_scaling_factor(10, 0), (10633823966279326983230456482242756608, -123).into());
            assert_eq!(fast_scaling_factor(10, 300), (15878657653273753079461938932723996012, 873).into());
        }
    }

    #[test]
    fn fast_ratio_test() {
        unsafe {
            let num = 42535295865117307932921825928971026432;
            let den = 17218479456385750618067377696052635483;
            assert_eq!(fast_ratio(10, -324, 0f64), (num, den));

            let num = 127605887595351923798765477786913079296;
            let den = 17218479456385750618067377696052635483;
            assert_eq!(fast_ratio(10, -324, 5e-324f64), (num, den));

            let num = 170141183460469250621153235194464960512;
            let den = 18928834978668395375564025560288424506;
            assert_eq!(fast_ratio(10, 307, 8.98846567431158e+307f64), (num, den));
        }
    }

    #[test]
    fn fast_compare_digits_test() {
        unsafe {
            // 2^-1074
            let num = 42535295865117307932921825928971026432;
            let den = 17218479456385750618067377696052635483;

            // Less than halfway, truncated.
            let digits = "247032822920623272088284396434110686";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Less);

            // Greater than halfway.
            let digits = "247032822920623272088284396435110686";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Greater);

            // Less than halfway.
            let digits = "247032822920623272088284396433110686";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Less);

            // 2*2^-1074
            let num = 127605887595351923798765477786913079296;
            let den = 17218479456385750618067377696052635483;

            // Less than halfway, truncated.
            let digits = "741098468761869816264853189302332058";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Less);

            // Greater than halfway.
            let digits = "741098468761869816264853189303332058";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Greater);

            // Less than halfway.
            let digits = "741098468761869816264853189301332058";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Less);

            // 4503599627370496*2^971
            let num = 170141183460469250621153235194464960512;
            let den = 18928834978668395375564025560288424506;

            // Less than halfway, truncated.
            let digits = "898846567431158053656668072130502949";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Less);

            // Greater than halfway.
            let digits = "898846567431158053656668072130602949";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Greater);

            // Less than halfway.
            let digits = "898846567431158053656668072130402949";
            assert_eq!(fast_compare_digits(digits.bytes(), 10, num, den), cmp::Ordering::Less);
        }
    }

    // SLOW PATH

    #[test]
    fn slow_scaling_factor_test() {
        unsafe {
            assert_eq!(slow_scaling_factor(10, 0), Bigint { data: stackvec![1], exp: 0 });
            assert_eq!(slow_scaling_factor(10, 20), Bigint { data: stackvec![1977800241, 22204], exp: 20 });
            assert_eq!(slow_scaling_factor(10, 300), Bigint { data: stackvec![2502905297, 773182544, 1122691908, 922368819, 2799959258, 2138784391, 2365897751, 2382789932, 3061508751, 1799019667, 3501640837, 269048281, 2748691596, 1866771432, 2228563347, 475471294, 278892994, 2258936920, 3352132269, 1505791508, 2147965370, 25052104], exp: 300 });
        }
    }

    #[test]
    fn slow_ratio_test() {
        unsafe {
            let (num, den) = slow_ratio(10, -324, 0f64);
            assert_eq!(num, Bigint { data: stackvec![1725370368, 1252154597, 1017462556, 675087593, 2805901938, 1401824593, 1124332496, 2380663002, 1612846757, 4128923878, 1492915356, 437569744, 2975325085, 3331531962, 3367627909, 730662168, 2699172281, 1440714968, 2778340312, 690527038, 1297115354, 763425880, 1453089653, 331561842], exp: 312 });
            assert_eq!(den, Bigint { data: stackvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134217728], exp: 312 });

            let (num, den) = slow_ratio(10, -324, 5e-324f64);
            assert_eq!(num, Bigint { data: stackvec![881143808, 3756463792, 3052387668, 2025262779, 4122738518, 4205473780, 3372997488, 2847021710, 543572976, 3796837043, 183778774, 1312709233, 336040663, 1404661296, 1512949137, 2191986506, 3802549547, 27177609, 4040053641, 2071581115, 3891346062, 2290277640, 64301663, 994685527], exp: 312 });
            assert_eq!(den, Bigint { data: stackvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134217728], exp: 312 });

            let (num, den) = slow_ratio(10, 307, 8.98846567431158e+307f64);
            assert_eq!(num, Bigint { data: stackvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 2147483648], exp: 288 });
            assert_eq!(den, Bigint { data: stackvec![1978138624, 2671552565, 2938166866, 3588566204, 1860064291, 2104472219, 2014975858, 2797301608, 462262832, 318515330, 1101517094, 1738264167, 3721375114, 414401884, 1406861075, 3053102637, 387329537, 2051556775, 1867945454, 3717689914, 1434550525, 1446648206, 238915486], exp: 288 });
        }
    }

    #[test]
    fn slow_compare_digits_test() {
        unsafe {
            // 2^-1074
            let num = Bigint { data: stackvec![1725370368, 1252154597, 1017462556, 675087593, 2805901938, 1401824593, 1124332496, 2380663002, 1612846757, 4128923878, 1492915356, 437569744, 2975325085, 3331531962, 3367627909, 730662168, 2699172281, 1440714968, 2778340312, 690527038, 1297115354, 763425880, 1453089653, 331561842], exp: 312 };
            let den = Bigint { data: stackvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134217728], exp: 312 };

            // Below halfway
            let digits = "24703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328124999";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Less);

            // Exactly halfway.
            let digits = "24703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Equal);

            // Above halfway.
            let digits = "24703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Greater);

            // 2*2^-1074
            let num = Bigint { data: stackvec![881143808, 3756463792, 3052387668, 2025262779, 4122738518, 4205473780, 3372997488, 2847021710, 543572976, 3796837043, 183778774, 1312709233, 336040663, 1404661296, 1512949137, 2191986506, 3802549547, 27177609, 4040053641, 2071581115, 3891346062, 2290277640, 64301663, 994685527], exp: 312 };
            let den = Bigint { data: stackvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 134217728], exp: 312 };

            // Below halfway
            let digits = "74109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984374999";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Less);

            // Exactly halfway.
            let digits = "74109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984375";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Equal);

            // Above halfway.
            let digits = "74109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984375001";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Greater);

            // 4503599627370496*2^971
            let num = Bigint { data: stackvec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1024, 2147483648], exp: 288 };
            let den = Bigint { data: stackvec![1978138624, 2671552565, 2938166866, 3588566204, 1860064291, 2104472219, 2014975858, 2797301608, 462262832, 318515330, 1101517094, 1738264167, 3721375114, 414401884, 1406861075, 3053102637, 387329537, 2051556775, 1867945454, 3717689914, 1434550525, 1446648206, 238915486], exp: 288 };

            // Below halfway
            let digits = "89884656743115805365666807213050294962762414131308158973971342756154045415486693752413698006024096935349884403114202125541629105369684531108613657287705365884742938136589844238179474556051429647415148697857438797685859063890851407391008830874765563025951597582513936655578157348020066364210154316532161708031999";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Less);

            // Exactly halfway.
            let digits = "89884656743115805365666807213050294962762414131308158973971342756154045415486693752413698006024096935349884403114202125541629105369684531108613657287705365884742938136589844238179474556051429647415148697857438797685859063890851407391008830874765563025951597582513936655578157348020066364210154316532161708032";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Equal);

            // Above halfway.
            let digits = "89884656743115805365666807213050294962762414131308158973971342756154045415486693752413698006024096935349884403114202125541629105369684531108613657287705365884742938136589844238179474556051429648741514697857438797685859063890851407391008830874765563025951597582513936655578157348020066364210154316532161708032001";
            assert_eq!(slow_compare_digits(digits.bytes(), 10, num.clone(), den.clone()), cmp::Ordering::Greater);
        }
    }
}
