//! Fast lexical integer-to-string conversion routines.
//!
//! The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//! CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//! (x86-64), using the lexical formatter, `itoa::write()` or `x.to_string()`,
//! avoiding any inefficiencies in Rust string parsing for `format!(...)`
//! or `write!()` macros. The code was compiled with LTO and at an optimization
//! level of 3.
//!
//! The benchmarks with `std` were compiled using "rustc 1.29.2 (17a9dc751
//! 2018-10-05", and the `no_std` benchmarks were compiled using "rustc
//! 1.31.0-nightly (46880f41b 2018-10-15)".
//!
//! The benchmark code may be found `benches/itoa.rs`.
//!
//! # Benchmarks
//!
//! | Type  |  lexical (ns/iter) | to_string (ns/iter)   | Relative Increase |
//! |:-----:|:------------------:|:---------------------:|:-----------------:|
//! | u8    | 233,850            | 521,612               | 2.25x             |
//! | u16   | 263,126            | 513,183               | 2.13x             |
//! | u32   | 266,256            | 529,319               | 1.72x             |
//! | u64   | 335,878            | 645,835               | 1.47x             |
//! | i8    | 264,393            | 710,683               | 2.80x             |
//! | i16   | 277,071            | 709,717               | 2.48x             |
//! | i32   | 313,994            | 784,850               | 2.12x             |
//! | i64   | 335,098            | 825,617               | 1.99x             |
//!
//! # Raw Benchmarks
//!
//! ```text
//! test i8_itoa       ... bench:     290,879 ns/iter (+/- 20,785)
//! test i8_lexical    ... bench:     264,393 ns/iter (+/- 13,174)
//! test i8_to_string  ... bench:     710,683 ns/iter (+/- 29,733)
//! test i16_itoa      ... bench:     291,568 ns/iter (+/- 17,685)
//! test i16_lexical   ... bench:     277,071 ns/iter (+/- 12,155)
//! test i16_to_string ... bench:     709,717 ns/iter (+/- 36,272)
//! test i32_itoa      ... bench:     315,750 ns/iter (+/- 17,166)
//! test i32_lexical   ... bench:     313,994 ns/iter (+/- 24,824)
//! test i32_to_string ... bench:     784,850 ns/iter (+/- 60,596)
//! test i64_itoa      ... bench:     339,346 ns/iter (+/- 25,263)
//! test i64_lexical   ... bench:     335,098 ns/iter (+/- 16,897)
//! test i64_to_string ... bench:     825,617 ns/iter (+/- 27,940)
//! test u8_itoa       ... bench:     278,985 ns/iter (+/- 22,038)
//! test u8_lexical    ... bench:     233,850 ns/iter (+/- 8,531)
//! test u8_to_string  ... bench:     521,612 ns/iter (+/- 30,309)
//! test u16_itoa      ... bench:     288,058 ns/iter (+/- 57,947)
//! test u16_lexical   ... bench:     263,126 ns/iter (+/- 104,268)
//! test u16_to_string ... bench:     513,183 ns/iter (+/- 27,565)
//! test u32_itoa      ... bench:     271,674 ns/iter (+/- 6,385)
//! test u32_lexical   ... bench:     266,256 ns/iter (+/- 116,874)
//! test u32_to_string ... bench:     529,319 ns/iter (+/- 109,369)
//! test u64_itoa      ... bench:     360,856 ns/iter (+/- 131,510)
//! test u64_lexical   ... bench:     335,878 ns/iter (+/- 75,110)
//! test u64_to_string ... bench:     645,835 ns/iter (+/- 93,398)
//! ```
//!
//! Raw Benchmarks (`no_std`)
//!
//! ```text
//! test i8_itoa       ... bench:     595,005 ns/iter (+/- 36,626)
//! test i8_lexical    ... bench:     561,319 ns/iter (+/- 17,670)
//! test i8_to_string  ... bench:   1,123,246 ns/iter (+/- 41,451)
//! test i16_itoa      ... bench:     602,613 ns/iter (+/- 31,383)
//! test i16_lexical   ... bench:     597,835 ns/iter (+/- 18,976)
//! test i16_to_string ... bench:   1,162,493 ns/iter (+/- 61,947)
//! test i32_itoa      ... bench:     643,928 ns/iter (+/- 48,297)
//! test i32_lexical   ... bench:     625,825 ns/iter (+/- 127,002)
//! test i32_to_string ... bench:   1,199,091 ns/iter (+/- 220,981)
//! test i64_itoa      ... bench:     670,835 ns/iter (+/- 75,959)
//! test i64_lexical   ... bench:     688,899 ns/iter (+/- 99,429)
//! test i64_to_string ... bench:   1,239,407 ns/iter (+/- 157,723)
//! test u8_itoa       ... bench:     585,364 ns/iter (+/- 29,233)
//! test u8_lexical    ... bench:     562,703 ns/iter (+/- 32,110)
//! test u8_to_string  ... bench:     826,371 ns/iter (+/- 39,158)
//! test u16_itoa      ... bench:     589,813 ns/iter (+/- 23,505)
//! test u16_lexical   ... bench:     584,662 ns/iter (+/- 36,987)
//! test u16_to_string ... bench:     823,388 ns/iter (+/- 43,951)
//! test u32_itoa      ... bench:     622,236 ns/iter (+/- 11,931)
//! test u32_lexical   ... bench:     603,591 ns/iter (+/- 15,666)
//! test u32_to_string ... bench:     840,490 ns/iter (+/- 41,951)
//! test u64_itoa      ... bench:     664,002 ns/iter (+/- 29,050)
//! test u64_lexical   ... bench:     664,414 ns/iter (+/- 29,542)
//! test u64_to_string ... bench:     914,314 ns/iter (+/- 51,479)
//! ```

