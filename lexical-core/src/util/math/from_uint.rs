//! Create a vector-like type from integral values.

use crate::util::config::*;
use crate::util::traits::*;

use super::cast::*;
use super::shared_ops::*;

// FROM UINT
// ---------

/// Split u16 into limbs, in little-endian order.
#[inline]
fn split_u16(x: u16) -> [Limb; 1] {
    [as_limb(x)]
}

/// Split u32 into limbs, in little-endian order.
#[inline]
fn split_u32(x: u32) -> [Limb; 1] {
    [as_limb(x)]
}

/// Split u64 into limbs, in little-endian order.
#[inline]
#[cfg(limb_width_32)]
fn split_u64(x: u64) -> [Limb; 2] {
    [as_limb(x), as_limb(x >> 32)]
}

/// Split u64 into limbs, in little-endian order.
#[inline]
#[cfg(limb_width_64)]
fn split_u64(x: u64) -> [Limb; 1] {
    [as_limb(x)]
}

/// Split u128 into limbs, in little-endian order.
#[inline]
#[cfg(limb_width_32)]
fn split_u128(x: u128) -> [Limb; 4] {
    [as_limb(x), as_limb(x >> 32), as_limb(x >> 64), as_limb(x >> 96)]
}

/// Split u128 into limbs, in little-endian order.
#[inline]
#[cfg(limb_width_64)]
fn split_u128(x: u128) -> [Limb; 2] {
    [as_limb(x), as_limb(x >> 64)]
}

/// Impl FromUint
macro_rules! from_uint {
    ($self:ident, $vec:ty, $split:ident) => {{
        let mut v = <$vec>::default();
        let slc = $split($self);
        v.data_mut().extend_from_slice(&slc);
        v.normalize();
        v
    }};
}

/// Create vector-like type from integral value.
pub(crate) trait FromUint: UnsignedInteger {
    /// Create vector-like type from value.
    fn from_uint<VecType: SharedOps>(self) -> VecType;
}

impl FromUint for u16 {
    #[inline]
    fn from_uint<VecType: SharedOps>(self) -> VecType {
        from_uint!(self, VecType, split_u16)
    }
}

impl FromUint for u32 {
    #[inline]
    fn from_uint<VecType: SharedOps>(self) -> VecType {
        from_uint!(self, VecType, split_u32)
    }
}

impl FromUint for u64 {
    #[inline]
    fn from_uint<VecType: SharedOps>(self) -> VecType {
        from_uint!(self, VecType, split_u64)
    }
}

impl FromUint for u128 {
    #[inline]
    fn from_uint<VecType: SharedOps>(self) -> VecType {
        from_uint!(self, VecType, split_u128)
    }
}
