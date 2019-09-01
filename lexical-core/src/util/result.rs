//! Parser result type.

use lib::result::Result as StdResult;
use super::error::Error;

/// A specialized Result type for lexical operations.
pub type Result<T> = StdResult<T, Error>;
