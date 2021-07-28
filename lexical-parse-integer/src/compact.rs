//! Radix-generic, unoptimized, string-to-integer conversion routines.
//!
//! These routines aim to be compact, at the cost of performance.

#![cfg(feature = "compact")]

use crate::sign::parse_sign_and_validate;
use lexical_util::assert::debug_assert_radix;
use lexical_util::digit::char_to_digit;
use lexical_util::format::NumberFormat;
use lexical_util::iterator::ByteIter;
use lexical_util::num::{as_cast, Integer};
use lexical_util::result::Result;

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
) -> Result<(T, usize)>
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

/// Core parsing algorithm.
/// See `parse_digits` for a detailed explanation of the algorithms.
///
/// Returns the parsed value and the number of digits processed.
#[inline]
pub fn algorithm<'a, T, Iter, const FORMAT: u128>(mut iter: Iter) -> Result<(T, usize)>
where
    T: Integer,
    Iter: ByteIter<'a>,
{
    let radix = NumberFormat::<{ FORMAT }>::MANTISSA_RADIX;
    debug_assert_radix(radix);

    let is_negative = parse_sign_and_validate::<T, _, FORMAT>(&mut iter)?;
    parse_digits(iter, radix, is_negative)
}
