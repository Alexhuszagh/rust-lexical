//! Low-level API generator.
//!
//! Uses either the imprecise or the precise algorithm.

use util::*;

// Select the back-end
cfg_if! {
if #[cfg(feature = "correct")] {
    use super::algorithm::correct as algorithm;
} else {
    use super::algorithm::incorrect as algorithm;
}}  // cfg_if

// TRAITS

/// Trait to define parsing of a string to float.
trait StringToFloat: Float {
    /// Serialize string to float, favoring correctness.
    fn default(radix: u32, bytes: &[u8], sign: Sign) -> (Self, usize);

    /// Serialize string to float, prioritizing speed over correctness.
    fn lossy(radix: u32, bytes: &[u8], sign: Sign) -> (Self, usize);
}

impl StringToFloat for f32 {
    #[inline]
    fn default(radix: u32, bytes: &[u8], sign: Sign) -> (f32, usize) {
        algorithm::atof(radix, bytes, sign)
    }

    #[inline]
    fn lossy(radix: u32, bytes: &[u8], sign: Sign) -> (f32, usize) {
        algorithm::atof_lossy(radix, bytes, sign)
    }
}

impl StringToFloat for f64 {
    #[inline]
    fn default(radix: u32, bytes: &[u8], sign: Sign) -> (f64, usize) {
        algorithm::atod(radix, bytes, sign)
    }

    #[inline]
    fn lossy(radix: u32, bytes: &[u8], sign: Sign) -> (f64, usize) {
        algorithm::atod_lossy(radix, bytes, sign)
    }
}

// SPECIAL
// Utilities to filter special values.

#[inline]
fn is_nan(bytes: &[u8]) -> bool {
    case_insensitive_starts_with_slice(bytes, get_nan_string())
}

#[inline]
fn is_inf(bytes: &[u8]) -> bool {
    case_insensitive_starts_with_slice(bytes, get_inf_string())
}

#[inline]
fn is_infinity(bytes: &[u8]) -> bool {
    case_insensitive_starts_with_slice(bytes, get_infinity_string())
}

#[inline]
fn is_zero(bytes: &[u8]) -> bool {
    // Ignore other variants of 0, we just want to most common literal ones.
    match bytes.len() {
        1 => equal_to_slice(bytes, b"0"),
        3 => equal_to_slice(bytes, b"0.0"),
        _ => false,
    }
}

// ATOF

/// Convert string to float and handle special floating-point strings.
/// Forcing inlining leads to much better codegen at high optimization levels.
#[inline]
fn filter_special<'a, F: StringToFloat>(radix: u32, bytes: &'a [u8], lossy: bool, sign: Sign)
    -> (F, usize)
{
    // Special case checks
    // Check long infinity first before short infinity.
    // Short infinity short-circuits, we want to parse as many characters
    // as possible.
    if is_zero(bytes) {
        (F::ZERO, bytes.len())
    } else if is_infinity(bytes) {
        let len = get_infinity_string().len();
        (F::INFINITY, len)
    } else if is_inf(bytes) {
        let len = get_inf_string().len();
        (F::INFINITY, len)
    } else if is_nan(bytes) {
        let len = get_nan_string().len();
        (F::NAN, len)
    } else if bytes.len() == 1 && index!(bytes[0]) == b'.' {
        // We know the above statement is safe, since `bytes.len() == 1`.
        // Handle case where we have a decimal point, but no leading or trailing
        // digits. This should return a value of 0, but the checked parsers
        // should reject this out-right.
        (F::ZERO, 0)
    } else if lossy {
        F::lossy(radix, bytes, sign)
    } else {
        F::default(radix, bytes, sign)
    }
}

