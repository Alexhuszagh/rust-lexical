//! Wrapper around David Tolnay's dtoa.

use crate::util::*;
use dtoa;

use super::replace::replace;

// F32

/// Wrapper for dtoa.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline]
pub(crate) fn float_decimal<'a>(f: f32, mut bytes: &'a mut [u8], format: NumberFormat) -> usize {
    let count = dtoa::write(&mut bytes, f).expect("Write to in-memory buffer.");
    replace(bytes, count, format);
    count
}

// F64

/// Wrapper for dtoa.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline]
pub(crate) fn double_decimal<'a>(d: f64, mut bytes: &'a mut [u8], format: NumberFormat) -> usize {
    let count = dtoa::write(&mut bytes, d).expect("Write to in-memory buffer.");
    replace(bytes, count, format);
    count
}
