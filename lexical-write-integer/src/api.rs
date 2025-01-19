//! Implements the algorithm in terms of the lexical API.

#![doc(hidden)]

use lexical_util::error::Error;
use lexical_util::format::{NumberFormat, STANDARD};
use lexical_util::num::SignedInteger;
use lexical_util::{to_lexical, to_lexical_with_options};

use crate::options::Options;
use crate::write::WriteInteger;

// UNSIGNED

// write the base prefix, if present.
macro_rules! write_base_prefix {
    ($format:ident, $buffer:ident) => {
        if cfg!(all(feature = "format", feature = "power-of-two")) && $format.required_base_prefix()
        {
            $buffer[0] = b'0';
            $buffer[1] = $format.base_prefix();
            (&mut $buffer[2..], 2)
        } else {
            ($buffer, 0)
        }
    };
}

// write the base prefix, if present.
macro_rules! write_base_suffix {
    ($format:ident, $buffer:ident, $index:ident) => {
        if cfg!(all(feature = "format", feature = "power-of-two")) && $format.required_base_suffix()
        {
            $buffer[$index] = $format.base_suffix();
            1
        } else {
            0
        }
    };
}

/// Callback for unsigned integer formatter.
///
/// # Safety
///
/// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
/// (or `FORMATTED_SIZE_DECIMAL` for decimal).
///
/// # Preconditions
///
/// This assumes it is writing the full integer: that is, it is not
/// writing it from within a float writer or similar.
#[cfg_attr(not(feature = "compact"), inline(always))]
fn unsigned<T, const FORMAT: u128>(value: T, mut buffer: &mut [u8]) -> usize
where
    T: WriteInteger,
{
    let format: NumberFormat<FORMAT> = NumberFormat::<FORMAT> {};
    let prefix: usize;
    let written: usize;
    let suffix: usize;
    if cfg!(feature = "format") && format.required_mantissa_sign() {
        buffer[0] = b'+';
        buffer = &mut buffer[1..];
        (buffer, prefix) = write_base_prefix!(format, buffer);
        written = value.write_mantissa::<FORMAT>(buffer);
        suffix = write_base_suffix!(format, buffer, written);
        written + prefix + suffix + 1
    } else {
        (buffer, prefix) = write_base_prefix!(format, buffer);
        written = value.write_mantissa::<FORMAT>(buffer);
        suffix = write_base_suffix!(format, buffer, written);
        written + prefix + suffix
    }
}

// SIGNED

/// Callback for signed integer formatter.
///
/// # Safety
///
/// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
/// (or `FORMATTED_SIZE_DECIMAL` for decimal).
///
/// # Preconditions
///
/// This assumes it is writing the full integer: that is, it is not
/// writing it from within a float writer or similar.
#[cfg_attr(not(feature = "compact"), inline(always))]
fn signed<Signed, Unsigned, const FORMAT: u128>(value: Signed, mut buffer: &mut [u8]) -> usize
where
    Signed: SignedInteger,
    Unsigned: WriteInteger,
{
    let format = NumberFormat::<FORMAT> {};
    let prefix: usize;
    let written: usize;
    let suffix: usize;
    if value < Signed::ZERO {
        // Need to cast the value to the same size as unsigned type, since if
        // the value is **exactly** `Narrow::MIN`, and it it is then cast
        // as the wrapping negative as the unsigned value, a wider type
        // will have a very different value.
        let unsigned = Unsigned::as_cast(value.wrapping_neg());
        buffer[0] = b'-';
        buffer = &mut buffer[1..];
        (buffer, prefix) = write_base_prefix!(format, buffer);
        let written = unsigned.write_mantissa_signed::<FORMAT>(buffer);
        suffix = write_base_suffix!(format, buffer, written);
        written + prefix + suffix + 1
    } else if cfg!(feature = "format") && format.required_mantissa_sign() {
        let unsigned = Unsigned::as_cast(value);
        buffer[0] = b'+';
        buffer = &mut buffer[1..];
        (buffer, prefix) = write_base_prefix!(format, buffer);
        written = unsigned.write_mantissa_signed::<FORMAT>(buffer);
        suffix = write_base_suffix!(format, buffer, written);
        written + prefix + suffix + 1
    } else {
        (buffer, prefix) = write_base_prefix!(format, buffer);
        let unsigned = Unsigned::as_cast(value);
        written = unsigned.write_mantissa_signed::<FORMAT>(buffer);
        suffix = write_base_suffix!(format, buffer, written);
        written + prefix + suffix
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
                let format = NumberFormat::<{ FORMAT }> {};
                if !format.supports_writing_integers() {
                    core::panic!("{}", Error::Unsupported.description());
                } else if !format.is_valid() {
                    core::panic!("{}", format.error().description());
                }
                let len = unsigned::<$t, FORMAT>(self, bytes);
                &mut bytes[..len]
            }
        }
    )*)
}

to_lexical!("lexical_write_integer", 1234, u64);
to_lexical_with_options!("lexical_write_integer", 1234, u64, Options);
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
                let format = NumberFormat::<{ FORMAT }> {};
                if !format.supports_writing_integers() {
                    core::panic!("{}", Error::Unsupported.description());
                } else if !format.is_valid() {
                    core::panic!("{}", format.error().description());
                }
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
