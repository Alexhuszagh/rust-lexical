//! C-compatible error type.

/// Error code, indicating success or failure.
#[repr(i8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ErrorCode {
    /// No error, success.
    Success = 0,
    /// Integral overflow occurred during numeric parsing.
    Overflow = -1,
    /// Invalid digit occurred before string termination.
    InvalidDigit = -2,
}

/// C-compatible error for FFI.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Error {
    /// Error code for the message.
    pub code: ErrorCode,
    /// Optional position for the error.
    pub index: usize,
}

/// Check if an error message was successful.
#[inline(always)]
pub extern "C" fn is_success(error: Error) -> bool {
    error.code == ErrorCode::Success
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
