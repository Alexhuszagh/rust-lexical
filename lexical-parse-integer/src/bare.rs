//! Bare-bones implementation algorithm to avoid any abstractions.
// TODO(ahuszagh) Need to make sure this gets moved to compact and algorithm.

use lexical_util::digit::{AsDigits, char_to_digit_const};
use lexical_util::format::NumberFormat;
use lexical_util::num::{as_cast, Integer};
use lexical_util::iterator::{Byte, ByteIter};
use lexical_util::result::Result;

/// Generic algorithm for both partial and complete parsers.
///
/// * `invalid_digit` - Behavior on finding an invalid digit.
/// * `into_ok` - Behavior when returning a valid value.
#[rustfmt::skip]
macro_rules! algorithm {
    (
        $bytes:ident,
        $format:ident,
        $parser:ident,
        $invalid_digit:ident,
        $into_ok:ident
    ) => {{
        let format = NumberFormat::<{ $format }> {};

        // None of this code can be changed for optimization reasons.
        // Do not change it without benchmarking every change.
        //  1. You cannot use the NoSkipIterator in the loop,
        //      you must either return a subslice (indexing)
        //      or increment outside of the loop.
        //      Failing to do so leads to numerous more, unnecessary
        //      conditional move instructions, killing performance.
        //  2. Return a 0 or 1 shift, and indexing unchecked outside
        //      of the loop is slightly faster.
        //  3. Partial and complete parsers cannot be efficiently done
        //      together.

        // With `step_by_unchecked`, this is sufficiently optimized.
        // Removes conditional paths, to, which simplifies maintenance.
        // The skip version of the iterator automatically coalesces to
        // the noskip iterator.
        let mut byte = $bytes.digits::<{ $format }>();

        let mut iter = byte.integer_iter();
        let (is_negative, shift) = parse_sign!(iter, format);
        unsafe { iter.step_by_unchecked(shift); }
        if ByteIter::is_empty(&iter) {
            return into_error!(Empty, shift);
        }
        // If we have a format that doesn't accept leading zeros,
        // check if the next value is invalid.
        if format.no_integer_leading_zeros() && iter.peek() == Some(&b'0') {
            return into_error!(InvalidLeadingZeros, shift);
        }

        let mut value = T::ZERO;
        if !T::IS_SIGNED || !is_negative {
            $parser!(value, iter, format.radix(), checked_add, Overflow, $invalid_digit);
        } else {
            $parser!(value, iter, format.radix(), checked_sub, Underflow, $invalid_digit);
        }
        $into_ok!(value, iter.length())
    }};
}

/// Algorithm for the complete parser.
#[cfg_attr(not(feature = "compact"), inline)]
pub fn algorithm_complete<'a, T, const FORMAT: u128>(bytes: &[u8]) -> Result<T>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, parse_compact, invalid_digit_err, into_ok_value)
}

/// Algorithm for the partial parser.
#[cfg_attr(not(feature = "compact"), inline)]
pub fn algorithm_partial<'a, T, const FORMAT: u128>(bytes: &[u8]) -> Result<(T, usize)>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, parse_compact, invalid_digit_ok, into_ok_index)
}
