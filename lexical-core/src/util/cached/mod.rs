//! Cached powers trait for extended-precision floats.

use crate::util::float::*;
use crate::util::traits::*;

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
}} // cfg_if

// MODERATE PATH POWERS
// --------------------

/// Precalculated powers of base N for the moderate path.
#[doc(hidden)]
pub struct ModeratePathPowers<M: Mantissa> {
    // Pre-calculated small powers.
    pub small: &'static [M],
    // Pre-calculated large powers.
    pub large: &'static [M],
    /// Pre-calculated small powers as 64-bit integers
    pub small_int: &'static [M],
    // Step between large powers and number of small powers.
    pub step: i32,
    // Exponent bias for the large powers.
    pub bias: i32,
    /// ceil(log2(radix)) scaled as a multiplier.
    pub log2: i64,
    /// Bitshift for the log2 multiplier.
    pub log2_shift: i32,
}

/// Allow indexing of values without bounds checking
impl<M: Mantissa> ModeratePathPowers<M> {
    #[inline]
    pub fn get_small(&self, index: usize) -> ExtendedFloat<M> {
        let mant = self.small[index];
        let exp = (1 - M::FULL as i64) + ((self.log2 * index as i64) >> self.log2_shift);
        ExtendedFloat {
            mant,
            exp: exp as i32,
        }
    }

    #[inline]
    pub fn get_large(&self, index: usize) -> ExtendedFloat<M> {
        let mant = self.large[index];
        let biased_e = index as i64 * self.step as i64 - self.bias as i64;
        let exp = (1 - M::FULL as i64) + ((self.log2 * biased_e) >> self.log2_shift);
        ExtendedFloat {
            mant,
            exp: exp as i32,
        }
    }

    #[inline]
    pub fn get_small_int(&self, index: usize) -> M {
        self.small_int[index]
    }
}

// CACHED EXTENDED POWERS
// ----------------------

/// Cached powers as a trait for a floating-point type.
pub trait ModeratePathCache: Mantissa {
    /// Get powers from radix.
    fn get_powers(radix: u32) -> &'static ModeratePathPowers<Self>;
}

impl ModeratePathCache for u64 {
    #[inline]
    fn get_powers(radix: u32) -> &'static ModeratePathPowers<u64> {
        float80::get_powers(radix)
    }
}

#[cfg(feature = "f128")]
impl ModeratePathCache for u128 {
    #[inline]
    fn get_powers(radix: u32) -> &'static ModeratePathPowers<u128> {
        float160::get_powers(radix)
    }
}
