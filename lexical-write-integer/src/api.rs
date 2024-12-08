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
fn unsigned<T, const FORMAT: u128>(value: T, buffer: &mut [u8]) -> usize
where
    T: WriteInteger,
{
    let format = NumberFormat::<FORMAT> {};
    if cfg!(feature = "format") && format.required_mantissa_sign() {
        buffer[0] = b'+';
        let buffer = &mut buffer[1..];
        value.write_mantissa::<FORMAT>(buffer) + 1
    } else {
        value.write_mantissa::<FORMAT>(buffer)
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
fn signed<Signed, Unsigned, const FORMAT: u128>(value: Signed, buffer: &mut [u8]) -> usize
where
    Signed: SignedInteger,
    Unsigned: WriteInteger,
{
    let format = NumberFormat::<FORMAT> {};
    if value < Signed::ZERO {
        // Need to cast the value to the same size as unsigned type, since if
        // the value is **exactly** `Narrow::MIN`, and it it is then cast
        // as the wrapping negative as the unsigned value, a wider type
        // will have a very different value.
        let unsigned = Unsigned::as_cast(value.wrapping_neg());
        buffer[0] = b'-';
        let buffer = &mut buffer[1..];
        unsigned.write_mantissa_signed::<FORMAT>(buffer) + 1
    } else if cfg!(feature = "format") && format.required_mantissa_sign() {
        let unsigned = Unsigned::as_cast(value);
        buffer[0] = b'+';
        let buffer = &mut buffer[1..];
        unsigned.write_mantissa_signed::<FORMAT>(buffer) + 1
    } else {
        let unsigned = Unsigned::as_cast(value);
        unsigned.write_mantissa_signed::<FORMAT>(buffer)
    }
}

// API

// Implement `ToLexical` for numeric type.
macro_rules! unsigned_to_lexical {
    ($($t:tt)*) => ($(
        impl ToLexical for $t {
            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical(self, bytes: &mut [u8])
                -> &mut [u8]
            {
                let len = unsigned::<$t, { STANDARD }>(self, bytes);
                &mut bytes[..len]
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
                _ = options;
                assert!(NumberFormat::<{ FORMAT }> {}.is_valid());
                let len = unsigned::<$t, FORMAT>(self, bytes);
                &mut bytes[..len]
            }
        }
    )*)
}

to_lexical! {}
to_lexical_with_options! {}
unsigned_to_lexical! { u8 u16 u32 u64 u128 usize }

// Implement `ToLexical` for numeric type.
macro_rules! signed_to_lexical {
    ($($signed:tt $unsigned:tt ; )*) => ($(
        impl ToLexical for $signed {
            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical(self, bytes: &mut [u8])
                -> &mut [u8]
            {
                let len = signed::<$signed, $unsigned, { STANDARD }>(self, bytes);
                &mut bytes[..len]
            }
        }

        impl ToLexicalWithOptions for $signed {
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
                let len = signed::<$signed, $unsigned, FORMAT>(self, bytes);
                &mut bytes[..len]
            }
        }
    )*)
}

signed_to_lexical! {
    i8 u8 ;
    i16 u16 ;
    i32 u32 ;
    i64 u64 ;
    i128 u128 ;
}

#[cfg(target_pointer_width = "16")]
signed_to_lexical! { isize u16 ; }

#[cfg(target_pointer_width = "32")]
signed_to_lexical! { isize u32 ; }

#[cfg(target_pointer_width = "64")]
signed_to_lexical! { isize u64 ; }
