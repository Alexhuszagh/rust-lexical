//! Fast lexical string-to-float conversion routines.

// Hide implementation details.
mod algorithm;
mod lossy;

#[cfg(feature = "correct")]
mod correct;

mod api;

// Re-exports
pub use self::api::*;
