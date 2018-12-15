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
use util::*;
use super::exponent::parse_exponent;

// FRACTION

type Wrapped = WrappedFloat<f64>;

/// Parse the integer portion of a positive, normal float string.
///
/// Use a float since for large numbers, this may even overflow a u64.
#[inline(always)]
unsafe extern "C" fn parse_integer(state: &mut ParseState, radix: u32, last: *const u8)
    -> f64
{
    // Trim leading zeros, since we haven't parsed anything yet.
    state.ltrim_char(last, b'0');

    let mut value = Wrapped::ZERO;
    atoi::unchecked(&mut value, state, radix, last);

    value.into_inner()
}

/// Parse the fraction portion of a positive, normal float string.
///
/// Parse separately from the integer portion, since the small
/// values for each may be too small to change the integer components
/// representation **immediately**.
#[inline(always)]
unsafe extern "C" fn parse_fraction(state: &mut ParseState, radix: u32, last: *const u8)
    -> f64
{
    // Ensure if there's a decimal, there are trailing values, so
    // invalid floats like "0." lead to an error.
    if state.curr != last && *state.curr == b'.' {
        state.increment();
        let first = state.curr;
        let mut fraction: f64 = 0.;
        loop {
            // Trim leading zeros, since that never gets called with the raw parser.
            // Since if it's after the decimal place and this increments state.curr,
            // but not first, this is safe.
            state.ltrim_char(last, b'0');

            // This would get better numerical precision using Horner's method,
            // but that would require.
            let mut value: u64 = 0;
            let l = last.min(state.curr.add(12));
            atoi::unchecked(&mut value, state, radix, l);
            let digits = distance(first, state.curr).try_i32_or_max();

            // Ignore leading 0s, just not we've passed them.
            if value != 0 {
                fraction += f64::iterative_pow(value as f64, radix, -digits);
            }

            // do/while condition
            if state.curr == last || char_to_digit(*state.curr) as u32 >= radix {
                break;
            }
        }
        // Store frac component over the parsed digits.
        fraction
    } else {
        // Store frac component as an empty slice, and return the value.
        0.0
    }
}

/// Parse the mantissa from a string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
unsafe extern "C" fn parse_mantissa(state: &mut ParseState, radix: u32, last: *const u8)
    -> f64
{
    let integer = parse_integer(state, radix, last);
    let fraction = parse_fraction(state, radix, last);

    integer + fraction
}

// PARSE

/// Parse the mantissa and exponent from a string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
unsafe extern "C" fn parse_float(radix: u32, first: *const u8, last: *const u8)
    -> (f64, i32, ParseState)
{
    let mut state = ParseState::new(first);
    let mantissa = parse_mantissa(&mut state, radix, last);
    let exponent = parse_exponent(&mut state, radix, last);

    (mantissa, exponent, state)
}

// ATOF/ATOD

/// Parse 32-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) unsafe extern "C" fn atof(radix: u32, first: *const u8, last: *const u8)
    -> (f32, *const u8)
{
    let (value, p) = atod(radix, first, last);
    (value as f32, p)
}

/// Parse 64-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) unsafe extern "C" fn atod(radix: u32, first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    let (mut value, exponent, state) = parse_float(radix, first, last);
    if exponent != 0 && value != 0.0 {
        value = value.iterative_pow(radix, exponent);
    }
    (value, state.curr)
}

#[inline]
pub(crate) unsafe extern "C" fn atof_lossy(radix: u32, first: *const u8, last: *const u8)
    -> (f32, *const u8)
{
    atof(radix, first, last)
}

#[inline]
pub(crate) unsafe extern "C" fn atod_lossy(radix: u32, first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    atod(radix, first, last)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn check_parse_integer(radix: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let mut state = ParseState::new(first);
        let v = parse_integer(&mut state, radix, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, state.curr), tup.1);
    }

    #[test]
    fn parse_integer_test() {
        unsafe {
            check_parse_integer(10, "1.2345", (1.0, 1));
            check_parse_integer(10, "12.345", (12.0, 2));
            check_parse_integer(10, "12345.6789", (12345.0, 5));
        }
    }

    unsafe fn check_parse_fraction(radix: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let mut state = ParseState::new(first);
        let v = parse_fraction(&mut state, radix, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, state.curr), tup.1);
    }

    #[test]
    fn parse_fraction_test() {
        unsafe {
            check_parse_fraction(10, ".2345", (0.2345, 5));
            check_parse_fraction(10, ".345", (0.345, 4));
            check_parse_fraction(10, ".6789", (0.6789, 5));
        }
    }

    unsafe fn check_parse_mantissa(radix: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let mut state = ParseState::new(first);
        let v = parse_mantissa(&mut state, radix, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, state.curr), tup.1);
    }

    #[test]
    fn parse_mantissa_test() {
        unsafe {
            check_parse_mantissa(10, "1.2345", (1.2345, 6));
            check_parse_mantissa(10, "12.345", (12.345, 6));
            check_parse_mantissa(10, "12345.6789", (12345.6789, 10));
        }
    }

    unsafe fn check_parse_float(radix: u32, s: &str, tup: (f64, i32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, e, state) = parse_float(radix, first, last);
        assert_eq!(v, tup.0);
        assert_eq!(e, tup.1);
        assert_eq!(distance(first, state.curr), tup.2);
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

    unsafe fn check_atof(radix: u32, s: &str, tup: (f32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atof(radix, first, last);
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

    unsafe fn check_atod(radix: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atod(radix, first, last);
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
    // Just a synonym for the regular overloads, since we're not using the
    // correct feature. Use the same tests.

    unsafe fn check_atof_lossy(radix: u32, s: &str, tup: (f32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atof_lossy(radix, first, last);
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

    unsafe fn check_atod_lossy(radix: u32, s: &str, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atod_lossy(radix, first, last);
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
