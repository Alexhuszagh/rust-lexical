//! Fast lexical string-to-integer conversion routines.
//!
//! These routines are wrapping, and therefore can accept any buffer for any
//! size type, but will wrap to the desired value if overflow occurs.
//!
//! The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//! CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//! (x86-64), using the lexical formatter or `x.parse()`,
//! avoiding any inefficiencies in Rust string parsing. The code was
//! compiled with LTO and at an optimization level of 3.
//!
//! The benchmarks with `std` were compiled using "rustc 1.29.2 (17a9dc751
//! 2018-10-05", and the `no_std` benchmarks were compiled using "rustc
//! 1.31.0-nightly (46880f41b 2018-10-15)".
//!
//! The benchmark code may be found `benches/atoi.rs`.
//!
//! # Benchmarks
//!
//! | Type  |  lexical (ns/iter) | parse (ns/iter)       | Relative Increase |
//! |:-----:|:------------------:|:---------------------:|:-----------------:|
//! | u8    | 62,790             | 67,926                | 1.08x             |
//! | u16   | 58,896             | 76,602                | 1.30x             |
//! | u32   | 103,962            | 139,434               | 1.34x             |
//! | u64   | 192,792            | 265,931               | 1.38x             |
//! | i8    | 89,828             | 109,099               | 1.21x             |
//! | i16   | 111,592            | 140,172               | 1.26x             |
//! | i32   | 155,172            | 189,377               | 1.22x             |
//! | i64   | 197,747            | 283,541               | 1.43x             |
//!
//! # Raw Benchmarks
//!
//! ```text
//! test i8_lexical  ... bench:      89,828 ns/iter (+/- 9,172)
//! test i8_parse    ... bench:     109,099 ns/iter (+/- 2,711)
//! test i16_lexical ... bench:     111,592 ns/iter (+/- 3,862)
//! test i16_parse   ... bench:     140,172 ns/iter (+/- 7,194)
//! test i32_lexical ... bench:     155,172 ns/iter (+/- 5,248)
//! test i32_parse   ... bench:     189,377 ns/iter (+/- 10,131)
//! test i64_lexical ... bench:     197,747 ns/iter (+/- 18,041)
//! test i64_parse   ... bench:     283,541 ns/iter (+/- 14,240)
//! test u8_lexical  ... bench:      62,790 ns/iter (+/- 3,146)
//! test u8_parse    ... bench:      67,926 ns/iter (+/- 3,767)
//! test u16_lexical ... bench:      58,896 ns/iter (+/- 3,238)
//! test u16_parse   ... bench:      76,602 ns/iter (+/- 3,771)
//! test u32_lexical ... bench:     103,962 ns/iter (+/- 4,870)
//! test u32_parse   ... bench:     139,434 ns/iter (+/- 3,944)
//! test u64_lexical ... bench:     192,792 ns/iter (+/- 9,147)
//! test u64_parse   ... bench:     265,931 ns/iter (+/- 8,308)
//! ```
//!
//! Raw Benchmarks (`no_std`)
//!
//! ```text
//! test i8_lexical  ... bench:      94,142 ns/iter (+/- 5,252)
//! test i8_parse    ... bench:     107,092 ns/iter (+/- 4,121)
//! test i16_lexical ... bench:     113,284 ns/iter (+/- 17,479)
//! test i16_parse   ... bench:     141,393 ns/iter (+/- 5,804)
//! test i32_lexical ... bench:     155,704 ns/iter (+/- 5,590)
//! test i32_parse   ... bench:     191,977 ns/iter (+/- 8,241)
//! test i64_lexical ... bench:     197,485 ns/iter (+/- 11,415)
//! test i64_parse   ... bench:     298,771 ns/iter (+/- 13,941)
//! test u8_lexical  ... bench:      61,893 ns/iter (+/- 1,171)
//! test u8_parse    ... bench:      73,681 ns/iter (+/- 7,508)
//! test u16_lexical ... bench:      60,014 ns/iter (+/- 2,605)
//! test u16_parse   ... bench:      78,667 ns/iter (+/- 2,899)
//! test u32_lexical ... bench:     102,840 ns/iter (+/- 2,770)
//! test u32_parse   ... bench:     140,070 ns/iter (+/- 3,443)
//! test u64_lexical ... bench:     191,493 ns/iter (+/- 2,648)
//! test u64_parse   ... bench:     279,269 ns/iter (+/- 12,914)
//! ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([62790, 58896, 103962, 192792, 89828, 111592, 155172, 197747]) / 1e6
//  parse = np.array([67926, 76602, 139434, 265931, 109099, 140172, 189377, 283541]) / 1e6
//  index = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
//  df = pd.DataFrame({'lexical': lexical, 'parse': parse}, index = index)
//  ax = df.plot.bar(rot=0)
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  plt.show()

