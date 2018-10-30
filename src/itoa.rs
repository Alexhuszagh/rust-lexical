//! Fast lexical integer-to-string conversion routines.
//!
//! The following benchmarks were run on an "Intel(R) Core(TM) i7-6560U
//! CPU @ 2.20GHz" CPU, on Fedora 28, Linux kernel version 4.18.16-200
//! (x86-64), using either the itoa formatter or `x.to_string()`,
//! avoiding any inefficiencies in Rust string parsing for `format!(...)`
//! or `write!()` macros. The code was compiled with LTO and at an optimization
//! level of 3.
//!
//! The benchmark code may be found `benches/itoa.rs`.
//!
//! # Benchmarks
//!
//! | Type  |  itoa (ns/iter)   | to_string (ns/iter)   | Percent Increase  |
//! |:-----:|:-----------------:|:---------------------:|:-----------------:|
//! | u8    | 251,526           | 565,540               | 225%              |
//! | u16   | 253,976           | 541,471               | 213%              |
//! | u32   | 321,663           | 554,155               | 172%              |
//! | u64   | 467,457           | 687,727               | 147%              |
//! | i8    | 267,711           | 749,067               | 280%              |
//! | i16   | 308,417           | 767,189               | 248%              |
//! | i32   | 397,399           | 847,318               | 212%              |
//! | i64   | 456,488           | 909,026               | 199%              |
//!
//! # Raw Benchmarks
//!
//! ```text
//! test i8_itoa       ... bench:     267,711 ns/iter (+/- 15,109)
//! test i8_to_string  ... bench:     749,067 ns/iter (+/- 57,279)
//! test i16_itoa      ... bench:     308,417 ns/iter (+/- 14,001)
//! test i16_to_string ... bench:     767,189 ns/iter (+/- 94,647)
//! test i32_itoa      ... bench:     397,399 ns/iter (+/- 32,418)
//! test i32_to_string ... bench:     847,318 ns/iter (+/- 223,192)
//! test i64_itoa      ... bench:     456,488 ns/iter (+/- 23,833)
//! test i64_to_string ... bench:     909,026 ns/iter (+/- 79,991)
//! test u8_itoa       ... bench:     251,526 ns/iter (+/- 8,546)
//! test u8_to_string  ... bench:     565,540 ns/iter (+/- 28,796)
//! test u16_itoa      ... bench:     253,976 ns/iter (+/- 13,588)
//! test u16_to_string ... bench:     541,471 ns/iter (+/- 44,584)
//! test u32_itoa      ... bench:     321,663 ns/iter (+/- 21,334)
//! test u32_to_string ... bench:     554,155 ns/iter (+/- 26,865)
//! test u64_itoa      ... bench:     467,457 ns/iter (+/- 24,331)
//! test u64_to_string ... bench:     687,727 ns/iter (+/- 29,603)
//! ```

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::string::String;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::vec::Vec;

use super::c::{distance, reverse};
use super::table::*;

// INSTRINSICS

/// `f64.floor()` feature for `no_std`
#[cfg(not(feature = "std"))]
#[inline(always)]
fn floor(f: f64) -> f64 {
    unsafe { core::intrinsics::floorf64(f) }
}

/// `f64.ln()` feature for `no_std`
#[cfg(not(feature = "std"))]
#[inline(always)]
fn ln(f: f64) -> f64 {
    unsafe { core::intrinsics::logf64(f) }
}

/// `f64.floor()` feature for `std`
#[cfg(feature = "std")]
#[inline(always)]
fn floor(f: f64) -> f64 {
    f.floor()
}

/// `f64.ln()` feature for `std`
#[cfg(feature = "std")]
#[inline(always)]
fn ln(f: f64) -> f64 {
    f.ln()
}

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

// OPTIMIZED

/// Optimized implementation for base-N numbers.
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

    if value == 0 {
        *first = b'0';
        return first.add(1);
    }

    let mut rem: usize;
    let mut p: *mut u8 = first;
    while value >= base2 {
        rem = (2 * (value % base2)) as usize;
        value /= base2;
        *p = *table.add(rem+1);
        p = p.add(1);
        *p = *table.add(rem);
        p = p.add(1);
    }

    while value > 0 {
        rem = (value % base) as usize;
        *p = *BASEN.get_unchecked(rem);
        p = p.add(1);
        value /= base;
    }

    reverse(first, p);
    p
}

// NAIVE

/// Naive implementation for base-N numbers.
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

    let mut rem: usize;
    let mut p: *mut u8 = first;
    while value >= base {
        rem = (value % base) as usize;
        value /= base;
        *p = *BASEN.get_unchecked(rem);
        p = p.add(1);
    }

    rem = (value % base) as usize;
    *p = *BASEN.get_unchecked(rem);
    p = p.add(1);

    reverse(first, p);
    p
}


