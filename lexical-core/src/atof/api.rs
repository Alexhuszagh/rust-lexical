//! Prototype for version 3 of atof.

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
    fn default(radix: u32, bytes: &[u8], sign: Sign, offset: usize) -> Result<Self>;

    /// Serialize string to float, prioritizing speed over correctness.
    fn lossy(radix: u32, bytes: &[u8], sign: Sign, offset: usize) -> Result<Self>;
}

impl StringToFloat for f32 {
    perftools_inline_always!{
    fn default(radix: u32, bytes: &[u8], sign: Sign, offset: usize) -> Result<f32> {
        algorithm::atof(radix, bytes, sign, offset)
    }}

    perftools_inline_always!{
    fn lossy(radix: u32, bytes: &[u8], sign: Sign, offset: usize) -> Result<f32> {
        algorithm::atof_lossy(radix, bytes, sign, offset)
    }}
}

impl StringToFloat for f64 {
    perftools_inline_always!{
    fn default(radix: u32, bytes: &[u8], sign: Sign, offset: usize) -> Result<f64> {
        algorithm::atod(radix, bytes, sign, offset)
    }}

    perftools_inline_always!{
    fn lossy(radix: u32, bytes: &[u8], sign: Sign, offset: usize) -> Result<f64> {
        algorithm::atod_lossy(radix, bytes, sign, offset)
    }}
}

// SPECIAL
// Utilities to filter special values.

perftools_inline!{
fn is_nan(bytes: &[u8]) -> bool {
    case_insensitive_equal_to_slice(bytes, get_nan_string())
}}

perftools_inline!{
fn is_inf(bytes: &[u8]) -> bool {
    case_insensitive_equal_to_slice(bytes, get_inf_string())
}}

perftools_inline!{
fn is_infinity(bytes: &[u8]) -> bool {
    case_insensitive_equal_to_slice(bytes, get_infinity_string())
}}

// PARSER

perftools_inline!{
fn parse_float<F: StringToFloat>(radix: u32, bytes: &[u8], lossy: bool, sign: Sign, offset: usize)
    -> Result<F>
{
    match lossy {
        true  => F::lossy(radix, bytes, sign, offset),
        false => F::default(radix, bytes, sign, offset),
    }
}}

// ATOF/ATOD

// Standalone atof processor.
perftools_inline!{
fn atof<F: StringToFloat>(radix: u32, bytes: &[u8], lossy: bool)
    -> Result<F>
{
    // Filter out empty inputs.
    if bytes.is_empty() {
        return Err(ErrorCode::Empty.into());
    }

    let (sign, offset, bytes) = match &index!(bytes[0]) {
        b'+' => (Sign::Positive, 1, &index!(bytes[1..])),
        b'-' => (Sign::Negative, 1, &index!(bytes[1..])),
        _    => (Sign::Positive, 0, bytes),
    };

    // Filter out empty inputs.
    if bytes.is_empty() {
        return Err(ErrorCode::Empty.into());
    }

    // Special case checks
    // Use predictive parsing to filter special cases. This leads to
    // dramatic performance gains.
    let float = match index!(bytes[0]) {
        b'i' | b'I' => {
            // Check long infinity first before short infinity.
            // Short infinity short-circuits, we want to parse as many characters
            // as possible.
            if is_infinity(bytes) || is_inf(bytes) {
                // Have a valid long-form or short-form infinity.
                Ok(F::INFINITY)
            } else {
                // Not infinity, may be valid with a different radix.
                if cfg!(feature = "radix"){
                    parse_float(radix, bytes, lossy, sign, offset)
                } else {
                    Err((ErrorCode::InvalidDigit, offset).into())
                }
            }
        },
        b'N' | b'n' => {
            if is_nan(bytes) {
                // Have a valid NaN.
                Ok(F::NAN)
            } else {
                // Not NaN, may be valid with a different radix.
                if cfg!(feature = "radix"){
                    parse_float(radix, bytes, lossy, sign, offset)
                } else {
                    Err((ErrorCode::InvalidDigit, offset).into())
                }
            }
        },
        _   => parse_float(radix, bytes, lossy, sign, offset),
    }?;

    // Process the sign.
    Ok(match sign {
        Sign::Positive => float,
        Sign::Negative => -float,
    })
}}

perftools_inline!{
fn atof_lossy<F: StringToFloat>(radix: u32, bytes: &[u8])
    -> Result<F>
{
    atof::<F>(radix, bytes, true)
}}

perftools_inline!{
fn atof_nonlossy<F: StringToFloat>(radix: u32, bytes: &[u8])
    -> Result<F>
{
    atof::<F>(radix, bytes, false)
}}

