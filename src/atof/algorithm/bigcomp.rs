//! An implementation of bigcomp for Rust.
//!
//! Compares the known string to theoretical digits generated on the
//! fly for `b+h`, where a string representation of a float is between
//! `b` and `b+u`, where `b+u` is 1 unit in the least-precision. Therefore,
//! the string must be close to `b+h`.
//!
//! Adapted from:
//!     https://www.exploringbinary.com/bigcomp-deciding-truncated-near-halfway-conversions/

#![allow(unused)]       // TODO(ahuszagh) Remove later

use float::*;
use util::*;
use super::cached::*;
use super::exponent::*;

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
pub unsafe fn fast_scaling_factor(base: u32, sci_exponent: i32) -> ExtendedFloat160 {
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
///
pub unsafe fn fast_ratio<F: Float>(base: u32, sci_exponent: i32, b: F)
    -> (u128, u128)
    where F::Unsigned: Mantissa
{
    let num = fast_normalize(bh(b));
    let den = fast_scaling_factor(base, sci_exponent);

    let diff = (den.exp - num.exp);
    debug_assert!(diff <= integral_binary_factor(base).as_i32(), "make_ratio() improper scaling.");

    (num.mant >> diff, den.mant)
}

// TODO(ahuszagh):
//      Steps:
//          1. Determine `b` from the extended-precision float.
//              Can do this from an extended-precision float, since we have custom
//              rounding schemes.
//          2. Extract the mantissa and the exponent from `b`.
//          3. Determine `b+h` from `b`.
//          4. Find a factor of `base` that scales it so exactly 1 digit
//              is to the left of the decimal place.
//              We can only do this exactly from a string, so this is easy.
//
//          5. Generate bignum representations of the numerator and denominator.
//              We can do this with 128-bit values.

//          6. Find the start of the digits in the coefficient.
//          7. Generate digits via divmod until a difference is found.

// Current example:
//  b = 1e308_f64
//      nd = 308
//      mant, exp = (5010420900022432, 971)
//      idx = nd + BIAS
//      scale_mant, scale_exp = (189288349786683953755640255602884245064, 896)


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

    // TODO(ahuszagh) Need normalize tests
    // calculate_scaling_factor

    #[test]
    fn fast_ratio_test() {
        unsafe {
            let num = 42535295865117307932921825928971026432;
            let den = 17218479456385750618067377696052635483;
            assert_eq!(fast_ratio(10, -324, 0f64), (num, den));
        }
    }
}
