//! Rounding-scheme identifiers.

#![allow(non_upper_case_globals)]
#![cfg_attr(rustfmt, rustfmt::skip)]

// Allow dead code so we compile these enum variants even
// if we don't expose them without the rounding feature.
// Since the bitflags are replacing an enum, use enum
// case conventions (Pascal).

use bitflags::bitflags;

// ROUNDING KIND
// -------------

bitflags! {
    /// Rounding type for float-parsing.
    ///
    /// Defines the IEEE754 rounding scheme to be used during float parsing.
    /// In general, this should be set to `NearestTieEven`, the default
    /// recommended rounding scheme by IEEE754 for binary and decimal
    /// operations.
    ///
    /// # FFI
    ///
    /// For interfacing with FFI-code, this may be approximated by:
    /// ```text
    /// const uint32_t NEAREST_TIE_EVEN = 0;
    /// const uint32_t NEAREST_TIE_AWAY_ZERO = 1;
    /// const uint32_t TOWARD_POSITIVE_INFINITY = 2;
    /// const uint32_t TOWARD_NEGATIVE_INFINITY = 3;
    /// const uint32_t TOWARD_ZERO = 4;
    /// ```
    ///
    /// # Safety
    ///
    /// Assigning any value outside the range `[1-4]` to value of type
    /// RoundingKind may invoke undefined-behavior. Internally,
    /// we never store a value > 0xF, so it may be represented in 4 bits.
    #[repr(C)]
    pub struct RoundingKind: u32 {
        /// Round to the nearest, tie to even.
        const NearestTieEven = 0;
        /// Round to the nearest, tie away from zero.
        const NearestTieAwayZero = 1;
        /// Round toward positive infinity.
        const TowardPositiveInfinity = 2;
        /// Round toward negative infinity.
        const TowardNegativeInfinity = 3;
        /// Round toward zero.
        const TowardZero = 4;

        // Hide the internal implementation details, for how we implement
        // TowardPositiveInfinity, TowardNegativeInfinity, and TowardZero.

        /// Round to increase the magnitude of the float.
        /// For example, for a negative number, this rounds to negative infinity,
        /// for a positive number, to positive infinity.
        #[doc(hidden)]
        const Upward = 0xE;

        /// Round to decrease the magnitude of the float.
        /// This always rounds toward zero.
        #[doc(hidden)]
        const Downward = 0xF;
    }
}

impl RoundingKind {
    const_fn!(
    /// Determine if we are rounding to the nearest value, then tying away.
    #[inline]
    pub const fn is_nearest(self) -> bool {
        match self {
            RoundingKind::NearestTieEven => true,
            RoundingKind::NearestTieAwayZero => true,
            _ => false,
        }
    });

    const_fn!(
    /// Determine if we are rounding to the nearest value, then tying away.
    #[inline]
    pub const fn is_toward(self) -> bool {
        !self.is_nearest()
    });

    /// Convert rounding kind to u32.
    #[inline(always)]
    pub const fn as_u32(self) -> u32 {
        self.bits()
    }

    /// Convert rounding kind to u32.
    #[inline(always)]
    pub const unsafe fn from_u32(bits: u32) -> Self {
        Self::from_bits_unchecked(bits)
    }
}

impl Default for RoundingKind {
    #[inline(always)]
    fn default() -> Self {
        RoundingKind::NearestTieEven
    }
}