use util::*;

// ALGORITHM

/// Optimized atoi implementation that uses a translation table.
macro_rules! atoi_impl {
    ($value:ident, $first:expr, $last:expr, $base:expr, $t:tt, $mul:ident, $add:ident) => ({
        let base = $base as $t;
        let mut p = $first;
        let mut overflow = false;

        // Remove leading zeros
        while p < $last && *p == b'0' {
            p = p.add(1);
        }

        // Continue while we have digits.
        // Don't check for overflow, we want to avoid as many conditions
        // as possible, it leads to significant speed increases on x86-64.
        // Just note it happens, and continue on.
        // Don't add a short-circuit either, since it adds significant time
        // and we want to continue parsing until everything is done, since
        // otherwise it may give us invalid results elsewhere.
        while p < $last {
            // Grab the next digit, and check if it's valid.
            let digit = char_to_digit!(*p) as $t;
            if digit >= base {
                break;
            }

            // Rust doesn't allow tuples as l-values...
            let (value, o1) = $value.$mul(base);
            let (value, o2) = value.$add(digit);
            $value = value;
            overflow |= o1 | o2;
            p = p.add(1);
        }

        (p, overflow)
    })
}

/// Parse value and get end pointer from parsing the integer within the block.
macro_rules! atoi_pointer {
    // Explicit multiply and add methods.
    ($value:ident, $first:expr, $last:expr, $base:ident, $t:tt, $mul:ident, $add:ident)
    =>
    ({
        // logic error, disable in release builds
        debug_assert!($base >= 2 && $base <= 36, "Numerical base must be from 2-36");
        atoi_impl!($value, $first, $last, $base, $t, $mul, $add)
    });
    // Non-explicit multiply and add methods
    ($value:ident, $first:expr, $last:expr, $base:ident, $t:tt) => (
        atoi_pointer!($value, $first, $last, $base, $t, overflowing_mul, overflowing_add)
    );
}

/// Parse value from a positive numeric string.
macro_rules! atoi_value {
    // Explicit multiply and add methods.
    ($first:expr, $last:expr, $base:expr, $t:tt, $mul:ident, $add:ident)
    =>
    ({
        // logic error, disable in release builds
        debug_assert!($base >= 2 && $base <= 36, "Numerical base must be from 2-36");

        let mut value: $t = 0;
        let base = $base as $t;
        let (p, overflow) = atoi_pointer!(value, $first, $last, base, $t, $mul, $add);

        (value, p, overflow)
    });
    // Non-explicit multiply and add methods
    ($first:expr, $last:expr, $base:expr, $t:tt) => (
        atoi_value!($first, $last, $base, $t, overflowing_mul, overflowing_add)
    );
}

/// Handle +/- numbers and forward to implementation.
///
/// `first` must be less than or equal to `last`.
macro_rules! atoi_sign {
    ($first:expr, $last:expr, $base:expr, $t:tt) => ({
        match *$first {
            b'+' => {
                let (v, p, o) = atoi_value!($first.add(1), $last, $base, $t);
                (v, p, o, 1)
            },
            b'-' => {
                let (v, p, o) = atoi_value!($first.add(1), $last, $base, $t);
                (v, p, o, -1)
            },
            _    => {
                let (v, p, o) = atoi_value!($first, $last, $base, $t);
                (v, p, o, 1)
            },
        }
    })
}

/// Handle unsigned +/- numbers and forward to implied implementation.
macro_rules! atoi_unsigned {
    ($first:expr, $last:expr, $base:expr, $t:tt) => ({
        if $first == $last {
            (0, nullptr!(), false)
        } else {
            let (v, p, o, s) = atoi_sign!($first, $last, $base, $t);
            match s {
                -1 => (v.wrapping_neg(), p, true),
                1  => (v, p, o),
                _  => unreachable!(),
            }
        }
    })
}

/// Handle signed +/- numbers and forward to implied implementation.
macro_rules! atoi_signed {
    ($first:expr, $last:expr, $base:expr, $t:tt) => ({
        if $first == $last {
            (0, nullptr!(), false)
        } else {
            let (v, p, o, s) = atoi_sign!($first, $last, $base, $t);
            match s {
                -1 => (-v, p, o),
                1  => (v, p, o),
                _  => unreachable!(),
            }
        }
    })
}

// UNSAFE API

/// Generate the unsigned, unsafe wrappers.
macro_rules! unsigned_unsafe_impl {
    ($func:ident, $t:tt) => (
        /// Unsafe, C-like importer for unsigned numbers.
        #[inline]
        pub unsafe extern "C" fn $func(
            first: *const u8,
            last: *const u8,
            base: u8
        )
            -> ($t, *const u8, bool)
        {
            atoi_unsigned!(first, last, base, $t)
        }
    )
}

