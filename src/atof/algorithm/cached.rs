//! Cached powers trait for extended-precision floats.

use float::{ExtendedFloat, Mantissa};
use util::*;
use super::{cached_float80, cached_float160};

// EXTENDED POWERS

/// Precalculated powers of base N.
#[doc(hidden)]
pub(crate) struct ExtendedPowers<M: Mantissa> {
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
impl<M: Mantissa> ExtendedPowers<M> {
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

// CACHED EXTENDED POWERS

/// Cached powers as a trait for a floating-point type.
pub(super) trait CachedExtendedPowers<M: Mantissa> {
    /// Get powers from base.
    fn get_powers(base: u32) -> &'static ExtendedPowers<M>;
}

impl CachedExtendedPowers<u64> for ExtendedFloat<u64> {
    #[inline(always)]
    fn get_powers(base: u32) -> &'static ExtendedPowers<u64> {
        cached_float80::get_powers(base)
    }
}

impl CachedExtendedPowers<u128> for ExtendedFloat<u128> {
    #[inline(always)]
    fn get_powers(base: u32) -> &'static ExtendedPowers<u128> {
        cached_float160::get_powers(base)
    }
}

// BIGCOMP POWERS

/// Precalculated bigcomp powers of base N.
#[doc(hidden)]
// TODO(ahuszagh) Need to actually use this...
// TODO(ahuszagh) Need a precalculated way to determine the decimal exponent.
pub(crate) struct BigcompPowers<M: UnsignedInteger> {
    // Pre-calculated mantissa for the power.
    pub mant: &'static [M],
    // Pre-calculated binary exponents for the power.
    pub exp: &'static [i32],
    /// Bias for the exponent power.
    pub bias: i32,
}

impl<M: UnsignedInteger> BigcompPowers<M> {
    /// Get the precalculated scaling factor from the basen exponent.
    /// Does not do any index checking, it is up to the caller to ensure
    /// the index is valid (which should be true for any non-special float).
    /// That is, as long as the exponent is generated from an actual float,
    /// this should be fine.
    #[inline(always)]
    pub unsafe fn get_scaling_factor(&self, exp: i32) -> (&'static M, &'static i32) {
        let idx: usize = as_cast(exp + self.bias);
        (self.mant.get_unchecked(idx), self.exp.get_unchecked(idx))
    }
}

// Why not use use 128.... It will work... And will simplify the implementation a lot...

// TODO(ahuszagh) Need to cache bigcomp, like as done in USE_BF96...
// Bias seems to be (e + 342)
// BF96 pten seems to be the following:
//      96-bit precision floats, with:
//          [b0,b1, b2] in big endian format
//          e, binary exponent
//      Equal to 7.9228162513 * 10^x
//          7.9228 was chosen because it gives the largest floating-point number.
// I really just need to number I can represent the best...
