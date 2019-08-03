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
//  | i8    | 112,152            | 115,147               | 1.03x             |
//  | i16   | 153,670            | 150,231               | 0.98x             |
//  | i32   | 202,512            | 204,880               | 1.01x             |
//  | i64   | 309,731            | 309,584               | 1.00x             |
//
//  # Raw Benchmarks
//
//  ```text
//  test atoi_u8_lexical  ... bench:      75,622 ns/iter (+/- 4,864)
//  test atoi_u8_parse    ... bench:      80,021 ns/iter (+/- 6,511)
//  test atoi_u16_lexical ... bench:      80,926 ns/iter (+/- 3,328)
//  test atoi_u16_parse   ... bench:      82,185 ns/iter (+/- 2,721)
//  test atoi_u32_lexical ... bench:     131,221 ns/iter (+/- 5,266)
//  test atoi_u32_parse   ... bench:     148,231 ns/iter (+/- 3,812)
//  test atoi_u64_lexical ... bench:     243,315 ns/iter (+/- 9,726)
//  test atoi_u64_parse   ... bench:     296,713 ns/iter (+/- 8,321)
//  test atoi_i8_lexical  ... bench:     112,152 ns/iter (+/- 4,527)
//  test atoi_i8_parse    ... bench:     115,147 ns/iter (+/- 3,190)
//  test atoi_i16_lexical ... bench:     153,670 ns/iter (+/- 9,993)
//  test atoi_i16_parse   ... bench:     150,231 ns/iter (+/- 3,934)
//  test atoi_i32_lexical ... bench:     202,512 ns/iter (+/- 18,486)
//  test atoi_i32_parse   ... bench:     204,880 ns/iter (+/- 8,278)
//  test atoi_i64_lexical ... bench:     309,731 ns/iter (+/- 22,313)
//  test atoi_i64_parse   ... bench:     309,584 ns/iter (+/- 7,578)
//  ```
//
// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([75622, 80926, 131221, 243315, 112152, 153670, 202512, 309731]) / 1e6
//  rustcore = np.array([80021, 82185, 148231, 296713, 115147, 150231, 204880, 309584]) / 1e6
//  index = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
//  df = pd.DataFrame({'lexical': lexical, 'rustcore': rustcore}, index = index, columns=['lexical', 'rustcore'])
//  ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14, color=['#E24A33', '#348ABD'])
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  ax.legend(loc=2, prop={'size': 14})
//  plt.show()

use util::*;
use lib::result::Result as StdResult;

// Calculate the offset of a reference inside an array.
macro_rules! offset {
    ($arr:ident, $v:ident) => (
        distance($arr.as_ptr(), $v)
    );
}

macro_rules! to_digit {
    ($c:expr, $radix:ident) => (($c as char).to_digit($radix));
}

// STANDALONE

/// Iterate over the digits and iteratively process them.
macro_rules! standalone {
    ($value:ident, $radix:ident, $digits:ident, $offset:expr, $op:ident, $code:ident) => (
        let index = move | c: &u8 | -> usize { offset!($digits, c) + $offset };
        for c in $digits.iter() {
            let digit = match to_digit!(*c, $radix) {
                Some(v) => v,
                None    => return Err((ErrorCode::InvalidDigit, index(c)).into()),
            };
            $value = match $value.checked_mul(as_cast($radix)) {
                Some(v) => v,
                None    => return Err((ErrorCode::$code, index(c)).into()),
            };
            $value = match $value.$op(as_cast(digit)) {
                Some(v) => v,
                None    => return Err((ErrorCode::$code, index(c)).into()),
            };
        }
    );
}

// Standalone atoi processor.
perftools_inline!{
pub(crate) fn standalone<T>(radix: u32, bytes: &[u8], is_signed: bool)
    -> Result<T>
    where T: Integer
{
    // Filter out empty inputs.
    if bytes.is_empty() {
        return Err(ErrorCode::Empty.into());
    }

    let (sign, offset, digits) = match index!(bytes[0]) {
        b'+'              => (Sign::Positive, 1, &index!(bytes[1..])),
        b'-' if is_signed => (Sign::Negative, 1, &index!(bytes[1..])),
        _                  => (Sign::Positive, 0, bytes),
    };

    // Filter out empty inputs.
    if digits.is_empty() {
        return Err(ErrorCode::Empty.into());
    }

    // Parse the integer.
    let mut value = T::ZERO;
    if sign == Sign::Positive {
        standalone!(value, radix, digits, offset, checked_add, Overflow);
    } else {
        standalone!(value, radix, digits, offset, checked_sub, Underflow);
    }
    Ok(value)
}}

