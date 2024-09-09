//! Radix-generic, lexical integer-to-string conversion routines.
//!
//! These routines are decently optimized: they unroll 4 loops at a time,
//! using pre-computed base^2 tables. However, due to static storage
//! reasons, it makes no sense to pre-compute the number of digits,
//! and therefore
//!
//! See [Algorithm.md](/docs/Algorithm.md) for a more detailed description of
//! the algorithm choice here.

#![cfg(not(feature = "compact"))]
#![cfg(feature = "power-of-two")]
#![doc(hidden)]

use crate::algorithm::{algorithm, algorithm_u128};
use crate::table::get_table;
use core::mem;
use lexical_util::algorithm::copy_to_dst;
use lexical_util::format;
use lexical_util::num::{Integer, UnsignedInteger};
use lexical_util::format::NumberFormat;
use lexical_util::assert::assert_buffer;

/// Write integer to radix string.
pub trait Radix: UnsignedInteger {
    /// # Safety
    ///
    /// Safe as long as buffer is at least `FORMATTED_SIZE` elements long,
    /// (or `FORMATTED_SIZE_DECIMAL` for decimal), and the radix is valid.
    fn radix<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize;
}

// Don't implement radix for small types, where we could have an overflow.
macro_rules! radix_unimpl {
    ($($t:ty)*) => ($(
        impl Radix for $t {
            #[inline(always)]
            fn radix<const __: u128, const ___: u128, const ____: i32>(self, _: &mut [u8]) -> usize {
                // Forces a hard error if we have a logic error in our code.
                unimplemented!()
            }
        }
    )*);
}

radix_unimpl! { u8 u16 usize }

// Implement radix for type.
macro_rules! radix_impl {
    ($($t:ty)*) => ($(
        impl Radix for $t {
            #[inline(always)]
            fn radix<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
                self,
                buffer: &mut [u8]
            ) -> usize {
                debug_assert!(<Self as Integer>::BITS <= 64);
                assert_buffer::<$t>(NumberFormat::<{ FORMAT }>::RADIX, buffer.len());
                let radix = format::radix_from_flags(FORMAT, MASK, SHIFT);
                // TODO: Remove unsafe
                let table = unsafe { get_table::<FORMAT, MASK, SHIFT>() };

                let mut digits: mem::MaybeUninit<[u8; 64]> = mem::MaybeUninit::uninit();
                // # Safety
                //
                // Safe as long as buffer is large enough to hold the max value, which we validate.
                // above. We never read unwritten values, and we never assume the data is initialized.
                // Need at least $T::BITS-bits, at least as many as the bits in the current type. Ensuring
                // no uninitialized memory is read is verified by miri.
                unsafe {
                    let digits = &mut *digits.as_mut_ptr();
                    let index = algorithm(self, radix, table, digits);
                    copy_to_dst(buffer, &mut index_unchecked_mut!(digits[index..]))
                }
            }
        }
    )*);
}

radix_impl! { u32 u64 }

impl Radix for u128 {
    #[inline(always)]
    fn radix<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize {
        debug_assert!(<Self as Integer>::BITS <= 128);
        assert_buffer::<u128>(NumberFormat::<{ FORMAT }>::RADIX, buffer.len());
        // TODO: Remove unsafe
        let table = unsafe { get_table::<FORMAT, MASK, SHIFT>() };

        let mut digits: mem::MaybeUninit<[u8; 128]> = mem::MaybeUninit::uninit();
        // # Safety
        //
        // Safe as long as buffer is large enough to hold the max value, which we validate.
        // above. We never read unwritten values, and we never assume the data is initialized.
        // Need at least 128-bits, at least as many as the bits in the current type. Ensuring
        // no uninitialized memory is read is verified by miri.
        unsafe {
            let digits = &mut *digits.as_mut_ptr();
            let index = algorithm_u128::<FORMAT, MASK, SHIFT>(self, table, digits);
            copy_to_dst(buffer, &mut index_unchecked_mut!(digits[index..]))
        }
    }
}
