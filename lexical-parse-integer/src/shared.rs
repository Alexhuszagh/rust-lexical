//! Shared algorithms and utilities for parsing integers.

/// Return an error, returning the index and the error.
macro_rules! into_error {
    ($code:ident, $index:expr) => {
        Err((lexical_util::error::ErrorCode::$code, $index).into())
    };
}

/// Return an value for a complete parser.
macro_rules! into_ok_value {
    ($value:ident, $index:expr) => {
        Ok(as_cast($value))
    };
}

/// Return an value and index for a partial parser.
macro_rules! into_ok_index {
    ($value:ident, $index:expr) => {
        Ok((as_cast($value), $index))
    };
}

/// Return an error for a complete parser upon an invalid digit.
macro_rules! invalid_digit_err {
    ($value:ident, $index:expr) => {
        into_error!(InvalidDigit, $index)
    };
}

/// Return a value for a partial parser upon an invalid digit.
macro_rules! invalid_digit_ok {
    ($value:ident, $index:expr) => {
        into_ok_index!($value, $index)
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
            Some(&b'-') if T::IS_SIGNED => (true, 1),
            Some(&b'+') if $format.no_positive_mantissa_sign() => {
                return into_error!(InvalidPositiveSign, 0);
            },
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
        if !<$t>::IS_SIGNED || !$is_negative {
            $parser!(value, $iter, $format.radix(), checked_add, Overflow, $invalid_digit);
        } else {
            $parser!(value, $iter, $format.radix(), checked_sub, Underflow, $invalid_digit);
        }
        $into_ok!(value, $iter.length())
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

        // Optimization for 128-bit integers.
        // This always works, since the length includes the sign bit.
        // If the input is 64 digits, base 2, with the sign, then the
        // value can't be greater than `2^63 - 1` magnitude, which can
        // be stored by any positive or negative 64-bit int.
        if cfg!(not(feature = "compact")) &&
            <$t>::BITS == 128 &&
            iter.length() <= u64_step(format.radix()) {
                // These types will get resolved at compile time.
                if <$t>::IS_SIGNED {
                    parse_value!(iter, is_negative, format, i64, $parser, $invalid_digit, $into_ok)
                } else {
                    parse_value!(iter, is_negative, format, u64, $parser, $invalid_digit, $into_ok)
                }
        } else {
            parse_value!(iter, is_negative, format, $t, $parser, $invalid_digit, $into_ok)
        }
    }};
}