// API
// ---

// RANGE API (FFI)
generate_from_range_api!(atof32_range, atof32_radix_range, f32, atof_nonlossy);
generate_from_range_api!(atof32_lossy_range, atof32_lossy_radix_range, f32, atof_lossy);
generate_from_range_api!(atof64_range, atof64_radix_range, f64, atof_nonlossy);
generate_from_range_api!(atof64_lossy_range, atof64_lossy_radix_range, f64, atof_lossy);

// SLICE API
generate_from_slice_api!(atof32_slice, atof32_radix_slice, f32, atof_nonlossy);
generate_from_slice_api!(atof32_lossy_slice, atof32_lossy_radix_slice, f32, atof_lossy);
generate_from_slice_api!(atof64_slice, atof64_radix_slice, f64, atof_nonlossy);
generate_from_slice_api!(atof64_lossy_slice, atof64_lossy_radix_slice, f64, atof_lossy);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atof32_base10_test() {
        // integer test
        assert_f32_eq!(0.0, atof32_slice(b"0").unwrap());
        assert_f32_eq!(1.0, atof32_slice(b"1").unwrap());
        assert_f32_eq!(12.0, atof32_slice(b"12").unwrap());
        assert_f32_eq!(123.0, atof32_slice(b"123").unwrap());
        assert_f32_eq!(1234.0, atof32_slice(b"1234").unwrap());
        assert_f32_eq!(12345.0, atof32_slice(b"12345").unwrap());
        assert_f32_eq!(123456.0, atof32_slice(b"123456").unwrap());
        assert_f32_eq!(1234567.0, atof32_slice(b"1234567").unwrap());
        assert_f32_eq!(12345678.0, atof32_slice(b"12345678").unwrap());

        // No decimal but decimal point test
        assert_f64_eq!(1.0, atof32_slice(b"1.").unwrap());
        assert_f64_eq!(12.0, atof32_slice(b"12.").unwrap());
        assert_f64_eq!(1234567.0, atof32_slice(b"1234567.").unwrap());

        // decimal test
        assert_f32_eq!(123.1, atof32_slice(b"123.1").unwrap());
        assert_f32_eq!(123.12, atof32_slice(b"123.12").unwrap());
        assert_f32_eq!(123.123, atof32_slice(b"123.123").unwrap());
        assert_f32_eq!(123.1234, atof32_slice(b"123.1234").unwrap());
        assert_f32_eq!(123.12345, atof32_slice(b"123.12345").unwrap());

        // rounding test
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789").unwrap());
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.1").unwrap());
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.12").unwrap());
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.123").unwrap());
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.1234").unwrap());
        assert_f32_eq!(123456790.0, atof32_slice(b"123456789.12345").unwrap());

        // exponent test
        assert_f32_eq!(123456789.12345, atof32_slice(b"1.2345678912345e8").unwrap());
        assert_f32_eq!(123450000.0, atof32_slice(b"1.2345e+8").unwrap());
        assert_f32_eq!(1.2345e+11, atof32_slice(b"1.2345e+11").unwrap());
        assert_f32_eq!(1.2345e+11, atof32_slice(b"123450000000").unwrap());
        assert_f32_eq!(1.2345e+38, atof32_slice(b"1.2345e+38").unwrap());
        assert_f32_eq!(1.2345e+38, atof32_slice(b"123450000000000000000000000000000000000").unwrap());
        assert_f32_eq!(1.2345e-8, atof32_slice(b"1.2345e-8").unwrap());
        assert_f32_eq!(1.2345e-8, atof32_slice(b"0.000000012345").unwrap());
        assert_f32_eq!(1.2345e-38, atof32_slice(b"1.2345e-38").unwrap());
        assert_f32_eq!(1.2345e-38, atof32_slice(b"0.000000000000000000000000000000000000012345").unwrap());

        assert!(atof32_slice(b"NaN").unwrap().is_nan());
        assert!(atof32_slice(b"nan").unwrap().is_nan());
        assert!(atof32_slice(b"NAN").unwrap().is_nan());
        assert!(atof32_slice(b"inf").unwrap().is_infinite());
        assert!(atof32_slice(b"INF").unwrap().is_infinite());
        assert!(atof32_slice(b"+inf").unwrap().is_infinite());
        assert!(atof32_slice(b"-inf").unwrap().is_infinite());

        // Check various expected failures.
        assert_eq!(Err(ErrorCode::Empty.into()), atof32_slice(b""));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof32_slice(b"e"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof32_slice(b"E"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof32_slice(b".e1"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof32_slice(b".e-1"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof32_slice(b"e1"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof32_slice(b"e-1"));
        assert_eq!(Err(ErrorCode::Empty.into()), atof32_slice(b"+"));
        assert_eq!(Err(ErrorCode::Empty.into()), atof32_slice(b"-"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), atof32_slice(b"5.002868148396374"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atof32_basen_test() {
        assert_f32_eq!(1234.0, atof32_radix_slice(36, b"YA").unwrap());
        assert_f32_eq!(1234.0, atof32_lossy_radix_slice(36, b"YA").unwrap());
    }

    #[test]
    fn atof64_base10_test() {
        // integer test
        assert_f64_eq!(0.0, atof64_slice(b"0").unwrap());
        assert_f64_eq!(1.0, atof64_slice(b"1").unwrap());
        assert_f64_eq!(12.0, atof64_slice(b"12").unwrap());
        assert_f64_eq!(123.0, atof64_slice(b"123").unwrap());
        assert_f64_eq!(1234.0, atof64_slice(b"1234").unwrap());
        assert_f64_eq!(12345.0, atof64_slice(b"12345").unwrap());
        assert_f64_eq!(123456.0, atof64_slice(b"123456").unwrap());
        assert_f64_eq!(1234567.0, atof64_slice(b"1234567").unwrap());
        assert_f64_eq!(12345678.0, atof64_slice(b"12345678").unwrap());

        // No decimal but decimal point test
        assert_f64_eq!(1.0, atof64_slice(b"1.").unwrap());
        assert_f64_eq!(12.0, atof64_slice(b"12.").unwrap());
        assert_f64_eq!(1234567.0, atof64_slice(b"1234567.").unwrap());

        // decimal test
        assert_f64_eq!(123456789.0, atof64_slice(b"123456789").unwrap());
        assert_f64_eq!(123456789.1, atof64_slice(b"123456789.1").unwrap());
        assert_f64_eq!(123456789.12, atof64_slice(b"123456789.12").unwrap());
        assert_f64_eq!(123456789.123, atof64_slice(b"123456789.123").unwrap());
        assert_f64_eq!(123456789.1234, atof64_slice(b"123456789.1234").unwrap());
        assert_f64_eq!(123456789.12345, atof64_slice(b"123456789.12345").unwrap());
        assert_f64_eq!(123456789.123456, atof64_slice(b"123456789.123456").unwrap());
        assert_f64_eq!(123456789.1234567, atof64_slice(b"123456789.1234567").unwrap());
        assert_f64_eq!(123456789.12345678, atof64_slice(b"123456789.12345678").unwrap());

        // rounding test
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.123456789").unwrap());
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.1234567890").unwrap());
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.123456789012").unwrap());
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.1234567890123").unwrap());
        assert_f64_eq!(123456789.12345679, atof64_slice(b"123456789.12345678901234").unwrap());

        // exponent test
        assert_f64_eq!(123456789.12345, atof64_slice(b"1.2345678912345e8").unwrap());
        assert_f64_eq!(123450000.0, atof64_slice(b"1.2345e+8").unwrap());
        assert_f64_eq!(1.2345e+11, atof64_slice(b"123450000000").unwrap());
        assert_f64_eq!(1.2345e+11, atof64_slice(b"1.2345e+11").unwrap());
        assert_f64_eq!(1.2345e+38, atof64_slice(b"1.2345e+38").unwrap());
        assert_f64_eq!(1.2345e+38, atof64_slice(b"123450000000000000000000000000000000000").unwrap());
        assert_f64_eq!(1.2345e+308, atof64_slice(b"1.2345e+308").unwrap());
        assert_f64_eq!(1.2345e+308, atof64_slice(b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap());
        assert_f64_eq!(0.000000012345, atof64_slice(b"1.2345e-8").unwrap());
        assert_f64_eq!(1.2345e-8, atof64_slice(b"0.000000012345").unwrap());
        assert_f64_eq!(1.2345e-38, atof64_slice(b"1.2345e-38").unwrap());
        assert_f64_eq!(1.2345e-38, atof64_slice(b"0.000000000000000000000000000000000000012345").unwrap());

        // denormalized (try extremely low values)
        assert_f64_eq!(1.2345e-308, atof64_slice(b"1.2345e-308").unwrap());
        assert_eq!(Ok(5e-322), atof64_slice(b"5e-322"));
        assert_eq!(Ok(5e-323), atof64_slice(b"5e-323"));
        assert_eq!(Ok(5e-324), atof64_slice(b"5e-324"));
        // due to issues in how the data is parsed, manually extracting
        // non-exponents of 1.<e-299 is prone to error
        // test the limit of our ability
        // We tend to get relative errors of 1e-16, even at super low values.
        assert_f64_eq!(1.2345e-299, atof64_slice(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=1e-314);

        // Keep pushing from -300 to -324
        assert_f64_eq!(1.2345e-300, atof64_slice(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=1e-315);
        assert_f64_eq!(1.2345e-310, atof64_slice(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        assert_f64_eq!(1.2345e-320, atof64_slice(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        assert_f64_eq!(1.2345e-321, atof64_slice(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        assert_f64_eq!(1.24e-322, atof64_slice(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000124").unwrap(), epsilon=5e-324);
        assert_eq!(Ok(1e-323), atof64_slice(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"));
        assert_eq!(Ok(5e-324), atof64_slice(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005"));

        assert!(atof64_slice(b"NaN").unwrap().is_nan());
        assert!(atof64_slice(b"nan").unwrap().is_nan());
        assert!(atof64_slice(b"NAN").unwrap().is_nan());
        assert!(atof64_slice(b"inf").unwrap().is_infinite());
        assert!(atof64_slice(b"INF").unwrap().is_infinite());
        assert!(atof64_slice(b"+inf").unwrap().is_infinite());
        assert!(atof64_slice(b"-inf").unwrap().is_infinite());

        // Check various expected failures.
        assert_eq!(Err(ErrorCode::Empty.into()), atof64_slice(b""));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof64_slice(b"e"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof64_slice(b"E"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof64_slice(b".e1"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof64_slice(b".e-1"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof64_slice(b"e1"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof64_slice(b"e-1"));

        // Check various reports from a fuzzer.
        assert_eq!(Err(ErrorCode::EmptyExponent.into()), atof64_slice(b"0e"));
        assert_eq!(Err(ErrorCode::EmptyExponent.into()), atof64_slice(b"0.0e"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof64_slice(b".E"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof64_slice(b".e"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof64_slice(b"E2252525225"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), atof64_slice(b"e2252525225"));
        assert_eq!(Ok(f64::INFINITY), atof64_slice(b"2E200000000000"));

        // Add various unittests from proptests.
        assert_eq!(Err(ErrorCode::EmptyExponent.into()), atof64_slice(b"0e"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof64_slice(b"."));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof64_slice(b"+."));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof64_slice(b"-."));
        assert_eq!(Err(ErrorCode::Empty.into()), atof64_slice(b"+"));
        assert_eq!(Err(ErrorCode::Empty.into()), atof64_slice(b"-"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), atof64_slice(b"5.002868148396374"));
    }

    #[test]
    #[should_panic]
    fn limit_test() {
        assert_relative_eq!(1.2345e-320, 0.0, epsilon=5e-324);
    }

    #[cfg(feature = "radix")]
    #[test]
    fn atof64_basen_test() {
        assert_f64_eq!(1234.0, atof64_radix_slice(36, b"YA").unwrap());
        assert_f64_eq!(1234.0, atof64_lossy_radix_slice(36, b"YA").unwrap());
    }

    #[test]
    fn try_atof32_base10_test() {
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof32_slice(b"."));
        assert_eq!(Err(ErrorCode::Empty.into()), atof32_slice(b""));
        assert_eq!(Ok(0.0), atof32_slice(b"0.0"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atof32_slice(b"1a"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), atof32_slice(b"5.002868148396374"));
    }

    #[test]
    fn try_atof64_base10_test() {
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), atof64_slice(b"."));
        assert_eq!(Err(ErrorCode::Empty.into()), atof64_slice(b""));
        assert_eq!(Ok(0.0), atof64_slice(b"0.0"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), atof64_slice(b"1a"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), atof64_slice(b"5.002868148396374"));
    }

    proptest! {
        #[test]
        fn f32_invalid_proptest(i in r"[+-]?[0-9]{2}\D?\.\D?[0-9]{2}\D?e[+-]?[0-9]+\D") {
            let res = atof32_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f32_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
            let res = atof32_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.code, ErrorCode::InvalidDigit);
            assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f32_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = atof32_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert!(err.code == ErrorCode::Empty || err.code == ErrorCode::EmptyFraction);
            assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f32_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = atof32_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f32_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = atof32_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[test]
        fn f64_invalid_proptest(i in r"[+-]?[0-9]{2}\D?\.\D?[0-9]{2}\D?e[+-]?[0-9]+\D") {
            let res = atof64_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f64_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
            let res = atof64_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.code, ErrorCode::InvalidDigit);
            assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f64_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = atof64_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert!(err.code == ErrorCode::Empty || err.code == ErrorCode::EmptyFraction);
            assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f64_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = atof64_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f64_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = atof64_slice(i.as_bytes());
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.code, ErrorCode::EmptyExponent);
        }
    }
}
