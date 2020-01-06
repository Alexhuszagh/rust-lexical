// C-compatible API for lexical conversion routines.

use lib::slice;
use lexical_core;

// HELPERS

/// Calculate the difference between two pointers.
#[inline]
pub fn distance<T>(first: *const T, last: *const T)
    -> usize
{
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

/// Convert a mutable pointer range to a mutable slice safely.
#[inline]
pub(crate) unsafe fn slice_from_range_mut<'a, T>(first: *mut T, last: *mut T)
    -> &'a mut [T]
{
    assert!(first <= last && !first.is_null() && !last.is_null());
    slice::from_raw_parts_mut(first, distance(first, last))
}

// FROM LEIXCAL

// Macro to generate the decimal, complete parser from a pointer range.
macro_rules! decimal_from_range {
    ($name:ident, $cb:ident, $t:ty) => (
        /// Checked parser for a string-to-number conversions.
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
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $name(first: *const u8, last: *const u8)
            -> $crate::result::Result<$t>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            lexical_core::$cb(bytes).into()
        }
    );
}

// Macro to generate the decimal, partial parser from a pointer range.
macro_rules! partial_decimal_from_range {
    ($name:ident, $cb:ident, $t:ty) => (
        /// Checked parser for a string-to-number conversions.
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
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $name(first: *const u8, last: *const u8)
            -> $crate::result::Result<$crate::result::Tuple<$t, usize>>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            match lexical_core::$cb(bytes) {
                Ok(v)  => Ok(v.into()),
                Err(e) => Err(e),
            }.into()
        }
    );
}

// Macro to generate the radix, complete parser from a pointer range.
macro_rules! radix_from_range {
    ($name:ident, $cb:ident, $t:ty) => (
        /// Checked parser for a string-to-number conversions.
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
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $name(first: *const u8, last: *const u8, radix: u8)
            -> $crate::result::Result<$t>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            lexical_core::$cb(bytes,radix).into()
        }
    );
}

// Macro to generate the radix, partial parser from a pointer range.
macro_rules! partial_radix_from_range {
    ($name:ident, $cb:ident, $t:ty) => (
        /// Checked parser for a string-to-number conversions.
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
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $name(first: *const u8, last: *const u8, radix: u8)
            -> $crate::result::Result<$crate::result::Tuple<$t, usize>>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            match lexical_core::$cb(bytes, radix) {
                Ok(v)  => Ok(v.into()),
                Err(e) => Err(e),
            }.into()
        }
    );
}

// Macro to generate parsers implementing the FromLexical trait.
macro_rules! from_lexical {
    (
        $decimal_name:ident, $partial_decimal_name:ident,
        $radix_name:ident, $partial_radix_name:ident, $t:ty
    ) => (
        decimal_from_range!($decimal_name, parse, $t);
        partial_decimal_from_range!($partial_decimal_name, parse_partial, $t);
        radix_from_range!($radix_name, parse_radix, $t);
        partial_radix_from_range!($partial_radix_name, parse_partial_radix, $t);
    );
}

// Macro to generate parsers implementing the FromLexicalLossy trait.
macro_rules! from_lexical_lossy {
    (
        $decimal_name:ident, $partial_decimal_name:ident,
        $radix_name:ident, $partial_radix_name:ident, $t:ty
    ) => (
        decimal_from_range!($decimal_name, parse_lossy, $t);
        partial_decimal_from_range!($partial_decimal_name, parse_partial_lossy, $t);
        radix_from_range!($radix_name, parse_lossy_radix, $t);
        partial_radix_from_range!($partial_radix_name, parse_partial_lossy_radix, $t);
    );
}

// TO LEXICAL

/// Macro to generate the decimal to_string API using a range.
macro_rules! decimal_to_range {
    ($name:ident, $cb:ident, $t:ty) => (
        /// Serializer for a number-to-string conversions.
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
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $name(value: $t, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            let bytes = $crate::api::slice_from_range_mut(first, last);
            let slc = lexical_core::$cb(value, bytes);
            let len = slc.len();
            slc[len..].as_mut_ptr()
        }
    );
}

