//! Implements the algorithm in terms of the lexical API.

#![doc(hidden)]

use crate::options::Options;
use crate::write::WriteFloat;
#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
use lexical_util::constants::FormattedSize;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_util::format::STANDARD;
use lexical_util::options::WriteOptions;
use lexical_util::{to_lexical, to_lexical_with_options};

/// Check if a buffer is sufficiently large.
#[inline(always)]
fn check_buffer<T, const FORMAT: u128>(len: usize, options: &Options) -> bool
where
    T: FormattedSize,
{
    let size = Options::buffer_size::<T, FORMAT>(options);
    len >= size
}

// API

const DEFAULT_OPTIONS: Options = Options::new();

// Implement ToLexical for numeric type.
macro_rules! float_to_lexical {
    ($($t:tt $(, #[$meta:meta])? ; )*) => ($(
        impl ToLexical for $t {
            $(#[$meta:meta])?
            fn to_lexical(self, bytes: &mut [u8])
                -> &mut [u8]
            {
                // TODO: Remove, move inside
                assert!(check_buffer::<Self, { STANDARD }>(bytes.len(), &DEFAULT_OPTIONS));
                // SAFETY: safe since `check_buffer::<STANDARD>(bytes.len(), &options)` passes.
                unsafe {
                    let len = self.write_float::<{ STANDARD }>(bytes, &DEFAULT_OPTIONS);
                    &mut index_unchecked_mut!(bytes[..len])
                }
            }
        }

        impl ToLexicalWithOptions for $t {
            type Options = Options;
            $(#[$meta:meta])?
            fn to_lexical_with_options<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8]
            {
                // TODO: Remove, move inside
                assert!(check_buffer::<Self, { FORMAT }>(bytes.len(), &options));
                // SAFETY: safe since `check_buffer::<FORMAT>(bytes.len(), &options)` passes.
                unsafe {
                    let len = self.write_float::<{ FORMAT }>(bytes, &options);
                    &mut index_unchecked_mut!(bytes[..len])
                }
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
