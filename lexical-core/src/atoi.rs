//! Fast lexical string-to-integer conversion routines.

//  The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//  CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//  (x86-64), using the lexical formatter or `x.parse()`,
//  avoiding any inefficiencies in Rust string parsing. The code was
//  compiled with LTO and at an optimization level of 3.
//
//  The benchmarks with `std` were compiled using "rustc 1.32.0
// (9fda7c223 2019-01-16)".
//
//  The benchmark code may be found `benches/atoi.rs`.
//
//  # Benchmarks
//
//  | Type  |  lexical (ns/iter) | parse (ns/iter)       | Relative Increase |
//  |:-----:|:------------------:|:---------------------:|:-----------------:|
//  | u8    | 75,622             | 80,021                | 1.06x             |
//  | u16   | 80,926             | 82,185                | 1.02x             |
//  | u32   | 131,221            | 148,231               | 1.13x             |
//  | u64   | 243,315            | 296,713               | 1.22x             |
//  | u128  | 512,552            | 1,175,946             | 2.29x             |
//  | i8    | 112,152            | 115,147               | 1.03x             |
//  | i16   | 153,670            | 150,231               | 0.98x             |
//  | i32   | 202,512            | 204,880               | 1.01x             |
//  | i64   | 309,731            | 309,584               | 1.00x             |
//  | i128  | 4,362,672          | 149,418,085           | 34.3x             |
//
//  # Raw Benchmarks
//
//  ```text
//  test atoi_i128_lexical        ... bench:   4,305,621 ns/iter (+/- 132,707)
//  test atoi_i128_parse          ... bench: 146,893,478 ns/iter (+/- 2,822,002)
//  test atoi_i16_lexical         ... bench:     132,255 ns/iter (+/- 5,503)
//  test atoi_i16_parse           ... bench:     137,965 ns/iter (+/- 5,906)
//  test atoi_i32_lexical         ... bench:     207,101 ns/iter (+/- 79,541)
//  test atoi_i32_parse           ... bench:     194,225 ns/iter (+/- 9,065)
//  test atoi_i64_lexical         ... bench:     271,538 ns/iter (+/- 9,137)
//  test atoi_i64_parse           ... bench:     293,542 ns/iter (+/- 9,706)
//  test atoi_i8_lexical          ... bench:     106,368 ns/iter (+/- 5,919)
//  test atoi_i8_parse            ... bench:     108,316 ns/iter (+/- 3,418)
//  test atoi_u128_lexical        ... bench:     496,426 ns/iter (+/- 40,197)
//  test atoi_u128_parse          ... bench:   1,119,213 ns/iter (+/- 54,945)
//  test atoi_u128_simple_lexical ... bench:     121,121 ns/iter (+/- 4,858)
//  test atoi_u128_simple_parse   ... bench:      97,518 ns/iter (+/- 2,739)
//  test atoi_u16_lexical         ... bench:      80,886 ns/iter (+/- 2,366)
//  test atoi_u16_parse           ... bench:      81,881 ns/iter (+/- 1,708)
//  test atoi_u16_simple_lexical  ... bench:      62,819 ns/iter (+/- 1,707)
//  test atoi_u16_simple_parse    ... bench:      60,916 ns/iter (+/- 8,340)
//  test atoi_u32_lexical         ... bench:     139,264 ns/iter (+/- 3,945)
//  test atoi_u32_parse           ... bench:     139,649 ns/iter (+/- 5,735)
//  test atoi_u32_simple_lexical  ... bench:      61,398 ns/iter (+/- 1,248)
//  test atoi_u32_simple_parse    ... bench:      59,560 ns/iter (+/- 3,388)
//  test atoi_u64_lexical         ... bench:     257,116 ns/iter (+/- 6,810)
//  test atoi_u64_parse           ... bench:     273,811 ns/iter (+/- 6,871)
//  test atoi_u64_simple_lexical  ... bench:      59,674 ns/iter (+/- 4,852)
//  test atoi_u64_simple_parse    ... bench:      55,982 ns/iter (+/- 2,288)
//  test atoi_u8_lexical          ... bench:      70,637 ns/iter (+/- 1,889)
//  test atoi_u8_parse            ... bench:      67,606 ns/iter (+/- 1,924)
//  test atoi_u8_simple_lexical   ... bench:      41,190 ns/iter (+/- 6,948)
//  test atoi_u8_simple_parse     ... bench:      36,836 ns/iter (+/- 2,958)
//  ```
//
// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([75622, 80926, 131221, 243315, 512552, 112152, 153670, 202512, 309731, 4362672]) / 1e6
//  rustcore = np.array([80021, 82185, 148231, 296713, 1175946, 115147, 150231, 204880, 309584, 149418085]) / 1e6
//  index = ["u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128"]
//  df = pd.DataFrame({'lexical': lexical, 'rustcore': rustcore}, index = index, columns=['lexical', 'rustcore'])
//  ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14, color=['#E24A33', '#348ABD'])
//  ax.set_ylabel("ms/iter")
//  ax.set_yscale('log')
//  ax.figure.tight_layout()
//  ax.legend(loc=2, prop={'size': 14})
//  plt.show()

