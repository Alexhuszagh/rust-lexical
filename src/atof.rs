//! Fast lexical string-to-float conversion routines.
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

use sealed::mem;
use sealed::ptr;

use ftoa::exponent_notation_char;
use util::*;

// TRAITS

/// Compatibility trait to allow wrapping arithmetic with atoi.
/// Doesn't really wrap, uses IEE754 float semantics.
trait WrappingFloat: Sized {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
}

macro_rules! wrapping_float_impl {
    ($($t:ty)*) => ($(
        impl WrappingFloat for $t {
            #[inline(always)]
            fn wrapping_add(self, rhs: $t) -> $t { self + rhs }

            #[inline(always)]
            fn wrapping_mul(self, rhs: $t) -> $t { self * rhs }
        }
    )*)
}

wrapping_float_impl! { f32 f64 }

// ATOF
// ----

/// Stores temporary state over atof
#[repr(C)]
struct State {
    /// Absolute start position.
    first: *const u8,
    /// Absolute last position.
    last: *const u8,
    /// Current first position.
    curr_first: *const u8,
    /// Current last position.
    curr_last: *const u8,
}

impl State {
    #[inline(always)]
    fn new(first: *const u8, last: *const u8) -> State {
        State {
            first: first,
            last: last,
            curr_first: unsafe { mem::uninitialized() },
            curr_last: unsafe { mem::uninitialized() }
        }
    }
}

#[inline(always)]
unsafe extern "C" fn starts_with_nan(first: *const u8, length: usize)
    -> bool
{
    starts_with(first, length, NAN_STRING.as_ptr(), NAN_STRING.len())
}

