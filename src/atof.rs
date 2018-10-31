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
//! | f32   | 761,670            | 67,926                | 37.62x            |
//! | f64   | 123,675,824        | 1,083,162             | 114.18x           |
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

use sealed::mem;
use sealed::ptr;

#[cfg(feature = "f128")]
use f128::f128;

use ftoa::exponent_notation_char;
use table::BASEN;
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

#[cfg(feature = "f128")]
wrapping_float_impl! { f128 }

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
    const NAN: [u8; 3] = [b'N', b'a', b'N'];
    starts_with(first, length, NAN.as_ptr(), NAN.len())
}

#[inline(always)]
unsafe extern "C" fn starts_with_infinity(first: *const u8, length: usize)
    -> bool
{
    const INFINITY: [u8; 8] = [b'I', b'n', b'f', b'i', b'n', b'i', b't', b'y'];
    starts_with(first, length, INFINITY.as_ptr(), INFINITY.len())
}

/// Check if any base digit is valid.
macro_rules! is_valid_digit {
    ($c:expr, $base:expr) => ({
        let upper = *BASEN.get_unchecked($base as usize - 1);
        if $base <= 10 { is_valid_num!($c, upper) } else { is_valid_alnum!($c, upper) }
    })
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
    let base_f = base as f64;
    if s.curr_last != s.last && *s.curr_last == b'.' {
        let mut digits: usize = 0;
        s.curr_last = s.curr_last.add(1);
        loop {
            // This would get better numerical precision using Horner's method,
            // but that would require.
            let mut value: i64 = 0;
            s.curr_first = s.curr_last;
            s.curr_last = minv!(s.last, s.curr_first.add(sig));
            s.curr_last = atoi_pointer!(value, s.curr_first, s.curr_last, base, i64);
            digits += distance(s.curr_first, s.curr_last);
            fraction += value as f64 / powi(base_f, digits as i32);

            // do/while condition
            if s.curr_last == s.last || !is_valid_digit!(*s.curr_last, base) {
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
    if exponent != 0 {
        // Use powi() with an integral exponent, both for speed and
        // stability.
        value *= powi(base as f64, exponent) as f64;
    }
    value
}


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
        assert_float_relative_eq!(0.0, atof32_bytes(b"0", 10), 1e-20);
        assert_float_relative_eq!(1.0, atof32_bytes(b"1", 10), 1e-20);
        assert_float_relative_eq!(12.0, atof32_bytes(b"12", 10), 1e-20);
        assert_float_relative_eq!(123.0, atof32_bytes(b"123", 10), 1e-20);
        assert_float_relative_eq!(1234.0, atof32_bytes(b"1234", 10), 1e-20);
        assert_float_relative_eq!(12345.0, atof32_bytes(b"12345", 10), 1e-20);
        assert_float_relative_eq!(123456.0, atof32_bytes(b"123456", 10), 1e-20);
        assert_float_relative_eq!(1234567.0, atof32_bytes(b"1234567", 10), 1e-20);
        assert_float_relative_eq!(12345678.0, atof32_bytes(b"12345678", 10), 1e-20);

        // decimal test
        assert_float_relative_eq!(123.1, atof32_bytes(b"123.1", 10), 1e-20);
        assert_float_relative_eq!(123.12, atof32_bytes(b"123.12", 10), 1e-20);
        assert_float_relative_eq!(123.123, atof32_bytes(b"123.123", 10), 1e-20);
        assert_float_relative_eq!(123.1234, atof32_bytes(b"123.1234", 10), 1e-20);
        assert_float_relative_eq!(123.12345, atof32_bytes(b"123.12345", 10), 1e-20);

        // rounding test
        assert_float_relative_eq!(123456790.0, atof32_bytes(b"123456789", 10), 1e-20);
        assert_float_relative_eq!(123456790.0, atof32_bytes(b"123456789.1", 10), 1e-20);
        assert_float_relative_eq!(123456790.0, atof32_bytes(b"123456789.12", 10), 1e-20);
        assert_float_relative_eq!(123456790.0, atof32_bytes(b"123456789.123", 10), 1e-20);
        assert_float_relative_eq!(123456790.0, atof32_bytes(b"123456789.1234", 10), 1e-20);
        assert_float_relative_eq!(123456790.0, atof32_bytes(b"123456789.12345", 10), 1e-20);

        // exponent test
        assert_float_relative_eq!(123456789.12345, atof32_bytes(b"1.2345678912345e8", 10), 1e-20);
        assert_float_relative_eq!(123450000.0, atof32_bytes(b"1.2345e+8", 10), 1e-20);
        assert_float_relative_eq!(1.2345e+11, atof32_bytes(b"1.2345e+11", 10), 1e-20);
        assert_float_relative_eq!(1.2345e+11, atof32_bytes(b"123450000000", 10), 1e-20);
        assert_float_relative_eq!(1.2345e+38, atof32_bytes(b"1.2345e+38", 10), 1e-20);
        assert_float_relative_eq!(1.2345e+38, atof32_bytes(b"123450000000000000000000000000000000000", 10), 1e-20);
        assert_float_relative_eq!(1.2345e-8, atof32_bytes(b"1.2345e-8", 10), 1e-20);
        assert_float_relative_eq!(1.2345e-8, atof32_bytes(b"0.000000012345", 10), 1e-20);
        assert_float_relative_eq!(1.2345e-38, atof32_bytes(b"1.2345e-38", 10), 1e-20);
        assert_float_relative_eq!(1.2345e-38, atof32_bytes(b"0.000000000000000000000000000000000000012345", 10), 1e-20);

        #[cfg(feature = "std")]
        assert!(atof32_bytes(b"NaN", 10).is_nan());
        assert!(atof32_bytes(b"Infinity", 10).is_infinite());
        assert!(atof32_bytes(b"+Infinity", 10).is_infinite());
        assert!(atof32_bytes(b"-Infinity", 10).is_infinite());
    }

    #[test]
    fn atof32_basen_test() {
        assert_float_relative_eq!(1234.0, atof32_bytes(b"YA", 36));
    }

    #[test]
    fn atof64_base10_test() {
        // integer test
        assert_float_relative_eq!(0.0, atof64_bytes(b"0", 10), 1e-12);
        assert_float_relative_eq!(1.0, atof64_bytes(b"1", 10), 1e-12);
        assert_float_relative_eq!(12.0, atof64_bytes(b"12", 10), 1e-12);
        assert_float_relative_eq!(123.0, atof64_bytes(b"123", 10), 1e-12);
        assert_float_relative_eq!(1234.0, atof64_bytes(b"1234", 10), 1e-12);
        assert_float_relative_eq!(12345.0, atof64_bytes(b"12345", 10), 1e-12);
        assert_float_relative_eq!(123456.0, atof64_bytes(b"123456", 10), 1e-12);
        assert_float_relative_eq!(1234567.0, atof64_bytes(b"1234567", 10), 1e-12);
        assert_float_relative_eq!(12345678.0, atof64_bytes(b"12345678", 10), 1e-12);

        // decimal test
        assert_float_relative_eq!(123456789.0, atof64_bytes(b"123456789", 10), 1e-12);
        assert_float_relative_eq!(123456789.1, atof64_bytes(b"123456789.1", 10), 1e-12);
        assert_float_relative_eq!(123456789.12, atof64_bytes(b"123456789.12", 10), 1e-12);
        assert_float_relative_eq!(123456789.123, atof64_bytes(b"123456789.123", 10), 1e-12);
        assert_float_relative_eq!(123456789.1234, atof64_bytes(b"123456789.1234", 10), 1e-12);
        assert_float_relative_eq!(123456789.12345, atof64_bytes(b"123456789.12345", 10), 1e-12);
        assert_float_relative_eq!(123456789.123456, atof64_bytes(b"123456789.123456", 10), 1e-12);
        assert_float_relative_eq!(123456789.1234567, atof64_bytes(b"123456789.1234567", 10), 1e-12);
        assert_float_relative_eq!(123456789.12345678, atof64_bytes(b"123456789.12345678", 10), 1e-12);

        // rounding test
        assert_float_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.123456789", 10), 1e-12);
        assert_float_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.1234567890", 10), 1e-12);
        assert_float_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.123456789012", 10), 1e-12);
        assert_float_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.1234567890123", 10), 1e-12);
        assert_float_relative_eq!(123456789.12345679, atof64_bytes(b"123456789.12345678901234", 10), 1e-12);

        // exponent test
        assert_float_relative_eq!(123456789.12345, atof64_bytes(b"1.2345678912345e8", 10), 1e-12);
        assert_float_relative_eq!(123450000.0, atof64_bytes(b"1.2345e+8", 10), 1e-12);
        assert_float_relative_eq!(1.2345e+11, atof64_bytes(b"123450000000", 10), 1e-12);
        assert_float_relative_eq!(1.2345e+11, atof64_bytes(b"1.2345e+11", 10), 1e-12);
        assert_float_relative_eq!(1.2345e+38, atof64_bytes(b"1.2345e+38", 10), 1e-12);
        assert_float_relative_eq!(1.2345e+38, atof64_bytes(b"123450000000000000000000000000000000000", 10), 1e-12);
        assert_float_relative_eq!(1.2345e+308, atof64_bytes(b"1.2345e+308", 10), 1e-12);
        assert_float_relative_eq!(1.2345e+308, atof64_bytes(b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", 10), 1e-12);
        assert_float_relative_eq!(0.000000012345, atof64_bytes(b"1.2345e-8", 10), 1e-12);
        assert_float_relative_eq!(1.2345e-8, atof64_bytes(b"0.000000012345", 10), 1e-12);
        assert_float_relative_eq!(1.2345e-38, atof64_bytes(b"1.2345e-38", 10), 1e-12);
        assert_float_relative_eq!(1.2345e-38, atof64_bytes(b"0.000000000000000000000000000000000000012345", 10), 1e-12);
        assert_float_relative_eq!(1.2345e-308, atof64_bytes(b"1.2345e-308", 10), 1e-12);
        // due to issues in how the data is parsed, manually extracting
        // non-exponents of 1.<e-299 is prone to error
        // test the limit of our ability
        assert_float_relative_eq!(1.2345e-299, atof64_bytes(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", 10), 1e-12);

        #[cfg(feature = "std")]
        assert!(atof64_bytes(b"NaN", 10).is_nan());
        assert!(atof64_bytes(b"Infinity", 10).is_infinite());
        assert!(atof64_bytes(b"+Infinity", 10).is_infinite());
        assert!(atof64_bytes(b"-Infinity", 10).is_infinite());
    }

    #[test]
    fn atof64_basen_test() {
        assert_float_relative_eq!(1234.0, atof64_bytes(b"YA", 36));
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