use util::*;
use lib::result::Result as StdResult;

// SHARED
// ------

// Convert u8 to digit.
macro_rules! to_digit {
    ($c:expr, $radix:expr) => (($c as char).to_digit($radix));
}

// Parse the sign bit and filter empty inputs from the atoi data.
macro_rules! parse_sign {
    ($bytes:ident, $is_signed:expr, $code:ident) => ({
        // Filter out empty inputs.
        if $bytes.is_empty() {
            return Err((ErrorCode::$code, $bytes.as_ptr()));
        }

        let (sign, digits) = match index!($bytes[0]) {
            b'+'               => (Sign::Positive, &index!($bytes[1..])),
            b'-' if $is_signed => (Sign::Negative, &index!($bytes[1..])),
            _                  => (Sign::Positive, $bytes),
        };

        // Filter out empty inputs.
        if digits.is_empty() {
            return Err((ErrorCode::$code, digits.as_ptr()));
        }

        (sign, digits)
    });
}

// STANDALONE
// ----------

/// Iterate over the digits and iteratively process them.
macro_rules! parse_digits {
    ($value:ident, $digits:ident, $radix:ident, $op:ident, $code:ident) => (
        for c in $digits.iter() {
            let digit = match to_digit!(*c, $radix) {
                Some(v) => v,
                None    => return Ok(($value, c)),
            };
            $value = match $value.checked_mul(as_cast($radix)) {
                Some(v) => v,
                None    => return Err((ErrorCode::$code, c)),
            };
            $value = match $value.$op(as_cast(digit)) {
                Some(v) => v,
                None    => return Err((ErrorCode::$code, c)),
            };
        }
    );
}

// Parse the digits for the atoi processor.
perftools_inline!{
pub(crate) fn parse_digits<T>(digits: &[u8], radix: u32, sign: Sign)
    -> StdResult<(T, *const u8), (ErrorCode, *const u8)>
    where T: Integer
{
    let mut value = T::ZERO;
    if sign == Sign::Positive {
        parse_digits!(value, digits, radix, checked_add, Overflow);
    } else {
        parse_digits!(value, digits, radix, checked_sub, Underflow);
    }
    let ptr = index!(digits[digits.len()..]).as_ptr();
    Ok((value, ptr))
}}

// Standalone atoi processor.
perftools_inline!{
pub(crate) fn standalone<T>(bytes: &[u8], radix: u32, is_signed: bool)
    -> StdResult<(T, *const u8), (ErrorCode, *const u8)>
    where T: Integer
{
    let (sign, digits) = parse_sign!(bytes, is_signed, Empty);
    parse_digits(digits, radix, sign)
}}

// STANDALONE U128
// ---------------

// Grab the step size and power for step_u64.
// This is the same as the u128 divisor, so don't duplicate the values
// there.
perftools_inline!{
#[cfg(has_i128)]
fn step_u64(radix: u32) -> usize {
    u128_divisor(radix).1
}}

// Add 64-bit temporary to the 128-bit value.
#[cfg(has_i128)]
macro_rules! add_temporary_128 {
    ($value:ident, $tmp:ident, $step_power:ident, $ptr:ident, $op:ident, $code:ident) => (
        if !$value.is_zero() {
            $value = match $value.checked_mul(as_cast($step_power)) {
                Some(v) => v,
                None    => return Err((ErrorCode::$code, $ptr)),
            };
        }
        $value = match $value.$op(as_cast($tmp)) {
            Some(v) => v,
            None    => return Err((ErrorCode::$code, $ptr)),
        };
    );
}

