//! Config settings for lexical.

use lib::slice;
use super::rounding::RoundingKind;

// GLOBALS

/// Not a Number literal
///
/// To change the expected representation of NaN as a string,
/// change this value during before using lexical.
///
/// # Safety
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut NAN_STRING: &[u8] = b"NaN";

/// Short infinity literal
///
/// To change the expected representation of Infinity as a string,
/// change this value during before using lexical.
///
/// # Safety
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut INF_STRING: &[u8] = b"inf";

/// Long infinity literal
///
/// To change the expected backup representation of Infinity as a string,
/// change this value during before using lexical.
///
/// # Safety
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut INFINITY_STRING: &[u8] = b"infinity";

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

// Since these types are not all C-FFI-compatible, we need to define
// a few getters and setters for the global constants.

/// Get [`NAN_STRING`] as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you should directly use
/// [`NAN_STRING`].
///
/// [`NAN_STRING`]: static.NAN_STRING.html
#[no_mangle]
pub unsafe extern fn get_nan_string(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    if ptr.is_null() || size.is_null() {
        -1
    } else {
        *ptr = NAN_STRING.as_ptr();
        *size = NAN_STRING.len();
        0
    }
}

/// Set [`NAN_STRING`] from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you should directly modify
/// [`NAN_STRING`]. Do not call this function in threaded-code, as it is
/// not thread-safe. The assigned string must be valid as long as
/// lexical-core is in use (ideally static), no copies are made.
///
/// [`NAN_STRING`]: static.NAN_STRING.html
#[no_mangle]
pub unsafe extern fn set_nan_string(ptr: *const u8, size: usize)
    -> i32
{
    if ptr.is_null() {
        -1
    } else {
        NAN_STRING = slice::from_raw_parts(ptr, size);
        0
    }
}

/// Get [`INF_STRING`] as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you should directly use
/// [`INF_STRING`].
///
/// [`INF_STRING`]: static.INF_STRING.html
#[no_mangle]
pub unsafe extern fn get_inf_string(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    if ptr.is_null() || size.is_null() {
        -1
    } else {
        *ptr = INF_STRING.as_ptr();
        *size = INF_STRING.len();
        0
    }
}

/// Set [`INF_STRING`] from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you should directly modify
/// [`INF_STRING`]. Do not call this function in threaded-code, as it is
/// not thread-safe. The assigned string must be valid as long as
/// lexical-core is in use (ideally static), no copies are made.
///
/// [`INF_STRING`]: static.INF_STRING.html
#[no_mangle]
pub unsafe extern fn set_inf_string(ptr: *const u8, size: usize)
    -> i32
{
    if ptr.is_null() {
        -1
    } else {
        INF_STRING = slice::from_raw_parts(ptr, size);
        0
    }
}

/// Get [`INFINITY_STRING`] as a pointer and size.
///
/// Returns 0 on success, -1 on error. This string is **not**
/// null-terminated.
///
/// * `ptr`     - Out-parameter for a pointer to the string.
/// * `size`    - Out-parameter for the size of the string.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you should directly use
/// [`INFINITY_STRING`].
///
/// [`INFINITY_STRING`]: static.INFINITY_STRING.html
#[no_mangle]
pub unsafe extern fn get_infinity_string(ptr: *mut *const u8, size: *mut usize)
    -> i32
{
    if ptr.is_null() || size.is_null() {
        -1
    } else {
        *ptr = INFINITY_STRING.as_ptr();
        *size = INFINITY_STRING.len();
        0
    }
}

