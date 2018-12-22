//! Rounding-scheme identifiers.

#![allow(dead_code)]

/// Rounding type for float-parsing.
///
/// Defines the IEEE754 rounding scheme to be used during float parsing.
/// In general, this should be set to `NearestTieEven`, the default
/// recommended rounding scheme by IEEE754 for binary and decimal
/// operations.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RoundingKind {
    /// Round to the nearest, tie to even.
    NearestTieEven,
    /// Round to the nearest, tie away from zero.
    NearestTieAwayZero,
    /// Round toward positive infinity.
    TowardPositiveInfinity,
    /// Round toward negative infinity.
    TowardNegativeInfinity,
    /// Round toward zero.
    TowardZero,

    // Hide the internal implementation details, for how we implement
    // TowardPositiveInfinity, TowardNegativeInfinity, and TowardZero.

    /// Round to increase the magnitude of the float.
    /// For example, for a negative number, this rounds to negative infinity,
    /// for a positive number, to positive infinity.
    #[doc(hidden)]
    Upward,

    /// Round to decrease the magnitude of the float.
    /// This always rounds toward zero.
    #[doc(hidden)]
    Downward,
}

/// Determine if we are rounding to the nearest value, then tying away.
#[inline]
pub(crate) fn is_nearest(kind: RoundingKind) -> bool {
    kind == RoundingKind::NearestTieEven || kind == RoundingKind::NearestTieAwayZero
}

/// Determine if we are rounding to the nearest value, then tying away.
#[inline]
pub(crate) fn is_toward(kind: RoundingKind) -> bool {
    !is_nearest(kind)
}
