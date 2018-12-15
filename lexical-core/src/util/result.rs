//! C-compatible result type.

use super::error::{self, Error};
use super::num::Number;

/// C-compatible error for FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Result<T: Number> {
    /// Value from the call.
    pub value: T,
    /// Error message.
    pub error: Error,
}

/// Helper function to create a success message.
#[inline(always)]
pub(crate) fn success<T>(value: T)
    -> Result<T>
    where T: Number
{
    Result { value: value, error: error::success() }
}

/// Helper function to create an overflow error.
#[inline(always)]
pub(crate) fn overflow_error<T>(value: T)
    -> Result<T>
    where T: Number
{
    Result { value: value, error: error::overflow_error() }
}

/// Helper function to create an invalid digit error.
#[inline(always)]
pub(crate) fn invalid_digit_error<T>(value: T, index: usize)
    -> Result<T>
    where T: Number
{
    Result { value: value, error: error::invalid_digit_error(index) }
}

// FFI
// Manually expand the templates for all known result types, since
// no other language has Rust-compatible generics.
pub type U8Result = Result<u8>;
pub type U16Result = Result<u16>;
pub type U32Result = Result<u32>;
pub type U64Result = Result<u64>;
pub type U128Result = Result<u128>;
pub type UsizeResult = Result<usize>;
pub type I8Result = Result<i8>;
pub type I16Result = Result<i16>;
pub type I32Result = Result<i32>;
pub type I64Result = Result<i64>;
pub type I128Result = Result<i128>;
pub type IsizeResult = Result<isize>;
pub type F32Result = Result<f32>;
pub type F64Result = Result<f64>;
