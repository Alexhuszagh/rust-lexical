//! Fast lexical integer-to-string conversion routines for decimal strings.
//!
//!  The following algorithms aim to minimize the number of conditional
//!  jumps required, by requiring at most 5 linear conditions before
//!  jumping to a condition-less set of instructions. This allows high
//!  performance formatting for integer sizes, and scales well for
//!  both sequential values (primarily low number of digits) and uniform
//!  values (primarily high numbers of digits), however, it also works
//!  well even with branch misprediction (tested using a linear congruent
//!  generator to choose between a sequential or uniform integer).
//!
//!  The performance is ~2-3x the performance of traditional integer
//!  formatters (see, dtolnay/itoa, or the generic algorithm) for 32-bits
//!  or less, highlighting the advantage of removing for loops with
//!  minimal branches. It also scales well for 64 or more bit integers.

// TODO(ahuszagh) Add more documentation...

// We use the identity op for indexing for spacing reasons.
#![allow(clippy::identity_op)]

use crate::lib::hint;

use super::table::DIGIT_TO_BASE10_SQUARED;
use lexical_util::algorithm::copy_to_dst;
use lexical_util::div128::u128_divrem_1e19;
use lexical_util::num::UnsignedInteger;

/// Fast integral log2.
///
/// This is fairly trivial to explain, since the log2 is related to the
/// number of bits in the value. Therefore, it has to be related to
/// `T::BITS - ctlz(x)`. For example, `log2(2) == 1`, and `log2(1) == 0`,
/// and `log2(3) == 1`. Therefore, we must take the log of an odd number,
/// and subtract one.
///
/// Described here:
///   https://lemire.me/blog/2021/05/28/computing-the-number-of-digits-of-an-integer-quickly/
#[inline]
pub fn fast_log2<T: UnsignedInteger>(x: T) -> usize {
    T::BITS - 1 - (x | T::ONE).leading_zeros() as usize
}

/// Calculate the fast, integral log10 of a value.
///
/// This is relatively easy to explain as well: we calculate the log2
/// of the value, then multiply by an integral constant for the log10(2).
///
/// Note that this value is frequently off by 1, so we need to round-up
/// accordingly. This magic number is valid at least up until `1<<18`,
/// which works for all values, since our max log2 is 127.
#[inline]
pub fn fast_log10<T: UnsignedInteger>(x: T) -> usize {
    let log2 = fast_log2(x);
    (log2 * 1233) >> 12
}

/// Fast algorithm to calculate the number of digits in an integer.
///
/// We only use this for 32-bit or smaller values: for larger numbers,
/// we first write digits until we get to 32-bits, then we call this.
///
/// The values are as follows:
///     2^32 for j = 1
///     ⌈log10(2^j)⌉ * 2^128 + 2^128 – 10^(⌈log10(2j)⌉) for j from 2 to 30
///     ⌈log10(2^j)⌉ for j = 31 and j = 32
///
/// Described here:
///     https://lemire.me/blog/2021/06/03/computing-the-number-of-digits-of-an-integer-even-faster/
#[inline]
pub fn fast_digit_count(x: u32) -> usize {
    const TABLE: [u64; 32] = [
        4294967296,
        8589934582,
        8589934582,
        8589934582,
        12884901788,
        12884901788,
        12884901788,
        17179868184,
        17179868184,
        17179868184,
        21474826480,
        21474826480,
        21474826480,
        21474826480,
        25769703776,
        25769703776,
        25769703776,
        30063771072,
        30063771072,
        30063771072,
        34349738368,
        34349738368,
        34349738368,
        34349738368,
        38554705664,
        38554705664,
        38554705664,
        41949672960,
        41949672960,
        41949672960,
        42949672960,
        42949672960,
    ];
    // SAFETY: always safe, since fast_log2 will always return a value
    // <= 32. This is because the range of values from `ctlz(x | 1)` is
    // `[0, 31]`, so `32 - 1 - ctlz(x | 1)` must be in the range `[0, 31]`.
    let shift = unsafe { *TABLE.get_unchecked(fast_log2(x)) };
    let count = (x as u64 + shift) >> 32;
    count as usize
}