/// Iterate over the digits and iteratively process them.
#[cfg(has_i128)]
macro_rules! parse_digits_u128 {
    ($value:ident, $digits:ident, $radix:ident, $step:ident, $op:ident, $code:ident) => ({
        // Break the input into chunks of len `step`, which can be parsed
        // as a 64-bit integer.
        for chunk in $digits.chunks($step) {
            let mut tmp: u64 = 0;
            for (i, c) in chunk.iter().enumerate() {
                let digit = match to_digit!(*c, $radix) {
                    Some(v) => v,
                    None    => {
                        // Add temporary to value and return early.
                        let radix_pow = $radix.as_u64().pow(i.as_u32());
                        add_temporary_128!($value, tmp, radix_pow, c, $op, $code);
                        return Ok(($value, c));
                    },
                };
                // Don't have to worry about overflows.
                tmp *= $radix.as_u64();
                tmp += digit.as_u64();
            }

            // Add the temporary value to the total value.
            let radix_pow = $radix.as_u64().pow(chunk.len().as_u32());
            let ptr = chunk[chunk.len()..].as_ptr();
            add_temporary_128!($value, tmp, radix_pow, ptr, $op, $code);
        }
    });
}

// Parse the digits for the 128-bit atoi processor.
perftools_inline!{
#[cfg(has_i128)]
pub(crate) fn parse_digits_u128<T>(digits: &[u8], radix: u32, step: usize, sign: Sign)
    -> StdResult<(T, *const u8), (ErrorCode, *const u8)>
    where T: Integer
{
    let mut value = T::ZERO;
    if sign == Sign::Positive {
        parse_digits_u128!(value, digits, radix, step, checked_add, Overflow)
    } else {
        parse_digits_u128!(value, digits, radix, step, checked_sub, Underflow)
    }
    let ptr = index!(digits[digits.len()..]).as_ptr();
    Ok((value, ptr))
}}

// Standalone atoi processor for u128.
// This algorithm may overestimate the number of digits to overflow
// on numeric overflow or underflow, otherwise, it will be accurate.
// This is because we break costly u128 addition/multiplications into
// temporary steps using u64, allowing much better performance.
// This is a similar approach to what we take in the arbitrary-precision
// arithmetic
perftools_inline!{
#[cfg(has_i128)]
pub(crate) fn standalone_128<W, N>(bytes: &[u8], radix: u32, is_signed: bool)
    -> StdResult<(W, *const u8), (ErrorCode, *const u8)>
    where W: Integer,
          N: Integer
{
    // This is guaranteed to be safe, since if the length is
    // 1 less than step, and the min radix is 2, the value must be
    // less than 2x u64::MAX, which means it must fit in an i64.
    let (sign, digits) = parse_sign!(bytes, is_signed, Empty);
    let step = step_u64(radix);
    if digits.len() < step {
        // Parse as narrow.
        let (value, ptr) = parse_digits::<N>(digits, radix, sign)?;
        Ok((as_cast(value), ptr))
    } else {
        // Parse as wide.
        parse_digits_u128(digits, radix, step, sign)
    }
}}

// ATOI TRAIT
// ----------

pub(crate) trait Atoi: Integer {
    // Parse integer from string.
    fn atoi(bytes: &[u8], radix: u32, is_signed: bool) -> StdResult<(Self, *const u8), (ErrorCode, *const u8)>;
}

// Implement atoi for type.
macro_rules! atoi_impl {
    ($($t:ty)*) => ($(
        impl Atoi for $t {
            perftools_inline_always!{
            fn atoi(bytes: &[u8], radix: u32, is_signed: bool)
                -> StdResult<($t, *const u8), (ErrorCode, *const u8)>
            {
                standalone(bytes, radix, is_signed)
            }}
        }
    )*);
}

atoi_impl! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

#[cfg(has_i128)]
impl Atoi for u128 {
    perftools_inline_always!{
    fn atoi(bytes: &[u8], radix: u32, is_signed: bool)
        -> StdResult<(u128, *const u8), (ErrorCode, *const u8)>
    {
        standalone_128::<u128, u64>(bytes, radix, is_signed)
    }}
}

#[cfg(has_i128)]
impl Atoi for i128 {
    perftools_inline_always!{
    fn atoi(bytes: &[u8], radix: u32, is_signed: bool)
        -> StdResult<(i128, *const u8), (ErrorCode, *const u8)>
    {
        standalone_128::<i128, i64>(bytes, radix, is_signed)
    }}
}

