//! Shared trait to implement a format parser.

use crate::util::*;
use crate::atof::algorithm::state::FloatState1;
use super::result::*;

#[cfg(test)]
use crate::lib::result::Result as StdResult;

/// Type definition for a test result when testing the parsing.
#[cfg(test)]
pub(crate) type TestResult<'a> = StdResult<FloatState1<'a>, ErrorCode>;

/// Generic trait to implement a specialized float parser.
pub(crate) trait FormatParser {
    // UNIMPLEMENTED

    // Consume until a non-digit character is found.
    fn consume_digits<'a>(
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    ) -> (&'a [u8], &'a [u8]);

    // Extract and parse the exponent substring from the float.
    //
    //  Preconditions:
    //      `bytes.len()` >= 1 and `bytes[0]` is an exponent signifier.
    fn parse_exponent<'a>(
        state: &mut FloatState1<'a>,
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    ) -> ParseResult<&'a [u8]>;

    // Validate the extracted mantissa components.
    fn validate_mantissa(
        state: &FloatState1,
        digit_separator: u8
    ) -> ParseResult<()>;

    // Validate the extracted exponent component.
    fn validate_exponent(
        state: &FloatState1,
        digit_separator: u8
    ) -> ParseResult<()>;

    // Trim leading 0s and digit separators.
    fn ltrim<'a>(bytes: &'a [u8], digit_separator: u8) -> (&'a [u8], usize);

    // Trim trailing 0s and digit separators.
    fn rtrim<'a>(bytes: &'a [u8], digit_separator: u8) -> (&'a [u8], usize);

    // IMPLEMENTED

    // Extract the integer substring from the float.
    perftools_inline!{
    fn extract_integer<'a>(
        state: &mut FloatState1<'a>,
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    )
        -> &'a [u8]
    {
        let result = Self::consume_digits(bytes, radix, digit_separator);
        state.integer = result.0;
        result.1
    }}

    // Extract the fraction substring from the float.
    //
    //  Preconditions:
    //      `bytes.len()` >= 1 and `bytes[0] == b'.'`.
    perftools_inline!{
    fn extract_fraction<'a>(
        state: &mut FloatState1<'a>,
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    )
        -> &'a [u8]
    {
        let digits = &index!(bytes[1..]);
        let result = Self::consume_digits(digits, radix, digit_separator);
        state.fraction = result.0;
        result.1
    }}

    // Post-process float to trim leading and trailing 0s and digit separators.
    // This is required for accurate results in the slow-path algorithm,
    // otherwise, we may incorrect guess the mantissa or scientific exponent.
    perftools_inline!{
    fn trim(state: &mut FloatState1, digit_separator: u8) {
        state.integer = Self::ltrim(state.integer, digit_separator).0;
        state.fraction = Self::rtrim(state.fraction, digit_separator).0;
    }}

    // Parse the float state from raw bytes.
    perftools_inline!{
    fn parse<'a>(
        state: &mut FloatState1<'a>,
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    )
        -> ParseResult<*const u8>
    {
        // Parse the integer, aka, the digits preceding any control characters.
        let mut digits = bytes;
        digits = Self::extract_integer(state, digits, radix, digit_separator);

        // Parse and validate a fraction, if present..
        let exp_char = exponent_notation_char(radix).to_ascii_lowercase();
        if let Some(&b'.') = digits.first() {
            digits = Self::extract_fraction(state, digits, radix, digit_separator);
        }
        Self::validate_mantissa(state, digit_separator)?;

        // Parse and validate an exponent, if present.
        if let Some(&c) = digits.first() {
            if c.to_ascii_lowercase() == exp_char {
                digits = Self::parse_exponent(state, digits, radix, digit_separator)?;
            }
        }
        Self::validate_exponent(state, digit_separator)?;

        // Trim the remaining digits.
        Self::trim(state, digit_separator);

        Ok(digits.as_ptr())
    }}

    // Calculate the digit start from the integer and fraction slices.
    perftools_inline!{
    #[cfg(feature = "correct")]
    fn digits_start<'a>(integer: &'a [u8], fraction: &'a [u8], digit_separator: u8) -> usize {
        match integer.len() {
            0 => Self::ltrim(fraction, digit_separator).1,
            _ => 0,
        }
    }}

    // TESTS

    /// Check the float state parses the desired data.
    #[cfg(test)]
    fn check_parse(
        digits: &[u8],
        radix: u32,
        digit_separator: u8,
        expected: &TestResult
    )
    {
        let mut state = FloatState1::new();
        let expected = expected.as_ref();
        match Self::parse(&mut state, digits, radix, digit_separator) {
            Ok(_)       => {
                let expected = expected.unwrap();
                assert_eq!(state.integer, expected.integer);
                assert_eq!(state.fraction, expected.fraction);
                assert_eq!(state.exponent, expected.exponent);
            },
            Err((c, _))  => assert_eq!(c, *expected.err().unwrap()),
        }
    }

    // Run series of tests.
    #[cfg(test)]
    fn run_tests<'a, Iter>(tests: Iter)
        where Iter: Iterator<Item=(&'a str, u8, &'a TestResult<'a>)>
    {
        for value in tests {
            Self::check_parse(value.0.as_bytes(), 10, value.1, &value.2);
        }
    }
}

/// Generic trait to implement a specialized float iterator.
pub(crate) trait FormatIterator<'a> {
    /// Integer iterator type.
    type IntegerIter: Iterator<Item=&'a u8>;
    type FractionIter: Iterator<Item=&'a u8>;

    /// Iterate over the integer digits.
    /// This should ignore any digit separator characters, since the
    /// input has previously been validated.
    fn integer_iter(
        integer: &'a [u8],
        digit_separator: u8
    ) -> Self::IntegerIter;

    /// Iterate over the fraction digits.
    /// This should ignore any digit separator characters, since the
    /// input has previously been validated.
    fn fraction_iter(
        integer: &'a [u8],
        digit_separator: u8
    ) -> Self::FractionIter;

    // TODO(ahuszagh) Implement...
}
