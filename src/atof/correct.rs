//! Moderately fast, correct base10 lexical string-to-float conversion routines.
// TODO(ahuszagh) Add documentation

use super::algorithm::correct::{atod, atof};

// F32

/// Import float from basen, using a correct algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[cfg(feature = "correct")]
pub(crate) unsafe extern "C" fn float_basen(first: *const u8, last: *const u8, base: u64)
    -> (f32, *const u8)
{
    atof(first, last, base)
}

/// Import float from base10, using a correct algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[cfg(feature = "correct")]
pub(crate) unsafe extern "C" fn float_base10(first: *const u8, last: *const u8)
    -> (f32, *const u8)
{
    atof(first, last, 10)
}

// F64

/// Import double from basen, using a correct algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[cfg(feature = "correct")]
pub(crate) unsafe extern "C" fn double_basen(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    atod(first, last, base)
}

/// Import double from base10, using a correct algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[cfg(feature = "correct")]
pub(crate) unsafe extern "C" fn double_base10(first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    atod(first, last, 10)
}
