//! Low-level API generator.
//!
//! Uses either the imprecise or the precise algorithm.

use crate::error::*;
use crate::lib::slice;
use crate::result::*;
use crate::traits::*;
use crate::util::*;

use super::algorithm::correct as algorithm;
use super::algorithm::*;

// NOTICE
//  These internal calls are all ugly, and pass **all** the values
//  as parameters to the function calls because the overhead of
//  adding them to a struct, and passing the struct by reference,
//  was adding a ~15% performance penalty to all calls, likely because
//  the compiler wasn't able to properly inline calls.
//
//  These functions are ugly as a result.

// SPECIAL
// Utilities to filter special values.

/// Convert slice to iterator without digit separators.
#[inline(always)]
fn to_iter<'a>(bytes: &'a [u8], _: u8) -> slice::Iter<'a, u8> {
    bytes.iter()
}

/// Convert slice to iterator with digit separators.
#[inline(always)]
#[cfg(feature = "format")]
fn to_iter_s<'a>(bytes: &'a [u8], digit_separator: u8) -> SkipValueIterator<'a, u8> {
    SkipValueIterator::new(bytes, digit_separator)
}

// PARSER

/// Parse infinity from string.
#[inline]
fn parse_infinity<'a, ToIter, StartsWith, Iter, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    inf_string: &'static [u8],
    infinity_string: &'static [u8],
    to_iter: ToIter,
    starts_with: StartsWith,
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    ToIter: Fn(&'a [u8], u8) -> Iter,
    Iter: AsPtrIterator<'a, u8>,
    StartsWith: Fn(Iter, slice::Iter<'a, u8>) -> (bool, Iter),
    Data: FastDataInterface<'a>,
{
    let digit_separator = data.format().digit_separator();
    if let (true, iter) = starts_with(to_iter(bytes, digit_separator), infinity_string.iter()) {
        Ok((F::INFINITY, iter.as_ptr()))
    } else if let (true, iter) = starts_with(to_iter(bytes, digit_separator), inf_string.iter()) {
        Ok((F::INFINITY, iter.as_ptr()))
    } else {
        // Not infinity, may be valid with a different radix.
        if cfg!(feature = "power_of_two") {
            algorithm::to_native::<F, Data>(data, bytes, sign, radix, incorrect, lossy, rounding)
        } else {
            Err((ErrorCode::InvalidDigit, bytes.as_ptr()))
        }
    }
}

/// Parse NaN from string.
#[inline]
fn parse_nan<'a, ToIter, StartsWith, Iter, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    to_iter: ToIter,
    starts_with: StartsWith,
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    ToIter: Fn(&'a [u8], u8) -> Iter,
    Iter: AsPtrIterator<'a, u8>,
    StartsWith: Fn(Iter, slice::Iter<'a, u8>) -> (bool, Iter),
    Data: FastDataInterface<'a>,
{
    let digit_separator = data.format().digit_separator();
    if let (true, iter) = starts_with(to_iter(bytes, digit_separator), nan_string.iter()) {
        Ok((F::NAN, iter.as_ptr()))
    } else {
        // Not NaN, may be valid with a different radix.
        if cfg!(feature = "power_of_two") {
            algorithm::to_native::<F, Data>(data, bytes, sign, radix, incorrect, lossy, rounding)
        } else {
            Err((ErrorCode::InvalidDigit, bytes.as_ptr()))
        }
    }
}

// ATOF/ATOD

/// Parse special or float values with the standard format.
/// Special values are allowed, the match is case-insensitive,
/// and no digit separators are allowed.
#[inline(always)]
fn parse_float_standard<'a, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    infinity_string: &'static [u8],
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    // Use predictive parsing to filter special cases. This leads to
    // dramatic performance gains.
    let starts_with = case_insensitive_starts_with_iter;
    match bytes[0] {
        b'i' | b'I' => parse_infinity(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            inf_string,
            infinity_string,
            to_iter,
            starts_with,
        ),
        b'N' | b'n' => parse_nan(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            nan_string,
            to_iter,
            starts_with,
        ),
        _ => algorithm::to_native::<F, Data>(data, bytes, sign, radix, incorrect, lossy, rounding),
    }
}

/// Parse special or float values.
/// Special values are allowed, the match is case-sensitive,
/// and digit separators are allowed.
#[inline]
#[cfg(feature = "format")]
fn parse_float_cs<'a, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    infinity_string: &'static [u8],
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    let digit_separator = data.format().digit_separator();
    let starts_with = starts_with_iter;
    match SkipValueIterator::new(bytes, digit_separator).next() {
        Some(&b'i') | Some(&b'I') => parse_infinity(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            inf_string,
            infinity_string,
            to_iter_s,
            starts_with,
        ),
        Some(&b'n') | Some(&b'N') => parse_nan(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            nan_string,
            to_iter_s,
            starts_with,
        ),
        _ => algorithm::to_native::<F, Data>(data, bytes, sign, radix, incorrect, lossy, rounding),
    }
}

/// Parse special or float values.
/// Special values are allowed, the match is case-sensitive,
/// and no digit separators are allowed.
#[inline]
#[cfg(feature = "format")]
fn parse_float_c<'a, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    infinity_string: &'static [u8],
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    // Use predictive parsing to filter special cases. This leads to
    // dramatic performance gains.
    let starts_with = starts_with_iter;
    match bytes[0] {
        b'i' | b'I' => parse_infinity(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            inf_string,
            infinity_string,
            to_iter,
            starts_with,
        ),
        b'N' | b'n' => parse_nan(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            nan_string,
            to_iter,
            starts_with,
        ),
        _ => algorithm::to_native::<F, Data>(data, bytes, sign, radix, incorrect, lossy, rounding),
    }
}

/// Parse special or float values.
/// Special values are allowed, the match is case-insensitive,
/// and digit separators are allowed.
#[inline]
#[cfg(feature = "format")]
fn parse_float_s<'a, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    infinity_string: &'static [u8],
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    let digit_separator = data.format().digit_separator();
    let starts_with = case_insensitive_starts_with_iter;
    match SkipValueIterator::new(bytes, digit_separator).next() {
        Some(&b'i') | Some(&b'I') => parse_infinity(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            inf_string,
            infinity_string,
            to_iter_s,
            starts_with,
        ),
        Some(&b'n') | Some(&b'N') => parse_nan(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            nan_string,
            to_iter_s,
            starts_with,
        ),
        _ => algorithm::to_native::<F, Data>(data, bytes, sign, radix, incorrect, lossy, rounding),
    }
}

