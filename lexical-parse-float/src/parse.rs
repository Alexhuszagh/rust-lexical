//! Shared trait and methods for parsing floats.
//!
//! This is adapted from [fast-float-rust](https://github.com/aldanor/fast-float-rust),
//! a port of [fast_float](https://github.com/fastfloat/fast_float) to Rust.

#![doc(hidden)]

use crate::float::LemireFloat;
use crate::number::Number;
use crate::options::Options;
#[cfg(not(feature = "compact"))]
use lexical_parse_integer::algorithm;
use lexical_util::digit::char_to_digit_const;
use lexical_util::error::Error;
use lexical_util::format::NumberFormat;
use lexical_util::iterator::{AsBytes, Bytes, BytesIter};
use lexical_util::result::Result;
use lexical_util::step::u64_step;

// API
// ---

/// Parse integer trait, implemented in terms of the optimized back-end.
pub trait ParseFloat: LemireFloat {
    /// Forward complete parser parameters to the backend.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn parse_complete<const FORMAT: u128>(bytes: &[u8], options: &Options) -> Result<Self> {
        parse_complete::<Self, FORMAT>(bytes, options)
    }

    /// Forward complete parser parameters to the backend.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn parse_partial<const FORMAT: u128>(bytes: &[u8], options: &Options) -> Result<(Self, usize)> {
        parse_partial::<Self, FORMAT>(bytes, options)
    }
}

macro_rules! parse_float_impl {
    ($($t:ty)*) => ($(
        impl ParseFloat for $t {}
    )*)
}

parse_float_impl! { f32 f64 }

// PARSE
// -----

macro_rules! parse_mantissa_sign {
    ($byte:ident, $format:ident) => {{
        match $byte.integer_iter().peek() {
            Some(&b'+') if !$format.no_positive_mantissa_sign() => (false, 1),
            Some(&b'+') if $format.no_positive_mantissa_sign() => {
                return Err(Error::InvalidPositiveSign($byte.cursor()));
            },
            Some(&b'-') => (true, 1),
            Some(_) if $format.required_mantissa_sign() => {
                return Err(Error::MissingSign($byte.cursor()));
            },
            _ => (false, 0),
        }
    }};
}

macro_rules! parse_exponent_sign {
    ($byte:ident, $format:ident) => {{
        match $byte.integer_iter().peek() {
            Some(&b'+') if !$format.no_positive_exponent_sign() => (false, 1),
            Some(&b'+') if $format.no_positive_exponent_sign() => {
                return Err(Error::InvalidPositiveExponentSign($byte.cursor()));
            },
            Some(&b'-') => (true, 1),
            Some(_) if $format.required_mantissa_sign() => {
                return Err(Error::MissingExponentSign($byte.cursor()));
            },
            _ => (false, 0),
        }
    }};
}

/// Parse a float from bytes using a complete parser.
#[inline]
#[allow(unused)] // TODO(ahuszagh) Remove...
pub fn parse_complete<F: LemireFloat, const FORMAT: u128>(
    bytes: &[u8],
    options: &Options,
) -> Result<F> {
    let format = NumberFormat::<{ FORMAT }> {};
    let mut byte = bytes.bytes::<{ FORMAT }>();
    let (is_negative, shift) = parse_mantissa_sign!(byte, format);
    // SAFETY: safe since we shift at most one for a parsed sign byte.
    unsafe { byte.step_by_unchecked(shift) };
    if byte.integer_iter().is_consumed() {
        return Err(Error::Empty(byte.cursor()));
    }

    // Parse our a small representation of our number.
    // Can only do fast and moderate path optimizations if we
    // can shift significant digits to and from the exponent.
    if format.mantissa_radix() == format.exponent_base() {
        let num = match parse_number::<FORMAT>(byte.clone(), is_negative) {
            Ok(n) => n,
            Err(e) => {
                if let Some(value) = parse_special::<_, FORMAT>(byte.clone(), is_negative, options)
                {
                    return Ok(value);
                } else {
                    return Err(e);
                }
            },
        };
        if let Some(value) = num.try_fast_path::<_, FORMAT>() {
            return Ok(value);
        }
    }

    // TODO(ahuszagh) Need parse_number now...

    todo!()
}

