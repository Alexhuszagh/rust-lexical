//! Parsers for different formats.

// Parsing utilities.
mod consume;
mod exponent;
mod result;
mod traits;
mod trim;
mod validate;

// Formats.
mod standard;

// Re-export formats.
pub(super) use result::*;
pub(super) use traits::*;
pub(super) use standard::*;
