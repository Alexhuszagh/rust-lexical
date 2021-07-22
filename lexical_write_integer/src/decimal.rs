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

// We use the identity op for indexing for spacing reasons.
#![allow(clippy::identity_op)]

// TODO(ahuszagh) Add more documentation...

use crate::lib::hint;

use super::table::DIGIT_TO_BASE10_SQUARED;
use lexical_util::algorithm::copy_to_dst;

// TODO(ahuszagh) Here...

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
pub fn fast_log2(x: u32) -> usize {
    u32::BITS as usize - 1 - (x | 1).leading_zeros() as usize
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

/// Convert a value from `[100, 1000)` into a table offset.
#[inline]
fn sequential_index(v0: u32, v1: u32) -> usize {
    2 * v0 as usize - 200 * v1 as usize
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
/// This is always true, since `u32::MAX < 10^10`.
#[inline]
unsafe fn write_10(value: u32, buffer: &mut [u8]) {
    debug_assert!(buffer.len() >= 10);
    debug_assert!(value >= 100000);

    let t0 = value / 100000000;
    let v_0 = value.wrapping_sub(t0.wrapping_mul(100000000));
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
#[inline]
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
#[inline]
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
#[inline]
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
        _ => {
            // SAFETY: safe if the buffer length is at least 10.
            // Since u8 has no trap representations, and a trivial drop,
            // this is also safe. The digit count cannot be larger than
            // 10, due to the limits of u32.
            let mut digits: [u8; 16] = [b'0'; 16];
            unsafe {
                write_10(value, &mut digits);
            }
            copy_to_dst(buffer, digits.get_unchecked(10 - count..));
        },
    }

    count
}
