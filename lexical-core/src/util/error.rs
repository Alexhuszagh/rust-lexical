//! C-compatible error type.

/// Error code, indicating success or failure.
///
/// Success, or no error, is 0, while error messages are designating
/// by an error code of less than 0. This is to be compatible with C
/// conventions.
#[repr(i8)]
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
#[inline(always)]
pub extern "C" fn is_success(error: Error) -> bool {
    error.code == ErrorCode::Success
}

/// Check if the error code designates integer overflow.
#[inline(always)]
pub extern "C" fn is_overflow(error: Error) -> bool {
    error.code == ErrorCode::Overflow
}

/// Check if the error code designates an invalid digit was encountered.
#[inline(always)]
pub extern "C" fn is_invalid_digit(error: Error) -> bool {
    error.code == ErrorCode::InvalidDigit
}

/// Helper function to create a success message.
#[inline(always)]
pub(crate) fn success() -> Error {
    Error { code: ErrorCode::Success, index: 0 }
}

/// Helper function to create an overflow error.
#[inline(always)]
pub(crate) fn overflow_error() -> Error {
    Error { code: ErrorCode::Overflow, index: 0 }
}

/// Helper function to create an invalid digit error.
#[inline(always)]
pub(crate) fn invalid_digit_error(index: usize) -> Error {
    Error { code: ErrorCode::InvalidDigit, index: index }
}