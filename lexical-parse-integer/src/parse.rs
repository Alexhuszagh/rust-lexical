//! Shared trait and methods for parsing integers.

// Select the correct back-end.
#[cfg(not(feature = "compact"))]
use crate::algorithm::{algorithm_complete, algorithm_partial};
#[cfg(feature = "compact")]
use crate::compact::{algorithm_complete, algorithm_partial};

use lexical_util::num::Integer;
use lexical_util::result::Result;

/// Parse integer trait, implemented in terms of the optimized back-end.
pub trait ParseInteger: Integer {
    /// Forward complete parser parameters to an unoptimized backend.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn parse_complete<const FORMAT: u128>(bytes: &[u8]) -> Result<Self> {
        algorithm_complete::<_, { FORMAT }>(bytes)
    }

    /// Forward partial parser parameters to an unoptimized backend.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn parse_partial<const FORMAT: u128>(bytes: &[u8]) -> Result<(Self, usize)> {
        algorithm_partial::<_, { FORMAT }>(bytes)
    }
}

macro_rules! parse_integer_impl {
    ($($t:ty)*) => ($(
        impl ParseInteger for $t {}
    )*)
}

parse_integer_impl! { u8 u16 u32 u64 u128 usize }
parse_integer_impl! { i8 i16 i32 i64 i128 isize }
