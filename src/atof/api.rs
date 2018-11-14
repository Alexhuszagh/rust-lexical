//! Low-level API generator.
//!
//! Uses either the lossy or the correct algorithm.

use lib::ptr;
use util::*;

// Select the back-end
cfg_if! {
if #[cfg(feature = "correct")] {
    use super::algorithm::correct as algorithm;
} else {
    use super::algorithm::lossy as algorithm;
}}  // cfg_if

// TRAITS

/// Trait to define parsing of a string to float.
trait StringToFloat: Float {
    /// Load float from basen string.
    unsafe extern "C" fn basen(base: u32, first: *const u8, last: *const u8) -> (Self, *const u8);
}

impl StringToFloat for f32 {
    #[inline(always)]
    unsafe extern "C" fn basen(base: u32, first: *const u8, last: *const u8) -> (f32, *const u8) {
        algorithm::atof(base, first, last)
    }
}

impl StringToFloat for f64 {
    #[inline(always)]
    unsafe extern "C" fn basen(base: u32, first: *const u8, last: *const u8) -> (f64, *const u8) {
        algorithm::atod(base, first, last)
    }
}

// SPECIAL
// Utilities to filter special values.

#[inline(always)]
unsafe extern "C" fn is_nan(first: *const u8, length: usize)
    -> bool
{
    starts_with(first, length, NAN_STRING.as_ptr(), NAN_STRING.len())
}

#[inline(always)]
unsafe extern "C" fn is_infinity(first: *const u8, length: usize)
    -> bool
{
    starts_with(first, length, INFINITY_STRING.as_ptr(), INFINITY_STRING.len())
}

#[inline(always)]
unsafe extern "C" fn is_zero(first: *const u8, length: usize)
    -> bool
{
    length == 3 && equal_to(first, "0.0".as_ptr(), 3)
}

// ATOF

/// Convert string to float and handle special floating-point strings.
#[inline]
#[allow(dead_code)]
unsafe fn filter_special<F: StringToFloat>(base: u32, first: *const u8, last: *const u8)
    -> (F, *const u8)
{
    // special case checks
    let length = distance(first, last);
    if is_nan(first, length) {
        (F::NAN, first.add(NAN_STRING.len()))
    } else if is_infinity(first, length) {
        (F::INFINITY, first.add(INFINITY_STRING.len()))
    } else if is_zero(first, length) {
        (F::ZERO, first.add(3))
    } else {
        F::basen(base, first, last)
    }
}

/// Handle +/- values and empty buffers.
#[inline]
unsafe fn filter_sign<F: StringToFloat>(base: u32, first: *const u8, last: *const u8)
    -> (F, *const u8)
{
    if first == last {
        (F::ZERO, ptr::null())
    } else if *first == b'-' {
        let (value, p) = filter_special::<F>(base, first.add(1), last);
        (-value, p)
    } else if *first == b'+' {
        filter_special::<F>(base, first.add(1), last)
    } else {
        filter_special::<F>(base, first, last)
    }
}

/// Iteratively filter simple cases and then invoke parser.
#[inline]
unsafe fn atof<F: StringToFloat>(base: u32, first: *const u8, last: *const u8)
    -> (F, *const u8)
{
    filter_sign::<F>(base, first, last)
}

// UNSAFE API

/// Generate the unsafe API wrappers.
///
/// * `name`        Function name.
/// * `f`           Float type.
macro_rules! generate_unsafe_api {
    ($name:ident, $f:tt) => (
        /// Unsafe, C-like importer for floating-point numbers.
        #[inline]
        pub unsafe extern "C" fn $name(base: u8, first: *const u8, last: *const u8) -> ($f, *const u8, bool)
        {
            let (value, p) = atof::<$f>(base as u32, first, last);
            (value, p, false)
        }
    )
}

generate_unsafe_api!(atof32_unsafe, f32);
generate_unsafe_api!(atof64_unsafe, f64);

// WRAP UNSAFE LOCAL
generate_from_bytes_local!(atof32_local, f32, atof32_unsafe);
generate_from_bytes_local!(atof64_local, f64, atof64_unsafe);