/// Set [`INFINITY_STRING`] from a pointer and size.
///
/// Returns 0 on success, -1 on error.
///
/// * `ptr`     - Pointer to the first character in the contiguous string.
/// * `size`    - Size of the string, without the null-terminator.
///
/// # Safety
///
/// Only use this in C-FFI code, otherwise, you should directly modify
/// [`INFINITY_STRING`]. Do not call this function in threaded-code, as
/// it is not thread-safe. The assigned string must be valid as long as
/// lexical-core is in use (ideally static), no copies are made.
///
/// [`INFINITY_STRING`]: static.INFINITY_STRING.html
#[no_mangle]
pub unsafe extern fn set_infinity_string(ptr: *const u8, size: usize)
    -> i32
{
    if ptr.is_null() {
        -1
    } else {
        INFINITY_STRING = slice::from_raw_parts(ptr, size);
        0
    }
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
            assert!(try_atof32_slice(10, b"NaN").value.is_nan());
            assert!(try_atof32_slice(10, b"nan").value.is_nan());
            assert!(try_atof32_slice(10, b"NAN").value.is_nan());
            assert!(try_atof32_slice(10, b"inf").value.is_infinite());
            assert!(try_atof32_slice(10, b"INF").value.is_infinite());
            assert!(try_atof32_slice(10, b"Infinity").value.is_infinite());
            assert_eq!(f64toa_slice(f64::NAN, 10, &mut buffer), b"NaN");
            assert_eq!(f64toa_slice(f64::INFINITY, 10, &mut buffer), b"inf");

            NAN_STRING = b"nan";
            INF_STRING = b"Infinity";

            assert!(try_atof32_slice(10, b"inf").error.code == ErrorCode::InvalidDigit);
            assert!(try_atof32_slice(10, b"Infinity").value.is_infinite());
            assert_eq!(f64toa_slice(f64::NAN, 10, &mut buffer), b"nan");
            assert_eq!(f64toa_slice(f64::INFINITY, 10, &mut buffer), b"Infinity");

            NAN_STRING = b"NaN";
            INF_STRING = b"inf";
        }
    }

    // Only enable when no other threads touch FLOAT_ROUNDING.
    #[cfg(feature = "correct")]
    #[test]
    #[ignore]
    fn special_rounding_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.
        unsafe {
            // Nearest, tie-even
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740996.0);

            // Nearest, tie-away-zero
            FLOAT_ROUNDING = RoundingKind::NearestTieAwayZero;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740996.0);

            // Toward positive infinity
            FLOAT_ROUNDING = RoundingKind::TowardPositiveInfinity;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740996.0);

            // Toward negative infinity
            FLOAT_ROUNDING = RoundingKind::TowardNegativeInfinity;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740994.0);

            // Toward zero
            FLOAT_ROUNDING = RoundingKind::TowardZero;
            assert_eq!(try_atof64_slice(10, b"-9007199254740993").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"-9007199254740995").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740993").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(10, b"9007199254740995").value, 9007199254740994.0);

            // Reset to default
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
        }
    }

    // Only enable when no other threads touch FLOAT_ROUNDING.
    #[cfg(all(feature = "correct", feature = "radix"))]
    #[test]
    #[ignore]
    fn special_rounding_binary_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.
        unsafe {
            // Nearest, tie-even
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740996.0);

            // Nearest, tie-away-zero
            FLOAT_ROUNDING = RoundingKind::NearestTieAwayZero;
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740994.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740996.0);

            // Toward positive infinity
            FLOAT_ROUNDING = RoundingKind::TowardPositiveInfinity;
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740994.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740996.0);

            // Toward negative infinity
            FLOAT_ROUNDING = RoundingKind::TowardNegativeInfinity;
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740996.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740994.0);

            // Toward zero
            FLOAT_ROUNDING = RoundingKind::TowardZero;
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000001").value, -9007199254740992.0);
            assert_eq!(try_atof64_slice(2, b"-100000000000000000000000000000000000000000000000000011").value, -9007199254740994.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000001").value, 9007199254740992.0);
            assert_eq!(try_atof64_slice(2, b"100000000000000000000000000000000000000000000000000011").value, 9007199254740994.0);

            // Reset to default
            FLOAT_ROUNDING = RoundingKind::NearestTieEven;
        }
    }
}
