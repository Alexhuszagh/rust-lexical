//! Macros for bit-wise shifts.

use util::*;
use super::float_type::FloatType;

// SHIFT RIGHT

/// Shift extended-precision float right `shift` bytes.
#[inline]
pub(super) fn shr<T: Integer>(fp: &mut FloatType, shift: T)
{
    fp.frac = fp.frac.wrapping_shr(shift.cast());
    let shift: i32 = shift.cast();
    fp.exp += shift;
}

/// Shift extended-precision float left `shift` bytes.
#[inline]
pub(super) fn shl<T: Integer>(fp: &mut FloatType, shift: T)
{
    fp.frac = fp.frac.wrapping_shl(shift.cast());
    let shift: i32 = shift.cast();
    fp.exp -= shift;
}
