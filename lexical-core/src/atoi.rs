//! Fast lexical string-to-integer conversion routines.
//!
//! These routines are wrapping, and therefore can accept any buffer for any
//! size type, but will wrap to the desired value if overflow occurs.

//  The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//  CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//  (x86-64), using the lexical formatter or `x.parse()`,
//  avoiding any inefficiencies in Rust string parsing. The code was
//  compiled with LTO and at an optimization level of 3.
//
//  The benchmarks with `std` were compiled using "rustc 1.29.2 (17a9dc751
//  2018-10-05", and the `no_std` benchmarks were compiled using "rustc
//  1.31.0-nightly (46880f41b 2018-10-15)".
//
//  The benchmark code may be found `benches/atoi.rs`.
//
//  # Benchmarks
//
//  | Type  |  lexical (ns/iter) | parse (ns/iter)       | Relative Increase |
//  |:-----:|:------------------:|:---------------------:|:-----------------:|
//  | u8    | 84,745             | 77,618                | 0.92x             |
//  | u16   | 74,499             | 82,272                | 1.10x             |
//  | u32   | 108,346            | 148,717               | 1.37x             |
//  | u64   | 194,103            | 297,330               | 1.53x             |
//  | i8    | 131,527            | 115,627               | 0.88x             |
//  | i16   | 132,949            | 151,612               | 1.14x             |
//  | i32   | 173,434            | 207,788               | 1.20x             |
//  | i64   | 211,412            | 311,535               | 1.47x             |
//
//  # Raw Benchmarks
//
//  ```text
//  test atoi_i8_lexical  ... bench:     131,527 ns/iter (+/- 5,839)
//  test atoi_i8_parse    ... bench:     115,627 ns/iter (+/- 3,132)
//  test atoi_i16_lexical ... bench:     132,949 ns/iter (+/- 4,467)
//  test atoi_i16_parse   ... bench:     151,612 ns/iter (+/- 6,245)
//  test atoi_i32_lexical ... bench:     173,434 ns/iter (+/- 5,765)
//  test atoi_i32_parse   ... bench:     207,788 ns/iter (+/- 7,880)
//  test atoi_i64_lexical ... bench:     211,412 ns/iter (+/- 13,169)
//  test atoi_i64_parse   ... bench:     311,535 ns/iter (+/- 19,743)
//  test atoi_u8_lexical  ... bench:      84,745 ns/iter (+/- 3,956)
//  test atoi_u8_parse    ... bench:      77,618 ns/iter (+/- 2,617)
//  test atoi_u16_lexical ... bench:      74,499 ns/iter (+/- 1,804)
//  test atoi_u16_parse   ... bench:      82,272 ns/iter (+/- 2,262)
//  test atoi_u32_lexical ... bench:     108,346 ns/iter (+/- 2,082)
//  test atoi_u32_parse   ... bench:     148,717 ns/iter (+/- 3,446)
//  test atoi_u64_lexical ... bench:     194,103 ns/iter (+/- 4,476)
//  test atoi_u64_parse   ... bench:     297,330 ns/iter (+/- 7,243)
//  ```
//
// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([84745, 74499, 108346, 194103, 131527, 132949, 173434, 211412]) / 1e6
//  rustcore = np.array([77618, 82272, 148717, 297330, 115627, 151612, 207788, 311535]) / 1e6
//  index = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
//  df = pd.DataFrame({'lexical': lexical, 'rustcore': rustcore}, index = index, columns=['lexical', 'rustcore'])
//  ax = df.plot.bar(rot=0, figsize=(16, 8), fontsize=14, color=['#E24A33', '#348ABD'])
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  ax.legend(loc=2, prop={'size': 14})
//  plt.show()

use util::*;

// ALGORITHM