/// Slightly slower algorithm to calculate the number of digits in an integer.
///
/// This uses no static storage, and uses a fast log10(2) estimation
/// to calculate the number of digits, from the log2 value.
///
/// Described here:
///     https://lemire.me/blog/2021/06/03/computing-the-number-of-digits-of-an-integer-even-faster/
#[inline]
pub fn fallback_digit_count<T: UnsignedInteger>(x: T) -> usize {
    const TABLE: [u128; 38] = [
        10,
        100,
        1000,
        10000,
        100000,
        1000000,
        10000000,
        100000000,
        1000000000,
        10000000000,
        100000000000,
        1000000000000,
        10000000000000,
        100000000000000,
        1000000000000000,
        10000000000000000,
        100000000000000000,
        1000000000000000000,
        10000000000000000000,
        100000000000000000000,
        1000000000000000000000,
        10000000000000000000000,
        100000000000000000000000,
        1000000000000000000000000,
        10000000000000000000000000,
        100000000000000000000000000,
        1000000000000000000000000000,
        10000000000000000000000000000,
        100000000000000000000000000000,
        1000000000000000000000000000000,
        10000000000000000000000000000000,
        100000000000000000000000000000000,
        1000000000000000000000000000000000,
        10000000000000000000000000000000000,
        100000000000000000000000000000000000,
        1000000000000000000000000000000000000,
        10000000000000000000000000000000000000,
        100000000000000000000000000000000000000,
    ];

    // This value is always within 1: calculate if we need to round-up
    // based on a pre-computed table. This is always safe, since we ensure
    // the index is in bounds right before.
    let log10 = fast_log10(x);
    let shift_up = log10 < TABLE.len() && x.as_u128() >= unsafe { *TABLE.get_unchecked(log10) };

    log10 + shift_up as usize + 1
}

/// Convert a value from `[100, 1000)` into a table offset.
#[inline]
fn sequential_index(v0: u32, v1: u32) -> usize {
    (2 * v0 - 200 * v1) as usize
}

/// Convert a value from `[10, 100)` into a table offset.
#[inline]
fn last_index(value: u32) -> usize {
    2 * value as usize
}

// WRITE
// -----

// Write N digits to a buffer.
// Note that these functions are inherently unsafe.

/// Write 1 digit to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 1 bytes,
/// and indexing the table will be safe as long as the `value < 10`.
#[inline]
unsafe fn write_1(value: u32, buffer: &mut [u8]) {
    debug_assert!(!buffer.is_empty());
    debug_assert!(value < 10);

    unsafe {
        *buffer.get_unchecked_mut(0) = value as u8 + b'0';
    }
}

/// Write 2 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 2 bytes,
/// and indexing the table will be safe as long as the `value < 10^2`.
#[inline]
unsafe fn write_2(value: u32, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 2);
    debug_assert!((10..100).contains(&value));

    let i_0 = last_index(value);
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
}

/// Write 3 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 3 bytes,
/// and indexing the table will be safe as long as the `value < 10^3`.
#[inline]
unsafe fn write_3(value: u32, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 3);
    debug_assert!((100..1000).contains(&value));

    let v_0 = value;
    let v_1 = v_0 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = last_index(v_1);
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
}

/// Write 4 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 4 bytes,
/// and indexing the table will be safe as long as the `value < 10^4`.
#[inline]
unsafe fn write_4(value: u32, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 4);
    debug_assert!((1000..10000).contains(&value));

    let v_0 = value;
    let v_1 = v_0 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = last_index(v_1);
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
}

/// Write 5 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 5 bytes,
/// and indexing the table will be safe as long as the `value < 10^5`.
#[inline]
unsafe fn write_5(value: u32, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 5);
    debug_assert!((10000..100000).contains(&value));

    let v_0 = value;
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = last_index(v_2);
    unsafe {
        *buffer.get_unchecked_mut(4) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 1);
    }
}

