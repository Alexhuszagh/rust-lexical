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
        /// Unchecked parser for a string-to-number conversion using pointer ranges.
        ///
        /// Returns the parsed value, ignoring any trailing invalid digits,
        /// and explicitly wrapping on arithmetic overflow.
        ///
        /// This parser is FFI-compatible, and therefore may be called externally
        /// from C code.
        ///
        /// * `radix`   - Radix for the number parsing (normally 10).
        /// * `first`   - Pointer to the start of the input data.
        /// * `last`    - Pointer to the one-past-the-end of the input data.
        ///
        /// # Panics
        ///
        /// If the `radix` feature is enabled, panics if radix is not in
        /// the range `[2, 36]`. If the `radix` feature is not enabled,
        /// panics if `radix != 10`.
        ///
        /// Also panics if either pointer is null.
        #[inline]
        pub unsafe extern "C" fn $name(radix: u8, first: *const u8, last: *const u8)
            -> $t
        {
            assert!(!first.is_null() || !last.is_null());
            $crate::util::api::from_bytes_wrapper::<$t, _>(radix, first, last, $cb)
        }
    )
}

/// Macro to generate the low-level, safe, parse API using a slice.
#[doc(hidden)]
macro_rules! generate_from_slice_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Unchecked parser for a string-to-number conversion using Rust slices.
        ///
        /// Returns the parsed value, ignoring any trailing invalid digits,
        /// and explicitly wrapping on arithmetic overflow.
        ///
        /// * `radix`   - Radix for the number parsing (normally 10).
        /// * `bytes`   - Slice containing a numeric string.
        ///
        /// # Panics
        ///
        /// If the `radix` feature is enabled, panics if radix is not in
        /// the range `[2, 36]`. If the `radix` feature is not enabled,
        /// panics if `radix != 10`.
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
        /// Checked parser for a string-to-number conversion using Rust pointer ranges.
        ///
        /// Returns a C-compatible result containing the parsed value,
        /// and an error container any errors that occurred during parser.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `radix`   - Radix for the number parsing (normally 10).
        /// * `first`   - Pointer to the start of the input data.
        /// * `last`    - Pointer to the one-past-the-end of the input data.
        ///
        /// # Panics
        ///
        /// If the `radix` feature is enabled, panics if radix is not in
        /// the range `[2, 36]`. If the `radix` feature is not enabled,
        /// panics if `radix != 10`.
        ///
        /// Also panics if either pointer is null.
        #[inline]
        pub unsafe extern "C" fn $name(radix: u8, first: *const u8, last: *const u8)
            -> Result<$t>
        {
            assert!(!first.is_null() || !last.is_null());
            $crate::util::api::try_from_bytes_wrapper::<$t, _>(radix, first, last, $cb)
        }
    )
}

/// Macro to generate the low-level, safe, try_parse API using a slice.
#[doc(hidden)]
macro_rules! generate_try_from_slice_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Checked parser for a string-to-number conversion using Rust slices.
        ///
        /// Returns a C-compatible result containing the parsed value,
        /// and an error container any errors that occurred during parser.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `radix`   - Radix for the number parsing (normally 10).
        /// * `bytes`   - Slice containing a numeric string.
        ///
        /// # Panics
        ///
        /// If the `radix` feature is enabled, panics if radix is not in
        /// the range `[2, 36]`. If the `radix` feature is not enabled,
        /// panics if `radix != 10`.
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
        /// Serializer for a number-to-string conversion using pointer ranges.
        ///
        /// Returns a pointer to the 1-past-the-last-byte-written, so that
        /// the range `[first, last)` contains the written bytes. No
        /// null-terminator is written.
        ///
        /// The data in the range may be uninitialized, these values are
        /// never read, only written to.
        ///
        /// * `radix`   - Radix for the number parsing (normally 10).
        /// * `first`   - Pointer to the start of the buffer to write to.
        /// * `last`    - Pointer to the one-past-the-end of the buffer to write to.
        ///
        /// # Panics
        ///
        /// If the `radix` feature is enabled, panics if radix is not in
        /// the range `[2, 36]`. If the `radix` feature is not enabled,
        /// panics if `radix != 10`.
        ///
        /// Also panics if the buffer is not of sufficient size, The caller
        /// must provide a range of sufficient size, and neither pointer
        /// may be null. In order to ensure the function will not panic,
        /// ensure the buffer has at least `MAX_*_SIZE` elements, using
        /// the proper constant for the serialized type from the
        /// lexical_core crate root.
        #[inline]
        pub unsafe extern "C" fn $name(value: $t, radix: u8, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            assert!(!first.is_null() || !last.is_null());
            $cb(value, radix, first, last)
        }
    )
}

/// Macro to generate the low-level, safe, to_string API using a slice.
#[doc(hidden)]
macro_rules! generate_to_slice_api {
    ($name:ident, $t:ty, $cb:ident) => (
        /// Serializer for a number-to-string conversion using Rust slices.
        ///
        /// Returns a subslice of the input buffer containing the written bytes,
        /// starting from the same address in memory as the input slice.
        ///
        /// If the buffer is not of sufficient size (see the constants
        /// named `MAX_*_SIZE` in the lexical_core crate), this function
        /// will panic (and call abort). You must provide a slice
        /// of sufficient length. The data in the slice may be
        /// uninitialized, these values are never read, only written to.
        ///
        /// * `radix`   - Radix for the number parsing (normally 10).
        /// * `bytes`   - Slice containing a numeric string.
        ///
        /// # Panics
        ///
        /// If the `radix` feature is enabled, panics if radix is not in
        /// the range `[2, 36]`. If the `radix` feature is not enabled,
        /// panics if `radix != 10`.
        ///
        /// Also panics if the buffer is not of sufficient size, The caller
        /// must provide a slice of sufficient size. In order to ensure
        /// the function will not panic, ensure the buffer has at least
        /// `MAX_*_SIZE` elements, using the proper constant for the
        /// serialized type from the lexical_core crate root.
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
