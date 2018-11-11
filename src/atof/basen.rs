//! Fast, lossy, basen lexical string-to-float conversion routines.
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

use sealed::ptr;

use ftoa::exponent_notation_char;
use util::*;
use super::util::*;

// ATOF
// ----

// ALGORITHM

/// Use powi() iteratively.
///
/// * `value`   - Base value.
/// * `op`      - Operation {*, /}.
/// * `base`    - Floating-point base for exponent.
/// * `exp`     - Iteration exponent {+256, -256}.
/// * `count`   - Number of times to iterate.
/// * `rem`     - Remaining exponent after iteration.
macro_rules! stable_powi_impl {
    ($value:ident, $op:tt, $base:ident, $exp:expr, $count:ident, $rem:ident) => ({
        for _ in 0..$count {
            $value = $value $op powi($base, $exp);
        }
        if $rem != 0 {
            $value = $value $op powi($base, $rem)
        }
        $value
    })
}

/// Cached powers to get the desired exponent.
/// Make sure all values are < 1e300.
const POWI_EXPONENTS: [i32; 35] = [
    512, 512, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
    256, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128
];

/// Stable powi implementation, with a base value.
///
/// Although valid results will occur with an exponent or value of 0,
/// ideally, you should not pass any value as such to this function.
///
/// Use powi() with an integral exponent, both for speed and
/// stability. Don't go any an exponent of magnitude >1e300, for numerical
/// stability.
macro_rules! stable_powi {
    ($value:ident, $op:tt, $base:ident, $exponent:ident) => ({
        let base = $base as f64;
        let exp = unsafe { *POWI_EXPONENTS.get_unchecked($base as usize) };
        if $exponent < 0 {
            // negative exponent
            let count = $exponent / -exp;
            let rem = $exponent % exp;
            stable_powi_impl!($value, $op, base, -exp, count, rem)
        } else {
            // positive exponent
            let count = $exponent / exp;
            let rem = $exponent % exp;
            stable_powi_impl!($value, $op, base, exp, count, rem)
        }
    })
}

/// `powi` implementation that is more stable at extremely low powers.
///
/// Equivalent to `value * powi(base, exponent)`
pub(crate) fn stable_powi_multiplier(mut value: f64, base: u64, exponent: i32) -> f64 {
    stable_powi!(value, *, base, exponent)
}

/// `powi` implementation that is more stable at extremely low powers.
///
/// Equivalent to `value / powi(base, exponent)`
pub(crate) fn stable_powi_divisor(mut value: f64, base: u64, exponent: i32) -> f64 {
    stable_powi!(value, /, base, exponent)
}

// Calculate the integer portion.
// Use a float since for large numbers, this may even overflow an
// integer 64.
#[inline(always)]
unsafe extern "C" fn calculate_integer(s: &mut State, base: u64) -> f64 {
    let mut integer: f64 = 0.0;
    s.curr_last = atoi_pointer!(integer, s.first, s.last, base, f64);
    integer
}

// Calculate the fraction portion.
// Calculate separately from the integer portion, since the small
// values for each may be too small to change the integer components
// representation **immediately**.
// For numeric stability, use this early.
#[inline(always)]
unsafe extern "C" fn calculate_fraction(s: &mut State, base: u64) -> f64 {
    let mut fraction: f64 = 0.0;
    // Ensure if there's a decimal, there are trailing values, so
    // invalid floats like "0." lead to an error.
    if distance(s.curr_last, s.last) > 1 && *s.curr_last == b'.' {
        let mut digits: usize = 0;
        s.curr_last = s.curr_last.add(1);
        loop {
            // This would get better numerical precision using Horner's method,
            // but that would require.
            let mut value: u64 = 0;
            s.curr_first = s.curr_last;
            s.curr_last = minv!(s.last, s.curr_first.add(12));
            s.curr_last = atoi_pointer!(value, s.curr_first, s.curr_last, base, u64);
            digits += distance(s.curr_first, s.curr_last);

            // Ignore leading 0s, just not we've passed them.
            if value != 0 {
                fraction += stable_powi_divisor(value as f64, base, digits as i32);
            }

            // do/while condition
            if s.curr_last == s.last || char_to_digit!(*s.curr_last) as u64 >= base {
                break;
            }
        }
    }

    fraction
}

// Calculate the exponential portion, if
// we have an `e[+-]?\d+`.
// We don't care about the pointer after this, so just use `atoi_value`.
#[inline(always)]
unsafe extern "C" fn calculate_exponent(s: &mut State, base: u64) -> i32 {
    let dist = distance(s.curr_last, s.last);
    if dist > 1 && (*s.curr_last).to_ascii_lowercase() == exponent_notation_char(base) {
        s.curr_last = s.curr_last.add(1);
        s.curr_first = s.curr_last;
        s.curr_last = s.last;
        let (value, p) = atoi_signed!(s.curr_first, s.curr_last, base, i32);
        s.curr_last = p;
        value
    } else {
        0
    }
}

/// Calculate value from pieces.
#[inline(always)]
unsafe extern "C" fn calculate_value(integer: f64, fraction: f64, exponent: i32, base: u64)
    -> f64
{
    let mut value = integer + fraction;
    if exponent != 0 && value != 0.0 {
        value = stable_powi_multiplier(value, base, exponent);
    }
    value
}

// F32

/// Import float from basen, using a lossy algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
pub(crate) unsafe extern "C" fn float_basen(first: *const u8, last: *const u8, base: u64)
    -> (f32, *const u8)
{
    let (f, p) = double_basen(first, last, base);
    (f as f32, p)
}

/// Import float from base10, using a lossy algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[cfg(not(feature = "correct"))]
pub(crate) unsafe extern "C" fn float_base10(first: *const u8, last: *const u8)
    -> (f32, *const u8)
{
    float_basen(first, last, 10)
}

// F64

/// Import double from basen, using a lossy algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
pub(crate) unsafe extern "C" fn double_basen(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    let mut s = State::new(first, last);
    let integer = calculate_integer(&mut s, base);
    let fraction = calculate_fraction(&mut s, base);
    let exponent = calculate_exponent(&mut s, base);
    let value = calculate_value(integer, fraction, exponent, base);
    (value, s.curr_last)
}

/// Import double from base10, using a lossy algorithm.
///
/// Number must be non-special, positive, and non-zero.
#[inline]
#[cfg(not(feature = "correct"))]
pub(crate) unsafe extern "C" fn double_base10(first: *const u8, last: *const u8)
    -> (f64, *const u8)
{
    double_basen(first, last, 10)
}
