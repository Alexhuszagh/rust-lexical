//! Rounding-scheme identifiers.

#![allow(dead_code)]

/// Rounding type for float rounding.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RoundingKind {
    /// Round to the nearest, tie to even.
    NearestTieEven,
    /// Round to the nearest, tie away from zero.
    NearestTieAwayZero,
    /// Round up (toward infinity, since ExtendedFloat is always positive).
    TowardInfinity,
    /// Round down (toward zero, since ExtendedFloat is always positive).
    TowardZero,
}
