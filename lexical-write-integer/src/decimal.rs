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

use crate::algorithm::{algorithm, algorithm_u128};
use crate::table::DIGIT_TO_BASE10_SQUARED;
use lexical_util::format::{RADIX, RADIX_SHIFT, STANDARD};
use lexical_util::num::UnsignedInteger;

/// Fast integral log2.
///
/// This is fairly trivial to explain, since the log2 is related to the
/// number of bits in the value. Therefore, it has to be related to
/// `T::BITS - ctlz(x)`. For example, `log2(2) == 1`, and `log2(1) == 0`,
/// and `log2(3) == 1`. Therefore, we must take the log of an odd number,
/// and subtract one.
///
/// This algorithm is described in detail in "Computing the number of digits
/// of an integer quickly", available
/// [here](https://lemire.me/blog/2021/05/28/computing-the-number-of-digits-of-an-integer-quickly/).
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
///
/// - `2^32 for j = 1`
/// - `⌈log10(2^j)⌉ * 2^128 + 2^128 – 10^(⌈log10(2j)⌉) for j from 2 to 30`
/// - `⌈log10(2^j)⌉ for j = 31 and j = 32`
///
/// This algorithm is described in detail in "Computing the number of digits
/// of an integer even faster", available
/// [here](https://lemire.me/blog/2021/06/03/computing-the-number-of-digits-of-an-integer-even-faster/).
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
    let shift = unsafe { index_unchecked!(TABLE[fast_log2(x)]) };
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
#[inline]
pub fn fallback_digit_count<T: UnsignedInteger>(x: T, table: &[T]) -> usize {
    // This value is always within 1: calculate if we need to round-up
    // based on a pre-computed table.
    let log10 = fast_log10(x);
    let shift_up = table.get(log10).map_or(false, |&y| x >= y);

    log10 + shift_up as usize + 1
}

/// Quickly calculate the number of digits in a type.
pub trait DigitCount: UnsignedInteger {
    /// Get the number of digits in a value.
    fn digit_count(self) -> usize;
}

macro_rules! digit_count_unimpl {
    ($($t:ty)*) => ($(
        impl DigitCount for $t {
            #[inline]
            fn digit_count(self) -> usize {
                unimplemented!()
            }
        }
    )*)
}

digit_count_unimpl! { u8 u16 usize }

impl DigitCount for u32 {
    #[inline]
    fn digit_count(self) -> usize {
        fast_digit_count(self)
    }
}

impl DigitCount for u64 {
    #[inline]
    fn digit_count(self) -> usize {
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

impl DigitCount for u128 {
    #[inline]
    fn digit_count(self) -> usize {
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

/// Write integer to decimal string.
pub trait Decimal: DigitCount {
    /// # Safety
    ///
    /// Safe as long as buffer is at least [`FORMATTED_SIZE`] elements long,
    /// (or [`FORMATTED_SIZE_DECIMAL`] for decimal), and the radix is valid.
    ///
    /// [`FORMATTED_SIZE`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE
    /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
    unsafe fn decimal(self, buffer: &mut [u8]) -> usize;
}

// Don't implement decimal for small types, where we could have an overflow.
macro_rules! decimal_unimpl {
    ($($t:ty)*) => ($(
        impl Decimal for $t {
            #[inline(always)]
            unsafe fn decimal(self, _: &mut [u8]) -> usize {
                // Forces a hard error if we have a logic error in our code.
                unimplemented!()
            }
        }
    )*);
}

decimal_unimpl! { u8 u16 usize }

// Implement decimal for type.
macro_rules! decimal_impl {
    ($($t:ty)*) => ($(
        impl Decimal for $t {
            #[inline(always)]
            unsafe fn decimal(self, buffer: &mut [u8]) -> usize {
                // SAFETY: safe as long as buffer is large enough to hold the max value.
                let count = self.digit_count();
                debug_assert!(count <= buffer.len());
                unsafe {
                    algorithm(self, 10, &DIGIT_TO_BASE10_SQUARED, &mut buffer[..count]);
                    count
                }
            }
        }
    )*);
}

decimal_impl! { u32 u64 }

impl Decimal for u128 {
    #[inline(always)]
    unsafe fn decimal(self, buffer: &mut [u8]) -> usize {
        // SAFETY: safe as long as buffer is large enough to hold the max value.
        let count = self.digit_count();
        debug_assert!(count <= buffer.len());
        unsafe {
            algorithm_u128::<{ STANDARD }, { RADIX }, { RADIX_SHIFT }>(
                self,
                &DIGIT_TO_BASE10_SQUARED,
                &mut buffer[..count],
            );
            count
        }
    }
}