// API
generate_from_bytes_api!(atof32_bytes, f32, atof32_local);
generate_from_bytes_api!(atof64_bytes, f64, atof64_local);
generate_try_from_bytes_api!(try_atof32_bytes, f32, atof32_local);
generate_try_from_bytes_api!(try_atof64_bytes, f64, atof64_local);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use error::invalid_digit;
    use super::*;

    #[test]
    fn atof32_base10_test() {
        // integer test
        assert_relative_eq!(0.0, atof32_bytes(10, b"0"), epsilon=1e-20);
        assert_relative_eq!(1.0, atof32_bytes(10, b"1"), epsilon=1e-20);
        assert_relative_eq!(12.0, atof32_bytes(10, b"12"), epsilon=1e-20);
        assert_relative_eq!(123.0, atof32_bytes(10, b"123"), epsilon=1e-20);
        assert_relative_eq!(1234.0, atof32_bytes(10, b"1234"), epsilon=1e-20);
        assert_relative_eq!(12345.0, atof32_bytes(10, b"12345"), epsilon=1e-20);
        assert_relative_eq!(123456.0, atof32_bytes(10, b"123456"), epsilon=1e-20);
        assert_relative_eq!(1234567.0, atof32_bytes(10, b"1234567"), epsilon=1e-20);
        assert_relative_eq!(12345678.0, atof32_bytes(10, b"12345678"), epsilon=1e-20);

        // decimal test
        assert_relative_eq!(123.1, atof32_bytes(10, b"123.1"), epsilon=1e-20);
        assert_relative_eq!(123.12, atof32_bytes(10, b"123.12"), epsilon=1e-20);
        assert_relative_eq!(123.123, atof32_bytes(10, b"123.123"), epsilon=1e-20);
        assert_relative_eq!(123.1234, atof32_bytes(10, b"123.1234"), epsilon=1e-20);
        assert_relative_eq!(123.12345, atof32_bytes(10, b"123.12345"), epsilon=1e-20);

        // rounding test
        assert_relative_eq!(123456790.0, atof32_bytes(10, b"123456789"), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(10, b"123456789.1"), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(10, b"123456789.12"), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(10, b"123456789.123"), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(10, b"123456789.1234"), epsilon=1e-20);
        assert_relative_eq!(123456790.0, atof32_bytes(10, b"123456789.12345"), epsilon=1e-20);

        // exponent test
        assert_relative_eq!(123456789.12345, atof32_bytes(10, b"1.2345678912345e8"), epsilon=1e-20);
        assert_relative_eq!(123450000.0, atof32_bytes(10, b"1.2345e+8"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof32_bytes(10, b"1.2345e+11"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof32_bytes(10, b"123450000000"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof32_bytes(10, b"1.2345e+38"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof32_bytes(10, b"123450000000000000000000000000000000000"), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof32_bytes(10, b"1.2345e-8"), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof32_bytes(10, b"0.000000012345"), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof32_bytes(10, b"1.2345e-38"), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof32_bytes(10, b"0.000000000000000000000000000000000000012345"), epsilon=1e-20);

        #[cfg(feature = "std")]
        assert!(atof32_bytes(10, b"NaN").is_nan());
        assert!(atof32_bytes(10, b"inf").is_infinite());
        assert!(atof32_bytes(10, b"+inf").is_infinite());
        assert!(atof32_bytes(10, b"-inf").is_infinite());
    }

    #[test]
    fn atof32_basen_test() {
        assert_relative_eq!(1234.0, atof32_bytes(36, b"YA"));
    }

    #[test]
    fn atof64_base10_test() {
        // integer test
        assert_relative_eq!(0.0, atof64_bytes(10, b"0"), epsilon=1e-20);
        assert_relative_eq!(1.0, atof64_bytes(10, b"1"), epsilon=1e-20);
        assert_relative_eq!(12.0, atof64_bytes(10, b"12"), epsilon=1e-20);
        assert_relative_eq!(123.0, atof64_bytes(10, b"123"), epsilon=1e-20);
        assert_relative_eq!(1234.0, atof64_bytes(10, b"1234"), epsilon=1e-20);
        assert_relative_eq!(12345.0, atof64_bytes(10, b"12345"), epsilon=1e-20);
        assert_relative_eq!(123456.0, atof64_bytes(10, b"123456"), epsilon=1e-20);
        assert_relative_eq!(1234567.0, atof64_bytes(10, b"1234567"), epsilon=1e-20);
        assert_relative_eq!(12345678.0, atof64_bytes(10, b"12345678"), epsilon=1e-20);

        // decimal test
        assert_relative_eq!(123456789.0, atof64_bytes(10, b"123456789"), epsilon=1e-20);
        assert_relative_eq!(123456789.1, atof64_bytes(10, b"123456789.1"), epsilon=1e-20);
        assert_relative_eq!(123456789.12, atof64_bytes(10, b"123456789.12"), epsilon=1e-20);
        assert_relative_eq!(123456789.123, atof64_bytes(10, b"123456789.123"), epsilon=1e-20);
        assert_relative_eq!(123456789.1234, atof64_bytes(10, b"123456789.1234"), epsilon=1e-20);
        assert_relative_eq!(123456789.12345, atof64_bytes(10, b"123456789.12345"), epsilon=1e-20);
        assert_relative_eq!(123456789.123456, atof64_bytes(10, b"123456789.123456"), epsilon=1e-20);
        assert_relative_eq!(123456789.1234567, atof64_bytes(10, b"123456789.1234567"), epsilon=1e-20);
        assert_relative_eq!(123456789.12345678, atof64_bytes(10, b"123456789.12345678"), epsilon=1e-20);

        // rounding test
        assert_relative_eq!(123456789.12345679, atof64_bytes(10, b"123456789.123456789"), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(10, b"123456789.1234567890"), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(10, b"123456789.123456789012"), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(10, b"123456789.1234567890123"), epsilon=1e-20);
        assert_relative_eq!(123456789.12345679, atof64_bytes(10, b"123456789.12345678901234"), epsilon=1e-20);

        // exponent test
        assert_relative_eq!(123456789.12345, atof64_bytes(10, b"1.2345678912345e8"), epsilon=1e-20);
        assert_relative_eq!(123450000.0, atof64_bytes(10, b"1.2345e+8"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof64_bytes(10, b"123450000000"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+11, atof64_bytes(10, b"1.2345e+11"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof64_bytes(10, b"1.2345e+38"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+38, atof64_bytes(10, b"123450000000000000000000000000000000000"), epsilon=1e-20);
        assert_relative_eq!(1.2345e+308, atof64_bytes(10, b"1.2345e+308"), max_relative=1e-12);
        assert_relative_eq!(1.2345e+308, atof64_bytes(10, b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"), max_relative=1e-12);
        assert_relative_eq!(0.000000012345, atof64_bytes(10, b"1.2345e-8"), epsilon=1e-20);
        assert_relative_eq!(1.2345e-8, atof64_bytes(10, b"0.000000012345"), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof64_bytes(10, b"1.2345e-38"), epsilon=1e-20);
        assert_relative_eq!(1.2345e-38, atof64_bytes(10, b"0.000000000000000000000000000000000000012345"), epsilon=1e-20);

        // denormalized (try extremely low values)
        assert_relative_eq!(1.2345e-308, atof64_bytes(10, b"1.2345e-308"), epsilon=1e-20);
        assert_eq!(5e-322, atof64_bytes(10, b"5e-322"));
        assert_eq!(5e-323, atof64_bytes(10, b"5e-323"));
        assert_eq!(5e-324, atof64_bytes(10, b"5e-324"));
        // due to issues in how the data is parsed, manually extracting
        // non-exponents of 1.<e-299 is prone to error
        // test the limit of our ability
        // We tend to get relative errors of 1e-16, even at super low values.
        assert_relative_eq!(1.2345e-299, atof64_bytes(10, b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=1e-314);

        // Keep pushing from -300 to -324
        assert_relative_eq!(1.2345e-300, atof64_bytes(10, b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=1e-315);
        assert_relative_eq!(1.2345e-310, atof64_bytes(10, b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=5e-324);
        assert_relative_eq!(1.2345e-320, atof64_bytes(10, b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=5e-324);
        assert_relative_eq!(1.2345e-321, atof64_bytes(10, b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=5e-324);
        assert_relative_eq!(1.24e-322, atof64_bytes(10, b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000124"), epsilon=5e-324);
        assert_eq!(1e-323, atof64_bytes(10, b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"));
        assert_eq!(5e-324, atof64_bytes(10, b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005"));

        #[cfg(feature = "std")]
        assert!(atof64_bytes(10, b"NaN").is_nan());
        assert!(atof64_bytes(10, b"inf").is_infinite());
        assert!(atof64_bytes(10, b"+inf").is_infinite());
        assert!(atof64_bytes(10, b"-inf").is_infinite());
    }

    #[test]
    #[should_panic]
    fn limit_test() {
        assert_relative_eq!(1.2345e-320, 0.0, epsilon=5e-324);
    }

    #[test]
    fn atof64_basen_test() {
        assert_relative_eq!(1234.0, atof64_bytes(36, b"YA"));
    }

    #[test]
    fn try_atof32_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atof32_bytes(10, b""));
        assert_eq!(Ok(0.0), try_atof32_bytes(10, b"0.0"));
        assert_eq!(Err(invalid_digit(1)), try_atof32_bytes(10, b"1a"));
    }

    #[test]
    fn try_atof64_base10_test() {
        assert_eq!(Err(invalid_digit(0)), try_atof64_bytes(10, b""));
        assert_eq!(Ok(0.0), try_atof64_bytes(10, b"0.0"));
        assert_eq!(Err(invalid_digit(1)), try_atof64_bytes(10, b"1a"));
    }
}
