//! Implements the algorithm in terms of the lexical API.

#![doc(hidden)]

#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_util::format::STANDARD;
use lexical_util::{to_lexical, to_lexical_with_options};

use crate::options::Options;
use crate::write::WriteFloat;

// API

const DEFAULT_OPTIONS: Options = Options::new();

// Implement `ToLexical` for numeric type.
macro_rules! float_to_lexical {
    ($($t:tt ; )*) => ($(
        impl ToLexical for $t {
            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical(self, bytes: &mut [u8])
                -> &mut [u8]
            {
                let count = self.write_float::<{ STANDARD }>(bytes, &DEFAULT_OPTIONS);
                &mut bytes[..count]
            }
        }

        impl ToLexicalWithOptions for $t {
            type Options = Options;
            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical_with_options<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8]
            {
                let count = self.write_float::<{ FORMAT }>(bytes, &options);
                &mut bytes[..count]
            }
        }
    )*)
}

to_lexical! {}
to_lexical_with_options! {}
float_to_lexical! {
    f32 ;
    f64 ;
}
#[cfg(feature = "f16")]
float_to_lexical! {
    f16 ;
    bf16 ;
}
