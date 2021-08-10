//! Shared trait and methods for parsing integers.

#![doc(hidden)]

// Select the correct back-end.
#[cfg(not(feature = "compact"))]
use crate::algorithm::{algorithm_complete, algorithm_partial};
#[cfg(feature = "compact")]
use crate::compact::{algorithm_complete, algorithm_partial};

use lexical_util::num::{Integer, UnsignedInteger};
use lexical_util::result::Result;

/// Parse integer trait, implemented in terms of the optimized back-end.
pub trait ParseInteger: Integer {
    /// Forward complete parser parameters to the backend.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn parse_complete<Unsigned: UnsignedInteger, const FORMAT: u128>(bytes: &[u8]) -> Result<Self> {
        algorithm_complete::<_, Unsigned, { FORMAT }>(bytes)
    }

    /// Forward partial parser parameters to the backend.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn parse_partial<Unsigned: UnsignedInteger, const FORMAT: u128>(
        bytes: &[u8],
    ) -> Result<(Self, usize)> {
        algorithm_partial::<_, Unsigned, { FORMAT }>(bytes)
    }
}

macro_rules! parse_integer_impl {
    ($($t:ty)*) => ($(
        impl ParseInteger for $t {}
    )*)
}

parse_integer_impl! { u8 u16 u32 u64 u128 usize }
parse_integer_impl! { i8 i16 i32 i64 i128 isize }
