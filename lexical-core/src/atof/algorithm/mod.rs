//! Algorithms for parsing strings to floats.

// Hide implementation details.
#[macro_use]
mod format;

mod alias;
mod cached;
mod mantissa;
mod math;
mod powers;

// Export high-level algorithms.
pub(crate) mod correct;
pub(crate) mod incorrect;

// Re-export the float type.
pub(crate) use self::alias::FloatType;
pub(crate) use self::cached::ModeratePathCache;
pub(crate) use self::format::*;

// ALGORITHMS
// ----------
// Dispatchers.
mod power_of_n;
#[cfg(feature = "power_of_two")]
mod power_of_two;

// Fast-path
mod fast;

// Moderate-Path
mod extended_float;
mod lemire; // TODO(ahuszagh) Implement...

// Slow-Path
mod bhcomp;
mod bigcomp;
mod bignum;

// Internal implementation details.
// These algorithms are no longer used, but they are useful.
// Feature-gate them for testing.
#[cfg(feature = "algorithm_m")]
mod algorithm_m;