/// Parse a float from bytes using a partial parser.
#[inline]
#[allow(unused)] // TODO(ahuszagh) Remove...
pub fn parse_partial<F: LemireFloat, const FORMAT: u128>(
    bytes: &[u8],
    options: &Options,
) -> Result<(F, usize)> {
    let format = NumberFormat::<{ FORMAT }> {};
    let mut byte = bytes.bytes::<{ FORMAT }>();
    let (is_negative, shift) = parse_mantissa_sign!(byte, format);
    // SAFETY: safe since we shift at most one for a parsed sign byte.
    unsafe { byte.step_by_unchecked(shift) };
    if byte.integer_iter().is_consumed() {
        return Err(Error::Empty(byte.cursor()));
    }

    // Parse our a small representation of our number.
    // Can only do fast and moderate path optimizations if we
    // can shift significant digits to and from the exponent.
    if format.mantissa_radix() == format.exponent_base() {
        let (num, count) = match parse_partial_number::<FORMAT>(byte.clone(), is_negative) {
            Ok(n) => n,
            Err(e) => {
                if let Some(value) =
                    parse_partial_special::<_, FORMAT>(byte.clone(), is_negative, options)
                {
                    return Ok(value);
                } else {
                    return Err(e);
                }
            },
        };
        if let Some(value) = num.try_fast_path::<_, FORMAT>() {
            return Ok((value, count));
        }
    }

    // TODO(ahuszagh) Need parse_number now...

    todo!()
}

// NUMBER
// ------

