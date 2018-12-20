//! Wrapper around David Tolnay's ryu.

use ryu::raw;

// F32

/// Wrapper for ryu.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) fn float_decimal(f: f32, bytes: &mut [u8])
    -> usize
{
    unsafe {
        raw::pretty_f2s_buffered_n(f, bytes.as_mut_ptr())
    }
}

// F64

/// Wrapper for ryu.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) fn double_decimal(d: f64, bytes: &mut [u8])
    -> usize
{
    unsafe {
        raw::pretty_d2s_buffered_n(d, bytes.as_mut_ptr())
    }
}
