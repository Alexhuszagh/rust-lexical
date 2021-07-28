//! Shared trait and methods for parsing integers.

#[cfg(feature = "format")]
use lexical_util::format::NumberFormat;
use lexical_util::noskip::NoSkipIter;
use lexical_util::num::Integer;
use lexical_util::result::Result;
#[cfg(feature = "format")]
use lexical_util::skip::SkipIter;

#[cfg(not(feature = "compact"))]
use crate::algorithm::{algorithm, algorithm_128};
/// Select the back-end.
#[cfg(feature = "compact")]
use crate::compact::algorithm;

/// Implement `ParseInteger` depending on the number format.
macro_rules! parse_integer {
    ($algorithm:ident, $bytes:ident, $format:ident) => {{
        #[cfg(not(feature = "format"))]
        {
            return $algorithm::<_, _, $format>($bytes.noskip_iter());
        }

        #[cfg(feature = "format")]
        {
            let format = NumberFormat::<{ $format }> {};
            if format.digit_separator() == 0 {
                return $algorithm::<_, _, $format>($bytes.noskip_iter());
            } else {
                return $algorithm::<_, _, $format>($bytes.skip_iter::<{ $format }>());
            }
        }
    }};
}

/// Parse integer trait, implemented in terms of the compact back-end.
#[cfg(feature = "compact")]
pub trait ParseInteger: Integer {
    /// Forward parse integer parameters to an unoptimized backend.
    fn parse_integer<const FORMAT: u128>(bytes: &[u8]) -> Result<(Self, usize)> {
        parse_integer!(algorithm, bytes, FORMAT)
    }
}

/// Parse integer trait, implemented in terms of the optimized back-end.
#[cfg(not(feature = "compact"))]
pub trait ParseInteger: Integer {
    /// Forward parse integer parameters to an optimized backend.
    fn parse_integer<const FORMAT: u128>(bytes: &[u8]) -> Result<(Self, usize)> {
        if Self::BITS == 128 {
            parse_integer!(algorithm_128, bytes, FORMAT)
        } else {
            parse_integer!(algorithm, bytes, FORMAT)
        }
    }
}

macro_rules! parse_integer_impl {
    ($($t:ty)*) => ($(
        impl ParseInteger for $t {}
    )*)
}

parse_integer_impl! { u8 u16 u32 u64 u128 usize }
parse_integer_impl! { i8 i16 i32 i64 i128 isize }