/// Write 10 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 10 bytes,
/// and indexing the table will be safe as long as the `value < 10^10`.
#[inline]
unsafe fn write_10(value: u64, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 10);
    debug_assert!((100000..10000000000).contains(&value));

    let t0 = (value / 100000000) as u32;
    let v_0 = (value as u32).wrapping_sub(t0.wrapping_mul(100000000));
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let v_3 = v_2 / 100;
    let v_4 = t0;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = sequential_index(v_2, v_3);
    let i_3 = last_index(v_3);
    let i_4 = last_index(v_4);

    unsafe {
        *buffer.get_unchecked_mut(9) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(8) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(7) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(6) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(5) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(4) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 0);
    }
}

/// Write 15 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 15 bytes,
/// and indexing the table will be safe as long as the `value < 10^15`.
#[inline]
unsafe fn write_15(value: u64, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 15);
    debug_assert!((10000000000..1000000000000000).contains(&value));

    let t_0 = (value / 100000000) as u32;
    let v_0 = (value as u32).wrapping_sub(t_0.wrapping_mul(100000000));
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let v_3 = v_2 / 100;
    let v_4 = t_0;
    let v_5 = v_4 / 100;
    let v_6 = v_5 / 100;
    let v_7 = v_6 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = sequential_index(v_2, v_3);
    let i_3 = last_index(v_3);
    let i_4 = sequential_index(v_4, v_5);
    let i_5 = sequential_index(v_5, v_6);
    let i_6 = sequential_index(v_6, v_7);
    let i_7 = last_index(v_7);
    unsafe {
        *buffer.get_unchecked_mut(14) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(13) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(12) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(11) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(10) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(9) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(8) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(7) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(6) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(5) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(4) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_6 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_6 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_7 + 1);
    }
}

/// Write 20 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 20 bytes,
/// and indexing the table will be safe as long as the `value < 10^20`.
///
/// # Note
///
/// Due to how slow 128-bit division is, this is only for u64. `write_25`
/// will handle all those cases, for 20-25 digits.
#[inline]
unsafe fn write_20(value: u64, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 20);
    debug_assert!(value >= 1000000000000000);

    let t_0 = (value / 100000000) as u32;
    let t_1 = (value / 10000000000000000) as u32;
    let v_0 = (value as u32).wrapping_sub(t_0.wrapping_mul(100000000));
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let v_3 = v_2 / 100;
    let v_4 = t_0.wrapping_sub(t_1.wrapping_mul(100000000));
    let v_5 = v_4 / 100;
    let v_6 = v_5 / 100;
    let v_7 = v_6 / 100;
    let v_8 = t_1;
    let v_9 = v_8 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = sequential_index(v_2, v_3);
    let i_3 = last_index(v_3);
    let i_4 = sequential_index(v_4, v_5);
    let i_5 = sequential_index(v_5, v_6);
    let i_6 = sequential_index(v_6, v_7);
    let i_7 = last_index(v_7);
    let i_8 = sequential_index(v_8, v_9);
    let i_9 = last_index(v_9);
    unsafe {
        *buffer.get_unchecked_mut(19) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(18) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(17) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(16) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(15) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(14) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(13) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(12) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(11) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(10) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(9) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(8) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(7) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_6 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(6) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_6 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(5) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_7 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(4) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_7 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_8 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_8 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_9 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_9 + 0);
    }
}

