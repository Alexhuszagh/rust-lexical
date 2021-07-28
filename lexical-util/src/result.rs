//! Result type for numeric parsing functions.

#![cfg(feature = "parse")]

use crate::error;
use crate::lib::result;

/// A specialized Result type for lexical operations.
pub type Result<T> = result::Result<T, error::Error>;

// TODO(ahuszagh) Should probably have result types for:
//  Options
//  Can only do after refactor.
//      Might mean this isn't parse-dependent anymore.
