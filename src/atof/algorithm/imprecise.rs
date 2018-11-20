//! Lossy algorithms for string-to-float conversions.
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
//! The benchmark code may be found `benches/atof.rs`.
//!
//! # Benchmarks
//!
//! | Type  |  lexical (ns/iter) | parse (ns/iter)       | Relative Increase |
//! |:-----:|:------------------:|:---------------------:|:-----------------:|
//! | f32   | 761,670            | 28,650,637            | 37.62x            |
//! | f64   | 1,083,162          | 123,675,824           | 114.18x           |
//!
//! # Raw Benchmarks
//!
//! ```text
//! test f32_lexical ... bench:     761,670 ns/iter (+/- 194,856)
//! test f32_parse   ... bench:  28,650,637 ns/iter (+/- 7,269,036)
//! test f64_lexical ... bench:   1,083,162 ns/iter (+/- 315,101)
//! test f64_parse   ... bench: 123,675,824 ns/iter (+/- 20,924,195)
//! ```
//!
//! Raw Benchmarks (`no_std`)
//!
//! ```text
//! test f32_lexical ... bench:     652,922 ns/iter (+/- 44,491)
//! test f32_parse   ... bench:  24,381,160 ns/iter (+/- 687,175)
//! test f64_lexical ... bench:     835,822 ns/iter (+/- 28,754)
//! test f64_parse   ... bench: 113,449,442 ns/iter (+/- 3,983,104)
//! ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([761670, 1083162]) / 1e6
//  parse = np.array([28650637, 123675824]) / 1e6
//  index = ["f32", "f64"]
//  df = pd.DataFrame({'lexical': lexical, 'parse': parse}, index = index)
//  ax = df.plot.bar(rot=0)
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  plt.show()

use atoi;
use table::*;
use util::*;
use super::exponent::parse_exponent;

// FRACTION

type Wrapped = WrappedFloat<f64>;

/// Parse the integer portion of a positive, normal float string.
///
/// Use a float since for large numbers, this may even overflow a u64.
#[inline(always)]
unsafe extern "C" fn parse_integer(base: u32, first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    let cb = atoi::unchecked::<Wrapped>;
    let (integer, p, _) = atoi::value::<Wrapped, _>(base, first, last, cb);
    (integer.into_inner(), p)
}

/// Parse the fraction portion of a positive, normal float string.
///
/// Parse separately from the integer portion, since the small
/// values for each may be too small to change the integer components
/// representation **immediately**.
#[inline(always)]
unsafe extern "C" fn parse_fraction(base: u32, first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    // Ensure if there's a decimal, there are trailing values, so
    // invalid floats like "0." lead to an error.
    if distance(first, last) > 1 && *first == b'.' {
        let mut fraction: f64 = 0.;
        let first = first.add(1);
        let mut f = first;
        loop {
            // Trim leading zeros, since that never gets called with the raw parser.
            f = ltrim_char(f, last, b'0');

            // This would get better numerical precision using Horner's method,
            // but that would require.
            let mut value: u64 = 0;
            let l = last.min(f.add(12));
            f = atoi::unchecked(&mut value, base, f, l).0;
            let digits = distance(first, f) as i32;

            // Ignore leading 0s, just not we've passed them.
            if value != 0 {
                fraction += f64::iterative_pow(value as f64, base, -digits);
            }

            // do/while condition
            if f == last || char_to_digit(*f) as u32 >= base {
                break;
            }
        }
        (fraction, f)
    } else {
        (0.0, first)
    }
}

// PARSE

/// Parse the mantissa and exponent from a string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
unsafe extern "C" fn parse_float(base: u32, first: *const u8, last: *const u8)
    -> (f64, i32, *const u8)
{
    // Parse components
    let (integer, p) = parse_integer(base, first, last);
    let (fraction, p) = parse_fraction(base, p, last);
    let (exponent, p) = parse_exponent(base, p, last);

    (integer + fraction, exponent, p)
}

// ATOF/ATOD

