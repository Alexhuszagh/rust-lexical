//! Correct algorithms for string-to-float conversions.

use util::*;
use ftoa::exponent_notation_char;

// TODO(ahuszagh)
//  Base implementation off of:
// Use fast path or bigint as a fallback.
//  https://github.com/gcc-mirror/gcc/blob/master/libgo/go/strconv/atof.go
//  https://github.com/python/cpython/blob/e42b705188271da108de42b55d9344642170aa2b/Python/dtoa.c

// SHARED

/// Parse the exponential portion from a float-string, if we have an `(e|^)[+-]?\d+`.
///
/// On overflow, just return a comically large exponent, since we don't
/// care. It will lead to infinity regardless, and doesn't affect whether
/// the type is representable.
///
/// Returns the exponent and a pointer to the current buffer position.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(super) unsafe extern "C" fn parse_exponent(first: *const u8, last: *const u8, base: u64)
    -> (i32, *const u8)
{
    let mut p = first;
    let dist = distance(first, last);
    if dist > 1 && (*p).to_ascii_lowercase() == exponent_notation_char(base) {
        p = p.add(1);
        // Use atoi_sign so we can handle overflow differently for +/- numbers.
        // We care whether the value is positive.
        // Use is32::max_value() since it's valid in 2s complement for
        // positive or negative numbers, and will trigger a short-circuit.
        let (exponent, p, overflow, sign) = atoi_sign!(p, last, base, i32);
        let exponent = if overflow { i32::max_value() } else { exponent };
        let exponent = if sign == -1 { -exponent } else { exponent };
        (exponent, p)
    } else {
        (0, p)
    }
}

// FAST

