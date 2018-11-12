//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod overflowing;

#[cfg(feature = "correct")]
mod bigint;

// Export algorithms.
pub(crate) mod correct;
pub(crate) mod lossy;