// Code the generate the benchmark plot:
//  import numpy as np
//  import pandas as pd
//  import matplotlib.pyplot as plt
//  plt.style.use('ggplot')
//  lexical = np.array([233850, 263126, 266256, 335878, 264393, 277071, 313994, 335098]) / 1e6
//  to_string = np.array([521612, 513183, 529319, 645835, 710683, 709717, 784850, 825617]) / 1e6
//  index = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"]
//  df = pd.DataFrame({'lexical': lexical, 'to_string': to_string}, index = index, columns=['lexical', 'to_string'])
//  ax = df.plot.bar(rot=0)
//  ax.set_ylabel("ms/iter")
//  ax.figure.tight_layout()
//  plt.show()

use lib::{mem, ptr};
use table::*;
use util::*;

// MACRO

/// Calculate the number of digits in a number, with a given base (radix).
#[inline]
fn digits<Value: Integer>(value: Value, base: u32) -> usize {
    match value.is_zero() {
        true  => 1,
        false => {
            let v: f64 = as_cast(value);
            let b: f64 = as_cast(base);
            let digits = ((v.ln() / b.ln()) + 1.0).floor();
            digits as usize
        }
    }
}

/// Check if the supplied buffer has enough range for the encoded size.
macro_rules! check_buffer {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        let has_space = distance($first, $last) >= digits($value, $base);
        debug_assert!(has_space, "Need a larger buffer.");
    })
}

// CONSTANTS

/// Maximum digits possible for a u64 export.
/// Value is `digits!(0XFFFFFFFFFFFFFFFF, 2)`, which is 65.
/// Up to the nearest power of 2, since it notably increases
/// performance (~25%) on x86-64 architectures.
const MAX_DIGITS: usize = 128;

// OPTIMIZED

