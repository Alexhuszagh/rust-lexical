//! Specialized float parsers for different formats.

use crate::util::*;
use crate::atof::algorithm::state::RawFloatState;
use crate::lib::result::Result as StdResult;

// Left-trim leading 0s.
macro_rules! ltrim_0 {
    ($bytes:expr) => { ltrim_char_slice($bytes, b'0') };
}

// Right-trim leading 0s.
macro_rules! rtrim_0 {
    ($bytes:expr) => { rtrim_char_slice($bytes, b'0') };
}

// Convert radix to value.
macro_rules! to_digit {
    ($c:expr, $radix:expr) => (($c as char).to_digit($radix));
}

// Convert character to digit.
perftools_inline_always!{
#[allow(unused_variables)]
pub(super) fn is_digit(c: u8, radix: u32) -> bool {
    to_digit!(c, radix).is_some()
}}

/// Implements a specialized float parser.
pub(in crate::atof::algorithm) trait FormatParser {
    // Consume until a non-digit character is found.
    fn consume_digits<'a>(
        digits: &'a [u8],
        radix: u32,
        digit_separator: u8
    ) -> (&'a [u8], &'a [u8]);

    // Extract the integer substring from the float.
    perftools_inline!{
    fn extract_integer<'a>(
        state: &mut RawFloatState<'a>,
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    )
        -> &'a [u8]
    {
        let (integer, trailing) = Self::consume_digits(bytes, radix, digit_separator);
        state.integer = integer;
        trailing
    }}

    // Extract the fraction substring from the float.
    perftools_inline!{
    fn extract_fraction<'a>(
        state: &mut RawFloatState<'a>,
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    )
        -> &'a [u8]
    {
        let (fraction, trailing) = Self::consume_digits(&index!(bytes[1..]), radix, digit_separator);
        state.fraction = fraction;
        trailing
    }}

    // Extract and parse the exponent substring from the float.
    fn parse_exponent<'a>(
        state: &mut RawFloatState<'a>,
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    ) -> StdResult<&'a [u8] , (ErrorCode, *const u8)>;

    // Validate the extracted float components.
    fn validate(
        state: &RawFloatState,
        bytes: &[u8],
        digit_separator: u8
    ) -> StdResult<(), (ErrorCode, *const u8)>;

    // Post-process float to trim leading and trailing 0s and digit separators.
    // This is required for accurate results in the slow-path algorithm,
    // otherwise, we may incorrect guess the mantissa or scientific
    // exponent.
    fn trim(state: &mut RawFloatState, digit_separator: u8);

    // Parse the float state from raw bytes.
    perftools_inline!{
    fn parse<'a>(
        state: &mut RawFloatState<'a>,
        bytes: &'a [u8],
        radix: u32,
        digit_separator: u8
    )
        -> StdResult<*const u8, (ErrorCode, *const u8)>
    {
        let mut digits = bytes;
        digits = Self::extract_integer(state, digits, radix, digit_separator);
        // Parse the remaining digits, which may include a fraction,
        // an exponent, or both.
        let exp_char = exponent_notation_char(radix).to_ascii_lowercase();
        if let Some(c) = digits.first() {
            if *c == b'.' {
                // Extract the fraction, and then check for a subsequent exponent.
                digits = Self::extract_fraction(state, digits, radix, digit_separator);
                if let Some(c) = digits.first() {
                    if c.to_ascii_lowercase() == exp_char {
                        digits = Self::parse_exponent(state, digits, radix, digit_separator)?;
                    }
                }
            } else if c.to_ascii_lowercase() == exp_char {
                // Parse the exponent.
                digits = Self::parse_exponent(state, digits, radix, digit_separator)?;
            }
        }
        Self::validate(state, bytes, digit_separator)?;
        Self::trim(state, digit_separator);

        Ok(digits.as_ptr())
    }}
}
