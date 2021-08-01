//! Result type for numeric parsing functions.

#![cfg(feature = "parse")]

use crate::error;
use crate::lib::result;

/// A specialized Result type for lexical operations.
pub type Result<T> = result::Result<T, error::Error>;
