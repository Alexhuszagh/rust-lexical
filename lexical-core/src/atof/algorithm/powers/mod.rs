//! Precalculated powers for performance gain.

// Hide implementation details.
mod large;
mod small;

// Always export, since it's required for the fast-path algorithm.
mod small64_decimal;
#[cfg(feature = "radix")]
mod small64_radix;

cfg_if! {
if #[cfg(limb_width_32)] {
    mod large32_decimal;
    mod small32_decimal;
    cfg_if! {
    if #[cfg(feature = "radix")] {
        mod large32_radix;
        mod small32_radix;
    }}  // cfg_if
} else {
    mod large64_decimal;
    #[cfg(feature = "radix")]
    mod large64_radix;
}} // cfg_if

// Re-export methods.
pub(crate) use self::large::*;
pub(crate) use self::small::*;
