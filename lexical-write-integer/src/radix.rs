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

use crate::algorithm::{algorithm, algorithm_u128};
use crate::lib::mem;
use crate::table::get_table;
use lexical_util::algorithm::copy_to_dst;
use lexical_util::num::UnsignedInteger;

/// Write integer to radix string.
pub trait Radix: UnsignedInteger {
    /// # SAFETY
    ///
    /// Safe as long as buffer is at least `FORMATTED_SIZE` elements long,
    /// (or `FORMATTED_SIZE_DECIMAL` for decimal), and the radix is valid.
    unsafe fn radix<const RADIX: u32>(self, buffer: &mut [u8]) -> usize;
}

// Don't implement radix for small types, where we could have an overflow.
macro_rules! radix_unimpl {
    ($($t:ty)*) => ($(
        impl Radix for $t {
            #[inline(always)]
            unsafe fn radix<const __: u32>(self, _: &mut [u8]) -> usize {
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
            unsafe fn radix<const RADIX: u32>(self, buffer: &mut [u8]) -> usize {
                // SAFETY: safe as long as buffer is large enough to hold the max value.
                // We never read unwritten values, and we never assume the data is initialized.
                debug_assert!(Self::BITS <= 64);
                let mut digits: mem::MaybeUninit<[u8; 64]> = mem::MaybeUninit::uninit();
                unsafe {
                    let digits = &mut *digits.as_mut_ptr();
                    let table = get_table::<RADIX>();
                    let index = algorithm(self, RADIX, table, digits);
                    copy_to_dst(buffer, &mut digits.get_unchecked_mut(index..))
                }
            }
        }
    )*);
}

radix_impl! { u32 u64 }

impl Radix for u128 {
    #[inline(always)]
    unsafe fn radix<const RADIX: u32>(self, buffer: &mut [u8]) -> usize {
        // SAFETY: safe as long as buffer is large enough to hold the max value.
        // We never read unwritten values, and we never assume the data is initialized.
        // Need at least 128-bits, at least as many as the bits in the current type.
        debug_assert!(Self::BITS <= 128);
        let mut digits: mem::MaybeUninit<[u8; 128]> = mem::MaybeUninit::uninit();
        unsafe {
            let digits = &mut *digits.as_mut_ptr();
            let table = get_table::<RADIX>();
            let index = algorithm_u128::<RADIX>(self, table, digits);
            copy_to_dst(buffer, &mut digits.get_unchecked_mut(index..))
        }
    }
}