#[inline(always)]
unsafe extern "C" fn starts_with_infinity(first: *const u8, length: usize)
    -> bool
{
    starts_with(first, length, INFINITY_STRING.as_ptr(), INFINITY_STRING.len())
}

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
const POWI_EXPONENTS: [i32; 35] = [512, 512, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128];

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
unsafe extern "C" fn calculate_fraction(s: &mut State, base: u64, sig: usize) -> f64 {
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
            s.curr_last = minv!(s.last, s.curr_first.add(sig));
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

// ATOF

/// Implied atof for non-special (no NaN or Infinity) numbers.
///
/// Allows a custom quad type (if enabled) to be passed for higher-precision
/// calculations.
macro_rules! atof_finite {
    ($first:expr, $last:expr, $base:expr, $sig:expr) => ({
        let mut s = State::new($first, $last);
        let integer = calculate_integer(&mut s, $base);
        let fraction = calculate_fraction(&mut s, $base, $sig);
        let exponent = calculate_exponent(&mut s, $base);
        let value = calculate_value(integer, fraction, exponent, $base);
        (value, s.curr_last)
    })
}

/// Convert string to float (must be called within an unsafe block).
macro_rules! atof_value {
    ($first:expr, $last:expr, $base:expr, $sig:expr, $nan:ident, $inf:ident) => ({
        // special case checks
        let length = distance($first, $last);
        if starts_with_nan($first, length) {
            return ($nan, $first.add(3));
        } else if starts_with_infinity($first, length) {
            return ($inf, $first.add(8));
        }

        atof_finite!($first, $last, $base, $sig)
    })
}

/// Sanitizer for string to float (must be called within an unsafe block).
macro_rules! atof {
    ($first:expr, $last:expr, $base:expr, $sig:expr, $nan:ident, $inf:ident) => ({
        if $first == $last {
            (0.0, ptr::null())
        } else if *$first == b'-' {
            let(value, p) = atof_value!($first.add(1), $last, $base, $sig, $nan, $inf);
            (-value, p)
        } else if *$first == b'+' {
            atof_value!($first.add(1), $last, $base, $sig, $nan, $inf)
        } else {
            atof_value!($first, $last, $base, $sig, $nan, $inf)
        }
    })
}

// UNSAFE API

/// Generate the unsafe public wrappers.
///
/// * `func`        Function name.
/// * `sig`         Significand step for exponent.
/// * `f`           Float type.
/// * `nan`         NaN literal.
/// * `inf`         Infinity literal.
macro_rules! unsafe_impl {
    ($func:ident, $sig:expr, $f:ty, $nan:ident, $inf:ident) => (
        /// Unsafe, C-like importer for signed numbers.
        #[inline]
        pub unsafe extern "C" fn $func(
            first: *const u8,
            last: *const u8,
            base: u8
        )
            -> ($f, *const u8)
        {
            let (value, p) = atof!(first, last, base as u64, $sig, $nan, $inf);
            (value as $f, p)
        }
    )
}

unsafe_impl!(atof32_unsafe, 6, f32, F32_NAN, F32_INFINITY);
unsafe_impl!(atof64_unsafe, 12, f64, F64_NAN, F64_INFINITY);

// LOW-LEVEL API

bytes_impl!(atof32_bytes, f32, atof32_unsafe);
bytes_impl!(atof64_bytes, f64, atof64_unsafe);
try_bytes_impl!(try_atof32_bytes, f32, atof32_unsafe);
try_bytes_impl!(try_atof64_bytes, f64, atof64_unsafe);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atof32_base10_test() {
        // integer test
        assert_relative_eq!(0.0, atof32_bytes(b"0", 10), epsilon=1e-20);
        assert_relative_eq!(1.0, atof32_bytes(b"1", 10), epsilon=1e-20);
        assert_relative_eq!(12.0, atof32_bytes(b"12", 10), epsilon=1e-20);
        assert_relative_eq!(123.0, atof32_bytes(b"123", 10), epsilon=1e-20);
        assert_relative_eq!(1234.0, atof32_bytes(b"1234", 10), epsilon=1e-20);
        assert_relative_eq!(12345.0, atof32_bytes(b"12345", 10), epsilon=1e-20);
        assert_relative_eq!(123456.0, atof32_bytes(b"123456", 10), epsilon=1e-20);
        assert_relative_eq!(1234567.0, atof32_bytes(b"1234567", 10), epsilon=1e-20);
        assert_relative_eq!(12345678.0, atof32_bytes(b"12345678", 10), epsilon=1e-20);

        // decimal test
        assert_relative_eq!(123.1, atof32_bytes(b"123.1", 10), epsilon=1e-20);
        assert_relative_eq!(123.12, atof32_bytes(b"123.12", 10), epsilon=1e-20);
        assert_relative_eq!(123.123, atof32_bytes(b"123.123", 10), epsilon=1e-20);
        assert_relative_eq!(123.1234, atof32_bytes(b"123.1234", 10), epsilon=1e-20);
        assert_relative_eq!(123.12345, atof32_bytes(b"123.12345", 10), epsilon=1e-20);

        // rounding test
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.1", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.12", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.123", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.1234", 10), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(b"123456789.12345", 10), epsilon=1e-20);

        // exponent test
        assert_relative_eq!(123456789.12345, atof32_bytes(b"1.2345678912345e8", 10), epsilon=1e-20);
        assert_relative_eq!(123450000.0, atof32_bytes(b"1.2345e+8", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof32_bytes(b"1.2345e+11", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof32_bytes(b"123450000000", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof32_bytes(b"1.2345e+38", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof32_bytes(b"123450000000000000000000000000000000000", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof32_bytes(b"1.2345e-8", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof32_bytes(b"0.000000012345", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof32_bytes(b"1.2345e-38", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof32_bytes(b"0.000000000000000000000000000000000000012345", 10), epsilon=1e-20);

        #[cfg(feature = "std")]
        assert!(atof32_bytes(b"NaN", 10).is_nan());
        assert!(atof32_bytes(b"inf", 10).is_infinite());
        assert!(atof32_bytes(b"+inf", 10).is_infinite());
        assert!(atof32_bytes(b"-inf", 10).is_infinite());
    }

    #[test]
    fn atof32_basen_test() {
        assert_relative_eq!(1234.0, atof32_bytes(b"YA", 36));
    }

    #[test]
    fn atof64_base10_test() {
        // integer test
        assert_relative_eq!(0.0, atof64_bytes(b"0", 10), epsilon=1e-20);
        assert_relative_eq!(1.0, atof64_bytes(b"1", 10), epsilon=1e-20);
        assert_relative_eq!(12.0, atof64_bytes(b"12", 10), epsilon=1e-20);
        assert_relative_eq!(123.0, atof64_bytes(b"123", 10), epsilon=1e-20);
        assert_relative_eq!(1234.0, atof64_bytes(b"1234", 10), epsilon=1e-20);
        assert_relative_eq!(12345.0, atof64_bytes(b"12345", 10), epsilon=1e-20);
        assert_relative_eq!(123456.0, atof64_bytes(b"123456", 10), epsilon=1e-20);
        assert_relative_eq!(1234567.0, atof64_bytes(b"1234567", 10), epsilon=1e-20);
        assert_relative_eq!(12345678.0, atof64_bytes(b"12345678", 10), epsilon=1e-20);

        // decimal test
        assert_relative_eq!(123456789.0, atof64_bytes(b"123456789", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.1, atof64_bytes(b"123456789.1", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12, atof64_bytes(b"123456789.12", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.123, atof64_bytes(b"123456789.123", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.1234, atof64_bytes(b"123456789.1234", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345, atof64_bytes(b"123456789.12345", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.123456, atof64_bytes(b"123456789.123456", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.1234567, atof64_bytes(b"123456789.1234567", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345678, atof64_bytes(b"123456789.12345678", 10), epsilon=1e-20);

        // rounding test
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.123456789", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.1234567890", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.123456789012", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.1234567890123", 10), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.12345678901234", 10), epsilon=1e-20);

        // exponent test
        assert_relative_eq!(123456789.12345, atof64_bytes(b"1.2345678912345e8", 10), epsilon=1e-20);
        assert_relative_eq!(123450000.0, atof64_bytes(b"1.2345e+8", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof64_bytes(b"123450000000", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof64_bytes(b"1.2345e+11", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof64_bytes(b"1.2345e+38", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof64_bytes(b"123450000000000000000000000000000000000", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e+308, atof64_bytes(b"1.2345e+308", 10), max_relative=1e-12);
        assert_relative_eq!(1.2345e+308, atof64_bytes(b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10), max_relative=1e-12);
        assert_relative_eq!(0.000000012345, atof64_bytes(b"1.2345e-8", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof64_bytes(b"0.000000012345", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof64_bytes(b"1.2345e-38", 10), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof64_bytes(b"0.000000000000000000000000000000000000012345", 10), epsilon=1e-20);

        // denormalized (try extremely low values)
        assert_relative_eq!(1.2345e-308, atof64_bytes(b"1.2345e-308", 10), epsilon=1e-20);
        assert_eq!(5e-322, atof64_bytes(b"5e-322", 10));
        assert_eq!(5e-323, atof64_bytes(b"5e-323", 10));
        assert_eq!(5e-324, atof64_bytes(b"5e-324", 10));
        // due to issues in how the data is parsed, manually extracting
        // non-exponents of 1.<e-299 is prone to error
        // test the limit of our ability
        // We tend to get relative errors of 1e-16, even at super low values.
        assert_relative_eq!(1.2345e-299, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=1e-314);

        // Keep pushing from -300 to -324
        assert_relative_eq!(1.2345e-300, atof64_bytes(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=1e-315);
        assert_relative_eq!(1.2345e-310, atof64_bytes(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=5e-324);
        assert_relative_eq!(1.2345e-320, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=5e-324);
        assert_relative_eq!(1.2345e-321, atof64_bytes(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), epsilon=5e-324);
        assert_relative_eq!(1.24e-322, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000124", 10), epsilon=5e-324);
        assert_eq!(1e-323, atof64_bytes(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001", 10));
        assert_eq!(5e-324, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005", 10));

        #[cfg(feature = "std")]
        assert!(atof64_bytes(b"NaN", 10).is_nan());
        assert!(atof64_bytes(b"inf", 10).is_infinite());
        assert!(atof64_bytes(b"+inf", 10).is_infinite());
        assert!(atof64_bytes(b"-inf", 10).is_infinite());
    }

    #[test]
    #[should_panic]
    fn limit_test() {
        assert_relative_eq!(1.2345e-320, 0.0, epsilon=5e-324);
    }

    #[test]
    fn atof64_basen_test() {
        assert_relative_eq!(1234.0, atof64_bytes(b"YA", 36));
    }

    #[test]
    fn try_atof32_base10_test() {
        assert_eq!(Err(0), try_atof32_bytes(b"", 10));
        assert_eq!(Ok(0.0), try_atof32_bytes(b"0.0", 10));
        assert_eq!(Err(1), try_atof32_bytes(b"1a", 10));
    }

    #[test]
    fn try_atof64_base10_test() {
        assert_eq!(Err(0), try_atof64_bytes(b"", 10));
        assert_eq!(Ok(0.0), try_atof64_bytes(b"0.0", 10));
        assert_eq!(Err(1), try_atof64_bytes(b"1a", 10));
    }
}
