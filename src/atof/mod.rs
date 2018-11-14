//! Fast lexical string-to-float conversion routines.

// Hide implementation details.
mod algorithm;
mod api;
mod lossy;

#[cfg(feature = "correct")]
mod correct;

// Re-exports
pub use self::api::*;
