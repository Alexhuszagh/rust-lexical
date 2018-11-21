//! Macros for bit-wise shifts.

use util::*;
use super::float::ExtendedFloat;
use super::mantissa::Mantissa;

// SHIFT RIGHT

/// Shift extended-precision float right `shift` bytes (force overflow checks).
#[inline]
pub(super) fn shr<M: Mantissa, T: Integer>(fp: &mut ExtendedFloat<M>, shift: T)
{
    fp.frac >>= as_cast::<M, _>(shift);
    fp.exp += as_cast::<i32, _>(shift);
}

/// Shift extended-precision float left `shift` bytes (force overflow checks).
#[inline]
pub(super) fn shl<M: Mantissa, T: Integer>(fp: &mut ExtendedFloat<M>, shift: T)
{
    fp.frac <<= as_cast::<M, _>(shift);
    fp.exp -= as_cast::<i32, _>(shift);
}
