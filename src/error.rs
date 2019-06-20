//! Error definitions for lexical.

use lib::fmt;
use lib::error::Error as StdError;

// ERROR

/// Type of error encountered during numeric parsing.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    /// Integral overflow occurred during numeric parsing.
    Overflow,
    /// Invalid digit occurred before string termination.
    InvalidDigit(usize),
    /// Empty byte array found.
    Empty,

    // We may add additional variants later, so ensure that client matching
    // does not depend on exhaustive matching.
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Custom error for numeric parsing.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Error(ErrorKind);

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Self {
        Error(kind)
    }
}

impl Error {
    /// Get error type.
    #[inline]
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            &ErrorKind::Overflow        => write!(f, "lexical error: integer overflow occurred during integer parsing."),
            &ErrorKind::InvalidDigit(u) => write!(f, "lexical error: invalid digit found at {}.", u),
            &ErrorKind::Empty           => write!(f, "lexical error: empty input data."),
            _                           => unreachable!(),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind() {
            &ErrorKind::Overflow        => "lexical error: integer overflow occurred during integer parsing.",
            &ErrorKind::InvalidDigit(_) => "lexical error: invalid digit found in string.",
            &ErrorKind::Empty           => "lexical error: empty input data.",
            _                           => unreachable!(),
        }
    }

    // Remove when we drop support below 1.27.
    #[allow(bare_trait_objects)]
    fn cause(&self) -> Option<&StdError> {
        None
    }
}

// HELPERS

// Internal helper methods for error creation.

/// Return an overflow error.
#[inline]
pub(crate) fn overflow() -> Error {
    ErrorKind::Overflow.into()
}

/// Return an invalid digit error.
#[inline]
pub(crate) fn invalid_digit(position: usize) -> Error {
    ErrorKind::InvalidDigit(position).into()
}

/// Return an empty error.
#[inline]
pub(crate) fn empty() -> Error {
    ErrorKind::Empty.into()
}
