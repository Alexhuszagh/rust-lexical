//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod overflowing;

cfg_if! {
    if #[cfg(any(test, feature = "correct"))] {
        mod bigint;
        mod double;
        mod float;
    }
}

// Export algorithms.
pub(crate) mod correct;
pub(crate) mod lossy;
