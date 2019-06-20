//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod exponent;

cfg_if! {
if #[cfg(feature = "correct")] {
pub(crate) mod bhcomp;
pub(crate) mod bigcomp;
mod alias;
mod bigint;
mod cached;
mod cached_float80;
mod large_powers;
mod math;
mod small_powers;

#[cfg(has_i128)]
mod cached_float160;

#[cfg(limb_width_32)]
mod large_powers_32;

#[cfg(limb_width_32)]
mod small_powers_32;

#[cfg(limb_width_64)]
mod large_powers_64;

// Required for fast-path, keep on all platforms.
mod small_powers_64;

}}  // cfg_if


// Export algorithms.
#[cfg(feature = "correct")]
pub(crate) mod correct;

#[cfg(not(feature = "correct"))]
pub(crate) mod incorrect;
