//! Implements the algorithm in terms of the lexical API.

/// Select the back-end.
use crate::decimal::Decimal;
#[cfg(feature = "power-of-two")]
use crate::generic::Generic;
use crate::options::Options;

use lexical_util::assert::{assert_buffer, assert_radix, debug_assert_buffer};
use lexical_util::num::{as_cast, SignedInteger, UnsignedInteger};
use lexical_util::{to_lexical, to_lexical_with_options};

#[cfg(not(feature = "power-of-two"))]
pub(crate) trait Itoa: Decimal + UnsignedInteger {}

#[cfg(feature = "power-of-two")]
pub(crate) trait Itoa: Decimal + Generic + UnsignedInteger {}

macro_rules! itoa_impl {
    ($($t:ty)*) => ($(
        impl Itoa for $t {}
    )*)
}

itoa_impl! { u8 u16 u32 u64 u128 usize }

/// Forward itoa arguments to an optimized backend.
/// Preconditions: `value` must be non-negative and unsigned.
///
/// # Safety
///
/// Safe as long as the buffer can hold `FORMATTED_SIZE_DECIMAL` elements.
#[inline]
#[cfg(not(feature = "power-of-two"))]
unsafe fn itoa_positive<T, U, const __: u32>(value: T, buffer: &mut [u8]) -> usize
where
    T: Itoa,
    U: Itoa,
{
    let value: U = as_cast(value);
    unsafe { value.decimal(buffer) }
}

/// Forward itoa arguments to an optimized backend.
/// Preconditions: `value` must be non-negative and unsigned.
///
/// # Safety
///
/// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
/// (or `FORMATTED_SIZE_DECIMAL` for decimal).
#[inline]
#[cfg(feature = "power-of-two")]
unsafe fn itoa_positive<T, U, const RADIX: u32>(value: T, buffer: &mut [u8]) -> usize
where
    T: Itoa,
    U: Itoa,
{
    let value: U = as_cast(value);
    if RADIX == 10 {
        unsafe { value.decimal(buffer) }
    } else {
        unsafe { value.generic::<RADIX>(buffer) }
    }
}

// UNSIGNED

/// Callback for unsigned integer formatter.
///
/// # Safety
///
/// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
/// (or `FORMATTED_SIZE_DECIMAL` for decimal).
#[inline]
unsafe fn unsigned<Narrow, Wide, const RADIX: u32>(value: Narrow, buffer: &mut [u8]) -> usize
where
    Narrow: Itoa,
    Wide: Itoa,
{
    unsafe { itoa_positive::<Narrow, Wide, RADIX>(value, buffer) }
}

