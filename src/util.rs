//! Helper utilities for low-level features.
//!
//! Utilities for working with pointers and compiler intrinsics that
//! may not be available  in rust, or in a `no_std` context.

#[cfg(feature = "std")]
use std::{f32, f64};

// GLOBALS

/// Not a Number literal
///
/// To change the expected representation of NaN as a string,
/// change this value during before using lexical.
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut NAN_STRING: &str = "NaN";

/// Infinity literal
///
/// To change the expected representation of Infinity as a string,
/// change this value during before using lexical.
pub static mut INFINITY_STRING: &str = "inf";

// CONSTANTS

/// Not a Number (NaN).
#[cfg(feature = "std")]
pub(crate) const F32_NAN: f32 = f32::NAN;

/// Not a Number (NaN).
#[cfg(not(feature = "std"))]
pub(crate) const F32_NAN: f32 = 0.0_f32 / 0.0_f32;

/// Infinity (∞).
#[cfg(feature = "std")]
pub(crate) const F32_INFINITY: f32 = f32::INFINITY;

/// Infinity (∞).
#[cfg(not(feature = "std"))]
pub(crate) const F32_INFINITY: f32 = 1.0_f32 / 0.0_f32;

/// Not a Number (NaN).
#[cfg(feature = "std")]
pub(crate) const F64_NAN: f64 = f64::NAN;

/// Not a Number (NaN).
#[cfg(not(feature = "std"))]
pub(crate) const F64_NAN: f64 = 0.0_f64 / 0.0_f64;

/// Infinity (∞).
#[cfg(feature = "std")]
pub(crate) const F64_INFINITY: f64 = f64::INFINITY;

/// Infinity (∞).
#[cfg(not(feature = "std"))]
pub(crate) const F64_INFINITY: f64 = 1.0_f64 / 0.0_f64;

// INSTRINSICS

/// `f64.floor()` feature for `no_std`
#[cfg(not(feature = "std"))]
#[inline(always)]
pub(crate) fn floor(f: f64) -> f64 {
    unsafe { core::intrinsics::floorf64(f) }
}

/// `f64.ln()` feature for `no_std`
#[cfg(not(feature = "std"))]
#[inline(always)]
pub(crate) fn ln(f: f64) -> f64 {
    unsafe { core::intrinsics::logf64(f) }
}

/// `f64.powi(i32)` feature for `no_std`
#[cfg(not(feature = "std"))]
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn powi(f: f64, i: i32) -> f64 {
    unsafe { core::intrinsics::powif64(f, i) }
}

/// `f64.floor()` feature for `std`
#[cfg(feature = "std")]
#[inline(always)]
pub(crate) fn floor(f: f64) -> f64 {
    f.floor()
}

/// `f64.ln()` feature for `std`
#[cfg(feature = "std")]
#[inline(always)]
pub(crate) fn ln(f: f64) -> f64 {
    f.ln()
}

/// `f64.powi(i32)` feature for `std`
#[cfg(feature = "std")]
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn powi(f: f64, i: i32) -> f64 {
    f.powi(i)
}

// MACRO

/// Fast macro absolute value calculator.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate lexical;
/// # pub main() {
/// }
/// ```
macro_rules! absv {
    ($n:expr) => ({
        let n = $n;
        if n < 0 { -n } else { n }
    })
}

/// Fast macro maximum value calculator.
///
///
macro_rules! maxv {
    ($a:expr, $b:expr) => ({
        let a = $a;
        let b = $b;
        if a > b { a } else { b }
    })
}

/// Fast macro minimum value calculator.
///
///
macro_rules! minv {
    ($a:expr, $b:expr) => ({
        let a = $a;
        let b = $b;
        if a < b { a } else { b }
    })
}

// ALGORITHMS

/// Reverse a range of pointers.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn reverse(first: *mut u8, last: *mut u8) {
    let mut f = first;
    let mut l = last;
    let mut x: u8;
    let mut li = l.sub(1);

    while f != l && f != li {
        l = li;
        x = *f;
        *f = *l;
        *l = x;
        li = l.sub(1);
        f = f.add(1);
    }
}

/// Calculate the difference between two pointers.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn distance(first: *const u8, last: *const u8)
    -> usize
{
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

extern {
    /// Need memcmp for efficient range comparisons.
    fn memcmp(l: *const u8, r: *const u8, n: usize) -> i32;
}

/// Check if two ranges are equal to each other.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn equal_to(l: *const u8, r: *const u8, n: usize)
    -> bool
{
    memcmp(l, r, n) == 0
}

/// Check if left range starts with right range.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn starts_with(l: *const u8, ln: usize, r: *const u8, rn: usize)
    -> bool
{
    ln >= rn && equal_to(l, r, rn)
}

/// Check if left range ends with right range.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn ends_with(l: *const u8, ln: usize, r: *const u8, rn: usize)
    -> bool
{
    ln >= rn && equal_to(l.add(ln - rn), r, rn)
}

// LOW LEVEL WRAPPERS

/// Generate the low-level bytes API.
///
/// Wraps unsafe functions to generate the low-level, unchecked, bytes parsers.
#[doc(hidden)]
macro_rules! bytes_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level bytes to number parser.
        #[inline]
        pub fn $func(bytes: &[u8], base: u8)
            -> $t
        {
            unsafe {
                let first = bytes.as_ptr();
                let last = first.add(bytes.len());
                let (value, _) = $callback(first, last, base);
                value
            }
        }
    )
}

