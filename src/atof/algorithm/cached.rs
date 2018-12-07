//! Cached powers trait for extended-precision floats.

use float::{ExtendedFloat, Mantissa};
use super::cached_float80;

// POWERS

/// Precalculated powers that uses two-separate arrays for memory-efficiency.
#[doc(hidden)]
pub(crate) struct ExtendedPowers<M: Mantissa> {
    // Pre-calculated mantissa for the powers.
    pub mant: &'static [M],
    // Pre-calculated binary exponents for the powers.
    pub exp: &'static [i32],
}

/// Allow indexing of values without bounds checking
impl<M: Mantissa> ExtendedPowers<M> {
    #[inline(always)]
    pub unsafe fn get_extended_float(&self, index: usize)
        -> ExtendedFloat<M>
    {
        let mant = *self.mant.get_unchecked(index);
        let exp = *self.exp.get_unchecked(index);
        ExtendedFloat { mant: mant, exp: exp }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.mant.len()
    }
}

// MODERATE PATH POWERS

/// Precalculated powers of base N for the moderate path.
#[doc(hidden)]
pub(crate) struct ModeratePathPowers<M: Mantissa> {
    // Pre-calculated small powers.
    pub small: ExtendedPowers<M>,
    // Pre-calculated large powers.
    pub large: ExtendedPowers<M>,
    /// Pre-calculated small powers as 64-bit integers
    pub small_int: &'static [M],
    // Step between large powers and number of small powers.
    pub step: i32,
    // Exponent bias for the large powers.
    pub bias: i32,
}

/// Allow indexing of values without bounds checking
impl<M: Mantissa> ModeratePathPowers<M> {
    #[inline(always)]
    pub unsafe fn get_small(&self, index: usize) -> ExtendedFloat<M> {
        self.small.get_extended_float(index)
    }

    #[inline(always)]
    pub unsafe fn get_large(&self, index: usize) -> ExtendedFloat<M> {
        self.large.get_extended_float(index)
    }

    #[inline(always)]
    pub unsafe fn get_small_int(&self, index: usize) -> M {
        *self.small_int.get_unchecked(index)
    }
}

// CACHED EXTENDED POWERS

/// Cached powers as a trait for a floating-point type.
pub(super) trait ModeratePathCache<M: Mantissa> {
    /// Get powers from base.
    fn get_powers(base: u32) -> &'static ModeratePathPowers<M>;
}

impl ModeratePathCache<u64> for ExtendedFloat<u64> {
    #[inline(always)]
    fn get_powers(base: u32) -> &'static ModeratePathPowers<u64> {
        cached_float80::get_powers(base)
    }
}

// BIGCOMP POWERS

// TODO(ahuszagh) Restore
///// Precalculated bigcomp powers of base N.
//#[doc(hidden)]
//pub(crate) struct BigcompPowers<M: Mantissa> {
//    // Pre-calculated mantissa for the power.
//    pub mant: &'static [M],
//    // Pre-calculated binary exponents for the power.
//    pub exp: &'static [i32],
//    /// Bias for the exponent power.
//    pub bias: i32,
//}

//impl<M: Mantissa> BigcompPowers<M> {
//    /// Get the precalculated scaling factor from the basen exponent.
//    /// Does not do any index checking, it is up to the caller to ensure
//    /// the index is valid (which should be true for any non-special float).
//    /// That is, as long as the exponent is generated from an actual float,
//    /// this should be fine.
//    #[inline(always)]
//    pub unsafe fn get_scaling_factor(&self, exp: i32) -> (&'static M, &'static i32) {
//        let idx: usize = as_cast(exp + self.bias);
//        (self.mant.get_unchecked(idx), self.exp.get_unchecked(idx))
//    }
//}