// STANDALONE MANTISSA
// -------------------

// These routines are a specialized parser for the mantissa of a floating-
// point number, from two buffers containing valid digits. We want to
// exit early on numeric overflow, returning the value parsed up until
// that point.

// Convert character to digit.
perftools_inline_always!{
fn to_digit<'a>(c: &'a u8, radix: u32) -> StdResult<u32, &'a u8> {
    match to_digit!(*c, radix) {
        Some(v) => Ok(v),
        None    => Err(c),
    }
}}

// Convert character to digit.
perftools_inline_always!{
fn is_not_digit_char(c: u8, radix: u32) -> bool {
    to_digit!(c, radix).is_none()
}}

// Add digit to mantissa.
perftools_inline_always!{
#[cfg(feature = "correct")]
fn add_digit<T>(value: T, digit: u32, radix: u32)
    -> Option<T>
    where T: UnsignedInteger
{
    return value
        .checked_mul(as_cast(radix))?
        .checked_add(as_cast(digit))
}}

// Calculate the mantissa and the number of truncated digits from a digits iterator.
// All the data **must** be a valid digit.
perftools_inline!{
#[cfg(feature = "correct")]
pub(crate) fn standalone_mantissa<'a, T>(integer: &'a [u8], fraction: &'a [u8], radix: u32)
    -> (T, usize)
    where T: UnsignedInteger
{
    // Mote:
    //  Do not use iter.chain(), since it is enormously slow.
    //  Since we need to maintain backwards compatibility, even if
    //  iter.chain() is patched, for older Rustc versions, it's nor
    //  worth the performance penalty.

    let mut integer_iter = integer.iter();
    let mut fraction_iter = fraction.iter();
    let mut value: T = T::ZERO;
    // On overflow, validate that all the remaining characters are valid
    // digits, if not, return the first invalid digit. Otherwise,
    // calculate the number of truncated digits.
    while let Some(c) = integer_iter.next() {
        value = match add_digit(value, to_digit!(*c, radix).unwrap(), radix) {
            Some(v) => v,
            None    => {
                let truncated = 1 + integer_iter.len() + fraction_iter.len();
                return (value, truncated);
            },
        };
    }
    while let Some(c) = fraction_iter.next() {
        value = match add_digit(value, to_digit!(*c, radix).unwrap(), radix) {
            Some(v) => v,
            None    => {
                let truncated = 1 + fraction_iter.len();
                return (value, truncated);
            },
        };
    }
    (value, 0)
}}

// Calculate the mantissa when it cannot have sign or other invalid digits.
perftools_inline!{
#[cfg(not(feature = "correct"))]
pub(crate) fn standalone_mantissa<T>(bytes: &[u8], radix: u32)
    -> StdResult<(T, *const u8), (ErrorCode, *const u8)>
    where T: Integer
{
    // Parse the integer.
    let mut value = T::ZERO;
    parse_digits!(value, bytes, radix, checked_add, Overflow);
    Ok((value, bytes[bytes.len()..].as_ptr()))
}}

// STANDALONE EXPONENT
// -------------------

// These routines are a specialized parser for the exponent of a floating-
// point number, from an unvalidated buffer with a potential sign bit.
// On numeric overflow or underflow, we want to return the max or min
// value possible, respectively. On overflow, find the first non-digit
// char (if applicable), and return the max/min value and the number
// of digits parsed.

// Add digit to mantissa.
macro_rules! add_digit {
    ($value:ident, $radix:ident, $op:ident, $digit:ident) => {
        match $value.checked_mul(as_cast($radix)) {
            Some(v) => v.$op(as_cast($digit)),
            None    => None,
        }
    };
}

// Iterate over the digits and iteratively process them.
macro_rules! parse_digits_exponent {
    ($value:ident, $digits:ident, $radix:ident, $op:ident, $default:expr) => (
        let mut iter = $digits.iter();
        while let Some(c) = iter.next() {
            let digit = match to_digit(c, $radix) {
                Ok(v)  => v,
                Err(c) => return Ok(($value, c)),
            };
            $value = match add_digit!($value, $radix, $op, digit) {
                Some(v) => v,
                None    => {
                    // Consume the rest of the iterator to validate
                    // the remaining data.
                    if let Some(c) = iter.find(|&c| is_not_digit_char(*c, $radix)) {
                        return Ok(($default, c));
                    }
                    $default
                },
            };
        }
    );
}

