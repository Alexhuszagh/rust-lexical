//! Fast lexical string-to-float conversion routines.

// Hide implementation details.
mod algorithm;
mod api;

// Re-exports
pub use self::api::*;

// Exposed for benchmarking only.
#[doc(hidden)]
#[cfg(feature = "correct")]
pub use self::algorithm::bigcomp::{fast_atof as bigcomp_fast_atof, slow_atof as bigcomp_slow_atof};

// Exposed for benchmarking only
#[doc(hidden)]
#[cfg(feature = "algorithm_m")]
pub use self::algorithm::algorithm_m::{atof as algom_atof};
