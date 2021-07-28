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

use crate::sign::parse_sign_and_validate;
use lexical_util::assert::debug_assert_radix;
use lexical_util::digit::char_to_digit_const;
use lexical_util::format::NumberFormat;
use lexical_util::iterator::ByteIter;
use lexical_util::num::{as_cast, Integer};
use lexical_util::result::Result;
use lexical_util::step::u64_step;

/// Parse 8-digits at a time.
#[rustfmt::skip]
macro_rules! parse_8digits {
    (
        $value:ident,
        $iter:ident,
        $format:ident,
        $addsub:ident,
        $overflow:ident,
        $t:ident,
        $iter_type:ty
    ) => {
        let radix: $t = as_cast(NumberFormat::<{ $format }>::MANTISSA_RADIX);
        let radix2: $t = radix.wrapping_mul(radix);
        let radix4: $t = radix2.wrapping_mul(radix2);
        let radix8: $t = radix4.wrapping_mul(radix4);

        // Try our fast, 8-digit at a time optimizations.
        while let Some(val8) = try_parse_8digits::<$t, $iter_type, $format>(&mut $iter) {
            $value = match $value.checked_mul(radix8) {
                Some(v) => v,
                None => return into_error!($overflow, $iter),
            };
            $value = match $value.$addsub(val8) {
                Some(v) => v,
                None => return into_error!($overflow, $iter),
            };
        }
    };
}

/// Parse 4-digits at a time.
#[rustfmt::skip]
macro_rules! parse_4digits {
    (
        $value:ident,
        $iter:ident,
        $format:ident,
        $addsub:ident,
        $overflow:ident,
        $t:ident,
        $iter_type:ty
    ) => {
        let radix: $t = as_cast(NumberFormat::<{ $format }>::MANTISSA_RADIX);
        let radix2: $t = radix.wrapping_mul(radix);
        let radix4: $t = radix2.wrapping_mul(radix2);

        // Try our fast, 4-digit at a time optimizations.
        while let Some(val4) = try_parse_4digits::<$t, $iter_type, $format>(&mut $iter) {
            $value = match $value.checked_mul(radix4) {
                Some(v) => v,
                None => return into_error!($overflow, $iter),
            };
            $value = match $value.$addsub(val4) {
                Some(v) => v,
                None => return into_error!($overflow, $iter),
            };
        }
    };
}

/// Parse digits for a positive or negative value.
/// Optimized for operations with machine integers.
#[rustfmt::skip]
macro_rules! parse_digits {
    (
        $iter:ident,
        $format:ident,
        $addsub:ident,
        $overflow:ident,
        $t:ident,
        $iter_type:ty
    ) => {{
        let mut value = <$t>::ZERO;
        let radix = NumberFormat::<{ $format }>::MANTISSA_RADIX;

        // Optimizations for reading 8-digits at a time.
        // Makes no sense to do 8 digits at a time for 32-bit values,
        // since it can only hold 8 digits for base 10.
        if T::BITS >= 64 && radix <= 10 && <$iter_type>::IS_CONTIGUOUS {
            parse_8digits!(value, $iter, $format, $addsub, $overflow, $t, $iter_type);
        }

        // Optimizations for reading 4-digits at a time.
        // 36^4 is larger than a 16-bit integer. Likewise, 10^4 is almost
        // the limit of u16, so it's not worth it.
        if T::BITS >= 32 && radix <= 10 && <$iter_type>::IS_CONTIGUOUS {
            parse_4digits!(value, $iter, $format, $addsub, $overflow, $t, $iter_type);
        }

        // Do our slow parsing algorithm: 1 digit at a time.
        while let Some(&c) = $iter.next() {
            let digit = match char_to_digit_const(c, radix) {
                Some(v) => v,
                None => return Ok((value, $iter.cursor() - 1)),
            };
            value = match value.checked_mul(as_cast(radix)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter - 1),
            };
            value = match value.$addsub(as_cast(digit)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter - 1),
            };
        }

        Ok((value, $iter.cursor()))
    }};
}

// Add 64-bit temporary to the 128-bit value.
#[rustfmt::skip]
macro_rules! add_temp_128 {
    (
        $value:ident,
        $val64:ident,
        $iter:ident,
        $addsub:ident,
        $overflow:ident,
        $mul:ident,
        $t:ident
        $(- $shift:expr)?
    ) => {{
        if $value != <$t>::ZERO {
            $value = match $value.checked_mul(as_cast($mul)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter $(- $shift)?),
            };
        }
        $value = match $value.$addsub(as_cast($val64)) {
            Some(v) => v,
            None => return into_error!($overflow, $iter $(- $shift)?),
        };
    }};
}