/// Parse special or float values with the default formatter.
#[inline(always)]
#[cfg(not(feature = "format"))]
fn parse_float<'a, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    infinity_string: &'static [u8],
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    parse_float_standard(
        data,
        bytes,
        sign,
        radix,
        incorrect,
        lossy,
        rounding,
        nan_string,
        inf_string,
        infinity_string,
    )
}

/// Parse special or float values with the default formatter.
#[inline(always)]
#[cfg(feature = "format")]
fn parse_float<'a, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    infinity_string: &'static [u8],
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    // Need to consider 3 possibilities:
    //  1). No special values are allowed.
    //  2). Special values are case-sensitive.
    //  3). Digit separators are allowed in the special.
    let format = data.format();
    let no_special = format.no_special();
    let case = format.case_sensitive_special();
    let has_sep = format.special_digit_separator();
    match (no_special, case, has_sep) {
        (true, _, _) => {
            algorithm::to_native::<F, Data>(data, bytes, sign, radix, incorrect, lossy, rounding)
        },
        (false, true, true) => parse_float_cs(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            nan_string,
            inf_string,
            infinity_string,
        ),
        (false, false, true) => parse_float_s(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            nan_string,
            inf_string,
            infinity_string,
        ),
        (false, true, false) => parse_float_c(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            nan_string,
            inf_string,
            infinity_string,
        ),
        (false, false, false) => parse_float_standard(
            data,
            bytes,
            sign,
            radix,
            incorrect,
            lossy,
            rounding,
            nan_string,
            inf_string,
            infinity_string,
        ),
    }
}

/// Validate sign byte is valid.
#[inline(always)]
#[cfg(not(feature = "format"))]
fn validate_sign(_: &[u8], _: &[u8], _: Sign, _: NumberFormat) -> ParseResult<()> {
    Ok(())
}

/// Validate sign byte is valid.
#[inline]
#[cfg(feature = "format")]
fn validate_sign(bytes: &[u8], digits: &[u8], sign: Sign, format: NumberFormat) -> ParseResult<()> {
    let has_sign = bytes.as_ptr() != digits.as_ptr();
    if format.no_positive_mantissa_sign() && has_sign && sign == Sign::Positive {
        Err((ErrorCode::InvalidPositiveMantissaSign, bytes.as_ptr()))
    } else if format.required_mantissa_sign() && !has_sign {
        Err((ErrorCode::MissingMantissaSign, bytes.as_ptr()))
    } else {
        Ok(())
    }
}

/// Convert float to signed representation.
#[inline(always)]
fn to_signed<F: FloatType>(float: F, sign: Sign) -> F {
    match sign {
        Sign::Positive => float,
        Sign::Negative => -float,
    }
}

