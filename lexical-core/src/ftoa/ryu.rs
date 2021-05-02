//! Wrapper around David Tolnay's ryu.

use ryu::raw;
use crate::util::*;

use super::replace::replace;

// F32

/// Wrapper for ryu.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline]
pub(crate) fn float_decimal<'a>(f: f32, bytes: &'a mut [u8], format: NumberFormat)
    -> usize
{
    let count = unsafe {
        raw::format32(f, bytes.as_mut_ptr())
    };
    replace(bytes, count, format);
    count
}

// F64

/// Wrapper for ryu.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline]
pub(crate) fn double_decimal<'a>(d: f64, bytes: &'a mut [u8], format: NumberFormat)
    -> usize
{
    let count = unsafe {
        raw::format64(d, bytes.as_mut_ptr())
    };
    replace(bytes, count, format);
    count
}
