//! Utilities to generate the low-level API.

use lib::{self, slice};
use super::algorithm::distance;
use super::error::ErrorCode;
use super::num::Number;
use super::pointer_methods::PointerMethods;

// HELPERS

/// Convert a mutable pointer range to a mutable slice safely.
#[inline]
pub(crate) unsafe fn slice_from_range_mut<'a, T>(first: *mut T, last: *mut T)
    -> &'a mut [T]
{
    assert!(first <= last && !first.is_null() && !last.is_null());
    slice::from_raw_parts_mut(first, distance(first, last))
}

/// Map Result<(T, usize)> to Result<T>.
macro_rules! map_result {
    ($result:expr, $input_length:expr) => {
        match $result {
            Err(e)                  => Err(e),
            Ok((value, processed))  => if processed == $input_length {
                Ok(value)
            } else{
                Err((ErrorCode::InvalidDigit, processed).into())
            }
        }
    };
}

// TO BYTES WRAPPER

// Do not inline any of the API functions, both to preserve symbols, and
// to flatten the internal API into these symbols.

/// Macro to generate the low-level, FFI API using a pointer range.
#[doc(hidden)]
macro_rules! generate_from_range_api {
    (
        $decimal_name:ident, $radix_name:ident,
        $leading_decimal_name:ident, $leading_radix_name:ident,
        $t:ty, $cb:ident
    ) => (
        /// Checked parser for a string-to-number conversion using Rust pointer ranges.
        ///
        /// This method parses the entire string, returning an error if
        /// any invalid digits are found during parsing.
        ///
        /// Returns a C-compatible result containing the parsed value,
        /// and an error container any errors that occurred during parser.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `first`   - Pointer to the start of the input data.
        /// * `last`    - Pointer to the one-past-the-end of the input data.
        ///
        /// # Panics
        ///
        /// Panics if either pointer is null.
        #[no_mangle]
        pub unsafe extern fn $decimal_name(first: *const u8, last: *const u8)
            -> $crate::ffi::Result<$t>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            map_result!($cb(10, bytes), bytes.len()).into()
        }

        /// Checked parser for a string-to-number conversion using Rust pointer ranges.
        ///
        /// This method parses until an invalid digit is found (or the end
        /// of the string), returning the number of processed digits
        /// and the parsed value until that point.
        ///
        /// Returns a C-compatible result containing the parsed value,
        /// and an error container any errors that occurred during parser.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `first`   - Pointer to the start of the input data.
        /// * `last`    - Pointer to the one-past-the-end of the input data.
        ///
        /// # Panics
        ///
        /// Panics if either pointer is null.
        #[no_mangle]
        pub unsafe extern fn $leading_decimal_name(first: *const u8, last: *const u8)
            -> $crate::ffi::Result<$crate::ffi::Tuple<$t, usize>>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            match $cb(10, bytes) {
                Ok(v)  => Ok(v.into()),
                Err(e) => Err(e),
            }.into()
        }

        /// Checked parser for a string-to-number conversion using Rust pointer ranges.
        ///
        /// Returns a C-compatible result containing the parsed value,
        /// and an error container any errors that occurred during parser.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `radix`   - Radix for the number parsing.
        /// * `first`   - Pointer to the start of the input data.
        /// * `last`    - Pointer to the one-past-the-end of the input data.
        ///
        /// # Panics
        ///
        /// Panics if the radix is not in the range `[2, 36]`. Also panics
        /// if either pointer is null.
        #[cfg(feature = "radix")]
        #[no_mangle]
        pub unsafe extern fn $radix_name(radix: u8, first: *const u8, last: *const u8)
            -> $crate::ffi::Result<$t>
        {
            assert_radix!(radix);
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            map_result!($cb(radix.as_u32(), bytes), bytes.len()).into()
        }

        /// Checked parser for a string-to-number conversion using Rust pointer ranges.
        ///
        /// This method parses until an invalid digit is found (or the end
        /// of the string), returning the number of processed digits
        /// and the parsed value until that point.
        ///
        /// Returns a C-compatible result containing the parsed value,
        /// and an error container any errors that occurred during parser.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `radix`   - Radix for the number parsing.
        /// * `first`   - Pointer to the start of the input data.
        /// * `last`    - Pointer to the one-past-the-end of the input data.
        ///
        /// # Panics
        ///
        /// Panics if the radix is not in the range `[2, 36]`. Also panics
        /// if either pointer is null.
        #[cfg(feature = "radix")]
        #[no_mangle]
        pub unsafe extern fn $leading_radix_name(radix: u8, first: *const u8, last: *const u8)
            -> $crate::ffi::Result<$crate::ffi::Tuple<$t, usize>>
        {
            assert_radix!(radix);
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            match $cb(radix.as_u32(), bytes) {
                Ok(v)  => Ok(v.into()),
                Err(e) => Err(e),
            }.into()
        }
    )
}

