//! Config settings for lexical-capi.

use lib::slice;
use lexical_core;

// FFI

// FUNCTIONS

/// Helper to get a pointer and size from a number literal slice.
macro_rules! get_string {
    ($ptr:ident, $size:ident, $cb:ident) => {
        if $ptr.is_null() || $size.is_null() {
            -1
        } else {
            let slc = lexical_core::$cb();
            *$ptr = slc.as_ptr();
            *$size = slc.len();
            0
        }
    };
}

/// Helper to set a number literal slice from a pointer and size.
macro_rules! set_string {
    ($ptr:ident, $size:ident, $cb:ident) => {
        if $ptr.is_null() {
            -1
        } else {
            let slc = slice::from_raw_parts($ptr, $size);
            lexical_core::$cb(slc);
            0
        }
    };
}

/// Get default character for the exponent symbol.
///
/// Default character for scientific notation, used when the radix < 15.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_get_exponent_default_char() -> u8
{
    lexical_core::get_exponent_default_char()
}

/// Set the default character for the exponent symbol.
///
/// Default character for scientific notation, used when the radix < 15.
///
/// To change the expected, default character for an exponent,
/// change this value before using lexical.
///
/// * `ch`      - Character for exponent symbol.
///
/// # Safety
///
/// Do not call this function in threaded-code, as it is not thread-safe.
///
/// # Panics
///
/// Panics if the character is in the character set `[A-Da-d.+\-]`.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_set_exponent_default_char(ch: u8)
{
    lexical_core::set_exponent_default_char(ch)
}

/// Get backup character for the exponent symbol.
///
/// For numerical strings of radix >= 15, 'e' or 'E' is a valid digit,
/// and therefore may no longer be used as a marker for the exponent.
#[cfg(feature ="radix")]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_get_exponent_backup_char() -> u8
{
    lexical_core::get_exponent_backup_char()
}

/// Set the backup character for the exponent symbol.
///
/// For numerical strings of radix >= 15, 'e' or 'E' is a valid digit,
/// and therefore may no longer be used as a marker for the exponent.
///
/// To change the expected, backup character for an exponent,
/// change this value before using lexical.
///
/// * `ch`      - Character for exponent symbol.
///
/// # Safety
///
/// Do not call this function in threaded-code, as it is not thread-safe.
///
/// # Panics
///
/// Panics if the character is in the character set `[A-Za-z.+\-]`.
#[cfg(feature ="radix")]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_set_exponent_backup_char(ch: u8)
{
    lexical_core::set_exponent_backup_char(ch)
}

/// Get the default rounding scheme for float conversions.
///
/// This defines the global rounding-scheme for float parsing operations.
/// By default, this is set to `RoundingKind::NearestTieEven`. IEEE754
/// recommends this as the default for all for decimal and binary
/// operations.
#[cfg(feature = "rounding")]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_get_float_rounding() -> lexical_core::RoundingKind
{
    lexical_core::get_float_rounding()
}

/// Set the default rounding scheme for float conversions.
///
/// This defines the global rounding-scheme for float parsing operations.
/// By default, this is set to `RoundingKind::NearestTieEven`. IEEE754
/// recommends this as the default for all for decimal and binary
/// operations.
///
/// # Safety
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
#[cfg(feature = "rounding")]
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_set_float_rounding(rounding: lexical_core::RoundingKind)
{
    lexical_core::set_float_rounding(rounding)
}

/// Get string representation of Not a Number as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_get_nan_string(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    get_string!(ptr, size, get_nan_string)
}

/// Set representation of Not a Number from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_set_nan_string(ptr: *const u8, size: usize)
    -> i32
{
    set_string!(ptr, size, set_nan_string)
}

/// Get the short representation of an Infinity literal as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_get_inf_string(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    get_string!(ptr, size, get_inf_string)
}

/// Set the short representation of Infinity from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_set_inf_string(ptr: *const u8, size: usize)
    -> i32
{
    set_string!(ptr, size, set_inf_string)
}

/// Get the long representation of an Infinity literal as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_get_infinity_string(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    get_string!(ptr, size, get_infinity_string)
}

/// Set the long representation of Infinity from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern fn lexical_set_infinity_string(ptr: *const u8, size: usize)
    -> i32
{
    set_string!(ptr, size, set_infinity_string)
}

// CONSTANTS

use lexical_core::Number;

/// Maximum number of bytes required to serialize an `i8` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I8_FORMATTED_SIZE: usize = i8::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `i16` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I16_FORMATTED_SIZE: usize = i16::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `i32` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I32_FORMATTED_SIZE: usize = i32::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `i64` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I64_FORMATTED_SIZE: usize = i64::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `i128` value to string.
#[cfg(has_i128)]
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I128_FORMATTED_SIZE: usize = i128::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `isize` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_ISIZE_FORMATTED_SIZE: usize = isize::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u8` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U8_FORMATTED_SIZE: usize = u8::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u16` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U16_FORMATTED_SIZE: usize = u16::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u32` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U32_FORMATTED_SIZE: usize = u32::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u64` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U64_FORMATTED_SIZE: usize = u64::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u128` value to string.
#[cfg(has_i128)]
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U128_FORMATTED_SIZE: usize = u128::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `usize` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_USIZE_FORMATTED_SIZE: usize = usize::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `f32` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_F32_FORMATTED_SIZE: usize = f32::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `f64` value to string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_F64_FORMATTED_SIZE: usize = f64::FORMATTED_SIZE;

// FFI DECIMAL

/// Maximum number of bytes required to serialize an `i8` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I8_FORMATTED_SIZE_DECIMAL: usize = i8::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `i16` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I16_FORMATTED_SIZE_DECIMAL: usize = i16::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `i32` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I32_FORMATTED_SIZE_DECIMAL: usize = i32::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `i64` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I64_FORMATTED_SIZE_DECIMAL: usize = i64::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `i128` value to a decimal string.
#[cfg(has_i128)]
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_I128_FORMATTED_SIZE_DECIMAL: usize = i128::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `isize` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL: usize = isize::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u8` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U8_FORMATTED_SIZE_DECIMAL: usize = u8::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u16` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U16_FORMATTED_SIZE_DECIMAL: usize = u16::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u32` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U32_FORMATTED_SIZE_DECIMAL: usize = u32::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u64` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U64_FORMATTED_SIZE_DECIMAL: usize = u64::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u128` value to a decimal string.
#[cfg(has_i128)]
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_U128_FORMATTED_SIZE_DECIMAL: usize = u128::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `usize` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL: usize = usize::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `f32` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_F32_FORMATTED_SIZE_DECIMAL: usize = f32::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `f64` value to a decimal string.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_F64_FORMATTED_SIZE_DECIMAL: usize = f64::FORMATTED_SIZE_DECIMAL;

// FFI BUFFER SIZE

/// Symbol-generating constant for the maximum number of bytes that any number-to-string function may write.
#[doc(hidden)]
#[no_mangle]
pub static LEXICAL_BUFFER_SIZE: usize = lexical_core::BUFFER_SIZE;
