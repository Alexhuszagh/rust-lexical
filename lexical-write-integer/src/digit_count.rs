//! Get the number of digits for an integer formatted for a radix.
//!
//! This will always accurately calculate the number of digits for
//! a given radix, using optimizations for cases with a power-of-two
//! and decimal numbers.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use lexical_util::{
    assert::debug_assert_radix,
    div128::u128_divrem,
    num::{AsPrimitive, UnsignedInteger},
    step::u64_step,
};

use crate::decimal::DecimalCount;

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
#[inline(always)]
pub fn fast_log2<T: UnsignedInteger>(x: T) -> usize {
    T::BITS - 1 - (x | T::ONE).leading_zeros() as usize
}

// Algorithms to calculate the number of digits from a single value.
macro_rules! digit_count {
    // Highly-optimized digit count for 2^N values.
    (@2 $x:expr) => {{
        digit_log2($x)
    }};

    (@4 $x:expr) => {{
        digit_log4($x)
    }};

    (@8 $x:expr) => {{
        digit_log8($x)
    }};

    (@16 $x:expr) => {{
        digit_log16($x)
    }};

    (@32 $x:expr) => {{
        digit_log32($x)
    }};

    // Uses a naive approach to calculate the number of digits.
    // This uses multi-digit optimizations when possible, and always
    // accurately calculates the number of digits similar to how
    // the digit generation algorithm works, just without the table
    // lookups.
    //
    // There's no good way to do this, since float logn functions are
    // lossy and the value might not be exactly represented in the type
    // (that is, `>= 2^53`), in which case `log(b^x, b)`, `log(b^x + 1, b)`,
    // and `log(b^x - 1, b)` would all be the same. Rust's integral [`ilog`]
    // functions just use naive 1-digit at a time multiplication, so it's
    // less efficient than our optimized variant.
    //
    // [`ilog`]: https://github.com/rust-lang/rust/blob/0e98766/library/core/src/num/uint_macros.rs#L1290-L1320
    (@naive $t:ty, $radix:expr, $x:expr) => {{
        // If we can do multi-digit optimizations, it's ideal,
        // so we want to check if our type size max value is >=
        // to the value.
        let radix = $radix as u32;
        let radix2 = radix * radix;
        let radix4 = radix2 * radix2;

        // NOTE: For radix 10, 0-9 would be 1 digit while 10-99 would be 2 digits,
        // so this needs to be `>=` and not `>`.
        let mut digits = 1;
        let mut value = $x;

        // try 4-digit optimizations
        if <$t>::BITS >= 32 || radix4 < <$t>::MAX.as_u32() {
            let radix4 = <$t as AsPrimitive>::from_u32(radix4);
            while value >= radix4 {
                digits += 4;
                value /= radix4;
            }
        }

        // try 2-digit optimizations
        if <$t>::BITS >= 16 || radix2 < <$t>::MAX.as_u32() {
            let radix2 = <$t as AsPrimitive>::from_u32(radix2);
            while value >= radix2 {
                digits += 2;
                value /= radix2;
            }
        }

        // can only do a single digit
        let radix = <$t as AsPrimitive>::from_u32(radix);
        while value >= radix {
            digits += 1;
            value /= radix;
        }
        digits
    }};
}

/// Highly-optimized digit count for base2 values.
///
/// This is always the number of `BITS - ctlz(x | 1)`, so it's
/// `fast_log2(x) + 1`. This is because 0 has 1 digit, as does 1,
/// but 2 and 3 have 2, etc.
#[inline(always)]
fn digit_log2<T: UnsignedInteger>(x: T) -> usize {
    fast_log2(x) + 1
}

/// Highly-optimized digit count for base4 values.
///
/// This is very similar to base 2, except we divide by 2
/// and adjust by 1. For example, `fast_log2(3) == 1`, so
/// `fast_log2(3) / 2 == 0`, which then gives us our result.
///
/// This works because `log2(x) / 2 == log4(x)`. Flooring is
/// the correct approach since `log2(15) == 3`, which should be
/// 2 digits (so `3 / 2 + 1`).
#[inline(always)]
fn digit_log4<T: UnsignedInteger>(x: T) -> usize {
    (fast_log2(x) / 2) + 1
}

