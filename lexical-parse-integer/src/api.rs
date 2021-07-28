//! Implements the algorithm in terms of the lexical API.

use crate::options::Options;
use crate::parse::ParseInteger;
use lexical_util::format::{NumberFormat, STANDARD};
use lexical_util::{from_lexical, from_lexical_with_options, lexical_partial_to_complete};

// Implement FromLexical for numeric type.
macro_rules! integer_from_lexical {
    ($($t:tt $(, #[$meta:meta])? ; )*) => ($(
        impl FromLexical for $t {
            $(#[$meta:meta])?
            fn from_lexical(bytes: &[u8]) -> lexical_util::result::Result<Self>
            {
                lexical_partial_to_complete!(Self::from_lexical_partial, bytes)
            }

            fn from_lexical_partial(
                bytes: &[u8],
            ) -> lexical_util::result::Result<(Self, usize)>
            {
                Self::parse_integer::<STANDARD>(bytes)
            }
        }

        impl FromLexicalWithOptions for $t {
            type Options = Options;

            $(#[$meta:meta])?
            fn from_lexical_with_options<const FORMAT: u128>(
                bytes: &[u8],
                options: &Self::Options,
            ) -> lexical_util::result::Result<Self>
            {
                lexical_partial_to_complete!(
                    Self::from_lexical_partial_with_options::<FORMAT>,
                    bytes,
                    options
                )
            }

            fn from_lexical_partial_with_options<const FORMAT: u128>(
                bytes: &[u8],
                _: &Self::Options,
            ) -> lexical_util::result::Result<(Self, usize)>
            {
                let format = NumberFormat::<{ FORMAT }> {};
                if !format.is_valid() {
                    return Err(format.error());
                }
                Self::parse_integer::<FORMAT>(bytes)
            }
        }
    )*)
}

from_lexical! {}
from_lexical_with_options! {}
integer_from_lexical! {
    u8 ;
    u16 ;
    u32 ;
    u64 ;
    u128 ;
    usize ;
    i8 ;
    i16 ;
    i32 ;
    i64 ;
    i128 ;
    isize ;
}