#[cfg(any(test, feature = "correct"))]
mod fast {
// Fast path for the parse algorithm.
// In this case, the mantissa can be represented by an integer,
// which allows any value to be exactly reconstructed.

use util::*;
use super::super::double;
use super::super::float;

/// Parse the mantissa from a string.
///
/// Returns the mantissa, number of digits since the dot was seen,
/// a pointer to the current buffer position, and if overflow occurred.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(super) unsafe extern "C" fn parse_mantissa(mut first: *const u8, last: *const u8, base: u64)
    -> (u64, i32, *const u8, bool)
{
    // Trim the leading 0s.
    // Need to force this here, since if not, conversion of usize dot to
    // i32 may truncate when mantissa does not, which would lead to faulty
    // results. If we trim the 0s here, we guarantee any time `dot as i32`
    // leads to a truncation, mantissa will overflow.
    while first < last && *first == b'0' {
        first = first.add(1);
    }

    // Parse the integral value.
    let (mut mantissa, p, overflow) = atoi_value!(first, last, base, u64);
    if distance(p, last) > 1 && *p == b'.' {
        // Has a decimal, calculate the rest of it.
        let p = p.add(1);
        let tup = atoi_pointer!(mantissa, p, last, base, u64);
        let dot = distance(p, tup.0) as i32;
        (mantissa, dot, tup.0, overflow | tup.1)
    } else {
        // No decimal, just return
        (mantissa, 0, p, overflow)
    }
}

/// Calculate the exact exponent without overflow.
///
/// Remove the number of digits that contributed to the mantissa past
/// the dot.
pub(super) extern "C" fn normalize_exponent(exponent: i32, dot: i32)
    -> i32
{
    match exponent {
         0x7FFFFFFF => i32::max_value(),
        -0x80000000 => i32::min_value(),
        _           => exponent - dot,
    }
}

/// Normalize the mantissa to check if it can use the fast-path.
///
/// Move digits from the mantissa to the exponent when possible.
pub(super) extern "C" fn normalize_mantissa(mut mantissa: u64, base: u64, mut exponent: i32)
    -> (u64, i32)
{
    let base2 = base * base;
    let base4 = base2 * base2;

    // Use power-reduction, we're likely never going to enter most of these
    // loops, but it minimizes the number of expensive operations we need
    // to do.
    while mantissa >= base4 && mantissa % base4 == 0 {
        mantissa /= base4;
        exponent += 4;
    }
    while mantissa >= base2 && mantissa % base2 == 0 {
        mantissa /= base2;
        exponent += 2;
    }
    if mantissa % base == 0 {
        mantissa /= base;
        exponent += 1;
    }
    (mantissa, exponent)
}

/// Parse the mantissa and exponent from a string.
///
/// Returns the mantissa, the exponent, number of digits since the dot
/// was seen, a pointer to the current buffer position, and if overflow
/// occurred.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(super) unsafe extern "C" fn parse_float(first: *const u8, last: *const u8, base: u64)
    -> (u64, i32, *const u8, bool)
{
    let (mantissa, dot, p, overflow) = parse_mantissa(first, last, base);
    let (exponent, p) = super::parse_exponent(p, last, base);
    let exponent = normalize_exponent(exponent, dot);
    let (mantissa, exponent) = normalize_mantissa(mantissa, base, exponent);
    (mantissa, exponent, p, overflow)
}

/// Macro to convert value to exact value, allowing code reuse.
/// Custom code can be executed inside the code-block to avoid code
macro_rules! to_exact {
    ($mantissa:ident, $base:ident, $exponent:ident, $size:ident, $min:ident, $max:ident, $f:ty, $mod:ident) => ({
        // logic error, disable in release builds
        debug_assert!($base >= 2 && $base <= 36, "Numerical base must be from 2-36");

        if $mantissa >> $size != 0 {
            // Would require truncation of the mantissa, use slow path.
            (0.0, false)
        } else {
            let float = $mantissa as $f;
            if $exponent == 0 {
                // 0 exponent, same as value, exact representation.
                (float,  true)
            } else if $exponent >= $min && $exponent >= $max {
                // Value can be exactly represented, return the value.
                let float = match $base {
                    2  => $mod::pow2_to_exact(float, 1, $exponent),
                    4  => $mod::pow2_to_exact(float, 2, $exponent),
                    8  => $mod::pow2_to_exact(float, 3, $exponent),
                    16 => $mod::pow2_to_exact(float, 4, $exponent),
                    32 => $mod::pow2_to_exact(float, 5, $exponent),
                    _  => $mod::basen_to_exact(float, $base, $exponent),
                };
                (float, true)
            } else {
                // Cannot be exactly represented, return false.
                (0.0, false)
            }
        }
    });
}

// FLOAT

/// Convert mantissa and exponent to exact f32.
///
/// Return the exact float and if the exact conversion was successful.
pub(super) unsafe fn to_exact_float(mantissa: u64, base: u64, exponent: i32) -> (f32, bool) {
    let (min_exp, max_exp) = f32_exact_exponent_limit!(base);
    to_exact!(mantissa, base, exponent, F32_SIGNIFICAND_SIZE, min_exp, max_exp, f32, float)
}

// DOUBLE

/// Convert mantissa and exponent to exact f64.
///
/// Return the exact float and if the exact conversion was successful.
pub(super) unsafe fn to_exact_double(mantissa: u64, base: u64, exponent: i32) -> (f64, bool) {
    let (min_exp, max_exp) = f64_exact_exponent_limit!(base);
    to_exact!(mantissa, base, exponent, F64_SIGNIFICAND_SIZE, min_exp, max_exp, f64, double)
}

}   // fast

// SLOW

#[cfg(any(test, feature = "correct"))]
mod slow {
// Slow path for the parse algorithm.
// In this case, the mantissa cannot be represented by an integer,
// which requires a big integer to do exact reconstruction.


// TODO(ahuszagh)
//  Need to implement the slow path... Here...

}   // slow

// ATOF/ATOD

/// Parse 32-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
#[allow(unused)]
#[cfg(any(test, feature = "correct"))]
pub(crate) unsafe extern "C" fn atof(first: *const u8, last: *const u8, base: u64)
    -> (f32, *const u8)
{
    let (mantissa, exponent, p, overflow) = fast::parse_float(first, last, base);

    // Try fast paths.
    if !overflow {
        if mantissa == 0 {
            return (0.0, p);
        } else {
            // Try a fast path
            let (float, overflow) = fast::to_exact_float(mantissa, base, exponent);
            if overflow {
                // Try another fast path
                // TODO(ahuszagh) Implement...
                unreachable!()
            } else {
                return (float, p);
            }
        }
    }

    // Slow path
    // TODO(ahuszagh) Implement...
    unreachable!()
}

