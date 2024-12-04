//! Get the number of digits for an integer formatted for a radix.
//!
//! This will always accurately calculate the number of digits for
//! a given radix, using optimizations for cases with a power-of-two
//! and decimal numbers.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use lexical_util::num::UnsignedInteger;

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
            // FIXME: Optimize for u128
            // FIXME: Optimize for non-power-of-two radices.
            _ => {
                let radix = Self::from_u32(radix);
                let mut digits = 1;
                // NOTE: For radix 10, 0-9 would be 1 digit while 10-99 would be 2 digits,
                // so this needs to be >=
                let mut value = self;
                while value >= radix {
                    digits += 1;
                    value /= radix;
                }
                digits
            },
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

digit_impl! { u8 u16 usize u32 u64 u128 }
