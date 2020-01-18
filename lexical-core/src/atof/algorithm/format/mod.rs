//! Module specifying float.

// Utilities.
mod consume;
mod exponent;
mod iterator;
mod result;
mod trim;
mod validate;

#[macro_use]
mod traits;

// Formats
mod standard;

// Re-export formats and traits.
pub(super) use standard::*;
pub(super) use result::*;
pub(super) use traits::*;
