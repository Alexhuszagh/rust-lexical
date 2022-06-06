//! Shared algorithms and utilities for parsing integers.
//!
//! These allow implementations of partial and complete parsers
//! using a single code-path via macros.
//!
//! This looks relatively, complex, but it's quite simple. Almost all
//! of these branches are resolved at compile-time, so the resulting
//! code is quite small while handling all of the internal complexity.
//!
//! 1. Helpers to process ok and error results for both complete and partial
//!     parsers. They have different APIs, and mixing the APIs leads to
//!     substantial performance hits.
//! 2. Overflow checking on invalid digits for partial parsers, while
//!     just returning invalid digits for complete parsers.
//! 3. A format-aware sign parser.
//! 4. Digit parsing algorithms which explicitly wrap on overflow, for no
//!     additional overhead. This has major performance wins for **most**
//!     real-world integers, so most valid input will be substantially faster.
//! 5. An algorithm to detect if overflow occurred. This is comprehensive,
//!     and short-circuits for common cases.
//! 6. A parsing algorithm for unsigned integers, always producing positive
//!     values. This avoids any unnecessary branching.
//! 7. Multi-digit optimizations for larger sizes.

#![doc(hidden)]

use lexical_util::format::NumberFormat;
use lexical_util::num::{as_cast, Integer, UnsignedInteger};
use lexical_util::step::max_step;

/// Return an error, returning the index and the error.
macro_rules! into_error {
    ($code:ident, $index:expr) => {
        Err((lexical_util::error::Error::$code($index)))
    };
}

/// Return an value for a complete parser.
macro_rules! into_ok_complete {
    ($value:expr, $index:expr) => {
        Ok(as_cast($value))
    };
}

/// Return an value and index for a partial parser.
macro_rules! into_ok_partial {
    ($value:expr, $index:expr) => {
        Ok((as_cast($value), $index))
    };
}

/// Return an error for a complete parser upon an invalid digit.
macro_rules! invalid_digit_complete {
    (
        $value:ident,
        $iter:ident,
        $format:ident,
        $is_negative:ident,
        $start_index:ident,
        $t:ident,
        $u:ident
    ) => {{
        // Don't do any overflow checking here: we don't need it.
        into_error!(InvalidDigit, $iter.cursor() - 1)
    }};
}

/// Return a value for a partial parser upon an invalid digit.
/// This checks for numeric overflow, and returns the appropriate error.
macro_rules! invalid_digit_partial {
    (
        $value:ident,
        $iter:ident,
        $format:ident,
        $is_negative:ident,
        $start_index:ident,
        $t:ident,
        $u:ident
    ) => {{
        let radix = NumberFormat::<{ $format }>::MANTISSA_RADIX;
        let count = $iter.current_count() - $start_index - 1;
        if is_overflow::<$t, $u, $format>($value, count, $is_negative) {
            let min = min_step(radix, <$t as Integer>::BITS, <$t>::IS_SIGNED);
            if <$t>::IS_SIGNED && $is_negative {
                into_error!(Underflow, (count - 1).min(min + 1))
            } else {
                into_error!(Overflow, (count - 1).min(min + 1))
            }
        } else if <$t>::IS_SIGNED && $is_negative {
            into_ok_partial!($value.wrapping_neg(), $iter.cursor() - 1)
        } else {
            into_ok_partial!($value, $iter.cursor() - 1)
        }
    }};
}

/// Parse the sign from the leading digits.
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
macro_rules! parse_sign {
    ($iter:ident, $format:ident) => {
        //  This works in all cases, and gets a few handy
        //  optimizations:
        //  1. It minimizes branching: we either need to subslice
        //      or return an offset from the loop. We can't increment
        //      the iterator in the loop or it decimates performance.
        //
        //  Using `iter.peek()` means we respect digit separators at
        //  the start of the number, when they're valid.
        //
        //  All the other cases are removed at compile time.
        //  Note: the compiler isn't smart enough to realize that
        //  `Some(_) if !$format.call() =>` and `Some(_) =>` are
        //  mutually exclusive, so make sure we manually expand
        //  these cases.
        match $iter.peek() {
            Some(&b'+') if !$format.no_positive_mantissa_sign() => (false, 1),
            Some(&b'+') if $format.no_positive_mantissa_sign() => {
                return into_error!(InvalidPositiveSign, 0);
            },
            // Don't add the check for the negative sign here if unsigned,
            // since it absolutely decimates performance. If it's for a
            // partial parser, we'll simply get 0 digits parsed, like before.
            // Complete parsers will still error, like before. That is, it's
            // correct **enough**.
            Some(&b'-') if T::IS_SIGNED => (true, 1),
            Some(_) if $format.required_mantissa_sign() => return into_error!(MissingSign, 0),
            _ => (false, 0),
        }
    };
}

