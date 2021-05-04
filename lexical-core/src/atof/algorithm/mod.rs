//! Algorithms for parsing strings to floats.

// Hide implementation details.
#[macro_use]
mod format;

mod alias;
mod bhcomp;
mod bigcomp;
mod bignum;
mod cached;
mod errors;
mod math;
mod powers;

// Export algorithms.
pub(crate) mod correct;
pub(crate) mod incorrect;

// Re-export the float type.
pub(crate) use self::alias::FloatType;
pub(crate) use self::cached::ModeratePathCache;
pub(crate) use self::format::*;

// Internal implementation details.
// These algorithms are no longer used, but they are useful.
// Feature-gate them for testing.
#[cfg(feature = "algorithm_m")]
mod algorithm_m;
