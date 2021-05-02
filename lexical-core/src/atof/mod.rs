//! Fast lexical string-to-float conversion routines.

// Hide implementation details.
#[macro_use]
mod algorithm;
mod api;

// Re-exports
pub use self::api::*;