/// Determine if the value has overflowed.
#[cfg_attr(not(feature = "compact"), inline)]
pub(super) fn is_overflow<T, U, const FORMAT: u128>(
    value: U,
    count: usize,
    is_negative: bool,
) -> bool
where
    T: Integer,
    U: UnsignedInteger,
{
    let format = NumberFormat::<{ FORMAT }> {};

    let max = max_step(format.radix(), T::BITS, T::IS_SIGNED);
    let radix: U = as_cast(format.radix());
    let min_value: U = radix.pow(max as u32 - 1);
    if T::IS_SIGNED {
        // Signed type: have to deal with 2's complement.
        let max_value: U = as_cast::<U, _>(T::MAX) + U::ONE;
        if count > max
            || (count == max
                && (value < min_value || value > max_value || (!is_negative && value == max_value)))
        {
            // Must have overflowed, or wrapped.
            // 1. Guaranteed overflow due to too many digits.
            // 2. Guaranteed overflow due to wrap.
            // 3. Guaranteed overflow since it's too large for the signed type.
            // 4. Guaranteed overflow due to 2's complement.
            return true;
        }
    } else if count > max || (count == max && value < min_value) {
        // Must have overflowed: too many digits or wrapped.
        return true;
    }
    false
}

/// Parse the value for the given type.
macro_rules! parse_value {
    (
        $iter:ident,
        $is_negative:ident,
        $format:ident,
        $start_index:ident,
        $t:ident,
        $u:ident,
        $parser:ident,
        $invalid_digit:ident,
        $into_ok:ident
    ) => {{
        // Use a simple optimization: parse as an unsigned integer, using
        // unsigned arithmetic , avoiding any branching in the initial stage.
        // We can then validate the input based on the signed integer limits,
        // and cast the value over, which is fast. Leads to substantial
        // improvements due to decreased branching for all but `i8`.
        let mut value = <$u>::ZERO;
        let format = NumberFormat::<{ $format }> {};
        $parser!(value, $iter, $format, $is_negative, $start_index, $t, $u, $invalid_digit);
        let count = $iter.current_count() - $start_index;

        if is_overflow::<$t, $u, $format>(value, count, $is_negative) {
            let min = min_step(format.radix(), <$t as Integer>::BITS, <$t>::IS_SIGNED);
            if <$t>::IS_SIGNED && $is_negative {
                into_error!(Underflow, (count - 1).min(min + 1))
            } else {
                into_error!(Overflow, (count - 1).min(min + 1))
            }
        } else if <$t>::IS_SIGNED && $is_negative {
            // Need to cast it to the signed type first, so we don't
            // get an invalid representation for i128 if it's widened.
            $into_ok!(as_cast::<$t, _>(value.wrapping_neg()), $iter.length())
        } else {
            $into_ok!(value, $iter.length())
        }
    }};
}

/// Parse a single digit at a time.
/// This has no multiple-digit optimizations.
#[rustfmt::skip]
macro_rules! parse_1digit {
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
        let format = NumberFormat::<{ $format }>;
        let radix = NumberFormat::<{ $format }>::MANTISSA_RADIX;

        // Do our slow parsing algorithm: 1 digit at a time.
        while let Some(&c) = $iter.next() {
            let digit = match char_to_digit_const(c, radix) {
                Some(v) => v,
                None => {
                    // Need to check for a base suffix, if so, return a valid value.
                    // We can't have a base suffix at the first value (need at least
                    // 1 digit).
                    let base_suffix = format.base_suffix();
                    if cfg!(feature = "format") && base_suffix != 0 && $iter.cursor() - $start_index > 1 {
                        let is_suffix = if format.case_sensitive_base_suffix() {
                            c == base_suffix
                        } else {
                            c.to_ascii_lowercase() == base_suffix.to_ascii_lowercase()
                        };
                        if is_suffix && $iter.is_done() {
                            // Break out of the loop, we've finished parsing.
                            break;
                        } else if is_suffix {
                            // Haven't finished parsing, so we're going to call
                            // invalid_digit!. Need to ensure we include the
                            // base suffix in that.
                            // SAFETY: safe since the iterator is not empty, as checked
                            // in `$iter.is_done()` above.
                            unsafe { $iter.step_unchecked() };
                        }
                    }
                    // Might have handled our base-prefix here.
                    return $invalid_digit!(
                        $value,
                        $iter,
                        $format,
                        $is_negative,
                        $start_index,
                        $t,
                        $u
                    );
                },
            };
            $value = $value.wrapping_mul(as_cast(radix));
            $value = $value.wrapping_add(as_cast(digit));
        }
    }};
}

