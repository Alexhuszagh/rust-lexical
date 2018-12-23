//! C-compatible error type.

/// Error code, indicating success or failure.
///
/// Success, or no error, is 0, while error messages are designating
/// by an error code of less than 0. This is to be compatible with C
/// conventions.
///
/// # FFI
///
/// For interfacing with FFI-code, this may be approximated by:
/// ```text
/// const int32_t SUCCESS = 0;
/// const int32_t OVERFLOW = -1;
/// const int32_t INVALID_DIGIT = -2;
/// const int32_t EMPTY = -3;
/// ```
///
/// # Safety
///
/// Assigning any value outside the range `[-3, 0]` to value of type
/// ErrorCode may invoke undefined-behavior.
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ErrorCode {
    /// No error, success.
    Success = 0,
    /// Integral overflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Overflow = -1,
    /// Invalid digit found before string termination.
    InvalidDigit = -2,
    /// Empty byte array found.
    Empty = -3,

    // We may add additional variants later, so ensure that client matching
    // does not depend on exhaustive matching.
    #[doc(hidden)]
    __Nonexhaustive = -4,
}

/// C-compatible error for FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Error {
    /// Error code designating the type of error occurred.
    pub code: ErrorCode,
    /// Optional position within the buffer for the error.
    pub index: usize,
}

/// Check if the error code is successful.
#[no_mangle]
pub extern fn is_success(error: Error) -> bool {
    error.code == ErrorCode::Success
}

/// Check if the error code designates integer overflow.
#[no_mangle]
pub extern fn is_overflow(error: Error) -> bool {
    error.code == ErrorCode::Overflow
}

/// Check if the error code designates an invalid digit was encountered.
#[no_mangle]
pub extern fn is_invalid_digit(error: Error) -> bool {
    error.code == ErrorCode::InvalidDigit
}

/// Check if the error code designates an empty byte array was encountered.
#[no_mangle]
pub extern fn is_empty(error: Error) -> bool {
    error.code == ErrorCode::Empty
}

/// Helper function to create a success message.
#[inline]
pub(crate) fn success() -> Error {
    Error { code: ErrorCode::Success, index: 0 }
}

/// Helper function to create an overflow error.
#[inline]
pub(crate) fn overflow_error() -> Error {
    Error { code: ErrorCode::Overflow, index: 0 }
}

/// Helper function to create an invalid digit error.
#[inline]
pub(crate) fn invalid_digit_error(index: usize) -> Error {
    Error { code: ErrorCode::InvalidDigit, index: index }
}

/// Helper function to create an empty error.
#[inline]
pub(crate) fn empty_error() -> Error {
    Error { code: ErrorCode::Empty, index: 0 }
}