/// Parse 32-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) unsafe extern "C" fn atof(base: u32, first: *const u8, last: *const u8)
    -> (f32, *const u8)
{
    let (value, p) = atod(base, first, last);
    (value as f32, p)
}

/// Parse 64-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) unsafe extern "C" fn atod(base: u32, first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    let (mut value, exponent, p) = parse_float(base, first, last);
    if exponent != 0 && value != 0.0 {
        value = value.iterative_pow(base, exponent);
    }
    (value, p)
}

#[inline]
pub(crate) unsafe extern "C" fn atof_lossy(base: u32, first: *const u8, last: *const u8)
    -> (f32, *const u8)
{
    atof(base, first, last)
}

#[inline]
pub(crate) unsafe extern "C" fn atod_lossy(base: u32, first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    atod(base, first, last)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn check_parse_integer(base: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = parse_integer(base, first, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn parse_integer_test() {
        unsafe {
            check_parse_integer(10, "1.2345", (1.0, 1));
            check_parse_integer(10, "12.345", (12.0, 2));
            check_parse_integer(10, "12345.6789", (12345.0, 5));
        }
    }

    unsafe fn check_parse_fraction(base: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = parse_fraction(base, first, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn parse_fraction_test() {
        unsafe {
            check_parse_fraction(10, ".2345", (0.2345, 5));
            check_parse_fraction(10, ".345", (0.345, 4));
            check_parse_fraction(10, ".6789", (0.6789, 5));
        }
    }

    unsafe fn check_parse_float(base: u32, s: &str, tup: (f64, i32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, e, p) = parse_float(base, first, last);
        assert_eq!(v, tup.0);
        assert_eq!(e, tup.1);
        assert_eq!(distance(first, p), tup.2);
    }

    #[test]
    fn parse_float_test() {
        unsafe {
            check_parse_float(10, "1.2345", (1.2345, 0, 6));
            check_parse_float(10, "12.345", (12.345, 0, 6));
            check_parse_float(10, "12345.6789", (12345.6789, 0, 10));
            check_parse_float(10, "1.2345e10", (1.2345, 10, 9));
        }
    }

    unsafe fn check_atof(base: u32, s: &str, tup: (f32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atof(base, first, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn atof_test() {
        unsafe {
            check_atof(10, "1.2345", (1.2345, 6));
            check_atof(10, "12.345", (12.345, 6));
            check_atof(10, "12345.6789", (12345.6789, 10));
            check_atof(10, "1.2345e10", (1.2345e10, 9));
        }
    }

    unsafe fn check_atod(base: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atod(base, first, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn atod_test() {
        unsafe {
            check_atod(10, "1.2345", (1.2345, 6));
            check_atod(10, "12.345", (12.345, 6));
            check_atod(10, "12345.6789", (12345.6789, 10));
            check_atod(10, "1.2345e10", (1.2345e10, 9));
        }
    }

    // Lossy
    // Just a synonym for the regular overloads, since we're using the
    // imprecise feature. Use the same tests.

    unsafe fn check_atof_lossy(base: u32, s: &str, tup: (f32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atof_lossy(base, first, last);
        assert_f32_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn atof_lossy_test() {
        unsafe {
            check_atof_lossy(10, "1.2345", (1.2345, 6));
            check_atof_lossy(10, "12.345", (12.345, 6));
            check_atof_lossy(10, "12345.6789", (12345.6789, 10));
            check_atof_lossy(10, "1.2345e10", (1.2345e10, 9));
        }
    }

    unsafe fn check_atod_lossy(base: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atod_lossy(base, first, last);
        assert_f64_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn atod_lossy_test() {
        unsafe {
            check_atod_lossy(10, "1.2345", (1.2345, 6));
            check_atod_lossy(10, "12.345", (12.345, 6));
            check_atod_lossy(10, "12345.6789", (12345.6789, 10));
            check_atod_lossy(10, "1.2345e10", (1.2345e10, 9));
        }
    }
}
