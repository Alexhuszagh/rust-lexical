//! C-compatible result type.

use lib::result::Result as StdResult;
use super::error::Error;

/// Rust intrinsic result type from parsing strings-to-numbers for FFI.
pub type Result<T> = StdResult<T, Error>;

pub(crate) mod result_ffi {

// FFI
// ---

use lib::mem;
use lib::result::Result as StdResult;
use super::super::error::{self, Error};

/// C-compatible tuple for the partial parsers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tuple<T: Copy, U: Copy> {
    pub x: T,
    pub y: U,
}

// Simplify conversion to and from std's Tuple.
impl<T: Copy, U: Copy> From<(T, U)> for Tuple<T, U> {
    fn from(tup: (T, U)) -> Tuple<T, U> {
        Tuple { x: tup.0, y: tup.1 }
    }
}

impl<T: Copy, U: Copy> Into<(T, U)> for Tuple<T, U> {
    fn into(self) -> (T, U) {
        (self.x, self.y)
    }
}

/// Tag for the FFI-compatible result.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum ResultTag {
    Ok,
    Err
}

/// Union for the FFI-compatible result.
#[repr(C)]
#[derive(Copy, Clone)]
union ResultUnion<T: Copy> {
    value: T,
    error: Error,
}

/// C-compatible result type from parsing strings-to-numbers for FFI.
///
/// This is an FFI-safe result type that is returned by the from range
/// APIs, for example, `atou8_range`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Result<T: Copy> {
    tag: ResultTag,
    data: ResultUnion<T>,
}

