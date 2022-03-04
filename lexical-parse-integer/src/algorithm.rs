//! Radix-generic, optimized, string-to-integer conversion routines.
//!
//! These routines are highly optimized: they use various optimizations
//! to read multiple digits at-a-time with less multiplication instructions,
//! as well as other optimizations to avoid unnecessary compile-time branching.
//!
//! See [Algorithm.md](/docs/Algorithm.md) for a more detailed description of
//! the algorithm choice here. See [Benchmarks.md](/docs/Benchmarks.md) for
//! recent benchmark data.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use crate::shared::is_overflow;
use lexical_util::digit::char_to_digit_const;
use lexical_util::format::NumberFormat;
use lexical_util::iterator::{AsBytes, BytesIter};
use lexical_util::num::{as_cast, Integer, UnsignedInteger};
use lexical_util::result::Result;
use lexical_util::step::min_step;

// ALGORITHM

/// Check if we can try to parse 8 digits.
macro_rules! can_try_parse_multidigits {
    ($iter:expr, $radix:expr) => {
        $iter.is_contiguous() && (cfg!(not(feature = "power-of-two")) || $radix <= 10)
    };
}

/// Parse 8-digits at a time.
///
/// See the algorithm description in `parse_8digits`.
/// This reduces the number of required multiplications
/// from 8 to 4.
#[rustfmt::skip]
macro_rules! parse_8digits {
    (
        $value:ident,
        $iter:ident,
        $format:ident,
        $t:ident
    ) => {{
        let radix: $t = as_cast(NumberFormat::<{ $format }>::MANTISSA_RADIX);
        let radix2: $t = radix.wrapping_mul(radix);
        let radix4: $t = radix2.wrapping_mul(radix2);
        let radix8: $t = radix4.wrapping_mul(radix4);

        // Try our fast, 8-digit at a time optimizations.
        while let Some(val8) = try_parse_8digits::<$t, _, $format>(&mut $iter) {
            $value = $value.wrapping_mul(radix8);
            $value = $value.wrapping_add(val8);
        }
    }};
}

/// Parse 4-digits at a time.
///
/// See the algorithm description in `parse_4digits`.
/// This reduces the number of required multiplications
/// from 4 to 3.
#[rustfmt::skip]
macro_rules! parse_4digits {
    (
        $value:ident,
        $iter:ident,
        $format:ident,
        $t:ident
    ) => {{
        let radix: $t = as_cast(NumberFormat::<{ $format }>::MANTISSA_RADIX);
        let radix2: $t = radix.wrapping_mul(radix);
        let radix4: $t = radix2.wrapping_mul(radix2);

        // Try our fast, 4-digit at a time optimizations.
        while let Some(val4) = try_parse_4digits::<$t, _, $format>(&mut $iter) {
            $value = $value.wrapping_mul(radix4);
            $value = $value.wrapping_add(val4);
        }
    }};
}

/// Parse digits for a positive or negative value.
/// Optimized for operations with machine integers.
#[rustfmt::skip]
macro_rules! parse_digits {
    (
        $value:ident,
        $iter:ident,
        $format:ident,
        $is_negative:ident,
        $start_index:ident,
        $t:ident,
        $u:ident,
        $invalid_digit:ident
    ) => {{
        //  WARNING:
        //      Performance is heavily dependent on the amount of branching.
        //      We therefore optimize for worst cases only to a certain extent:
        //      that is, since most integers aren't randomly distributed, but
        //      are more likely to be smaller values, we need to avoid overbranching
        //      to ensure small digit parsing isn't impacted too much. We therefore
        //      only enable 4-digit **or** 8-digit optimizations, but not both.
        //      If not, the two branch passes kill performance for small 64-bit
        //      and 128-bit values.
        //
        //      However, for signed integers, the increased amount of branching
        //      makes these multi-digit optimizations not worthwhile. For large
        //      64-bit, signed integers, the performance benefit is ~23% faster.
        //      However, the performance penalty for smaller, more common integers
        //      is ~50%. Therefore, these optimizations are not worth the penalty.
        //
        //      For unsigned and 128-bit signed integers, the performance penalties
        //      are minimal and the performance gains are substantial, so re-enable
        //      the optimizations.
        //
        //  DO NOT MAKE CHANGES without monitoring the resulting benchmarks,
        //  or performance could greatly be impacted.
        let radix = NumberFormat::<{ $format }>::MANTISSA_RADIX;

        // Optimizations for reading 8-digits at a time.
        // Makes no sense to do 8 digits at a time for 32-bit values,
        // since it can only hold 8 digits for base 10.
        if <$t>::BITS == 128 && can_try_parse_multidigits!($iter, radix) {
            parse_8digits!($value, $iter, $format, $u);
        }
        if <$t>::BITS == 64 && can_try_parse_multidigits!($iter, radix) && !<$t>::IS_SIGNED {
            parse_8digits!($value, $iter, $format, $u);
        }

        // Optimizations for reading 4-digits at a time.
        // 36^4 is larger than a 16-bit integer. Likewise, 10^4 is almost
        // the limit of u16, so it's not worth it.
        if <$t>::BITS == 32 && can_try_parse_multidigits!($iter, radix) && !<$t>::IS_SIGNED {
            parse_4digits!($value, $iter, $format, $u);
        }

        parse_1digit!($value, $iter, $format, $is_negative, $start_index, $t, $u, $invalid_digit)
    }};
}