/// Generate both the add and sub versions of unchecked.
macro_rules! unchecked {
    ($func:ident, $op:ident) => (
        /// Returns the number of parsed bytes and the index where the input was
        /// truncated at.
        ///
        /// Don't trim leading zeros, since the value may be non-zero and
        /// therefore invalid.
        #[inline]
        pub(crate) fn $func<'a, T>(value: &mut T, radix: T, bytes: &'a [u8])
            -> (usize, Option<&'a u8>)
            where T: Integer
        {
            // Continue while we have digits.
            // Don't check for overflow, we want to avoid as many conditions
            // as possible, it leads to significant speed increases on x86-64.
            // Just note it happens, and continue on.
            // Don't add a short-circuit either, since it adds significant time
            // and we want to continue parsing until everything is done, since
            // otherwise it may give us invalid results elsewhere.
            let mut digit: T;
            let mut truncated = None;
            for (i, c) in bytes.iter().enumerate() {
                digit = as_cast(char_to_digit(*c));
                if digit < radix {
                    let (v, o1) = value.overflowing_mul(radix);
                    let (v, o2) = v.$op(digit);
                    *value = v;
                    if truncated.is_none() && (o1 | o2) {
                        truncated = Some(c);
                    }
                } else {
                    return (i, truncated);
                }
            }

            (bytes.len(), truncated)
        }
    );
}

unchecked!(unchecked_positive, overflowing_add);
unchecked!(unchecked_negative, overflowing_sub);

/// Unchecked callback for the string-to-integer parser.
#[allow(dead_code)]
#[inline]
pub(crate) fn unchecked<'a, T>(value: &mut T, radix: T, bytes: &'a [u8], sign: Sign)
    -> (usize, Option<&'a u8>)
    where T: Integer
{
    match sign {
        Sign::Positive => unchecked_positive(value, radix, bytes),
        Sign::Negative => unchecked_negative(value, radix, bytes),
    }
}

/// Generate both the add and sub versions of checked.
macro_rules! checked {
    ($func:ident, $op:ident) => (
        /// Returns the number of parsed bytes and the index where the input was
        /// truncated at.
        ///
        /// Don't trim leading zeros, since the value may be non-zero and
        /// therefore invalid.
        #[cfg(feature = "correct")]
        #[inline]
        pub(crate) fn $func<'a, T>(value: &mut T, radix: T, bytes: &'a [u8])
            -> (usize, Option<&'a u8>)
            where T: Integer
        {
            // Continue while we have digits.
            // Don't check for overflow, we want to avoid as many conditions
            // as possible, it leads to significant speed increases on x86-64.
            // Just note it happens, and continue on.
            // Don't add a short-circuit either, since it adds significant time
            // and we want to continue parsing until everything is done, since
            // otherwise it may give us invalid results elsewhere.
            let mut digit: T;
            let mut truncated = None;
            for (i, c) in bytes.iter().enumerate() {
                digit = as_cast(char_to_digit(*c));
                if digit < radix {
                    // Only multiply to the radix and add the parsed digit if
                    // the value hasn't overflowed yet, and only assign to the
                    // original value if the operations don't overflow.
                    if truncated.is_none() {
                        // Chain these two operations before we assign, since
                        // otherwise we get an invalid result.
                        match value.checked_mul(radix).and_then(|v| v.$op(digit)) {
                            // No overflow, assign the value.
                            Some(v) => *value = v,
                            // Overflow occurred, set truncated position
                            None    => truncated = Some(c),
                        }
                    }
                } else {
                    return (i, truncated);
                }
            }

            (bytes.len(), truncated)
        }
    );
}

checked!(checked_positive, checked_add);
checked!(checked_negative, checked_sub);

/// Checked callback for the string-to-integer parser.
#[allow(dead_code)]
#[cfg(feature = "correct")]
#[inline]
pub(crate) fn checked<'a, T>(value: &mut T, radix: T, bytes: &'a [u8], sign: Sign)
    -> (usize, Option<&'a u8>)
    where T: Integer
{
    match sign {
        Sign::Positive => checked_positive(value, radix, bytes),
        Sign::Negative => checked_negative(value, radix, bytes),
    }
}

/// Handle +/- numbers and forward to implementation.
#[inline]
pub(crate) fn filter_sign<'a, T, Cb>(radix: u32, bytes: &'a [u8], cb: Cb)
    -> (T, Sign, usize, Option<&'a u8>)
    where T: Integer,
          Cb: FnOnce(&mut T, T, &'a [u8], Sign) -> (usize, Option<&'a u8>)
{
    let (sign_bytes, sign) = match bytes.get(0) {
        Some(&b'+') => (1, Sign::Positive),
        Some(&b'-') => (1, Sign::Negative),
        _           => (0, Sign::Positive),
    };

    if bytes.len() > sign_bytes {
        // `bytes.len() > sign_bytes`, so this range is always valid.
        let bytes = &index!(bytes[sign_bytes..]);

        // Trim the leading 0s here, where we can guarantee the value is 0,
        // and therefore trimming these leading 0s is actually valid.
        let (bytes, count) = ltrim_char_slice(bytes, b'0');

        // Initialize a 0 version of our value, and invoke the low-level callback.
        let mut value: T = T::ZERO;
        let (len, truncated) = cb(&mut value, as_cast(radix), bytes, sign);
        (value, sign, sign_bytes + count + len, truncated)
    } else {
        (T::ZERO, sign, 0, None)
    }
}

/// Handle unsigned +/- numbers and forward to implied implementation.
//  Can just use local namespace
#[inline]
pub(crate) fn unsigned<'a, T, Cb>(radix: u32, bytes: &'a [u8], cb: Cb)
    -> (T, usize, bool)
    where T: UnsignedInteger,
          Cb: FnOnce(&mut T, T, &'a [u8], Sign) -> (usize, Option<&'a u8>)
{
    let (value, sign, processed, truncated) = filter_sign::<T, Cb>(radix, bytes, cb);
    match sign {
        // Need to return 0 early if we have a 0 value.
        Sign::Negative => (value, 0, truncated.is_some()),
        Sign::Positive => (value, processed, truncated.is_some()),
    }
}

