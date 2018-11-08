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

use sealed::mem;
use sealed::ptr;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::string::String;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::vec::Vec;

use table::*;
use util::{distance, floor, ln};

// MACRO

/// Calculate the number of digits in a number, with a given base (radix).
macro_rules! digits {
    ($value:ident, $base:ident) => ({
        match $value {
            0 => 1,
            _ => {
                let v = $value as f64;
                let b = $base as f64;
                let digits = floor((ln(v) / ln(b)) + 1.0);
                digits as usize
            }
        }
    })
}

/// Check if the supplied buffer has enough range for the encoded size.
macro_rules! check_digits {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        debug_assert!(distance($first, $last) >= digits!($value, $base), "Need a larger buffer.");
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
#[cfg(feature = "table")]
#[inline]
unsafe extern "C" fn itoa_optimized(
    mut value: u64,
    first: *mut u8,
    base: u64,
    table: *const u8
)
    -> *mut u8
{
    let base2 = base * base;
    let base4 = base2 * base2;

    if value == 0 {
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
        let r1 = (2 * (rem / base2)) as usize;
        let r2 = (2 * (rem % base2)) as usize;

        curr -= 4;
        ptr::copy_nonoverlapping(table.add(r1), p.add(curr), 2);
        ptr::copy_nonoverlapping(table.add(r2), p.add(curr + 2), 2);
    }

    // Decode 2 digits at a time.
    while value >= base2 {
        rem = (2 * (value % base2)) as usize;
        value /= base2;

        curr -= 2;
        ptr::copy_nonoverlapping(table.add(rem), p.add(curr), 2);
    }

    // Decode last 2 digits.
    if value < base {
        curr -= 1;
        *p.add(curr) = *BASEN.get_unchecked(value as usize);
    } else {
        let rem = 2 * value as usize;
        curr -= 2;
        ptr::copy_nonoverlapping(table.add(rem), p.add(curr), 2);
    }

    let len = buffer.len() - curr;
    ptr::copy_nonoverlapping(p.add(curr), first, len);

    first.add(len)
}

// NAIVE

/// Naive implementation for base-N numbers.
#[cfg(not(feature = "table"))]
#[inline]
unsafe extern "C" fn itoa_naive(
    mut value: u64,
    first: *mut u8,
    base: u64
)
    -> *mut u8
{
    // Logic error, base should not be passed dynamically.
    debug_assert!(base >= 2 && base <= 36,"Numerical base must be from 2-36");

    // Create a temporary buffer, and copy into it.
    // Way faster than reversing a buffer in-place.
    let mut buffer: [u8; MAX_DIGITS] = mem::uninitialized();
    let mut rem: usize;
    let mut curr = buffer.len();
    let p: *mut u8 = buffer.as_mut_ptr();

    // Decode all but last digit, 1 at a time.
    while value >= base {
        rem = (value % base) as usize;
        value /= base;

        curr -= 1;
        *p.add(curr) = *BASEN.get_unchecked(rem);
    }

    // Decode last digit.
    rem = (value % base) as usize;
    curr -= 1;
    *p.add(curr) = *BASEN.get_unchecked(rem);

    let len = buffer.len() - curr;
    ptr::copy_nonoverlapping(p.add(curr), first, len);

    first.add(len)
}


/// Forward the correct arguments to the implementation.
#[inline]
pub(crate) unsafe extern "C" fn itoa_forward(
    value: u64,
    first: *mut u8,
    base: u64
)    -> *mut u8
{
    #[cfg(feature = "table")]
    match base {
        2   => itoa_optimized(value, first, base, BASE2.as_ptr()),
        3   => itoa_optimized(value, first, base, BASE3.as_ptr()),
        4   => itoa_optimized(value, first, base, BASE4.as_ptr()),
        5   => itoa_optimized(value, first, base, BASE5.as_ptr()),
        6   => itoa_optimized(value, first, base, BASE6.as_ptr()),
        7   => itoa_optimized(value, first, base, BASE7.as_ptr()),
        8   => itoa_optimized(value, first, base, BASE8.as_ptr()),
        9   => itoa_optimized(value, first, base, BASE9.as_ptr()),
        10  => itoa_optimized(value, first, base, BASE10.as_ptr()),
        11  => itoa_optimized(value, first, base, BASE11.as_ptr()),
        12  => itoa_optimized(value, first, base, BASE12.as_ptr()),
        13  => itoa_optimized(value, first, base, BASE13.as_ptr()),
        14  => itoa_optimized(value, first, base, BASE14.as_ptr()),
        15  => itoa_optimized(value, first, base, BASE15.as_ptr()),
        16  => itoa_optimized(value, first, base, BASE16.as_ptr()),
        17  => itoa_optimized(value, first, base, BASE17.as_ptr()),
        18  => itoa_optimized(value, first, base, BASE18.as_ptr()),
        19  => itoa_optimized(value, first, base, BASE19.as_ptr()),
        20  => itoa_optimized(value, first, base, BASE20.as_ptr()),
        21  => itoa_optimized(value, first, base, BASE21.as_ptr()),
        22  => itoa_optimized(value, first, base, BASE22.as_ptr()),
        23  => itoa_optimized(value, first, base, BASE23.as_ptr()),
        24  => itoa_optimized(value, first, base, BASE24.as_ptr()),
        25  => itoa_optimized(value, first, base, BASE25.as_ptr()),
        26  => itoa_optimized(value, first, base, BASE26.as_ptr()),
        27  => itoa_optimized(value, first, base, BASE27.as_ptr()),
        28  => itoa_optimized(value, first, base, BASE28.as_ptr()),
        29  => itoa_optimized(value, first, base, BASE29.as_ptr()),
        30  => itoa_optimized(value, first, base, BASE30.as_ptr()),
        31  => itoa_optimized(value, first, base, BASE31.as_ptr()),
        32  => itoa_optimized(value, first, base, BASE32.as_ptr()),
        33  => itoa_optimized(value, first, base, BASE33.as_ptr()),
        34  => itoa_optimized(value, first, base, BASE34.as_ptr()),
        35  => itoa_optimized(value, first, base, BASE35.as_ptr()),
        36  => itoa_optimized(value, first, base, BASE36.as_ptr()),
        _   => unreachable!(),
    }

    #[cfg(not(feature = "table"))]
    itoa_naive(value, first, base)
}

/// Sanitizer for an unsigned number-to-string implementation.
macro_rules! itoa_unsigned {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        // Sanity checks
        debug_assert!($first <= $last);
        check_digits!($value, $first, $last, $base);

        // Invoke forwarder
        let v = $value as u64;
        let b = $base as u64;
        itoa_forward(v, $first, b)
    })
}