/// Parse a partial, non-special floating point number.
///
/// This creates a representation of the float as the
/// significant digits and the decimal exponent.
#[inline]
pub fn parse_partial_number<const FORMAT: u128>(
    mut byte: Bytes<FORMAT>,
    is_negative: bool,
) -> Result<(Number, usize)> {
    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    let decimal_point = format.decimal_point();
    let exponent_character = format.exponent();
    debug_assert!(format.is_valid());
    debug_assert!(!byte.is_done());

    // Parse our integral digits.
    let mut mantissa = 0_u64;
    let start = byte.clone();
    parse_digits::<_, _, FORMAT>(byte.integer_iter(), |digit| {
        mantissa = mantissa.wrapping_mul(format.radix() as _).wrapping_add(digit as _);
    });
    let mut n_digits = byte.cursor() - start.cursor();
    if cfg!(feature = "format") && format.required_integer_digits() && n_digits == 0 {
        return Err(Error::EmptyInteger(byte.cursor()));
    }

    // Handle decimal point and digits afterwards.
    let mut n_after_dot = 0;
    let mut exponent = 0_i64;
    let int_end = byte.cursor() as i64;
    if byte.first_is(decimal_point) {
        // SAFETY: s cannot be empty due to first_is
        unsafe { byte.step_unchecked() };
        let before = byte.cursor();
        #[cfg(not(feature = "compact"))]
        parse_8digits::<_, FORMAT>(byte.fraction_iter(), &mut mantissa);
        parse_digits::<_, _, FORMAT>(byte.fraction_iter(), |digit| {
            mantissa = mantissa.wrapping_mul(format.radix() as _).wrapping_add(digit as _);
        });
        n_after_dot = byte.cursor() - before;
        exponent = -(n_after_dot as i64);
        if cfg!(feature = "format") && format.required_fraction_digits() && n_after_dot == 0 {
            return Err(Error::EmptyFraction(byte.cursor()));
        }
    }

    n_digits += n_after_dot;
    if format.required_mantissa_digits() && n_digits == 0 {
        return Err(Error::EmptyMantissa(byte.cursor()));
    }

    // Handle scientific notation.
    let mut explicit_exponent = 0_i64;
    let is_exponent = if cfg!(feature = "format") && format.case_sensitive_exponent() {
        byte.first_is(exponent_character)
    } else {
        byte.case_insensitive_first_is(exponent_character)
    };
    if is_exponent {
        // SAFETY: byte cannot be empty due to first_is
        unsafe { byte.step_unchecked() };
        let (is_negative, shift) = parse_exponent_sign!(byte, format);
        let before = byte.cursor();
        // SAFETY: safe since we shift at most one for a parsed sign byte.
        unsafe { byte.step_by_unchecked(shift) };
        parse_digits::<_, _, FORMAT>(byte.exponent_iter(), |digit| {
            if explicit_exponent < 0x10000 {
                explicit_exponent *= format.radix() as i64;
                explicit_exponent += digit as i64;
            }
        });
        if format.required_exponent_digits() && byte.cursor() - before == 0 {
            return Err(Error::EmptyExponent(byte.cursor()));
        }
        // Handle our sign, and get the explicit part of the exponent.
        let explicit_exponent = if is_negative {
            -explicit_exponent
        } else {
            explicit_exponent
        };
        exponent += explicit_exponent;
    }

    // Get the number of parsed digits (total), and redo if we had overflow.
    let end = byte.cursor();
    let mut step = u64_step(format.radix());
    let mut many_digits = false;
    if cfg!(feature = "format") && !format.required_mantissa_digits() && n_digits == 0 {
        exponent = 0;
    }
    if n_digits <= step {
        return Ok((
            Number {
                exponent,
                mantissa,
                is_negative,
                many_digits: false,
            },
            end,
        ));
    }

    // Check for leading zeros, and to see if we had a false overflow.
    n_digits -= step;
    let mut zeros = start.clone();
    let mut zeros_integer = zeros.integer_iter();
    while zeros_integer.peek_is(b'0') {
        n_digits -= 1;
        // SAFETY: safe since zeros cannot be empty due to peek_is
        unsafe { zeros_integer.step_unchecked() };
    }
    if zeros.first_is(decimal_point) {
        // SAFETY: safe since zeros cannot be empty due to first_is
        unsafe { zeros.step_unchecked() };
    }
    let mut zeros_fraction = zeros.fraction_iter();
    while zeros_fraction.peek_is(b'0') {
        n_digits -= 1;
        // SAFETY: safe since zeros cannot be empty due to peek_is
        unsafe { zeros_fraction.step_unchecked() };
    }

    // Now, check if we explicitly overflowed.
    if n_digits > 0 {
        // Have more than 19 significant digits, so we overflowed.
        many_digits = true;
        mantissa = 0;
        let mut byte = start;
        parse_u64_digits::<_, FORMAT>(byte.integer_iter(), &mut mantissa, &mut step);
        exponent = if step == 0 {
            // Filled our mantissa with just the integer.
            byte.cursor() as i64 - int_end
        } else {
            // SAFETY: the next byte must be present and be '.'
            // We know this is true because we had more than 19
            // digits previously, so we overflowed a 64-bit integer,
            // but parsing only the integral digits produced less
            // than 19 digits. That means we must have a decimal
            // point, and at least 1 fractional digit.
            unsafe { byte.step_unchecked() };
            let before = byte.cursor() as i64;
            parse_u64_digits::<_, FORMAT>(byte.fraction_iter(), &mut mantissa, &mut step);
            before - byte.cursor() as i64
        };
        // Add back the explicit exponent.
        exponent += explicit_exponent;
    }

    Ok((
        Number {
            exponent,
            mantissa,
            is_negative,
            many_digits,
        },
        end,
    ))
}

/// Try to parse a non-special floating point number.
#[inline]
pub fn parse_number<const FORMAT: u128>(byte: Bytes<FORMAT>, is_negative: bool) -> Result<Number> {
    let length = byte.length();
    let (float, count) = parse_partial_number::<FORMAT>(byte, is_negative)?;
    if count == length {
        Ok(float)
    } else {
        Err(Error::InvalidDigit(count))
    }
}

// DIGITS
// ------