/// Macro to generate the low-level, safe, parse API using a slice.
#[doc(hidden)]
macro_rules! generate_from_slice_api {
    (
        $decimal_name:ident, $radix_name:ident,
        $leading_decimal_name:ident, $leading_radix_name:ident,
        $t:ty, $cb:ident
    ) => (
        /// Checked parser for a string-to-number conversion using Rust slices.
        ///
        /// This method parses the entire string, returning an error if
        /// any invalid digits are found during parsing.
        ///
        /// Returns a `Result` containing either the parsed value,
        /// or an error container any errors that occurred during parsing.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `bytes`   - Slice containing a numeric string.
        #[inline]
        pub fn $decimal_name(bytes: &[u8])
            -> Result<$t>
        {
            map_result!($cb(10, bytes), bytes.len())
        }

        /// Checked parser for a string-to-number conversion using Rust slices.
        ///
        /// This method parses until an invalid digit is found (or the end
        /// of the string), returning the number of processed digits
        /// and the parsed value until that point.
        ///
        /// Returns a `Result` containing either the parsed value,
        /// or an error container any errors that occurred during parsing.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `bytes`   - Slice containing a numeric string.
        #[inline]
        pub fn $leading_decimal_name(bytes: &[u8])
            -> Result<($t, usize)>
        {
            $cb(10, bytes)
        }

        /// Checked parser for a string-to-number conversion using Rust slices.
        ///
        /// This method parses the entire string, returning an error if
        /// any invalid digits are found during parsing.
        ///
        /// Returns a `Result` containing either the parsed value,
        /// or an error container any errors that occurred during parsing.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `radix`   - Radix for the number parsing.
        /// * `bytes`   - Slice containing a numeric string.
        ///
        /// # Panics
        ///
        /// Panics if the radix is not in the range `[2, 36]`.
        #[cfg(feature = "radix")]
        #[inline]
        pub fn $radix_name(radix: u8, bytes: &[u8])
            -> Result<$t>
        {
            assert_radix!(radix);
            map_result!($cb(radix.as_u32(), bytes), bytes.len())
        }

        /// Checked parser for a string-to-number conversion using Rust slices.
        ///
        /// This method parses until an invalid digit is found (or the end
        /// of the string), returning the number of processed digits
        /// and the parsed value until that point.
        ///
        /// Returns a `Result` containing either the parsed value,
        /// or an error container any errors that occurred during parsing.
        ///
        /// Numeric overflow takes precedence over the presence of an invalid
        /// digit, and therefore may mask an invalid digit error.
        ///
        /// * `radix`   - Radix for the number parsing.
        /// * `bytes`   - Slice containing a numeric string.
        ///
        /// # Panics
        ///
        /// Panics if the radix is not in the range `[2, 36]`.
        #[cfg(feature = "radix")]
        #[inline]
        pub fn $leading_radix_name(radix: u8, bytes: &[u8])
            -> Result<($t, usize)>
        {
            assert_radix!(radix);
            $cb(radix.as_u32(), bytes)
        }
    )
}

// TO BYTES WRAPPER

