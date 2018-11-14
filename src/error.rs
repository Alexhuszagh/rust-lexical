//! Error definitions for lexical.

use lib::fmt;

// ERROR

/// Type of error encountered during numeric parsing.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    /// Integral overflow occurred during numeric parsing.
    Overflow,
    /// Invalid digit occurred before string termination.
    InvalidDigit(usize),
}

/// Custom error for numeric parsing.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Error(ErrorKind);

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error(kind)
    }
}

impl Error {
    /// Get error type.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind() {
            ErrorKind::Overflow        => write!(f, "lexical error: integer overflow occurred during integer parsing."),
            ErrorKind::InvalidDigit(u) => write!(f, "lexical error: invalid digit found at {}.", u),
        }
    }
}

// HELPERS

// Internal helper methods for testing.

/// Return an overflow error.
#[cfg(test)]
pub(crate) fn overflow() -> Error {
    From::from(ErrorKind::Overflow)
}

/// Return an invalid digit error.
#[cfg(test)]
pub(crate) fn invalid_digit(position: usize) -> Error {
    From::from(ErrorKind::InvalidDigit(position))
}
