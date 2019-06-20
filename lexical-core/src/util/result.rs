//! C-compatible result type.

use super::error::{self, Error};

/// C-compatible result type from parsing strings-to-numbers for FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Result<T> {
    /// Value from the parser function call.
    pub value: T,
    /// Error information, including the error code and other metadata.
    pub error: Error,
}

/// Helper function to create a success message.
#[inline]
pub(crate) fn success<T>(value: T)
    -> Result<T>
{
    Result { value: value, error: error::success() }
}

/// Helper function to create an overflow error.
#[inline]
pub(crate) fn overflow_error<T>(value: T)
    -> Result<T>
{
    Result { value: value, error: error::overflow_error() }
}

/// Helper function to create an invalid digit error.
#[inline]
pub(crate) fn invalid_digit_error<T>(value: T, index: usize)
    -> Result<T>
{
    Result { value: value, error: error::invalid_digit_error(index) }
}

#[inline]
pub(crate) fn empty_error<T>(value: T)
    -> Result<T>
{
    Result { value: value, error: error::empty_error() }
}

// FFI
// Manually expand the templates for all known result types, since
// no other language has Rust-compatible generics.

/// Expanded generic for a result type containing a value of type u8.
pub type U8Result = Result<u8>;

/// Expanded generic for a result type containing a value of type u16.
pub type U16Result = Result<u16>;

/// Expanded generic for a result type containing a value of type u32.
pub type U32Result = Result<u32>;

/// Expanded generic for a result type containing a value of type u64.
pub type U64Result = Result<u64>;

/// Expanded generic for a result type containing a value of type u128.
#[cfg(has_i128)]
pub type U128Result = Result<u128>;

/// Expanded generic for a result type containing a value of type usize.
pub type UsizeResult = Result<usize>;

/// Expanded generic for a result type containing a value of type i8.
pub type I8Result = Result<i8>;

/// Expanded generic for a result type containing a value of type i16.
pub type I16Result = Result<i16>;

/// Expanded generic for a result type containing a value of type i32.
pub type I32Result = Result<i32>;

/// Expanded generic for a result type containing a value of type i64.
pub type I64Result = Result<i64>;

/// Expanded generic for a result type containing a value of type i128.
#[cfg(has_i128)]
pub type I128Result = Result<i128>;

/// Expanded generic for a result type containing a value of type isize.
pub type IsizeResult = Result<isize>;

/// Expanded generic for a result type containing a value of type f32.
pub type F32Result = Result<f32>;

/// Expanded generic for a result type containing a value of type f64.
pub type F64Result = Result<f64>;
