//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod state;

cfg_if! {
if #[cfg(feature = "correct")] {
pub(crate) mod bhcomp;
pub(crate) mod bigcomp;
mod alias;
mod bignum;
mod cached;
mod cached_float80;
mod cached_float160;
mod errors;
mod exponent;
mod large_powers;
mod math;
mod small_powers;

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "powerpc64",
    target_arch = "x86_64"
)))]
mod large_powers_32;

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "powerpc64",
    target_arch = "x86_64"
)))]
mod small_powers_32;

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "powerpc64",
    target_arch = "x86_64"
))]
mod large_powers_64;

// Required for fast-path, keep on all platforms.
mod small_powers_64;

}}  // cfg_if

// Export algorithms.
#[cfg(feature = "correct")]
pub(crate) mod correct;

#[cfg(not(feature = "correct"))]
pub(crate) mod incorrect;

