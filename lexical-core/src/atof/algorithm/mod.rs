//! Algorithms for parsing strings to floats.

// Hide implementation details.
#[macro_use]
mod format;

mod alias;
mod bhcomp;
mod bigcomp;
mod bignum;
mod cached;
mod cached_float160;
mod cached_float80;
mod errors;
mod large_powers;
mod math;
mod small_powers;
// Required for fast-path, keep on all platforms.
mod small_powers_64;

cfg_if! {
if #[cfg(limb_width_32)] {
    mod small_powers_32;
    mod large_powers_32;
} else {
    mod large_powers_64;
}} // cfg_if

// Export algorithms.
pub(crate) mod correct;
pub(crate) mod incorrect;

// Re-export the float type.
pub(crate) use self::alias::FloatType;
pub(crate) use self::format::*;
