//! Result type for numeric parsing functions.

#![cfg(feature = "parse")]

use crate::error;
use crate::lib::result;

/// A specialized Result type for lexical operations.
pub type ParseResult<T> = result::Result<T, error::ParseError>;

// TODO(ahuszagh) Should probably have result types for:
//  NumberFormat
//  Options
//  Can only do after refactor.
