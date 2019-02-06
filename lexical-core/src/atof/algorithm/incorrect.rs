//! Lossy algorithms for string-to-float conversions.

use atoi;
use util::*;
use super::exponent::parse_exponent;

// FRACTION

type Wrapped = WrappedFloat<f64>;

/// Parse the integer portion of a positive, normal float string.
///
/// Use a float since for large numbers, this may even overflow a u64.
#[inline]
fn parse_integer<'a>(radix: u32, bytes: &'a [u8])
    -> (f64, &'a [u8])
{
    // Trim leading zeros, since we haven't parsed anything yet.
    let bytes = ltrim_char_slice(bytes, b'0').0;

    let mut value = Wrapped::ZERO;
    let (len, _) = atoi::unchecked_positive(&mut value, as_cast(radix), bytes);

    // We know this is always true, since `len` is the length processed
    // from atoi, which must be <= bytes.len().
    (value.into_inner(), &index!(bytes[len..]))
}

/// Parse the fraction portion of a positive, normal float string.
///
/// Parse separately from the integer portion, since the small
/// values for each may be too small to change the integer components
/// representation **immediately**.
#[inline]
fn parse_fraction<'a>(radix: u32, bytes: &'a [u8])
    -> (f64, &'a [u8])
{
    // Ensure if there's a decimal, there are trailing values, so
    // invalid floats like "0." lead to an error.
    if Some(&b'.') == bytes.get(0) {
        // We know this must be true, since we just got the first value.
        let mut bytes = &index!(bytes[1..]);
        let first = bytes.as_ptr();
        let mut fraction: f64 = 0.;
        loop {
            // Trim leading zeros, since that never gets called with the raw parser.
            // Since if it's after the decimal place and this increments state.curr,
            // but not first, this is safe.
            bytes = ltrim_char_slice(bytes, b'0').0;

            // This would get better numerical precision using Horner's method,
            // but that would require.
            let mut value: u64 = 0;
            // We know this is safe, since we grab 12 digits, or the length
            // of the buffer, whichever is smaller.
            let buf = &index!(bytes[..bytes.len().min(12)]);
            let (processed, _) = atoi::unchecked_positive(&mut value, radix.as_u64(), buf);
            // We know this is safe, since atoi returns a value <= buf.len().
            bytes = &index!(bytes[processed..]);
            let digits = distance(first, bytes.as_ptr()).try_i32_or_max();

            // Ignore leading 0s, just not we've passed them.
            if value != 0 {
                fraction += f64::iterative_pow(value as f64, radix, -digits);
            }

            // do/while condition
            if char_to_digit(*bytes.get(0).unwrap_or(&b'\0')).as_u32() >= radix {
                break;
            }
        }
        // Store frac component over the parsed digits.
        (fraction, bytes)
    } else {
        (0.0, bytes)
    }
}

/// Parse the mantissa from a string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
fn parse_mantissa<'a>(radix: u32, bytes: &'a [u8])
    -> (f64, &'a [u8])
{
    let (integer, bytes) = parse_integer(radix, bytes);
    let (fraction, bytes) = parse_fraction(radix, bytes);

    (integer + fraction, bytes)
}

// PARSE

/// Parse the mantissa and exponent from a string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
fn parse_float<'a>(radix: u32, bytes: &'a [u8])
    -> (f64, i32, &'a [u8])
{
    let (mantissa, bytes) = parse_mantissa(radix, bytes);
    let (exponent, bytes) = parse_exponent(radix, bytes);

    (mantissa, exponent, bytes)
}

// ATOF/ATOD

/// Parse 32-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) fn atof(radix: u32, bytes: &[u8], sign: Sign)
    -> (f32, usize)
{
    let (value, len) = atod(radix, bytes, sign);
    (value as f32, len)
}

/// Parse 64-bit float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(crate) fn atod(radix: u32, bytes: &[u8], _: Sign)
    -> (f64, usize)
{
    let (mut value, exponent, slc) = parse_float(radix, bytes);
    if exponent != 0 && value != 0.0 {
        value = value.iterative_pow(radix, exponent);
    }
    (value, bytes.len() - slc.len())
}

#[inline]
pub(crate) fn atof_lossy(radix: u32, bytes: &[u8], sign: Sign)
    -> (f32, usize)
{
    atof(radix, bytes, sign)
}

#[inline]
pub(crate) fn atod_lossy(radix: u32, bytes: &[u8], sign: Sign)
    -> (f64, usize)
{
    atod(radix, bytes, sign)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    fn check_parse_integer(radix: u32, s: &str, tup: (f64, usize)) {
        let (value, slc) = parse_integer(radix, s.as_bytes());
        assert_eq!(value, tup.0);
        assert_eq!(s.len() - slc.len(), tup.1);
    }

    #[test]
    fn parse_integer_test() {
        check_parse_integer(10, "1.2345", (1.0, 1));
        check_parse_integer(10, "12.345", (12.0, 2));
        check_parse_integer(10, "12345.6789", (12345.0, 5));
    }

    fn check_parse_fraction(radix: u32, s: &str, tup: (f64, usize)) {
        let (value, slc) = parse_fraction(radix, s.as_bytes());
        assert_eq!(value, tup.0);
        assert_eq!(s.len() - slc.len(), tup.1);
    }

    #[test]
    fn parse_fraction_test() {
        check_parse_fraction(10, ".2345", (0.2345, 5));
        check_parse_fraction(10, ".345", (0.345, 4));
        check_parse_fraction(10, ".6789", (0.6789, 5));
    }

    fn check_parse_mantissa(radix: u32, s: &str, tup: (f64, usize)) {
        let (value, slc) = parse_mantissa(radix, s.as_bytes());
        assert_eq!(value, tup.0);
        assert_eq!(s.len() - slc.len(), tup.1);
    }

    #[test]
    fn parse_mantissa_test() {
        check_parse_mantissa(10, "1.2345", (1.2345, 6));
        check_parse_mantissa(10, "12.345", (12.345, 6));
        check_parse_mantissa(10, "12345.6789", (12345.6789, 10));
    }

    fn check_parse_float(radix: u32, s: &str, tup: (f64, i32, usize)) {
        let (value, exponent, slc) = parse_float(radix, s.as_bytes());
        assert_eq!(value, tup.0);
        assert_eq!(exponent, tup.1);
        assert_eq!(s.len() - slc.len(), tup.2);
    }

    #[test]
    fn parse_float_test() {
        check_parse_float(10, "1.2345", (1.2345, 0, 6));
        check_parse_float(10, "12.345", (12.345, 0, 6));
        check_parse_float(10, "12345.6789", (12345.6789, 0, 10));
        check_parse_float(10, "1.2345e10", (1.2345, 10, 9));
    }

    fn check_atof(radix: u32, s: &str, tup: (f32, usize)) {
        let (value, len) = atof(radix, s.as_bytes(), Sign::Positive);
        assert_eq!(value, tup.0);
        assert_eq!(len, tup.1);
    }

    #[test]
    fn atof_test() {
        check_atof(10, "1.2345", (1.2345, 6));
        check_atof(10, "12.345", (12.345, 6));
        check_atof(10, "12345.6789", (12345.6789, 10));
        check_atof(10, "1.2345e10", (1.2345e10, 9));
    }

    fn check_atod(radix: u32, s: &str, tup: (f64, usize)) {
        let (value, len) = atod(radix, s.as_bytes(), Sign::Positive);
        assert_eq!(value, tup.0);
        assert_eq!(len, tup.1);
    }

    #[test]
    fn atod_test() {
        check_atod(10, "1.2345", (1.2345, 6));
        check_atod(10, "12.345", (12.345, 6));
        check_atod(10, "12345.6789", (12345.6789, 10));
        check_atod(10, "1.2345e10", (1.2345e10, 9));
    }

    // Lossy
    // Just a synonym for the regular overloads, since we're not using the
    // correct feature. Use the same tests.

    fn check_atof_lossy(radix: u32, s: &str, tup: (f32, usize)) {
        let (value, len) = atof_lossy(radix, s.as_bytes(), Sign::Positive);
        assert_f32_eq!(value, tup.0);
        assert_eq!(len, tup.1);
    }

    #[test]
    fn atof_lossy_test() {
        check_atof_lossy(10, "1.2345", (1.2345, 6));
        check_atof_lossy(10, "12.345", (12.345, 6));
        check_atof_lossy(10, "12345.6789", (12345.6789, 10));
        check_atof_lossy(10, "1.2345e10", (1.2345e10, 9));
    }

    fn check_atod_lossy(radix: u32, s: &str, tup: (f64, usize)) {
        let (value, len) = atod_lossy(radix, s.as_bytes(), Sign::Positive);
        assert_f64_eq!(value, tup.0);
        assert_eq!(len, tup.1);
    }

    #[test]
    fn atod_lossy_test() {
        check_atod_lossy(10, "1.2345", (1.2345, 6));
        check_atod_lossy(10, "12.345", (12.345, 6));
        check_atod_lossy(10, "12345.6789", (12345.6789, 10));
        check_atod_lossy(10, "1.2345e10", (1.2345e10, 9));
    }
}
