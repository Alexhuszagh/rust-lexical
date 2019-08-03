//! C-compatible result type.

use lib::{mem, result};
use super::error::{self, Error};

/// Tag for the FFI-compatible result.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum ResultTag {
    Ok,
    Err
}

/// Union for the FFI-compatible result.
#[repr(C)]
union ResultUnion<T: Copy> {
    value: T,
    error: Error,
}

/// C-compatible result type from parsing strings-to-numbers for FFI.
///
/// This is an FFI-safe result type that is returned by the from range
/// APIs, for example, `atou8_range`.
#[repr(C)]
pub struct ResultFfi<T: Copy> {
    tag: ResultTag,
    data: ResultUnion<T>,
}

impl<T: Copy> ResultFfi<T> {
    /// Get if result is ok.
    pub fn is_ok(&self) -> bool {
        self.tag == ResultTag::Ok
    }

    /// Get if result is an error.
    pub fn is_err(&self) -> bool {
        self.tag == ResultTag::Err
    }

    /// Converts result into Option<T>, discarding any error.
    pub fn ok(self) -> Option<T> {
        unsafe {
            match self.tag {
                ResultTag::Ok  => Some(self.data.value),
                ResultTag::Err => None,
            }
        }
    }

    /// Converts result into Option<Error>, discarding any value.
    pub fn err(self) -> Option<Error> {
        unsafe {
            match self.tag {
                ResultTag::Ok  => None,
                ResultTag::Err => Some(self.data.error),
            }
        }
    }
}

// Simplify conversion between the FFI-compatible result type and Rust's
// default result type.
impl<T: Copy> From<result::Result<T, Error>> for ResultFfi<T> {
    fn from(res: result::Result<T, Error>) -> ResultFfi<T> {
        match res {
            Ok(v)  => {
                let data = ResultUnion { value: v };
                ResultFfi { tag: ResultTag::Ok, data }
            },
            Err(e) => {
                let data = ResultUnion { error: e };
                ResultFfi { tag: ResultTag::Err, data }
            },
        }
    }
}

impl<T: Copy> Into<result::Result<T, Error>> for ResultFfi<T> {
    fn into(self) -> result::Result<T, Error> {
        unsafe {
            match self.tag {
                ResultTag::Ok  => Ok(self.data.value),
                ResultTag::Err => Err(self.data.error),
            }
        }
    }
}

/// Rust intrinsic result type from parsing strings-to-numbers for FFI.
pub type Result<T> = result::Result<T, Error>;

// FFI
// Manually expand the templates for all known result types, since
// no other language has Rust-compatible generics.

// U8

/// Expanded generic for a result type containing a value of type u8.
pub type U8ResultFfi = ResultFfi<u8>;

