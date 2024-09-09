//! Radix-generic, unoptimized, string-to-integer conversion routines.
//!
//! These routines aim to be compact, at the cost of performance.

#![cfg(feature = "compact")]
#![doc(hidden)]

use lexical_util::digit::char_to_digit_const;
use lexical_util::format::NumberFormat;
use lexical_util::iterator::{AsBytes, BytesIter};
use lexical_util::num::{as_cast, Integer};
use lexical_util::result::Result;
use lexical_util::step::min_step;

/// Algorithm for the complete parser.
pub fn algorithm_complete<T, const FORMAT: u128>(bytes: &[u8]) -> Result<T>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, T, parse_1digit, invalid_digit_complete, into_ok_complete)
}

/// Algorithm for the partial parser.
pub fn algorithm_partial<T, const FORMAT: u128>(bytes: &[u8]) -> Result<(T, usize)>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, T, parse_1digit, invalid_digit_partial, into_ok_partial)
}
