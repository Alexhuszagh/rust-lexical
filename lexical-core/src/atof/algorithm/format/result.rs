//! Result and error types for format parsers.

use crate::util::*;
use crate::lib::result::Result as StdResult;

/// Specialized error type for format parsers.
pub(crate) type ParseError = (ErrorCode, *const u8);

/// Specialized result type for format parsers.
pub(crate) type ParseResult<T> = StdResult<T, ParseError>;
