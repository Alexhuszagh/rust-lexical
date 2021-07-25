//! The algorithm definitions for the the string-to-integer conversions.

use lexical_util::assert::debug_assert_radix;
use lexical_util::div128::u64_step;
use lexical_util::error::ParseErrorCode;
use lexical_util::iterator::Iterator;
use lexical_util::num::{as_cast, Integer};
use lexical_util::result::ParseResult;

/// Simple short-circuit to an error.
macro_rules! into_error {
    ($code:ident, $iter:ident $(,$shift:expr)?) => {
        Err((ParseErrorCode::$code, $iter.slice_len() $(+ $shift)?).into())
    };
}

/// Convert a character to a digit.
/// This optimizes for cases where radix is <= 10, and uses a decent,
/// match-based fallback algorithm.
#[inline]
pub fn char_to_digit<const RADIX: u32>(c: u8) -> Option<u32> {
    let digit = if RADIX <= 10 {
        // Optimize for small radixes.
        (c.wrapping_sub(b'0')) as u32
    } else {
        // Fallback, still decently fast.
        let digit = match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'Z' => c - b'A' + 10,
            b'a'..=b'z' => c - b'a' + 10,
            _ => 0xFF,
        };
        digit as u32
    };
    if digit < RADIX {
        Some(digit)
    } else {
        None
    }
}

