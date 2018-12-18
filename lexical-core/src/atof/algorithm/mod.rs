//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod exponent;

#[cfg(any(feature = "algorithm_m", feature = "bhcomp"))]
mod bigint;

cfg_if! {
if #[cfg(feature = "correct")] {
pub(crate) mod bigcomp;
mod cached;
mod cached_float80;
mod cached_float160;
mod large_powers;
mod math;
mod small_powers;

#[cfg(target_pointer_width = "16")]
mod large_powers_16;

#[cfg(target_pointer_width = "16")]
mod small_powers_16;

#[cfg(target_pointer_width = "32")]
mod large_powers_32;

#[cfg(target_pointer_width = "32")]
mod small_powers_32;

#[cfg(target_pointer_width = "64")]
mod large_powers_64;

// Required for fast-path, keep on all platforms.
mod small_powers_64;

}}  // cfg_if

#[cfg(feature = "algorithm_m")]
pub(crate) mod algorithm_m;

#[cfg(feature = "bhcomp")]
pub(crate) mod bhcomp;

// Export algorithms.
#[cfg(feature = "correct")]
pub(crate) mod correct;

#[cfg(not(feature = "correct"))]
pub(crate) mod incorrect;
