//! Stores the current state of the parsed float.

use atoi;
use util::*;
use lib::slice;
use lib::result::Result as StdResult;

cfg_if! {
if #[cfg(feature = "correct")] {
use super::alias::*;
use super::exponent::*;
}}  // cfg_if

// PARSE
// -----

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
fn is_digit(c: u8, radix: u32) -> bool {
    to_digit!(c, radix).is_some()
}}

// Consume until a non-digit character is found.
perftools_inline!{
fn consume_digits<'a>(digits: &'a [u8], radix: u32) -> (&'a [u8], &'a [u8]) {
    match digits.iter().position(|&c| !is_digit(c, radix)) {
        Some(v) => (&digits[..v], &digits[v..]),
        None    => (&digits[..], &digits[digits.len()..]),
    }
}}

// RAW FLOAT STATE
// ---------------

/// Raw substring and information from parsing the float.
#[allow(dead_code)]
#[cfg_attr(test, derive(Debug))]
pub(super) struct RawFloatState<'a> {
    /// Substring for the integer component of the mantissa.
    pub(super) integer: &'a [u8],
    /// Substring for the fraction component of the mantissa.
    pub(super) fraction: &'a [u8],
    /// Parsed exponent.
    pub(super) exponent: i32,
}

impl<'a> RawFloatState<'a> {
    /// Create new raw float state.
    perftools_inline!{
    pub(super) fn new() -> RawFloatState<'a> {
        RawFloatState {
            integer: &[],
            fraction: &[],
            exponent: 0,
        }
    }}

    // Extract the integer substring from the float.
    perftools_inline!{
    fn extract_integer(&mut self, bytes: &'a [u8], radix: u32) -> &'a [u8] {
        let (integer, trailing) = consume_digits(bytes, radix);
        self.integer = integer;
        trailing
    }}

    // Extract the fraction substring from the float.
    perftools_inline!{
    fn extract_fraction(&mut self, bytes: &'a [u8], radix: u32) -> &'a [u8] {
        let (fraction, trailing) = consume_digits(&index!(bytes[1..]), radix);
        self.fraction = fraction;
        trailing
    }}

    // Extract and parse the exponent substring from the float.
    perftools_inline!{
    fn parse_exponent(&mut self, bytes: &'a [u8], radix: u32)
        -> StdResult<&'a [u8] , (ErrorCode, *const u8)>
    {
        let (exp, first) = atoi::standalone_exponent(&index!(bytes[1..]), radix)?;
        self.exponent = exp;
        let last = index!(bytes[bytes.len()..]).as_ptr();
        Ok(unsafe { slice::from_raw_parts(first, distance(first, last)) })
    }}

    // Validate the extracted subsections.
    //      1. Validate all integer characters are digits.
    //      2. Validate all fraction characters are digits.
    //      3. Validate non-empty significant digits (integer or fraction).
    perftools_inline!{
    fn validate(&self, bytes: &'a [u8])
        -> StdResult<(), (ErrorCode, *const u8)>
    {
        // Do a simple verification of the parsed data.
        if self.integer.len().is_zero() && self.fraction.len().is_zero() {
            // Invalid floating-point number, no integer or fraction components.
            Err((ErrorCode::EmptyFraction, bytes.as_ptr()))
        } else {
            Ok(())
        }
    }}

    // Do our post-processing on the digits the create a pretty float.
    // This is required for accurate results in the slow-path algorithm,
    // otherwise, we may incorrect guess the mantissa or scientific
    // exponent.
    perftools_inline!{
    fn trim(&mut self) {
        self.integer = ltrim_0!(self.integer).0;
        self.fraction = rtrim_0!(self.fraction).0;
    }}

    // Parse the float state from raw bytes.
    perftools_inline!{
    pub(super) fn parse(&mut self, bytes: &'a [u8], radix: u32)
        -> StdResult<*const u8, (ErrorCode, *const u8)>
    {
        let mut digits = bytes;
        digits = self.extract_integer(digits, radix);
        // Parse the remaining digits, which may include a fraction,
        // an exponent, or both.
        let exp_char = exponent_notation_char(radix).to_ascii_lowercase();
        if let Some(c) = digits.first() {
            if *c == b'.' {
                // Extract the fraction, and then check for a subsequent exponent.
                digits = self.extract_fraction(digits, radix);
                if let Some(c) = digits.first() {
                    if c.to_ascii_lowercase() == exp_char {
                        digits = self.parse_exponent(digits, radix)?;
                    }
                }
            } else if c.to_ascii_lowercase() == exp_char {
                // Parse the exponent.
                digits = self.parse_exponent(digits, radix)?;
            }
        }
        self.validate(bytes)?;
        self.trim();

        Ok(digits.as_ptr())
    }}

    // Process the float state for the moderate or slow atof processor.
    perftools_inline!{
    #[cfg(feature = "correct")]
    pub(super) fn process(self, truncated: usize) -> FloatState<'a> {
        let integer = self.integer;
        let fraction = self.fraction;
        let digits_start = match integer.len() {
            0 => ltrim_char_slice(fraction, b'0').1,
            _ => 0,
        };
        FloatState { integer, fraction, digits_start, truncated, raw_exponent: self.exponent }
    }}
}

// FLOAT STATE
// -----------

/// Substrings and information from parsing the float.
#[cfg(feature = "correct")]
#[cfg_attr(test, derive(Debug))]
pub(super) struct FloatState<'a> {
    /// Substring for the integer component of the mantissa.
    pub(super) integer: &'a [u8],
    /// Substring for the fraction component of the mantissa.
    pub(super) fraction: &'a [u8],
    /// Offset to where the digits start in either integer or fraction.
    pub(super) digits_start: usize,
    /// Number of truncated digits from the mantissa.
    pub(super) truncated: usize,
    /// Raw exponent for the float.
    pub(super) raw_exponent: i32,
}

#[cfg(feature = "correct")]
impl<'a> FloatState<'a> {
    /// Get the length of the integer substring.
    perftools_inline!{
    pub(super) fn integer_len(&self) -> usize {
        self.integer.len()
    }}

    /// Get number of parsed integer digits.
    perftools_inline!{
    pub(super) fn integer_digits(&self) -> usize {
        self.integer_len()
    }}

    /// Iterate over the integer digits.
    perftools_inline!{
    pub(super) fn integer_iter(&self) -> SliceIter<u8> {
        self.integer.iter()
    }}

    /// Get the length of the fraction substring.
    perftools_inline!{
    pub(super) fn fraction_len(&self) -> usize {
        self.fraction.len()
    }}

    /// Iterate over the fraction digits.
    perftools_inline!{
    pub(super) fn fraction_digits(&self) -> usize {
        self.fraction_len() - self.digits_start
    }}

    /// Iterate over the digits, by chaining two slices.
    perftools_inline!{
    pub(super) fn fraction_iter(&self) -> SliceIter<u8> {
        // We need to rtrim the zeros in the slice fraction.
        // These are useless and just add computational complexity later,
        // just like leading zeros in the integer.
        // We need them to calculate the number of truncated bytes,
        // but we should remove them before doing anything costly.
        // In practice, we only call `mantissa_iter()` once per parse,
        // so this is effectively free.
        self.fraction[self.digits_start..].iter()
    }}

    /// Get the number of digits in the mantissa.
    /// Cannot overflow, since this is based off a single usize input string.
    perftools_inline!{
    pub(super) fn mantissa_digits(&self) -> usize {
        self.integer_digits() + self.fraction_digits()
    }}

    /// Iterate over the mantissa digits, by chaining two slices.
    perftools_inline!{
    pub(super) fn mantissa_iter(&self) -> ChainedSliceIter<u8> {
        self.integer_iter().chain(self.fraction_iter())
    }}

    /// Get number of truncated digits.
    perftools_inline!{
    pub(super) fn truncated_digits(&self) -> usize {
        self.truncated
    }}

    /// Get the mantissa exponent from the raw exponent.
    perftools_inline!{
    pub(super) fn mantissa_exponent(&self) -> i32 {
        mantissa_exponent(self.raw_exponent, self.fraction_len(), self.truncated_digits())
    }}

    /// Get the scientific exponent from the raw exponent.
    perftools_inline!{
    pub(super) fn scientific_exponent(&self) -> i32 {
        scientific_exponent(self.raw_exponent, self.integer_digits(), self.digits_start)
    }}
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    fn new_state<'a>(integer: &'a [u8], fraction: &'a [u8], exponent: i32)
        -> RawFloatState<'a>
    {
        RawFloatState { integer, fraction, exponent }
    }

    fn check_parse(digits: &str, radix: u32, expected: StdResult<RawFloatState, ErrorCode>)
    {
        let mut state = RawFloatState::new();
        match state.parse(digits.as_bytes(), radix) {
            Ok(_)       => {
                let expected = expected.unwrap();
                assert_eq!(state.integer, expected.integer);
                assert_eq!(state.fraction, expected.fraction);
                assert_eq!(state.exponent, expected.exponent);
            },
            Err((c, _))  => assert_eq!(c, expected.err().unwrap()),
        }
    }

    #[test]
    fn parse_test() {
        // Valid
        check_parse("1.2345", 10, Ok(new_state(b"1", b"2345", 0)));
        check_parse("12.345", 10, Ok(new_state(b"12", b"345", 0)));
        check_parse("12345.6789", 10, Ok(new_state(b"12345", b"6789", 0)));
        check_parse("1.2345e10", 10, Ok(new_state(b"1", b"2345", 10)));
        check_parse("1.2345e+10", 10, Ok(new_state(b"1", b"2345", 10)));
        check_parse("1.2345e-10", 10, Ok(new_state(b"1", b"2345", -10)));
        check_parse("100000000000000000000", 10, Ok(new_state(b"100000000000000000000", b"", 0)));
        check_parse("100000000000000000001", 10, Ok(new_state(b"100000000000000000001", b"", 0)));
        check_parse("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", 10, Ok(new_state(b"179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791", b"9999999999999999999999999999999999999999999999999999999999999999999999", 0)));
        check_parse("1009e-31", 10, Ok(new_state(b"1009", b"", -31)));
        check_parse("001.0", 10, Ok(new_state(b"1", b"", 0)));

        // Invalid
        check_parse("1.2345e", 10, Err(ErrorCode::EmptyExponent));
        check_parse(".", 10, Err(ErrorCode::EmptyFraction));
    }

    #[cfg(feature = "correct")]
    #[test]
    fn scientific_exponent_test() {
        // Check "1.2345", simple.
        let state = FloatState {
            integer: "1".as_bytes(),
            fraction: "2345".as_bytes(),
            digits_start: 0,
            truncated: 0,
            raw_exponent: 0,
        };
        assert_eq!(state.scientific_exponent(), 0);

        // Check "0.12345", simple.
        let state = FloatState {
            integer: "".as_bytes(),
            fraction: "12345".as_bytes(),
            digits_start: 0,
            truncated: 0,
            raw_exponent: 0,
        };
        assert_eq!(state.scientific_exponent(), -1);
    }
}
