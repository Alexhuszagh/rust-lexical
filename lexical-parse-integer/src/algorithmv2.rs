// TODO(ahuszagh) Rename to algorithm
#![cfg(not(feature = "compact"))]

use lexical_util::digit::{char_to_digit_const, AsDigits};
use lexical_util::format::NumberFormat;
use lexical_util::iterator::{Byte, ByteIter};
use lexical_util::num::{as_cast, Integer, Number};
use lexical_util::result::Result;
use lexical_util::step::u64_step;

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
        // TODO(ahuszagh) Need optimizations...
        // Needs the data type...

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

// TODO(ahuszagh) Need to share things...

/// Algorithm for the complete parser.
#[inline]
pub fn algorithm_complete<'a, T, const FORMAT: u128>(bytes: &[u8]) -> Result<T>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, T, parse_digits, invalid_digit_err, into_ok_value)
}

/// Algorithm for the partial parser.
#[inline]
pub fn algorithm_partial<'a, T, const FORMAT: u128>(bytes: &[u8]) -> Result<(T, usize)>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, T, parse_digits, invalid_digit_ok, into_ok_index)
}