/// Write 19 digits to buffer (used internally for the u128 writers).
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 19 bytes,
/// and indexing the table will be safe as long as the `value < 10^19`.
#[inline]
unsafe fn write_19(value: u64, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 19);
    // The value might have lower bits with all zeros, so we should just
    // check it's below the maximum range.
    debug_assert!(value <= 10000000000000000000);

    let t_0 = (value / 100000000) as u32;
    let t_1 = (value / 10000000000000000) as u32;
    let v_0 = (value as u32).wrapping_sub(t_0.wrapping_mul(100000000));
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let v_3 = v_2 / 100;
    let v_4 = t_0.wrapping_sub(t_1.wrapping_mul(100000000));
    let v_5 = v_4 / 100;
    let v_6 = v_5 / 100;
    let v_7 = v_6 / 100;
    let v_8 = t_1;
    let v_9 = v_8 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = sequential_index(v_2, v_3);
    let i_3 = last_index(v_3);
    let i_4 = sequential_index(v_4, v_5);
    let i_5 = sequential_index(v_5, v_6);
    let i_6 = sequential_index(v_6, v_7);
    let i_7 = last_index(v_7);
    let i_8 = sequential_index(v_8, v_9);
    let i_9 = last_index(v_9);
    unsafe {
        *buffer.get_unchecked_mut(18) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(17) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(16) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(15) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(14) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(13) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(12) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(11) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(10) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(9) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(8) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(7) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(6) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_6 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(5) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_6 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(4) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_7 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_7 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_8 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_8 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_9 + 1);
    }
}

/// Write 25 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 25 bytes,
/// and indexing the table will be safe as long as the `value < 10^25`.
#[inline]
unsafe fn write_25(value: u128, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 25);
    debug_assert!((10000000000000000000..10000000000000000000000000).contains(&value));

    // Split value into high 6 and low 19.
    let (high, low) = u128_divrem_1e19(value);

    // Write low 19 to the end of the buffer.
    // SAFETY: safe since we have at least 25 elements, so we can index safely
    // from 6-onward and have at least 19 elements.
    unsafe {
        write_19(low, buffer.get_unchecked_mut(6..));
    }

    // Write high 6 to the front of the buffer.
    let value = high as u64;
    let v_0 = value as u32;
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = last_index(v_2);
    unsafe {
        *buffer.get_unchecked_mut(5) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(4) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 0);
    }
}

/// Write 30 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 30 bytes,
/// and indexing the table will be safe as long as the `value < 10^30`.
#[inline]
unsafe fn write_30(value: u128, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 30);
    debug_assert!((10000000000000000000000000..1000000000000000000000000000000).contains(&value));

    // Split value into high 11 and low 19.
    let (high, low) = u128_divrem_1e19(value);

    // Write low 19 to the end of the buffer.
    // SAFETY: safe since we have at least 30 elements, so we can index safely
    // from 11-onward and have at least 19 elements.
    unsafe {
        write_19(low, buffer.get_unchecked_mut(11..));
    }

    // Write high 11 to the front of the buffer.
    let value = high as u64;
    let t_0 = (value / 100000000) as u32;
    let v_0 = (value as u32).wrapping_sub(t_0.wrapping_mul(100000000));
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let v_3 = v_2 / 100;
    let v_4 = t_0;
    let v_5 = v_4 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = sequential_index(v_2, v_3);
    let i_3 = last_index(v_3);
    let i_4 = sequential_index(v_4, v_5);
    let i_5 = last_index(v_5);
    unsafe {
        *buffer.get_unchecked_mut(10) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(9) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(8) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(7) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(6) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(5) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(4) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 1);
    }
}

