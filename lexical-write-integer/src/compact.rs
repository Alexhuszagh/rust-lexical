//! Radix-generic, lexical integer-to-string conversion routines.
//!
//! These routines are optimized for code size: they are significantly
//! slower at the cost of smaller binary size.

#![cfg(feature = "compact")]
#![doc(hidden)]

use core::mem;

use lexical_util::algorithm::copy_to_dst;
use lexical_util::assert::debug_assert_radix;
use lexical_util::digit::digit_to_char;
use lexical_util::num::{AsCast, UnsignedInteger};

/// Write integral digits to buffer.
///
/// This algorithm does not use any pre-computed tables, reducing code
/// size at the cost of faster performance.
///
/// # Safety
///
/// This is safe as long as the buffer is large enough to hold `T::MAX`
/// digits in radix `N`.
pub unsafe fn write_digits<T: UnsignedInteger>(
    mut value: T,
    radix: u32,
    buffer: &mut [u8],
    mut index: usize,
) -> usize {
    debug_assert_radix(radix);

    // SAFETY: All of these are safe for the buffer writes as long as
    // the buffer is large enough to hold `T::FORMATTED_SIZE` digits,
    // and `radix <= 36`.

    // Decode all but the last digit.
    let radix = T::from_u32(radix);
    while value >= radix {
        let r = value % radix;
        value /= radix;
        index -= 1;
        unsafe { index_unchecked_mut!(buffer[index]) = digit_to_char(u32::as_cast(r)) };
    }

    // Decode last digit.
    let r = value % radix;
    index -= 1;
    unsafe { index_unchecked_mut!(buffer[index]) = digit_to_char(u32::as_cast(r)) };

    index
}

/// Write integer to string.
pub trait Compact: UnsignedInteger {
    /// # Safety
    ///
    /// Safe as long as buffer is at least `FORMATTED_SIZE` elements long,
    /// (or `FORMATTED_SIZE_DECIMAL` for decimal), and the radix is valid.
    unsafe fn compact(self, radix: u32, buffer: &mut [u8]) -> usize {
        // SAFETY: safe as long as buffer is large enough to hold the max value.
        // We never read unwritten values, and we never assume the data is initialized.
        // Need at least 128-bits, at least as many as the bits in the current type.
        debug_assert!(Self::BITS <= 128);
        let mut digits: mem::MaybeUninit<[u8; 128]> = mem::MaybeUninit::uninit();
        unsafe {
            let digits = &mut *digits.as_mut_ptr();
            let length = digits.len();
            let index = write_digits(self, radix, digits, length);
            copy_to_dst(buffer, &mut index_unchecked_mut!(digits[index..]))
        }
    }
}

macro_rules! compact_impl {
    ($($t:ty)*) => ($(
        impl Compact for $t {}
    )*)
}

compact_impl! { u8 u16 u32 u64 u128 usize }
