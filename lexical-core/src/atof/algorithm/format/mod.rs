//! Parsers for different formats.

#[macro_use]
mod shared;
mod standard;

// Re-export formats.
pub(super) use shared::FormatParser;
pub(super) use standard::*;