/// Write 35 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 35 bytes,
/// and indexing the table will be safe as long as the `value < 10^35`.
#[inline]
unsafe fn write_35(value: u128, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 35);
    debug_assert!(
        (1000000000000000000000000000000..100000000000000000000000000000000000).contains(&value)
    );

    // Split value into high 16 and low 19.
    let (high, low) = u128_divrem_1e19(value);

    // Write low 19 to the end of the buffer.
    // SAFETY: safe since we have at least 35 elements, so we can index safely
    // from 16-onward and have at least 19 elements.
    unsafe {
        write_19(low, buffer.get_unchecked_mut(16..));
    }

    // Write high 16 to the front of the buffer.
    let value = high as u64;
    let t_0 = (value / 100000000) as u32;
    let v_0 = (value as u32).wrapping_sub(t_0.wrapping_mul(100000000));
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let v_3 = v_2 / 100;
    let v_4 = t_0;
    let v_5 = v_4 / 100;
    let v_6 = v_5 / 100;
    let v_7 = v_6 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = sequential_index(v_2, v_3);
    let i_3 = last_index(v_3);
    let i_4 = sequential_index(v_4, v_5);
    let i_5 = sequential_index(v_5, v_6);
    let i_6 = sequential_index(v_6, v_7);
    let i_7 = last_index(v_7);

    unsafe {
        *buffer.get_unchecked_mut(15) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(14) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_0 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(13) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(12) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_1 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(11) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(10) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_2 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(9) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(8) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_3 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(7) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(6) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_4 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(5) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(4) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_5 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(3) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_6 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(2) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_6 + 0);
    }
    unsafe {
        *buffer.get_unchecked_mut(1) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_7 + 1);
    }
    unsafe {
        *buffer.get_unchecked_mut(0) = *DIGIT_TO_BASE10_SQUARED.get_unchecked(i_7 + 0);
    }
}

/// Write 39 digits to buffer.
///
/// # Safety
///
/// Writing to the buffer is safe as long as the buffer is at least 39 bytes,
/// and indexing the table will be safe as long as the `value < 10^39`.
#[inline]
unsafe fn write_39(value: u128, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 39);
    debug_assert!(value >= 100000000000000000000000000000000000);

    // Split value into high 20 and low 19.
    let (high, low) = u128_divrem_1e19(value);

    // Write low 19 to the end of the buffer.
    // SAFETY: safe since we have at least 39 elements, so we can index safely
    // from 20-onward and have at least 19 elements.
    unsafe {
        write_19(low, buffer.get_unchecked_mut(20..));
    }

    // Split the value into the high 1 and mid 19.
    let (high, mid) = u128_divrem_1e19(high);

    // Write low 19 to the middle of the buffer.
    // SAFETY: safe since we have at least 39 elements, so we can index safely
    // from 1-onward and have at least 19 elements.
    unsafe {
        write_19(mid, buffer.get_unchecked_mut(1..));
    }

    // Write the high 1 to the front of the buffer
    // SAFETY: safe since we have at least 39 elements, and high must be
    // in the range `[0, 9]`.
    unsafe {
        *buffer.get_unchecked_mut(0) = high as u8 + b'0';
    }
}

// FORMATTERS
// ----------

// Each flow-path should have no more than 5 comparisons, or
// else we're poorly optimizing our code.
// Use the number of leading zeros to minimize the number
// of jumps we have possible.

