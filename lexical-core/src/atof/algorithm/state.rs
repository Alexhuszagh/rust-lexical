//! Stores the current state of the parsed float.

use atoi;
use util::*;
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

// RAW FLOAT STATE
// ---------------

/// Raw substring and information from parsing the float.
#[derive(Debug)]
pub(super) struct RawFloatState<'a> {
    /// Substring for the integer component of the mantissa.
    pub(super) integer: &'a [u8],
    /// Substring for the fraction component of the mantissa.
    pub(super) fraction: &'a [u8],
    /// Substring for the exponent component.
    pub(super) exponent: &'a [u8],
}

impl<'a> RawFloatState<'a> {
    /// Create new raw float state.
    perftools_inline!{
    pub(super) fn new() -> RawFloatState<'a> {
        RawFloatState {
            integer: &[],
            fraction: &[],
            exponent: &[],
        }
    }}

    /// Parse the float state from raw bytes.
    perftools_inline!{
    pub(super) fn parse(&mut self, radix: u32, bytes: &'a [u8]) -> Result<()>
    {
        // Initialize our needles and start our search.
        let mut digits = bytes;
        let mut exponent_found = false;
        let exp_char = exponent_notation_char(radix).to_ascii_lowercase();
        match digits.iter().position(|&c| c == b'.' || c.to_ascii_lowercase() == exp_char) {
            None    => self.integer = digits,
            Some(v) => {
                match index!(digits[v]) {
                    b'.' => {
                        // Have our fraction, now need to search for the exponent.
                        self.integer = &index!(digits[..v]);
                        digits = &index!(digits[v+1..]);
                        match digits.iter().position(|&c| c.to_ascii_lowercase() == exp_char) {
                            None    => self.fraction = digits,
                            Some(v) => {
                                // Have a fraction and exponent.
                                self.fraction = &index!(digits[..v]);
                                self.exponent = &index!(digits[v+1..]);
                                exponent_found = true;
                            }
                        }
                    },
                    _   => {
                        // No fraction, only integer and exponent.
                        self.integer = &index!(digits[..v]);
                        self.exponent = &index!(digits[v+1..]);
                        exponent_found = true;
                    }
                }
            }
        }

        // Do a simple verification of the parsed data.
        let is_plus_minus = | x: u8| x == b'+' || x == b'-';
        if self.integer.len().is_zero() && self.fraction.len().is_zero() {
            // Invalid floating-point number, no integer or fraction components.
            return Err(ErrorCode::EmptyFraction.into())
        } else if exponent_found && self.exponent.len().is_zero() {
            // Invalid exponent, exponent character found but nothing trailing it.
            return Err(ErrorCode::EmptyExponent.into());
        } else if self.exponent.len() == 1 && is_plus_minus(index!(self.exponent[0])) {
            // Invalid exponent, single character +/-.
            return Err(ErrorCode::EmptyExponent.into());
        }

        // Do our post-processing on the digits the create a pretty float.
        // This is required for accurate results in the slow-path algorithm,
        // otherwise, we may incorrect guess the mantissa or scientific
        // exponent.
        self.integer = ltrim_0!(self.integer).0;
        self.fraction = rtrim_0!(self.fraction).0;

        Ok(())
    }}

    /// Parse the raw float state into an exponent.
    perftools_inline!{
    pub(super) fn raw_exponent(&self, radix: u32)
        -> StdResult<i32, &'a u8>
    {
        match self.exponent.len() {
            // No exponent, we good here.
            0 => Ok(0),
            // Parse the exponent
            _ => atoi::standalone_exponent(radix, self.exponent),
        }
    }}

    /// Process the float state for the moderate or slow atof processor.
    perftools_inline!{
    #[cfg(feature = "correct")]
    pub(super) fn process(self, truncated: usize, raw_exponent: i32) -> FloatState<'a> {
        let integer = self.integer;
        let fraction = self.fraction;
        let digits_start = match integer.len() {
            0 => ltrim_char_slice(fraction, b'0').1,
            _ => 0,
        };
        FloatState { integer, fraction, digits_start, truncated, raw_exponent }
    }}
}

// FLOAT STATE
// -----------

/// Substrings and information from parsing the float.
#[cfg(feature = "correct")]
#[derive(Debug)]
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

    fn new_state<'a>(integer: &'a [u8], fraction: &'a [u8], exponent: &'a [u8])
        -> RawFloatState<'a>
    {
        RawFloatState { integer, fraction, exponent }
    }

    fn check_parse(radix: u32, digits: &str, expected: Result<RawFloatState>)
    {
        let mut state = RawFloatState::new();
        match state.parse(radix, digits.as_bytes()) {
            Ok(())  => {
                let expected = expected.unwrap();
                assert_eq!(state.integer, expected.integer);
                assert_eq!(state.fraction, expected.fraction);
                assert_eq!(state.exponent, expected.exponent);
            },
            Err(e) => assert_eq!(e, expected.err().unwrap()),
        }
    }

    #[test]
    fn parse_test() {
        // Valid
        check_parse(10, "1.2345", Ok(new_state(b"1", b"2345", b"")));
        check_parse(10, "12.345", Ok(new_state(b"12", b"345", b"")));
        check_parse(10, "12345.6789", Ok(new_state(b"12345", b"6789", b"")));
        check_parse(10, "1.2345e10", Ok(new_state(b"1", b"2345", b"10")));
        check_parse(10, "1.2345e+10", Ok(new_state(b"1", b"2345", b"+10")));
        check_parse(10, "1.2345e-10", Ok(new_state(b"1", b"2345", b"-10")));
        check_parse(10, "100000000000000000000", Ok(new_state(b"100000000000000000000", b"", b"")));
        check_parse(10, "100000000000000000001", Ok(new_state(b"100000000000000000001", b"", b"")));
        check_parse(10, "179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", Ok(new_state(b"179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791", b"9999999999999999999999999999999999999999999999999999999999999999999999", b"")));
        check_parse(10, "1009e-31", Ok(new_state(b"1009", b"", b"-31")));
        check_parse(10, "001.0", Ok(new_state(b"1", b"", b"")));

        // Invalid
        check_parse(10, "1.2345e", Err(ErrorCode::EmptyExponent.into()));
        check_parse(10, ".", Err(ErrorCode::EmptyFraction.into()));
    }

    #[test]
    fn raw_exponent_test() {
        assert_eq!(Ok(0), new_state(b"1", b"2345", b"").raw_exponent(10));
        assert_eq!(Ok(0), new_state(b"1", b"2345", b"0").raw_exponent(10));
        assert_eq!(Ok(0), new_state(b"1", b"2345", b"+0").raw_exponent(10));
        assert_eq!(Ok(0), new_state(b"1", b"2345", b"-0").raw_exponent(10));
        assert_eq!(Ok(5), new_state(b"1", b"2345", b"5").raw_exponent(10));
        assert_eq!(Ok(123), new_state(b"1", b"2345", b"+123").raw_exponent(10));
        assert_eq!(Ok(i32::max_value()), new_state(b"1", b"2345", b"4294967296").raw_exponent(10));
        assert_eq!(Ok(i32::max_value()), new_state(b"1", b"2345", b"+4294967296").raw_exponent(10));
        assert_eq!(Ok(i32::min_value()), new_state(b"1", b"2345", b"-4294967296").raw_exponent(10));
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
