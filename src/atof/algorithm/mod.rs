//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod exponent;

cfg_if! {
if #[cfg(any(test, feature = "correct"))] {
// Needed for the actual items.
mod cached;
mod cached80;
mod cached160;
}}  // cfg_if

// Export algorithms.
#[cfg(any(test, feature = "correct"))]
pub(crate) mod correct;

#[cfg(any(test, not(feature = "correct")))]
pub(crate) mod lossy;