/// Write a `u8` to a buffer, and return the number of bytes written.
///
/// # Safety
///
/// Safe as long as the buffer can hold at least 3 elements.
pub unsafe fn u8toa(value: u8, buffer: &mut [u8]) -> usize {
    debug_assert!(buffer.len() >= 3);
    debug_assert!(fast_digit_count(u8::MAX as u32) <= 3);

    let value = value as u32;
    let count = fast_digit_count(value);
    match count {
        // SAFETY: safe if the buffer length is at least 1.
        1 => unsafe {
            write_1(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 2.
        2 => unsafe {
            write_2(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 3.
        3 => unsafe {
            write_3(value, buffer);
        },
        // SAFETY: Safe, since there cannot be more than 3 digits.
        // 255 is the max value of a `u8`, which we statically assert.
        _ => unsafe { hint::unreachable_unchecked() },
    }

    count
}

/// Write a `u16` to a buffer, and return the number of bytes written.
///
/// # Safety
///
/// Safe as long as the buffer can hold at least 5 elements.
pub unsafe fn u16toa(value: u16, buffer: &mut [u8]) -> usize {
    debug_assert!(buffer.len() >= 5);
    debug_assert!(fast_digit_count(u16::MAX as u32) <= 5);

    let value = value as u32;
    let count = fast_digit_count(value);
    match count {
        // SAFETY: safe if the buffer length is at least 1.
        1 => unsafe {
            write_1(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 2.
        2 => unsafe {
            write_2(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 3.
        3 => unsafe {
            write_3(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 4.
        4 => unsafe {
            write_4(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 5.
        5 => unsafe {
            write_5(value, buffer);
        },
        // SAFETY: Safe, since there cannot be more than 5 digits.
        // 65535 is the max value of a `u16`, which we statically assert.
        _ => unsafe { hint::unreachable_unchecked() },
    }

    count
}

/// Write a `u32` to a buffer, and return the number of bytes written.
///
/// # Safety
///
/// Safe as long as the buffer can hold at least 10 elements.
pub unsafe fn u32toa(value: u32, buffer: &mut [u8]) -> usize {
    debug_assert!(buffer.len() >= 10);
    debug_assert!(fast_digit_count(u32::MAX) <= 10);

    let count = fast_digit_count(value);
    match count {
        // SAFETY: safe if the buffer length is at least 1.
        1 => unsafe {
            write_1(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 2.
        2 => unsafe {
            write_2(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 3.
        3 => unsafe {
            write_3(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 4.
        4 => unsafe {
            write_4(value, buffer);
        },
        // SAFETY: safe if the buffer length is at least 5.
        5 => unsafe {
            write_5(value, buffer);
        },
        6..=10 => {
            // SAFETY: safe if the buffer length is at least 10.
            // The digit count cannot be larger than 10, due to the
            // limits of u32.
            let mut digits: [u8; 16] = [b'0'; 16];
            unsafe {
                write_10(value as u64, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(10 - count..10));
            }
        },
        // SAFETY: Safe, since there cannot be more than 10 digits.
        _ => unsafe { hint::unreachable_unchecked() },
    }

    count
}

/// Write a `u64` to a buffer, and return the number of bytes written.
///
/// # Safety
///
/// Safe as long as the buffer can hold at least 20 elements.
pub unsafe fn u64toa(value: u64, buffer: &mut [u8]) -> usize {
    debug_assert!(buffer.len() >= 20);
    debug_assert!(fallback_digit_count(u64::MAX) <= 20);

    let count = fallback_digit_count(value);
    match count {
        // SAFETY: safe if the buffer length is at least 1.
        1 => unsafe {
            write_1(value as u32, buffer);
        },
        // SAFETY: safe if the buffer length is at least 2.
        2 => unsafe {
            write_2(value as u32, buffer);
        },
        // SAFETY: safe if the buffer length is at least 3.
        3 => unsafe {
            write_3(value as u32, buffer);
        },
        // SAFETY: safe if the buffer length is at least 4.
        4 => unsafe {
            write_4(value as u32, buffer);
        },
        // SAFETY: safe if the buffer length is at least 5.
        5 => unsafe {
            write_5(value as u32, buffer);
        },
        6..=10 => {
            // SAFETY: safe if the buffer length is at least 10.
            let mut digits: [u8; 16] = [b'0'; 16];
            unsafe {
                write_10(value, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(10 - count..10));
            }
        },
        11..=15 => {
            // SAFETY: safe if the buffer length is at least 15.
            let mut digits: [u8; 16] = [b'0'; 16];
            unsafe {
                write_15(value, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(15 - count..15));
            }
        },
        16..=20 => {
            // SAFETY: safe if the buffer length is at least 20.
            let mut digits: [u8; 32] = [b'0'; 32];
            unsafe {
                write_20(value, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(20 - count..20));
            }
        },
        // SAFETY: Safe, since there cannot be more than 20 digits.
        _ => unsafe { hint::unreachable_unchecked() },
    }

    count
}

/// Write a `u128` to a buffer, and return the number of bytes written.
///
/// # Safety
///
/// Safe as long as the buffer can hold at least 39 elements.
pub unsafe fn u128toa(value: u128, buffer: &mut [u8]) -> usize {
    debug_assert!(buffer.len() >= 39);
    debug_assert!(fallback_digit_count(u128::MAX) <= 39);

    let count = fallback_digit_count(value);
    match count {
        // SAFETY: safe if the buffer length is at least 1.
        1 => unsafe {
            write_1(value as u32, buffer);
        },
        // SAFETY: safe if the buffer length is at least 2.
        2 => unsafe {
            write_2(value as u32, buffer);
        },
        // SAFETY: safe if the buffer length is at least 3.
        3 => unsafe {
            write_3(value as u32, buffer);
        },
        // SAFETY: safe if the buffer length is at least 4.
        4 => unsafe {
            write_4(value as u32, buffer);
        },
        // SAFETY: safe if the buffer length is at least 5.
        5 => unsafe {
            write_5(value as u32, buffer);
        },
        6..=10 => {
            // SAFETY: safe if the buffer length is at least 10.
            let mut digits: [u8; 16] = [b'0'; 16];
            unsafe {
                write_10(value as u64, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(10 - count..10));
            }
        },
        11..=15 => {
            // SAFETY: safe if the buffer length is at least 15.
            let mut digits: [u8; 16] = [b'0'; 16];
            unsafe {
                write_15(value as u64, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(15 - count..15));
            }
        },
        16..=19 => {
            // SAFETY: safe if the buffer length is at least 20.
            let mut digits: [u8; 32] = [b'0'; 32];
            unsafe {
                write_20(value as u64, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(20 - count..20));
            }
        },
        20..=25 => {
            // SAFETY: safe if the buffer length is at least 25.
            let mut digits: [u8; 32] = [b'0'; 32];
            unsafe {
                write_25(value, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(25 - count..25));
            }
        },
        26..=30 => {
            // SAFETY: safe if the buffer length is at least 30.
            let mut digits: [u8; 32] = [b'0'; 32];
            unsafe {
                write_30(value, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(30 - count..30));
            }
        },
        31..=35 => {
            // SAFETY: safe if the buffer length is at least 35.
            let mut digits: [u8; 48] = [b'0'; 48];
            unsafe {
                write_35(value, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(35 - count..35));
            }
        },
        36..=39 => {
            // SAFETY: safe if the buffer length is at least 39.
            let mut digits: [u8; 48] = [b'0'; 48];
            unsafe {
                write_39(value, &mut digits);
                copy_to_dst(buffer, digits.get_unchecked(39 - count..39));
            }
        },
        // SAFETY: Safe, since there cannot be more than 39 digits.
        _ => unsafe { hint::unreachable_unchecked() },
    }

    count
}

/// Write a `usize` to a buffer, and return the number of bytes written.
///
/// # Safety
///
/// Safe as long as the buffer can hold at least `FORMATTED_SIZE_DECIMAL`
/// elements.
pub unsafe fn usizetoa(value: usize, buffer: &mut [u8]) -> usize {
    if cfg!(target_pointer_width = "16") {
        u16toa(value as u16, buffer)
    } else if cfg!(target_pointer_width = "32") {
        u32toa(value as u32, buffer)
    } else {
        u64toa(value as u64, buffer)
    }
}

// Export integer to string.
pub(super) trait Decimal {
    unsafe fn decimal(self, buffer: &mut [u8]) -> usize;
}

macro_rules! decimal_impl {
    ($($t:tt $cb:ident ; )*) => ($(
        impl Decimal for $t {
            #[inline(always)]
            unsafe fn decimal(self, buffer: &mut [u8]) -> usize {
                // SAFETY: safe as long as buffer is large enough to hold the max value.
                unsafe { $cb(self, buffer) }
            }
        }
    )*)
}

decimal_impl! {
    u8 u8toa ;
    u16 u16toa ;
    u32 u32toa ;
    u64 u64toa ;
    u128 u128toa ;
    usize usizetoa ;
}