/// Algorithm for the complete parser.
#[inline]
pub fn algorithm_complete<T, Unsigned, const FORMAT: u128>(bytes: &[u8]) -> Result<T>
where
    T: Integer,
    Unsigned: UnsignedInteger,
{
    algorithm!(bytes, FORMAT, T, Unsigned, parse_digits, invalid_digit_complete, into_ok_complete)
}

/// Algorithm for the partial parser.
#[inline]
pub fn algorithm_partial<T, Unsigned, const FORMAT: u128>(bytes: &[u8]) -> Result<(T, usize)>
where
    T: Integer,
    Unsigned: UnsignedInteger,
{
    algorithm!(bytes, FORMAT, T, Unsigned, parse_digits, invalid_digit_partial, into_ok_partial)
}

// DIGIT OPTIMIZATIONS

/// Determine if 4 bytes, read raw from bytes, are 4 digits for the radix.
#[inline]
pub fn is_4digits<const FORMAT: u128>(v: u32) -> bool {
    let radix = NumberFormat::<{ FORMAT }>::MANTISSA_RADIX;
    debug_assert!(radix <= 10);

    // We want to have a wrapping add and sub such that only values from the
    // range `[0x30, 0x39]` (or narrower for custom radixes) will not
    // overflow into the high bit. This means that the value needs to overflow
    // into into `0x80` if the digit is 1 above, or `0x46` for the value `0x39`.
    // Likewise, we only valid for `[0x30, 0x38]` for radix 8, so we need
    // `0x47`.
    let add = 0x46 + 10 - radix;
    let add = add + (add << 8) + (add << 16) + (add << 24);
    // This aims to underflow if anything is below the min digit: if we have any
    // values under `0x30`, then this underflows and wraps into the high bit.
    let sub = 0x3030_3030;
    let a = v.wrapping_add(add);
    let b = v.wrapping_sub(sub);

    (a | b) & 0x8080_8080 == 0
}

/// Parse 4 bytes read from bytes into 4 digits.
#[inline]
pub fn parse_4digits<const FORMAT: u128>(mut v: u32) -> u32 {
    let radix = NumberFormat::<{ FORMAT }>::MANTISSA_RADIX;
    debug_assert!(radix <= 10);

    // Normalize our digits to the range `[0, 9]`.
    v -= 0x3030_3030;
    // Scale digits in 0 <= Nn <= 99.
    v = (v * radix) + (v >> 8);
    // Scale digits in 0 <= Nnnn <= 9999.
    v = ((v & 0x0000007f) * radix * radix) + ((v >> 16) & 0x0000007f);

    v
}

