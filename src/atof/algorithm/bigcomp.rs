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

use util::*;
use super::exponent::*;

// TODO(ahuszagh) Can just return the.... Extended Float tbh...
///// Simplified
//pub struct Bigfloat<M: UnsignedInteger> {
//    mant: M,
//    exp: i32,
//}

/// Calculate `b` from a a representation of `b` as a float.
///
/// Returns the normalized mantissa in 128 bits and the exponent,
/// so the float is represented as `mant * 2^exp`.
#[inline]
pub fn calculate_b<F: Float>(b: F)
    -> (F::Unsigned, i32)
{
    // Get our mantissa and exponent.
    let mant = b.mantissa();
    let exp = b.exponent();
    debug_assert!(mant != F::Unsigned::ZERO, "calculate_b() mantissa is 0.");

    // Need to shift to the hidden bit. This should never overflow,
    // since we're grabbing the bottom MANTISSA_SIZE+1 bits.
    // This is only true for denormal floats.
    let upper = F::BITS.as_i32() - (F::MANTISSA_SIZE+1);
    let shift = mant.leading_zeros().as_i32() - upper;
    debug_assert!(shift >= 0, "calculate_b() shift is negative {}.", shift);

    (mant << shift, exp - shift)
}

/// Calculate `b+h` from a a representation of `b` as a float.
///
/// Returns the mantissa (to F::MANTISSA_SIZE+2 bits) and the exponent,
/// so the float is represented as `mant * 2^exp`.
#[inline]
pub fn calculate_bh<F: Float>(b: F)
    -> (F::Unsigned, i32)
{
    // None of these can overflow.
    let (mant, exp) = calculate_b(b);
    (mant * F::Unsigned::TWO + F::Unsigned::ONE, exp - 1)
}

/// Normalize the mantissa and exponent.
//#[inline]

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
    fn calculate_b_test() {
        assert_eq!(calculate_b(1e-45_f32), (8388608, -172));
        assert_eq!(calculate_b(5e-324_f64), (4503599627370496, -1126));
        assert_eq!(calculate_b(1e-323_f64), (4503599627370496, -1125));
        assert_eq!(calculate_b(2e-323_f64), (4503599627370496, -1124));
        assert_eq!(calculate_b(3e-323_f64), (6755399441055744, -1124));
        assert_eq!(calculate_b(4e-323_f64), (4503599627370496, -1123));
        assert_eq!(calculate_b(5e-323_f64), (5629499534213120, -1123));
        assert_eq!(calculate_b(6e-323_f64), (6755399441055744, -1123));
        assert_eq!(calculate_b(7e-323_f64), (7881299347898368, -1123));
        assert_eq!(calculate_b(8e-323_f64), (4503599627370496, -1122));
        assert_eq!(calculate_b(9e-323_f64), (5066549580791808, -1122));
        assert_eq!(calculate_b(1_f32), (8388608, -23));
        assert_eq!(calculate_b(1_f64), (4503599627370496, -52));
        assert_eq!(calculate_b(1e38_f32), (9860761, 103));
        assert_eq!(calculate_b(1e308_f64), (5010420900022432, 971));
    }

    #[test]
    fn calculate_bh_test() {
        assert_eq!(calculate_bh(1e-45_f32), (16777217, -173));
        assert_eq!(calculate_bh(5e-324_f64), (9007199254740993, -1127));
        assert_eq!(calculate_bh(1_f32), (16777217, -24));
        assert_eq!(calculate_bh(1_f64), (9007199254740993, -53));
        assert_eq!(calculate_bh(1e38_f32), (19721523, 102));
        assert_eq!(calculate_bh(1e308_f64), (10020841800044865, 970));
    }
}
