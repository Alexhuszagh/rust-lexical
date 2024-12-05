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

use lexical_util::format;
use lexical_util::num::{Integer, UnsignedInteger};

use crate::algorithm::{algorithm, algorithm_u128};
use crate::table::get_table;

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
                let radix = format::radix_from_flags(FORMAT, MASK, SHIFT);
                let table = get_table::<FORMAT, MASK, SHIFT>();
                algorithm(self, radix, table, buffer)
            }
        }
    )*);
}

radix_impl! { u8 u16 u32 u64 usize }

impl Radix for u128 {
    #[inline(always)]
    fn radix<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize {
        let table = get_table::<FORMAT, MASK, SHIFT>();
        algorithm_u128::<FORMAT, MASK, SHIFT>(self, table, buffer)
    }
}
