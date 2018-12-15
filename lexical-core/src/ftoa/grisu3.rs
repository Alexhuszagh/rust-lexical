//! Wrapper around David Tolnay's dtoa.

use dtoa;
use lib::slice;
use util::*;

// F32

/// Wrapper for dtoa.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) unsafe extern "C" fn float_decimal(f: f32, first: *mut u8)
    -> *mut u8
{
    let mut s = slice::from_raw_parts_mut(first, BUFFER_SIZE);
    let len = dtoa::write(&mut s, f).expect("Write to in-memory buffer.");
    first.add(len)
}

// F64

/// Wrapper for dtoa.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) unsafe extern "C" fn double_decimal(d: f64, first: *mut u8)
    -> *mut u8
{
    let mut s = slice::from_raw_parts_mut(first, BUFFER_SIZE);
    let len = dtoa::write(&mut s, d).expect("Write to in-memory buffer.");
    first.add(len)
}
