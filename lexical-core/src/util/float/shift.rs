//! Macros for bit-wise shifts.

use crate::util::traits::*;

use super::extended_float::*;

// SHIFT RIGHT

/// Shift extended-precision float right `shift` bytes.
#[inline]
pub(super) fn shr<M: Mantissa>(fp: &mut ExtendedFloat<M>, shift: i32) {
    debug_assert!(shift < M::FULL, "shr() overflow in shift right.");

    fp.mant >>= shift;
    fp.exp += shift;
}

/// Shift extended-precision float right `shift` bytes.
///
/// Accepts when the shift is the same as the type size, and
/// sets the value to 0.
#[inline]
pub(super) fn overflowing_shr<M: Mantissa>(fp: &mut ExtendedFloat<M>, shift: i32) {
    debug_assert!(shift <= M::FULL, "overflowing_shr() overflow in shift right.");

    fp.mant = match shift == M::FULL {
        true => M::ZERO,
        false => fp.mant >> shift,
    };
    fp.exp += shift;
}

/// Shift extended-precision float left `shift` bytes.
#[inline]
pub(super) fn shl<M: Mantissa>(fp: &mut ExtendedFloat<M>, shift: i32) {
    debug_assert!(shift < M::FULL, "shl() overflow in shift left.");

    fp.mant <<= shift;
    fp.exp -= shift;
}