/// Handle signed +/- numbers and forward to implied implementation.
//  Can just use local namespace
#[inline]
pub(crate) fn signed<'a, T, Cb>(radix: u32, bytes: &'a [u8], cb: Cb)
    -> (T, usize, bool)
    where T: SignedInteger,
          Cb: FnOnce(&mut T, T, &'a [u8], Sign) -> (usize, Option<&'a u8>)
{
    let (value, _, processed, truncated) = filter_sign::<T, Cb>(radix, bytes, cb);
    (value, processed, truncated.is_some())
}

// UNSAFE API

/// Expand the generic unsigned atoi function for specified types.
macro_rules! wrap_unsigned {
    ($func:ident, $t:tt) => (
        /// Parse unsigned integer and return value, subslice read, and if truncated.
        #[inline]
        fn $func(radix: u8, bytes: &[u8])
            -> ($t, usize, bool)
        {
            let (value, len, truncated) = unsigned::<$t, _>(radix.into(), bytes, unchecked::<$t>);
            (value, len, truncated)
        }
    )
}

wrap_unsigned!(atou8_impl, u8);
wrap_unsigned!(atou16_impl, u16);
wrap_unsigned!(atou32_impl, u32);
wrap_unsigned!(atou64_impl, u64);
wrap_unsigned!(atousize_impl, usize);

#[cfg(has_i128)]
wrap_unsigned!(atou128_impl, u128);

/// Expand the generic signed atoi function for specified types.
macro_rules! wrap_signed {
    ($func:ident, $t:tt) => (
        /// Parse signed integer and return value, subslice read, and if truncated.
        #[inline]
        fn $func(radix: u8, bytes: &[u8])
            -> ($t, usize, bool)
        {
            let (value, len, truncated) = signed::<$t, _>(radix.into(), bytes, unchecked::<$t>);
            (value, len, truncated)
        }
    )
}

wrap_signed!(atoi8_impl, i8);
wrap_signed!(atoi16_impl, i16);
wrap_signed!(atoi32_impl, i32);
wrap_signed!(atoi64_impl, i64);
wrap_signed!(atoisize_impl, isize);

#[cfg(has_i128)]
wrap_signed!(atoi128_impl, i128);

