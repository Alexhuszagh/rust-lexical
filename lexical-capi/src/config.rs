//! Config settings for lexical-capi.

use lexical_core::{self, Number};

// FEATURES


/// Exported symbol to denote the presence of the format feature.
#[no_mangle]
#[doc(hidden)]
#[cfg(feature = "format")]
pub static LEXICAL_HAS_FORMAT: usize = 1;

/// Exported symbol to denote the presence of the i128 feature.
#[no_mangle]
#[doc(hidden)]
#[cfg(feature = "i128")]
pub static LEXICAL_HAS_I128: usize = 1;

/// Exported symbol to denote the presence of the radix feature.
#[no_mangle]
#[doc(hidden)]
#[cfg(feature = "radix")]
pub static LEXICAL_HAS_RADIX: usize = 1;

/// Exported symbol to denote the presence of the rounding feature.
#[no_mangle]
#[doc(hidden)]
#[cfg(feature = "rounding")]
pub static LEXICAL_HAS_ROUNDING: usize = 1;

// CONSTANTS

/// Maximum number of bytes required to serialize an `i8` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_I8_FORMATTED_SIZE: usize = i8::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `i16` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_I16_FORMATTED_SIZE: usize = i16::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `i32` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_I32_FORMATTED_SIZE: usize = i32::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `i64` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_I64_FORMATTED_SIZE: usize = i64::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `i128` value to string.
#[no_mangle]
#[doc(hidden)]
#[cfg(feature = "i128")]
pub static LEXICAL_I128_FORMATTED_SIZE: usize = i128::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize an `isize` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_ISIZE_FORMATTED_SIZE: usize = isize::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u8` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_U8_FORMATTED_SIZE: usize = u8::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u16` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_U16_FORMATTED_SIZE: usize = u16::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u32` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_U32_FORMATTED_SIZE: usize = u32::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u64` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_U64_FORMATTED_SIZE: usize = u64::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `u128` value to string.
#[no_mangle]
#[doc(hidden)]
#[cfg(feature = "i128")]
pub static LEXICAL_U128_FORMATTED_SIZE: usize = u128::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `usize` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_USIZE_FORMATTED_SIZE: usize = usize::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `f32` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_F32_FORMATTED_SIZE: usize = f32::FORMATTED_SIZE;

/// Maximum number of bytes required to serialize a `f64` value to string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_F64_FORMATTED_SIZE: usize = f64::FORMATTED_SIZE;

// FFI DECIMAL

/// Maximum number of bytes required to serialize an `i8` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_I8_FORMATTED_SIZE_DECIMAL: usize = i8::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `i16` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_I16_FORMATTED_SIZE_DECIMAL: usize = i16::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `i32` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_I32_FORMATTED_SIZE_DECIMAL: usize = i32::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `i64` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_I64_FORMATTED_SIZE_DECIMAL: usize = i64::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `i128` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
#[cfg(feature = "i128")]
pub static LEXICAL_I128_FORMATTED_SIZE_DECIMAL: usize = i128::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize an `isize` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_ISIZE_FORMATTED_SIZE_DECIMAL: usize = isize::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u8` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_U8_FORMATTED_SIZE_DECIMAL: usize = u8::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u16` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_U16_FORMATTED_SIZE_DECIMAL: usize = u16::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u32` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_U32_FORMATTED_SIZE_DECIMAL: usize = u32::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u64` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_U64_FORMATTED_SIZE_DECIMAL: usize = u64::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `u128` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
#[cfg(feature = "i128")]
pub static LEXICAL_U128_FORMATTED_SIZE_DECIMAL: usize = u128::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `usize` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_USIZE_FORMATTED_SIZE_DECIMAL: usize = usize::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `f32` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_F32_FORMATTED_SIZE_DECIMAL: usize = f32::FORMATTED_SIZE_DECIMAL;

/// Maximum number of bytes required to serialize a `f64` value to a decimal string.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_F64_FORMATTED_SIZE_DECIMAL: usize = f64::FORMATTED_SIZE_DECIMAL;

// FFI BUFFER SIZE

/// Symbol-generating constant for the maximum number of bytes that any number-to-string function may write.
#[no_mangle]
#[doc(hidden)]
pub static LEXICAL_BUFFER_SIZE: usize = lexical_core::BUFFER_SIZE;
