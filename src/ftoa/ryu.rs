//! Wrapper around David Tolnay's ryu.

use ryu::raw;

// F32

/// Wrapper for ryu.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) unsafe extern "C" fn float_base10(f: f32, first: *mut u8)
    -> *mut u8
{
    // Not a public API, but we don't want the C-API.
    let len = raw::pretty_f2s_buffered_n(f, first);
    first.add(len)
}

// F64

/// Wrapper for ryu.
///
/// `d` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) unsafe extern "C" fn double_base10(d: f64, first: *mut u8)
    -> *mut u8
{
    // Not a public API, but we don't want the C-API.
    let len = raw::pretty_d2s_buffered_n(d, first);
    first.add(len)
}
