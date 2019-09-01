//! Low-level API generator.
//!
//! Uses either the imprecise or the precise algorithm.

use util::*;
use lib::result::Result as StdResult;

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
    fn default(bytes: &[u8], radix: u32, sign: Sign) -> StdResult<(Self, *const u8), (ErrorCode, *const u8)>;

    /// Serialize string to float, prioritizing speed over correctness.
    fn lossy(bytes: &[u8], radix: u32, sign: Sign) -> StdResult<(Self, *const u8), (ErrorCode, *const u8)>;
}

impl StringToFloat for f32 {
    perftools_inline_always!{
    fn default(bytes: &[u8], radix: u32, sign: Sign)
        -> StdResult<(f32, *const u8), (ErrorCode, *const u8)>
    {
        algorithm::atof(bytes, radix, sign)
    }}

    perftools_inline_always!{
    fn lossy(bytes: &[u8], radix: u32, sign: Sign)
        -> StdResult<(f32, *const u8), (ErrorCode, *const u8)>
    {
        algorithm::atof_lossy(bytes, radix, sign)
    }}
}

impl StringToFloat for f64 {
    perftools_inline_always!{
    fn default(bytes: &[u8], radix: u32, sign: Sign)
        -> StdResult<(f64, *const u8), (ErrorCode, *const u8)>
    {
        algorithm::atod(bytes, radix, sign)
    }}