/// Optimized implementation for base-N numbers.
///
/// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
///
/// `value` must be non-negative and mutable.
#[cfg(feature = "table")]
#[inline]
unsafe fn optimized<T>(mut value: T, base: T, table: *const u8, first: *mut u8)
    -> *mut u8
    where T: UnsignedInteger
{
    let base2 = base * base;
    let base4 = base2 * base2;

    if value == T::ZERO {
        *first = b'0';
        return first.add(1);
    }

    // Create a temporary buffer, and copy into it.
    // Way faster than reversing a buffer in-place.
    let mut buffer: [u8; MAX_DIGITS] = mem::uninitialized();
    let mut rem: usize;
    let mut curr = buffer.len();
    let p: *mut u8 = buffer.as_mut_ptr();

    // Decode 4 digits at a time
    while value >= base4 {
        let rem = value % base4;
        value /= base4;
        let r1: usize = as_cast(T::TWO * (rem / base2));
        let r2: usize = as_cast(T::TWO * (rem % base2));

        curr -= 4;
        ptr::copy_nonoverlapping(table.add(r1), p.add(curr), 2);
        ptr::copy_nonoverlapping(table.add(r2), p.add(curr + 2), 2);
    }

    // Decode 2 digits at a time.
    while value >= base2 {
        rem = as_cast(T::TWO * (value % base2));
        value /= base2;

        curr -= 2;
        ptr::copy_nonoverlapping(table.add(rem), p.add(curr), 2);
    }

    // Decode last 2 digits.
    if value < base {
        curr -= 1;
        *p.add(curr) = digit_to_char(value);
    } else {
        rem = as_cast(T::TWO * value);
        curr -= 2;
        ptr::copy_nonoverlapping(table.add(rem), p.add(curr), 2);
    }

    let len = buffer.len() - curr;
    ptr::copy_nonoverlapping(p.add(curr), first, len);

    first.add(len)
}

// NAIVE

/// Naive implementation for base-N numbers.
///
/// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
///
/// `value` must be non-negative and mutable.
#[cfg(not(feature = "table"))]
#[inline]
unsafe fn naive<T>(mut value: T, base: T, first: *mut u8)
    -> *mut u8
    where T: UnsignedInteger
{
    // Create a temporary buffer, and copy into it.
    // Way faster than reversing a buffer in-place.
    let mut buffer: [u8; MAX_DIGITS] = mem::uninitialized();
    let mut rem: usize;
    let mut curr = buffer.len();
    let p: *mut u8 = buffer.as_mut_ptr();

    // Decode all but last digit, 1 at a time.
    while value >= base {
        rem = as_cast(value % base);
        value /= base;

        curr -= 1;
        *p.add(curr) = digit_to_char(rem);
    }

    // Decode last digit.
    rem = as_cast(value % base);
    curr -= 1;
    *p.add(curr) = digit_to_char(rem);

    let len = buffer.len() - curr;
    ptr::copy_nonoverlapping(p.add(curr), first, len);

    first.add(len)
}