/// Error-checking version of `bytes_impl`.
///
/// Wraps unsafe functions to generate the low-level, checked, bytes parsers.
#[doc(hidden)]
macro_rules! try_bytes_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level bytes to number parser.
        /// On error, returns position of invalid char.
        #[inline]
        pub fn $func(bytes: &[u8], base: u8)
            -> Result<$t, usize>
        {
            unsafe {
                let first = bytes.as_ptr();
                let last = first.add(bytes.len());
                let (value, p) = $callback(first, last, base);
                match p == last {
                    true => Ok(value),
                    _    => Err(match p == ptr::null() {
                        true  => 0,     // Empty string.
                        false => distance(first, p),
                    }),
                }
            }
        }
    )
}

/// Generate the low-level string API using wrappers around the unsafe function.
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! string_impl {
    ($func:ident, $t:ty, $callback:ident, $capacity:expr) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub fn $func(value: $t, base: u8)
            -> String
        {
            let mut buf: Vec<u8> = Vec::with_capacity($capacity);
            unsafe {
                let first: *mut u8 = buf.as_mut_ptr();
                let last = first.add(buf.capacity());
                let end = $callback(value, first, last, base);
                let size = distance(first, end);
                buf.set_len(size);

                String::from_utf8_unchecked(buf)
            }
        }
    )
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::atof::*;
    use super::super::ftoa::*;

    #[test]
    fn reverse_test() {
        unsafe {
            let mut x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let y: [u8; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
            let first: *mut u8 = x.as_mut_ptr();
            let last = first.add(x.len());
            reverse(first, last);
            assert_eq!(x, y);
        }
    }

    #[test]
    fn distance_test() {
        unsafe {
            let x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let first: *const u8 = x.as_ptr();
            let last = first.add(x.len());
            assert_eq!(distance(first, last), 10);
        }
    }

    #[test]
    fn equal_to_test() {
        unsafe {
            let x = "Hello";
            let y = "Hello";
            let z = "hello";
            assert!(equal_to(x.as_ptr(), y.as_ptr(), x.len()));
            assert!(!equal_to(x.as_ptr(), z.as_ptr(), x.len()));
            assert!(!equal_to(y.as_ptr(), z.as_ptr(), x.len()));
        }
    }

    #[test]
    fn starts_with_test() {
        unsafe {
            let x = "Hello";
            let y = "H";
            let z = "h";

            // forward
            assert!(starts_with(x.as_ptr(), x.len(), y.as_ptr(), y.len()));
            assert!(!starts_with(x.as_ptr(), x.len(), z.as_ptr(), z.len()));
            assert!(!starts_with(y.as_ptr(), y.len(), z.as_ptr(), z.len()));

            // back
            assert!(!starts_with(y.as_ptr(), y.len(), x.as_ptr(), x.len()));
            assert!(!starts_with(z.as_ptr(), z.len(), x.as_ptr(), x.len()));
        }
    }

    #[test]
    fn ends_with_test() {
        unsafe {
            let w = "Hello";
            let x = "lO";
            let y = "lo";
            let z = "o";

            // forward
            assert!(!ends_with(w.as_ptr(), w.len(), x.as_ptr(), x.len()));
            assert!(ends_with(w.as_ptr(), w.len(), y.as_ptr(), y.len()));
            assert!(ends_with(w.as_ptr(), w.len(), z.as_ptr(), z.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), y.as_ptr(), y.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), z.as_ptr(), z.len()));
            assert!(ends_with(y.as_ptr(), y.len(), z.as_ptr(), z.len()));

            // back
            assert!(!ends_with(z.as_ptr(), z.len(), y.as_ptr(), y.len()));
            assert!(!ends_with(z.as_ptr(), z.len(), x.as_ptr(), x.len()));
            assert!(!ends_with(z.as_ptr(), z.len(), w.as_ptr(), w.len()));
            assert!(!ends_with(y.as_ptr(), y.len(), x.as_ptr(), x.len()));
            assert!(!ends_with(y.as_ptr(), y.len(), w.as_ptr(), w.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), w.as_ptr(), w.len()));
        }
    }

    // Only enable when no other threads touch NAN_STRING or INFINITY_STRING.
    #[cfg(feature = "std")]
    #[test]
    #[ignore]
    fn special_string_test() {
        // Test serializing and deserializing special strings.
        assert!(atof32_bytes(b"NaN", 10).is_nan());
        assert!(atof32_bytes(b"inf", 10).is_infinite());
        assert!(!atof32_bytes(b"nan", 10).is_nan());
        assert!(!atof32_bytes(b"Infinity", 10).is_infinite());
        assert_eq!(&f64toa_string(F64_NAN, 10), "NaN");
        assert_eq!(&f64toa_string(F64_INFINITY, 10), "inf");

        unsafe {
            NAN_STRING = "nan";
            INFINITY_STRING = "Infinity";
        }

        assert!(!atof32_bytes(b"NaN", 10).is_nan());
        assert!(!atof32_bytes(b"inf", 10).is_infinite());
        assert!(atof32_bytes(b"nan", 10).is_nan());
        assert!(atof32_bytes(b"Infinity", 10).is_infinite());
        assert_eq!(&f64toa_string(F64_NAN, 10), "nan");
        assert_eq!(&f64toa_string(F64_INFINITY, 10), "Infinity");

        unsafe {
            NAN_STRING = "NaN";
            INFINITY_STRING = "inf";
        }
    }
}
