//! Cached powers trait for extended-precision floats.

// Just flatten everything out, simplifies the cfg_if logic.
mod cache;

cfg_if! {
if #[cfg(all(feature = "radix", feature = "f128"))] {
    // Use everything.
    mod float80;
    mod float80_decimal;
    mod float80_radix;
    mod float160;
    mod float160_decimal;
    mod float160_radix;
} else if #[cfg(feature = "radix")] {
    // Disable float160*.
    mod float80;
    mod float80_decimal;
    mod float80_radix;
} else if #[cfg(feature = "f128")] {
    // Disable radix only.
    mod float80;
    mod float80_decimal;
    mod float160;
    mod float160_decimal;
} else {
    // Only enable float80 and float80 decimal.
    mod float80;
    mod float80_decimal;
}}  // cfg_if

// Re-export everything
pub use self::cache::*;
