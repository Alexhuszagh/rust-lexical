//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod overflowing;

// TODO(ahuszagh) Always expose this...
cfg_if! {
    if #[cfg(any(test, feature = "correct"))] {
        mod cached;
        mod decimal;
        mod double;
        mod float;
    }
}

// Export algorithms.
pub(crate) mod correct;
pub(crate) mod lossy;
