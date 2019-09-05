//! Big integer type definition.

use arrayvec;
use float::*;
use util::*;
use super::math::*;

// DATA TYPE

cfg_if! {
if #[cfg(feature = "radix")] {
    use lib::Vec;
    type IntStorageType = Vec<Limb>;
} else {
    // Maximum denominator is 767 mantissa digits + 324 exponent,
    // or 1091 digits, or approximately 3600 bits (round up to 4k).
    #[cfg(limb_width_32)]
    type IntStorageType = arrayvec::ArrayVec<[Limb; 128]>;

    #[cfg(limb_width_64)]
    type IntStorageType = arrayvec::ArrayVec<[Limb; 64]>;
}}  // cfg_if

// BIGINT

/// Storage for a big integer type.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Bigint {
    /// Internal storage for the Bigint, in little-endian order.
    pub(crate) data: IntStorageType,
}

impl Default for Bigint {
    fn default() -> Self {
        // We want to avoid lower-order
        let mut bigint = Bigint { data: IntStorageType::default() };
        bigint.data.reserve(20);
        bigint
    }
}

impl SharedOps for Bigint {
    type StorageType = IntStorageType;

    perftools_inline_always!{
    fn data<'a>(&'a self) -> &'a Self::StorageType {
        &self.data
    }}

    perftools_inline_always!{
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType {
        &mut self.data
    }}
}

impl SmallOps for Bigint {
}

impl LargeOps for Bigint {
}

// BIGFLOAT

// Adjust the storage capacity for the underlying array.
cfg_if! {
if #[cfg(limb_width_64)] {
    type FloatStorageType = arrayvec::ArrayVec<[Limb; 20]>;
} else {
    type FloatStorageType = arrayvec::ArrayVec<[Limb; 36]>;
}}   // cfg_if

/// Storage for a big floating-point type.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub struct Bigfloat {
    /// Internal storage for the Bigint, in little-endian order.
    ///
    /// Enough storage for up to 10^345, which is 2^1146, or more than
    /// the max for f64.
    pub(crate) data: FloatStorageType,
    /// It also makes sense to store an exponent, since this simplifies
    /// normalizing and powers of 2.
    pub(crate) exp: i32,
}

impl Default for Bigfloat {
    perftools_inline!{
    fn default() -> Self {
        // We want to avoid lower-order
        let mut bigfloat = Bigfloat { data: FloatStorageType::default(), exp: 0 };
        bigfloat.data.reserve(10);
        bigfloat
    }}
}

impl SharedOps for Bigfloat {
    type StorageType = FloatStorageType;

    perftools_inline_always!{
    fn data<'a>(&'a self) -> &'a Self::StorageType {
        &self.data
    }}

    perftools_inline_always!{
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType {
        &mut self.data
    }}
}

impl SmallOps for Bigfloat {
    perftools_inline!{
    fn imul_pow2(&mut self, n: u32) {
        // Increment exponent to simulate actual multiplication.
        self.exp += n.as_i32();
    }}
}

impl LargeOps for Bigfloat {
}

// TO BIGFLOAT

/// Simple overloads to allow conversions of extended floats to big integers.
pub trait ToBigfloat<M: Mantissa> {
    fn to_bigfloat(&self) -> Bigfloat;
}

impl ToBigfloat<u32> for ExtendedFloat<u32> {
    perftools_inline!{
    fn to_bigfloat(&self) -> Bigfloat {
        let mut bigfloat = Bigfloat::from_u32(self.mant);
        bigfloat.exp = self.exp;
        bigfloat
    }}
}

impl ToBigfloat<u64> for ExtendedFloat<u64> {
    perftools_inline!{
    fn to_bigfloat(&self) -> Bigfloat {
        let mut bigfloat = Bigfloat::from_u64(self.mant);
        bigfloat.exp = self.exp;
        bigfloat
    }}
}

#[cfg(has_i128)]
impl ToBigfloat<u128> for ExtendedFloat<u128> {
    perftools_inline!{
    fn to_bigfloat(&self) -> Bigfloat {
        let mut bigfloat = Bigfloat::from_u128(self.mant);
        bigfloat.exp = self.exp;
        bigfloat
    }}
}