/// Forward the correct arguments to the implementation.
macro_rules! itoa_forward {
    ($value:ident, $first:ident, $base:ident) => (match $base {
        2   => itoa_optimized($value, $first, $base, BASE2.as_ptr()),
        3   => itoa_optimized($value, $first, $base, BASE3.as_ptr()),
        4   => itoa_optimized($value, $first, $base, BASE4.as_ptr()),
        5   => itoa_optimized($value, $first, $base, BASE5.as_ptr()),
        6   => itoa_optimized($value, $first, $base, BASE6.as_ptr()),
        7   => itoa_optimized($value, $first, $base, BASE7.as_ptr()),
        8   => itoa_optimized($value, $first, $base, BASE8.as_ptr()),
        9   => itoa_optimized($value, $first, $base, BASE9.as_ptr()),
        10  => itoa_optimized($value, $first, $base, BASE10.as_ptr()),
        11  => itoa_optimized($value, $first, $base, BASE11.as_ptr()),
        12  => itoa_optimized($value, $first, $base, BASE12.as_ptr()),
        13  => itoa_optimized($value, $first, $base, BASE13.as_ptr()),
        14  => itoa_optimized($value, $first, $base, BASE14.as_ptr()),
        15  => itoa_optimized($value, $first, $base, BASE15.as_ptr()),
        16  => itoa_optimized($value, $first, $base, BASE16.as_ptr()),
        17  => itoa_optimized($value, $first, $base, BASE17.as_ptr()),
        18  => itoa_optimized($value, $first, $base, BASE18.as_ptr()),
        19  => itoa_optimized($value, $first, $base, BASE19.as_ptr()),
        20  => itoa_optimized($value, $first, $base, BASE20.as_ptr()),
        21  => itoa_optimized($value, $first, $base, BASE21.as_ptr()),
        22  => itoa_optimized($value, $first, $base, BASE22.as_ptr()),
        23  => itoa_optimized($value, $first, $base, BASE23.as_ptr()),
        24  => itoa_optimized($value, $first, $base, BASE24.as_ptr()),
        25  => itoa_optimized($value, $first, $base, BASE25.as_ptr()),
        26  => itoa_optimized($value, $first, $base, BASE26.as_ptr()),
        27  => itoa_optimized($value, $first, $base, BASE27.as_ptr()),
        28  => itoa_optimized($value, $first, $base, BASE28.as_ptr()),
        29  => itoa_optimized($value, $first, $base, BASE29.as_ptr()),
        30  => itoa_optimized($value, $first, $base, BASE30.as_ptr()),
        31  => itoa_optimized($value, $first, $base, BASE31.as_ptr()),
        32  => itoa_optimized($value, $first, $base, BASE32.as_ptr()),
        33  => itoa_optimized($value, $first, $base, BASE33.as_ptr()),
        34  => itoa_optimized($value, $first, $base, BASE34.as_ptr()),
        35  => itoa_optimized($value, $first, $base, BASE35.as_ptr()),
        36  => itoa_optimized($value, $first, $base, BASE36.as_ptr()),
        _   => itoa_naive($value, $first, $base),
    })
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
        itoa_forward!(v, $first, b)
    })
}