/// Handle +/- values and empty buffers.
/// Forcing inlining leads to much better codegen at high optimization levels.
#[inline]
fn filter_sign<'a, F: StringToFloat>(radix: u32, bytes: &'a [u8], lossy: bool)
    -> (F, Sign, usize)
{
    let len = bytes.len();
    let (sign_bytes, sign) = match bytes.get(0) {
        Some(&b'+') => (1, Sign::Positive),
        Some(&b'-') => (1, Sign::Negative),
        _           => (0, Sign::Positive),
    };

    if len > sign_bytes {
        // `bytes.len() > sign_bytes`, so this range is always valid.
        let bytes = &index!(bytes[sign_bytes..]);
        let (value, len) = filter_special::<F>(radix, bytes, lossy, sign);
        (value, sign, len + sign_bytes)
    } else {
        (F::ZERO, sign, 0)
    }
}

/// Iteratively filter simple cases and then invoke parser.
/// Forcing inlining leads to much better codegen at high optimization levels.
#[inline]
fn atof<F: StringToFloat>(radix: u32, bytes: &[u8], lossy: bool)
    -> (F, usize)
{
    let (value, sign, len) = filter_sign::<F>(radix, bytes, lossy);
    match sign {
        Sign::Negative => (-value, len),
        Sign::Positive => (value, len),
    }
}

// UNSAFE API

/// Expand the generic atof function for specified types.
macro_rules! wrap {
    ($name:ident, $f:tt, $lossy:expr) => (
        /// Parse float and return value, subslice read, and if truncated.
        #[inline]
        fn $name(radix: u8, bytes: &[u8])
            -> ($f, usize, bool)
        {
            let (value, len) = atof::<$f>(radix.into(), bytes, $lossy);
            (value, len, false)
        }
    )
}

wrap!(atof32_impl, f32, false);
wrap!(atof64_impl, f64, false);
wrap!(atof32_lossy_impl, f32, true);
wrap!(atof64_lossy_impl, f64, true);

// RANGE API (FFI)
generate_from_range_api!(atof32_range, atof32_radix_range, f32, atof32_impl);
generate_from_range_api!(atof64_range, atof64_radix_range, f64, atof64_impl);
generate_from_range_api!(atof32_lossy_range, atof32_lossy_radix_range, f32, atof32_lossy_impl);
generate_from_range_api!(atof64_lossy_range, atof64_lossy_radix_range, f64, atof64_lossy_impl);
generate_try_from_range_api!(try_atof32_range, try_atof32_radix_range, f32, atof32_impl);
generate_try_from_range_api!(try_atof64_range, try_atof64_radix_range, f64, atof64_impl);
generate_try_from_range_api!(try_atof32_lossy_range, try_atof32_lossy_radix_range, f32, atof32_lossy_impl);
generate_try_from_range_api!(try_atof64_lossy_range, try_atof64_lossy_radix_range, f64, atof64_lossy_impl);

