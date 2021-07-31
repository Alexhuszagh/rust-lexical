// Bare-bones implementation algorithm to avoid any abstractions.

use lexical_util::digit::{AsDigits, char_to_digit_const};
use lexical_util::format::{NumberFormat, STANDARD};
use lexical_util::num::{as_cast, Integer};
use lexical_util::error::ErrorCode;
use lexical_util::iterator::{Byte, ByteIter};
use lexical_util::result::Result;

use lexical_util::from_lexical;

/// Return an error, returning the index and the error.
macro_rules! into_error {
    ($code:ident, $index:expr) => {
        Err((ErrorCode::$code, $index).into())
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
/// Optimized for operations with machine integers.
#[rustfmt::skip]
macro_rules! parse_digits {
    (
        $value:ident,
        $iter:ident,
        $radix:expr,
        $addsub:ident,
        $overflow:ident,
        $invalid_digit:ident
    ) => {{
        // TODO(ahuszagh) Add optimizations for types...

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

/// Generic algorithm for both partial and complete parsers.
///
/// * `invalid_digit` - Behavior on finding an invalid digit.
/// * `into_ok` - Behavior when returning a valid value.
#[rustfmt::skip]
macro_rules! algorithm {
    (
        $bytes:ident,
        $format:ident,
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
        if format.no_integer_leading_zeros() && iter.peek() == Some(&b'0') {
            return into_error!(InvalidLeadingZeros, shift);
        }

        let mut value = T::ZERO;
        if !T::IS_SIGNED || !is_negative {
            parse_digits!(value, iter, format.radix(), checked_add, Overflow, $invalid_digit);
        } else {
            parse_digits!(value, iter, format.radix(), checked_sub, Underflow, $invalid_digit);
        }
        $into_ok!(value, iter.length())
    }};
}

/// Algorithm for the complete parser.
#[inline]
fn algorithm<'a, T, const FORMAT: u128>(bytes: &[u8]) -> Result<T>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, invalid_digit_err, into_ok_value)
}

/// Algorithm for the partial parser.
#[inline]
fn algorithm_partial<'a, T, const FORMAT: u128>(bytes: &[u8]) -> Result<(T, usize)>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, invalid_digit_ok, into_ok_index)
}

// Implement FromLexical for numeric type.
macro_rules! integer_from_lexical {
    ($($t:tt $(, #[$meta:meta])? ; )*) => ($(
        impl FromLexical for $t {
            $(#[$meta:meta])?
            #[inline]
            fn from_lexical(bytes: &[u8]) -> Result<Self> {
                algorithm::<_, STANDARD>(bytes)
            }

            #[inline]
            fn from_lexical_partial(bytes: &[u8]) -> Result<(Self, usize)> {
                algorithm_partial::<_, STANDARD>(bytes)
            }
        }
    )*)
}

from_lexical! {}
integer_from_lexical! {
    u8 ;
    u16 ;
    u32 ;
    u64 ;
    u128 ;
    usize ;
    i8 ;
    i16 ;
    i32 ;
    i64 ;
    i128 ;
    isize ;
}
