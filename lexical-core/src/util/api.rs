//! Utilities to generate the low-level API.

use lib;
use super::algorithm::distance;
use super::num::Number;
use super::result::*;

// TO BYTES WRAPPER

/// Wrap the unsafe API into the safe API parsing raw bytes.
#[doc(hidden)]
#[inline]
pub(crate) fn from_bytes_wrapper<T, Cb>(radix: u8, first: *const u8, last: *const u8, cb: Cb)
    -> T
    where Cb: FnOnce(u8, *const u8, *const u8) -> (T, *const u8, bool)
{
    let (value, _, _) = cb(radix, first, last);
    value
}

/// Generate local wrappers which falsely claim the function is safe.
///
/// Allows us to avoid macro magic and use FnOnce, don't export or expose
/// these functions.
#[doc(hidden)]
macro_rules! generate_from_bytes_local {
    ($name:ident, $t:ty, $cb:ident) => (
        #[inline]
        fn $name(radix: u8, first: *const u8, last: *const u8)
            -> ($t, *const u8, bool)
        {
            // This is the "choke-point", where it panics at runtime
            // if the radix is invalid.
            assert_radix!(radix);
            unsafe { $cb(radix, first, last) }
        }
    )
}

/// Macro to generate the low-level, FFI API using a pointer range.
#[doc(hidden)]
macro_rules! generate_from_range_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub unsafe extern "C" fn $name(radix: u8, first: *const u8, last: *const u8)
            -> $t
        {
            $crate::util::api::from_bytes_wrapper::<$t, _>(radix, first, last, $cb)
        }
    )
}

/// Macro to generate the low-level, safe, parse API using a slice.
#[doc(hidden)]
macro_rules! generate_from_slice_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub fn $name(radix: u8, bytes: &[u8])
            -> $t
        {
            unsafe {
                let first = bytes.as_ptr();
                let last = first.add(bytes.len());
                $crate::util::api::from_bytes_wrapper::<$t, _>(radix, first, last, $cb)
            }
        }
    )
}

/// Wrap the unsafe API into the safe, parse API trying to parse raw bytes.
#[doc(hidden)]
#[inline]
pub(crate) unsafe fn try_from_bytes_wrapper<T, Cb>(radix: u8, first: *const u8, last: *const u8, cb: Cb)
    -> Result<T>
    where T: Number,
          Cb: FnOnce(u8, *const u8, *const u8) -> (T, *const u8, bool)
{
    let (value, p, overflow) = cb(radix, first, last);
    if overflow {
        overflow_error(value)
    } else if p == last {
        success(value)
    } else {
        let dist = if p == lib::ptr::null() { 0 } else { distance(first, p) };
        invalid_digit_error(value, dist)
    }
}

/// Macro to generate the low-level, FFI, try_parse API using a pointer range.
#[doc(hidden)]
macro_rules! generate_try_from_range_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub unsafe extern "C" fn $name(radix: u8, first: *const u8, last: *const u8)
            -> Result<$t>
        {
            $crate::util::api::try_from_bytes_wrapper::<$t, _>(radix, first, last, $cb)
        }
    )
}

/// Macro to generate the low-level, safe, try_parse API using a slice.
#[doc(hidden)]
macro_rules! generate_try_from_slice_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub fn $name(radix: u8, bytes: &[u8])
            -> Result<$t>
        {
            unsafe {
                let first = bytes.as_ptr();
                let last = first.add(bytes.len());
                $crate::util::api::try_from_bytes_wrapper::<$t, _>(radix, first, last, $cb)
            }
        }
    )
}

// TO BYTES WRAPPER

/// Generate local wrappers which falsely claim the function is safe.
///
/// Allows us to avoid macro magic and use FnOnce, don't export or expose
/// these functions.
#[doc(hidden)]
macro_rules! generate_to_bytes_local {
    ($name:ident, $t:ty, $cb:ident) => (
        #[inline]
        fn $name(value: $t, radix: u8, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            // This is the "choke-point", where it panics at runtime
            // if the radix is invalid.
            assert_radix!(radix);
            unsafe { $cb(value, radix, first, last) }
        }
    )
}

/// Macro to generate the low-level, FFI, to_string API using a range.
#[doc(hidden)]
macro_rules! generate_to_range_api {
    ($name:ident, $t:ty, $cb:ident) => (
        #[inline]
        pub unsafe extern "C" fn $name(value: $t, radix: u8, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            $cb(value, radix, first, last)
        }
    )
}

/// Macro to generate the low-level, safe, to_string API using a slice.
#[doc(hidden)]
macro_rules! generate_to_slice_api {
    ($name:ident, $t:ty, $cb:ident) => (
        #[inline]
        pub fn $name<'a>(value: $t, radix: u8, bytes: &mut [u8])
            -> &'a mut [u8]
        {
            unsafe {
                let first = bytes.as_mut_ptr();
                let last = first.add(bytes.len());
                let last = $cb(value, radix, first, last);
                $crate::lib::slice::from_raw_parts_mut(first, distance(first, last))
            }
        }
    )
}
