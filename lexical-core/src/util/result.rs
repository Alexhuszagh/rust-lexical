//! Parser result type.

use crate::lib::result::Result as StdResult;

use super::error::*;

/// A specialized Result type for lexical operations.
pub type ParseResult<T> = StdResult<T, ParseError>;

/// Alias of ParseResult for backwards compatibility.
pub type Result<T> = ParseResult<T>;

/// Specialized result type for number parsers.
pub(crate) type ParseTupleResult<T> = StdResult<T, (ParseErrorCode, *const u8)>;

/// Type definition for result when testing parsing.
#[cfg(test)]
pub(crate) type ParseTestResult<T> = StdResult<T, ParseErrorCode>;
