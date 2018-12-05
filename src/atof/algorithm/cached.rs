//! Cached powers trait for extended-precision floats.

use float::{ExtendedFloat, Mantissa};
use super::{cached80, cached160};

// POWERS

/// Precalculated powers of base N.
#[doc(hidden)]
pub(crate) struct Powers<M: Mantissa> {
    // Pre-calculated small powers.
    pub small: &'static [ExtendedFloat<M>],
    /// Pre-calculated small powers as 64-bit integers
    pub small_int: &'static [M],
    // Pre-calculated large powers.
    pub large: &'static [ExtendedFloat<M>],
    // Step between large powers and number of small powers.
    pub step: i32,
    // Exponent bias for the large powers.
    pub bias: i32,
}

/// Allow indexing of values without bounds checking
impl<M: Mantissa> Powers<M> {
    #[inline(always)]
    pub unsafe fn get_small(&self, index: usize) -> &'static ExtendedFloat<M> {
        self.small.get_unchecked(index)
    }

    #[inline(always)]
    pub unsafe fn get_small_int(&self, index: usize) -> M {
        *self.small_int.get_unchecked(index)
    }

    #[inline(always)]
    pub unsafe fn get_large(&self, index: usize) -> &'static ExtendedFloat<M> {
        self.large.get_unchecked(index)
    }
}

// CACHED POWERS

/// Cached powers as a trait for a floating-point type.
pub(super) trait CachedPowers<M: Mantissa> {
    /// Get powers from base.
    fn get_powers(base: u32) -> &'static Powers<M>;
}

impl CachedPowers<u64> for ExtendedFloat<u64> {
    #[inline(always)]
    fn get_powers(base: u32) -> &'static Powers<u64> {
        cached80::get_powers(base)
    }
}

impl CachedPowers<u128> for ExtendedFloat<u128> {
    #[inline(always)]
    fn get_powers(base: u32) -> &'static Powers<u128> {
        cached160::get_powers(base)
    }
}
