//! Shared trait and methods for parsing floats.
//!
//! This is adapted from [fast-float-rust](https://github.com/aldanor/fast-float-rust),
//! a port of [fast_float](https://github.com/fastfloat/fast_float) to Rust.

#![doc(hidden)]

use crate::float::LemireFloat;
use crate::number::Number;
use crate::options::Options;
use lexical_util::digit::AsDigits;
use lexical_util::error::Error;
use lexical_util::format::NumberFormat;
use lexical_util::iterator::{Byte, ByteIter};
use lexical_util::result::Result;

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

macro_rules! parse_sign {
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

/// Parse a float from bytes using a complete parser.
#[inline]
#[allow(unused)] // TODO(ahuszagh) Remove...
pub fn parse_complete<F: LemireFloat, const FORMAT: u128>(
    bytes: &[u8],
    options: &Options,
) -> Result<F> {
    let format = NumberFormat::<{ FORMAT }> {};
    let mut byte = bytes.digits::<{ FORMAT }>();
    let (is_negative, shift) = parse_sign!(byte, format);
    // SAFETY: safe since we shift at most one for a parsed sign byte.
    unsafe { byte.step_by_unchecked(shift) };
    if byte.integer_iter().is_consumed() {
        return Err(Error::Empty(byte.cursor()));
    }

    // Parse our a small representation of our number.
    let num = match parse_number::<_, FORMAT>(byte.clone(), is_negative) {
        Some(r) => r,
        None => {
            if let Some(value) = parse_special::<_, _, FORMAT>(byte.clone(), is_negative, options) {
                return Ok(value);
            } else {
                return Err(Error::InvalidDigit(byte.cursor()));
            }
        },
    };
    if let Some(value) = num.try_fast_path::<_, FORMAT>() {
        return Ok(value);
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
    let mut byte = bytes.digits::<{ FORMAT }>();
    let (is_negative, shift) = parse_sign!(byte, format);
    // SAFETY: safe since we shift at most one for a parsed sign byte.
    unsafe { byte.step_by_unchecked(shift) };
    if byte.integer_iter().is_consumed() {
        return Err(Error::Empty(byte.cursor()));
    }

    // Parse our a small representation of our number.
    let (num, count) = match parse_partial_number::<_, FORMAT>(byte.clone(), is_negative) {
        Some(r) => r,
        None => {
            if let Some(value) =
                parse_partial_special::<_, _, FORMAT>(byte.clone(), is_negative, options)
            {
                return Ok(value);
            } else {
                return Err(Error::InvalidDigit(byte.cursor()));
            }
        },
    };
    if let Some(value) = num.try_fast_path::<_, FORMAT>() {
        return Ok((value, count));
    }

    // TODO(ahuszagh) Need parse_number now...

    todo!()
}

/// Parse a partial, non-special floating point number.
///
/// This creates a representation of the float as the
/// significant digits and the decimal exponent.
#[inline]
#[allow(unused)] // TODO(ahuszagh) Remove...
pub fn parse_partial_number<'a: 'b, 'b, Bytes, const FORMAT: u128>(
    byte: Bytes,
    is_negative: bool,
) -> Option<(Number, usize)>
where
    Bytes: Byte<'a, 'b>,
{
    todo!();
}

/// Try to parse a non-special floating point number.
#[inline]
pub fn parse_number<'a: 'b, 'b, Bytes, const FORMAT: u128>(
    byte: Bytes,
    is_negative: bool,
) -> Option<Number>
where
    Bytes: Byte<'a, 'b>,
{
    let cursor = byte.cursor();
    let length = byte.length();
    if let Some((float, count)) = parse_partial_number::<_, FORMAT>(byte, is_negative) {
        if count == length - cursor {
            return Some(float);
        }
    }
    None
}

// TODO(ahuszagh) We know the left one is longer, so...

/// Determine if the input data matches the special string.
/// If there's no match, returns 0. Otherwise, returns the byte's cursor.
#[inline]
#[allow(unused)] // TODO(ahuszagh) Remove...
fn is_special_eq<'a: 'b, 'b, Bytes, const FORMAT: u128>(mut byte: Bytes, string: &'static [u8]) -> usize
where
    Bytes: Byte<'a, 'b>,
{
    let format = NumberFormat::<{ FORMAT }> {};
    // TODO(ahuszagh) This fails due to lifetime issues.
    //  This is of course, fucking hell.
//    let mut x = byte.special_iter();
//    let mut y = string.iter();
//    if cfg!(feature = "format") && format.case_sensitive_special() {
//        // TODO(ahuszagh) I do need to remember to trim the bytes afterwards.
//        if starts_with(x, y) {
//            todo!();
//        }
//    } else {
//        // TODO(ahuszagh) I do need to remember to trim the bytes afterwards.
//        if case_insensitive_starts_with(x, y) {
//            todo!();
//        }
//    }
    0
}

/// Parse a positive representation of a special, non-finite float.
#[inline]
pub fn parse_positive_special<'a: 'b, 'b, F, Bytes, const FORMAT: u128>(
    byte: Bytes,
    options: &Options,
) -> Option<(F, usize)>
where
    F: LemireFloat,
    Bytes: Byte<'a, 'b>,
{
    let format = NumberFormat::<{ FORMAT }> {};
    if cfg!(feature = "format") && format.no_special() {
        return None;
    }

    let cursor = byte.cursor();
    let length = byte.length() - cursor;
    if length >= options.nan_string().len() {
        let count = is_special_eq::<_, FORMAT>(byte.clone(), options.nan_string());
        if count != 0 {
            return Some((F::NAN, count));
        }
    }
    if length >= options.inf_string().len() {
        let count = is_special_eq::<_, FORMAT>(byte.clone(), options.infinity_string());
        if count != 0 {
            return Some((F::INFINITY, count));
        }
        let count = is_special_eq::<_, FORMAT>(byte.clone(), options.inf_string());
        if count != 0 {
            return Some((F::INFINITY, count));
        }
    }

    None
}

/// Parse a partial representation of a special, non-finite float.
#[inline]
pub fn parse_partial_special<'a: 'b, 'b, F, Bytes, const FORMAT: u128>(
    byte: Bytes,
    is_negative: bool,
    options: &Options,
) -> Option<(F, usize)>
where
    F: LemireFloat,
    Bytes: Byte<'a, 'b>,
{
    let (mut float, count) = parse_positive_special::<F, _, FORMAT>(byte, options)?;
    if is_negative {
        float = -float;
    }
    Some((float, count))
}

/// Try to parse a special, non-finite float.
#[inline]
pub fn parse_special<'a: 'b, 'b, F, Bytes, const FORMAT: u128>(
    byte: Bytes,
    is_negative: bool,
    options: &Options,
) -> Option<F>
where
    F: LemireFloat,
    Bytes: Byte<'a, 'b>,
{
    let cursor = byte.cursor();
    let length = byte.length();
    if let Some((float, count)) = parse_partial_special::<F, _, FORMAT>(byte, is_negative, options)
    {
        if count == length - cursor {
            return Some(float);
        }
    }
    None
}

// HELPERS
// -------

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
        if x.next().map_or(true, |&xi| {
            let xor = xi ^ yi;
            xor != 0 && xor != 0x20
        }) {
            return false;
        }
    }
}
