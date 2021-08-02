//! Result type for numeric parsing functions.

use crate::error;
use core::result;

/// A specialized Result type for lexical operations.
pub type Result<T> = result::Result<T, error::Error>;
