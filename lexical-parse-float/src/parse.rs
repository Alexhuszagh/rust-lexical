//! Shared trait and methods for parsing floats.

#![doc(hidden)]
#![allow(unused)] // TODO(ahuszagh) Remove...

use crate::options::Options;
use lexical_util::num::Float;
use lexical_util::result::Result;

// API
// ---

/// Parse integer trait, implemented in terms of the optimized back-end.
pub trait ParseFloat: Float {
    /// Forward complete parser parameters to the backend.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn parse_complete<const FORMAT: u128>(bytes: &[u8], options: &Options) -> Result<Self> {
        // TODO(ahuszagh) Need to implement...
        todo!()
    }

    /// Forward complete parser parameters to the backend.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn parse_partial<const FORMAT: u128>(bytes: &[u8], options: &Options) -> Result<(Self, usize)> {
        // TODO(ahuszagh) Need to implement...
        todo!()
    }
}

macro_rules! parse_float_impl {
    ($($t:ty)*) => ($(
        impl ParseFloat for $t {}
    )*)
}

parse_float_impl! { f32 f64 }

// PARSE
// -----
