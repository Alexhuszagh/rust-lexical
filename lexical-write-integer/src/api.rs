//! Implements the algorithm in terms of the lexical API.

#![doc(hidden)]

use lexical_util::format::{NumberFormat, STANDARD};
use lexical_util::num::SignedInteger;
use lexical_util::{to_lexical, to_lexical_with_options};

use crate::options::Options;
use crate::write::WriteInteger;

// UNSIGNED

/// Callback for unsigned integer formatter.
///
/// # Safety
///
/// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
/// (or `FORMATTED_SIZE_DECIMAL` for decimal).
#[cfg_attr(not(feature = "compact"), inline(always))]
fn unsigned<Narrow, Wide, const FORMAT: u128>(value: Narrow, buffer: &mut [u8]) -> usize
where
    Narrow: WriteInteger,
    Wide: WriteInteger,
{
    let format = NumberFormat::<FORMAT> {};
    if cfg!(feature = "format") && format.required_mantissa_sign() {
        buffer[0] = b'+';
        let buffer = &mut buffer[1..];
        value.write_mantissa::<Wide, FORMAT>(buffer) + 1
    } else {
        value.write_mantissa::<Wide, FORMAT>(buffer)
    }
}

// SIGNED

/// Callback for signed integer formatter.
///
/// # Safety
///
/// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
/// (or `FORMATTED_SIZE_DECIMAL` for decimal).
#[cfg_attr(not(feature = "compact"), inline(always))]
fn signed<Narrow, Wide, Unsigned, const FORMAT: u128>(value: Narrow, buffer: &mut [u8]) -> usize
where
    Narrow: SignedInteger,
    Wide: SignedInteger,
    Unsigned: WriteInteger,
{
    let format = NumberFormat::<FORMAT> {};
    if value < Narrow::ZERO {
        // Need to cast the value to the same size as unsigned type, since if
        // the value is **exactly** `Narrow::MIN`, and it it is then cast
        // as the wrapping negative as the unsigned value, a wider type
        // will have a very different value.
        let value = Wide::as_cast(value);
        let unsigned = Unsigned::as_cast(value.wrapping_neg());
        buffer[0] = b'-';
        let buffer = &mut buffer[1..];
        unsigned.write_mantissa::<Unsigned, FORMAT>(buffer) + 1
    } else if cfg!(feature = "format") && format.required_mantissa_sign() {
        let unsigned = Unsigned::as_cast(value);
        buffer[0] = b'+';
        let buffer = &mut buffer[1..];
        unsigned.write_mantissa::<Unsigned, FORMAT>(buffer) + 1
    } else {
        let unsigned = Unsigned::as_cast(value);
        unsigned.write_mantissa::<Unsigned, FORMAT>(buffer)
    }
}

// API

// Implement `ToLexical` for numeric type.
macro_rules! unsigned_to_lexical {
    ($($narrow:tt $wide:tt ; )*) => ($(
        impl ToLexical for $narrow {
            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical(self, bytes: &mut [u8])
                -> &mut [u8]
            {
                let len = unsigned::<$narrow, $wide, { STANDARD }>(self, bytes);
                &mut bytes[..len]
            }
        }

        impl ToLexicalWithOptions for $narrow {
            type Options = Options;

            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical_with_options<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8]
            {
                _ = options;
                assert!(NumberFormat::<{ FORMAT }> {}.is_valid());
                let len = unsigned::<$narrow, $wide, FORMAT>(self, bytes);
                &mut bytes[..len]
            }
        }
    )*)
}

to_lexical! {}
to_lexical_with_options! {}
unsigned_to_lexical! {
    u8 u32 ;
    u16 u32 ;
    u32 u32 ;
    u64 u64 ;
    u128 u128 ;
}

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
unsigned_to_lexical! { usize u32 ; }

#[cfg(target_pointer_width = "64")]
unsigned_to_lexical! { usize u64 ; }

// Implement `ToLexical` for numeric type.
macro_rules! signed_to_lexical {
    ($($narrow:tt $wide:tt $unsigned:tt ; )*) => ($(
        impl ToLexical for $narrow {
            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical(self, bytes: &mut [u8])
                -> &mut [u8]
            {
                let len = signed::<$narrow, $wide, $unsigned, { STANDARD }>(self, bytes);
                &mut bytes[..len]
            }
        }

        impl ToLexicalWithOptions for $narrow {
            type Options = Options;
            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical_with_options<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8]
            {
                _ = options;
                assert!(NumberFormat::<{ FORMAT }> {}.is_valid());
                let len = signed::<$narrow, $wide, $unsigned, FORMAT>(self, bytes);
                &mut bytes[..len]
            }
        }
    )*)
}

signed_to_lexical! {
    i8 i32 u32 ;
    i16 i32 u32 ;
    i32 i32 u32 ;
    i64 i64 u64 ;
    i128 i128 u128 ;
}

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
signed_to_lexical! { isize i32 u32 ; }

#[cfg(target_pointer_width = "64")]
signed_to_lexical! { isize i64 u64 ; }
