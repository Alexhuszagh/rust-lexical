//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod exponent;

cfg_if! {
if #[cfg(any(test, not(feature = "imprecise")))] {
// Needed for the actual items.
mod bigfloat;
mod cached;
mod cached80;
mod cached160;
}}  // cfg_if

// Export algorithms.
#[cfg(any(test, not(feature = "imprecise")))]
pub(crate) mod precise;

#[cfg(any(test, feature = "imprecise"))]
pub(crate) mod imprecise;