// RANGE API (FFI)
generate_from_range_api!(atou8_range, atou8_radix_range, u8, atou8_impl);
generate_from_range_api!(atou16_range, atou16_radix_range, u16, atou16_impl);
generate_from_range_api!(atou32_range, atou32_radix_range, u32, atou32_impl);
generate_from_range_api!(atou64_range, atou64_radix_range, u64, atou64_impl);
generate_from_range_api!(atousize_range, atousize_radix_range, usize, atousize_impl);
generate_from_range_api!(atoi8_range, atoi8_radix_range, i8, atoi8_impl);
generate_from_range_api!(atoi16_range, atoi16_radix_range, i16, atoi16_impl);
generate_from_range_api!(atoi32_range, atoi32_radix_range, i32, atoi32_impl);
generate_from_range_api!(atoi64_range, atoi64_radix_range, i64, atoi64_impl);
generate_from_range_api!(atoisize_range, atoisize_radix_range, isize, atoisize_impl);
generate_try_from_range_api!(try_atou8_range, try_atou8_radix_range, u8, atou8_impl);
generate_try_from_range_api!(try_atou16_range, try_atou16_radix_range, u16, atou16_impl);
generate_try_from_range_api!(try_atou32_range, try_atou32_radix_range, u32, atou32_impl);
generate_try_from_range_api!(try_atou64_range, try_atou64_radix_range, u64, atou64_impl);
generate_try_from_range_api!(try_atousize_range, try_atousize_radix_range, usize, atousize_impl);
generate_try_from_range_api!(try_atoi8_range, try_atoi8_radix_range, i8, atoi8_impl);
generate_try_from_range_api!(try_atoi16_range, try_atoi16_radix_range, i16, atoi16_impl);
generate_try_from_range_api!(try_atoi32_range, try_atoi32_radix_range, i32, atoi32_impl);
generate_try_from_range_api!(try_atoi64_range, try_atoi64_radix_range, i64, atoi64_impl);
generate_try_from_range_api!(try_atoisize_range, try_atoisize_radix_range, isize, atoisize_impl);

#[cfg(has_i128)] generate_from_range_api!(atou128_range, atou128_radix_range, u128, atou128_impl);
#[cfg(has_i128)] generate_from_range_api!(atoi128_range, atoi128_radix_range, i128, atoi128_impl);
#[cfg(has_i128)] generate_try_from_range_api!(try_atou128_range, try_atou128_radix_range, u128, atou128_impl);
#[cfg(has_i128)] generate_try_from_range_api!(try_atoi128_range, try_atoi128_radix_range, i128, atoi128_impl);

// SLICE API
generate_from_slice_api!(atou8_slice, atou8_radix_slice, u8, atou8_impl);
generate_from_slice_api!(atou16_slice, atou16_radix_slice, u16, atou16_impl);
generate_from_slice_api!(atou32_slice, atou32_radix_slice, u32, atou32_impl);
generate_from_slice_api!(atou64_slice, atou64_radix_slice, u64, atou64_impl);
generate_from_slice_api!(atousize_slice, atousize_radix_slice, usize, atousize_impl);
generate_from_slice_api!(atoi8_slice, atoi8_radix_slice, i8, atoi8_impl);
generate_from_slice_api!(atoi16_slice, atoi16_radix_slice, i16, atoi16_impl);
generate_from_slice_api!(atoi32_slice, atoi32_radix_slice, i32, atoi32_impl);
generate_from_slice_api!(atoi64_slice, atoi64_radix_slice, i64, atoi64_impl);
generate_from_slice_api!(atoisize_slice, atoisize_radix_slice, isize, atoisize_impl);
generate_try_from_slice_api!(try_atou8_slice, try_atou8_radix_slice, u8, atou8_impl);
generate_try_from_slice_api!(try_atou16_slice, try_atou16_radix_slice, u16, atou16_impl);
generate_try_from_slice_api!(try_atou32_slice, try_atou32_radix_slice, u32, atou32_impl);
generate_try_from_slice_api!(try_atou64_slice, try_atou64_radix_slice, u64, atou64_impl);
generate_try_from_slice_api!(try_atousize_slice, try_atousize_radix_slice, usize, atousize_impl);
generate_try_from_slice_api!(try_atoi8_slice, try_atoi8_radix_slice, i8, atoi8_impl);
generate_try_from_slice_api!(try_atoi16_slice, try_atoi16_radix_slice, i16, atoi16_impl);
generate_try_from_slice_api!(try_atoi32_slice, try_atoi32_radix_slice, i32, atoi32_impl);
generate_try_from_slice_api!(try_atoi64_slice, try_atoi64_radix_slice, i64, atoi64_impl);
generate_try_from_slice_api!(try_atoisize_slice, try_atoisize_radix_slice, isize, atoisize_impl);

