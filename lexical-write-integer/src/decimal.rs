//! Radix-generic, lexical integer-to-string conversion routines.
//!
//! An optimization for decimal is pre-computing the number of digits written
//! prior to actually writing digits, avoiding the use of temporary buffers.
//! This scales well with integer size, short of `u128`, due to the slower
//! division algorithms required.
//!
//! See [`Algorithm.md`] for a more detailed description of the algorithm
//! choice here.
//!
//! [`Algorithm.md`]: https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Algorithm.md

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use lexical_util::num::UnsignedInteger;

use crate::digit_count::fast_log2;
use crate::jeaiii;

/// Calculate the fast, integral log10 of a value.
///
/// This is relatively easy to explain as well: we calculate the log2
/// of the value, then multiply by an integral constant for the log10(2).
///
/// Note that this value is frequently off by 1, so we need to round-up
/// accordingly. This magic number is valid at least up until `1<<18`,
/// which works for all values, since our max log2 is 127.
#[inline(always)]
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
///
/// - `2^32 for j = 1`
/// - `⌈log10(2^j)⌉ * 2^128 + 2^128 – 10^(⌈log10(2j)⌉) for j from 2 to 30`
/// - `⌈log10(2^j)⌉ for j = 31 and j = 32`
///
/// This algorithm is described in detail in "Computing the number of digits
/// of an integer even faster", available
/// [here](https://lemire.me/blog/2021/06/03/computing-the-number-of-digits-of-an-integer-even-faster/).
#[inline(always)]
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
    // This always safe, since `fast_log2` will always return a value
    // <= 32. This is because the range of values from `ctlz(x | 1)` is
    // `[0, 31]`, so `32 - 1 - ctlz(x | 1)` must be in the range `[0, 31]`.
    let shift = TABLE[fast_log2(x)];
    let count = (x as u64 + shift) >> 32;
    count as usize
}

/// Slightly slower algorithm to calculate the number of digits in an integer.
///
/// This uses no static storage, and uses a fast log10(2) estimation
/// to calculate the number of digits, from the log2 value.
///
/// This algorithm is described in detail in "Computing the number of digits
/// of an integer even faster", available
/// [here](https://lemire.me/blog/2021/06/03/computing-the-number-of-digits-of-an-integer-even-faster/).
#[inline(always)]
pub fn fallback_digit_count<T: UnsignedInteger>(x: T, table: &[T]) -> usize {
    // This value is always within 1: calculate if we need to round-up
    // based on a pre-computed table.
    let log10 = fast_log10(x);
    let shift_up = table.get(log10).map_or(false, |&y| x >= y);

    log10 + shift_up as usize + 1
}

/// Quickly calculate the number of decimal digits in a type.
///
/// # Safety
///
/// Safe as long as `digit_count` returns at least the number of
/// digits that would be written by the integer. If the value is
/// too small, then the buffer might underflow, causing out-of-bounds
/// read/writes.
pub unsafe trait DecimalCount: UnsignedInteger {
    /// Get the number of digits in a value.
    fn decimal_count(self) -> usize;
}

// SAFETY: Safe since `fast_digit_count` is always correct for `<= u32::MAX`.
unsafe impl DecimalCount for u8 {
    #[inline(always)]
    fn decimal_count(self) -> usize {
        fast_digit_count(self as u32)
    }
}

// SAFETY: Safe since `fast_digit_count` is always correct for `<= u32::MAX`.
unsafe impl DecimalCount for u16 {
    #[inline(always)]
    fn decimal_count(self) -> usize {
        fast_digit_count(self as u32)
    }
}

// SAFETY: Safe since `fast_digit_count` is always correct for `<= u32::MAX`.
unsafe impl DecimalCount for u32 {
    #[inline(always)]
    fn decimal_count(self) -> usize {
        fast_digit_count(self)
    }
}

// SAFETY: Safe since `fallback_digit_count` is valid for the current table,
// as described in <https://lemire.me/blog/2021/06/03/computing-the-number-of-digits-of-an-integer-even-faster/>
unsafe impl DecimalCount for u64 {
    #[inline(always)]
    fn decimal_count(self) -> usize {
        const TABLE: [u64; 19] = [
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
        ];
        fallback_digit_count(self, &TABLE)
    }
}

// SAFETY: Safe since `fallback_digit_count` is valid for the current table,
// as described in <https://lemire.me/blog/2021/06/03/computing-the-number-of-digits-of-an-integer-even-faster/>
unsafe impl DecimalCount for u128 {
    #[inline(always)]
    fn decimal_count(self) -> usize {
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
        fallback_digit_count(self, &TABLE)
    }
}

// SAFETY: Safe since it uses the default implementation for the type size.
unsafe impl DecimalCount for usize {
    #[inline(always)]
    fn decimal_count(self) -> usize {
        match Self::BITS {
            8 | 16 | 32 => (self as u32).decimal_count(),
            64 => (self as u64).decimal_count(),
            128 => (self as u128).decimal_count(),
            _ => unimplemented!(),
        }
    }
}

/// Write integer to decimal string.
pub trait Decimal: DecimalCount {
    fn decimal(self, buffer: &mut [u8]) -> usize;

    /// Specialized overload is the type is sized.
    ///
    /// # Panics
    ///
    /// If the data original provided was unsigned and therefore
    /// has more digits than the signed variant. This only affects
    /// `i64` (see #191).
    #[inline(always)]
    fn decimal_signed(self, buffer: &mut [u8]) -> usize {
        self.decimal(buffer)
    }
}

// Implement decimal for type.
macro_rules! decimal_impl {
    ($($t:ty; $f:ident)*) => ($(
        impl Decimal for $t {
            #[inline(always)]
            fn decimal(self, buffer: &mut [u8]) -> usize {
                jeaiii::$f(self, buffer)
            }
        }
    )*);
}

decimal_impl! {
    u8; from_u8
    u16; from_u16
    u32; from_u32
    u128; from_u128
}

impl Decimal for u64 {
    #[inline(always)]
    fn decimal(self, buffer: &mut [u8]) -> usize {
        jeaiii::from_u64(self, buffer)
    }

    #[inline(always)]
    fn decimal_signed(self, buffer: &mut [u8]) -> usize {
        jeaiii::from_i64(self, buffer)
    }
}

impl Decimal for usize {
    #[inline(always)]
    fn decimal(self, buffer: &mut [u8]) -> usize {
        match usize::BITS {
            8 => (self as u8).decimal(buffer),
            16 => (self as u16).decimal(buffer),
            32 => (self as u32).decimal(buffer),
            64 => (self as u64).decimal(buffer),
            128 => (self as u128).decimal(buffer),
            _ => unimplemented!(),
        }
    }

    #[inline(always)]
    fn decimal_signed(self, buffer: &mut [u8]) -> usize {
        match usize::BITS {
            8 => (self as u8).decimal_signed(buffer),
            16 => (self as u16).decimal_signed(buffer),
            32 => (self as u32).decimal_signed(buffer),
            64 => (self as u64).decimal_signed(buffer),
            128 => (self as u128).decimal_signed(buffer),
            _ => unimplemented!(),
        }
    }
}
