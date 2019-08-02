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
    /// Integral overflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Overflow = -1,
    /// Integral underflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Underflow = -2,
    /// Invalid digit found before string termination.
    InvalidDigit = -3,
    /// Empty byte array found.
    Empty = -4,
    /// Empty fraction found.
    EmptyFraction = -5,
    /// Empty exponent found.
    EmptyExponent = -6,

    // We may add additional variants later, so ensure that client matching
    // does not depend on exhaustive matching.
    #[doc(hidden)]
    __Nonexhaustive = -7,
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

impl From<ErrorCode> for Error {
    #[inline]
    fn from(code: ErrorCode) -> Self {
        Error { code: code, index: 0 }
    }
}

impl From<(ErrorCode, usize)> for Error {
    #[inline]
    fn from(error: (ErrorCode, usize)) -> Self {
        Error { code: error.0, index: error.1 }
    }
}

/// Check if the error code designates integer overflow.
#[no_mangle]
pub extern fn error_is_overflow(error: Error) -> bool {
    error.code == ErrorCode::Overflow
}

/// Check if the error code designates integer underflow.
#[no_mangle]
pub extern fn error_is_underflow(error: Error) -> bool {
    error.code == ErrorCode::Underflow
}

/// Check if the error code designates an invalid digit was encountered.
#[no_mangle]
pub extern fn error_is_invalid_digit(error: Error) -> bool {
    error.code == ErrorCode::InvalidDigit
}

/// Check if the error code designates an empty byte array was encountered.
#[no_mangle]
pub extern fn error_is_empty(error: Error) -> bool {
    error.code == ErrorCode::Empty
}

/// Check if the error code designates an empty fraction was encountered.
#[no_mangle]
pub extern fn error_is_empty_fraction(error: Error) -> bool {
    error.code == ErrorCode::EmptyFraction
}

/// Check if the error code designates an empty exponent was encountered.
#[no_mangle]
pub extern fn error_is_empty_exponent(error: Error) -> bool {
    error.code == ErrorCode::EmptyExponent
}
