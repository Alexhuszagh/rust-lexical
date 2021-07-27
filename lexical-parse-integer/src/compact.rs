//! Radix-generic, unoptimized, string-to-integer conversion routines.
//!
//! These routines aim to be compact, at the cost of performance.

#![cfg(feature = "compact")]

use lexical_util::assert::debug_assert_radix;
use lexical_util::digit::char_to_digit;
use lexical_util::error::ParseErrorCode;
use lexical_util::iterator::ByteIter;
use lexical_util::num::{as_cast, Integer};
use lexical_util::result::ParseResult;

/// Simple short-circuit to an error.
macro_rules! into_error {
    ($code:ident, $iter:ident $(- $shift:expr)?) => {
        Err((ParseErrorCode::$code, $iter.cursor() $(- $shift)?).into())
    };
}

/// Parse digits for a positive or negative value.
/// Optimized for operations with machine integers.
macro_rules! parse_digits {
    ($iter:ident, $radix:ident, $addsub:ident, $overflow:ident, $t:ident) => {{
        let mut value = <$t>::ZERO;

        // Do our slow parsing algorithm: 1 digit at a time.
        while let Some(&c) = $iter.next() {
            let digit = match char_to_digit(c, $radix) {
                Some(v) => v,
                None => return Ok((value, $iter.cursor() - 1)),
            };
            value = match value.checked_mul(as_cast($radix)) {
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

/// Unoptimized algorithm to parse digits 1 at a time.
///
/// Returns a result containing the value and the number of digits parsed.
pub fn parse_digits<'a, T, Iter>(
    mut iter: Iter,
    radix: u32,
    is_negative: bool,
) -> ParseResult<(T, usize)>
where
    T: Integer,
    Iter: ByteIter<'a>,
{
    if T::IS_SIGNED && is_negative {
        parse_digits!(iter, radix, checked_sub, Underflow, T)
    } else {
        parse_digits!(iter, radix, checked_add, Overflow, T)
    }
}

// TODO(ahuszagh) Remove this, just for the format logic right now.
#[inline]
const fn positive_sign_allowed(_: u128) -> bool {
    true
}

// TODO(ahuszagh) Remove this, just for the format logic right now.
#[inline]
const fn required_sign(_: u128) -> bool {
    false
}

// TODO(ahuszagh) Remove this, just for the format logic right now.
#[inline]
const fn leading_zeros_allowed(_: u128) -> bool {
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
///
/// Returns if the value is negative, or any values detected when
/// validating the input.
#[inline]
fn parse_sign_and_validate<'a, Iter>(
    iter: &mut Iter,
    format: u128,
    is_signed: bool,
) -> ParseResult<bool>
where
    Iter: ByteIter<'a>,
{
    let is_negative = match iter.peek() {
        Some(&b'+') if positive_sign_allowed(format) => {
            iter.next();
            false
        },
        Some(&b'-') if is_signed => {
            iter.next();
            true
        },
        Some(&b'+') => return into_error!(InvalidPositiveSign, iter),
        Some(&b'-') => return into_error!(InvalidNegativeSign, iter),
        Some(_) if !required_sign(format) => false,
        Some(_) => return into_error!(MissingSign, iter),
        None => return into_error!(Empty, iter),
    };
    // Note: need to call as a trait function.
    //  The standard library may add an `is_empty` function for iterators.
    if ByteIter::is_empty(iter) {
        return into_error!(Empty, iter);
    }
    if !leading_zeros_allowed(format) && iter.peek() == Some(&b'0') {
        return into_error!(InvalidLeadingZeros, iter);
    }
    Ok(is_negative)
}

/// Core parsing algorithm.
/// See `parse_digits` for a detailed explanation of the algorithms.
///
/// Returns the parsed value and the number of digits processed.
#[inline]
pub fn algorithm<'a, T, Iter, const RADIX: u32, const FORMAT: u128>(
    mut iter: Iter,
) -> ParseResult<(T, usize)>
where
    T: Integer,
    Iter: ByteIter<'a>,
{
    debug_assert_radix(RADIX);

    let is_negative = parse_sign_and_validate(&mut iter, FORMAT, T::IS_SIGNED)?;
    parse_digits(iter, RADIX, is_negative)
}