/// Parse 8-digits at a time.
macro_rules! parse_8digits {
    (
        $value:ident,
        $iter:ident,
        $radix:ident,
        $addsub:ident,
        $overflow:ident,
        $t:ty,
        $iter_type:ty
    ) => {
        let radix: $t = as_cast($radix);
        let radix2: $t = radix.wrapping_mul(radix);
        let radix4: $t = radix2.wrapping_mul(radix2);
        let radix8: $t = radix4.wrapping_mul(radix4);

        // Try our fast, 8-digit at a time optimizations.
        while let Some(val8) = try_parse_8digits::<$t, $iter_type, $radix>(&mut $iter) {
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
macro_rules! parse_4digits {
    (
        $value:ident,
        $iter:ident,
        $radix:ident,
        $addsub:ident,
        $overflow:ident,
        $t:ty,
        $iter_type:ty
    ) => {
        let radix: $t = as_cast($radix);
        let radix2: $t = radix.wrapping_mul(radix);
        let radix4: $t = radix2.wrapping_mul(radix2);

        // Try our fast, 4-digit at a time optimizations.
        while let Some(val4) = try_parse_4digits::<$t, $iter_type, $radix>(&mut $iter) {
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
macro_rules! parse_digits {
    ($iter:ident, $radix:ident, $addsub:ident, $overflow:ident, $t:ty, $iter_type:ty) => {{
        let mut value = <$t>::ZERO;

        // Optimizations for reading 8-digits at a time.
        // Makes no sense to do 8 digits at a time for 32-bit values,
        // since it can only hold 8 digits for base 10.
        if T::BITS >= 64 && $radix <= 10 && <$iter_type>::IS_CONTIGUOUS {
            parse_8digits!(value, $iter, $radix, $addsub, $overflow, $t, $iter_type);
        }

        // Optimizations for reading 4-digits at a time.
        // 36^4 is larger than a 16-bit integer. Likewise, 10^4 is almost
        // the limit of u16, so it's not worth it.
        if T::BITS >= 32 && $radix <= 10 && <$iter_type>::IS_CONTIGUOUS {
            parse_4digits!(value, $iter, $radix, $addsub, $overflow, $t, $iter_type);
        }

        // Do our slow parsing algorithm: 1 digit at a time.
        while let Some(&c) = $iter.next() {
            let digit = match char_to_digit::<$radix>(c) {
                Some(v) => v,
                None => return Ok((value, $iter.slice_len())),
            };
            value = match value.checked_mul(as_cast($radix)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter, 1),
            };
            value = match value.$addsub(as_cast(digit)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter, 1),
            };
        }

        // Iterator must be empty.
        Ok((value, 0))
    }};
}

// Add 64-bit temporary to the 128-bit value.
macro_rules! add_temporary_128 {
    (
        $value:ident, $val64:ident, $iter:ident, $addsub:ident, $overflow:ident, $mul:ident, $t:ty
    ) => {{
        if $value != <$t>::ZERO {
            $value = match $value.checked_mul(as_cast($mul)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter),
            };
        }
        $value = match $value.$addsub(as_cast($val64)) {
            Some(v) => v,
            None => return into_error!($overflow, $iter),
        };
    }};
}

/// Parse digits for a positive or negative value.
/// Uses a few optimizations to speed up operations on a non-native,
/// 128-bit type.
macro_rules! parse_digits_128 {
    ($iter:ident, $radix:ident, $addsub:ident, $overflow:ident, $t:ty, $iter_type:ty) => {{
        let mut value = <$t>::ZERO;
        parse_8digits!(value, $iter, $radix, $addsub, $overflow, $t, $iter_type);
        parse_4digits!(value, $iter, $radix, $addsub, $overflow, $t, $iter_type);

        // After our fast-path optimizations, now try to parse 1 digit at a time.
        // We use temporary 64-bit values for better performance here.
        let step = u64_step::<$radix>();
        while !$iter.is_consumed() {
            let mut val64: u64 = 0;
            let mut index = 0;
            while index < step {
                if let Some(&c) = $iter.next() {
                    index += 1;
                    let digit = match char_to_digit::<$radix>(c) {
                        Some(v) => v,
                        None => {
                            // Add temporary to value and return early.
                            let mul = ($radix as u64).pow(index as u32);
                            add_temporary_128!(value, val64, $iter, $addsub, $overflow, mul, $t);
                            return Ok((value, $iter.slice_len()));
                        },
                    };

                    // Don't have to worry about overflows.
                    val64 *= $radix as u64;
                    val64 += digit as u64;
                } else {
                    break;
                }
            }

            // Add the temporary value to the total value.
            let mul = ($radix as u64).pow(index as u32);
            add_temporary_128!(value, val64, $iter, $addsub, $overflow, mul, $t);
        }

        // Iterator must be empty.
        Ok((value, 0))
    }};
}

/// Determine if 4 bytes, read raw from bytes, are 4 digits for the radix.
#[inline]
pub fn is_4digits<const RADIX: u32>(v: u32) -> bool {
    debug_assert!(RADIX <= 10);

    // We want to have a wrapping add and sub such that only values from the
    // range `[0x30, 0x39]` (or narrower for custom radixes) will not
    // overflow into the high bit. This means that the value needs to overflow
    // into into `0x80` if the digit is 1 above, or `0x46` for the value `0x39`.
    // Likewise, we only valid for `[0x30, 0x38]` for radix 8, so we need
    // `0x47`.
    let add = 0x46 + 10 - RADIX;
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
pub fn parse_4digits<const RADIX: u32>(mut v: u32) -> u32 {
    debug_assert!(RADIX <= 10);

    // Normalize our digits to the range `[0, 9]`.
    v -= 0x3030_3030;
    // Scale digits in 0 <= Nn <= 99.
    v = (v * RADIX) + (v >> 8);
    // Scale digits in 0 <= Nnnn <= 9999.
    v = ((v & 0x0000007f) * RADIX * RADIX) + ((v >> 16) & 0x0000007f);

    v
}

/// Use a fast-path optimization, where we attempt to parse 4 digits at a time.
/// This reduces the number of multiplications necessary to 2, instead of 4.
///
/// This approach is described in full here:
///     https://johnnylee-sde.github.io/Fast-numeric-string-to-int/
#[inline]
pub fn try_parse_4digits<'a, T, Iter, const RADIX: u32>(iter: &mut Iter) -> Option<T>
where
    T: Integer,
    Iter: Iterator<'a, u8>,
{
    // Can't do fast optimizations with radixes larger than 10, since
    // we no longer have a contiguous ASCII block. Likewise, cannot
    // use non-contiguous iterators.
    debug_assert!(RADIX <= 10);
    debug_assert!(Iter::IS_CONTIGUOUS);

    // Read our digits, validate the input, and check from there.
    let bytes = u32::from_le(iter.read::<u32>()?);
    if is_4digits::<RADIX>(bytes) {
        // SAFETY: safe since we have at least 4 bytes in the buffer.
        unsafe {
            iter.step_by_unchecked(4);
        }
        Some(as_cast(parse_4digits::<RADIX>(bytes)))
    } else {
        None
    }
}

/// Determine if 8 bytes, read raw from bytes, are 8 digits for the radix.
/// See `is_4digits` for the algorithm description.
#[inline]
pub fn is_8digits<const RADIX: u32>(v: u64) -> bool {
    debug_assert!(RADIX <= 10);

    let add = 0x46 + 10 - RADIX;
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
pub fn parse_8digits<const RADIX: u32>(mut v: u64) -> u64 {
    debug_assert!(RADIX <= 10);

    // Create our masks. Assume the optimizer will do this at compile time.
    // It seems like an optimizing compiler **will** do this, so we
    // should be safe.
    let radix = RADIX as u64;
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
pub fn try_parse_8digits<'a, T, Iter, const RADIX: u32>(iter: &mut Iter) -> Option<T>
where
    T: Integer,
    Iter: Iterator<'a, u8>,
{
    // Can't do fast optimizations with radixes larger than 10, since
    // we no longer have a contiguous ASCII block. Likewise, cannot
    // use non-contiguous iterators.
    debug_assert!(RADIX <= 10);
    debug_assert!(Iter::IS_CONTIGUOUS);

    // Read our digits, validate the input, and check from there.
    let bytes = u64::from_le(iter.read::<u64>()?);
    if is_8digits::<RADIX>(bytes) {
        // SAFETY: safe since we have at least 8 bytes in the buffer.
        unsafe {
            iter.step_by_unchecked(8);
        }
        Some(as_cast(parse_8digits::<RADIX>(bytes)))
    } else {
        None
    }
}

/// Highly optimized algorithm to parse digits for machine floats.
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
#[inline]
pub fn parse_digits<'a, T, Iter, const RADIX: u32>(
    mut iter: Iter,
    is_negative: bool,
) -> ParseResult<(T, usize)>
where
    T: Integer,
    Iter: Iterator<'a, u8>,
{
    if T::IS_SIGNED && is_negative {
        parse_digits!(iter, RADIX, checked_sub, Underflow, T, Iter)
    } else {
        parse_digits!(iter, RADIX, checked_add, Overflow, T, Iter)
    }
}

/// Highly optimized algorithm to parse digits for 128-bit integers.
/// 128-bit integers have slower native operations, so using native
/// operations when possible leads to faster performance.
#[inline]
pub fn parse_digits_128<'a, T, Iter, const RADIX: u32>(
    mut iter: Iter,
    is_negative: bool,
) -> ParseResult<(T, usize)>
where
    T: Integer,
    Iter: Iterator<'a, u8>,
{
    if T::IS_SIGNED && is_negative {
        parse_digits_128!(iter, RADIX, checked_sub, Underflow, T, Iter)
    } else {
        parse_digits_128!(iter, RADIX, checked_add, Overflow, T, Iter)
    }
}

// TODO(ahuszagh) Remove this, just for the format logic right now.
#[inline]
const fn positive_sign_allowed<const FORMAT: u128>() -> bool {
    true
}

// TODO(ahuszagh) Remove this, just for the format logic right now.
#[inline]
const fn required_sign<const FORMAT: u128>() -> bool {
    false
}

// TODO(ahuszagh) Remove this, just for the format logic right now.
#[inline]
const fn leading_zeros_allowed<const FORMAT: u128>() -> bool {
    true
}

/// Determines if the integer is negative and validates the input data.
///
/// This routine does the following:
///
/// 1. Parses the sign digit.
/// 2. Handles if positive signs before integers are not allowed.
/// 3. Handles negative signs if the type is unsigned.
/// 4. Handles if the sign is required, but missing.
/// 5. Handles if the iterator is empty, before or after parsing the sign.
/// 6. Handles if the iterator has invalid, leading zeros.
#[inline]
fn parse_sign_and_validate<'a, T, Iter, const RADIX: u32, const FORMAT: u128>(
    iter: &mut Iter,
) -> ParseResult<bool>
where
    T: Integer,
    Iter: Iterator<'a, u8>,
{
    let is_negative = match iter.peek() {
        Some(&b'+') if positive_sign_allowed::<FORMAT>() => {
            iter.next();
            false
        },
        Some(&b'-') if T::IS_SIGNED => {
            iter.next();
            true
        },
        Some(&b'+') => return into_error!(InvalidPositiveSign, iter),
        Some(&b'-') => return into_error!(InvalidNegativeSign, iter),
        Some(_) if !required_sign::<FORMAT>() => false,
        Some(_) => return into_error!(MissingSign, iter),
        None => return into_error!(Empty, iter),
    };
    // Note: need to call as a trait function.
    //  The standard library may add an `is_empty` function for iterators.
    if Iterator::is_empty(iter) {
        return into_error!(Empty, iter);
    }
    if !leading_zeros_allowed::<FORMAT>() && iter.peek() == Some(&b'0') {
        return into_error!(InvalidLeadingZeros, iter);
    }
    Ok(is_negative)
}

/// Core parsing algorithm for machine integers.
/// See `parse_digits` for a detailed explanation of the algorithms.
#[inline]
pub fn algorithm<T, const RADIX: u32, const FORMAT: u128>(bytes: &[u8]) -> ParseResult<(T, usize)>
where
    T: Integer,
{
    debug_assert_radix(RADIX);

    // TODO(ahuszagh) Going to need a proper iterator...
    let mut iter = bytes.iter();
    let is_negative = parse_sign_and_validate::<T, _, RADIX, FORMAT>(&mut iter)?;
    parse_digits::<T, _, RADIX>(iter, is_negative)
}

/// Optimized implementation of the above algorithm for 128-bit integers.
#[inline]
pub fn algorithm_128<T, const RADIX: u32, const FORMAT: u128>(
    bytes: &[u8],
) -> ParseResult<(T, usize)>
where
    T: Integer,
{
    debug_assert!(T::BITS == 128);

    // TODO(ahuszagh) Going to need a proper iterator...
    let mut iter = bytes.iter();
    let is_negative = parse_sign_and_validate::<T, _, RADIX, FORMAT>(&mut iter)?;
    parse_digits_128::<T, _, RADIX>(iter, is_negative)
}