#[cfg(has_i128)] generate_from_slice_api!(atou128_slice, atou128_radix_slice, u128, atou128_impl);
#[cfg(has_i128)] generate_from_slice_api!(atoi128_slice, atoi128_radix_slice, i128, atoi128_impl);
#[cfg(has_i128)] generate_try_from_slice_api!(try_atou128_slice, try_atou128_radix_slice, u128, atou128_impl);
#[cfg(has_i128)] generate_try_from_slice_api!(try_atoi128_slice, try_atoi128_radix_slice, i128, atoi128_impl);

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

    #[cfg(feature = "correct")]
    #[test]
    fn checked_test() {
        let s = "1234567891234567890123";
        let mut value: u64 = 0;
        let (processed, truncated) = checked_positive(&mut value, 10, s.as_bytes());
        // check it doesn't overflow
        assert_eq!(value, 12345678912345678901);
        assert_eq!(processed, s.len());
        assert_eq!(distance(s.as_ptr(), truncated.unwrap()), s.len()-2);
    }

    #[test]
    fn unchecked_test() {
        let s = "1234567891234567890123";
        let mut value: u64 = 0;
        let (processed, truncated) = unchecked_positive(&mut value, 10, s.as_bytes());
        // check it does overflow
        assert_eq!(value, 17082782369737483467);
        assert_eq!(processed, s.len());
        assert_eq!(distance(s.as_ptr(), truncated.unwrap()), s.len()-2);
    }

    #[test]
    fn atou8_base10_test() {
        assert_eq!(0, atou8_slice(b"0"));
        assert_eq!(127, atou8_slice(b"127"));
        assert_eq!(128, atou8_slice(b"128"));
        assert_eq!(255, atou8_slice(b"255"));
        assert_eq!(255, atou8_slice(b"-1"));
        assert_eq!(1, atou8_slice(b"1a"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atou8_basen_test() {
        for (b, s) in DATA.iter() {
            assert_eq!(atou8_radix_slice(*b, s.as_bytes()), 37);
        }
    }

    #[test]
    fn atoi8_base10_test() {
        assert_eq!(0, atoi8_slice(b"0"));
        assert_eq!(127, atoi8_slice(b"127"));
        assert_eq!(-128, atoi8_slice(b"128"));
        assert_eq!(-1, atoi8_slice(b"255"));
        assert_eq!(-1, atoi8_slice(b"-1"));
        assert_eq!(1, atoi8_slice(b"1a"));
    }

    #[test]
    fn atou16_base10_test() {
        assert_eq!(0, atou16_slice(b"0"));
        assert_eq!(32767, atou16_slice(b"32767"));
        assert_eq!(32768, atou16_slice(b"32768"));
        assert_eq!(65535, atou16_slice(b"65535"));
        assert_eq!(65535, atou16_slice(b"-1"));
        assert_eq!(1, atou16_slice(b"1a"));
    }

    #[test]
    fn atoi16_base10_test() {
        assert_eq!(0, atoi16_slice(b"0"));
        assert_eq!(32767, atoi16_slice(b"32767"));
        assert_eq!(-32768, atoi16_slice(b"32768"));
        assert_eq!(-1, atoi16_slice(b"65535"));
        assert_eq!(-1, atoi16_slice(b"-1"));
        assert_eq!(1, atoi16_slice(b"1a"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atoi16_basen_test() {
        assert_eq!(atoi16_radix_slice(36, b"YA"), 1234);
    }

    #[test]
    fn atou32_base10_test() {
        assert_eq!(0, atou32_slice(b"0"));
        assert_eq!(2147483647, atou32_slice(b"2147483647"));
        assert_eq!(2147483648, atou32_slice(b"2147483648"));
        assert_eq!(4294967295, atou32_slice(b"4294967295"));
        assert_eq!(4294967295, atou32_slice(b"-1"));
        assert_eq!(1, atou32_slice(b"1a"));
    }

    #[test]
    fn atoi32_base10_test() {
        assert_eq!(0, atoi32_slice(b"0"));
        assert_eq!(2147483647, atoi32_slice(b"2147483647"));
        assert_eq!(-2147483648, atoi32_slice(b"2147483648"));
        assert_eq!(-1, atoi32_slice(b"4294967295"));
        assert_eq!(-1, atoi32_slice(b"-1"));
        assert_eq!(1, atoi32_slice(b"1a"));
    }

    #[test]
    fn atou64_base10_test() {
        assert_eq!(0, atou64_slice(b"0"));
        assert_eq!(9223372036854775807, atou64_slice(b"9223372036854775807"));
        assert_eq!(9223372036854775808, atou64_slice(b"9223372036854775808"));
        assert_eq!(18446744073709551615, atou64_slice(b"18446744073709551615"));
        assert_eq!(18446744073709551615, atou64_slice(b"-1"));
        assert_eq!(1, atou64_slice(b"1a"));
    }

    #[test]
    fn atoi64_base10_test() {
        assert_eq!(0, atoi64_slice(b"0"));
        assert_eq!(9223372036854775807, atoi64_slice(b"9223372036854775807"));
        assert_eq!(-9223372036854775808, atoi64_slice(b"9223372036854775808"));
        assert_eq!(-1, atoi64_slice(b"18446744073709551615"));
        assert_eq!(-1, atoi64_slice(b"-1"));
        assert_eq!(1, atoi64_slice(b"1a"));

        // Add tests discovered via fuzzing.
        assert_eq!(2090691195633139712, atoi64_slice(b"406260572150672006000066000000060060007667760000000000000000000+00000006766767766666767665670000000000000000000000666"));
    }
    #[test]
    fn try_atou8_base10_test() {
        assert_eq!(empty_error(0), try_atou8_slice(b""));
        assert_eq!(success(0), try_atou8_slice(b"0"));
        assert_eq!(invalid_digit_error(1, 1), try_atou8_slice(b"1a"));
        assert_eq!(overflow_error(0), try_atou8_slice(b"256"));

        // Add tests discovered via proptests.
        assert_eq!(invalid_digit_error(0, 0), try_atou8_slice(b"-+00"));
    }

    #[test]
    fn try_atoi8_base10_test() {
        assert_eq!(empty_error(0), try_atoi8_slice(b""));
        assert_eq!(success(0), try_atoi8_slice(b"0"));
        assert_eq!(invalid_digit_error(1, 1), try_atoi8_slice(b"1a"));
        assert_eq!(success(-128), try_atoi8_slice(b"-128"));
        assert_eq!(overflow_error(-128), try_atoi8_slice(b"128"));
    }

    #[test]
    fn try_atou16_base10_test() {
        assert_eq!(empty_error(0), try_atou16_slice(b""));
        assert_eq!(success(0), try_atou16_slice(b"0"));
        assert_eq!(invalid_digit_error(1, 1), try_atou16_slice(b"1a"));
        assert_eq!(overflow_error(0), try_atou16_slice(b"65536"));
    }

    #[test]
    fn try_atoi16_base10_test() {
        assert_eq!(empty_error(0), try_atoi16_slice(b""));
        assert_eq!(success(0), try_atoi16_slice(b"0"));
        assert_eq!(invalid_digit_error(1, 1), try_atoi16_slice(b"1a"));
        assert_eq!(success(-32768), try_atoi16_slice(b"-32768"));
        assert_eq!(overflow_error(-32768), try_atoi16_slice(b"32768"));
    }

    #[test]
    fn try_atou32_base10_test() {
        assert_eq!(empty_error(0), try_atou32_slice(b""));
        assert_eq!(success(0), try_atou32_slice(b"0"));
        assert_eq!(invalid_digit_error(1, 1), try_atou32_slice(b"1a"));
        assert_eq!(overflow_error(0), try_atou32_slice(b"4294967296"));
    }

    #[test]
    fn try_atoi32_base10_test() {
        assert_eq!(empty_error(0), try_atoi32_slice(b""));
        assert_eq!(success(0), try_atoi32_slice(b"0"));
        assert_eq!(invalid_digit_error(1, 1), try_atoi32_slice(b"1a"));
        assert_eq!(success(-2147483648), try_atoi32_slice(b"-2147483648"));
        assert_eq!(overflow_error(-2147483648), try_atoi32_slice(b"2147483648"));
    }

    #[test]
    fn try_atou64_base10_test() {
        assert_eq!(empty_error(0), try_atou64_slice(b""));
        assert_eq!(success(0), try_atou64_slice(b"0"));
        assert_eq!(invalid_digit_error(1, 1), try_atou64_slice(b"1a"));
        assert_eq!(overflow_error(0), try_atou64_slice(b"18446744073709551616"));
    }

    #[test]
    fn try_atoi64_base10_test() {
        assert_eq!(empty_error(0), try_atoi64_slice(b""));
        assert_eq!(success(0), try_atoi64_slice(b"0"));
        assert_eq!(invalid_digit_error(1, 1), try_atoi64_slice(b"1a"));
        assert_eq!(overflow_error(-9223372036854775808), try_atoi64_slice(b"9223372036854775808"));

        // Check overflow and invalid digits, overflow should take precedence.
        assert_eq!(success(-9223372036854775808), try_atoi64_slice(b"-9223372036854775808"));
        assert_eq!(overflow_error(-9223372036854775808), try_atoi64_slice(b"9223372036854775808abc"));

        // Add tests discovered via fuzzing.
        assert_eq!(overflow_error(-9223372036854775808), try_atoi64_slice(b"-000000000000000000000000066000000000000000000000000000000000000000000695092744062605721500000000695092744062600000000000000000000000000000000000000000000000000000000000000?0000000000000000000000000000000000000000000000000\x100000000006666600000000006000000066666666000766776676677000676766509274406260572150000000069509274406260572150000000000000000000000000000000000066000000000000000000000000000000000000000000600000950927440626057215000000006950927440062600057215000000666600666666666600001000000676766766766770000666000766776676000000000000000000000000006950927440626666676676676676660066666000000000060000000600000000000000000000000000000000000+?676677000695092744"));
        assert_eq!(overflow_error(2090691195633139712), try_atoi64_slice(b"406260572150672006000066000000060060007667760000000000000000000+00000006766767766666767665670000000000000000000000666"));
        assert_eq!(overflow_error(7125759012462002176), try_atoi64_slice(b"6260572000000000000000-3*+\x006666600099000066006660066665?666666666599990000666"));
    }

    proptest! {
        #[test]
        fn u8_invalid_proptest(i in r"[+]?[0-9]{2}\D") {
            let res = try_atou8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 2 || res.error.index == 3);
        }

        #[test]
        fn u8_overflow_proptest(i in r"[+-]?[1-9][0-9]{3}\D") {
            let res = try_atou8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::Overflow);
        }

        #[test]
        fn u8_double_sign_proptest(i in r"[+-]{2}[0-9]{2}") {
            let res = try_atou8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0 || res.error.index == 1);
        }

        #[test]
        fn u8_sign_only_proptest(i in r"[+-]") {
            let res = try_atou8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0);
        }

        #[test]
        fn u8_trailing_digits_proptest(i in r"[+]?[0-9]{2}\D[0-9]{2}") {
            let res = try_atou8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 2 || res.error.index == 3);
        }

        #[test]
        fn i8_invalid_proptest(i in r"[+-]?[0-9]{2}\D") {
            let res = try_atoi8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 2 || res.error.index == 3);
        }

        #[test]
        fn i8_overflow_proptest(i in r"[+-]?[1-9][0-9]{3}\D") {
            let res = try_atoi8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::Overflow);
        }

        #[test]
        fn i8_double_sign_proptest(i in r"[+-]{2}[0-9]{2}") {
            let res = try_atoi8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 1);
        }

        #[test]
        fn i8_sign_only_proptest(i in r"[+-]") {
            let res = try_atoi8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0);
        }

        #[test]
        fn i8_trailing_digits_proptest(i in r"[+-]?[0-9]{2}\D[0-9]{2}") {
            let res = try_atoi8_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 2 || res.error.index == 3);
        }

        #[test]
        fn u16_invalid_proptest(i in r"[+]?[0-9]{4}\D") {
            let res = try_atou16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 4 || res.error.index == 5);
        }

        #[test]
        fn u16_overflow_proptest(i in r"[+-]?[1-9][0-9]{5}\D") {
            let res = try_atou16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::Overflow);
        }

        #[test]
        fn u16_double_sign_proptest(i in r"[+-]{2}[0-9]{4}") {
            let res = try_atou16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0 || res.error.index == 1);
        }

        #[test]
        fn u16_sign_only_proptest(i in r"[+-]") {
            let res = try_atou16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0);
        }

        #[test]
        fn u16_trailing_digits_proptest(i in r"[+]?[0-9]{4}\D[0-9]{2}") {
            let res = try_atou16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 4 || res.error.index == 5);
        }

        #[test]
        fn i16_invalid_proptest(i in r"[+-]?[0-9]{4}\D") {
            let res = try_atoi16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 4 || res.error.index == 5);
        }

        #[test]
        fn i16_overflow_proptest(i in r"[+-]?[1-9][0-9]{5}\D") {
            let res = try_atoi16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::Overflow);
        }

        #[test]
        fn i16_double_sign_proptest(i in r"[+-]{2}[0-9]{4}") {
            let res = try_atoi16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 1);
        }

        #[test]
        fn i16_sign_only_proptest(i in r"[+-]") {
            let res = try_atoi16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0);
        }

        #[test]
        fn i16_trailing_digits_proptest(i in r"[+-]?[0-9]{4}\D[0-9]{2}") {
            let res = try_atoi16_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 4 || res.error.index == 5);
        }

        #[test]
        fn u32_invalid_proptest(i in r"[+]?[0-9]{9}\D") {
            let res = try_atou32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 9 || res.error.index == 10);
        }

        #[test]
        fn u32_overflow_proptest(i in r"[+-]?[1-9][0-9]{10}\D") {
            let res = try_atou32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::Overflow);
        }

        #[test]
        fn u32_double_sign_proptest(i in r"[+-]{2}[0-9]{9}") {
            let res = try_atou32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0 || res.error.index == 1);
        }

        #[test]
        fn u32_sign_only_proptest(i in r"[+-]") {
            let res = try_atou32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0);
        }

        #[test]
        fn u32_trailing_digits_proptest(i in r"[+]?[0-9]{9}\D[0-9]{2}") {
            let res = try_atou32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 9 || res.error.index == 10);
        }

        #[test]
        fn i32_invalid_proptest(i in r"[+-]?[0-9]{9}\D") {
            let res = try_atoi32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 9 || res.error.index == 10);
        }

        #[test]
        fn i32_overflow_proptest(i in r"[+-]?[1-9][0-9]{10}\D") {
            let res = try_atoi32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::Overflow);
        }

        #[test]
        fn i32_double_sign_proptest(i in r"[+-]{2}[0-9]{9}") {
            let res = try_atoi32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 1);
        }

        #[test]
        fn i32_sign_only_proptest(i in r"[+-]") {
            let res = try_atoi32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0);
        }

        #[test]
        fn i32_trailing_digits_proptest(i in r"[+-]?[0-9]{9}\D[0-9]{2}") {
            let res = try_atoi32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 9 || res.error.index == 10);
        }

        #[test]
        fn u64_invalid_proptest(i in r"[+]?[0-9]{19}\D") {
            let res = try_atou64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 19 || res.error.index == 20);
        }

        #[test]
        fn u64_overflow_proptest(i in r"[+-]?[1-9][0-9]{21}\D") {
            let res = try_atou64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::Overflow);
        }

        #[test]
        fn u64_double_sign_proptest(i in r"[+-]{2}[0-9]{19}") {
            let res = try_atou64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0 || res.error.index == 1);
        }

        #[test]
        fn u64_sign_only_proptest(i in r"[+-]") {
            let res = try_atou64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0);
        }

        #[test]
        fn u64_trailing_digits_proptest(i in r"[+]?[0-9]{19}\D[0-9]{2}") {
            let res = try_atou64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 19 || res.error.index == 20);
        }

        #[test]
        fn i64_invalid_proptest(i in r"[+-]?[0-9]{18}\D") {
            let res = try_atoi64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 18 || res.error.index == 19);
        }

        #[test]
        fn i64_overflow_proptest(i in r"[+-]?[1-9][0-9]{19}\D") {
            let res = try_atoi64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::Overflow);
        }

        #[test]
        fn i64_double_sign_proptest(i in r"[+-]{2}[0-9]{18}") {
            let res = try_atoi64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 1);
        }

        #[test]
        fn i64_sign_only_proptest(i in r"[+-]") {
            let res = try_atoi64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0);
        }

        #[test]
        fn i64_trailing_digits_proptest(i in r"[+-]?[0-9]{18}\D[0-9]{2}") {
            let res = try_atoi64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 18 || res.error.index == 19);
        }
    }
}
