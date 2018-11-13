//! Lossy algorithms for string-to-float conversions.

use util::*;
use super::correct::parse_exponent;
use super::overflowing::*;

// FRACTION

/// Parse the integer portion of a positive, normal float string.
///
/// Use a float since for large numbers, this may even overflow a u64.
#[inline(always)]
unsafe extern "C" fn parse_integer(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    let mut integer: f64 = 0.0;
    let p = atoi_unchecked!(integer, first, last, base, f64).0;
    (integer, p)
}

/// Parse the fraction portion of a positive, normal float string.
///
/// Parse separately from the integer portion, since the small
/// values for each may be too small to change the integer components
/// representation **immediately**.
#[inline(always)]
unsafe extern "C" fn parse_fraction(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    // Ensure if there's a decimal, there are trailing values, so
    // invalid floats like "0." lead to an error.
    if distance(first, last) > 1 && *first == b'.' {
        let mut fraction: f64 = 0.;
        let first = first.add(1);
        let mut f = first;
        loop {
            // This would get better numerical precision using Horner's method,
            // but that would require.
            let mut value: u64 = 0;
            let l = min!(last, f.add(12));
            f = atoi_unchecked!(value, f, l, base, u64).0;
            let digits = distance(first, f) as i32;

            // Ignore leading 0s, just not we've passed them.
            if value != 0 {
                fraction += stable_powi_f64(value as f64, base, -digits);
            }

            // do/while condition
            if f == last || char_to_digit!(*f) >= base as u8 {
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
pub(super) unsafe extern "C" fn parse_float(first: *const u8, last: *const u8, base: u64)
    -> (f64, i32, *const u8)
{
    // Parse components
    let (integer, p) = parse_integer(first, last, base);
    let (fraction, p) = parse_fraction(p, last, base);
    let (exponent, p) = parse_exponent(p, last, base);

    (integer + fraction, exponent, p)
}

// ATOF/ATOD

/// Parse 32-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) unsafe extern "C" fn atof(first: *const u8, last: *const u8, base: u64)
    -> (f32, *const u8)
{
    let (value, p) = atod(first, last, base);
    (value as f32, p)
}

/// Parse 64-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) unsafe extern "C" fn atod(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    let (mut value, exponent, p) = parse_float(first, last, base);
    if exponent != 0 && value != 0.0 {
        value = stable_powi_f64(value, base, exponent);
    }
    (value, p)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn check_parse_integer(s: &str, base: u64, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = parse_integer(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn parse_integer_test() {
        unsafe {
            check_parse_integer("1.2345", 10, (1.0, 1));
            check_parse_integer("12.345", 10, (12.0, 2));
            check_parse_integer("12345.6789", 10, (12345.0, 5));
        }
    }

    unsafe fn check_parse_fraction(s: &str, base: u64, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = parse_fraction(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn parse_fraction_test() {
        unsafe {
            check_parse_fraction(".2345", 10, (0.2345, 5));
            check_parse_fraction(".345", 10, (0.345, 4));
            check_parse_fraction(".6789", 10, (0.6789, 5));
        }
    }

    unsafe fn check_parse_float(s: &str, base: u64, tup: (f64, i32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, e, p) = parse_float(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(e, tup.1);
        assert_eq!(distance(first, p), tup.2);
    }

    #[test]
    fn parse_float_test() {
        unsafe {
            check_parse_float("1.2345", 10, (1.2345, 0, 6));
            check_parse_float("12.345", 10, (12.345, 0, 6));
            check_parse_float("12345.6789", 10, (12345.6789, 0, 10));
            check_parse_float("1.2345e10", 10, (1.2345, 10, 9));
        }
    }

    unsafe fn check_atof(s: &str, base: u64, tup: (f32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atof(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn atof_test() {
        unsafe {
            check_atof("1.2345", 10, (1.2345, 6));
            check_atof("12.345", 10, (12.345, 6));
            check_atof("12345.6789", 10, (12345.6789, 10));
            check_atof("1.2345e10", 10, (1.2345e10, 9));
        }
    }

    unsafe fn check_atod(s: &str, base: u64, tup: (f64, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = atod(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn atod_test() {
        unsafe {
            check_atod("1.2345", 10, (1.2345, 6));
            check_atod("12.345", 10, (12.345, 6));
            check_atod("12345.6789", 10, (12345.6789, 10));
            check_atod("1.2345e10", 10, (1.2345e10, 9));
        }
    }
}
