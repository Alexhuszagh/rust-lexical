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

/// Calculate `b` from the mantissa and the exponent.
// TODO(ahuszagh) This only works with.... SHIT... base 2 powers... FML
/// Mantissa must not be zero.
#[inline]
pub fn calculate_b<F: Float>(f: F)
{
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
//          5. Generate bignum representations of the numerator and denominator.
//          6. Find the start of the digits in the coefficient.
//          7. Generate digits via divmod until a difference is found.


#[cfg(test)]
mod tests {
    //#[test]
    //fn tes
}