/// Macro to generate the radix to_string API using a range.
macro_rules! radix_to_range {
    ($name:ident, $cb:ident, $t:ty) => (
        /// Serializer for a number-to-string conversions.
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
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern fn $name(value: $t, radix: u8, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            let bytes = $crate::api::slice_from_range_mut(first, last);
            let slc = lexical_core::$cb(value, radix, bytes);
            let len = slc.len();
            slc[len..].as_mut_ptr()
        }
    );
}

// Macro to generate serializers implementing the ToLexical trait.
macro_rules! to_lexical {
    ($decimal_name:ident, $radix_name:ident, $t:ty) => (
        decimal_to_range!($decimal_name, write, $t);
        radix_to_range!($radix_name, write_radix, $t);
    );
}

// API

// ATOF
from_lexical!(lexical_atof32, lexical_atof32_partial, lexical_atof32_radix, lexical_atof32_partial_radix, f32);
from_lexical!(lexical_atof64, lexical_atof64_partial, lexical_atof64_radix, lexical_atof64_partial_radix, f64);
from_lexical_lossy!(lexical_atof32_lossy, lexical_atof32_partial_lossy, lexical_atof32_lossy_radix, lexical_atof32_partial_lossy_radix, f32);
from_lexical_lossy!(lexical_atof64_lossy, lexical_atof64_partial_lossy, lexical_atof64_lossy_radix, lexical_atof64_partial_lossy_radix, f64);

// ATOI
from_lexical!(lexical_atou8, lexical_atou8_partial, lexical_atou8_radix, lexical_atou8_partial_radix, u8);
from_lexical!(lexical_atou16, lexical_atou16_partial, lexical_atou16_radix, lexical_atou16_partial_radix, u16);
from_lexical!(lexical_atou32, lexical_atou32_partial, lexical_atou32_radix, lexical_atou32_partial_radix, u32);
from_lexical!(lexical_atou64, lexical_atou64_partial, lexical_atou64_radix, lexical_atou64_partial_radix, u64);
from_lexical!(lexical_atousize, lexical_atousize_partial, lexical_atousize_radix, lexical_atousize_partial_radix, usize);
from_lexical!(lexical_atou128, lexical_atou128_partial, lexical_atou128_radix, lexical_atou128_partial_radix, u128);

from_lexical!(lexical_atoi8, lexical_atoi8_partial, lexical_atoi8_radix, lexical_atoi8_partial_radix, i8);
from_lexical!(lexical_atoi16, lexical_atoi16_partial, lexical_atoi16_radix, lexical_atoi16_partial_radix, i16);
from_lexical!(lexical_atoi32, lexical_atoi32_partial, lexical_atoi32_radix, lexical_atoi32_partial_radix, i32);
from_lexical!(lexical_atoi64, lexical_atoi64_partial, lexical_atoi64_radix, lexical_atoi64_partial_radix, i64);
from_lexical!(lexical_atoisize, lexical_atoisize_partial, lexical_atoisize_radix, lexical_atoisize_partial_radix, isize);
from_lexical!(lexical_atoi128, lexical_atoi128_partial, lexical_atoi128_radix, lexical_atoi128_partial_radix, i128);

// FTOA
to_lexical!(lexical_f32toa, lexical_f32toa_radix, f32);
to_lexical!(lexical_f64toa, lexical_f64toa_radix, f64);

// ITOA
to_lexical!(lexical_u8toa, lexical_u8toa_radix, u8);
to_lexical!(lexical_u16toa, lexical_u16toa_radix, u16);
to_lexical!(lexical_u32toa, lexical_u32toa_radix, u32);
to_lexical!(lexical_u64toa, lexical_u64toa_radix, u64);
to_lexical!(lexical_usizetoa, lexical_usizetoa_radix, usize);
to_lexical!(lexical_u128toa, lexical_u128toa_radix, u128);

to_lexical!(lexical_i8toa, lexical_i8toa_radix, i8);
to_lexical!(lexical_i16toa, lexical_i16toa_radix, i16);
to_lexical!(lexical_i32toa, lexical_i32toa_radix, i32);
to_lexical!(lexical_i64toa, lexical_i64toa_radix, i64);
to_lexical!(lexical_isizetoa, lexical_isizetoa_radix, isize);
to_lexical!(lexical_i128toa, lexical_i128toa_radix, i128);
