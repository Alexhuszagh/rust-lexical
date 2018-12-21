//! Wrapper around David Tolnay's dtoa.

use dtoa;
use util::*;

// F32

/// Wrapper for dtoa.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) fn float_decimal<'a>(f: f32, bytes: &'a mut [u8])
    -> &'a mut [u8]
{
    let len = dtoa::write(bytes, f).expect("Write to in-memory buffer.");
    &mut bytes[len..]
}

// F64

/// Wrapper for dtoa.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) fn double_decimal<'a>(d: f64, bytes: &'a mut [u8])
    -> &'a mut [u8]
{
    let len = dtoa::write(bytes, d).expect("Write to in-memory buffer.");
    &mut bytes[len..]
}
