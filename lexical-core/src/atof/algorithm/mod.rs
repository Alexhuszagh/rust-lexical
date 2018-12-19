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

#[cfg(not(target_arch = "x86_64"))]
mod large_powers_32;

#[cfg(not(target_arch = "x86_64"))]
mod small_powers_32;

#[cfg(target_arch = "x86_64")]
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