/// Highly-optimized digit count for base8 values.
///
/// This works because `log2(x) / 3 == log8(x)`. Flooring is
/// the correct approach since `log2(63) == 5`, which should be
/// 2 digits (so `5 / 3 + 1`).
#[inline(always)]
fn digit_log8<T: UnsignedInteger>(x: T) -> usize {
    (fast_log2(x) / 3) + 1
}

/// Highly-optimized digit count for base16 values.
///
/// This works because `log2(x) / 4 == log16(x)`. Flooring is
/// the correct approach since `log2(255) == 7`, which should be
/// 2 digits (so `7 / 4 + 1`).
#[inline(always)]
fn digit_log16<T: UnsignedInteger>(x: T) -> usize {
    (fast_log2(x) / 4) + 1
}

/// Highly-optimized digit count for base32 values.
///
/// This works because `log2(x) / 5 == log32(x)`. Flooring is
/// the correct approach since `log2(1023) == 9`, which should be
/// 2 digits (so `9 / 5 + 1`).
#[inline(always)]
fn digit_log32<T: UnsignedInteger>(x: T) -> usize {
    (fast_log2(x) / 5) + 1
}

/// Quickly calculate the number of digits in a type.
///
/// This uses optimizations for powers-of-two and decimal
/// numbers, which can correctly calculate the number of
/// values without requiring logs or other expensive
/// calculations.
///
/// # Safety
///
/// Safe as long as `digit_count` returns at least the number of
/// digits that would be written by the integer. If the value is
/// too small, then the buffer might underflow, causing out-of-bounds
/// read/writes.
pub unsafe trait DigitCount: UnsignedInteger + DecimalCount {
    /// Get the number of digits in a value.
    #[inline(always)]
    fn digit_count(self, radix: u32) -> usize {
        assert!((2..=36).contains(&radix), "radix must be >= 2 and <= 36");
        match radix {
            // decimal
            10 => self.decimal_count(),
            // 2^N
            2 => digit_count!(@2 self),
            4 => digit_count!(@4 self),
            8 => digit_count!(@8 self),
            16 => digit_count!(@16 self),
            32 => digit_count!(@32 self),
            // fallback
            _ => digit_count!(@naive Self, radix, self),
        }
    }

    /// Get the number of digits in a value, always using the slow algorithm.
    ///
    /// This is exposed for testing purposes.
    #[inline(always)]
    fn slow_digit_count(self, radix: u32) -> usize {
        digit_count!(@naive Self, radix, self)
    }
}

// Implement digit counts for all types.
macro_rules! digit_impl {
    ($($t:ty)*) => ($(
        // SAFETY: Safe since it uses the default implementation.
        unsafe impl DigitCount for $t {
        }
    )*);
}

digit_impl! { u8 u16 usize u32 u64 }

// SAFETY: Safe since it specialized in terms of `div128_rem`, which
// is the same way the digits are generated.
unsafe impl DigitCount for u128 {
    /// Get the number of digits in a value.
    #[inline(always)]
    fn digit_count(self, radix: u32) -> usize {
        debug_assert_radix(radix);
        match radix {
            // decimal
            10 => self.decimal_count(),
            // 2^N
            2 => digit_count!(@2 self),
            4 => digit_count!(@4 self),
            8 => digit_count!(@8 self),
            16 => digit_count!(@16 self),
            32 => digit_count!(@32 self),
            // fallback
            _ => {
                // NOTE: This follows the same implementation as the digit count
                // generation, so this is safe.
                if self <= u64::MAX as u128 {
                    return digit_count!(@naive u64, radix, self as u64);
                }

                // Doesn't fit in 64 bits, let's try our divmod.
                let step = u64_step(radix);
                let (value, _) = u128_divrem(self, radix);
                let mut count = step;
                if value <= u64::MAX as u128 {
                    count += digit_count!(@naive u64, radix, value as u64);
                } else {
                    // Value has to be greater than 1.8e38
                    let (value, _) = u128_divrem(value, radix);
                    count += step;
                    if value != 0 {
                        count += digit_count!(@naive u64, radix, value as u64);
                    }
                }

                count
            },
        }
    }
}
