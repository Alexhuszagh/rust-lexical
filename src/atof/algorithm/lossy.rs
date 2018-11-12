//! Lossy algorithms for string-to-float conversions.

use util::*;
use super::correct::parse_exponent;
use super::overflowing::*;

// POWI

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

/// Simplify base to powi to avoid bugs.
macro_rules! base_to_powi {
    ($base:expr) => (unsafe { *POWI_EXPONENTS.get_unchecked($base as usize - 2) })
}

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
        let exp = base_to_powi!($base);
        // Choose a multipler of 5 for this since the exp is chosen
        // so at max 2.1 iterations occur to the max exponent.
        // 5 means any input value times the exponent must be insignificant.
        if $exponent > 5*exp {
            // Value is impossibly large, must be infinity.
            F64_INFINITY
        } else if $exponent < -5*exp {
            // Value is impossibly small, must be 0.
            0.0
        } else if $exponent < 0 {
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
fn stable_powi_multiplier(mut value: f64, base: u64, exponent: i32) -> f64 {
    stable_powi!(value, *, base, exponent)
}

/// `powi` implementation that is more stable at extremely low powers.
///
/// Equivalent to `value / powi(base, exponent)`
fn stable_powi_divisor(mut value: f64, base: u64, exponent: i32) -> f64 {
    stable_powi!(value, /, base, exponent)
}

// FRACTION

/// Parse the integer portion of a positive, normal float string.
///
/// Use a float since for large numbers, this may even overflow a u64.
#[inline(always)]
unsafe extern "C" fn parse_integer(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    let mut integer: f64 = 0.0;
    let p = atoi_pointer!(integer, first, last, base, f64).0;
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
            f = atoi_pointer!(value, f, l, base, u64).0;
            let digits = distance(first, f) as i32;

            // Ignore leading 0s, just not we've passed them.
            if value != 0 {
                fraction += stable_powi_divisor(value as f64, base, digits);
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
        value = stable_powi_multiplier(value, base, exponent);
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
