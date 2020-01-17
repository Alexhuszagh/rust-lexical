//! Stores the current state of the parsed float.

use crate::util::*;
use crate::lib::result::Result as StdResult;
use super::format::*;

cfg_if! {
if #[cfg(feature = "correct")] {
use super::alias::*;
use super::exponent::*;
}}  // cfg_if

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
    /// Substring for the exponent component.
    pub(super) exponent: &'a [u8],
    /// Parsed raw exponent.
    pub(super) raw_exponent: i32,
}

impl<'a> RawFloatState<'a> {
    /// Create new raw float state.
    perftools_inline!{
    pub(super) fn new() -> RawFloatState<'a> {
        RawFloatState {
            integer: &[],
            fraction: &[],
            exponent: &[],
            raw_exponent: 0,
        }
    }}

    // Parse the float state from raw bytes.
    perftools_inline!{
    pub(super) fn parse(&mut self, bytes: &'a [u8], radix: u32)
        -> StdResult<*const u8, (ErrorCode, *const u8)>
    {
        // TODO(ahuszagh) Change depending on the format.
        Standard::parse(self, bytes, radix, b'\x00')
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
        FloatState { integer, fraction, digits_start, truncated, raw_exponent: self.raw_exponent }
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

    fn new_state<'a>(integer: &'a [u8], fraction: &'a [u8], exponent: &'a [u8], raw_exponent: i32)
        -> RawFloatState<'a>
    {
        RawFloatState { integer, fraction, exponent, raw_exponent }
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
        check_parse("1.2345", 10, Ok(new_state(b"1", b"2345", b"", 0)));
        check_parse("12.345", 10, Ok(new_state(b"12", b"345", b"", 0)));
        check_parse("12345.6789", 10, Ok(new_state(b"12345", b"6789", b"", 0)));
        check_parse("1.2345e10", 10, Ok(new_state(b"1", b"2345", b"10", 10)));
        check_parse("1.2345e+10", 10, Ok(new_state(b"1", b"2345", b"+10", 10)));
        check_parse("1.2345e-10", 10, Ok(new_state(b"1", b"2345", b"-10", -10)));
        check_parse("100000000000000000000", 10, Ok(new_state(b"100000000000000000000", b"", b"", 0)));
        check_parse("100000000000000000001", 10, Ok(new_state(b"100000000000000000001", b"", b"", 0)));
        check_parse("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", 10, Ok(new_state(b"179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791", b"9999999999999999999999999999999999999999999999999999999999999999999999", b"", 0)));
        check_parse("1009e-31", 10, Ok(new_state(b"1009", b"", b"-31", -31)));
        check_parse("001.0", 10, Ok(new_state(b"1", b"", b"", 0)));

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
