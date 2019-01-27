//! Config settings for lexical.

use lib::slice;
use super::algorithm::copy_to_dst;
use super::rounding::RoundingKind;

// HELPERS

/// Fixed-size string for float configurations.
///
/// These values are guaranteed less than or equal to the maximum
/// number of bytes after a sign byte has been written.
pub(crate) struct FloatConfigString {
    /// Storage data for the config string.
    data: [u8; MAX_F32_SIZE - 1],
    /// Actual length of the data.
    length: usize,
}

impl FloatConfigString {
    /// Reset data from a byte string.
    pub(crate) fn load_bytes(&mut self, bytes: &[u8]) {
        assert!(bytes.len() <= self.data.len());
        copy_to_dst(&mut self.data, bytes);
        self.length = bytes.len();
    }

    /// Convert to byte slice.
    pub(crate) fn as_bytes(&self) -> &[u8] {
        // Always safe, since length can only be set from `load_bytes`.
        unsafe {
            self.data.get_unchecked(..self.length)
        }
    }
}

// GLOBALS

cfg_if! {
if #[cfg(feature = "radix")] {
    /// Not a Number literal.
    static mut NAN_STRING: FloatConfigString = FloatConfigString {
        // b"NaN"
        data: [b'N', b'a', b'N', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'],
        length: 3
    };

    /// Short infinity literal.
    static mut INF_STRING: FloatConfigString = FloatConfigString {
        // b"inf"
        data: [b'i', b'n', b'f', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'],
        length: 3
    };

    /// Long infinity literal.
    static mut INFINITY_STRING: FloatConfigString = FloatConfigString {
        // b"infinity"
        data: [b'i', b'n', b'f', b'i', b'n', b'i', b't', b'y', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'],
        length: 8
    };
} else {
    /// Not a Number literal.
    static mut NAN_STRING: FloatConfigString = FloatConfigString {
        // b"NaN"
        data: [b'N', b'a', b'N', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'],
        length: 3
    };

    /// Short infinity literal.
    static mut INF_STRING: FloatConfigString = FloatConfigString {
        // b"inf"
        data: [b'i', b'n', b'f', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'],
        length: 3
    };

    /// Long infinity literal.
    static mut INFINITY_STRING: FloatConfigString = FloatConfigString {
        // b"infinity"
        data: [b'i', b'n', b'f', b'i', b'n', b'i', b't', b'y', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'],
        length: 8
    };
}}   // cfg_if

/// Default character for scientific notation, used when the radix < 15.
///
/// To change the expected, default character for an exponent,
/// change this value during before using lexical.
///
/// # Safety
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
#[no_mangle]
pub static mut EXPONENT_DEFAULT_CHAR: u8 = b'e';

/// Backup character for scientific notation, used when the radix >= 15.
///
/// For numerical strings of radix >= 15, 'e' or 'E' is a valid digit,
/// and therefore may no longer be used as a marker for the exponent.
///
/// To change the expected, default character for an exponent,
/// change this value during before using lexical.
///
/// # Safety
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
#[cfg(feature ="radix")]
#[no_mangle]
pub static mut EXPONENT_BACKUP_CHAR: u8 = b'^';

// GETTERS/SETTERS

// We need both C-FFI compatible getters and setters, and non-FFI ones,
// since we don't want to publicly expose the FloatConfigString type.

/// Helper to get a pointer and size from a FloatConfigString.
#[inline]
unsafe extern fn get_string_ffi(ptr: *mut *const u8, size: *mut usize, string: &'static FloatConfigString)
    -> i32
{
    if ptr.is_null() || size.is_null() {
        -1
    } else {
        let slc = string.as_bytes();
        *ptr = slc.as_ptr();
        *size = slc.len();
        0
    }
}

/// Helper to set a FloatConfigString from a pointer and size.
#[inline]
unsafe extern fn set_string_ffi(ptr: *const u8, size: usize, string: &'static mut FloatConfigString)
    -> i32
{
    if ptr.is_null() {
        -1
    } else {
        string.load_bytes(slice::from_raw_parts(ptr, size));
        0
    }
}

/// Get string representation of Not a Number as a byte slice.
#[inline]
pub fn get_nan_string() -> &'static [u8]
{
    unsafe {
        NAN_STRING.as_bytes()
    }
}

/// Get string representation of Not a Number as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you use [`get_nan_string`].
///
/// [`get_nan_string`]: fn.get_nan_string.html
#[no_mangle]
pub unsafe extern fn get_nan_string_ffi(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    get_string_ffi(ptr, size, &NAN_STRING)
}

/// Set representation of Not a Number from a byte slice.
///
/// * `bytes`    - Slice of bytes to assign as NaN string representation.
///
/// # Safety
///
/// Do not call this function in threaded-code, as it is not thread-safe.
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[inline]
pub unsafe fn set_nan_string(bytes: &[u8])
{
    NAN_STRING.load_bytes(bytes);
}

/// Set representation of Not a Number from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you use [`set_nan_string`].
/// Do not call this function in threaded-code, as it is not thread-safe.
///
/// [`set_nan_string`]: fn.set_nan_string.html
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[no_mangle]
pub unsafe extern fn set_nan_string_ffi(ptr: *const u8, size: usize)
    -> i32
{
    set_string_ffi(ptr, size, &mut NAN_STRING)
}

/// Get the short representation of an Infinity literal as a byte slice.
#[inline]
pub fn get_inf_string() -> &'static [u8]
{
    unsafe {
        INF_STRING.as_bytes()
    }
}

/// Get the short representation of an Infinity literal as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you use [`get_inf_string`].
///
/// [`get_inf_string`]: fn.get_inf_string.html
#[no_mangle]
pub unsafe extern fn get_inf_string_ffi(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    get_string_ffi(ptr, size, &mut INF_STRING)
}

/// Set the short representation of Infinity from a byte slice.
///
/// * `bytes`    - Slice of bytes to assign as Infinity string representation.
///
/// # Safety
///
/// Do not call this function in threaded-code, as it is not thread-safe.
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[inline]
pub unsafe fn set_inf_string(bytes: &[u8])
{
    INF_STRING.load_bytes(bytes);
}

/// Set the short representation of Infinity from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you use [`set_inf_string`].
/// Do not call this function in threaded-code, as it is not thread-safe.
///
/// [`set_inf_string`]: fn.set_inf_string.html
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[no_mangle]
pub unsafe extern fn set_inf_string_ffi(ptr: *const u8, size: usize)
    -> i32
{
    set_string_ffi(ptr, size, &mut INF_STRING)
}

/// Get the long representation of an Infinity literal as a byte slice.
#[inline]
pub fn get_infinity_string() -> &'static [u8]
{
    unsafe {
        INFINITY_STRING.as_bytes()
    }
}

/// Get the long representation of an Infinity literal as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you use [`get_infinity_string`].
///
/// [`get_infinity_string`]: fn.get_infinity_string.html
#[no_mangle]
pub unsafe extern fn get_infinity_string_ffi(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    get_string_ffi(ptr, size, &mut INFINITY_STRING)
}

/// Set the long representation of Infinity from a byte slice.
///
/// * `bytes`    - Slice of bytes to assign as Infinity string representation.
///
/// # Safety
///
/// Do not call this function in threaded-code, as it is not thread-safe.
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[inline]
pub unsafe fn set_infinity_string(bytes: &[u8])
{
    INFINITY_STRING.load_bytes(bytes);
}

/// Set the long representation of Infinity from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you use [`set_infinity_string`].
/// Do not call this function in threaded-code, as it is not thread-safe.
///
/// [`set_infinity_string`]: fn.set_infinity_string.html
///
/// # Panics
///
/// Panics if `bytes.len() >= MAX_F32_SIZE`.
#[no_mangle]
pub unsafe extern fn set_infinity_string_ffi(ptr: *const u8, size: usize)
    -> i32
{
    set_string_ffi(ptr, size, &mut INFINITY_STRING)
}

// CONSTANTS

// Simple, fast optimization.
// Since we're declaring a variable on the stack, and our power-of-two
// alignment dramatically improved atoi performance, do it.
cfg_if! {
if #[cfg(feature = "radix")] {
    // Use 256, actually, since we seem to have memory issues with f64.
    // Clearly not sufficient memory allocated for non-base10 values.

    /// The minimum buffer size required to serialize any `i8` value.
    pub const MAX_I8_SIZE: usize = 16;

    /// The minimum buffer size required to serialize any `i16` value.
    pub const MAX_I16_SIZE: usize = 32;

    /// The minimum buffer size required to serialize any `i32` value.
    pub const MAX_I32_SIZE: usize = 64;

    /// The minimum buffer size required to serialize any `i64` value.
    pub const MAX_I64_SIZE: usize = 128;

    /// The minimum buffer size required to serialize any `i128` value.
    pub const MAX_I128_SIZE: usize = 256;

    /// The minimum buffer size required to serialize any `u8` value.
    pub const MAX_U8_SIZE: usize = 16;

    /// The minimum buffer size required to serialize any `u16` value.
    pub const MAX_U16_SIZE: usize = 32;

    /// The minimum buffer size required to serialize any `u32` value.
    pub const MAX_U32_SIZE: usize = 64;

    /// The minimum buffer size required to serialize any `u64` value.
    pub const MAX_U64_SIZE: usize = 128;

    /// The minimum buffer size required to serialize any `u128` value.
    pub const MAX_U128_SIZE: usize = 256;

    /// The minimum buffer size required to serialize any `f32` value.
    pub const MAX_F32_SIZE: usize = 256;

    /// The minimum buffer size required to serialize any `f64` value.
    pub const MAX_F64_SIZE: usize = 256;
} else {
    // The f64 buffer is actually a size of 60, but use 64 since it's a
    // power of 2.

    /// The minimum buffer size required to serialize any `i8` value.
    pub const MAX_I8_SIZE: usize = 4;

    /// The minimum buffer size required to serialize any `i16` value.
    pub const MAX_I16_SIZE: usize = 6;

    /// The minimum buffer size required to serialize any `i32` value.
    pub const MAX_I32_SIZE: usize = 11;

    /// The minimum buffer size required to serialize any `i64` value.
    pub const MAX_I64_SIZE: usize = 20;

    /// The minimum buffer size required to serialize any `i128` value.
    pub const MAX_I128_SIZE: usize = 40;

    /// The minimum buffer size required to serialize any `u8` value.
    pub const MAX_U8_SIZE: usize = 3;

    /// The minimum buffer size required to serialize any `u16` value.
    pub const MAX_U16_SIZE: usize = 5;

    /// The minimum buffer size required to serialize any `u32` value.
    pub const MAX_U32_SIZE: usize = 10;

    /// The minimum buffer size required to serialize any `u64` value.
    pub const MAX_U64_SIZE: usize = 20;

    /// The minimum buffer size required to serialize any `u128` value.
    pub const MAX_U128_SIZE: usize = 39;

    /// The minimum buffer size required to serialize any `f32` value.
    pub const MAX_F32_SIZE: usize = 64;

    /// The minimum buffer size required to serialize any `f64` value.
    pub const MAX_F64_SIZE: usize = 64;
}} // cfg_if

cfg_if! {
if #[cfg(target_pointer_width = "16")] {
    /// The minimum buffer size required to serialize any `isize` value.
    pub const MAX_ISIZE_SIZE: usize = MAX_I16_SIZE;

    /// The minimum buffer size required to serialize any `usize` value.
    pub const MAX_USIZE_SIZE: usize = MAX_U16_SIZE;
} else if #[cfg(target_pointer_width = "32")] {
    /// The minimum buffer size required to serialize any `isize` value.
    pub const MAX_ISIZE_SIZE: usize = MAX_I32_SIZE;

    /// The minimum buffer size required to serialize any `usize` value.
    pub const MAX_USIZE_SIZE: usize = MAX_U32_SIZE;
} else if #[cfg(target_pointer_width = "64")] {
    /// The minimum buffer size required to serialize any `isize` value.
    pub const MAX_ISIZE_SIZE: usize = MAX_I64_SIZE;

    /// The minimum buffer size required to serialize any `usize` value.
    pub const MAX_USIZE_SIZE: usize = MAX_U64_SIZE;
}}  // cfg_if

/// The maximum number of bytes that any number-to-string function may write.
pub const BUFFER_SIZE: usize = MAX_F64_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `i8` value.
#[no_mangle]
pub static MAX_I8_SIZE_FFI: usize = MAX_I8_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `i16` value.
#[no_mangle]
pub static MAX_I16_SIZE_FFI: usize = MAX_I16_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `i32` value.
#[no_mangle]
pub static MAX_I32_SIZE_FFI: usize = MAX_I32_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `i64` value.
#[no_mangle]
pub static MAX_I64_SIZE_FFI: usize = MAX_I64_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `i128` value.
#[no_mangle]
pub static MAX_I128_SIZE_FFI: usize = MAX_I128_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `isize` value.
#[no_mangle]
pub static MAX_ISIZE_SIZE_FFI: usize = MAX_ISIZE_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `u8` value.
#[no_mangle]
pub static MAX_U8_SIZE_FFI: usize = MAX_U8_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `u16` value.
#[no_mangle]
pub static MAX_U16_SIZE_FFI: usize = MAX_U16_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `u32` value.
#[no_mangle]
pub static MAX_U32_SIZE_FFI: usize = MAX_U32_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `u64` value.
#[no_mangle]
pub static MAX_U64_SIZE_FFI: usize = MAX_U64_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `u128` value.
#[no_mangle]
pub static MAX_U128_SIZE_FFI: usize = MAX_U128_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `usize` value.
#[no_mangle]
pub static MAX_USIZE_SIZE_FFI: usize = MAX_USIZE_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `f32` value.
#[no_mangle]
pub static MAX_F32_SIZE_FFI: usize = MAX_F32_SIZE;

/// Symbol-generating constant for the minimum buffer required to serialize any `f64` value.
#[no_mangle]
pub static MAX_F64_SIZE_FFI: usize = MAX_F64_SIZE;

/// Symbol-generating constant for the maximum number of bytes that any number-to-string function may write.
#[no_mangle]
pub static BUFFER_SIZE_FFI: usize = BUFFER_SIZE;

/// The rounding scheme for float conversions.
///
/// This defines the global rounding-scheme for float parsing operations.
/// By default, this is set to `RoundingKind::NearestTieEven`. IEEE754
/// recommends this as the default for all for decimal and binary
/// operations.
///
/// # Safety
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
/// See the documentation for [`RoundingKind`] for the permissible
/// values of `FLOAT_ROUNDING` in FFI-code.
///
/// [`RoundingKind`]: enum.RoundingKind.html
#[no_mangle]
#[cfg(feature = "rounding")]
pub static mut FLOAT_ROUNDING: RoundingKind = RoundingKind::NearestTieEven;

// FUNCTIONS

/// Get the exponent notation character.
#[inline]
#[allow(unused_variables)]
pub(crate) fn exponent_notation_char(radix: u32) -> u8 {
    unsafe {
        #[cfg(not(feature ="radix"))] {
            EXPONENT_DEFAULT_CHAR
        }

        #[cfg(feature ="radix")] {
            if radix >= 15 { EXPONENT_BACKUP_CHAR } else { EXPONENT_DEFAULT_CHAR }
        }
    }
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use atof::*;
    use ftoa::*;
    use util::*;
    use util::test::*;
    use super::*;

    #[cfg(feature ="radix")]
    #[test]
    fn exponent_notation_char_test() {
        unsafe {
            assert_eq!(exponent_notation_char(2), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(8), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(10), EXPONENT_DEFAULT_CHAR);
            assert_eq!(exponent_notation_char(15), EXPONENT_BACKUP_CHAR);
            assert_eq!(exponent_notation_char(16), EXPONENT_BACKUP_CHAR);
            assert_eq!(exponent_notation_char(32), EXPONENT_BACKUP_CHAR);
        }
    }

    // Only enable when no other threads touch NAN_STRING or INFINITY_STRING.
    #[test]
    #[ignore]
    fn special_bytes_test() {
        unsafe {
            let mut buffer = new_buffer();
            // Test serializing and deserializing special strings.
            assert!(try_atof32_slice(b"NaN").value.is_nan());
            assert!(try_atof32_slice(b"nan").value.is_nan());
            assert!(try_atof32_slice(b"NAN").value.is_nan());
            assert!(try_atof32_slice(b"inf").value.is_infinite());
            assert!(try_atof32_slice(b"INF").value.is_infinite());
            assert!(try_atof32_slice(b"Infinity").value.is_infinite());
            assert_eq!(f64toa_slice(f64::NAN, &mut buffer), b"NaN");
            assert_eq!(f64toa_slice(f64::INFINITY, &mut buffer), b"inf");

            NAN_STRING.load_bytes(b"nan");
            INF_STRING.load_bytes(b"Infinity");

            assert!(try_atof32_slice(b"inf").error.code == ErrorCode::InvalidDigit);
            assert!(try_atof32_slice(b"Infinity").value.is_infinite());
            assert_eq!(f64toa_slice(f64::NAN, &mut buffer), b"nan");
            assert_eq!(f64toa_slice(f64::INFINITY, &mut buffer), b"Infinity");

            NAN_STRING.load_bytes(b"NaN");
            INF_STRING.load_bytes(b"inf");
        }
    }

    // Only enable when no other threads touch FLOAT_ROUNDING.
    #[cfg(all(feature = "correct", feature = "rounding"))]
    #[test]
    #[ignore]
    fn special_rounding_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.
        unsafe {
            // Nearest, tie-even
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
            assert_eq!(try_atof64_slice(b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(b"9007199254740995").value, 9007199254740996.0);

            // Nearest, tie-away-zero
            FLOAT_ROUNDING = RoundingKind::NearestTieAwayZero;
            assert_eq!(try_atof64_slice(b"-9007199254740993").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(b"9007199254740993").value, 9007199254740994.0);
            assert_eq!(try_atof64_slice(b"9007199254740995").value, 9007199254740996.0);

            // Toward positive infinity
            FLOAT_ROUNDING = RoundingKind::TowardPositiveInfinity;
            assert_eq!(try_atof64_slice(b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(b"-9007199254740995").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(b"9007199254740993").value, 9007199254740994.0);
            assert_eq!(try_atof64_slice(b"9007199254740995").value, 9007199254740996.0);

            // Toward negative infinity
            FLOAT_ROUNDING = RoundingKind::TowardNegativeInfinity;
            assert_eq!(try_atof64_slice(b"-9007199254740993").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(b"9007199254740995").value, 9007199254740994.0);

            // Toward zero
            FLOAT_ROUNDING = RoundingKind::TowardZero;
            assert_eq!(try_atof64_slice(b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(b"-9007199254740995").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(b"9007199254740995").value, 9007199254740994.0);

            // Reset to default
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
        }
    }

    // Only enable when no other threads touch FLOAT_ROUNDING.
    #[cfg(all(feature = "correct", feature = "radix", feature = "rounding"))]
    #[test]
    #[ignore]
    fn special_rounding_binary_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.
        unsafe {
            // Nearest, tie-even
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740992.0);
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740996.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740992.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740996.0);

            // Nearest, tie-away-zero
            FLOAT_ROUNDING = RoundingKind::NearestTieAwayZero;
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740994.0);
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740996.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740994.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740996.0);

            // Toward positive infinity
            FLOAT_ROUNDING = RoundingKind::TowardPositiveInfinity;
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740992.0);
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740994.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740994.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740996.0);

            // Toward negative infinity
            FLOAT_ROUNDING = RoundingKind::TowardNegativeInfinity;
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740994.0);
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740996.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740992.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740994.0);

            // Toward zero
            FLOAT_ROUNDING = RoundingKind::TowardZero;
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740992.0);
            assert_eq!(try_atof64_radix_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740994.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740992.0);
            assert_eq!(try_atof64_radix_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740994.0);

            // Reset to default
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
        }
    }
}
