//! Implements the algorithm in terms of the lexical API.

/// Select the back-end.
use super::decimal::Decimal;
#[cfg(feature = "power_of_two")]
use super::generic::Generic;

use lexical_util::num::{as_cast, SignedInteger, UnsignedInteger};
use lexical_util::{to_lexical_impl, to_lexical_trait};

#[cfg(not(feature = "power_of_two"))]
pub(crate) trait Itoa: Decimal + UnsignedInteger {}

#[cfg(feature = "power_of_two")]
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
#[cfg(not(feature = "power_of_two"))]
unsafe fn itoa_positive<T, U>(value: T, _: u32, buffer: &mut [u8]) -> usize
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
#[cfg(feature = "power_of_two")]
unsafe fn itoa_positive<T, U>(value: T, radix: u32, buffer: &mut [u8]) -> usize
where
    T: Itoa,
    U: Itoa,
{
    let value: U = as_cast(value);
    if radix == 10 {
        unsafe { value.decimal(buffer) }
    } else {
        unsafe { value.generic(radix, buffer) }
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
unsafe fn unsigned<Narrow, Wide>(value: Narrow, radix: u32, buffer: &mut [u8]) -> usize
where
    Narrow: Itoa,
    Wide: Itoa,
{
    unsafe { itoa_positive::<Narrow, Wide>(value, radix, buffer) }
}

macro_rules! unsigned_to_lexical {
    ($($narrow:tt $wide:tt ;)*) => ($(
        to_lexical_impl!(unsigned::<$narrow, $wide>, $narrow);
    )*);
}

to_lexical_trait! {}
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
unsafe fn signed<Narrow, Wide, Unsigned>(value: Narrow, radix: u32, mut buffer: &mut [u8]) -> usize
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
        unsafe { itoa_positive::<Unsigned, Unsigned>(unsigned, radix, buffer) + 1 }
    } else {
        let unsigned: Unsigned = as_cast(value);
        // SAFETY: safe as long as there is at least 1 element, which
        // the buffer should have at least `FORMATTED_SIZE` elements.
        unsafe { itoa_positive::<Unsigned, Unsigned>(unsigned, radix, buffer) }
    }
}

macro_rules! signed_to_lexical {
    ($($narrow:tt $wide:tt $unsigned:tt ;)*) => ($(
        to_lexical_impl!(signed::<$narrow, $wide, $unsigned>, $narrow);
    )*);
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