/// Forward the correct arguments to the implementation.
///
/// Use a macro to allow for u32 or u64 to be used (u32 is generally faster).
///
/// `value` must be non-negative and mutable.
#[inline]
pub(crate) unsafe fn forward<T>(value: T, base: u32, first: *mut u8)
    -> *mut u8
    where T: UnsignedInteger
{
    #[cfg(feature = "table")] {
        let table = match base {
            2   => DIGIT_TO_BASE2_SQUARED.as_ptr(),
            3   => DIGIT_TO_BASE3_SQUARED.as_ptr(),
            4   => DIGIT_TO_BASE4_SQUARED.as_ptr(),
            5   => DIGIT_TO_BASE5_SQUARED.as_ptr(),
            6   => DIGIT_TO_BASE6_SQUARED.as_ptr(),
            7   => DIGIT_TO_BASE7_SQUARED.as_ptr(),
            8   => DIGIT_TO_BASE8_SQUARED.as_ptr(),
            9   => DIGIT_TO_BASE9_SQUARED.as_ptr(),
            10  => DIGIT_TO_BASE10_SQUARED.as_ptr(),
            11  => DIGIT_TO_BASE11_SQUARED.as_ptr(),
            12  => DIGIT_TO_BASE12_SQUARED.as_ptr(),
            13  => DIGIT_TO_BASE13_SQUARED.as_ptr(),
            14  => DIGIT_TO_BASE14_SQUARED.as_ptr(),
            15  => DIGIT_TO_BASE15_SQUARED.as_ptr(),
            16  => DIGIT_TO_BASE16_SQUARED.as_ptr(),
            17  => DIGIT_TO_BASE17_SQUARED.as_ptr(),
            18  => DIGIT_TO_BASE18_SQUARED.as_ptr(),
            19  => DIGIT_TO_BASE19_SQUARED.as_ptr(),
            20  => DIGIT_TO_BASE20_SQUARED.as_ptr(),
            21  => DIGIT_TO_BASE21_SQUARED.as_ptr(),
            22  => DIGIT_TO_BASE22_SQUARED.as_ptr(),
            23  => DIGIT_TO_BASE23_SQUARED.as_ptr(),
            24  => DIGIT_TO_BASE24_SQUARED.as_ptr(),
            25  => DIGIT_TO_BASE25_SQUARED.as_ptr(),
            26  => DIGIT_TO_BASE26_SQUARED.as_ptr(),
            27  => DIGIT_TO_BASE27_SQUARED.as_ptr(),
            28  => DIGIT_TO_BASE28_SQUARED.as_ptr(),
            29  => DIGIT_TO_BASE29_SQUARED.as_ptr(),
            30  => DIGIT_TO_BASE30_SQUARED.as_ptr(),
            31  => DIGIT_TO_BASE31_SQUARED.as_ptr(),
            32  => DIGIT_TO_BASE32_SQUARED.as_ptr(),
            33  => DIGIT_TO_BASE33_SQUARED.as_ptr(),
            34  => DIGIT_TO_BASE34_SQUARED.as_ptr(),
            35  => DIGIT_TO_BASE35_SQUARED.as_ptr(),
            36  => DIGIT_TO_BASE36_SQUARED.as_ptr(),
            _   => unreachable!(),
        };
        let base: T = as_cast(base);
        optimized(value, base, table, first)
    }

    #[cfg(not(feature = "table"))] {
        let base: T = as_cast(base);
        naive(value, base, first)
    }
}

/// Sanitizer for an unsigned number-to-string implementation.
#[inline]
pub(crate) unsafe fn unsigned<Value, UWide>(value: Value, base: u32, first: *mut u8, last: *mut u8)
    -> *mut u8
    where Value: UnsignedInteger,
          UWide: UnsignedInteger
{
    // Sanity checks
    debug_assert!(first <= last);
    check_buffer!(value, first, last, base);

    // Invoke forwarder
    let v: UWide = as_cast(value);
    forward(v, base, first)
}

/// Sanitizer for an signed number-to-string implementation.
#[inline]
pub(crate) unsafe fn signed<Value, UWide, IWide>(value: Value, base: u32, mut first: *mut u8, last: *mut u8)
    -> *mut u8
    where Value: SignedInteger,
          UWide: UnsignedInteger,
          IWide: SignedInteger
{
    // Sanity checks
    debug_assert!(first <= last);
    check_buffer!(value, first, last, base);

    // Handle negative numbers, use an unsigned type to avoid overflow.
    // Use a wrapping neg to allow overflow.
    // These routines wrap on one condition, where the input number is equal
    // to the minimum possible value of that type (for example, -128 for i8).
    // In this case, and this case only, the value wraps to itself with
    // `x.wrapping_neg()`, so `-128i8.wrapping_neg() == -128i8` in two's
    // complement (the only true integer representation). Conversion of
    // this wrapped value to an unsigned integer of the same size with
    // effectively negates the value, for example, `-128i8 as u8 == 128u8`.
    // Due to type widening, this wrap only occurs for `i64::min_value()`,
    // and since it is converted to `u64`, this algorithm is correct
    // for all numerical input values, since Rust guarantees two's
    // complement representation for signed integers.
    let v: UWide;
    if value < Value::ZERO {
        *first = b'-';
        let wide: IWide = as_cast(value);
        v = as_cast(wide.wrapping_neg());
        first = first.add(1);
    } else {
        v = as_cast(value);
    }

    // Invoke forwarder
    forward(v, base, first)
}

