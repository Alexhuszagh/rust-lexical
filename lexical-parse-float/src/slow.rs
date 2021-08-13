//! Slow, fallback cases where we cannot unambiguously round a float.
//!
//! This occurs when we cannot determine the exact representation using
//! both the fast path (native) cases nor the Lemire/Bellerophon algorithms,
//! and therefore must fallback to a slow, arbitrary-precision representation.

#![doc(hidden)]

use crate::float::{ExtendedFloat80, RawFloat};
use lexical_util::iterator::{Bytes, BytesIter};

// TODO(ahuszagh) Implement...

/// Parse the significant digits and biased, binary exponent of a float.
///
/// This is a fallback algorithm that uses a big-integer representation
/// of the float, and therefore is considerably slower than faster
/// approximations. However, it will always determine how to round
/// the significant digits to the nearest machine float, allowing
/// use to handle near half-way cases.
///
/// Near half-way cases are halfway between two consecutive machine floats.
/// For example, the float `16777217.0` has a bitwise representation of
/// `100000000000000000000000 1`. Rounding to a single-precision float,
/// the trailing `1` is truncated. Using round-nearest, tie-even, any
/// value above `16777217.0` must be rounded up to `16777218.0`, while
/// any value before or equal to `16777217.0` must be rounded down
/// to `16777216.0`. These near-halfway conversions therefore may require
/// a large number of digits to unambiguously determine how to round.
#[inline]
pub fn slow_decimal<F: RawFloat, const FORMAT: u128>(
    mut byte: Bytes<FORMAT>,
    exponent: i64,
    decimal_point: u8,
) -> ExtendedFloat80 {
    // This assumes the sign bit has already been parsed, and we're
    // starting with the integer digits, and the float format has been
    // correctly validated.
    todo!();
}