impl<T: Copy> Result<T> {
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

// Simplify conversion to and from std's Result.
impl<T: Copy> From<StdResult<T, Error>> for Result<T> {
    fn from(res: StdResult<T, Error>) -> Result<T> {
        match res {
            Ok(v)  => {
                let data = ResultUnion { value: v };
                Result { tag: ResultTag::Ok, data }
            },
            Err(e) => {
                let data = ResultUnion { error: e };
                Result { tag: ResultTag::Err, data }
            },
        }
    }
}

impl<T: Copy> Into<StdResult<T, Error>> for Result<T> {
    fn into(self) -> StdResult<T, Error> {
        unsafe {
            match self.tag {
                ResultTag::Ok  => Ok(self.data.value),
                ResultTag::Err => Err(self.data.error),
            }
        }
    }
}

// COMPLETE
// --------

// Manually expand the templates for all known result types, since
// no other language has Rust-compatible generics.

// U8

/// Expanded generic for a result type containing a value of type u8.
pub type U8Result = Result<u8>;

/// Number of bytes required to store U8Result.
#[no_mangle]
pub static U8_RESULT_SIZE: usize = mem::size_of::<U8Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u8_result_is_ok(result: U8Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u8_result_is_err(result: U8Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u8_result_ok(result: U8Result) -> u8 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u8_result_err(result: U8Result) -> Error {
    result.err().unwrap()
}

// U16

/// Expanded generic for a result type containing a value of type u16.
pub type U16Result = Result<u16>;

/// Number of bytes required to store U16Result.
#[no_mangle]
pub static U16_RESULT_SIZE: usize = mem::size_of::<U16Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u16_result_is_ok(result: U16Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u16_result_is_err(result: U16Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u16_result_ok(result: U16Result) -> u16 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u16_result_err(result: U16Result) -> Error {
    result.err().unwrap()
}

// U32

/// Expanded generic for a result type containing a value of type u32.
pub type U32Result = Result<u32>;

/// Number of bytes required to store U32Result.
#[no_mangle]
pub static U32_RESULT_SIZE: usize = mem::size_of::<U32Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u32_result_is_ok(result: U32Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u32_result_is_err(result: U32Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u32_result_ok(result: U32Result) -> u32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u32_result_err(result: U32Result) -> Error {
    result.err().unwrap()
}

// U64

/// Expanded generic for a result type containing a value of type u64.
pub type U64Result = Result<u64>;

/// Number of bytes required to store U64Result.
#[no_mangle]
pub static U64_RESULT_SIZE: usize = mem::size_of::<U64Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u64_result_is_ok(result: U64Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u64_result_is_err(result: U64Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u64_result_ok(result: U64Result) -> u64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u64_result_err(result: U64Result) -> Error {
    result.err().unwrap()
}

// U128

/// Expanded generic for a result type containing a value of type u128.
#[cfg(has_i128)]
pub type U128Result = Result<u128>;

/// Number of bytes required to store U128Result.
#[cfg(has_i128)]
#[no_mangle]
pub static U128_RESULT_SIZE: usize = mem::size_of::<U128Result>();

/// Check if the result was ok.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128_result_is_ok(result: U128Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128_result_is_err(result: U128Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128_result_ok(result: U128Result) -> u128 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn u128_result_err(result: U128Result) -> Error {
    result.err().unwrap()
}

// USIZE

/// Expanded generic for a result type containing a value of type usize.
pub type UsizeResult = Result<usize>;

/// Number of bytes required to store UsizeResult.
#[no_mangle]
pub static USIZE_RESULT_SIZE: usize = mem::size_of::<UsizeResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn usize_result_is_ok(result: UsizeResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn usize_result_is_err(result: UsizeResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn usize_result_ok(result: UsizeResult) -> usize {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn usize_result_err(result: UsizeResult) -> Error {
    result.err().unwrap()
}

// I8

/// Expanded generic for a result type containing a value of type i8.
pub type I8Result = Result<i8>;

/// Number of bytes required to store I8Result.
#[no_mangle]
pub static I8_RESULT_SIZE: usize = mem::size_of::<I8Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i8_result_is_ok(result: I8Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i8_result_is_err(result: I8Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i8_result_ok(result: I8Result) -> i8 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i8_result_err(result: I8Result) -> Error {
    result.err().unwrap()
}

// I16

/// Expanded generic for a result type containing a value of type i16.
pub type I16Result = Result<i16>;

/// Number of bytes required to store I16Result.
#[no_mangle]
pub static I16_RESULT_SIZE: usize = mem::size_of::<I16Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i16_result_is_ok(result: I16Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i16_result_is_err(result: I16Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i16_result_ok(result: I16Result) -> i16 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i16_result_err(result: I16Result) -> Error {
    result.err().unwrap()
}

// I32

/// Expanded generic for a result type containing a value of type i32.
pub type I32Result = Result<i32>;

/// Number of bytes required to store I32Result.
#[no_mangle]
pub static I32_RESULT_SIZE: usize = mem::size_of::<I32Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i32_result_is_ok(result: I32Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i32_result_is_err(result: I32Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i32_result_ok(result: I32Result) -> i32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i32_result_err(result: I32Result) -> Error {
    result.err().unwrap()
}

// I64

/// Expanded generic for a result type containing a value of type i64.
pub type I64Result = Result<i64>;

/// Number of bytes required to store I64Result.
#[no_mangle]
pub static I64_RESULT_SIZE: usize = mem::size_of::<I64Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i64_result_is_ok(result: I64Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i64_result_is_err(result: I64Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i64_result_ok(result: I64Result) -> i64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i64_result_err(result: I64Result) -> Error {
    result.err().unwrap()
}

// I128

/// Expanded generic for a result type containing a value of type i128.
#[cfg(has_i128)]
pub type I128Result = Result<i128>;

/// Number of bytes required to store I128Result.
#[cfg(has_i128)]
#[no_mangle]
pub static I128_RESULT_SIZE: usize = mem::size_of::<I128Result>();

/// Check if the result was ok.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128_result_is_ok(result: I128Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128_result_is_err(result: I128Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128_result_ok(result: I128Result) -> i128 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[cfg(has_i128)]
#[no_mangle]
pub extern fn i128_result_err(result: I128Result) -> Error {
    result.err().unwrap()
}

// ISIZE

/// Expanded generic for a result type containing a value of type isize.
pub type IsizeResult = Result<isize>;

/// Number of bytes required to store IsizeResult.
#[no_mangle]
pub static ISIZE_RESULT_SIZE: usize = mem::size_of::<IsizeResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn isize_result_is_ok(result: IsizeResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn isize_result_is_err(result: IsizeResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn isize_result_ok(result: IsizeResult) -> isize {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn isize_result_err(result: IsizeResult) -> Error {
    result.err().unwrap()
}

// F32

/// Expanded generic for a result type containing a value of type f32.
pub type F32Result = Result<f32>;

/// Number of bytes required to store F32Result.
#[no_mangle]
pub static F32_RESULT_SIZE: usize = mem::size_of::<F32Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn f32_result_is_ok(result: F32Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn f32_result_is_err(result: F32Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn f32_result_ok(result: F32Result) -> f32 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn f32_result_err(result: F32Result) -> Error {
    result.err().unwrap()
}

// F64

/// Expanded generic for a result type containing a value of type f64.
pub type F64Result = Result<f64>;

/// Number of bytes required to store F64Result.
#[no_mangle]
pub static F64_RESULT_SIZE: usize = mem::size_of::<F64Result>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn f64_result_is_ok(result: F64Result) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn f64_result_is_err(result: F64Result) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn f64_result_ok(result: F64Result) -> f64 {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn f64_result_err(result: F64Result) -> Error {
    result.err().unwrap()
}

// PARTIAL
// -------

// U8

/// Expanded-generic for a tuple of (u8, usize).
pub type U8Tuple = Tuple<u8, usize>;

/// Number of bytes required to store U8Tuple.
#[no_mangle]
pub static U8_TUPLE_SIZE: usize = mem::size_of::<U8Tuple>();

/// Expanded generic for a result type containing a value of type U8Tuple.
pub type U8PartialResult = Result<U8Tuple>;

/// Number of bytes required to store U8PartialResult.
#[no_mangle]
pub static U8_PARTIAL_RESULT_SIZE: usize = mem::size_of::<U8PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u8_partial_result_is_ok(result: U8PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u8_partial_result_is_err(result: U8PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u8_partial_result_ok(result: U8PartialResult) -> U8Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u8_partial_result_err(result: U8PartialResult) -> Error {
    result.err().unwrap()
}

// U16

/// Expanded-generic for a tuple of (u16, usize).
pub type U16Tuple = Tuple<u16, usize>;

/// Number of bytes required to store U16Tuple.
#[no_mangle]
pub static U16_TUPLE_SIZE: usize = mem::size_of::<U16Tuple>();

/// Expanded generic for a result type containing a value of type U16Tuple.
pub type U16PartialResult = Result<U16Tuple>;

/// Number of bytes required to store U16PartialResult.
#[no_mangle]
pub static U16_PARTIAL_RESULT_SIZE: usize = mem::size_of::<U16PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u16_partial_result_is_ok(result: U16PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u16_partial_result_is_err(result: U16PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u16_partial_result_ok(result: U16PartialResult) -> U16Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u16_partial_result_err(result: U16PartialResult) -> Error {
    result.err().unwrap()
}

// U32

/// Expanded-generic for a tuple of (u32, usize).
pub type U32Tuple = Tuple<u32, usize>;

/// Number of bytes required to store U32Tuple.
#[no_mangle]
pub static U32_TUPLE_SIZE: usize = mem::size_of::<U32Tuple>();

/// Expanded generic for a result type containing a value of type U32Tuple.
pub type U32PartialResult = Result<U32Tuple>;

/// Number of bytes required to store U32PartialResult.
#[no_mangle]
pub static U32_PARTIAL_RESULT_SIZE: usize = mem::size_of::<U32PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u32_partial_result_is_ok(result: U32PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u32_partial_result_is_err(result: U32PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u32_partial_result_ok(result: U32PartialResult) -> U32Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u32_partial_result_err(result: U32PartialResult) -> Error {
    result.err().unwrap()
}

// U64

/// Expanded-generic for a tuple of (u64, usize).
pub type U64Tuple = Tuple<u64, usize>;

/// Number of bytes required to store U64Tuple.
#[no_mangle]
pub static U64_TUPLE_SIZE: usize = mem::size_of::<U64Tuple>();

/// Expanded generic for a result type containing a value of type U64Tuple.
pub type U64PartialResult = Result<U64Tuple>;

/// Number of bytes required to store U64PartialResult.
#[no_mangle]
pub static U64_PARTIAL_RESULT_SIZE: usize = mem::size_of::<U64PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn u64_partial_result_is_ok(result: U64PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn u64_partial_result_is_err(result: U64PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn u64_partial_result_ok(result: U64PartialResult) -> U64Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn u64_partial_result_err(result: U64PartialResult) -> Error {
    result.err().unwrap()
}

// Usize

/// Expanded-generic for a tuple of (usize, usize).
pub type UsizeTuple = Tuple<usize, usize>;

/// Number of bytes required to store UsizeTuple.
#[no_mangle]
pub static USIZE_TUPLE_SIZE: usize = mem::size_of::<UsizeTuple>();

/// Expanded generic for a result type containing a value of type UsizeTuple.
pub type UsizePartialResult = Result<UsizeTuple>;

/// Number of bytes required to store UsizePartialResult.
#[no_mangle]
pub static USIZE_PARTIAL_RESULT_SIZE: usize = mem::size_of::<UsizePartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn usize_partial_result_is_ok(result: UsizePartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn usize_partial_result_is_err(result: UsizePartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn usize_partial_result_ok(result: UsizePartialResult) -> UsizeTuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn usize_partial_result_err(result: UsizePartialResult) -> Error {
    result.err().unwrap()
}

// I8

/// Expanded-generic for a tuple of (i8, usize).
pub type I8Tuple = Tuple<i8, usize>;

/// Number of bytes required to store I8Tuple.
#[no_mangle]
pub static I8_TUPLE_SIZE: usize = mem::size_of::<I8Tuple>();

/// Expanded generic for a result type containing a value of type I8Tuple.
pub type I8PartialResult = Result<I8Tuple>;

/// Number of bytes required to store I8PartialResult.
#[no_mangle]
pub static I8_PARTIAL_RESULT_SIZE: usize = mem::size_of::<I8PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i8_partial_result_is_ok(result: I8PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i8_partial_result_is_err(result: I8PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i8_partial_result_ok(result: I8PartialResult) -> I8Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i8_partial_result_err(result: I8PartialResult) -> Error {
    result.err().unwrap()
}

// I16

/// Expanded-generic for a tuple of (i16, usize).
pub type I16Tuple = Tuple<i16, usize>;

/// Number of bytes required to store I16Tuple.
#[no_mangle]
pub static I16_TUPLE_SIZE: usize = mem::size_of::<I16Tuple>();

/// Expanded generic for a result type containing a value of type I16Tuple.
pub type I16PartialResult = Result<I16Tuple>;

/// Number of bytes required to store I16PartialResult.
#[no_mangle]
pub static I16_PARTIAL_RESULT_SIZE: usize = mem::size_of::<I16PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i16_partial_result_is_ok(result: I16PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i16_partial_result_is_err(result: I16PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i16_partial_result_ok(result: I16PartialResult) -> I16Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i16_partial_result_err(result: I16PartialResult) -> Error {
    result.err().unwrap()
}

// I32

/// Expanded-generic for a tuple of (i32, usize).
pub type I32Tuple = Tuple<i32, usize>;

/// Number of bytes required to store I32Tuple.
#[no_mangle]
pub static I32_TUPLE_SIZE: usize = mem::size_of::<I32Tuple>();

/// Expanded generic for a result type containing a value of type I32Tuple.
pub type I32PartialResult = Result<I32Tuple>;

/// Number of bytes required to store I32PartialResult.
#[no_mangle]
pub static I32_PARTIAL_RESULT_SIZE: usize = mem::size_of::<I32PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i32_partial_result_is_ok(result: I32PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i32_partial_result_is_err(result: I32PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i32_partial_result_ok(result: I32PartialResult) -> I32Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i32_partial_result_err(result: I32PartialResult) -> Error {
    result.err().unwrap()
}

// I64

/// Expanded-generic for a tuple of (i64, usize).
pub type I64Tuple = Tuple<i64, usize>;

/// Number of bytes required to store I64Tuple.
#[no_mangle]
pub static I64_TUPLE_SIZE: usize = mem::size_of::<I64Tuple>();

/// Expanded generic for a result type containing a value of type I64Tuple.
pub type I64PartialResult = Result<I64Tuple>;

/// Number of bytes required to store I64PartialResult.
#[no_mangle]
pub static I64_PARTIAL_RESULT_SIZE: usize = mem::size_of::<I64PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn i64_partial_result_is_ok(result: I64PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn i64_partial_result_is_err(result: I64PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn i64_partial_result_ok(result: I64PartialResult) -> I64Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn i64_partial_result_err(result: I64PartialResult) -> Error {
    result.err().unwrap()
}

// Isize

/// Expanded-generic for a tuple of (isize, usize).
pub type IsizeTuple = Tuple<isize, usize>;

/// Number of bytes required to store IsizeTuple.
#[no_mangle]
pub static ISIZE_TUPLE_SIZE: usize = mem::size_of::<IsizeTuple>();

/// Expanded generic for a result type containing a value of type IsizeTuple.
pub type IsizePartialResult = Result<IsizeTuple>;

/// Number of bytes required to store IsizePartialResult.
#[no_mangle]
pub static ISIZE_PARTIAL_RESULT_SIZE: usize = mem::size_of::<IsizePartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn isize_partial_result_is_ok(result: IsizePartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn isize_partial_result_is_err(result: IsizePartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn isize_partial_result_ok(result: IsizePartialResult) -> IsizeTuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn isize_partial_result_err(result: IsizePartialResult) -> Error {
    result.err().unwrap()
}

// F32

/// Expanded-generic for a tuple of (f32, usize).
pub type F32Tuple = Tuple<f32, usize>;

/// Number of bytes required to store F32Tuple.
#[no_mangle]
pub static F32_TUPLE_SIZE: usize = mem::size_of::<F32Tuple>();

/// Expanded generic for a result type containing a value of type F32Tuple.
pub type F32PartialResult = Result<F32Tuple>;

/// Number of bytes required to store F32PartialResult.
#[no_mangle]
pub static F32_PARTIAL_RESULT_SIZE: usize = mem::size_of::<F32PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn f32_partial_result_is_ok(result: F32PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn f32_partial_result_is_err(result: F32PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn f32_partial_result_ok(result: F32PartialResult) -> F32Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn f32_partial_result_err(result: F32PartialResult) -> Error {
    result.err().unwrap()
}

// F64

/// Expanded-generic for a tuple of (f64, usize).
pub type F64Tuple = Tuple<f64, usize>;

/// Number of bytes required to store F64Tuple.
#[no_mangle]
pub static F64_TUPLE_SIZE: usize = mem::size_of::<F64Tuple>();

/// Expanded generic for a result type containing a value of type F64Tuple.
pub type F64PartialResult = Result<F64Tuple>;

/// Number of bytes required to store F64PartialResult.
#[no_mangle]
pub static F64_PARTIAL_RESULT_SIZE: usize = mem::size_of::<F64PartialResult>();

/// Check if the result was ok.
#[no_mangle]
pub extern fn f64_partial_result_is_ok(result: F64PartialResult) -> bool {
    result.is_ok()
}

/// Check if the result was an error.
#[no_mangle]
pub extern fn f64_partial_result_is_err(result: F64PartialResult) -> bool {
    result.is_err()
}

/// Get the value from the result. Panics if the result is an error.
#[no_mangle]
pub extern fn f64_partial_result_ok(result: F64PartialResult) -> F64Tuple {
    result.ok().unwrap()
}

/// Get the error from the result. Panics if the result is successful.
#[no_mangle]
pub extern fn f64_partial_result_err(result: F64PartialResult) -> Error {
    result.err().unwrap()
}

}   // result_ffi