// UNSAFE API

/// Generate the unsigned, unsafe wrappers.
macro_rules! generate_unsafe_unsigned {
    ($name:ident, $t:ty, $uwide:ty) => (
        /// Unsafe, C-like exporter for unsigned numbers.
        ///
        /// # Warning
        ///
        /// Do not call this function directly, unless you **know**
        /// you have a buffer of sufficient size. No size checking is
        /// done in release mode, this function is **highly** dangerous.
        /// Sufficient buffer sizes are as follows:
        ///
        /// `u8  -> 9`
        /// `u16 -> 17`
        /// `u32 -> 33`
        /// `u64 -> 65`
        #[inline]
        pub unsafe extern "C" fn $name(value: $t, base: u8, first: *mut u8, last: *mut u8) -> *mut u8
        {
            unsigned::<$t, $uwide>(value, base.into(), first, last)
        }
    )
}

generate_unsafe_unsigned!(u8toa_unsafe, u8, u32);
generate_unsafe_unsigned!(u16toa_unsafe, u16, u32);
generate_unsafe_unsigned!(u32toa_unsafe, u32, u32);
generate_unsafe_unsigned!(u64toa_unsafe, u64, u64);
generate_unsafe_unsigned!(usizetoa_unsafe, usize, usize);

/// Generate the signed, unsafe wrappers.
macro_rules! generate_unsafe_signed {
    ($name:ident, $t:ty, $uwide:ty, $iwide:ty) => (
        /// Unsafe, C-like exporter for signed numbers.
        ///
        /// # Warning
        ///
        /// Do not call this function directly, unless you **know**
        /// you have a buffer of sufficient size. No size checking is
        /// done in release mode, this function is **highly** dangerous.
        /// Sufficient buffer sizes are as follows:
        ///
        /// `u8  -> 9`
        /// `u16 -> 17`
        /// `u32 -> 33`
        /// `u64 -> 65`
        #[inline]
        pub unsafe extern "C" fn $name(value: $t, base: u8, first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            signed::<$t, $uwide, $iwide>(value, base.into(), first, last)
        }
    )
}

generate_unsafe_signed!(i8toa_unsafe, i8, u32, i32);
generate_unsafe_signed!(i16toa_unsafe, i16, u32, i32);
generate_unsafe_signed!(i32toa_unsafe, i32, u32, i32);
generate_unsafe_signed!(i64toa_unsafe, i64, u64, i64);
generate_unsafe_signed!(isizetoa_unsafe, isize, usize, isize);

// LOW-LEVEL API
// -------------

// Use powers of 2 for allocation.
// It really doesn't, make a difference here, especially since
// the value is just a suggestion for the vector.

// WRAP UNSAFE LOCAL
generate_to_bytes_local!(u8toa_local, u8, u8toa_unsafe);
generate_to_bytes_local!(u16toa_local, u16, u16toa_unsafe);
generate_to_bytes_local!(u32toa_local, u32, u32toa_unsafe);
generate_to_bytes_local!(u64toa_local, u64, u64toa_unsafe);
generate_to_bytes_local!(usizetoa_local, usize, usizetoa_unsafe);
generate_to_bytes_local!(i8toa_local, i8, i8toa_unsafe);
generate_to_bytes_local!(i16toa_local, i16, i16toa_unsafe);
generate_to_bytes_local!(i32toa_local, i32, i32toa_unsafe);
generate_to_bytes_local!(i64toa_local, i64, i64toa_unsafe);
generate_to_bytes_local!(isizetoa_local, isize, isizetoa_unsafe);