/// Sanitizer for an signed number-to-string implementation.
macro_rules! itoa_signed {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        // Sanity checks
        debug_assert!($first <= $last);
        check_digits!($value, $first, $last, $base);

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
        let v: u64;
        if $value < 0 {
            *$first = b'-';
            v = ($value as i64).wrapping_neg() as u64;
            $first = $first.add(1);
        } else {
            v = $value as u64;
        }

        // Invoke forwarder
        let b = $base as u64;
        itoa_forward(v, $first, b)
    })
}

// UNSAFE API

/// Generate the unsigned, unsafe wrappers.
macro_rules! unsigned_unsafe_impl {
    ($func:ident, $t:ty) => (
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
        pub unsafe extern "C" fn $func(
            value: $t,
            first: *mut u8,
            last: *mut u8,
            base: u8
        )
            -> *mut u8
        {
            itoa_unsigned!(value, first, last, base)
        }
    )
}

unsigned_unsafe_impl!(u8toa_unsafe, u8);
unsigned_unsafe_impl!(u16toa_unsafe, u16);
unsigned_unsafe_impl!(u32toa_unsafe, u32);
unsigned_unsafe_impl!(u64toa_unsafe, u64);
unsigned_unsafe_impl!(usizetoa_unsafe, usize);

/// Generate the signed, unsafe wrappers.
macro_rules! signed_unsafe_impl {
    ($func:ident, $t:ty) => (
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
        pub unsafe extern "C" fn $func(
            value: $t,
            mut first: *mut u8,
            last: *mut u8,
            base: u8
        )
            -> *mut u8
        {
            itoa_signed!(value, first, last, base)
        }
    )
}

signed_unsafe_impl!(i8toa_unsafe, i8);
signed_unsafe_impl!(i16toa_unsafe, i16);
signed_unsafe_impl!(i32toa_unsafe, i32);
signed_unsafe_impl!(i64toa_unsafe, i64);
signed_unsafe_impl!(isizetoa_unsafe, isize);

// LOW-LEVEL API

