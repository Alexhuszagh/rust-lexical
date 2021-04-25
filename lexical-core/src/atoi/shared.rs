//! Shared definitions for string-to-integer conversions.

#[cfg(feature = "correct")]
use crate::util::*;

// SHARED
// ------

// Parse the sign bit and filter empty inputs from the atoi data.
macro_rules! parse_sign {
    ($bytes:ident, $is_signed:expr, $code:ident) => ({
        // Filter out empty inputs.
        if $bytes.is_empty() {
            return Err((ErrorCode::$code, $bytes.as_ptr()));
        }

        let (sign, digits) = match $bytes[0] {
            b'+'               => (Sign::Positive, &$bytes[1..]),
            b'-' if $is_signed => (Sign::Negative, &$bytes[1..]),
            _                  => (Sign::Positive, $bytes),
        };

        // Filter out empty inputs.
        if digits.is_empty() {
            return Err((ErrorCode::$code, digits.as_ptr()));
        }

        (sign, digits)
    });
}

// Get pointer to 1-past-end of slice.
// Performance note: Use slice, as `iter.as_ptr()` turns out to
// quite slow performance wise, likely since it needs to calculate
// the end ptr, while for a slice this is effectively a no-op.
#[inline(always)]
pub(super) fn last_ptr<T>(slc: &[T]) -> *const T {
    slc[slc.len()..].as_ptr()
}

// Add digit to mantissa.
#[inline(always)]
#[cfg(feature = "correct")]
pub(super) fn add_digit<T>(value: T, digit: u32, radix: u32)
    -> Option<T>
    where T: UnsignedInteger
{
    return value
        .checked_mul(as_cast(radix))?
        .checked_add(as_cast(digit))
}
