//! Fast lexical string-to-float conversion routines.

// Hide implementation details.
mod algorithm;
mod api;

// Re-exports
pub use self::api::*;

// Exposed for benchmarking only.
#[doc(hidden)]
#[cfg(not(feature = "imprecise"))]
pub use self::algorithm::bigfloat::Bigfloat;