// Use powers of 2 for allocation.
// It really doesn't, make a difference here, especially since
// the value is just a suggestion for the vector.
#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(u8toa_string, u8, u8toa_unsafe, 16);     // 9

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(u16toa_string, u16, u16toa_unsafe, 32);  // 17

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(u32toa_string, u32, u32toa_unsafe, 64);  // 33

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(u64toa_string, u64, u64toa_unsafe, 128); // 65

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(usizetoa_string, usize, usizetoa_unsafe, 128); // 65

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(i8toa_string, i8, i8toa_unsafe, 16);     // 9

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(i16toa_string, i16, i16toa_unsafe, 32);  // 17

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(i32toa_string, i32, i32toa_unsafe, 64);  // 33

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(i64toa_string, i64, i64toa_unsafe, 128); // 65

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(isizetoa_string, isize, isizetoa_unsafe, 128); // 65

// TESTS
// -----

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8toa_test() {
        assert_eq!("0", u8toa_string(0, 10));
        assert_eq!("1", u8toa_string(1, 10));
        assert_eq!("127", u8toa_string(127, 10));
        assert_eq!("128", u8toa_string(128, 10));
        assert_eq!("255", u8toa_string(255, 10));
        assert_eq!("255", u8toa_string(-1i8 as u8, 10));
    }

    #[test]
    fn i8toa_test() {
        assert_eq!("0", i8toa_string(0, 10));
        assert_eq!("1", i8toa_string(1, 10));
        assert_eq!("127", i8toa_string(127, 10));
        assert_eq!("-128", i8toa_string(128u8 as i8, 10));
        assert_eq!("-1", i8toa_string(255u8 as i8, 10));
        assert_eq!("-1", i8toa_string(-1, 10));
    }

    #[test]
    fn u16toa_test() {
        assert_eq!("0", u16toa_string(0, 10));
        assert_eq!("1", u16toa_string(1, 10));
        assert_eq!("32767", u16toa_string(32767, 10));
        assert_eq!("32768", u16toa_string(32768, 10));
        assert_eq!("65535", u16toa_string(65535, 10));
        assert_eq!("65535", u16toa_string(-1i16 as u16, 10));
    }

    #[test]
    fn i16toa_test() {
        assert_eq!("0", i16toa_string(0, 10));
        assert_eq!("1", i16toa_string(1, 10));
        assert_eq!("32767", i16toa_string(32767, 10));
        assert_eq!("-32768", i16toa_string(32768u16 as i16, 10));
        assert_eq!("-1", i16toa_string(65535u16 as i16, 10));
        assert_eq!("-1", i16toa_string(-1, 10));
    }

    #[test]
    fn u32toa_test() {
        assert_eq!("0", u32toa_string(0, 10));
        assert_eq!("1", u32toa_string(1, 10));
        assert_eq!("2147483647", u32toa_string(2147483647, 10));
        assert_eq!("2147483648", u32toa_string(2147483648, 10));
        assert_eq!("4294967295", u32toa_string(4294967295, 10));
        assert_eq!("4294967295", u32toa_string(-1i32 as u32, 10));
    }

    #[test]
    fn i32toa_test() {
        assert_eq!("0", i32toa_string(0, 10));
        assert_eq!("1", i32toa_string(1, 10));
        assert_eq!("2147483647", i32toa_string(2147483647, 10));
        assert_eq!("-2147483648", i32toa_string(2147483648u32 as i32, 10));
        assert_eq!("-1", i32toa_string(4294967295u32 as i32, 10));
        assert_eq!("-1", i32toa_string(-1, 10));
    }

    #[test]
    fn u64toa_test() {
        assert_eq!("0", u64toa_string(0, 10));
        assert_eq!("1", u64toa_string(1, 10));
        assert_eq!("9223372036854775807", u64toa_string(9223372036854775807, 10));
        assert_eq!("9223372036854775808", u64toa_string(9223372036854775808, 10));
        assert_eq!("18446744073709551615", u64toa_string(18446744073709551615, 10));
        assert_eq!("18446744073709551615", u64toa_string(-1i64 as u64, 10));
    }

    #[test]
    fn i64toa_test() {
        assert_eq!("0", i64toa_string(0, 10));
        assert_eq!("1", i64toa_string(1, 10));
        assert_eq!("9223372036854775807", i64toa_string(9223372036854775807, 10));
        assert_eq!("-9223372036854775808", i64toa_string(9223372036854775808u64 as i64, 10));
        assert_eq!("-1", i64toa_string(18446744073709551615u64 as i64, 10));
        assert_eq!("-1", i64toa_string(-1, 10));
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
            assert_eq!(*expected, i8toa_string(37, *base));
        }
    }
}