/// Macro to generate the low-level, FFI, to_string API using a range.
#[doc(hidden)]
macro_rules! generate_to_range_api {
    ($decimal_name:ident, $radix_name:ident, $t:ty, $cb:ident, $size:ident) => (
        /// Serializer for a number-to-string conversion using pointer ranges.
        ///
        /// Returns a pointer to the 1-past-the-last-byte-written, so that
        /// the range `[first, last)` contains the written bytes. No
        /// null-terminator is written.
        ///
        /// The data in the range may be uninitialized, these values are
        /// never read, only written to.
        ///
        /// * `value`   - Number to serialize.
        /// * `first`   - Pointer to the start of the buffer to write to.
        /// * `last`    - Pointer to the one-past-the-end of the buffer to write to.
        ///
        /// # Panics
        ///
        /// Panics if the buffer is not of sufficient size, The caller
        /// must provide a range of sufficient size, and neither pointer
        /// may be null. In order to ensure the function will not panic,
        /// ensure the buffer has at least `MAX_*_SIZE` elements, using
        /// the proper constant for the serialized type from the
        /// lexical_core crate root.
        #[no_mangle]
        pub unsafe extern fn $decimal_name(value: $t, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            let bytes = $crate::util::api::slice_from_range_mut(first, last);

            assert_buffer!(10, bytes, $t);
            let len = $cb(value, 10, bytes);
            bytes.as_mut_ptr().padd(len)
        }

        /// Serializer for a number-to-string conversion using pointer ranges.
        ///
        /// Returns a pointer to the 1-past-the-last-byte-written, so that
        /// the range `[first, last)` contains the written bytes. No
        /// null-terminator is written.
        ///
        /// The data in the range may be uninitialized, these values are
        /// never read, only written to.
        ///
        /// * `value`   - Number to serialize.
        /// * `radix`   - Radix for number encoding.
        /// * `first`   - Pointer to the start of the buffer to write to.
        /// * `last`    - Pointer to the one-past-the-end of the buffer to write to.
        ///
        /// # Panics
        ///
        /// Panics if the radix is not in the range `[2, 36]`.
        ///
        /// Also panics if the buffer is not of sufficient size, The caller
        /// must provide a range of sufficient size, and neither pointer
        /// may be null. In order to ensure the function will not panic,
        /// ensure the buffer has at least `MAX_*_SIZE` elements, using
        /// the proper constant for the serialized type from the
        /// lexical_core crate root.
        #[cfg(feature = "radix")]
        #[no_mangle]
        pub unsafe extern fn $radix_name(value: $t, radix: u8, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            assert_radix!(radix);
            let bytes = $crate::util::api::slice_from_range_mut(first, last);

            assert_buffer!(radix, bytes, $t);
            let len = $cb(value, radix, bytes);
            bytes.as_mut_ptr().padd(len)
        }
    )
}

/// Macro to generate the low-level, safe, to_string API using a slice.
#[doc(hidden)]
macro_rules! generate_to_slice_api {
    ($decimal_name:ident, $radix_name:ident, $t:ty, $cb:ident, $size:ident) => (
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
        /// * `value`   - Number to serialize.
        /// * `bytes`   - Slice containing a numeric string.
        ///
        /// # Panics
        ///
        /// Panics if the buffer is not of sufficient size, The caller
        /// must provide a slice of sufficient size. In order to ensure
        /// the function will not panic, ensure the buffer has at least
        /// `MAX_*_SIZE` elements, using the proper constant for the
        /// serialized type from the lexical_core crate root.
        #[inline]
        pub fn $decimal_name<'a>(value: $t, bytes: &'a mut [u8])
            -> &'a mut [u8]
        {
            assert_buffer!(10, bytes, $t);
            let len = $cb(value, 10, bytes);
            &mut index_mut!(bytes[..len])
        }

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
        /// * `value`   - Number to serialize.
        /// * `radix`   - Radix for number encoding.
        /// * `bytes`   - Slice containing a numeric string.
        ///
        /// # Panics
        ///
        /// Panics if the radix is not in the range `[2, 36]`.
        ///
        /// Also panics if the buffer is not of sufficient size, The caller
        /// must provide a slice of sufficient size. In order to ensure
        /// the function will not panic, ensure the buffer has at least
        /// `MAX_*_SIZE` elements, using the proper constant for the
        /// serialized type from the lexical_core crate root.
        #[cfg(feature = "radix")]
        #[inline]
        pub fn $radix_name<'a>(value: $t, radix: u8, bytes: &'a mut [u8])
            -> &'a mut [u8]
        {
            assert_radix!(radix);
            assert_buffer!(radix, bytes, $t);
            // This is always safe, since len is returned as
            // `distance(bytes.as_ptr(), slc.as_ptr())`, where `slc` is
            // a subslice from writes.
            let len = $cb(value, radix, bytes);
            &mut index_mut!(bytes[..len])
        }
    )
}