// Specialized parser for the exponent, which validates digits and
// returns a default min or max value on overflow.
perftools_inline!{
pub(crate) fn standalone_exponent(bytes: &[u8], radix: u32)
    -> StdResult<(i32, *const u8), (ErrorCode, *const u8)>
{
    let (sign, digits) = parse_sign!(bytes, true, EmptyExponent);
    let mut value = 0;
    if sign == Sign::Positive {
        parse_digits_exponent!(value, digits, radix, checked_add, i32::max_value());
    } else {
        parse_digits_exponent!(value, digits, radix, checked_sub, i32::min_value());
    }
    let ptr = index!(digits[digits.len()..]).as_ptr();
    Ok((value, ptr))
}}

// INTERNAL
// --------

// Handle unsigned +/- numbers and forward to implied implementation.
//  Can just use local namespace
perftools_inline!{
pub(crate) fn standalone_unsigned<'a, T>(bytes: &'a [u8], radix: u32)
    -> Result<(T, usize)>
    where T: Atoi + UnsignedInteger
{
    let index = | ptr | distance(bytes.as_ptr(), ptr);
    match T::atoi(bytes, radix, false) {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}}

// Handle signed +/- numbers and forward to implied implementation.
//  Can just use local namespace
perftools_inline!{
pub(crate) fn standalone_signed<'a, T>(bytes: &'a [u8], radix: u32)
    -> Result<(T, usize)>
    where T: Atoi + SignedInteger
{
    let index = | ptr | distance(bytes.as_ptr(), ptr);
    match T::atoi(bytes, radix, true) {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}}

// FROM LEXICAL
// ------------

from_lexical!(standalone_unsigned, u8);
from_lexical!(standalone_unsigned, u16);
from_lexical!(standalone_unsigned, u32);
from_lexical!(standalone_unsigned, u64);
from_lexical!(standalone_unsigned, usize);
#[cfg(has_i128)] from_lexical!(standalone_unsigned, u128);

