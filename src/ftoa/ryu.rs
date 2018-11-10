//! Wrapper around David Tolnay's ryu.

use ryu::Float;

// F32

/// Wrapper for ryu.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
pub(crate) unsafe extern "C" fn float_base10(f: f32, first: *mut u8)
    -> *mut u8
{
    // Not a public API, but we don't want the C-API.
    let len = f.write_to_ryu_buffer(first);
    first.offset(len as isize)
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
    let len = d.write_to_ryu_buffer(first);
    first.offset(len as isize)
}
