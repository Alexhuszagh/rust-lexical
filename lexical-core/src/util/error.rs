//! C-compatible error type.

use crate::lib::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error as StdError;

/// Error code, indicating failure type.
///
/// Error messages are designating by an error code of less than 0.
/// This is to be compatible with C conventions. This enumeration is
/// FFI-compatible for interfacing with C code.
///
/// # FFI
///
/// For interfacing with FFI-code, this may be approximated by:
/// ```text
/// const int32_t OVERFLOW = -1;
/// const int32_t UNDERFLOW = -2;
/// const int32_t INVALID_DIGIT = -3;
/// const int32_t EMPTY = -4;
/// const int32_t EMPTY_MANTISSA = -5;
/// const int32_t EMPTY_EXPONENT = -6;
/// const int32_t EMPTY_INTEGER = -7;
/// const int32_t EMPTY_FRACTION = -8;
/// const int32_t INVALID_POSITIVE_MANTISSA_SIGN = -9;
/// const int32_t MISSING_MANTISSA_SIGN = -10;
/// const int32_t INVALID_EXPONENT = -11;
/// const int32_t INVALID_POSITIVE_EXPONENT_SIGN = -12;
/// const int32_t MISSING_EXPONENT_SIGN = -13;
/// const int32_t EXPONENT_WITHOUT_FRACTION = -14;
/// const int32_t INVALID_LEADING_ZEROS = -15;
/// const int32_t MISSING_EXPONENT = -16;
/// ```
///
/// # Safety
///
/// Assigning any value outside the range `[-15, -1]` to value of type
/// ParseErrorCode may invoke undefined-behavior.
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParseErrorCode {
    /// Integral overflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Overflow                    = -1,
    /// Integral underflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Underflow                   = -2,
    /// Invalid digit found before string termination.
    InvalidDigit                = -3,
    /// Empty byte array found.
    Empty                       = -4,
    /// Empty mantissa found.
    EmptyMantissa               = -5,
    /// Empty exponent found.
    EmptyExponent               = -6,
    /// Empty integer found.
    EmptyInteger                = -7,
    /// Empty fraction found.
    EmptyFraction               = -8,
    /// Invalid positive mantissa sign was found.
    InvalidPositiveMantissaSign = -9,
    /// Mantissa sign was required, but not found.
    MissingMantissaSign         = -10,
    /// Exponent was present but not allowed.
    InvalidExponent             = -11,
    /// Invalid positive exponent sign was found.
    InvalidPositiveExponentSign = -12,
    /// Exponent sign was required, but not found.
    MissingExponentSign         = -13,
    /// Exponent was present without fraction component.
    ExponentWithoutFraction     = -14,
    /// Integer had invalid leading zeros.
    InvalidLeadingZeros         = -15,
    /// No exponent with required exponent notation.
    MissingExponent             = -16,

    // We may add additional variants later, so ensure that client matching
    // does not depend on exhaustive matching.
    #[doc(hidden)]
    __Nonexhaustive             = -200,
}

/// Error type for lexical parsing.
///
/// This error is FFI-compatible for interfacing with C code.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ParseError {
    /// Error code designating the type of error occurred.
    pub code: ParseErrorCode,
    /// Optional position within the buffer for the error.
    pub index: usize,
}

impl From<ParseErrorCode> for ParseError {
    #[inline]
    fn from(code: ParseErrorCode) -> Self {
        ParseError {
            code,
            index: 0,
        }
    }
}

impl From<(ParseErrorCode, usize)> for ParseError {
    #[inline]
    fn from(error: (ParseErrorCode, usize)) -> Self {
        ParseError {
            code: error.0,
            index: error.1,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "lexical error: {:?} at index {}.", self.code, self.index)
    }
}

#[cfg(feature = "std")]
impl StdError for ParseError {
}

// AlIASES
// -------

/// Alias for ParseErrorCode for backwards compatibility.
pub type ErrorCode = ParseErrorCode;

/// Alias for ParseError for backwards compatibility.
pub type Error = ParseError;