from_lexical!(standalone_signed, i8);
from_lexical!(standalone_signed, i16);
from_lexical!(standalone_signed, i32);
from_lexical!(standalone_signed, i64);
from_lexical!(standalone_signed, isize);
#[cfg(has_i128)] from_lexical!(standalone_signed, i128);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use util::*;

    #[cfg(feature = "radix")]
    const DATA: [(u8, &'static str); 35] = [
        (2, "100101"),
        (3, "1101"),
        (4, "211"),
        (5, "122"),
        (6, "101"),
        (7, "52"),
        (8, "45"),
        (9, "41"),
        (10, "37"),
        (11, "34"),
        (12, "31"),
        (13, "2B"),
        (14, "29"),
        (15, "27"),
        (16, "25"),
        (17, "23"),
        (18, "21"),
        (19, "1I"),
        (20, "1H"),
        (21, "1G"),
        (22, "1F"),
        (23, "1E"),
        (24, "1D"),
        (25, "1C"),
        (26, "1B"),
        (27, "1A"),
        (28, "19"),
        (29, "18"),
        (30, "17"),
        (31, "16"),
        (32, "15"),
        (33, "14"),
        (34, "13"),
        (35, "12"),
        (36, "11"),
    ];

    #[test]
    fn u8_decimal_test() {
        assert_eq!(Ok(0), u8::from_lexical(b"0"));
        assert_eq!(Ok(127), u8::from_lexical(b"127"));
        assert_eq!(Ok(128), u8::from_lexical(b"128"));
        assert_eq!(Ok(255), u8::from_lexical(b"255"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u8::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u8::from_lexical(b"1a"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn u8_radix_test() {
        for (b, s) in DATA.iter() {
            assert_eq!(u8::from_lexical_radix(s.as_bytes(), *b), Ok(37));
        }
    }

    #[test]
    fn i8_decimal_test() {
        assert_eq!(Ok(0), i8::from_lexical(b"0"));
        assert_eq!(Ok(127), i8::from_lexical(b"127"));
        assert_eq!(Err((ErrorCode::Overflow, 2).into()), i8::from_lexical(b"128"));
        assert_eq!(Err((ErrorCode::Overflow, 2).into()), i8::from_lexical(b"255"));
        assert_eq!(Ok(-1), i8::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i8::from_lexical(b"1a"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn i8_radix_test() {
        for (b, s) in DATA.iter() {
            assert_eq!(i8::from_lexical_radix(s.as_bytes(), *b), Ok(37));
        }
    }

    #[test]
    fn u16_decimal_test() {
        assert_eq!(Ok(0), u16::from_lexical(b"0"));
        assert_eq!(Ok(32767), u16::from_lexical(b"32767"));
        assert_eq!(Ok(32768), u16::from_lexical(b"32768"));
        assert_eq!(Ok(65535), u16::from_lexical(b"65535"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u16::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u16::from_lexical(b"1a"));
    }

    #[test]
    fn i16_decimal_test() {
        assert_eq!(Ok(0), i16::from_lexical(b"0"));
        assert_eq!(Ok(32767), i16::from_lexical(b"32767"));
        assert_eq!(Err((ErrorCode::Overflow, 4).into()), i16::from_lexical(b"32768"));
        assert_eq!(Err((ErrorCode::Overflow, 4).into()), i16::from_lexical(b"65535"));
        assert_eq!(Ok(-1), i16::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i16::from_lexical(b"1a"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn i16_radix_test() {
        for (b, s) in DATA.iter() {
            assert_eq!(i16::from_lexical_radix(s.as_bytes(), *b), Ok(37));
        }
        assert_eq!(i16::from_lexical_radix(b"YA", 36), Ok(1234));
    }

    #[test]
    fn u32_decimal_test() {
        assert_eq!(Ok(0), u32::from_lexical(b"0"));
        assert_eq!(Ok(2147483647), u32::from_lexical(b"2147483647"));
        assert_eq!(Ok(2147483648), u32::from_lexical(b"2147483648"));
        assert_eq!(Ok(4294967295), u32::from_lexical(b"4294967295"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u32::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u32::from_lexical(b"1a"));
    }

    #[test]
    fn i32_decimal_test() {
        assert_eq!(Ok(0), i32::from_lexical(b"0"));
        assert_eq!(Ok(2147483647), i32::from_lexical(b"2147483647"));
        assert_eq!(Err((ErrorCode::Overflow, 9).into()), i32::from_lexical(b"2147483648"));
        assert_eq!(Err((ErrorCode::Overflow, 9).into()), i32::from_lexical(b"4294967295"));
        assert_eq!(Ok(-1), i32::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i32::from_lexical(b"1a"));
    }

    #[test]
    fn u64_decimal_test() {
        assert_eq!(Ok(0), u64::from_lexical(b"0"));
        assert_eq!(Ok(9223372036854775807), u64::from_lexical(b"9223372036854775807"));
        assert_eq!(Ok(9223372036854775808), u64::from_lexical(b"9223372036854775808"));
        assert_eq!(Ok(18446744073709551615), u64::from_lexical(b"18446744073709551615"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u64::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u64::from_lexical(b"1a"));
    }

    #[test]
    fn i64_decimal_test() {
        assert_eq!(Ok(0), i64::from_lexical(b"0"));
        assert_eq!(Ok(9223372036854775807), i64::from_lexical(b"9223372036854775807"));
        assert_eq!(Err((ErrorCode::Overflow, 18).into()), i64::from_lexical(b"9223372036854775808"));
        assert_eq!(Err((ErrorCode::Overflow, 19).into()), i64::from_lexical(b"18446744073709551615"));
        assert_eq!(Ok(-1), i64::from_lexical(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i64::from_lexical(b"1a"));

        // Add tests discovered via fuzzing.
        assert_eq!(Err((ErrorCode::Overflow, 19).into()), i64::from_lexical(b"406260572150672006000066000000060060007667760000000000000000000+00000006766767766666767665670000000000000000000000666"));
    }

    #[cfg(feature = "std")]
    proptest! {
        #[test]
        fn u8_invalid_proptest(i in r"[+]?[0-9]{2}\D") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let index = result.err().unwrap().index;
            prop_assert!(index == 2 || index == 3);
        }

        #[test]
        fn u8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u8_negative_proptest(i in r"[-][1-9][0-9]{2}") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u8_double_sign_proptest(i in r"[+]{2}[0-9]{2}") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn u8_sign_only_proptest(i in r"[+]") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u8_trailing_digits_proptest(i in r"[+]?[0-9]{2}\D[0-9]{2}") {
            let result = u8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn i8_invalid_proptest(i in r"[+-]?[0-9]{2}\D") {
            let result = i8::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn i8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}\D") {
            let result = i8::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i8_underflow_proptest(i in r"[-][1-9][0-9]{3}\D") {
            let result = i8::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i8_double_sign_proptest(i in r"[+-]{2}[0-9]{2}") {
            let result = i8::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn i8_sign_only_proptest(i in r"[+-]") {
            let result = i8::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::Empty);
        }

        #[test]
        fn i8_trailing_digits_proptest(i in r"[+-]?[0-9]{2}\D[0-9]{2}") {
            let result = i8::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn u16_invalid_proptest(i in r"[+]?[0-9]{4}\D") {
            let result = u16::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn u16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}\D") {
            let result = u16::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u16_negative_proptest(i in r"[-][1-9][0-9]{4}") {
            let result = u16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u16_double_sign_proptest(i in r"[+]{2}[0-9]{4}") {
            let result = u16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn u16_sign_only_proptest(i in r"[+]") {
            let result = u16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u16_trailing_digits_proptest(i in r"[+]?[0-9]{4}\D[0-9]{2}") {
            let result = u16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn i16_invalid_proptest(i in r"[+-]?[0-9]{4}\D") {
            let result = i16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn i16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}\D") {
            let result = i16::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i16_underflow_proptest(i in r"[-][1-9][0-9]{5}\DD") {
            let result = i16::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i16_double_sign_proptest(i in r"[+-]{2}[0-9]{4}") {
            let result = i16::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn i16_sign_only_proptest(i in r"[+-]") {
            let result = i16::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i16_trailing_digits_proptest(i in r"[+-]?[0-9]{4}\D[0-9]{2}") {
            let result = i16::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn u32_invalid_proptest(i in r"[+]?[0-9]{9}\D") {
            let result = u32::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn u32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}\D") {
            let result = u32::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u32_negative_proptest(i in r"[-][1-9][0-9]{9}") {
            let result = u32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u32_double_sign_proptest(i in r"[+]{2}[0-9]{9}") {
            let result = u32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn u32_sign_only_proptest(i in r"[+]") {
            let result = u32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u32_trailing_digits_proptest(i in r"[+]?[0-9]{9}\D[0-9]{2}") {
            let result = u32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn i32_invalid_proptest(i in r"[+-]?[0-9]{9}\D") {
            let result = i32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn i32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}\D") {
            let result = i32::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i32_underflow_proptest(i in r"-[1-9][0-9]{10}\D") {
            let result = i32::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i32_double_sign_proptest(i in r"[+-]{2}[0-9]{9}") {
            let result = i32::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn i32_sign_only_proptest(i in r"[+-]") {
            let result = i32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i32_trailing_digits_proptest(i in r"[+-]?[0-9]{9}\D[0-9]{2}") {
            let result = i32::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn u64_invalid_proptest(i in r"[+]?[0-9]{19}\D") {
            let result = u64::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 19 || error.index == 20);
        }

        #[test]
        fn u64_overflow_proptest(i in r"[+]?[1-9][0-9]{21}\D") {
            let result = u64::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u64_negative_proptest(i in r"[-][1-9][0-9]{21}") {
            let result = u64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u64_double_sign_proptest(i in r"[+]{2}[0-9]{19}") {
            let result = u64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn u64_sign_only_proptest(i in r"[+]") {
            let result = u64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u64_trailing_digits_proptest(i in r"[+]?[0-9]{19}\D[0-9]{2}") {
            let result = u64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 19 || error.index == 20);
        }

        #[test]
        fn i64_invalid_proptest(i in r"[+-]?[0-9]{18}\D") {
            let result = i64::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 18 || error.index == 19);
        }

        #[test]
        fn i64_overflow_proptest(i in r"[+]?[1-9][0-9]{19}\D") {
            let result = i64::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i64_underflow_proptest(i in r"-[1-9][0-9]{19}\D") {
            let result = i64::from_lexical(i.as_bytes());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i64_double_sign_proptest(i in r"[+-]{2}[0-9]{18}") {
            let result = i64::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 1);
        }

        #[test]
        fn i64_sign_only_proptest(i in r"[+-]") {
            let result = i32::from_lexical(i.as_bytes());
            prop_assert!(result.is_err());
            let code = result.err().unwrap().code;
            prop_assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i64_trailing_digits_proptest(i in r"[+-]?[0-9]{18}\D[0-9]{2}") {
            let result = i64::from_lexical(i.as_bytes());
            let error = result.err().unwrap();
            prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
            prop_assert!(error.index == 18 || error.index == 19);
        }
    }
}