// Convert character to digit.
perftools_inline!{
fn to_digit<'a>(c: &'a u8, radix: u32) -> StdResult<u32, &'a u8> {
    match to_digit!(*c, radix) {
        Some(v) => Ok(v),
        None    => Err(c),
    }
}}

// Add digit to mantissa.
perftools_inline!{
#[cfg(feature = "correct")]
fn add_digit<T>(value: T, digit: u32, radix: u32)
    -> Option<T>
    where T: UnsignedInteger
{
    value
        .checked_mul(as_cast(radix))
        .and_then(|v| v.checked_add(as_cast(digit)))
}}

/// Iterate over iterator and get if truncated.
macro_rules! add_truncated {
    ($iter:ident, $truncated:ident, $radix:ident) => (
        for c in $iter {
            $truncated += 1;
            if to_digit!(*c, $radix).is_none() {
                return Err(c);
            }
        }
    );
}

// Calculate the mantissa and the number of truncated digits from a digits iterator.
perftools_inline!{
#[cfg(feature = "correct")]
pub(crate) fn standalone_mantissa<'a, T>(radix: u32, integer: &'a [u8], fraction: &'a [u8])
    -> StdResult<(T, usize), &'a u8>
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
        value = match add_digit(value, to_digit(c, radix)?, radix) {
            Some(v) => v,
            None    => {
                let mut truncated: usize = 1;
                add_truncated!(integer_iter, truncated, radix);
                add_truncated!(fraction_iter, truncated, radix);
                return Ok((value, truncated));
            },
        };
    }
    while let Some(c) = fraction_iter.next() {
        value = match add_digit(value, to_digit(c, radix)?, radix) {
            Some(v) => v,
            None    => {
                let mut truncated: usize = 1;
                add_truncated!(fraction_iter, truncated, radix);
                return Ok((value, truncated));
            },
        };
    }
    Ok((value, 0))
}}

// Calculate the mantissa when it cannot have sign or other invalid digits..
perftools_inline!{
#[cfg(not(feature = "correct"))]
pub(crate) fn standalone_mantissa<T>(radix: u32, bytes: &[u8])
    -> Result<T>
    where T: Integer
{
    // Parse the integer.
    let mut value = T::ZERO;
    standalone!(value, radix, bytes, 0, checked_add, Overflow);
    Ok(value)
}}

/// Iterate over the digits and iteratively process them.
macro_rules! standalone_exponent {
    ($value:ident, $radix:ident, $digits:ident, $op:ident, $default:expr) => (
        let mut iter = $digits.iter();
        while let Some(c) = iter.next() {
            let digit = to_digit(c, $radix)?;
            $value = match $value.checked_mul(as_cast($radix)).and_then(|v| v.$op(as_cast(digit))) {
                Some(v) => v,
                None    => {
                    // Consume rthe rest of the iterator to validate
                    // the remaining data.
                    for c in iter {
                        if to_digit!(*c, $radix).is_none() {
                            return Err(c);
                        }
                    }
                    return Ok($default);
                },
            };
        }
    );
}

// Specialized parser for the exponent, which validates digits and
// returns a default min or max value on overflow.
perftools_inline!{
pub(crate) fn standalone_exponent<'a>(radix: u32, bytes: &'a [u8])
    -> StdResult<i32, &'a u8>
{
    let (sign, digits) = match index!(bytes[0]) {
        b'+'              => (Sign::Positive, &index!(bytes[1..])),
        b'-'              => (Sign::Negative, &index!(bytes[1..])),
        _                 => (Sign::Positive, bytes),
    };

    // Filter out empty inputs.
    if digits.is_empty() {
        return Err(&index!(bytes[0]));
    }

    // Parse the integer.
    let mut value = 0;
    if sign == Sign::Positive {
        standalone_exponent!(value, radix, digits, checked_add, i32::max_value());
    } else {
        standalone_exponent!(value, radix, digits, checked_sub, i32::min_value());
    }
    Ok(value)
}}


// Handle unsigned +/- numbers and forward to implied implementation.
//  Can just use local namespace
perftools_inline!{
pub(crate) fn standalone_unsigned<'a, T>(radix: u32, bytes: &'a [u8])
    -> Result<T>
    where T: UnsignedInteger
{
    standalone(radix, bytes, false)
}}

// Handle signed +/- numbers and forward to implied implementation.
//  Can just use local namespace
perftools_inline!{
pub(crate) fn standalone_signed<'a, T>(radix: u32, bytes: &'a [u8])
    -> Result<T>
    where T: SignedInteger
{
    standalone(radix, bytes, true)
}}