    perftools_inline_always!{
    fn lossy(bytes: &[u8], radix: u32, sign: Sign)
        -> StdResult<(f64, *const u8), (ErrorCode, *const u8)>
    {
        algorithm::atod_lossy(bytes, radix, sign)
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
fn last(bytes: &[u8]) -> *const u8 {
    index!(bytes[bytes.len()..]).as_ptr()
}}

perftools_inline!{
fn parse_float<F: StringToFloat>(bytes: &[u8], radix: u32, lossy: bool, sign: Sign)
    -> StdResult<(F, *const u8), (ErrorCode, *const u8)>
{
    match lossy {
        true  => F::lossy(bytes, radix, sign),
        false => F::default(bytes, radix, sign),
    }
}}

// Parse infinity from string.
perftools_inline!{
fn parse_infinity<F: StringToFloat>(bytes: &[u8], radix: u32, lossy: bool, sign: Sign)
    -> StdResult<(F, *const u8), (ErrorCode, *const u8)>
{
    // Check long infinity first before short infinity.
    // Short infinity short-circuits, we want to parse as many characters
    // as possible.
    if is_infinity(bytes) || is_inf(bytes) {
        // Have a valid long-form or short-form infinity.
        Ok((F::INFINITY, last(bytes)))
    } else {
        // Not infinity, may be valid with a different radix.
        if cfg!(feature = "radix"){
            parse_float(bytes, radix, lossy, sign)
        } else {
            Err((ErrorCode::InvalidDigit, bytes.as_ptr()))
        }
    }
}}

// Parse NaN from string.
perftools_inline!{
fn parse_nan<F: StringToFloat>(bytes: &[u8], radix: u32, lossy: bool, sign: Sign)
    -> StdResult<(F, *const u8), (ErrorCode, *const u8)>
{
    if is_nan(bytes) {
        // Have a valid NaN.
        Ok((F::NAN, last(bytes)))
    } else {
        // Not NaN, may be valid with a different radix.
        if cfg!(feature = "radix"){
            parse_float(bytes, radix, lossy, sign)
        } else {
            Err((ErrorCode::InvalidDigit, bytes.as_ptr()))
        }
    }
}}

// ATOF/ATOD

// Standalone atof processor.
perftools_inline!{
fn atof<F: StringToFloat>(bytes: &[u8], radix: u32, lossy: bool)
    -> StdResult<(F, *const u8), (ErrorCode, *const u8)>
{
    // Filter out empty inputs.
    if bytes.is_empty() {
        return Err((ErrorCode::Empty, bytes.as_ptr()));
    }

    let (sign, bytes) = match index!(bytes[0]) {
        b'+' => (Sign::Positive, &index!(bytes[1..])),
        b'-' => (Sign::Negative, &index!(bytes[1..])),
        _    => (Sign::Positive, bytes),
    };

    // Filter out empty inputs.
    if bytes.is_empty() {
        return Err((ErrorCode::Empty, bytes.as_ptr()));
    }

    // Special case checks
    // Use predictive parsing to filter special cases. This leads to
    // dramatic performance gains.
    let (float, ptr): (F, *const u8) = match index!(bytes[0]) {
        b'i' | b'I' => parse_infinity(bytes, radix, lossy, sign),
        b'N' | b'n' => parse_nan(bytes, radix, lossy, sign),
        _           => parse_float(bytes, radix, lossy, sign),
    }?;

    // Process the sign.
    let signed_float = match sign {
        Sign::Positive => float,
        Sign::Negative => -float,
    };
    Ok((signed_float, ptr))
}}

perftools_inline!{
fn atof_lossy<F: StringToFloat>(bytes: &[u8], radix: u32)
    -> Result<(F, usize)>
{
    let index = | ptr | distance(bytes.as_ptr(), ptr);
    match atof::<F>(bytes, radix, true) {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}}

perftools_inline!{
fn atof_nonlossy<F: StringToFloat>(bytes: &[u8], radix: u32)
    -> Result<(F, usize)>
{
    let index = | ptr | distance(bytes.as_ptr(), ptr);
    match atof::<F>(bytes, radix, false) {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}}

// FROM LEXICAL
// ------------

from_lexical!(atof_nonlossy, f32);
from_lexical!(atof_nonlossy, f64);
from_lexical_lossy!(atof_lossy, f32);
from_lexical_lossy!(atof_lossy, f64);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use util::*;

    #[test]
    fn f32_decimal_test() {
        // integer test
        assert_f32_eq!(0.0, f32::from_lexical(b"0").unwrap());
        assert_f32_eq!(1.0, f32::from_lexical(b"1").unwrap());
        assert_f32_eq!(12.0, f32::from_lexical(b"12").unwrap());
        assert_f32_eq!(123.0, f32::from_lexical(b"123").unwrap());
        assert_f32_eq!(1234.0, f32::from_lexical(b"1234").unwrap());
        assert_f32_eq!(12345.0, f32::from_lexical(b"12345").unwrap());
        assert_f32_eq!(123456.0, f32::from_lexical(b"123456").unwrap());
        assert_f32_eq!(1234567.0, f32::from_lexical(b"1234567").unwrap());
        assert_f32_eq!(12345678.0, f32::from_lexical(b"12345678").unwrap());

        // No decimal but decimal point test
        assert_f64_eq!(1.0, f32::from_lexical(b"1.").unwrap());
        assert_f64_eq!(12.0, f32::from_lexical(b"12.").unwrap());
        assert_f64_eq!(1234567.0, f32::from_lexical(b"1234567.").unwrap());

        // decimal test
        assert_f32_eq!(123.1, f32::from_lexical(b"123.1").unwrap());
        assert_f32_eq!(123.12, f32::from_lexical(b"123.12").unwrap());
        assert_f32_eq!(123.123, f32::from_lexical(b"123.123").unwrap());
        assert_f32_eq!(123.1234, f32::from_lexical(b"123.1234").unwrap());
        assert_f32_eq!(123.12345, f32::from_lexical(b"123.12345").unwrap());

        // rounding test
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.1").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.12").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.123").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.1234").unwrap());
        assert_f32_eq!(123456790.0, f32::from_lexical(b"123456789.12345").unwrap());

        // exponent test
        assert_f32_eq!(123456789.12345, f32::from_lexical(b"1.2345678912345e8").unwrap());
        assert_f32_eq!(123450000.0, f32::from_lexical(b"1.2345e+8").unwrap());
        assert_f32_eq!(1.2345e+11, f32::from_lexical(b"1.2345e+11").unwrap());
        assert_f32_eq!(1.2345e+11, f32::from_lexical(b"123450000000").unwrap());
        assert_f32_eq!(1.2345e+38, f32::from_lexical(b"1.2345e+38").unwrap());
        assert_f32_eq!(1.2345e+38, f32::from_lexical(b"123450000000000000000000000000000000000").unwrap());
        assert_f32_eq!(1.2345e-8, f32::from_lexical(b"1.2345e-8").unwrap());
        assert_f32_eq!(1.2345e-8, f32::from_lexical(b"0.000000012345").unwrap());
        assert_f32_eq!(1.2345e-38, f32::from_lexical(b"1.2345e-38").unwrap());
        assert_f32_eq!(1.2345e-38, f32::from_lexical(b"0.000000000000000000000000000000000000012345").unwrap());

        assert!(f32::from_lexical(b"NaN").unwrap().is_nan());
        assert!(f32::from_lexical(b"nan").unwrap().is_nan());
        assert!(f32::from_lexical(b"NAN").unwrap().is_nan());
        assert!(f32::from_lexical(b"inf").unwrap().is_infinite());
        assert!(f32::from_lexical(b"INF").unwrap().is_infinite());
        assert!(f32::from_lexical(b"+inf").unwrap().is_infinite());
        assert!(f32::from_lexical(b"-inf").unwrap().is_infinite());

        // Check various expected failures.
        assert_eq!(Err(ErrorCode::Empty.into()), f32::from_lexical(b""));
        assert_eq!(Err((ErrorCode::EmptyExponent, 1).into()), f32::from_lexical(b"e"));
        assert_eq!(Err((ErrorCode::EmptyExponent, 1).into()), f32::from_lexical(b"E"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), f32::from_lexical(b".e1"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), f32::from_lexical(b".e-1"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), f32::from_lexical(b"e1"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), f32::from_lexical(b"e-1"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f32::from_lexical(b"+"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f32::from_lexical(b"-"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f32::from_lexical(b"5.002868148396374"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn f32_radix_test() {
        assert_f32_eq!(1234.0, f32::from_lexical_radix(b"YA", 36).unwrap());
        assert_f32_eq!(1234.0, f32::from_lexical_lossy_radix(b"YA", 36).unwrap());
    }

    #[test]
    fn f64_decimal_test() {
        // integer test
        assert_f64_eq!(0.0, f64::from_lexical(b"0").unwrap());
        assert_f64_eq!(1.0, f64::from_lexical(b"1").unwrap());
        assert_f64_eq!(12.0, f64::from_lexical(b"12").unwrap());
        assert_f64_eq!(123.0, f64::from_lexical(b"123").unwrap());
        assert_f64_eq!(1234.0, f64::from_lexical(b"1234").unwrap());
        assert_f64_eq!(12345.0, f64::from_lexical(b"12345").unwrap());
        assert_f64_eq!(123456.0, f64::from_lexical(b"123456").unwrap());
        assert_f64_eq!(1234567.0, f64::from_lexical(b"1234567").unwrap());
        assert_f64_eq!(12345678.0, f64::from_lexical(b"12345678").unwrap());

        // No decimal but decimal point test
        assert_f64_eq!(1.0, f64::from_lexical(b"1.").unwrap());
        assert_f64_eq!(12.0, f64::from_lexical(b"12.").unwrap());
        assert_f64_eq!(1234567.0, f64::from_lexical(b"1234567.").unwrap());

        // decimal test
        assert_f64_eq!(123456789.0, f64::from_lexical(b"123456789").unwrap());
        assert_f64_eq!(123456789.1, f64::from_lexical(b"123456789.1").unwrap());
        assert_f64_eq!(123456789.12, f64::from_lexical(b"123456789.12").unwrap());
        assert_f64_eq!(123456789.123, f64::from_lexical(b"123456789.123").unwrap());
        assert_f64_eq!(123456789.1234, f64::from_lexical(b"123456789.1234").unwrap());
        assert_f64_eq!(123456789.12345, f64::from_lexical(b"123456789.12345").unwrap());
        assert_f64_eq!(123456789.123456, f64::from_lexical(b"123456789.123456").unwrap());
        assert_f64_eq!(123456789.1234567, f64::from_lexical(b"123456789.1234567").unwrap());
        assert_f64_eq!(123456789.12345678, f64::from_lexical(b"123456789.12345678").unwrap());

        // rounding test
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.123456789").unwrap());
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.1234567890").unwrap());
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.123456789012").unwrap());
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.1234567890123").unwrap());
        assert_f64_eq!(123456789.12345679, f64::from_lexical(b"123456789.12345678901234").unwrap());

        // exponent test
        assert_f64_eq!(123456789.12345, f64::from_lexical(b"1.2345678912345e8").unwrap());
        assert_f64_eq!(123450000.0, f64::from_lexical(b"1.2345e+8").unwrap());
        assert_f64_eq!(1.2345e+11, f64::from_lexical(b"123450000000").unwrap());
        assert_f64_eq!(1.2345e+11, f64::from_lexical(b"1.2345e+11").unwrap());
        assert_f64_eq!(1.2345e+38, f64::from_lexical(b"1.2345e+38").unwrap());
        assert_f64_eq!(1.2345e+38, f64::from_lexical(b"123450000000000000000000000000000000000").unwrap());
        assert_f64_eq!(1.2345e+308, f64::from_lexical(b"1.2345e+308").unwrap());
        assert_f64_eq!(1.2345e+308, f64::from_lexical(b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap());
        assert_f64_eq!(0.000000012345, f64::from_lexical(b"1.2345e-8").unwrap());
        assert_f64_eq!(1.2345e-8, f64::from_lexical(b"0.000000012345").unwrap());
        assert_f64_eq!(1.2345e-38, f64::from_lexical(b"1.2345e-38").unwrap());
        assert_f64_eq!(1.2345e-38, f64::from_lexical(b"0.000000000000000000000000000000000000012345").unwrap());

        // denormalized (try extremely low values)
        assert_f64_eq!(1.2345e-308, f64::from_lexical(b"1.2345e-308").unwrap());
        // These next 3 tests fail on arm-unknown-linux-gnueabi with the
        // incorrect parser.
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(5e-322), f64::from_lexical(b"5e-322"));
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(5e-323), f64::from_lexical(b"5e-323"));
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(5e-324), f64::from_lexical(b"5e-324"));
        // due to issues in how the data is parsed, manually extracting
        // non-exponents of 1.<e-299 is prone to error
        // test the limit of our ability
        // We tend to get relative errors of 1e-16, even at super low values.
        assert_f64_eq!(1.2345e-299, f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=1e-314);

        // Keep pushing from -300 to -324
        assert_f64_eq!(1.2345e-300, f64::from_lexical(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=1e-315);

        // These next 3 tests fail on arm-unknown-linux-gnueabi with the
        // incorrect parser.
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_f64_eq!(1.2345e-310, f64::from_lexical(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_f64_eq!(1.2345e-320, f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_f64_eq!(1.2345e-321, f64::from_lexical(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=5e-324);
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_f64_eq!(1.24e-322, f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000124").unwrap(), epsilon=5e-324);
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(1e-323), f64::from_lexical(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"));
        #[cfg(all(not(feature = "correct"), not(target_arch = "arm")))]
        assert_eq!(Ok(5e-324), f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005"));

        assert!(f64::from_lexical(b"NaN").unwrap().is_nan());
        assert!(f64::from_lexical(b"nan").unwrap().is_nan());
        assert!(f64::from_lexical(b"NAN").unwrap().is_nan());
        assert!(f64::from_lexical(b"inf").unwrap().is_infinite());
        assert!(f64::from_lexical(b"INF").unwrap().is_infinite());
        assert!(f64::from_lexical(b"+inf").unwrap().is_infinite());
        assert!(f64::from_lexical(b"-inf").unwrap().is_infinite());

        // Check various expected failures.
        assert_eq!(Err(ErrorCode::Empty.into()), f64::from_lexical(b""));
        assert_eq!(Err((ErrorCode::EmptyExponent, 1).into()), f64::from_lexical(b"e"));
        assert_eq!(Err((ErrorCode::EmptyExponent, 1).into()), f64::from_lexical(b"E"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), f64::from_lexical(b".e1"));
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), f64::from_lexical(b".e-1"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), f64::from_lexical(b"e1"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), f64::from_lexical(b"e-1"));

        // Check various reports from a fuzzer.
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), f64::from_lexical(b"0e"));
        assert_eq!(Err((ErrorCode::EmptyExponent, 4).into()), f64::from_lexical(b"0.0e"));
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), f64::from_lexical(b".E"));
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), f64::from_lexical(b".e"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), f64::from_lexical(b"E2252525225"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), f64::from_lexical(b"e2252525225"));
        assert_eq!(Ok(f64::INFINITY), f64::from_lexical(b"2E200000000000"));

        // Add various unittests from proptests.
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), f64::from_lexical(b"0e"));
        assert_eq!(Err((ErrorCode::EmptyFraction, 0).into()), f64::from_lexical(b"."));
        assert_eq!(Err((ErrorCode::EmptyFraction, 1).into()), f64::from_lexical(b"+."));
        assert_eq!(Err((ErrorCode::EmptyFraction, 1).into()), f64::from_lexical(b"-."));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f64::from_lexical(b"+"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f64::from_lexical(b"-"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f64::from_lexical(b"5.002868148396374"));
    }

    #[test]
    #[should_panic]
    fn limit_test() {
        assert_relative_eq!(1.2345e-320, 0.0, epsilon=5e-324);
    }

    #[cfg(feature = "radix")]
    #[test]
    fn f64_radix_test() {
        assert_f64_eq!(1234.0, f64::from_lexical_radix(b"YA", 36).unwrap());
        assert_f64_eq!(1234.0, f64::from_lexical_lossy_radix(b"YA", 36).unwrap());
    }

    #[test]
    fn f32_lossy_decimal_test() {
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), f32::from_lexical_lossy(b"."));
        assert_eq!(Err(ErrorCode::Empty.into()), f32::from_lexical_lossy(b""));
        assert_eq!(Ok(0.0), f32::from_lexical_lossy(b"0.0"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), f32::from_lexical_lossy(b"1a"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f32::from_lexical_lossy(b"5.002868148396374"));
    }

    #[test]
    fn f64_lossy_decimal_test() {
        assert_eq!(Err(ErrorCode::EmptyFraction.into()), f64::from_lexical_lossy(b"."));
        assert_eq!(Err(ErrorCode::Empty.into()), f64::from_lexical_lossy(b""));
        assert_eq!(Ok(0.0), f64::from_lexical_lossy(b"0.0"));
        assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), f64::from_lexical_lossy(b"1a"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f64::from_lexical_lossy(b"5.002868148396374"));
    }

    #[cfg(feature = "std")]
    proptest! {
        #[test]
        fn f32_invalid_proptest(i in r"[+-]?[0-9]{2}\D?\.\D?[0-9]{2}\D?e[+-]?[0-9]+\D") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f32_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::InvalidDigit || err.code == ErrorCode::EmptyFraction);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f32_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::Empty || err.code == ErrorCode::EmptyFraction);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f32_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f32_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f32_roundtrip_display_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f32_roundtrip_debug_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{:?}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f32_roundtrip_scientific_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{:e}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[test]
        fn f64_invalid_proptest(i in r"[+-]?[0-9]{2}\D?\.\D?[0-9]{2}\D?e[+-]?[0-9]+\D") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f64_double_sign_proptest(i in r"[+-]{2}[0-9]{2}\.[0-9]{2}e[+-]?[0-9]+") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::InvalidDigit || err.code == ErrorCode::EmptyFraction);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f64_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::Empty || err.code == ErrorCode::EmptyFraction);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f64_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::InvalidDigit);
        }

        #[test]
        fn f64_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f64_roundtrip_display_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f64_roundtrip_debug_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{:?}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }

        #[cfg(feature = "correct")]
        #[test]
        fn f64_roundtrip_scientific_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{:e}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }
    }
}
