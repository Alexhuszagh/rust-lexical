//! Stores the current state of the parsed float.

use crate::util::*;
use super::format::*;

cfg_if! {
if #[cfg(feature = "correct")] {
use super::alias::*;
use super::exponent::*;
}}  // cfg_if

// FLOAT STATE 1
// -------------

/// Step 1 of the float state.
#[allow(dead_code)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct FloatState1<'a> {
    /// Substring for the integer component of the mantissa.
    pub(crate) integer: &'a [u8],
    /// Substring for the fraction component of the mantissa.
    pub(crate) fraction: &'a [u8],
    /// Substring for the exponent component.
    pub(crate) exponent: &'a [u8],
    /// Parsed raw exponent.
    pub(crate) raw_exponent: i32,
}

impl<'a> FloatState1<'a> {
    /// Create new float state.
    perftools_inline!{
    pub(crate) fn new() -> FloatState1<'a> {
        FloatState1 {
            integer: &[],
            fraction: &[],
            exponent: &[],
            raw_exponent: 0,
        }
    }}

    // Parse the float state from raw bytes.
    perftools_inline!{
    pub(crate) fn parse(&mut self, bytes: &'a [u8], radix: u32)
        -> ParseResult<*const u8>
    {
        // TODO(ahuszagh) Change depending on the format.
        Standard::parse(self, bytes, radix, b'\x00')
    }}

    // Process the float state for the moderate or slow atof processor.
    perftools_inline!{
    #[cfg(feature = "correct")]
    pub(crate) fn process(self, truncated: usize) -> FloatState2<'a> {
        let digits_start = match self.integer.len() {
            0 => ltrim_char_slice(self.fraction, b'0').1,
            _ => 0,
        };

        (self.integer, self.fraction, digits_start, truncated, self.raw_exponent).into()
    }}
}

type FloatState1Tuple<'a> = (&'a [u8], &'a [u8], &'a [u8], i32);

impl<'a> From<FloatState1Tuple<'a>> for FloatState1<'a> {
    perftools_inline!{
    fn from(data: FloatState1Tuple<'a>) -> Self {
        FloatState1 {
            integer: data.0,
            fraction: data.1,
            exponent: data.2,
            raw_exponent: data.3
        }
    }}
}

// FLOAT STATE 2
// -------------

/// Step 2 of the float state.
#[cfg(feature = "correct")]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct FloatState2<'a> {
    /// Substring for the integer component of the mantissa.
    pub(crate) integer: &'a [u8],
    /// Substring for the fraction component of the mantissa.
    pub(crate) fraction: &'a [u8],
    /// Offset to where the digits start in either integer or fraction.
    pub(crate) digits_start: usize,
    /// Number of truncated digits from the mantissa.
    pub(crate) truncated: usize,
    /// Raw exponent for the float.
    pub(crate) raw_exponent: i32,
}

#[cfg(feature = "correct")]
impl<'a> FloatState2<'a> {
    /// Get the length of the integer substring.
    perftools_inline!{
    pub(crate) fn integer_len(&self) -> usize {
        // TODO(ahuszagh) This is going to need to consider formats.
        self.integer.len()
    }}

    /// Get number of parsed integer digits.
    perftools_inline!{
    pub(crate) fn integer_digits(&self) -> usize {
        self.integer_len()
    }}

    /// Iterate over the integer digits.
    perftools_inline!{
    pub(crate) fn integer_iter(&self) -> SliceIter<u8> {
        // TODO(ahuszagh) This is going to need to consider formats.
        self.integer.iter()
    }}

    /// Get the length of the fraction substring.
    perftools_inline!{
    pub(crate) fn fraction_len(&self) -> usize {
        // TODO(ahuszagh) This is going to need to consider formats.
        self.fraction.len()
    }}

    /// Iterate over the fraction digits.
    perftools_inline!{
    pub(crate) fn fraction_digits(&self) -> usize {
        // TODO(ahuszagh) This is going to need to consider formats.
        self.fraction_len() - self.digits_start
    }}

    /// Iterate over the digits, by chaining two slices.
    perftools_inline!{
    pub(crate) fn fraction_iter(&self) -> SliceIter<u8> {
        // TODO(ahuszagh) This is going to need to consider formats.
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
    pub(crate) fn mantissa_digits(&self) -> usize {
        self.integer_digits() + self.fraction_digits()
    }}

    /// Iterate over the mantissa digits, by chaining two slices.
    perftools_inline!{
    pub(crate) fn mantissa_iter(&self) -> ChainedSliceIter<u8> {
        self.integer_iter().chain(self.fraction_iter())
    }}

    /// Get number of truncated digits.
    perftools_inline!{
    pub(crate) fn truncated_digits(&self) -> usize {
        self.truncated
    }}

    /// Get the mantissa exponent from the raw exponent.
    perftools_inline!{
    pub(crate) fn mantissa_exponent(&self) -> i32 {
        mantissa_exponent(self.raw_exponent, self.fraction_len(), self.truncated_digits())
    }}

    /// Get the scientific exponent from the raw exponent.
    perftools_inline!{
    pub(crate) fn scientific_exponent(&self) -> i32 {
        scientific_exponent(self.raw_exponent, self.integer_digits(), self.digits_start)
    }}
}

type FloatState2Tuple<'a> = (&'a [u8], &'a [u8], usize, usize, i32);

impl<'a> From<FloatState2Tuple<'a>> for FloatState2<'a> {
    perftools_inline!{
    fn from(data: FloatState2Tuple<'a>) -> Self {
        FloatState2 {
            integer: data.0,
            fraction: data.1,
            digits_start: data.2,
            truncated: data.3,
            raw_exponent: data.4
        }
    }}
}

// TESTS
// -----

#[cfg(all(test, feature = "correct"))]
mod tests {
    use super::*;

    #[test]
    fn float_state_test() {
        // Check "1.2345", simple.
        let state: FloatState2 = (b!("1"), b!("2345"), 0, 0, 0).into();
        assert_eq!(state.integer_len(), 1);
        assert_eq!(state.integer_digits(), 1);
        assert!(state.integer_iter().eq(b"1".iter()));
        assert_eq!(state.fraction_len(), 4);
        assert_eq!(state.fraction_digits(), 4);
        assert!(state.fraction_iter().eq(b"2345".iter()));
        assert_eq!(state.mantissa_exponent(), -4);
        assert_eq!(state.scientific_exponent(), 0);
        assert_eq!(state.mantissa_digits(), 5);
        assert!(state.mantissa_iter().eq(b"12345".iter()));
        assert_eq!(state.truncated_digits(), 0);

        // Check "0.12345", simple.
        let state: FloatState2 = (b!(""), b!("12345"), 0, 0, 0).into();
        assert_eq!(state.integer_len(), 0);
        assert_eq!(state.integer_digits(), 0);
        assert!(state.integer_iter().eq(b"".iter()));
        assert_eq!(state.fraction_len(), 5);
        assert_eq!(state.fraction_digits(), 5);
        assert!(state.fraction_iter().eq(b"12345".iter()));
        assert_eq!(state.mantissa_exponent(), -5);
        assert_eq!(state.scientific_exponent(), -1);
        assert_eq!(state.mantissa_digits(), 5);
        assert!(state.mantissa_iter().eq(b"12345".iter()));
        assert_eq!(state.truncated_digits(), 0);
    }
}