// API
generate_to_bytes_api!(u8toa_bytes, u8, u8toa_local, 16);            // 9
generate_to_bytes_api!(u16toa_bytes, u16, u16toa_local, 32);         // 17
generate_to_bytes_api!(u32toa_bytes, u32, u32toa_local, 64);         // 33
generate_to_bytes_api!(u64toa_bytes, u64, u64toa_local, 128);        // 65
generate_to_bytes_api!(usizetoa_bytes, usize, usizetoa_local, 128);  // 65
generate_to_bytes_api!(i8toa_bytes, i8, i8toa_local, 16);            // 9
generate_to_bytes_api!(i16toa_bytes, i16, i16toa_local, 32);         // 17
generate_to_bytes_api!(i32toa_bytes, i32, i32toa_local, 64);         // 33
generate_to_bytes_api!(i64toa_bytes, i64, i64toa_local, 128);        // 65
generate_to_bytes_api!(isizetoa_bytes, isize, isizetoa_local, 128);  // 65

// TESTS
// -----

#[cfg(test)]
mod tests {
    use atoi::*;
    use super::*;

    #[test]
    fn u8toa_test() {
        assert_eq!(b"0".to_vec(), u8toa_bytes(0, 10));
        assert_eq!(b"1".to_vec(), u8toa_bytes(1, 10));
        assert_eq!(b"127".to_vec(), u8toa_bytes(127, 10));
        assert_eq!(b"128".to_vec(), u8toa_bytes(128, 10));
        assert_eq!(b"255".to_vec(), u8toa_bytes(255, 10));
        assert_eq!(b"255".to_vec(), u8toa_bytes(-1i8 as u8, 10));
    }

    #[test]
    fn i8toa_test() {
        assert_eq!(b"0".to_vec(), i8toa_bytes(0, 10));
        assert_eq!(b"1".to_vec(), i8toa_bytes(1, 10));
        assert_eq!(b"127".to_vec(), i8toa_bytes(127, 10));
        assert_eq!(b"-128".to_vec(), i8toa_bytes(128u8 as i8, 10));
        assert_eq!(b"-1".to_vec(), i8toa_bytes(255u8 as i8, 10));
        assert_eq!(b"-1".to_vec(), i8toa_bytes(-1, 10));
    }

    #[test]
    fn u16toa_test() {
        assert_eq!(b"0".to_vec(), u16toa_bytes(0, 10));
        assert_eq!(b"1".to_vec(), u16toa_bytes(1, 10));
        assert_eq!(b"32767".to_vec(), u16toa_bytes(32767, 10));
        assert_eq!(b"32768".to_vec(), u16toa_bytes(32768, 10));
        assert_eq!(b"65535".to_vec(), u16toa_bytes(65535, 10));
        assert_eq!(b"65535".to_vec(), u16toa_bytes(-1i16 as u16, 10));
    }

    #[test]
    fn i16toa_test() {
        assert_eq!(b"0".to_vec(), i16toa_bytes(0, 10));
        assert_eq!(b"1".to_vec(), i16toa_bytes(1, 10));
        assert_eq!(b"32767".to_vec(), i16toa_bytes(32767, 10));
        assert_eq!(b"-32768".to_vec(), i16toa_bytes(32768u16 as i16, 10));
        assert_eq!(b"-1".to_vec(), i16toa_bytes(65535u16 as i16, 10));
        assert_eq!(b"-1".to_vec(), i16toa_bytes(-1, 10));
    }

    #[test]
    fn u32toa_test() {
        assert_eq!(b"0".to_vec(), u32toa_bytes(0, 10));
        assert_eq!(b"1".to_vec(), u32toa_bytes(1, 10));
        assert_eq!(b"2147483647".to_vec(), u32toa_bytes(2147483647, 10));
        assert_eq!(b"2147483648".to_vec(), u32toa_bytes(2147483648, 10));
        assert_eq!(b"4294967295".to_vec(), u32toa_bytes(4294967295, 10));
        assert_eq!(b"4294967295".to_vec(), u32toa_bytes(-1i32 as u32, 10));
    }

