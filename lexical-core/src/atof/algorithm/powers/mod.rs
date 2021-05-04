//! Precalculated powers for performance gain.

// Hide implementation details.
mod large;
mod small;

// Always export, since it's required for the fast-path algorithm.
#[cfg(feature = "power_of_two")]
mod small64_binary;
mod small64_decimal;
#[cfg(feature = "radix")]
mod small64_radix;

// Limb-width dependent types.
cfg_if! {
if #[cfg(limb_width_32)] {
    mod large32_decimal;
    mod small32_decimal;
    #[cfg(feature = "power_of_two")]
    mod small32_binary;
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

// Only for fast-path for f128.
cfg_if! {
if #[cfg(feature = "f128")] {
    mod small128_decimal;
    #[cfg(feature = "power_of_two")]
    mod small128_binary;
    #[cfg(feature = "radix")]
    mod small128_radix;
}}  // cfg_if

// Re-export methods.
pub(crate) use self::large::*;
pub(crate) use self::small::*;