/// Iteratively parse and consume digits from bytes.
#[inline]
pub fn parse_digits<'a, Iter, Cb, const FORMAT: u128>(mut iter: Iter, mut cb: Cb)
where
    Iter: BytesIter<'a>,
    Cb: FnMut(u32),
{
    let format = NumberFormat::<{ FORMAT }> {};
    let radix = format.radix();
    while let Some(&c) = iter.peek() {
        match char_to_digit_const(c, radix) {
            Some(v) => cb(v),
            None => break,
        }
        // SAFETY: iter cannot be empty
        unsafe { iter.step_unchecked() };
    }
}

/// Iteratively parse and consume digits in intervals of 8.
#[inline]
#[cfg(not(feature = "compact"))]
pub fn parse_8digits<'a, Iter, const FORMAT: u128>(mut iter: Iter, mantissa: &mut u64)
where
    Iter: BytesIter<'a>,
{
    let format = NumberFormat::<{ FORMAT }> {};
    let radix: u64 = format.radix() as u64;
    if cfg!(not(feature = "radix")) || radix <= 10 {
        let radix2 = radix.wrapping_mul(radix);
        let radix4 = radix2.wrapping_mul(radix2);
        let radix8 = radix4.wrapping_mul(radix4);
        // Can do up to 2 iterations without overflowing.
        if let Some(v) = algorithm::try_parse_8digits::<u64, _, FORMAT>(&mut iter) {
            *mantissa = mantissa.wrapping_mul(radix8).wrapping_add(v);
            if let Some(v) = algorithm::try_parse_8digits::<u64, _, FORMAT>(&mut iter) {
                *mantissa = mantissa.wrapping_mul(radix8).wrapping_add(v);
            }
        }
    }
}

/// Iteratively parse and consume digits in intervals of 8.
#[inline]
pub fn parse_u64_digits<'a, Iter, const FORMAT: u128>(
    mut iter: Iter,
    mantissa: &mut u64,
    step: &mut usize,
) where
    Iter: BytesIter<'a>,
{
    let format = NumberFormat::<{ FORMAT }> {};
    let radix = format.radix();
    while *step > 0 {
        if let Some(&c) = iter.peek() {
            match char_to_digit_const(c, radix) {
                Some(digit) => {
                    *mantissa = *mantissa * radix as u64 + digit as u64;
                    *step -= 1;
                    // SAFETY: iter cannot be empty
                    unsafe { iter.step_unchecked() };
                },
                None => break,
            }
        } else {
            break;
        }
    }
}

// SPECIAL
// -------

/// Determine if the input data matches the special string.
/// If there's no match, returns 0. Otherwise, returns the byte's cursor.
#[inline]
fn is_special_eq<const FORMAT: u128>(mut byte: Bytes<FORMAT>, string: &'static [u8]) -> usize {
    let format = NumberFormat::<{ FORMAT }> {};
    if cfg!(feature = "format") && format.case_sensitive_special() {
        if starts_with(byte.special_iter(), string.iter()) {
            // Trim the iterator afterwards.
            byte.special_iter().peek();
            return byte.cursor();
        }
    } else if case_insensitive_starts_with(byte.special_iter(), string.iter()) {
        // Trim the iterator afterwards.
        byte.special_iter().peek();
        return byte.cursor();
    }
    0
}

/// Parse a positive representation of a special, non-finite float.
#[inline]
pub fn parse_positive_special<F, const FORMAT: u128>(
    byte: Bytes<FORMAT>,
    options: &Options,
) -> Option<(F, usize)>
where
    F: LemireFloat,
{
    let format = NumberFormat::<{ FORMAT }> {};
    if cfg!(feature = "format") && format.no_special() {
        return None;
    }

    let cursor = byte.cursor();
    let length = byte.length() - cursor;
    if length >= options.nan_string().len() {
        let count = is_special_eq::<FORMAT>(byte.clone(), options.nan_string());
        if count != 0 {
            return Some((F::NAN, count));
        }
    }
    if length >= options.inf_string().len() {
        let count = is_special_eq::<FORMAT>(byte.clone(), options.infinity_string());
        if count != 0 {
            return Some((F::INFINITY, count));
        }
        let count = is_special_eq::<FORMAT>(byte.clone(), options.inf_string());
        if count != 0 {
            return Some((F::INFINITY, count));
        }
    }

    None
}