// API
// ---

// RANGE API (FFI)
generate_from_range_api!(atou8_range, atou8_radix_range, u8, standalone_unsigned);
generate_from_range_api!(atou16_range, atou16_radix_range, u16, standalone_unsigned);
generate_from_range_api!(atou32_range, atou32_radix_range, u32, standalone_unsigned);
generate_from_range_api!(atou64_range, atou64_radix_range, u64, standalone_unsigned);
generate_from_range_api!(atousize_range, atousize_radix_range, usize, standalone_unsigned);
generate_from_range_api!(atoi8_range, atoi8_radix_range, i8, standalone_signed);
generate_from_range_api!(atoi16_range, atoi16_radix_range, i16, standalone_signed);
generate_from_range_api!(atoi32_range, atoi32_radix_range, i32, standalone_signed);
generate_from_range_api!(atoi64_range, atoi64_radix_range, i64, standalone_signed);
generate_from_range_api!(atoisize_range, atoisize_radix_range, isize, standalone_signed);

#[cfg(has_i128)] generate_from_range_api!(atou128_range, atou128_radix_range, u128, standalone_unsigned);
#[cfg(has_i128)] generate_from_range_api!(atoi128_range, atoi128_radix_range, i128, standalone_signed);

// SLICE API
generate_from_slice_api!(atou8_slice, atou8_radix_slice, u8, standalone_unsigned);
generate_from_slice_api!(atou16_slice, atou16_radix_slice, u16, standalone_unsigned);
generate_from_slice_api!(atou32_slice, atou32_radix_slice, u32, standalone_unsigned);
generate_from_slice_api!(atou64_slice, atou64_radix_slice, u64, standalone_unsigned);
generate_from_slice_api!(atousize_slice, atousize_radix_slice, usize, standalone_unsigned);
generate_from_slice_api!(atoi8_slice, atoi8_radix_slice, i8, standalone_signed);
generate_from_slice_api!(atoi16_slice, atoi16_radix_slice, i16, standalone_signed);
generate_from_slice_api!(atoi32_slice, atoi32_radix_slice, i32, standalone_signed);
generate_from_slice_api!(atoi64_slice, atoi64_radix_slice, i64, standalone_signed);
generate_from_slice_api!(atoisize_slice, atoisize_radix_slice, isize, standalone_signed);

