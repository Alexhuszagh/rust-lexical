//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod exponent;

cfg_if! {
if #[cfg(any(test, not(feature = "imprecise")))] {
// Needed for the actual items.
pub(crate) mod bigcomp;
mod cached;
mod cached_float80;
mod cached_float160;
mod math;
mod small_powers;
}}  // cfg_if

// Export algorithms.
#[cfg(any(test, not(feature = "imprecise")))]
pub(crate) mod precise;

#[cfg(any(test, feature = "imprecise"))]
pub(crate) mod imprecise;