/// Parse a partial representation of a special, non-finite float.
#[inline]
pub fn parse_partial_special<F, const FORMAT: u128>(
    byte: Bytes<FORMAT>,
    is_negative: bool,
    options: &Options,
) -> Option<(F, usize)>
where
    F: LemireFloat,
{
    let (mut float, count) = parse_positive_special::<F, FORMAT>(byte, options)?;
    if is_negative {
        float = -float;
    }
    Some((float, count))
}

/// Try to parse a special, non-finite float.
#[inline]
pub fn parse_special<F, const FORMAT: u128>(
    byte: Bytes<FORMAT>,
    is_negative: bool,
    options: &Options,
) -> Option<F>
where
    F: LemireFloat,
{
    let length = byte.length();
    if let Some((float, count)) = parse_partial_special::<F, FORMAT>(byte, is_negative, options) {
        if count == length {
            return Some(float);
        }
    }
    None
}

// STARTS WITH
// -----------

/// Check if left iter starts with right iter.
///
/// This optimizes decently well, to the following ASM for pure slices:
///
/// ```text
/// starts_with_slc:
///         xor     eax, eax
/// .LBB0_1:
///         cmp     rcx, rax
///         je      .LBB0_2
///         cmp     rsi, rax
///         je      .LBB0_5
///         movzx   r8d, byte ptr [rdi + rax]
///         lea     r9, [rax + 1]
///         cmp     r8b, byte ptr [rdx + rax]
///         mov     rax, r9
///         je      .LBB0_1
/// .LBB0_5:
///         xor     eax, eax
///         ret
/// .LBB0_2:
///         mov     al, 1
///         ret
/// ```
#[inline]
pub fn starts_with<'a, 'b, Iter1, Iter2>(mut x: Iter1, mut y: Iter2) -> bool
where
    Iter1: Iterator<Item = &'a u8>,
    Iter2: Iterator<Item = &'b u8>,
{
    loop {
        // Only call `next()` on x if y is not None, otherwise,
        // we may incorrectly consume an x character.
        let yi = y.next();
        if yi.is_none() {
            return true;
        } else if x.next() != yi {
            return false;
        }
    }
}

/// Check if left iter starts with right iter without case-sensitivity.
///
/// This optimizes decently well, to the following ASM for pure slices:
///
/// ```text
/// case_insensitive_starts_with_slc:
///         xor     eax, eax
/// .LBB1_1:
///         cmp     rcx, rax
///         je      .LBB1_2
///         cmp     rsi, rax
///         je      .LBB1_5
///         movzx   r8d, byte ptr [rdi + rax]
///         xor     r8b, byte ptr [rdx + rax]
///         add     rax, 1
///         test    r8b, -33
///         je      .LBB1_1
/// .LBB1_5:
///         xor     eax, eax
///         ret
/// .LBB1_2:
///         mov     al, 1
///         ret
/// ```
#[inline]
pub fn case_insensitive_starts_with<'a, 'b, Iter1, Iter2>(mut x: Iter1, mut y: Iter2) -> bool
where
    Iter1: Iterator<Item = &'a u8>,
    Iter2: Iterator<Item = &'b u8>,
{
    // We use a faster optimization here for ASCII letters, which NaN
    // and infinite strings **must** be. [A-Z] is 0x41-0x5A, while
    // [a-z] is 0x61-0x7A. Therefore, the xor must be 0 or 32 if they
    // are case-insensitive equal, but only if at least 1 of the inputs
    // is an ASCII letter.
    loop {
        let yi = y.next();
        if yi.is_none() {
            return true;
        }
        let yi = *yi.unwrap();
        let is_not_equal = x.next().map_or(true, |&xi| {
            let xor = xi ^ yi;
            xor != 0 && xor != 0x20
        });
        if is_not_equal {
            return false;
        }
    }
}