#[cfg(has_i128)] generate_from_slice_api!(atou128_slice, atou128_radix_slice, u128, standalone_unsigned);
#[cfg(has_i128)] generate_from_slice_api!(atoi128_slice, atoi128_radix_slice, i128, standalone_signed);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

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
    fn atou8_base10_test() {
        assert_eq!(Ok(0), atou8_slice(b"0"));
        assert_eq!(Ok(127), atou8_slice(b"127"));
        assert_eq!(Ok(128), atou8_slice(b"128"));
        assert_eq!(Ok(255), atou8_slice(b"255"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), atou8_slice(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atou8_slice(b"1a"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atou8_basen_test() {
        for (b, s) in DATA.iter() {
            assert_eq!(atou8_radix_slice(*b, s.as_bytes()), Ok(37));
        }
    }

    #[test]
    fn atoi8_base10_test() {
        assert_eq!(Ok(0), atoi8_slice(b"0"));
        assert_eq!(Ok(127), atoi8_slice(b"127"));
        assert_eq!(Err((ErrorCode::Overflow, 2).into()), atoi8_slice(b"128"));
        assert_eq!(Err((ErrorCode::Overflow, 2).into()), atoi8_slice(b"255"));
        assert_eq!(Ok(-1), atoi8_slice(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atoi8_slice(b"1a"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atoi8_basen_test() {
        for (b, s) in DATA.iter() {
            assert_eq!(atoi8_radix_slice(*b, s.as_bytes()), Ok(37));
        }
    }

    #[test]
    fn atou16_base10_test() {
        assert_eq!(Ok(0), atou16_slice(b"0"));
        assert_eq!(Ok(32767), atou16_slice(b"32767"));
        assert_eq!(Ok(32768), atou16_slice(b"32768"));
        assert_eq!(Ok(65535), atou16_slice(b"65535"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), atou16_slice(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atou16_slice(b"1a"));
    }

    #[test]
    fn atoi16_base10_test() {
        assert_eq!(Ok(0), atoi16_slice(b"0"));
        assert_eq!(Ok(32767), atoi16_slice(b"32767"));
        assert_eq!(Err((ErrorCode::Overflow, 4).into()), atoi16_slice(b"32768"));
        assert_eq!(Err((ErrorCode::Overflow, 4).into()), atoi16_slice(b"65535"));
        assert_eq!(Ok(-1), atoi16_slice(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atoi16_slice(b"1a"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atoi16_basen_test() {
        for (b, s) in DATA.iter() {
            assert_eq!(atoi16_radix_slice(*b, s.as_bytes()), Ok(37));
        }
        assert_eq!(atoi16_radix_slice(36, b"YA"), Ok(1234));
    }

    #[test]
    fn atou32_base10_test() {
        assert_eq!(Ok(0), atou32_slice(b"0"));
        assert_eq!(Ok(2147483647), atou32_slice(b"2147483647"));
        assert_eq!(Ok(2147483648), atou32_slice(b"2147483648"));
        assert_eq!(Ok(4294967295), atou32_slice(b"4294967295"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), atou32_slice(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atou32_slice(b"1a"));
    }

    #[test]
    fn atoi32_base10_test() {
        assert_eq!(Ok(0), atoi32_slice(b"0"));
        assert_eq!(Ok(2147483647), atoi32_slice(b"2147483647"));
        assert_eq!(Err((ErrorCode::Overflow, 9).into()), atoi32_slice(b"2147483648"));
        assert_eq!(Err((ErrorCode::Overflow, 9).into()), atoi32_slice(b"4294967295"));
        assert_eq!(Ok(-1), atoi32_slice(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atoi32_slice(b"1a"));
    }

    #[test]
    fn atou64_base10_test() {
        assert_eq!(Ok(0), atou64_slice(b"0"));
        assert_eq!(Ok(9223372036854775807), atou64_slice(b"9223372036854775807"));
        assert_eq!(Ok(9223372036854775808), atou64_slice(b"9223372036854775808"));
        assert_eq!(Ok(18446744073709551615), atou64_slice(b"18446744073709551615"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), atou64_slice(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atou64_slice(b"1a"));
    }

    #[test]
    fn atoi64_base10_test() {
        assert_eq!(Ok(0), atoi64_slice(b"0"));
        assert_eq!(Ok(9223372036854775807), atoi64_slice(b"9223372036854775807"));
        assert_eq!(Err((ErrorCode::Overflow, 18).into()), atoi64_slice(b"9223372036854775808"));
        assert_eq!(Err((ErrorCode::Overflow, 19).into()), atoi64_slice(b"18446744073709551615"));
        assert_eq!(Ok(-1), atoi64_slice(b"-1"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atoi64_slice(b"1a"));

        // Add tests discovered via fuzzing.
        assert_eq!(Err((ErrorCode::Overflow, 19).into()), atoi64_slice(b"406260572150672006000066000000060060007667760000000000000000000+00000006766767766666767665670000000000000000000000666"));
    }

    proptest! {
        #[test]
        fn u8_invalid_proptest(i in r"[+]?[0-9]{2}\D") {
            let result = atou8_slice(i.as_bytes());
            assert!(result.is_err());
            let index = result.err().unwrap().index;
            assert!(index == 2 || index == 3);
        }

        #[test]
        fn u8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}") {
            let result = atou8_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u8_negative_proptest(i in r"[-][1-9][0-9]{2}") {
            let result = atou8_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u8_double_sign_proptest(i in r"[+]{2}[0-9]{2}") {
            let result = atou8_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 1);
        }

        #[test]
        fn u8_sign_only_proptest(i in r"[+]") {
            let result = atou8_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u8_trailing_digits_proptest(i in r"[+]?[0-9]{2}\D[0-9]{2}") {
            let result = atou8_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn i8_invalid_proptest(i in r"[+-]?[0-9]{2}\D") {
            let result = atoi8_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn i8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}\D") {
            let result = atoi8_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i8_underflow_proptest(i in r"[-][1-9][0-9]{3}\D") {
            let result = atoi8_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i8_double_sign_proptest(i in r"[+-]{2}[0-9]{2}") {
            let result = atoi8_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 1);
        }

        #[test]
        fn i8_sign_only_proptest(i in r"[+-]") {
            let result = atoi8_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::Empty);
        }

        #[test]
        fn i8_trailing_digits_proptest(i in r"[+-]?[0-9]{2}\D[0-9]{2}") {
            let result = atoi8_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 2 || error.index == 3);
        }

        #[test]
        fn u16_invalid_proptest(i in r"[+]?[0-9]{4}\D") {
            let result = atou16_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn u16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}\D") {
            let result = atou16_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u16_negative_proptest(i in r"[-][1-9][0-9]{4}") {
            let result = atou16_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u16_double_sign_proptest(i in r"[+]{2}[0-9]{4}") {
            let result = atou16_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 1);
        }

        #[test]
        fn u16_sign_only_proptest(i in r"[+]") {
            let result = atou16_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u16_trailing_digits_proptest(i in r"[+]?[0-9]{4}\D[0-9]{2}") {
            let result = atou16_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn i16_invalid_proptest(i in r"[+-]?[0-9]{4}\D") {
            let result = atoi16_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn i16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}\D") {
            let result = atoi16_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i16_underflow_proptest(i in r"[-][1-9][0-9]{5}\DD") {
            let result = atoi16_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i16_double_sign_proptest(i in r"[+-]{2}[0-9]{4}") {
            let result = atoi16_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 1);
        }

        #[test]
        fn i16_sign_only_proptest(i in r"[+-]") {
            let result = atoi16_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i16_trailing_digits_proptest(i in r"[+-]?[0-9]{4}\D[0-9]{2}") {
            let result = atoi16_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 4 || error.index == 5);
        }

        #[test]
        fn u32_invalid_proptest(i in r"[+]?[0-9]{9}\D") {
            let result = atou32_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn u32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}\D") {
            let result = atou32_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u32_negative_proptest(i in r"[-][1-9][0-9]{9}") {
            let result = atou32_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u32_double_sign_proptest(i in r"[+]{2}[0-9]{9}") {
            let result = atou32_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 1);
        }

        #[test]
        fn u32_sign_only_proptest(i in r"[+]") {
            let result = atou32_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u32_trailing_digits_proptest(i in r"[+]?[0-9]{9}\D[0-9]{2}") {
            let result = atou32_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn i32_invalid_proptest(i in r"[+-]?[0-9]{9}\D") {
            let result = atoi32_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn i32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}\D") {
            let result = atoi32_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i32_underflow_proptest(i in r"-[1-9][0-9]{10}\D") {
            let result = atoi32_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i32_double_sign_proptest(i in r"[+-]{2}[0-9]{9}") {
            let result = atoi32_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 1);
        }

        #[test]
        fn i32_sign_only_proptest(i in r"[+-]") {
            let result = atoi32_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i32_trailing_digits_proptest(i in r"[+-]?[0-9]{9}\D[0-9]{2}") {
            let result = atoi32_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 9 || error.index == 10);
        }

        #[test]
        fn u64_invalid_proptest(i in r"[+]?[0-9]{19}\D") {
            let result = atou64_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 19 || error.index == 20);
        }

        #[test]
        fn u64_overflow_proptest(i in r"[+]?[1-9][0-9]{21}\D") {
            let result = atou64_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn u64_negative_proptest(i in r"[-][1-9][0-9]{21}") {
            let result = atou64_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn u64_double_sign_proptest(i in r"[+]{2}[0-9]{19}") {
            let result = atou64_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 1);
        }

        #[test]
        fn u64_sign_only_proptest(i in r"[+]") {
            let result = atou64_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn u64_trailing_digits_proptest(i in r"[+]?[0-9]{19}\D[0-9]{2}") {
            let result = atou64_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 19 || error.index == 20);
        }

        #[test]
        fn i64_invalid_proptest(i in r"[+-]?[0-9]{18}\D") {
            let result = atoi64_slice(i.as_bytes());
            assert!(result.is_err());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 18 || error.index == 19);
        }

        #[test]
        fn i64_overflow_proptest(i in r"[+]?[1-9][0-9]{19}\D") {
            let result = atoi64_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Overflow);
        }

        #[test]
        fn i64_underflow_proptest(i in r"-[1-9][0-9]{19}\D") {
            let result = atoi64_slice(i.as_bytes());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Underflow);
        }

        #[test]
        fn i64_double_sign_proptest(i in r"[+-]{2}[0-9]{18}") {
            let result = atoi64_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 1);
        }

        #[test]
        fn i64_sign_only_proptest(i in r"[+-]") {
            let result = atoi32_slice(i.as_bytes());
            assert!(result.is_err());
            let code = result.err().unwrap().code;
            assert_eq!(code, ErrorCode::Empty);
        }

        #[test]
        fn i64_trailing_digits_proptest(i in r"[+-]?[0-9]{18}\D[0-9]{2}") {
            let result = atoi64_slice(i.as_bytes());
            let error = result.err().unwrap();
            assert_eq!(error.code, ErrorCode::InvalidDigit);
            assert!(error.index == 18 || error.index == 19);
        }
    }
}
