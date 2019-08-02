//! C-compatible result type.

use super::error::{self, Error};

/// C-compatible result type from parsing strings-to-numbers for FFI.
pub type Result<T> = std::result::Result<T, Error>;

// FFI
// Manually expand the templates for all known result types, since
// no other language has Rust-compatible generics.

// U8

/// Expanded generic for a result type containing a value of type u8.
pub type U8Result = Result<u8>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn u8result_is_ok(result: U8Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u8result_is_err(result: U8Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u8result_ok(result: U8Result) -> u8 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u8result_err(result: U8Result) -> Error {
    result.err().unwrap()
}

// U16

/// Expanded generic for a result type containing a value of type u16.
pub type U16Result = Result<u16>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn u16result_is_ok(result: U16Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u16result_is_err(result: U16Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u16result_ok(result: U16Result) -> u16 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u16result_err(result: U16Result) -> Error {
    result.err().unwrap()
}

// U32

/// Expanded generic for a result type containing a value of type u32.
pub type U32Result = Result<u32>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn u32result_is_ok(result: U32Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u32result_is_err(result: U32Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u32result_ok(result: U32Result) -> u32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u32result_err(result: U32Result) -> Error {
    result.err().unwrap()
}

// U64

/// Expanded generic for a result type containing a value of type u64.
pub type U64Result = Result<u64>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn u64result_is_ok(result: U64Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u64result_is_err(result: U64Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u64result_ok(result: U64Result) -> u64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u64result_err(result: U64Result) -> Error {
    result.err().unwrap()
}

// U128

/// Expanded generic for a result type containing a value of type u128.
#[cfg(has_i128)]
pub type U128Result = Result<u128>;

/// Check if the result was ok.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128result_is_ok(result: U128Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128result_is_err(result: U128Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128result_ok(result: U128Result) -> u128 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128result_err(result: U128Result) -> Error {
    result.err().unwrap()
}

// USIZE

/// Expanded generic for a result type containing a value of type usize.
pub type UsizeResult = Result<usize>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn usizeresult_is_ok(result: UsizeResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn usizeresult_is_err(result: UsizeResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn usizeresult_ok(result: UsizeResult) -> usize {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn usizeresult_err(result: UsizeResult) -> Error {
    result.err().unwrap()
}

// I8

/// Expanded generic for a result type containing a value of type i8.
pub type I8Result = Result<i8>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn i8result_is_ok(result: I8Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i8result_is_err(result: I8Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i8result_ok(result: I8Result) -> i8 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i8result_err(result: I8Result) -> Error {
    result.err().unwrap()
}

// I16

/// Expanded generic for a result type containing a value of type i16.
pub type I16Result = Result<i16>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn i16result_is_ok(result: I16Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i16result_is_err(result: I16Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i16result_ok(result: I16Result) -> i16 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i16result_err(result: I16Result) -> Error {
    result.err().unwrap()
}

// I32

/// Expanded generic for a result type containing a value of type i32.
pub type I32Result = Result<i32>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn i32result_is_ok(result: I32Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i32result_is_err(result: I32Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i32result_ok(result: I32Result) -> i32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i32result_err(result: I32Result) -> Error {
    result.err().unwrap()
}

// I64

/// Expanded generic for a result type containing a value of type i64.
pub type I64Result = Result<i64>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn i64result_is_ok(result: I64Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i64result_is_err(result: I64Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i64result_ok(result: I64Result) -> i64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i64result_err(result: I64Result) -> Error {
    result.err().unwrap()
}

// I128

/// Expanded generic for a result type containing a value of type i128.
#[cfg(has_i128)]
pub type I128Result = Result<i128>;

/// Check if the result was ok.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128result_is_ok(result: I128Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128result_is_err(result: I128Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128result_ok(result: I128Result) -> i128 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128result_err(result: I128Result) -> Error {
    result.err().unwrap()
}

// ISIZE

/// Expanded generic for a result type containing a value of type isize.
pub type IsizeResult = Result<isize>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn isizeresult_is_ok(result: IsizeResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn isizeresult_is_err(result: IsizeResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn isizeresult_ok(result: IsizeResult) -> isize {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn isizeresult_err(result: IsizeResult) -> Error {
    result.err().unwrap()
}

// F32

/// Expanded generic for a result type containing a value of type f32.
pub type F32Result = Result<f32>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn f32result_is_ok(result: F32Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn f32result_is_err(result: F32Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn f32result_ok(result: F32Result) -> f32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn f32result_err(result: F32Result) -> Error {
    result.err().unwrap()
}

// F64

/// Expanded generic for a result type containing a value of type f64.
pub type F64Result = Result<f64>;

/// Check if the result was ok.
#[no_mangle]
pub extern fn f64result_is_ok(result: F64Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn f64result_is_err(result: F64Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn f64result_ok(result: F64Result) -> f64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn f64result_err(result: F64Result) -> Error {
    result.err().unwrap()
}
