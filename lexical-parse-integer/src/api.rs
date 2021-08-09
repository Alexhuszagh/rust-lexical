////! Implements the algorithm in terms of the lexical API.

#![doc(hidden)]

use crate::options::Options;
use crate::parse::ParseInteger;
use lexical_util::format::{NumberFormat, STANDARD};
use lexical_util::{from_lexical, from_lexical_with_options};

/// Implement FromLexical for numeric type.
///
/// Need to inline these, otherwise codegen is suboptimal.
/// For some reason, it can't determine some of the const evaluations
/// can actually be evaluated at compile-time, which causes major branching
/// issues.
macro_rules! integer_from_lexical {
    ($($t:ident $unsigned:ident $(, #[$meta:meta])? ; )*) => ($(
        impl FromLexical for $t {
            $(#[$meta:meta])?
            #[cfg_attr(not(feature = "compact"), inline)]
            fn from_lexical(bytes: &[u8]) -> lexical_util::result::Result<Self>
            {
                Self::parse_complete::<$unsigned, STANDARD>(bytes)
            }

            $(#[$meta:meta])?
            #[cfg_attr(not(feature = "compact"), inline)]
            fn from_lexical_partial(
                bytes: &[u8],
            ) -> lexical_util::result::Result<(Self, usize)>
            {
                Self::parse_partial::<$unsigned, STANDARD>(bytes)
            }
        }

        impl FromLexicalWithOptions for $t {
            type Options = Options;

            $(#[$meta:meta])?
            #[cfg_attr(not(feature = "compact"), inline)]
            fn from_lexical_with_options<const FORMAT: u128>(
                bytes: &[u8],
                _: &Self::Options,
            ) -> lexical_util::result::Result<Self>
            {
                let format = NumberFormat::<{ FORMAT }> {};
                if !format.is_valid() {
                    return Err(format.error());
                }
                Self::parse_complete::<$unsigned, FORMAT>(bytes)
            }

            $(#[$meta:meta])?
            #[cfg_attr(not(feature = "compact"), inline)]
            fn from_lexical_partial_with_options<const FORMAT: u128>(
                bytes: &[u8],
                _: &Self::Options,
            ) -> lexical_util::result::Result<(Self, usize)>
            {
                let format = NumberFormat::<{ FORMAT }> {};
                if !format.is_valid() {
                    return Err(format.error());
                }
                Self::parse_partial::<$unsigned, FORMAT>(bytes)
            }
        }
    )*)
}

from_lexical! {}
from_lexical_with_options! {}
integer_from_lexical! {
    u8 u8 ;
    u16 u16 ;
    u32 u32 ;
    u64 u64 ;
    u128 u128 ;
    usize usize ;
    i8 u8 ;
    i16 u16 ;
    i32 u32 ;
    i64 u64 ;
    i128 u128 ;
    isize usize ;
}
