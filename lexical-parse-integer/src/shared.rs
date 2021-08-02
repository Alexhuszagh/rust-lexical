//! Shared algorithms and utilities for parsing integers.
//!
//! These allow implementations of partial and complete parsers
//! using a single code-path via macros.

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
    ($value:expr, $index:expr) => {
        into_error!(InvalidDigit, $index)
    };
}

/// Return a value for a partial parser upon an invalid digit.
macro_rules! invalid_digit_partial {
    ($value:expr, $index:expr) => {
        into_ok_partial!($value, $index)
    };
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

/// Parse the value for the given type.
macro_rules! parse_value {
    (
        $iter:ident,
        $is_negative:ident,
        $format:ident,
        $t:ident,
        $parser:ident,
        $invalid_digit:ident,
        $into_ok:ident
    ) => {{
        let mut value = <$t>::ZERO;
        let format = NumberFormat::<{ $format }> {};
        if !<$t>::IS_SIGNED || !$is_negative {
            $parser!(
                value,
                $iter,
                $format,
                format.radix(),
                checked_add,
                Overflow,
                $t,
                $invalid_digit
            );
        } else {
            $parser!(
                value,
                $iter,
                $format,
                format.radix(),
                checked_sub,
                Underflow,
                $t,
                $invalid_digit
            );
        }
        $into_ok!(value, $iter.length())
    }};
}

/// Parse digits for a positive or negative value.
/// This has no multiple-digit optimizations.
#[rustfmt::skip]
macro_rules! parse_compact {
    (
        $value:ident,
        $iter:ident,
        $format:ident,
        $radix:expr,
        $addsub:ident,
        $overflow:ident,
        $t:ident,
        $invalid_digit:ident
    ) => {{
        // Do our slow parsing algorithm: 1 digit at a time.
        while let Some(&c) = $iter.next() {
            let digit = match char_to_digit_const(c, $radix) {
                Some(v) => v,
                None => return $invalid_digit!($value, $iter.cursor() - 1),
            };
            $value = match $value.checked_mul(<$t>::from_u32($radix)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter.cursor() - 1),
            };
            $value = match $value.$addsub(<$t>::from_u32(digit)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter.cursor() - 1),
            };
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
        //  10 hours so far.

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
        // check if the next value is invalid. It's invalid if the
        // first is 0, and the next is not a valid digit.
        if format.no_integer_leading_zeros() && iter.peek() == Some(&b'0') {
            // Has at least 1 element, and is a 0. Check if the next
            // peeked value is a valid digit.
            let index = iter.cursor();
            unsafe { iter.step_by_unchecked(1); }
            match iter.peek().map(|&c| char_to_digit_const(c, format.radix())) {
                // Valid digit, we have an invalid value.
                Some(Some(_)) => return into_error!(InvalidLeadingZeros, index),
                // Either not a digit that follows, or nothing follows.
                _ => return $into_ok!(<$t>::ZERO, index),
            };
        }

        // Optimization for 128-bit integers.
        // This always works, since the length includes the sign bit.
        // If the input is 64 digits, base 2, with the sign, then the
        // value can't be greater than `2^63 - 1` magnitude, which can
        // be stored by any positive or negative 64-bit int.
        if cfg!(not(feature = "compact")) &&
            <$t>::BITS == 128 &&
            iter.length() <= u64_step(format.radix()) {
                // These types will get resolved at compile time.
                // Choose the compact parser, since we've already got enough
                // branching at this point, and it kills performance for
                // small 128-bit values. Can increase parse time for simple
                // values by ~60% or more.
                if <$t>::IS_SIGNED {
                    parse_value!(iter, is_negative, $format, i64, parse_compact, $invalid_digit, $into_ok)
                } else {
                    parse_value!(iter, is_negative, $format, u64, parse_compact, $invalid_digit, $into_ok)
                }
        } else {
            parse_value!(iter, is_negative, $format, $t, $parser, $invalid_digit, $into_ok)
        }
    }};
}
