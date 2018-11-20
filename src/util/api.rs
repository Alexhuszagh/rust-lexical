//! Utilities to generate the low-level API.

use error::{Error, ErrorKind};
use lib;
use super::algorithm::distance;

// TO BYTES WRAPPER

/// Wrap the unsafe API into the safe API parsing raw bytes.
#[doc(hidden)]
#[inline]
pub(crate) fn from_bytes_wrapper<T, Cb>(base: u8, bytes: &[u8], cb: Cb)
    -> T
    where Cb: FnOnce(u8, *const u8, *const u8) -> (T, *const u8, bool)
{
    unsafe {
        let first = bytes.as_ptr();
        let last = first.add(bytes.len());
        let (value, _, _) = cb(base, first, last);
        value
    }
}

/// Generate local wrappers which falsely claim the function is safe.
///
/// Allows us to avoid macro magic and use FnOnce, don't export or expose
/// these functions.
#[doc(hidden)]
macro_rules! generate_from_bytes_local {
    ($name:ident, $t:ty, $cb:ident) => (
        #[inline]
        fn $name(base: u8, first: *const u8, last: *const u8)
            -> ($t, *const u8, bool)
        {
            unsafe { $cb(base, first, last) }
        }
    )
}

/// Macro to facilitate exporting the from_bytes wrappers.
#[doc(hidden)]
macro_rules! generate_from_bytes_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub fn $name(base: u8, bytes: &[u8]) -> $t
        {
            $crate::util::api::from_bytes_wrapper::<$t, _>(base, bytes, $cb)
        }
    )
}

/// Wrap the unsafe API into the safe API trying to parse raw bytes.
#[doc(hidden)]
#[inline]
pub(crate) fn try_from_bytes_wrapper<T, Cb>(base: u8, bytes: &[u8], cb: Cb)
    -> Result<T, Error>
    where Cb: FnOnce(u8, *const u8, *const u8) -> (T, *const u8, bool)
{
    unsafe {
        let first = bytes.as_ptr();
        let last = first.add(bytes.len());
        let (value, p, overflow) = cb(base, first, last);
        if overflow {
            Err(From::from(ErrorKind::Overflow))
        } else if p == last {
            Ok(value)
        } else {
            let dist = if p == lib::ptr::null() { 0 } else { distance(first, p) };
            Err(From::from(ErrorKind::InvalidDigit(dist)))
        }
    }
}

/// Macro to facilitate exporting the try_from_bytes wrappers.
#[doc(hidden)]
macro_rules! generate_try_from_bytes_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub fn $name(base: u8, bytes: &[u8]) -> Result<$t, $crate::Error>
        {
            $crate::util::api::try_from_bytes_wrapper::<$t, _>(base, bytes, $cb)
        }
    )
}

// TO BYTES WRAPPER

/// Wrap the unsafe API into the safe API exporting raw bytes.
#[doc(hidden)]
#[inline]
pub(crate) fn to_bytes_wrapper<T, Cb>(value: T, base: u8, capacity: usize, cb: Cb)
    -> lib::Vec<u8>
    where Cb: FnOnce(T, u8, *mut u8, *mut u8) -> *mut u8
{
    let mut buf = lib::Vec::<u8>::with_capacity(capacity);
    unsafe {
        let first: *mut u8 = buf.as_mut_ptr();
        let last = first.add(buf.capacity());
        let end = cb(value, base, first, last);
        let size = distance(first, end);
        buf.set_len(size);

    }
    buf
}

/// Generate local wrappers which falsely claim the function is safe.
///
/// Allows us to avoid macro magic and use FnOnce, don't export or expose
/// these functions.
#[doc(hidden)]
macro_rules! generate_to_bytes_local {
    ($name:ident, $t:ty, $cb:ident) => (
        #[inline]
        fn $name(value: $t, base: u8, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            unsafe { $cb(value, base, first, last) }
        }
    )
}

/// Macro to facilitate exporting the to_bytes wrappers.
#[doc(hidden)]
macro_rules! generate_to_bytes_api {
    ($name:ident, $t:ty, $cb:ident, $capacity:expr) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub fn $name(value: $t, base: u8)
            -> $crate::lib::Vec<u8>
        {
            $crate::util::api::to_bytes_wrapper(value, base, $capacity, $cb)
        }
    )
}
