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

/// Parse the mantissa from a string.
///
/// Returns the mantissa, index where the decimal point was seen,
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
    let dot = distance(first, p) as i32;
    if distance(p, last) > 1 && *p == b'.' {
        // Has a decimal, calculate the rest of it.
        let p = p.add(1);
        let tup = atoi_pointer!(mantissa, p, last, base, u64);
        (mantissa, dot, tup.0, overflow | tup.1)
    } else {
        // No decimal, just return
        (mantissa, dot, p, overflow)
    }
}

/// Parse the mantissa and exponent from a string.
///
/// Returns the mantissa, the exponent, the index where the dot was seen,
/// a pointer to the current buffer position, and if overflow occurred.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(super) unsafe extern "C" fn parse_float(first: *const u8, last: *const u8, base: u64)
    -> (u64, i32, i32, *const u8, bool)
{
    let (mantissa, dot, p, overflow) = parse_mantissa(first, last, base);
    let (exponent, p) = super::parse_exponent(p, last, base);
    (mantissa, exponent, dot, p, overflow)
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
pub(crate) unsafe extern "C" fn atof(first: *const u8, last: *const u8, base: u64)
    -> (f32, *const u8)
{
    // TODO(ahuszagh) Implement
    unreachable!()
}

/// Parse 64-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
#[allow(unused)]
pub(crate) unsafe extern "C" fn atod(first: *const u8, last: *const u8, base: u64)
    -> (f64, *const u8)
{
    // TODO(ahuszagh) Implement
    unreachable!()
}

// TODO(ahuszagh) Add atof

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
            check_parse_mantissa("1.2345", 10, (12345, 1, 6, false));
            check_parse_mantissa("12.345", 10, (12345, 2, 6, false));
            check_parse_mantissa("12345.6789", 10, (123456789, 5, 10, false));
            check_parse_mantissa("1.2345e10", 10, (12345, 1, 6, false));
            check_parse_mantissa("100000000000000000000", 10, (7766279631452241920, 21, 21, true));
        }
    }

    unsafe fn check_parse_float(s: &str, base: u64, tup: (u64, i32, i32, usize, bool))
    {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, e, d, p, o) = fast::parse_float(first, last, base);
        assert_eq!(v, tup.0);
        assert_eq!(e, tup.1);
        assert_eq!(d, tup.2);
        assert_eq!(distance(first, p), tup.3);
        assert_eq!(o, tup.4);
    }

    #[test]
    fn parse_float_test() {
        unsafe {
            check_parse_float("1.2345", 10, (12345, 0, 1, 6, false));
            check_parse_float("12.345", 10, (12345, 0, 2, 6, false));
            check_parse_float("12345.6789", 10, (123456789, 0, 5, 10, false));
            check_parse_float("1.2345e10", 10, (12345, 10, 1, 9, false));
            check_parse_float("100000000000000000000", 10, (7766279631452241920, 0, 21, 21, true));
        }
    }
}

#[cfg(test)]
mod slow_tests {
//    use super::*;
// TODO(ahuszagh) Implement...
}