    #[test]
    fn i32toa_test() {
        assert_eq!(b"0".to_vec(), i32toa_bytes(0, 10));
        assert_eq!(b"1".to_vec(), i32toa_bytes(1, 10));
        assert_eq!(b"2147483647".to_vec(), i32toa_bytes(2147483647, 10));
        assert_eq!(b"-2147483648".to_vec(), i32toa_bytes(2147483648u32 as i32, 10));
        assert_eq!(b"-1".to_vec(), i32toa_bytes(4294967295u32 as i32, 10));
        assert_eq!(b"-1".to_vec(), i32toa_bytes(-1, 10));
    }

    #[test]
    fn u64toa_test() {
        assert_eq!(b"0".to_vec(), u64toa_bytes(0, 10));
        assert_eq!(b"1".to_vec(), u64toa_bytes(1, 10));
        assert_eq!(b"9223372036854775807".to_vec(), u64toa_bytes(9223372036854775807, 10));
        assert_eq!(b"9223372036854775808".to_vec(), u64toa_bytes(9223372036854775808, 10));
        assert_eq!(b"18446744073709551615".to_vec(), u64toa_bytes(18446744073709551615, 10));
        assert_eq!(b"18446744073709551615".to_vec(), u64toa_bytes(-1i64 as u64, 10));
    }

    #[test]
    fn i64toa_test() {
        assert_eq!(b"0".to_vec(), i64toa_bytes(0, 10));
        assert_eq!(b"1".to_vec(), i64toa_bytes(1, 10));
        assert_eq!(b"9223372036854775807".to_vec(), i64toa_bytes(9223372036854775807, 10));
        assert_eq!(b"-9223372036854775808".to_vec(), i64toa_bytes(9223372036854775808u64 as i64, 10));
        assert_eq!(b"-1".to_vec(), i64toa_bytes(18446744073709551615u64 as i64, 10));
        assert_eq!(b"-1".to_vec(), i64toa_bytes(-1, 10));
    }

    #[test]
    fn basen_test() {
        let data = [
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

        for (base, expected) in data.iter() {
            assert_eq!(expected.as_bytes().to_vec(), i8toa_bytes(37, *base));
        }
    }

    quickcheck! {
        fn u8_quickcheck(i: u8) -> bool {
            i == atou8_bytes(10, u8toa_bytes(i, 10).as_slice())
        }

        fn u16_quickcheck(i: u16) -> bool {
            i == atou16_bytes(10, u16toa_bytes(i, 10).as_slice())
        }

        fn u32_quickcheck(i: u32) -> bool {
            i == atou32_bytes(10, u32toa_bytes(i, 10).as_slice())
        }

        fn u64_quickcheck(i: u64) -> bool {
            i == atou64_bytes(10, u64toa_bytes(i, 10).as_slice())
        }

        fn usize_quickcheck(i: usize) -> bool {
            i == atousize_bytes(10, usizetoa_bytes(i, 10).as_slice())
        }

        fn i8_quickcheck(i: i8) -> bool {
            i == atoi8_bytes(10, i8toa_bytes(i, 10).as_slice())
        }

        fn i16_quickcheck(i: i16) -> bool {
            i == atoi16_bytes(10, i16toa_bytes(i, 10).as_slice())
        }

        fn i32_quickcheck(i: i32) -> bool {
            i == atoi32_bytes(10, i32toa_bytes(i, 10).as_slice())
        }

        fn i64_quickcheck(i: i64) -> bool {
            i == atoi64_bytes(10, i64toa_bytes(i, 10).as_slice())
        }

        fn isize_quickcheck(i: isize) -> bool {
            i == atoisize_bytes(10, isizetoa_bytes(i, 10).as_slice())
        }
    }
}