/// Use a fast-path optimization, where we attempt to parse 4 digits at a time.
/// This reduces the number of multiplications necessary to 2, instead of 4.
///
/// This approach is described in full here:
/// <https://johnnylee-sde.github.io/Fast-numeric-string-to-int/>
#[inline]
pub fn try_parse_4digits<'a, T, Iter, const FORMAT: u128>(iter: &mut Iter) -> Option<T>
where
    T: Integer,
    Iter: BytesIter<'a>,
{
    // Can't do fast optimizations with radixes larger than 10, since
    // we no longer have a contiguous ASCII block. Likewise, cannot
    // use non-contiguous iterators.
    debug_assert!(NumberFormat::<{ FORMAT }>::MANTISSA_RADIX <= 10);
    debug_assert!(Iter::IS_CONTIGUOUS);

    // Read our digits, validate the input, and check from there.
    let bytes = u32::from_le(iter.read::<u32>()?);
    if is_4digits::<FORMAT>(bytes) {
        // SAFETY: safe since we have at least 4 bytes in the buffer.
        unsafe { iter.step_by_unchecked(4) };
        Some(T::as_cast(parse_4digits::<FORMAT>(bytes)))
    } else {
        None
    }
}

/// Determine if 8 bytes, read raw from bytes, are 8 digits for the radix.
/// See `is_4digits` for the algorithm description.
#[inline]
pub fn is_8digits<const FORMAT: u128>(v: u64) -> bool {
    let radix = NumberFormat::<{ FORMAT }>::MANTISSA_RADIX;
    debug_assert!(radix <= 10);

    let add = 0x46 + 10 - radix;
    let add = add + (add << 8) + (add << 16) + (add << 24);
    let add = (add as u64) | ((add as u64) << 32);
    // This aims to underflow if anything is below the min digit: if we have any
    // values under `0x30`, then this underflows and wraps into the high bit.
    let sub = 0x3030_3030_3030_3030;
    let a = v.wrapping_add(add);
    let b = v.wrapping_sub(sub);

    (a | b) & 0x8080_8080_8080_8080 == 0
}

/// Parse 8 bytes read from bytes into 8 digits.
/// Credit for this goes to @aqrit, which further optimizes the
/// optimization described by Johnny Lee above.
#[inline]
pub fn parse_8digits<const FORMAT: u128>(mut v: u64) -> u64 {
    let radix = NumberFormat::<{ FORMAT }>::MANTISSA_RADIX as u64;
    debug_assert!(radix <= 10);

    // Create our masks. Assume the optimizer will do this at compile time.
    // It seems like an optimizing compiler **will** do this, so we
    // should be safe.
    let radix2 = radix * radix;
    let radix4 = radix2 * radix2;
    let radix6 = radix2 * radix4;
    let mask = 0x0000_00FF_0000_00FFu64;
    let mul1 = radix2 + (radix6 << 32);
    let mul2 = 1 + (radix4 << 32);

    // Normalize our digits to the base.
    v -= 0x3030_3030_3030_3030;
    // Scale digits in 0 <= Nn <= 99.
    v = (v * radix) + (v >> 8);
    let v1 = (v & mask).wrapping_mul(mul1);
    let v2 = ((v >> 16) & mask).wrapping_mul(mul2);

    ((v1.wrapping_add(v2) >> 32) as u32) as u64
}

/// Use a fast-path optimization, where we attempt to parse 8 digits at a time.
/// This reduces the number of multiplications necessary to 3, instead of 8.
#[inline]
pub fn try_parse_8digits<'a, T, Iter, const FORMAT: u128>(iter: &mut Iter) -> Option<T>
where
    T: Integer,
    Iter: BytesIter<'a>,
{
    // Can't do fast optimizations with radixes larger than 10, since
    // we no longer have a contiguous ASCII block. Likewise, cannot
    // use non-contiguous iterators.
    debug_assert!(NumberFormat::<{ FORMAT }>::MANTISSA_RADIX <= 10);
    debug_assert!(Iter::IS_CONTIGUOUS);

    // Read our digits, validate the input, and check from there.
    let bytes = u64::from_le(iter.read::<u64>()?);
    if is_8digits::<FORMAT>(bytes) {
        // SAFETY: safe since we have at least 8 bytes in the buffer.
        unsafe { iter.step_by_unchecked(8) };
        Some(T::as_cast(parse_8digits::<FORMAT>(bytes)))
    } else {
        None
    }
}
