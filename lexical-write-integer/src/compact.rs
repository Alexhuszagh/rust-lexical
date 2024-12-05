//! Radix-generic, lexical integer-to-string conversion routines.
//!
//! These routines are optimized for code size: they are significantly
//! slower at the cost of smaller binary size.

#![cfg(feature = "compact")]
#![doc(hidden)]

use lexical_util::algorithm::copy_to_dst;
use lexical_util::constants::FormattedSize;
use lexical_util::digit::digit_to_char;
use lexical_util::num::{AsCast, UnsignedInteger};

// NOTE: Testing our algorithms can be done effectively as:
//  table = [
//      '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D',
//      'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
//      'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
//  ]
//  def to_string(value, radix):
//      result = ''
//      while value >= radix:
//          r = value % radix
//          value //= radix
//          result = table[int(r)] + result
//
//      r = value % radix
//      result = table[int(r)] + result
//
//      return result

/// Write integer to string.
pub trait Compact: UnsignedInteger + FormattedSize {
    /// Write our integer to string without optimizations.
    ///
    /// This iterates over the buffer in reverse, and ensures that at least
    /// 128 elements exist in the temporary buffer. This ensures that the digits
    /// can never overflow, even in base 2. The buffer then must be able to hold
    /// at least 128 digits as well, after subslicing the data. This guarantee
    /// is likely already made.
    fn compact(self, radix: u32, buffer: &mut [u8]) -> usize {
        // NOTE: We do not have to validate the buffer length because `copy_to_dst` is
        // safe.
        assert!(Self::BITS <= 128);
        let mut digits: [u8; 128] = [0u8; 128];
        let mut index = digits.len();

        // Decode all but the last digit.
        let radix = Self::from_u32(radix);
        let mut value = self;
        while value >= radix {
            let r = value % radix;
            value /= radix;
            index -= 1;
            digits[index] = digit_to_char(u32::as_cast(r));
        }

        // Decode last digit.
        let r = value % radix;
        index -= 1;
        digits[index] = digit_to_char(u32::as_cast(r));
        let slc = &digits[index..];
        copy_to_dst(buffer, slc)
    }
}

macro_rules! compact_impl {
    ($($t:ty)*) => ($(
        impl Compact for $t {}
    )*)
}

compact_impl! { u8 u16 u32 u64 u128 usize }