/// Generic algorithm for both partial and complete parsers.
///
/// * `invalid_digit` - Behavior on finding an invalid digit.
/// * `into_ok` - Behavior when returning a valid value.
#[rustfmt::skip]
macro_rules! algorithm {
    (
        $bytes:ident,
        $format:ident,
        $t:ident,
        $u:ident,
        $parser:ident,
        $invalid_digit:ident,
        $into_ok:ident
    ) => {{
        let format = NumberFormat::<{ $format }> {};

        // WARNING:
        // --------
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
        //
        // If you try to refactor without carefully monitoring benchmarks or
        // assembly generation, please log the number of wasted hours: so
        //  16 hours so far.

        // With `step_by_unchecked`, this is sufficiently optimized.
        // Removes conditional paths, to, which simplifies maintenance.
        // The skip version of the iterator automatically coalesces to
        // the noskip iterator.
        let mut byte = $bytes.bytes::<{ $format }>();

        let mut iter = byte.integer_iter();
        let (is_negative, shift) = parse_sign!(iter, format);
        // SAFETY: safe since we shift at most one for a parsed sign byte.
        unsafe { iter.step_by_unchecked(shift) };
        if iter.is_done() {
            return into_error!(Empty, shift);
        }
        // Skip any leading zeros.
        let mut start_index = iter.cursor();
        let zeros = iter.skip_zeros();
        start_index += zeros;

        // Now, check to see if we have a valid base prefix.
        let base_prefix = format.base_prefix();
        let mut is_prefix = false;
        if cfg!(feature = "format") && base_prefix != 0 && zeros == 1 {
            // Check to see if the next character is the base prefix.
            // We must have a format like `0x`, `0d`, `0o`. Note:
            if let Some(&c) = iter.peek() {
                is_prefix = if format.case_sensitive_base_prefix() {
                    c == base_prefix
                } else {
                    c.to_ascii_lowercase() == base_prefix.to_ascii_lowercase()
                };
                if is_prefix {
                    // SAFETY: safe since we `byte.len() >= 1`.
                    unsafe { iter.step_unchecked() };
                    if iter.is_done() {
                        return into_error!(Empty, iter.cursor());
                    } else {
                        start_index += 1;
                    }
                }
            }
        }

        // If we have a format that doesn't accept leading zeros,
        // check if the next value is invalid. It's invalid if the
        // first is 0, and the next is not a valid digit.
        if cfg!(feature = "format") && !is_prefix && format.no_integer_leading_zeros() && zeros != 0 {
            // Cannot have a base prefix and no leading zeros.
            let index = iter.cursor() - zeros;
            if zeros > 1 {
                return into_error!(InvalidLeadingZeros, index);
            }
            match iter.peek().map(|&c| char_to_digit_const(c, format.radix())) {
                // Valid digit, we have an invalid value.
                Some(Some(_)) => return into_error!(InvalidLeadingZeros, index),
                // Either not a digit that follows, or nothing follows.
                _ => return $into_ok!(<$t>::ZERO, index),
            };
        }

        //  NOTE:
        //      Don't add optimizations for 128-bit integers.
        //      128-bit multiplication is rather efficient, it's only division
        //      that's very slow. Any shortcut optimizations increasing branching,
        //      and even if parsing a 64-bit integer is marginally faster, it
        //      culminates in **way** slower performance overall for simple
        //      integers, and no improvement for large integers.
        parse_value!(
            iter,
            is_negative,
            $format,
            start_index,
            $t,
            $u,
            $parser,
            $invalid_digit,
            $into_ok
        )
    }};
}
