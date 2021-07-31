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
        Ok($value)
    };
}

/// Return an value and index for a partial parser.
macro_rules! into_ok_index {
    ($value:ident, $index:expr) => {
        Ok(($value, $index))
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
    ($iter:ident, $format:ident) => (
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
    );
}

/// Parse digits for a positive or negative value.
/// This has no multiple-digit optimizations.
#[rustfmt::skip]
macro_rules! parse_compact {
    (
        $value:ident,
        $iter:ident,
        $radix:expr,
        $addsub:ident,
        $overflow:ident,
        $invalid_digit:ident
    ) => {{
        // Do our slow parsing algorithm: 1 digit at a time.
        while let Some(&c) = $iter.next() {
            let digit = match char_to_digit_const(c, $radix) {
                Some(v) => v,
                None => return $invalid_digit!($value, $iter.cursor() - 1),
            };
            $value = match $value.checked_mul(as_cast($radix)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter.cursor() - 1),
            };
            $value = match $value.$addsub(as_cast(digit)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter.cursor() - 1),
            };
        }
    }};
}
