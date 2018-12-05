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

/// Calculate `b` from a a representation of `b` as a float.
///
/// Returns the mantissa (to F::MANTISSA_SIZE+1 bits)
#[inline]
pub fn calculate_b<F: Float>(b: F)
    -> (F::Unsigned, i32)
{
    //let shift = F::BITS.as_i32() - F::MANTISSA_SIZE - 1;
    // TODO(ahuszagh) Need to shift it to 53-bits..
    //let mant = b.mantissa();

    (b.mantissa(), b.exponent())
}

/// Calculate `b+h` from a a representation of `b` as a float.
#[inline]
pub fn calculate_bh<F: Float>(b: F)
    -> (F::Unsigned, i32)
{
    // None of these can overflow.
    let (mant, exp) = calculate_b(b);
    (mant * F::Unsigned::TWO + F::Unsigned::ONE, exp - 1)
}

/// Calculate the scaling factor so exactly 1 digit is left of the decimal.
///
/// Normalize float so it can be represented in scientific notation, so
/// we can compare digits.
#[inline]
pub fn scaling_factor<F: Float>(base: u32, mant: F::Unsigned, exp: i32)
    -> (F::Unsigned, i32)
{
    // TODO(ahuszagh) Need to add the number of digits in the mantissa...
    let basen_exp = basen_exponent(base, exp);
    unimplemented!()
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
//              This can likely work using leading_zeros() + exp + some log magic.
//          5. Generate bignum representations of the numerator and denominator.
//          6. Find the start of the digits in the coefficient.
//          7. Generate digits via divmod until a difference is found.


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_b_test() {
//        assert_eq!(calculate_b(1e-45_f32), (1, -149));
//        assert_eq!(calculate_b(5e-324_f64), (1, -1074));
//        assert_eq!(calculate_b(1_f32), (8388608, -23));
//        assert_eq!(calculate_b(1_f64), (4503599627370496, -52));
//        assert_eq!(calculate_b(1e38_f32), (9860761, 103));
//        assert_eq!(calculate_b(1e308_f64), (5010420900022432, 971));
    }

    #[test]
    fn calculate_bh_test() {
//        assert_eq!(calculate_bh(1e-45_f32), (3, -150));
//        assert_eq!(calculate_bh(5e-324_f64), (3, -1075));
//        assert_eq!(calculate_bh(1_f32), (16777217, -24));
//        assert_eq!(calculate_bh(1_f64), (9007199254740993, -53));
//        assert_eq!(calculate_bh(1e38_f32), (19721523, 102));
//        assert_eq!(calculate_bh(1e308_f64), (10020841800044865, 970));
    }
}
