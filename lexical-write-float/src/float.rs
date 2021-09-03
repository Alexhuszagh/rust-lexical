//! Extended helper trait for generic float types.
//!
//! This adds cache types and other helpers for extended-precision work.

#![doc(hidden)]

#[cfg(not(feature = "compact"))]
use crate::algorithm::DragonboxFloat;
#[cfg(feature = "compact")]
use crate::compact::GrisuFloat;
#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
use lexical_util::extended_float::ExtendedFloat;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;

/// Alias with ~80 bits of precision, 64 for the mantissa and 16 for exponent.
/// This exponent is biased, and if the exponent is negative, it represents
/// a value with a bias of `i32::MIN + F::EXPONENT_BIAS`.
pub type ExtendedFloat80 = ExtendedFloat<u64>;

/// Helper trait to add more float characteristics for parsing floats.
#[cfg(feature = "compact")]
pub trait RawFloat: GrisuFloat {}

#[cfg(not(feature = "compact"))]
pub trait RawFloat: DragonboxFloat {}

impl RawFloat for f32 {
}
impl RawFloat for f64 {
}
#[cfg(feature = "f16")]
impl RawFloat for f16 {
}
#[cfg(feature = "f16")]
impl RawFloat for bf16 {
}