/// Parse digits for a positive or negative value.
/// Uses a few optimizations to speed up operations on a non-native,
/// 128-bit type.
#[rustfmt::skip]
macro_rules! parse_digits_128 {
    (
        $iter:ident,
        $format:ident,
        $addsub:ident,
        $overflow:ident,
        $t:ident,
        $iter_type:ty
    ) => {{
        let mut value = <$t>::ZERO;
        let radix = NumberFormat::<{ $format }>::MANTISSA_RADIX;
        parse_8digits!(value, $iter, $format, $addsub, $overflow, $t, $iter_type);
        parse_4digits!(value, $iter, $format, $addsub, $overflow, $t, $iter_type);

        // After our fast-path optimizations, now try to parse 1 digit at a time.
        // We use temporary 64-bit values for better performance here.
        let step = u64_step(radix);
        while !$iter.is_consumed() {
            let mut val64: u64 = 0;
            let mut index = 0;
            while index < step {
                if let Some(&c) = $iter.next() {
                    index += 1;
                    let digit = match char_to_digit_const(c, radix) {
                        Some(v) => v,
                        None => {
                            // Add temporary to value and return early.
                            let mul = (radix as u64).pow(index as u32);
                            add_temp_128!(value, val64, $iter, $addsub, $overflow, mul, $t - 1);
                            return Ok((value, $iter.cursor() - 1));
                        },
                    };

                    // Don't have to worry about overflows.
                    val64 *= radix as u64;
                    val64 += digit as u64;
                } else {
                    break;
                }
            }

            // Add the temporary value to the total value.
            let mul = (radix as u64).pow(index as u32);
            add_temp_128!(value, val64, $iter, $addsub, $overflow, mul, $t);
        }

        Ok((value, $iter.cursor()))
    }};
}

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
///     https://johnnylee-sde.github.io/Fast-numeric-string-to-int/
#[inline]
pub fn try_parse_4digits<'a, T, Iter, const FORMAT: u128>(iter: &mut Iter) -> Option<T>
where
    T: Integer,
    Iter: ByteIter<'a>,
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
        unsafe {
            iter.step_by_unchecked(4);
        }
        Some(as_cast(parse_4digits::<FORMAT>(bytes)))
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
    Iter: ByteIter<'a>,
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
        unsafe {
            iter.step_by_unchecked(8);
        }
        Some(as_cast(parse_8digits::<FORMAT>(bytes)))
    } else {
        None
    }
}

/// Highly optimized algorithm to parse digits for machine integers.
///
/// If the type size is >= 64 bits, then use optimizations to parse up
/// to 8-digits at a time, since we can get at most 2 iterations.
/// If the type size is >= 32 bits, then use optimizations to parse up
/// to 4-digits at a time, since we can get at most 2 iterations.
///
/// These optimizations greatly reduce the number of multiplication
/// instructions we need to do.
///
/// Finally, once all the optimized reads are finished, then read 1 digit
/// at a time to construct our value.
///
/// Note that due to two's complement, we can't simple parse the value
/// using positive data, then negate it. Parsing `-128` for `i8` would lead
/// to numerical overflow, which would then incorrectly produce an invalid
/// value. Therefore, we must split the digit parsing into 2 discrete paths.
///
/// Returns a result containing the value and the number of digits parsed.
#[inline]
pub fn parse_digits<'a, T, Iter, const FORMAT: u128>(
    mut iter: Iter,
    is_negative: bool,
) -> Result<(T, usize)>
where
    T: Integer,
    Iter: ByteIter<'a>,
{
    if T::IS_SIGNED && is_negative {
        parse_digits!(iter, FORMAT, checked_sub, Underflow, T, Iter)
    } else {
        parse_digits!(iter, FORMAT, checked_add, Overflow, T, Iter)
    }
}

/// Highly optimized algorithm to parse digits for 128-bit integers.
/// 128-bit integers have slower native operations, so using native
/// operations when possible leads to faster performance.
///
/// Returns a result containing the value and the number of digits parsed.
#[inline]
pub fn parse_digits_128<'a, T, Iter, const FORMAT: u128>(
    mut iter: Iter,
    is_negative: bool,
) -> Result<(T, usize)>
where
    T: Integer,
    Iter: ByteIter<'a>,
{
    if T::IS_SIGNED && is_negative {
        parse_digits_128!(iter, FORMAT, checked_sub, Underflow, T, Iter)
    } else {
        parse_digits_128!(iter, FORMAT, checked_add, Overflow, T, Iter)
    }
}

/// Core parsing algorithm for machine integers.
/// See `parse_digits` for a detailed explanation of the algorithms.
///
/// Returns the parsed value and the number of digits processed.
#[inline]
pub fn algorithm<'a, T, Iter, const FORMAT: u128>(mut iter: Iter) -> Result<(T, usize)>
where
    T: Integer,
    Iter: ByteIter<'a>,
{
    debug_assert!(T::BITS < 128);
    debug_assert_radix(NumberFormat::<{ FORMAT }>::MANTISSA_RADIX);

    let is_negative = parse_sign_and_validate::<T, _, FORMAT>(&mut iter)?;
    parse_digits::<T, _, FORMAT>(iter, is_negative)
}

/// Optimized implementation of the above algorithm for 128-bit integers.
///
/// Returns the parsed value and the number of digits processed.
#[inline]
pub fn algorithm_128<'a, T, Iter, const FORMAT: u128>(mut iter: Iter) -> Result<(T, usize)>
where
    T: Integer,
    Iter: ByteIter<'a>,
{
    debug_assert!(T::BITS == 128);
    debug_assert_radix(NumberFormat::<{ FORMAT }>::MANTISSA_RADIX);

    let is_negative = parse_sign_and_validate::<T, _, FORMAT>(&mut iter)?;
    parse_digits_128::<T, _, FORMAT>(iter, is_negative)
}