unsigned_unsafe_impl!(atou8_unsafe, u8);
unsigned_unsafe_impl!(atou16_unsafe, u16);
unsigned_unsafe_impl!(atou32_unsafe, u32);
unsigned_unsafe_impl!(atou64_unsafe, u64);
unsigned_unsafe_impl!(atousize_unsafe, usize);

/// Generate the signed, unsafe wrappers.
macro_rules! signed_unsafe_impl {
    ($func:ident, $t:tt) => (
        /// Unsafe, C-like importer for signed numbers.
        #[inline]
        pub unsafe extern "C" fn $func(
            first: *const u8,
            last: *const u8,
            base: u8
        )
            -> ($t, *const u8, bool)
        {
            atoi_signed!(first, last, base, $t)
        }
    )
}

signed_unsafe_impl!(atoi8_unsafe, i8);
signed_unsafe_impl!(atoi16_unsafe, i16);
signed_unsafe_impl!(atoi32_unsafe, i32);
signed_unsafe_impl!(atoi64_unsafe, i64);
signed_unsafe_impl!(atoisize_unsafe, isize);

// LOW-LEVEL API

bytes_impl!(atou8_bytes, u8, atou8_unsafe);
bytes_impl!(atou16_bytes, u16, atou16_unsafe);
bytes_impl!(atou32_bytes, u32, atou32_unsafe);
bytes_impl!(atou64_bytes, u64, atou64_unsafe);
bytes_impl!(atousize_bytes, usize, atousize_unsafe);
bytes_impl!(atoi8_bytes, i8, atoi8_unsafe);
bytes_impl!(atoi16_bytes, i16, atoi16_unsafe);
bytes_impl!(atoi32_bytes, i32, atoi32_unsafe);
bytes_impl!(atoi64_bytes, i64, atoi64_unsafe);
bytes_impl!(atoisize_bytes, isize, atoisize_unsafe);
try_bytes_impl!(try_atou8_bytes, u8, atou8_unsafe);
try_bytes_impl!(try_atou16_bytes, u16, atou16_unsafe);
try_bytes_impl!(try_atou32_bytes, u32, atou32_unsafe);
try_bytes_impl!(try_atou64_bytes, u64, atou64_unsafe);
try_bytes_impl!(try_atousize_bytes, usize, atousize_unsafe);
try_bytes_impl!(try_atoi8_bytes, i8, atoi8_unsafe);
try_bytes_impl!(try_atoi16_bytes, i16, atoi16_unsafe);
try_bytes_impl!(try_atoi32_bytes, i32, atoi32_unsafe);
try_bytes_impl!(try_atoi64_bytes, i64, atoi64_unsafe);
try_bytes_impl!(try_atoisize_bytes, isize, atoisize_unsafe);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use error::{invalid_digit, overflow};
    use super::*;

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
        assert_eq!(0, atou8_bytes(b"0", 10));
        assert_eq!(127, atou8_bytes(b"127", 10));
        assert_eq!(128, atou8_bytes(b"128", 10));
        assert_eq!(255, atou8_bytes(b"255", 10));
        assert_eq!(255, atou8_bytes(b"-1", 10));
        assert_eq!(1, atou8_bytes(b"1a", 10));
    }

    #[test]
    fn atou8_basen_test() {
        for (b, s) in DATA.iter() {
            assert_eq!(atou8_bytes(s.as_bytes(), *b), 37);
        }
    }

    #[test]
    fn atoi8_base10_test() {
        assert_eq!(0, atoi8_bytes(b"0", 10));
        assert_eq!(127, atoi8_bytes(b"127", 10));
        assert_eq!(-128, atoi8_bytes(b"128", 10));
        assert_eq!(-1, atoi8_bytes(b"255", 10));
        assert_eq!(-1, atoi8_bytes(b"-1", 10));
        assert_eq!(1, atoi8_bytes(b"1a", 10));
    }

    #[test]
    fn atou16_base10_test() {
        assert_eq!(0, atou16_bytes(b"0", 10));
        assert_eq!(32767, atou16_bytes(b"32767", 10));
        assert_eq!(32768, atou16_bytes(b"32768", 10));
        assert_eq!(65535, atou16_bytes(b"65535", 10));
        assert_eq!(65535, atou16_bytes(b"-1", 10));
        assert_eq!(1, atou16_bytes(b"1a", 10));
    }

    #[test]
    fn atoi16_base10_test() {
        assert_eq!(0, atoi16_bytes(b"0", 10));
        assert_eq!(32767, atoi16_bytes(b"32767", 10));
        assert_eq!(-32768, atoi16_bytes(b"32768", 10));
        assert_eq!(-1, atoi16_bytes(b"65535", 10));
        assert_eq!(-1, atoi16_bytes(b"-1", 10));
        assert_eq!(1, atoi16_bytes(b"1a", 10));
    }

    #[test]
    fn atoi16_basen_test() {
        assert_eq!(atoi16_bytes(b"YA", 36), 1234);
    }

    #[test]
    fn atou32_base10_test() {
        assert_eq!(0, atou32_bytes(b"0", 10));
        assert_eq!(2147483647, atou32_bytes(b"2147483647", 10));
        assert_eq!(2147483648, atou32_bytes(b"2147483648", 10));
        assert_eq!(4294967295, atou32_bytes(b"4294967295", 10));
        assert_eq!(4294967295, atou32_bytes(b"-1", 10));
        assert_eq!(1, atou32_bytes(b"1a", 10));
    }

    #[test]
    fn atoi32_base10_test() {
        assert_eq!(0, atoi32_bytes(b"0", 10));
        assert_eq!(2147483647, atoi32_bytes(b"2147483647", 10));
        assert_eq!(-2147483648, atoi32_bytes(b"2147483648", 10));
        assert_eq!(-1, atoi32_bytes(b"4294967295", 10));
        assert_eq!(-1, atoi32_bytes(b"-1", 10));
        assert_eq!(1, atoi32_bytes(b"1a", 10));
    }

    #[test]
    fn atou64_base10_test() {
        assert_eq!(0, atou64_bytes(b"0", 10));
        assert_eq!(9223372036854775807, atou64_bytes(b"9223372036854775807", 10));
        assert_eq!(9223372036854775808, atou64_bytes(b"9223372036854775808", 10));
        assert_eq!(18446744073709551615, atou64_bytes(b"18446744073709551615", 10));
        assert_eq!(18446744073709551615, atou64_bytes(b"-1", 10));
        assert_eq!(1, atou64_bytes(b"1a", 10));
    }

    #[test]
    fn atoi64_base10_test() {
        assert_eq!(0, atoi64_bytes(b"0", 10));
        assert_eq!(9223372036854775807, atoi64_bytes(b"9223372036854775807", 10));
        assert_eq!(-9223372036854775808, atoi64_bytes(b"9223372036854775808", 10));
        assert_eq!(-1, atoi64_bytes(b"18446744073709551615", 10));
        assert_eq!(-1, atoi64_bytes(b"-1", 10));
        assert_eq!(1, atoi64_bytes(b"1a", 10));
    }

    #[test]
    fn try_atou8_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atou8_bytes(b"", 10));
        assert_eq!(Ok(0), try_atou8_bytes(b"0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atou8_bytes(b"1a", 10));
        assert_eq!(Err(overflow()), try_atou8_bytes(b"256", 10));
    }

    #[test]
    fn try_atoi8_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atoi8_bytes(b"", 10));
        assert_eq!(Ok(0), try_atoi8_bytes(b"0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atoi8_bytes(b"1a", 10));
        assert_eq!(Err(overflow()), try_atoi8_bytes(b"128", 10));
    }

    #[test]
    fn try_atou16_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atou16_bytes(b"", 10));
        assert_eq!(Ok(0), try_atou16_bytes(b"0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atou16_bytes(b"1a", 10));
        assert_eq!(Err(overflow()), try_atou16_bytes(b"65536", 10));
    }

    #[test]
    fn try_atoi16_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atoi16_bytes(b"", 10));
        assert_eq!(Ok(0), try_atoi16_bytes(b"0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atoi16_bytes(b"1a", 10));
        assert_eq!(Err(overflow()), try_atoi16_bytes(b"32768", 10));
    }

    #[test]
    fn try_atou32_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atou32_bytes(b"", 10));
        assert_eq!(Ok(0), try_atou32_bytes(b"0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atou32_bytes(b"1a", 10));
        assert_eq!(Err(overflow()), try_atou32_bytes(b"4294967296", 10));
    }

    #[test]
    fn try_atoi32_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atoi32_bytes(b"", 10));
        assert_eq!(Ok(0), try_atoi32_bytes(b"0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atoi32_bytes(b"1a", 10));
        assert_eq!(Err(overflow()), try_atoi32_bytes(b"2147483648", 10));
    }

    #[test]
    fn try_atou64_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atou64_bytes(b"", 10));
        assert_eq!(Ok(0), try_atou64_bytes(b"0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atou64_bytes(b"1a", 10));
        assert_eq!(Err(overflow()), try_atou64_bytes(b"18446744073709551616", 10));
    }

    #[test]
    fn try_atoi64_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atoi64_bytes(b"", 10));
        assert_eq!(Ok(0), try_atoi64_bytes(b"0", 10));
        assert_eq!(Err(invalid_digit(1)), try_atoi64_bytes(b"1a", 10));
        assert_eq!(Err(overflow()), try_atoi64_bytes(b"9223372036854775808", 10));
    }
}