/// Number of bytes required to store U8ResultFfi.
#[no_mangle]
pub static U8_RESULT_FFI_SIZE: usize = mem::size_of::<U8ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u8_result_ffi_is_ok(result: U8ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u8_result_ffi_is_err(result: U8ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u8_result_ffi_ok(result: U8ResultFfi) -> u8 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u8_result_ffi_err(result: U8ResultFfi) -> Error {
    result.err().unwrap()
}

// U16

/// Expanded generic for a result type containing a value of type u16.
pub type U16ResultFfi = ResultFfi<u16>;

/// Number of bytes required to store U16ResultFfi.
#[no_mangle]
pub static U16_RESULT_FFI_SIZE: usize = mem::size_of::<U16ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u16_result_ffi_is_ok(result: U16ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u16_result_ffi_is_err(result: U16ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u16_result_ffi_ok(result: U16ResultFfi) -> u16 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u16_result_ffi_err(result: U16ResultFfi) -> Error {
    result.err().unwrap()
}

// U32

/// Expanded generic for a result type containing a value of type u32.
pub type U32ResultFfi = ResultFfi<u32>;

/// Number of bytes required to store U32ResultFfi.
#[no_mangle]
pub static U32_RESULT_FFI_SIZE: usize = mem::size_of::<U32ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u32_result_ffi_is_ok(result: U32ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u32_result_ffi_is_err(result: U32ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u32_result_ffi_ok(result: U32ResultFfi) -> u32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u32_result_ffi_err(result: U32ResultFfi) -> Error {
    result.err().unwrap()
}

// U64

/// Expanded generic for a result type containing a value of type u64.
pub type U64ResultFfi = ResultFfi<u64>;

/// Number of bytes required to store U64ResultFfi.
#[no_mangle]
pub static U64_RESULT_FFI_SIZE: usize = mem::size_of::<U64ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u64_result_ffi_is_ok(result: U64ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u64_result_ffi_is_err(result: U64ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u64_result_ffi_ok(result: U64ResultFfi) -> u64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u64_result_ffi_err(result: U64ResultFfi) -> Error {
    result.err().unwrap()
}

// U128

/// Expanded generic for a result type containing a value of type u128.
#[cfg(has_i128)]
pub type U128ResultFfi = ResultFfi<u128>;

/// Number of bytes required to store U128ResultFfi.
#[cfg(has_i128)]
#[no_mangle]
pub static U128_RESULT_FFI_SIZE: usize = mem::size_of::<U128ResultFfi>();

/// Check if the result was ok.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128_result_ffi_is_ok(result: U128ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128_result_ffi_is_err(result: U128ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128_result_ffi_ok(result: U128ResultFfi) -> u128 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128_result_ffi_err(result: U128ResultFfi) -> Error {
    result.err().unwrap()
}

// USIZE

/// Expanded generic for a result type containing a value of type usize.
pub type UsizeResultFfi = ResultFfi<usize>;

/// Number of bytes required to store UsizeResultFfi.
#[no_mangle]
pub static USIZE_RESULT_FFI_SIZE: usize = mem::size_of::<UsizeResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn usize_result_ffi_is_ok(result: UsizeResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn usize_result_ffi_is_err(result: UsizeResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn usize_result_ffi_ok(result: UsizeResultFfi) -> usize {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn usize_result_ffi_err(result: UsizeResultFfi) -> Error {
    result.err().unwrap()
}

// I8

/// Expanded generic for a result type containing a value of type i8.
pub type I8ResultFfi = ResultFfi<i8>;

/// Number of bytes required to store I8ResultFfi.
#[no_mangle]
pub static I8_RESULT_FFI_SIZE: usize = mem::size_of::<I8ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i8_result_ffi_is_ok(result: I8ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i8_result_ffi_is_err(result: I8ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i8_result_ffi_ok(result: I8ResultFfi) -> i8 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i8_result_ffi_err(result: I8ResultFfi) -> Error {
    result.err().unwrap()
}

// I16

/// Expanded generic for a result type containing a value of type i16.
pub type I16ResultFfi = ResultFfi<i16>;

/// Number of bytes required to store I16ResultFfi.
#[no_mangle]
pub static I16_RESULT_FFI_SIZE: usize = mem::size_of::<I16ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i16_result_ffi_is_ok(result: I16ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i16_result_ffi_is_err(result: I16ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i16_result_ffi_ok(result: I16ResultFfi) -> i16 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i16_result_ffi_err(result: I16ResultFfi) -> Error {
    result.err().unwrap()
}

// I32

/// Expanded generic for a result type containing a value of type i32.
pub type I32ResultFfi = ResultFfi<i32>;

/// Number of bytes required to store I32ResultFfi.
#[no_mangle]
pub static I32_RESULT_FFI_SIZE: usize = mem::size_of::<I32ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i32_result_ffi_is_ok(result: I32ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i32_result_ffi_is_err(result: I32ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i32_result_ffi_ok(result: I32ResultFfi) -> i32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i32_result_ffi_err(result: I32ResultFfi) -> Error {
    result.err().unwrap()
}

// I64

/// Expanded generic for a result type containing a value of type i64.
pub type I64ResultFfi = ResultFfi<i64>;

/// Number of bytes required to store I64ResultFfi.
#[no_mangle]
pub static I64_RESULT_FFI_SIZE: usize = mem::size_of::<I64ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i64_result_ffi_is_ok(result: I64ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i64_result_ffi_is_err(result: I64ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i64_result_ffi_ok(result: I64ResultFfi) -> i64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i64_result_ffi_err(result: I64ResultFfi) -> Error {
    result.err().unwrap()
}

// I128

/// Expanded generic for a result type containing a value of type i128.
#[cfg(has_i128)]
pub type I128ResultFfi = ResultFfi<i128>;

/// Number of bytes required to store I128ResultFfi.
#[cfg(has_i128)]
#[no_mangle]
pub static I128_RESULT_FFI_SIZE: usize = mem::size_of::<I128ResultFfi>();

/// Check if the result was ok.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128_result_ffi_is_ok(result: I128ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128_result_ffi_is_err(result: I128ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128_result_ffi_ok(result: I128ResultFfi) -> i128 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128_result_ffi_err(result: I128ResultFfi) -> Error {
    result.err().unwrap()
}

// ISIZE

/// Expanded generic for a result type containing a value of type isize.
pub type IsizeResultFfi = ResultFfi<isize>;

/// Number of bytes required to store IsizeResultFfi.
#[no_mangle]
pub static ISIZE_RESULT_FFI_SIZE: usize = mem::size_of::<IsizeResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn isize_result_ffi_is_ok(result: IsizeResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn isize_result_ffi_is_err(result: IsizeResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn isize_result_ffi_ok(result: IsizeResultFfi) -> isize {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn isize_result_ffi_err(result: IsizeResultFfi) -> Error {
    result.err().unwrap()
}

// F32

/// Expanded generic for a result type containing a value of type f32.
pub type F32ResultFfi = ResultFfi<f32>;

/// Number of bytes required to store F32ResultFfi.
#[no_mangle]
pub static F32_RESULT_FFI_SIZE: usize = mem::size_of::<F32ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn f32_result_ffi_is_ok(result: F32ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn f32_result_ffi_is_err(result: F32ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn f32_result_ffi_ok(result: F32ResultFfi) -> f32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn f32_result_ffi_err(result: F32ResultFfi) -> Error {
    result.err().unwrap()
}

// F64

/// Expanded generic for a result type containing a value of type f64.
pub type F64ResultFfi = ResultFfi<f64>;

/// Number of bytes required to store F64ResultFfi.
#[no_mangle]
pub static F64_RESULT_FFI_SIZE: usize = mem::size_of::<F64ResultFfi>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn f64_result_ffi_is_ok(result: F64ResultFfi) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn f64_result_ffi_is_err(result: F64ResultFfi) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn f64_result_ffi_ok(result: F64ResultFfi) -> f64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn f64_result_ffi_err(result: F64ResultFfi) -> Error {
    result.err().unwrap()
}
