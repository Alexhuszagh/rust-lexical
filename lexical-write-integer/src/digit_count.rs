//! Get the number of digits for an integer formatted for a radix.
//!
//! This will always accurately calculate the number of digits for
//! a given radix, using optimizations for cases with a power-of-two
//! and decimal numbers.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use lexical_util::{
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

// Uses a naive approach to calculate the number of digits.
macro_rules! naive_count {
    ($t:ty, $radix:expr, $x:expr) => {{
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
            10 => self.decimal_count(),
            // NOTE: This is currently horribly inefficient and exists just for correctness.
            // FIXME: Optimize for power-of-two radices
            // FIXME: Optimize for non-power-of-two radices.
            _ => naive_count!(Self, radix, self),
        }
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
        assert!((2..=36).contains(&radix), "radix must be >= 2 and <= 36");
        match radix {
            10 => self.decimal_count(),
            // FIXME: Optimize this
            _ => {
                // NOTE: This follows the same implementation as the digit count
                // generation, so this is safe.
                if self <= u64::MAX as u128 {
                    return naive_count!(u64, radix, self as u64);
                }

                // Doesn't fit in 64 bits, let's try our divmod.
                let step = u64_step(radix);
                let (value, _) = u128_divrem(self, radix);
                let mut count = step;
                if value <= u64::MAX as u128 {
                    count += naive_count!(u64, radix, value as u64);
                } else {
                    // Value has to be greater than 1.8e38
                    let (value, _) = u128_divrem(value, radix);
                    count += step;
                    if value != 0 {
                        count += naive_count!(u64, radix, value as u64);
                    }
                }

                count
            },
        }
    }
}