// Implement ToLexical for numeric type.
macro_rules! unsigned_to_lexical {
    ($($narrow:tt $wide:tt $(, #[$meta:meta])? ; )*) => ($(
        impl ToLexical for $narrow {
            $(#[$meta:meta])?
            unsafe fn to_lexical_unchecked<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                debug_assert_buffer::<$narrow>(10, bytes.len());
                // SAFETY: safe if the buffer is sufficiently large.
                let len = unsafe { unsigned::<$narrow, $wide, 10>(self, bytes) };
                &mut bytes[..len]
            }

            $(#[$meta:meta])?
            fn to_lexical<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                assert_buffer::<$narrow>(10, bytes.len());
                // SAFETY: safe since the buffer is sufficiently large.
                unsafe { self.to_lexical_unchecked(bytes) }
            }
        }

        impl ToLexicalWithOptions for $narrow {
            type Options = Options;

            $(#[$meta:meta])?
            unsafe fn to_lexical_with_options_unchecked<'a, const RADIX: u32>(
                self,
                bytes: &'a mut [u8],
                _: &Self::Options,
            ) -> &'a mut [u8]
            {
                debug_assert_buffer::<$narrow>(RADIX, bytes.len());
                assert_radix::<RADIX>();
                // SAFETY: safe if the buffer is sufficiently large.
                let len = unsafe { unsigned::<$narrow, $wide, RADIX>(self, bytes) };
                &mut bytes[..len]
            }

            $(#[$meta:meta])?
            fn to_lexical_with_options<'a, const RADIX: u32>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8]
            {
                assert_buffer::<$narrow>(RADIX, bytes.len());
                assert_radix::<RADIX>();
                // SAFETY: safe since the buffer is sufficiently large.
                unsafe { self.to_lexical_with_options_unchecked::<RADIX>(bytes, options) }
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

// SIGNED

/// Callback for signed integer formatter.
///
/// # Safety
///
/// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
/// (or `FORMATTED_SIZE_DECIMAL` for decimal).
#[inline]
unsafe fn signed<Narrow, Wide, Unsigned, const RADIX: u32>(value: Narrow, mut buffer: &mut [u8]) -> usize
where
    Narrow: SignedInteger,
    Wide: SignedInteger,
    Unsigned: Itoa,
{
    if value < Narrow::ZERO {
        // Need to cast the value to the same size as unsigned type, since if
        // the value is **exactly** `Narrow::MIN`, and it it is then cast
        // as the wrapping negative as the unsigned value, a wider type
        // will have a very different value.
        let value: Wide = as_cast(value);
        let unsigned: Unsigned = as_cast(value.wrapping_neg());
        // SAFETY: safe as long as there is at least 1 element, which
        // the buffer should have at least `FORMATTED_SIZE` elements.
        unsafe {
            *buffer.get_unchecked_mut(0) = b'-';
            buffer = buffer.get_unchecked_mut(1..);
        }
        // SAFETY: safe as long as there is at least 1 element, which
        // the buffer should have at least `FORMATTED_SIZE` elements.
        unsafe { itoa_positive::<Unsigned, Unsigned, RADIX>(unsigned, buffer) + 1 }
    } else {
        let unsigned: Unsigned = as_cast(value);
        // SAFETY: safe as long as there is at least 1 element, which
        // the buffer should have at least `FORMATTED_SIZE` elements.
        unsafe { itoa_positive::<Unsigned, Unsigned, RADIX>(unsigned, buffer) }
    }
}

// Implement ToLexical for numeric type.
macro_rules! signed_to_lexical {
    ($($narrow:tt $wide:tt $unsigned:tt $(, #[$meta:meta])? ; )*) => ($(
        impl ToLexical for $narrow {
            $(#[$meta:meta])?
            unsafe fn to_lexical_unchecked<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                debug_assert_buffer::<$narrow>(10, bytes.len());
                // SAFETY: safe if the buffer is sufficiently large.
                let len = unsafe { signed::<$narrow, $wide, $unsigned, 10>(self, bytes) };
                &mut bytes[..len]
            }

            $(#[$meta:meta])?
            fn to_lexical<'a>(self, bytes: &'a mut [u8])
                -> &'a mut [u8]
            {
                assert_buffer::<$narrow>(10, bytes.len());
                // SAFETY: safe since the buffer is sufficiently large.
                unsafe { self.to_lexical_unchecked(bytes) }
            }
        }

        impl ToLexicalWithOptions for $narrow {
            type Options = Options;

            $(#[$meta:meta])?
            unsafe fn to_lexical_with_options_unchecked<'a, const RADIX: u32>(
                self,
                bytes: &'a mut [u8],
                _: &Self::Options,
            ) -> &'a mut [u8]
            {
                debug_assert_buffer::<$narrow>(RADIX, bytes.len());
                assert_radix::<RADIX>();
                // SAFETY: safe if the buffer is sufficiently large.
                let len = unsafe { signed::<$narrow, $wide, $unsigned, RADIX>(self, bytes) };
                &mut bytes[..len]
            }

            $(#[$meta:meta])?
            fn to_lexical_with_options<'a, const RADIX: u32>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8]
            {
                assert_buffer::<$narrow>(RADIX, bytes.len());
                assert_radix::<RADIX>();
                // SAFETY: safe since the buffer is sufficiently large.
                unsafe { self.to_lexical_with_options_unchecked::<RADIX>(bytes, options) }
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