// SLICE API
generate_from_slice_api!(atof32_slice, atof32_radix_slice, f32, atof32_impl);
generate_from_slice_api!(atof64_slice, atof64_radix_slice, f64, atof64_impl);
generate_from_slice_api!(atof32_lossy_slice, atof32_lossy_radix_slice, f32, atof32_lossy_impl);
generate_from_slice_api!(atof64_lossy_slice, atof64_lossy_radix_slice, f64, atof64_lossy_impl);
generate_try_from_slice_api!(try_atof32_slice, try_atof32_radix_slice, f32, atof32_impl);
generate_try_from_slice_api!(try_atof64_slice, try_atof64_radix_slice, f64, atof64_impl);
generate_try_from_slice_api!(try_atof32_lossy_slice, try_atof32_lossy_radix_slice, f32, atof32_lossy_impl);
generate_try_from_slice_api!(try_atof64_lossy_slice, try_atof64_lossy_radix_slice, f64, atof64_lossy_impl);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atof32_base10_test() {
        // integer test
        assert_f32_eq!(0.0, atof32_slice(b"0"));
        assert_f32_eq!(1.0, atof32_slice(b"1"));
        assert_f32_eq!(12.0, atof32_slice(b"12"));
        assert_f32_eq!(123.0, atof32_slice(b"123"));
        assert_f32_eq!(1234.0, atof32_slice(b"1234"));
        assert_f32_eq!(12345.0, atof32_slice(b"12345"));
        assert_f32_eq!(123456.0, atof32_slice(b"123456"));
        assert_f32_eq!(1234567.0, atof32_slice(b"1234567"));
        assert_f32_eq!(12345678.0, atof32_slice(b"12345678"));

        // No decimal but decimal point test
        assert_f64_eq!(1.0, atof32_slice(b"1."));
        assert_f64_eq!(12.0, atof32_slice(b"12."));
        assert_f64_eq!(1234567.0, atof32_slice(b"1234567."));

        // decimal test
        assert_f32_eq!(123.1, atof32_slice(b"123.1"));
        assert_f32_eq!(123.12, atof32_slice(b"123.12"));
        assert_f32_eq!(123.123, atof32_slice(b"123.123"));
        assert_f32_eq!(123.1234, atof32_slice(b"123.1234"));
        assert_f32_eq!(123.12345, atof32_slice(b"123.12345"));

        // rounding test
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789"));
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.1"));
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.12"));
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.123"));
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.1234"));
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.12345"));

        // exponent test
        assert_f32_eq!(123456789.12345, atof32_slice(b"1.2345678912345e8"));
        assert_f32_eq!(123450000.0, atof32_slice(b"1.2345e+8"));
        assert_f32_eq!(1.2345e+11, atof32_slice(b"1.2345e+11"));
        assert_f32_eq!(1.2345e+11, atof32_slice(b"123450000000"));
        assert_f32_eq!(1.2345e+38, atof32_slice(b"1.2345e+38"));
        assert_f32_eq!(1.2345e+38, atof32_slice(b"123450000000000000000000000000000000000"));
        assert_f32_eq!(1.2345e-8, atof32_slice(b"1.2345e-8"));
        assert_f32_eq!(1.2345e-8, atof32_slice(b"0.000000012345"));
        assert_f32_eq!(1.2345e-38, atof32_slice(b"1.2345e-38"));
        assert_f32_eq!(1.2345e-38, atof32_slice(b"0.000000000000000000000000000000000000012345"));

        assert!(atof32_slice(b"NaN").is_nan());
        assert!(atof32_slice(b"nan").is_nan());
        assert!(atof32_slice(b"NAN").is_nan());
        assert!(atof32_slice(b"inf").is_infinite());
        assert!(atof32_slice(b"INF").is_infinite());
        assert!(atof32_slice(b"+inf").is_infinite());
        assert!(atof32_slice(b"-inf").is_infinite());

        // Bug fix for Issue #8
        assert_eq!(5.002868148396374, atof32_slice(b"5.002868148396374"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atof32_basen_test() {
        assert_f32_eq!(1234.0, atof32_radix_slice(36, b"YA"));
        assert_f32_eq!(1234.0, atof32_lossy_radix_slice(36, b"YA"));
    }

    #[test]
    fn atof64_base10_test() {
        // integer test
        assert_f64_eq!(0.0, atof64_slice(b"0"));
        assert_f64_eq!(1.0, atof64_slice(b"1"));
        assert_f64_eq!(12.0, atof64_slice(b"12"));
        assert_f64_eq!(123.0, atof64_slice(b"123"));
        assert_f64_eq!(1234.0, atof64_slice(b"1234"));
        assert_f64_eq!(12345.0, atof64_slice(b"12345"));
        assert_f64_eq!(123456.0, atof64_slice(b"123456"));
        assert_f64_eq!(1234567.0, atof64_slice(b"1234567"));
        assert_f64_eq!(12345678.0, atof64_slice(b"12345678"));

        // No decimal but decimal point test
        assert_f64_eq!(1.0, atof64_slice(b"1."));
        assert_f64_eq!(12.0, atof64_slice(b"12."));
        assert_f64_eq!(1234567.0, atof64_slice(b"1234567."));

        // decimal test
        assert_f64_eq!(123456789.0, atof64_slice(b"123456789"));
        assert_f64_eq!(123456789.1, atof64_slice(b"123456789.1"));
        assert_f64_eq!(123456789.12, atof64_slice(b"123456789.12"));
        assert_f64_eq!(123456789.123, atof64_slice(b"123456789.123"));
        assert_f64_eq!(123456789.1234, atof64_slice(b"123456789.1234"));
        assert_f64_eq!(123456789.12345, atof64_slice(b"123456789.12345"));
        assert_f64_eq!(123456789.123456, atof64_slice(b"123456789.123456"));
        assert_f64_eq!(123456789.1234567, atof64_slice(b"123456789.1234567"));
        assert_f64_eq!(123456789.12345678, atof64_slice(b"123456789.12345678"));

        // rounding test
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.123456789"));
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.1234567890"));
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.123456789012"));
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.1234567890123"));
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.12345678901234"));

        // exponent test
        assert_f64_eq!(123456789.12345, atof64_slice(b"1.2345678912345e8"));
        assert_f64_eq!(123450000.0, atof64_slice(b"1.2345e+8"));
        assert_f64_eq!(1.2345e+11, atof64_slice(b"123450000000"));
        assert_f64_eq!(1.2345e+11, atof64_slice(b"1.2345e+11"));
        assert_f64_eq!(1.2345e+38, atof64_slice(b"1.2345e+38"));
        assert_f64_eq!(1.2345e+38, atof64_slice(b"123450000000000000000000000000000000000"));
        assert_f64_eq!(1.2345e+308, atof64_slice(b"1.2345e+308"));
        assert_f64_eq!(1.2345e+308, atof64_slice(b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"));
        assert_f64_eq!(0.000000012345, atof64_slice(b"1.2345e-8"));
        assert_f64_eq!(1.2345e-8, atof64_slice(b"0.000000012345"));
        assert_f64_eq!(1.2345e-38, atof64_slice(b"1.2345e-38"));
        assert_f64_eq!(1.2345e-38, atof64_slice(b"0.000000000000000000000000000000000000012345"));

        // denormalized (try extremely low values)
        assert_f64_eq!(1.2345e-308, atof64_slice(b"1.2345e-308"));

        // These tests fail with the incorrect parser on the ARMv6 architecture.
        // This works fine with the correct parser, or ARMv7 or ARMv8 (aarch64).
        #[cfg(any(
            feature = "correct",
            not(all(target_arch = "arm", not(target_feature = "v7")))
        ))] {
            assert_eq!(5e-322, atof64_slice(b"5e-322"));
            assert_eq!(5e-323, atof64_slice(b"5e-323"));
            assert_eq!(5e-324, atof64_slice(b"5e-324"));
            // due to issues in how the data is parsed, manually extracting
            // non-exponents of 1.<e-299 is prone to error
            // test the limit of our ability
            // We tend to get relative errors of 1e-16, even at super low values.
            assert_f64_eq!(1.2345e-299, atof64_slice(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=1e-314);

            // Keep pushing from -300 to -324
            assert_f64_eq!(1.2345e-300, atof64_slice(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=1e-315);
            assert_f64_eq!(1.2345e-310, atof64_slice(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=5e-324);
            assert_f64_eq!(1.2345e-320, atof64_slice(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=5e-324);
            assert_f64_eq!(1.2345e-321, atof64_slice(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345"), epsilon=5e-324);
            assert_f64_eq!(1.24e-322, atof64_slice(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000124"), epsilon=5e-324);
            assert_eq!(1e-323, atof64_slice(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"));
            assert_eq!(5e-324, atof64_slice(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005"));
        }

        assert!(atof64_slice(b"NaN").is_nan());
        assert!(atof64_slice(b"nan").is_nan());
        assert!(atof64_slice(b"NAN").is_nan());
        assert!(atof64_slice(b"inf").is_infinite());
        assert!(atof64_slice(b"INF").is_infinite());
        assert!(atof64_slice(b"+inf").is_infinite());
        assert!(atof64_slice(b"-inf").is_infinite());

        // Check various reports from a fuzzer
        assert_eq!(0.0, atof64_slice(b"0e"));
        assert_eq!(0.0, atof64_slice(b"0.0e"));
        assert_eq!(0.0, atof64_slice(b".E"));
        assert_eq!(0.0, atof64_slice(b".e"));
        assert_eq!(0.0, atof64_slice(b"E2252525225"));
        assert_eq!(f64::INFINITY, atof64_slice(b"2E200000000000"));

        // Add various unittests from proptests.
        assert_eq!(ErrorCode::InvalidDigit, try_atof64_slice(b"0e").error.code);
        assert_eq!(ErrorCode::InvalidDigit, try_atof64_slice(b".").error.code);
        assert_eq!(ErrorCode::InvalidDigit, try_atof64_slice(b"+.").error.code);
        assert_eq!(ErrorCode::InvalidDigit, try_atof64_slice(b"-.").error.code);
        assert_eq!(ErrorCode::InvalidDigit, try_atof64_slice(b"+").error.code);
        assert_eq!(ErrorCode::InvalidDigit, try_atof64_slice(b"-").error.code);

        // Bug fix for Issue #8
        assert_eq!(5.002868148396374, atof64_slice(b"5.002868148396374"));
    }

    #[test]
    #[should_panic]
    fn limit_test() {
        assert_relative_eq!(1.2345e-320, 0.0, epsilon=5e-324);
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atof64_basen_test() {
        assert_f64_eq!(1234.0, atof64_radix_slice(36, b"YA"));
        assert_f64_eq!(1234.0, atof64_lossy_radix_slice(36, b"YA"));
    }

    #[test]
    fn try_atof32_base10_test() {
        assert_eq!(invalid_digit_error(0.0, 0), try_atof32_slice(b"."));
        assert_eq!(empty_error(0.0), try_atof32_slice(b""));
        assert_eq!(success(0.0), try_atof32_slice(b"0.0"));
        assert_eq!(invalid_digit_error(1.0, 1), try_atof32_slice(b"1a"));

        // Bug fix for Issue #8
        assert_eq!(success(5.002868148396374), try_atof32_slice(b"5.002868148396374"));
    }

    #[test]
    fn try_atof64_base10_test() {
        assert_eq!(invalid_digit_error(0.0, 0), try_atof64_slice(b"."));
        assert_eq!(empty_error(0.0), try_atof64_slice(b""));
        assert_eq!(success(0.0), try_atof64_slice(b"0.0"));
        assert_eq!(invalid_digit_error(1.0, 1), try_atof64_slice(b"1a"));

        // Bug fix for Issue #8
        assert_eq!(success(5.002868148396374), try_atof64_slice(b"5.002868148396374"));
    }

    proptest! {
        #[test]
        fn f32_invalid_proptest(i in r"[+-]?[0-9]{2}\D?\.\D?[0-9]{2}\D?e[+-]?[0-9]+\D") {
            let res = try_atof32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f32_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
            let res = try_atof32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0 || res.error.index == 1);
        }

        #[test]
        fn f32_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = try_atof32_slice(i.as_bytes());
            if i.is_empty() {
                assert_eq!(res.error.code, ErrorCode::Empty);
            } else {
                assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            }
            assert!(res.error.index == 0 || res.error.index == 1);
        }

        #[test]
        fn f32_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = try_atof32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f32_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = try_atof32_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f64_invalid_proptest(i in r"[+-]?[0-9]{2}\D?\.\D?[0-9]{2}\D?e[+-]?[0-9]+\D") {
            let res = try_atof64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f64_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
            let res = try_atof64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            assert!(res.error.index == 0 || res.error.index == 1);
        }

        #[test]
        fn f64_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = try_atof64_slice(i.as_bytes());
            if i.is_empty() {
                assert_eq!(res.error.code, ErrorCode::Empty);
            } else {
                assert_eq!(res.error.code, ErrorCode::InvalidDigit);
            }
            assert!(res.error.index == 0 || res.error.index == 1);
        }

        #[test]
        fn f64_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = try_atof64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f64_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = try_atof64_slice(i.as_bytes());
            assert_eq!(res.error.code, ErrorCode::InvalidDigit);
        }
    }
}