/// Standalone atof processor.
#[inline]
fn atof<'a, F, Data>(
    data: Data,
    bytes: &'a [u8],
    radix: u32,
    incorrect: bool,
    lossy: bool,
    rounding: RoundingKind,
    nan_string: &'static [u8],
    inf_string: &'static [u8],
    infinity_string: &'static [u8],
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    let format = data.format();
    let (sign, digits) = parse_sign::<F>(bytes, format);
    if digits.is_empty() {
        return Err((ErrorCode::Empty, digits.as_ptr()));
    }
    let (float, ptr): (F, *const u8) = parse_float(
        data,
        digits,
        sign,
        radix,
        incorrect,
        lossy,
        rounding,
        nan_string,
        inf_string,
        infinity_string,
    )?;
    validate_sign(bytes, digits, sign, format)?;

    Ok((to_signed(float, sign), ptr))
}

// Optimized atof with default options.
#[inline(always)]
fn atof_default<F: FloatType>(bytes: &[u8]) -> Result<(F, usize)> {
    let format = NumberFormat::STANDARD;
    let result = apply_standard_interface!(
        atof::<F, _>,
        format,
        bytes,
        10,
        DEFAULT_INCORRECT,
        DEFAULT_LOSSY,
        DEFAULT_ROUNDING,
        DEFAULT_NAN_STRING,
        DEFAULT_INF_STRING,
        DEFAULT_INFINITY_STRING
    );
    let index = |ptr| distance(bytes.as_ptr(), ptr);
    match result {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}

// Atof with custom options.
#[inline(always)]
fn atof_with_options<F: FloatType>(
    bytes: &[u8],
    options: &ParseFloatOptions,
) -> Result<(F, usize)> {
    let format = options.format();
    let radix = options.radix();
    let incorrect = options.incorrect();
    let lossy = options.lossy();
    let rounding = options.rounding();
    let nan = options.nan_string();
    let inf = options.inf_string();
    let infinity = options.infinity_string();
    let result = apply_interface!(
        atof::<F, _>,
        format,
        bytes,
        radix,
        incorrect,
        lossy,
        rounding,
        nan,
        inf,
        infinity
    );
    let index = |ptr| distance(bytes.as_ptr(), ptr);
    match result {
        Ok((value, ptr)) => Ok((value, index(ptr))),
        Err((code, ptr)) => Err((code, index(ptr)).into()),
    }
}

// FROM LEXICAL
// ------------

from_lexical!(atof_default, f32);
from_lexical!(atof_default, f64);

from_lexical_with_options!(atof_with_options, f32);
from_lexical_with_options!(atof_with_options, f64);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use crate::error::*;
    use crate::traits::*;
    use crate::util::*;

    use approx::assert_relative_eq;
    #[cfg(feature = "property_tests")]
    use proptest::{prop_assert, prop_assert_eq, proptest};

    #[test]
    fn special_bytes_test() {
        // Test serializing and deserializing special strings.
        assert!(f32::from_lexical(b"NaN").unwrap().is_nan());
        assert!(f32::from_lexical(b"nan").unwrap().is_nan());
        assert!(f32::from_lexical(b"NAN").unwrap().is_nan());
        assert!(f32::from_lexical(b"inf").unwrap().is_infinite());
        assert!(f32::from_lexical(b"INF").unwrap().is_infinite());
        assert!(f32::from_lexical(b"Infinity").unwrap().is_infinite());

        let options = ParseFloatOptions::builder()
            .nan_string(b"nan")
            .inf_string(b"Infinity")
            .build()
            .unwrap();

        // The error message depends on whether the radix feature is enabled.
        assert!(f32::from_lexical_with_options(b"inf", &options).is_err());
        assert!(f32::from_lexical_with_options(b"Infinity", &options).unwrap().is_infinite());
    }

    #[test]
    #[cfg(feature = "rounding")]
    fn special_rounding_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.

        // Nearest, tie-even
        let options =
            ParseFloatOptions::builder().rounding(RoundingKind::NearestTieEven).build().unwrap();
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740993", &options).unwrap(),
            -9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740995", &options).unwrap(),
            -9007199254740996.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740993", &options).unwrap(),
            9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740995", &options).unwrap(),
            9007199254740996.0
        );

        // Nearest, tie-away-zero
        let options = ParseFloatOptions::builder()
            .rounding(RoundingKind::NearestTieAwayZero)
            .build()
            .unwrap();
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740993", &options).unwrap(),
            -9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740995", &options).unwrap(),
            -9007199254740996.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740993", &options).unwrap(),
            9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740995", &options).unwrap(),
            9007199254740996.0
        );

        // Toward positive infinity
        let options = ParseFloatOptions::builder()
            .rounding(RoundingKind::TowardPositiveInfinity)
            .build()
            .unwrap();
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740993", &options).unwrap(),
            -9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740995", &options).unwrap(),
            -9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740993", &options).unwrap(),
            9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740995", &options).unwrap(),
            9007199254740996.0
        );

        // Toward negative infinity
        let options = ParseFloatOptions::builder()
            .rounding(RoundingKind::TowardNegativeInfinity)
            .build()
            .unwrap();
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740993", &options).unwrap(),
            -9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740995", &options).unwrap(),
            -9007199254740996.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740993", &options).unwrap(),
            9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740995", &options).unwrap(),
            9007199254740994.0
        );

        // Toward zero
        let options =
            ParseFloatOptions::builder().rounding(RoundingKind::TowardZero).build().unwrap();
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740993", &options).unwrap(),
            -9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"-9007199254740995", &options).unwrap(),
            -9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740993", &options).unwrap(),
            9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(b"9007199254740995", &options).unwrap(),
            9007199254740994.0
        );
    }

    #[test]
    #[cfg(all(feature = "power_of_two", feature = "rounding"))]
    fn special_rounding_binary_test() {
        // Each one of these pairs is halfway, and we can detect the
        // rounding schemes from this.

        // Nearest, tie-even
        let options = ParseFloatOptions::builder()
            .radix(2)
            .rounding(RoundingKind::NearestTieEven)
            .build()
            .unwrap();
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            -9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            -9007199254740996.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            9007199254740996.0
        );

        // Nearest, tie-away-zero
        let options = ParseFloatOptions::builder()
            .radix(2)
            .rounding(RoundingKind::NearestTieAwayZero)
            .build()
            .unwrap();
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            -9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            -9007199254740996.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            9007199254740996.0
        );

        // Toward positive infinity
        let options = ParseFloatOptions::builder()
            .radix(2)
            .rounding(RoundingKind::TowardPositiveInfinity)
            .build()
            .unwrap();
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            -9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            -9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            9007199254740996.0
        );

        // Toward negative infinity
        let options = ParseFloatOptions::builder()
            .radix(2)
            .rounding(RoundingKind::TowardNegativeInfinity)
            .build()
            .unwrap();
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            -9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            -9007199254740996.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            9007199254740994.0
        );

        // Toward zero
        let options = ParseFloatOptions::builder()
            .radix(2)
            .rounding(RoundingKind::TowardZero)
            .build()
            .unwrap();
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            -9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"-100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            -9007199254740994.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000001",
                &options
            )
            .unwrap(),
            9007199254740992.0
        );
        assert_eq!(
            f64::from_lexical_with_options(
                b"100000000000000000000000000000000000000000000000000011",
                &options
            )
            .unwrap(),
            9007199254740994.0
        );
    }

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

        // No fraction after decimal point test
        assert_f32_eq!(1.0, f32::from_lexical(b"1.").unwrap());
        assert_f32_eq!(12.0, f32::from_lexical(b"12.").unwrap());
        assert_f32_eq!(1234567.0, f32::from_lexical(b"1234567.").unwrap());

        // No integer before decimal point test
        assert_f32_eq!(0.1, f32::from_lexical(b".1").unwrap());
        assert_f32_eq!(0.12, f32::from_lexical(b".12").unwrap());
        assert_f32_eq!(0.1234567, f32::from_lexical(b".1234567").unwrap());

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
        assert_f32_eq!(
            1.2345e+38,
            f32::from_lexical(b"123450000000000000000000000000000000000").unwrap()
        );
        assert_f32_eq!(1.2345e-8, f32::from_lexical(b"1.2345e-8").unwrap());
        assert_f32_eq!(1.2345e-8, f32::from_lexical(b"0.000000012345").unwrap());
        assert_f32_eq!(1.2345e-38, f32::from_lexical(b"1.2345e-38").unwrap());
        assert_f32_eq!(
            1.2345e-38,
            f32::from_lexical(b"0.000000000000000000000000000000000000012345").unwrap()
        );

        assert!(f32::from_lexical(b"NaN").unwrap().is_nan());
        assert!(f32::from_lexical(b"nan").unwrap().is_nan());
        assert!(f32::from_lexical(b"NAN").unwrap().is_nan());
        assert!(f32::from_lexical(b"inf").unwrap().is_infinite());
        assert!(f32::from_lexical(b"INF").unwrap().is_infinite());
        assert!(f32::from_lexical(b"+inf").unwrap().is_infinite());
        assert!(f32::from_lexical(b"-inf").unwrap().is_infinite());

        // Check various expected failures.
        assert_eq!(Err(ErrorCode::Empty.into()), f32::from_lexical(b""));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f32::from_lexical(b"e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f32::from_lexical(b"E"));
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f32::from_lexical(b".e1"));
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f32::from_lexical(b".e-1"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f32::from_lexical(b"e1"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f32::from_lexical(b"e-1"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f32::from_lexical(b"+"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f32::from_lexical(b"-"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f32::from_lexical(b"5.002868148396374"));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn f32_radix_test() {
        let options = ParseFloatOptions::builder().radix(36).build().unwrap();
        assert_f32_eq!(1234.0, f32::from_lexical_with_options(b"YA", &options).unwrap());
        let options = options.rebuild().lossy(true).build().unwrap();
        assert_f32_eq!(1234.0, f32::from_lexical_with_options(b"YA", &options).unwrap());
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

        // No fraction after decimal point test
        assert_f64_eq!(1.0, f64::from_lexical(b"1.").unwrap());
        assert_f64_eq!(12.0, f64::from_lexical(b"12.").unwrap());
        assert_f64_eq!(1234567.0, f64::from_lexical(b"1234567.").unwrap());

        // No integer before decimal point test
        assert_f64_eq!(0.1, f64::from_lexical(b".1").unwrap());
        assert_f64_eq!(0.12, f64::from_lexical(b".12").unwrap());
        assert_f64_eq!(0.1234567, f64::from_lexical(b".1234567").unwrap());

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
        assert_f64_eq!(
            1.2345e+38,
            f64::from_lexical(b"123450000000000000000000000000000000000").unwrap()
        );
        assert_f64_eq!(1.2345e+308, f64::from_lexical(b"1.2345e+308").unwrap());
        assert_f64_eq!(1.2345e+308, f64::from_lexical(b"123450000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap());
        assert_f64_eq!(0.000000012345, f64::from_lexical(b"1.2345e-8").unwrap());
        assert_f64_eq!(1.2345e-8, f64::from_lexical(b"0.000000012345").unwrap());
        assert_f64_eq!(1.2345e-38, f64::from_lexical(b"1.2345e-38").unwrap());
        assert_f64_eq!(
            1.2345e-38,
            f64::from_lexical(b"0.000000000000000000000000000000000000012345").unwrap()
        );

        // denormalized (try extremely low values)
        assert_f64_eq!(1.2345e-308, f64::from_lexical(b"1.2345e-308").unwrap());
        // These next 3 tests fail on arm-unknown-linux-gnueabi with the
        // incorrect parser.
        #[cfg(not(target_arch = "arm"))]
        {
            let options = ParseFloatOptions::builder().incorrect(true).build().unwrap();
            assert_eq!(Ok(5e-322), f64::from_lexical_with_options(b"5e-322", &options));
            assert_eq!(Ok(5e-323), f64::from_lexical_with_options(b"5e-323", &options));
            assert_eq!(Ok(5e-324), f64::from_lexical_with_options(b"5e-324", &options));
        }
        // due to issues in how the data is parsed, manually extracting
        // non-exponents of 1.<e-299 is prone to error
        // test the limit of our ability
        // We tend to get relative errors of 1e-16, even at super low values.
        assert_f64_eq!(1.2345e-299, f64::from_lexical(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=1e-314);

        // Keep pushing from -300 to -324
        assert_f64_eq!(1.2345e-300, f64::from_lexical(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345").unwrap(), epsilon=1e-315);

        // These next 3 tests fail on arm-unknown-linux-gnueabi with the
        // incorrect parser.
        #[cfg(not(target_arch = "arm"))]
        {
            let options = ParseFloatOptions::builder().incorrect(true).build().unwrap();
            assert_f64_near_eq!(1.2345e-310, f64::from_lexical_with_options(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", &options).unwrap(), epsilon=5e-324);
            assert_f64_near_eq!(1.2345e-320, f64::from_lexical_with_options(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", &options).unwrap(), epsilon=5e-324);
            assert_f64_near_eq!(1.2345e-321, f64::from_lexical_with_options(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000012345", &options).unwrap(), epsilon=5e-324);
            assert_f64_near_eq!(1.24e-322, f64::from_lexical_with_options(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000124", &options).unwrap(), epsilon=5e-324);
            assert_eq!(Ok(1e-323), f64::from_lexical_with_options(b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001", &options));
            assert_eq!(Ok(5e-324), f64::from_lexical_with_options(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005", &options));
        }

        assert!(f64::from_lexical(b"NaN").unwrap().is_nan());
        assert!(f64::from_lexical(b"nan").unwrap().is_nan());
        assert!(f64::from_lexical(b"NAN").unwrap().is_nan());
        assert!(f64::from_lexical(b"inf").unwrap().is_infinite());
        assert!(f64::from_lexical(b"INF").unwrap().is_infinite());
        assert!(f64::from_lexical(b"+inf").unwrap().is_infinite());
        assert!(f64::from_lexical(b"-inf").unwrap().is_infinite());

        // Check various expected failures.
        assert_eq!(Err(ErrorCode::Empty.into()), f64::from_lexical(b""));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"E"));
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f64::from_lexical(b".e1"));
        assert_eq!(Err(ErrorCode::EmptyMantissa.into()), f64::from_lexical(b".e-1"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"e1"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"e-1"));

        // Check various reports from a fuzzer.
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), f64::from_lexical(b"0e"));
        assert_eq!(Err((ErrorCode::EmptyExponent, 4).into()), f64::from_lexical(b"0.0e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b".E"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b".e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"E2252525225"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"e2252525225"));
        assert_eq!(Ok(f64::INFINITY), f64::from_lexical(b"2E200000000000"));

        // Add various unittests from proptests.
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), f64::from_lexical(b"0e"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), f64::from_lexical(b"."));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 1).into()), f64::from_lexical(b"+."));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 1).into()), f64::from_lexical(b"-."));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f64::from_lexical(b"+"));
        assert_eq!(Err((ErrorCode::Empty, 1).into()), f64::from_lexical(b"-"));

        // Bug fix for Issue #8
        assert_eq!(Ok(5.002868148396374), f64::from_lexical(b"5.002868148396374"));
    }

    #[test]
    #[should_panic]
    fn limit_test() {
        assert_relative_eq!(1.2345e-320, 0.0, epsilon = 5e-324);
    }

    #[test]
    #[cfg(feature = "radix")]
    fn f64_radix_test() {
        let options = ParseFloatOptions::builder().radix(36).build().unwrap();
        assert_f64_eq!(1234.0, f64::from_lexical_with_options(b"YA", &options).unwrap());
        let options = options.rebuild().lossy(true).build().unwrap();
        assert_f64_eq!(1234.0, f64::from_lexical_with_options(b"YA", &options).unwrap());
    }

    #[test]
    fn f32_lossy_decimal_test() {
        let options = ParseFloatOptions::builder().lossy(true).build().unwrap();
        assert_eq!(
            Err(ErrorCode::EmptyMantissa.into()),
            f32::from_lexical_with_options(b".", &options)
        );
        assert_eq!(Err(ErrorCode::Empty.into()), f32::from_lexical_with_options(b"", &options));
        assert_eq!(Ok(0.0), f32::from_lexical_with_options(b"0.0", &options));
        assert_eq!(
            Err((ErrorCode::InvalidDigit, 1).into()),
            f32::from_lexical_with_options(b"1a", &options)
        );

        // Bug fix for Issue #8
        assert_eq!(
            Ok(5.002868148396374),
            f32::from_lexical_with_options(b"5.002868148396374", &options)
        );
    }

    #[test]
    fn f64_lossy_decimal_test() {
        let options = ParseFloatOptions::builder().lossy(true).build().unwrap();
        assert_eq!(
            Err(ErrorCode::EmptyMantissa.into()),
            f64::from_lexical_with_options(b".", &options)
        );
        assert_eq!(Err(ErrorCode::Empty.into()), f64::from_lexical_with_options(b"", &options));
        assert_eq!(Ok(0.0), f64::from_lexical_with_options(b"0.0", &options));
        assert_eq!(
            Err((ErrorCode::InvalidDigit, 1).into()),
            f64::from_lexical_with_options(b"1a", &options)
        );

        // Bug fix for Issue #8
        assert_eq!(
            Ok(5.002868148396374),
            f64::from_lexical_with_options(b"5.002868148396374", &options)
        );
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_special_test() {
        //  Comments match (no_special, case_sensitive, has_sep)
        let f1 = NumberFormat::STANDARD;
        let f2 = NumberFormat::IGNORE.rebuild().digit_separator(b'_').build().unwrap();
        let f3 = f1.rebuild().no_special(true).build().unwrap();
        let f4 = f1.rebuild().case_sensitive_special(true).build().unwrap();
        let f5 = f2.rebuild().case_sensitive_special(true).build().unwrap(); // false, true, true

        let o1 = ParseFloatOptions::builder().format(Some(f1)).build().unwrap();
        let o2 = ParseFloatOptions::builder().format(Some(f2)).build().unwrap();
        let o3 = ParseFloatOptions::builder().format(Some(f3)).build().unwrap();
        let o4 = ParseFloatOptions::builder().format(Some(f4)).build().unwrap();
        let o5 = ParseFloatOptions::builder().format(Some(f5)).build().unwrap();

        // Easy NaN
        assert!(f64::from_lexical_with_options(b"NaN", &o1).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"NaN", &o2).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"NaN", &o3).is_err());
        assert!(f64::from_lexical_with_options(b"NaN", &o4).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"NaN", &o5).unwrap().is_nan());

        // Case-sensitive NaN.
        assert!(f64::from_lexical_with_options(b"nan", &o1).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"nan", &o2).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"nan", &o3).is_err());
        assert!(f64::from_lexical_with_options(b"nan", &o4).is_err());
        assert!(f64::from_lexical_with_options(b"nan", &o5).is_err());

        // Digit-separator NaN.
        assert!(f64::from_lexical_with_options(b"N_aN", &o1).is_err());
        assert!(f64::from_lexical_with_options(b"N_aN", &o2).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"N_aN", &o3).is_err());
        assert!(f64::from_lexical_with_options(b"N_aN", &o4).is_err());
        assert!(f64::from_lexical_with_options(b"N_aN", &o5).unwrap().is_nan());

        // Digit-separator + case-sensitive NaN.
        assert!(f64::from_lexical_with_options(b"n_an", &o1).is_err());
        assert!(f64::from_lexical_with_options(b"n_an", &o2).unwrap().is_nan());
        assert!(f64::from_lexical_with_options(b"n_an", &o3).is_err());
        assert!(f64::from_lexical_with_options(b"n_an", &o4).is_err());
        assert!(f64::from_lexical_with_options(b"n_an", &o5).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_integer_digits_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().required_integer_digits(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b".0", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_fraction_digits_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().required_fraction_digits(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_digits_test() {
        let format = NumberFormat::PERMISSIVE.rebuild().required_digits(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3", &options).is_ok());
        assert!(f64::from_lexical_with_options(b".0", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_positive_mantissa_sign_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().no_positive_mantissa_sign(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_err());
        assert!(f64::from_lexical_with_options(b"-3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_mantissa_sign_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().required_mantissa_sign(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"-3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_exponent_notation_test() {
        let format = NumberFormat::PERMISSIVE.rebuild().no_exponent_notation(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3.0e-7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3e", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3e-", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_optional_exponent_test() {
        let format = NumberFormat::PERMISSIVE;
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e-7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e-", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_exponent_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().required_exponent_digits(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e-7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"+3.0e", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3.0e-", &options).is_err());
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_positive_exponent_sign_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().no_positive_exponent_sign(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0e+7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3.0e-7", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_exponent_sign_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().required_exponent_sign(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"3.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3.0e+7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.0e-7", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_exponent_without_fraction_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().no_exponent_without_fraction(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3e7", &options).is_err());

        let format = format.rebuild().required_fraction_digits(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"3.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"3.e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_no_leading_zeros_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().no_float_leading_zeros(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"1.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"0.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"01.0", &options).is_err());
        assert!(f64::from_lexical_with_options(b"10.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"010.0", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_required_exponent_notation_test() {
        let format =
            NumberFormat::PERMISSIVE.rebuild().required_exponent_notation(true).build().unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"+3.0", &options).is_err());
        assert!(f64::from_lexical_with_options(b"3.0e", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"0.e", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_integer_internal_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .integer_internal_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"3_1.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"_31.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31_.0e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_fraction_internal_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .fraction_internal_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"31.0_1e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31._01e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01_e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_exponent_internal_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .exponent_internal_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"31.01e7_1", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31.01e_71", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e71_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_integer_leading_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .integer_leading_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"3_1.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"_31.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31_.0e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_fraction_leading_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .fraction_leading_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"31.0_1e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31._01e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31.01_e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_exponent_leading_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .exponent_leading_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"31.01e7_1", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e_71", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31.01e71_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_integer_trailing_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .integer_trailing_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"3_1.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"_31.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31_.0e7", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_fraction_trailing_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .fraction_trailing_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"31.0_1e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31._01e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01_e7", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_exponent_trailing_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .exponent_trailing_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"31.01e7_1", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e_71", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e71_", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_integer_consecutive_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .integer_internal_digit_separator(true)
            .integer_consecutive_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"3__1.0e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"_31.0e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31_.0e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_fraction_consecutive_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .fraction_internal_digit_separator(true)
            .fraction_consecutive_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"31.0__1e7", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31._01e7", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01_e7", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_exponent_consecutive_digit_separator_test() {
        let format = NumberFormat::PERMISSIVE
            .rebuild()
            .exponent_internal_digit_separator(true)
            .exponent_consecutive_digit_separator(true)
            .digit_separator(b'_')
            .build()
            .unwrap();
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"31.01e7__1", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"31.01e_71", &options).is_err());
        assert!(f64::from_lexical_with_options(b"31.01e71_", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_json_exponent_without_dot() {
        // Tests courtesy of @ijl:
        //  https://github.com/Alexhuszagh/rust-lexical/issues/24#issuecomment-578153783
        let format = NumberFormat::JSON;
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        // JSONTestSuite/test_parsing/y_number_0e1.json
        assert!(f64::from_lexical_with_options(b"0e1", &options).is_ok());
        // JSONTestSuite/test_parsing/y_number_int_with_exp.json
        assert!(f64::from_lexical_with_options(b"20e1", &options).is_ok());
        // JSONTestSuite/test_parsing/y_number_real_capital_e_pos_exp.json
        assert!(f64::from_lexical_with_options(b"1E+2", &options).is_ok());
        // JSONTestSuite/test_transform/number_1e-999.json
        assert!(f64::from_lexical_with_options(b"1E-999", &options).is_ok());
        // nativejson-benchmark/data/jsonchecker/pass01.json
        assert!(f64::from_lexical_with_options(b"23456789012E66", &options).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_json_exponent_requires_digit() {
        // Tests courtesy of @ijl:
        //  https://github.com/Alexhuszagh/rust-lexical/issues/24#issuecomment-578153783
        let format = NumberFormat::JSON;
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"1e", &options).is_err());
        // JSONTestSuite/test_parsing/n_number_9.e+.json
        assert!(f64::from_lexical_with_options(b"9.e+", &options).is_err());
        // JSONTestSuite/test_parsing/n_number_2.e-3.json
        assert!(f64::from_lexical_with_options(b"2.e-3", &options).is_err());
        // JSONTestSuite/test_parsing/n_number_real_without_fractional_part.json
        assert!(f64::from_lexical_with_options(b"1.", &options).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn f64_json_no_leading_zero() {
        let format = NumberFormat::JSON;
        let options = ParseFloatOptions::builder().format(Some(format)).build().unwrap();
        assert!(f64::from_lexical_with_options(b"12.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"-12.0", &options).is_ok());
        assert!(f64::from_lexical_with_options(b"012.0", &options).is_err());
        assert!(f64::from_lexical_with_options(b"-012.0", &options).is_err());
    }

    #[cfg(feature = "property_tests")]
    proptest! {
        #[test]
        fn f32_invalid_proptest(i in r"[+-]?[0-9]{2}[^\deE]?\.[^\deE]?[0-9]{2}[^\deE]?e[+-]?[0-9]+[^\deE]") {
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
            prop_assert!(err.code == ErrorCode::InvalidDigit || err.code == ErrorCode::EmptyMantissa);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f32_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::Empty || err.code == ErrorCode::EmptyMantissa);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f32_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[test]
        fn f32_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = f32::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[test]
        fn f32_roundtrip_display_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[test]
        fn f32_roundtrip_debug_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{:?}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[test]
        fn f32_roundtrip_scientific_proptest(i in f32::MIN..f32::MAX) {
            let input: String = format!("{:e}", i);
            prop_assert_eq!(i, f32::from_lexical(input.as_bytes()).unwrap());
        }

        #[test]
        fn f64_invalid_proptest(i in r"[+-]?[0-9]{2}[^\deE]?\.[^\deE]?[0-9]{2}[^\deE]?e[+-]?[0-9]+[^\deE]") {
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
            prop_assert!(err.code == ErrorCode::InvalidDigit || err.code == ErrorCode::EmptyMantissa);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f64_sign_or_dot_only_proptest(i in r"[+-]?\.?") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert!(err.code == ErrorCode::Empty || err.code == ErrorCode::EmptyMantissa);
            prop_assert!(err.index == 0 || err.index == 1);
        }

        #[test]
        fn f64_double_exponent_sign_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]{2}[0-9]+") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[test]
        fn f64_missing_exponent_proptest(i in r"[+-]?[0-9]{2}\.[0-9]{2}e[+-]?") {
            let res = f64::from_lexical(i.as_bytes());
            prop_assert!(res.is_err());
            let err = res.err().unwrap();
            prop_assert_eq!(err.code, ErrorCode::EmptyExponent);
        }

        #[test]
        fn f64_roundtrip_display_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }

        #[test]
        fn f64_roundtrip_debug_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{:?}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }

        #[test]
        fn f64_roundtrip_scientific_proptest(i in f64::MIN..f64::MAX) {
            let input: String = format!("{:e}", i);
            prop_assert_eq!(i, f64::from_lexical(input.as_bytes()).unwrap());
        }
    }
}
