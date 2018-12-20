//! Wrapper around David Tolnay's dtoa.

use dtoa;
use util::*;

// F32

/// Wrapper for dtoa.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) fn float_decimal(f: f32, bytes: &mut [u8])
    -> usize
{
    dtoa::write(bytes, f).expect("Write to in-memory buffer.")
}

// F64

/// Wrapper for dtoa.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) fn double_decimal(d: f64, bytes: &mut [u8])
    -> usize
{
    dtoa::write(bytes, d).expect("Write to in-memory buffer.")
}
