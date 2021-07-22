//! Radix-generic, lexical integer-to-string conversion routines.
//!
//! These routines are decently optimized: they unroll 4 loops at a time,
//! using pre-computed base^2 tables. This is not nearly as fast as the
//! decimal routines, however, they're decently optimized.

#![cfg(feature = "power_of_two")]

use crate::table::{digit_to_char, get_table};
use lexical_util::algorithm::copy_to_dst;
use lexical_util::assert::debug_assert_radix;
use lexical_util::constants::FormattedSize;
use lexical_util::div128::{u128_divisor, u128_divrem};
use lexical_util::num::{as_cast, UnsignedInteger};

// TODO(ahuszagh) Add more documentation...

/// Generic itoa algorithm.
///
/// This algorithm first writers 4, then 2 digits at a time, finally
/// the last 1 or 2 digits, using power reduction to speed up the
/// algorithm a lot.
///
/// # Safety
///
/// This is safe as long as the buffer is large enough to hold `T::MAX`
/// digits in radix `N`.
unsafe fn generic_algorithm<T: UnsignedInteger>(
    mut value: T,
    radix: u32,
    table: &[u8],
    buffer: &mut [u8],
    mut index: usize,
) -> usize {
    debug_assert_radix(radix);

    // Pre-compute our powers of radix.
    let radix = as_cast(radix);
    let radix2 = radix * radix;
    let radix4 = radix2 * radix2;

    // SAFETY: All of these are safe for the buffer writes as long as
    // the buffer is large enough to hold `T::MAX` digits in radix `N`.

    // Decode 4 digits at a time.
    while value >= radix4 {
        let r = value % radix4;
        value /= radix4;
        let r1 = (T::TWO * (r / radix2)).as_usize();
        let r2 = (T::TWO * (r % radix2)).as_usize();

        // SAFETY: This is always safe, since the table is 2*radix^2, and
        // r1 and r2 must be in the range [0, 2*radix^2-1), since the maximum
        // value of r is `radix4-1`, which must have a div and r
        // in the range [0, radix^2-1).
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = *table.get_unchecked(r2 + 1);
        }
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = *table.get_unchecked(r2);
        }
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = *table.get_unchecked(r1 + 1);
        }
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = *table.get_unchecked(r1);
        }
    }

    // Decode 2 digits at a time.
    while value >= radix2 {
        let r = (T::TWO * (value % radix2)).as_usize();
        value /= radix2;

        // SAFETY: this is always safe, since the table is 2*radix^2, and
        // r must be in the range [0, 2*radix^2-1).
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = *table.get_unchecked(r + 1);
        }
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = *table.get_unchecked(r);
        }
    }

    // Decode last 2 digits.
    if value < radix {
        // SAFETY: this is always safe, since value < radix, so it must be < 36.
        // Digit must be < 36.
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = digit_to_char(value.as_usize());
        }
    } else {
        let r = (T::TWO * value).as_usize();
        // SAFETY: this is always safe, since the table is 2*radix^2, and
        // the value must <= radix^2, so rem must be in the range
        // [0, 2*radix^2-1).
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = *table.get_unchecked(r + 1);
        }
        index -= 1;
        unsafe {
            *buffer.get_unchecked_mut(index) = *table.get_unchecked(r);
        }
    }

    index
}

/// Optimized implementation for radix-N numbers.
///
/// # Safety
///
/// Safe as long as the buffer is large enough to hold as many digits
/// that can be in the largest value of `T`, in radix `N`.
#[inline]
pub unsafe fn generic<T>(value: T, radix: u32, table: &[u8], buffer: &mut [u8]) -> usize
where
    T: UnsignedInteger,
{
    // This is so that radix^4 does not overflow, since 36^4 overflows a u16.
    debug_assert!(T::BITS >= 32, "Must have at least 32 bits in the input.");
    debug_assert_radix(radix);

    // SAFETY: Both forms of unchecked indexing cannot overflow.
    // The table always has 2*radix^2 elements, so it must be a legal index.
    // The buffer is ensured to have at least `FORMATTED_SIZE` or
    // `FORMATTED_SIZE_DECIMAL` characters, which is the maximum number of
    // digits an integer of that size may write.
    unsafe { generic_algorithm(value, radix, table, buffer, buffer.len()) }
}

/// Optimized implementation for radix-N 128-bit numbers.
///
/// # Safety
///
/// Safe as long as the buffer is large enough to hold as many digits
/// that can be in the largest value of `T`, in radix `N`.
#[inline]
pub unsafe fn generic_u128(value: u128, radix: u32, table: &[u8], buffer: &mut [u8]) -> usize {
    debug_assert_radix(radix);

    // SAFETY: Both forms of unchecked indexing cannot overflow.
    // The table always has 2*radix^2 elements, so it must be a legal index.
    // The buffer is ensured to have at least `FORMATTED_SIZE` or
    // `FORMATTED_SIZE_DECIMAL` characters, which is the maximum number of
    // digits an integer of that size may write.

    // Use power-reduction to minimize the number of operations.
    // Idea taken from "3 Optimization Tips for C++".
    // Need to keep the steps, cause the lower values may
    // have internal 0s.
    let (divisor, step, d_ctlz) = u128_divisor(radix);

    // Decode 4-digits at a time.
    // To deal with internal 0 values or values with internal 0 digits set,
    // we store the starting index, and if not all digits are written,
    // we just skip down `digits` digits for the next value.
    let (value, low) = u128_divrem(value, divisor, d_ctlz);
    let mut index = buffer.len();
    unsafe {
        generic_algorithm(low, radix, table, buffer, index);
    }
    index -= step;
    if value != 0 {
        let (value, mid) = u128_divrem(value, divisor, d_ctlz);
        unsafe {
            generic_algorithm(mid, radix, table, buffer, index);
        }
        index -= step;

        if value != 0 {
            index = unsafe { generic_algorithm(value as u64, radix, table, buffer, index) };
        }
    }
    index
}

// Export integer to string.
pub(super) trait Generic {
    /// # SAFETY
    ///
    /// Safe as long as buffer is at least `FORMATTED_SIZE` elements long,
    /// (or `FORMATTED_SIZE_DECIMAL` for decimal), and the radix is valid.
    unsafe fn generic(self, radix: u32, buffer: &mut [u8]) -> usize;
}

// Implement generic for type.
macro_rules! generic_impl {
    ($($t:ty)*) => ($(
        impl Generic for $t {
            #[inline(always)]
            unsafe fn generic(self, radix: u32, buffer: &mut [u8]) -> usize {
                let mut digits = [b'0'; <$t>::FORMATTED_SIZE];
                unsafe {
                    let table = get_table(radix);
                    let index = generic(self, radix, table, &mut digits);
                    copy_to_dst(buffer, &mut digits.get_unchecked_mut(index..))
                }
            }
        }
    )*);
}

generic_impl! { u8 u16 u32 u64 usize }

impl Generic for u128 {
    #[inline(always)]
    unsafe fn generic(self, radix: u32, buffer: &mut [u8]) -> usize {
        let mut digits = [b'0'; u128::FORMATTED_SIZE];
        // SAFETY: safe as long as buffer is large enough to hold the max value.
        unsafe {
            let table = get_table(radix);
            let index = generic_u128(self, radix, table, &mut digits);
            copy_to_dst(buffer, &mut digits.get_unchecked_mut(index..))
        }
    }
}
