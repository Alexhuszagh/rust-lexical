//! Parser result type.

use crate::lib::result::Result as StdResult;
use super::error::{Error, ErrorCode};

/// A specialized Result type for lexical operations.
#[cfg(any(feature = "atof", feature = "atoi"))]
pub type Result<T> = StdResult<T, Error>;

/// Specialized error type for format parsers.
#[cfg(any(feature = "atof", feature = "atoi"))]
pub(crate) type ParseError = (ErrorCode, *const u8);

/// Specialized result type for format parsers.
#[cfg(any(feature = "atof", feature = "atoi"))]
pub(crate) type ParseResult<T> = StdResult<T, ParseError>;

/// Type definition for result when testing parsing.
#[cfg(all(test, feature = "atof"))]
pub(crate) type ParseTestResult<T> = StdResult<T, ErrorCode>;