/// Sanitizer for an signed number-to-string implementation.
macro_rules! itoa_signed {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        // Sanity checks
        debug_assert!($first <= $last);
        check_digits!($value, $first, $last, $base);

        // Handle negative numbers, use an unsigned type to avoid overflow.
        let v: u64;
        if $value < 0 {
            *$first = b'-';
            v = (-($value as i64)) as u64;
            $first = $first.add(1);
        } else {
            v = $value as u64;
        }

        // Invoke forwarder
        let b = $base as u64;
        itoa_forward!(v, $first, b)
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

// LOW-LEVEL API

/// Generate the low-level bytes API using wrappers around the unsafe function.
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! bytes_impl {
    ($func:ident, $t:ty, $callback:ident, $capacity:expr) => (
        /// Low-level bytes exporter for numbers.
        pub fn $func(value: $t, base: u8)
            -> Vec<u8>
        {
            let mut buf: Vec<u8> = Vec::with_capacity($capacity);
            unsafe {
                let first: *mut u8 = buf.as_mut_ptr();
                let last = first.add(buf.capacity());
                let end = $callback(value, first, last, base);
                let size = distance(first, end);
                buf.set_len(size);
            }

            buf
        }
    )
}

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(u8toa_bytes, u8, u8toa_unsafe, 9);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(u16toa_bytes, u16, u16toa_unsafe, 17);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(u32toa_bytes, u32, u32toa_unsafe, 33);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(u64toa_bytes, u64, u64toa_unsafe, 65);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(i8toa_bytes, i8, i8toa_unsafe, 9);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(i16toa_bytes, i16, i16toa_unsafe, 17);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(i32toa_bytes, i32, i32toa_unsafe, 33);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(i64toa_bytes, i64, i64toa_unsafe, 65);

/// Generate the low-level string API using wrappers around the bytes function.
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! string_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level string exporter for numbers.
        pub fn $func(value: $t, base: u8)
            -> String
        {
            unsafe { String::from_utf8_unchecked($callback(value, base)) }
        }
    )
}

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(u8toa_string, u8, u8toa_bytes);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(u16toa_string, u16, u16toa_bytes);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(u32toa_string, u32, u32toa_bytes);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(u64toa_string, u64, u64toa_bytes);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(i8toa_string, i8, i8toa_bytes);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(i16toa_string, i16, i16toa_bytes);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(i32toa_string, i32, i32toa_bytes);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(i64toa_string, i64, i64toa_bytes);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn u8toa_test() {
        assert_eq!("0", u8toa_string(0, 10));
        assert_eq!("1", u8toa_string(1, 10));
        assert_eq!("127", u8toa_string(127, 10));
        assert_eq!("128", u8toa_string(128, 10));
        assert_eq!("255", u8toa_string(255, 10));
        assert_eq!("255", u8toa_string(-1i8 as u8, 10));
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn i8toa_test() {
        assert_eq!("0", i8toa_string(0, 10));
        assert_eq!("1", i8toa_string(1, 10));
        assert_eq!("127", i8toa_string(127, 10));
        assert_eq!("-128", i8toa_string(128u8 as i8, 10));
        assert_eq!("-1", i8toa_string(255u8 as i8, 10));
        assert_eq!("-1", i8toa_string(-1, 10));
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn u16toa_test() {
        assert_eq!("0", u16toa_string(0, 10));
        assert_eq!("1", u16toa_string(1, 10));
        assert_eq!("32767", u16toa_string(32767, 10));
        assert_eq!("32768", u16toa_string(32768, 10));
        assert_eq!("65535", u16toa_string(65535, 10));
        assert_eq!("65535", u16toa_string(-1i16 as u16, 10));
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn i16toa_test() {
        assert_eq!("0", i16toa_string(0, 10));
        assert_eq!("1", i16toa_string(1, 10));
        assert_eq!("32767", i16toa_string(32767, 10));
        assert_eq!("-32768", i16toa_string(32768u16 as i16, 10));
        assert_eq!("-1", i16toa_string(65535u16 as i16, 10));
        assert_eq!("-1", i16toa_string(-1, 10));
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn u32toa_test() {
        assert_eq!("0", u32toa_string(0, 10));
        assert_eq!("1", u32toa_string(1, 10));
        assert_eq!("2147483647", u32toa_string(2147483647, 10));
        assert_eq!("2147483648", u32toa_string(2147483648, 10));
        assert_eq!("4294967295", u32toa_string(4294967295, 10));
        assert_eq!("4294967295", u32toa_string(-1i32 as u32, 10));
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn i32toa_test() {
        assert_eq!("0", i32toa_string(0, 10));
        assert_eq!("1", i32toa_string(1, 10));
        assert_eq!("2147483647", i32toa_string(2147483647, 10));
        assert_eq!("-2147483648", i32toa_string(2147483648u32 as i32, 10));
        assert_eq!("-1", i32toa_string(4294967295u32 as i32, 10));
        assert_eq!("-1", i32toa_string(-1, 10));
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn u64toa_test() {
        assert_eq!("0", u64toa_string(0, 10));
        assert_eq!("1", u64toa_string(1, 10));
        assert_eq!("9223372036854775807", u64toa_string(9223372036854775807, 10));
        assert_eq!("9223372036854775808", u64toa_string(9223372036854775808, 10));
        assert_eq!("18446744073709551615", u64toa_string(18446744073709551615, 10));
        assert_eq!("18446744073709551615", u64toa_string(-1i64 as u64, 10));
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn i64toa_test() {
        assert_eq!("0", i64toa_string(0, 10));
        assert_eq!("1", i64toa_string(1, 10));
        assert_eq!("9223372036854775807", i64toa_string(9223372036854775807, 10));
        // We would expect it to overflow, this single value.
        // assert_eq!("-9223372036854775808", i64toa_string(9223372036854775808u64 as i64, 10));
        assert_eq!("-1", i64toa_string(18446744073709551615u64 as i64, 10));
        assert_eq!("-1", i64toa_string(-1, 10));
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
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