/// Parse 64-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
#[allow(unused)]
#[cfg(any(test, feature = "correct"))]
pub(crate) unsafe extern "C" fn atod(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    let (mantissa, exponent, p, overflow) = fast::parse_float(first, last, base);

    // Try fast paths.
    if !overflow {
        if mantissa == 0 {
            return (0.0, p);
        } else {
            // Try a fast path
            let (double, overflow) = fast::to_exact_double(mantissa, base, exponent);
            if overflow {
                // Try another fast path
                // TODO(ahuszagh) Implement...
                unreachable!()
            } else {
                return (double, p);
            }
        }
    }

    // Slow path
    // TODO(ahuszagh) Implement...
    unreachable!()
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn check_parse_exponent(s: &str, base: u64, tup: (i32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = parse_exponent(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn parse_exponent_test() {
        unsafe {
            // empty
            check_parse_exponent("", 10, (0, 0));

            // lowercase
            check_parse_exponent("e20", 10, (20, 3));
            check_parse_exponent("e+20", 10, (20, 4));
            check_parse_exponent("e-20", 10, (-20, 4));

            // uppercase
            check_parse_exponent("E20", 10, (20, 3));
            check_parse_exponent("E+20", 10, (20, 4));
            check_parse_exponent("E-20", 10, (-20, 4));

            // >= base15
            check_parse_exponent("^20", 15, (30, 3));
            check_parse_exponent("^+20", 15, (30, 4));
            check_parse_exponent("^-20", 15, (-30, 4));

            // overflow
            check_parse_exponent("e10000000000", 10, (i32::max_value(), 12));
            check_parse_exponent("e+10000000000", 10, (i32::max_value(), 13));
            check_parse_exponent("e-10000000000", 10, (-i32::max_value(), 13));

            // trailing
            check_parse_exponent("e20 ", 10, (20, 3));
            check_parse_exponent("e+20 ", 10, (20, 4));
        }
    }

    // TODO(ahuszagh) atof, atod
    // Check both known fast and slow paths.
}

#[cfg(test)]
mod fast_tests {
    use super::*;

    unsafe fn check_parse_mantissa(s: &str, base: u64, tup: (u64, i32, usize, bool))
    {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, d, p, o) = fast::parse_mantissa(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(d, tup.1);
        assert_eq!(distance(first, p), tup.2);
        assert_eq!(o, tup.3);
    }

    #[test]
    fn parse_mantissa_test() {
        unsafe {
            check_parse_mantissa("1.2345", 10, (12345, 4, 6, false));
            check_parse_mantissa("12.345", 10, (12345, 3, 6, false));
            check_parse_mantissa("12345.6789", 10, (123456789, 4, 10, false));
            check_parse_mantissa("1.2345e10", 10, (12345, 4, 6, false));
            check_parse_mantissa("100000000000000000000", 10, (7766279631452241920, 0, 21, true));
        }
    }

    #[test]
    fn normalize_exponent_test() {
        assert_eq!(fast::normalize_exponent(10, 5), 5);
        assert_eq!(fast::normalize_exponent(0, 5), -5);
        assert_eq!(fast::normalize_exponent(i32::max_value(), 5), i32::max_value());
        assert_eq!(fast::normalize_exponent(i32::min_value(), 5), i32::min_value());
    }

    #[test]
    fn normalize_mantissa_test() {
        assert_eq!(fast::normalize_mantissa(100, 10, 0), (1, 2));
        assert_eq!(fast::normalize_mantissa(101, 10, 0), (101, 0));
        assert_eq!(fast::normalize_mantissa(110, 10, 0), (11, 1));
    }

    unsafe fn check_parse_float(s: &str, base: u64, tup: (u64, i32, usize, bool))
    {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, e, p, o) = fast::parse_float(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(e, tup.1);
        assert_eq!(distance(first, p), tup.2);
        assert_eq!(o, tup.3);
    }

    #[test]
    fn parse_float_test() {
        unsafe {
            check_parse_float("1.2345", 10, (12345, -4, 6, false));
            check_parse_float("12.345", 10, (12345, -3, 6, false));
            check_parse_float("12345.6789", 10, (123456789, -4, 10, false));
            check_parse_float("1.2345e10", 10, (12345, 6, 9, false));
            check_parse_float("100000000000000000000", 10, (776627963145224192, 1, 21, true));
        }
    }

    // TODO(ahuszagh)
    //  to_exact_float
    //  to_exact_double
}

#[cfg(test)]
mod slow_tests {
//    use super::*;
// TODO(ahuszagh) Implement...
}
