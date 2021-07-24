//! Radix-generic, lexical integer-to-string conversion routines.
//!
//! These routines are decently optimized: they unroll 4 loops at a time,
//! using pre-computed base^2 tables. However, due to static storage
//! reasons, it makes no sense to pre-compute the number of digits,
//! and therefore
//!
//! See [Algorithm.md](/Algorithm/md) for a more detailed description of
//! the algorithm choice here.

#![cfg(feature = "power-of-two")]

use crate::algorithm::{algorithm, algorithm_u128};
use crate::lib::mem;
use crate::table::get_table;
use lexical_util::algorithm::copy_to_dst;
use lexical_util::num::UnsignedInteger;

// Export integer to string.
pub trait Generic: UnsignedInteger {
    /// # SAFETY
    ///
    /// Safe as long as buffer is at least `FORMATTED_SIZE` elements long,
    /// (or `FORMATTED_SIZE_DECIMAL` for decimal), and the radix is valid.
    unsafe fn generic(self, radix: u32, buffer: &mut [u8]) -> usize;
}

// Don't implement generic for small types, where we could have an overflow.
macro_rules! generic_unimpl {
    ($($t:ty)*) => ($(
        impl Generic for $t {
            #[inline(always)]
            unsafe fn generic(self, _: u32, _: &mut [u8]) -> usize {
                // Forces a hard error if we have a logic error in our code.
                unimplemented!()
            }
        }
    )*);
}

generic_unimpl! { u8 u16 usize }

// Implement generic for type.
macro_rules! generic_impl {
    ($($t:ty)*) => ($(
        impl Generic for $t {
            #[inline(always)]
            unsafe fn generic(self, radix: u32, buffer: &mut [u8]) -> usize {
                // SAFETY: safe as long as buffer is large enough to hold the max value.
                // We never read unwritten values, and we never assume the data is initialized.
                let mut digits: mem::MaybeUninit<[u8; 64]> = mem::MaybeUninit::uninit();
                unsafe {
                    let digits = &mut *digits.as_mut_ptr();
                    let table = get_table(radix);
                    let index = algorithm(self, radix, table, digits);
                    copy_to_dst(buffer, &mut digits.get_unchecked_mut(index..))
                }
            }
        }
    )*);
}

generic_impl! { u32 u64 }

impl Generic for u128 {
    #[inline(always)]
    unsafe fn generic(self, radix: u32, buffer: &mut [u8]) -> usize {
        // SAFETY: safe as long as buffer is large enough to hold the max value.
        // We never read unwritten values, and we never assume the data is initialized.
        let mut digits: mem::MaybeUninit<[u8; 128]> = mem::MaybeUninit::uninit();
        unsafe {
            let digits = &mut *digits.as_mut_ptr();
            let table = get_table(radix);
            let index = algorithm_u128(self, radix, table, digits);
            copy_to_dst(buffer, &mut digits.get_unchecked_mut(index..))
        }
    }
}
