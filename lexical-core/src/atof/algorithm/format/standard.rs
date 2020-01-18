//! Standard float-parsing data.

use super::consume::*;
use super::exponent::*;
use super::iterator::*;
use super::result::*;
use super::traits::*;
use super::trim::*;
use super::validate::*;

/// Standard data interface for fast float parsers.
///
/// Guaranteed to parse `FloatFormat::RUST_STRING`, and
/// therefore will track that exact specification.
///
/// The requirements:
///     1). Must contain significant digits.
///     2). Must contain exponent digits if an exponent is present.
///     3). Does not contain any digit separators.
pub(crate) struct StandardFastDataInterface<'a> {
    integer: &'a [u8],
    fraction: &'a [u8],
    exponent: &'a [u8],
    raw_exponent: i32
}

type DataTuple<'a> = (&'a [u8], &'a [u8], &'a [u8], i32);

// Add `From` to remove repition in unit-testing.
impl<'a> From<DataTuple<'a>> for StandardFastDataInterface<'a> {
    perftools_inline!{
    fn from(data: DataTuple<'a>) -> Self {
        StandardFastDataInterface {
            integer: data.0,
            fraction: data.1,
            exponent: data.2,
            raw_exponent: data.3
        }
    }}
}

fast_data_interface_impl!(StandardFastDataInterface);

impl<'a> FastDataInterface<'a> for StandardFastDataInterface<'a> {
    type IntegerIter = IteratorNoSeparator<'a>;
    type FractionIter = IteratorNoSeparator<'a>;

    #[cfg(feature = "correct")]
    type SlowInterface = StandardSlowDataInterface<'a>;

    perftools_inline!{
    fn new(_: u32) -> Self {
        Self {
            integer: &[],
            fraction: &[],
            exponent: &[],
            raw_exponent: 0
        }
    }}

    // DATA

    perftools_inline!{
    fn integer_iter(&self) -> Self::IntegerIter {
        iterate_no_separator(self.integer)
    }}

    perftools_inline!{
    fn fraction_iter(&self) -> Self::IntegerIter {
        iterate_no_separator(self.fraction)
    }}

    // EXTRACT

    perftools_inline!{
    fn consume_digits(&self, digits: &'a [u8], radix: u32)
        -> (&'a [u8], &'a [u8])
    {
        consume_digits_no_separator(digits, radix)
    }}

    perftools_inline!{
    fn extract_exponent(&mut self, bytes: &'a [u8], radix: u32) -> &'a [u8]
    {
        extract_required_exponent_no_separator(self, bytes, radix)
    }}

    perftools_inline!{
    fn validate_mantissa(&self) -> ParseResult<()> {
        validate_mantissa_no_separator(self)
    }}

    perftools_inline!{
    fn validate_exponent(&self) -> ParseResult<()> {
        validate_required_exponent_no_separator(self)
    }}

    perftools_inline!{
    fn ltrim_zero(&self, bytes: &'a [u8]) -> (&'a [u8], usize) {
        ltrim_zero_no_separator(bytes)
    }}

    perftools_inline!{
    fn ltrim_separator(&self, bytes: &'a [u8]) -> (&'a [u8], usize) {
        ltrim_separator_no_separator(bytes)
    }}

    perftools_inline!{
    fn rtrim_zero(&self, bytes: &'a [u8]) -> (&'a [u8], usize) {
        rtrim_zero_no_separator(bytes)
    }}

    perftools_inline!{
    fn rtrim_separator(&self, bytes: &'a [u8]) -> (&'a [u8], usize) {
        rtrim_separator_no_separator(bytes)
    }}

    // TO SLOW DATA

    #[cfg(feature = "correct")]
    perftools_inline!{
    fn to_slow(self, truncated_digits: usize) -> Self::SlowInterface {
        let digits_start = self.digits_start();
        Self::SlowInterface {
            digits_start,
            truncated_digits,
            integer: self.integer,
            fraction: self.fraction,
            raw_exponent: self.raw_exponent
        }
    }}
}

/// Standard data interface for moderate/slow float parsers.
///
/// Guaranteed to parse `FloatFormat::RUST_STRING`, and
/// therefore will track that exact specification.
///
/// The requirements:
///     1). Must contain significant digits.
///     2). Must contain exponent digits if an exponent is present.
///     3). Does not contain any digit separators.
#[cfg(feature = "correct")]
pub(crate) struct StandardSlowDataInterface<'a> {
    integer: &'a [u8],
    fraction: &'a [u8],
    digits_start: usize,
    truncated_digits: usize,
    raw_exponent: i32
}

#[cfg(feature = "correct")]
slow_data_interface_impl!(StandardSlowDataInterface);

#[cfg(feature = "correct")]
impl<'a> SlowDataInterface<'a> for StandardSlowDataInterface<'a> {
    type IntegerIter = IteratorNoSeparator<'a>;
    type FractionIter = IteratorNoSeparator<'a>;

    perftools_inline!{
    fn integer_iter(&self) -> Self::IntegerIter {
        iterate_no_separator(self.integer)
    }}

    perftools_inline!{
    fn fraction_iter(&self) -> Self::IntegerIter {
        iterate_no_separator(self.fraction)
    }}

    perftools_inline!{
    fn significant_fraction_iter(&self) -> Self::IntegerIter {
        let fraction = &index!(self.fraction[self.digits_start..]);
        iterate_no_separator(fraction)
    }}

    perftools_inline!{
    fn digits_start(&self) -> usize {
        self.digits_start
    }}

    perftools_inline!{
    fn truncated_digits(&self) -> usize {
        self.truncated_digits
    }}
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn extract_test() {
        StandardFastDataInterface::new(0).run_tests([
            // Valid
            ("1.2345", Ok((b!("1"), b!("2345"), b!(""), 0).into())),
            ("12.345", Ok((b!("12"), b!("345"), b!(""), 0).into())),
            ("12345.6789", Ok((b!("12345"), b!("6789"), b!(""), 0).into())),
            ("1.2345e10", Ok((b!("1"), b!("2345"), b!("e10"), 10).into())),
            ("1.2345e+10", Ok((b!("1"), b!("2345"), b!("e+10"), 10).into())),
            ("1.2345e-10", Ok((b!("1"), b!("2345"), b!("e-10"), -10).into())),
            ("100000000000000000000", Ok((b!("100000000000000000000"), b!(""), b!(""), 0).into())),
            ("100000000000000000001", Ok((b!("100000000000000000001"), b!(""), b!(""), 0).into())),
            ("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", Ok((b!("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791"), b!("9999999999999999999999999999999999999999999999999999999999999999999999"), b!(""), 0).into())),
            ("1009e-31", Ok((b!("1009"), b!(""), b!("e-31"), -31).into())),
            ("001.0", Ok((b!("1"), b!(""), b!(""), 0).into())),
            ("1.", Ok((b!("1"), b!(""), b!(""), 0).into())),
            ("12.", Ok((b!("12"), b!(""), b!(""), 0).into())),
            ("1234567.", Ok((b!("1234567"), b!(""), b!(""), 0).into())),
            (".1", Ok((b!(""), b!("1"), b!(""), 0).into())),
            (".12", Ok((b!(""), b!("12"), b!(""), 0).into())),
            (".1234567", Ok((b!(""), b!("1234567"), b!(""), 0).into())),

            // Invalid
            ("1.2345e", Err(ErrorCode::EmptyExponent)),
            ("", Err(ErrorCode::EmptyFraction)),
            ("+", Err(ErrorCode::EmptyFraction)),
            ("-", Err(ErrorCode::EmptyFraction)),
            (".", Err(ErrorCode::EmptyFraction)),
            ("+.", Err(ErrorCode::EmptyFraction)),
            ("-.", Err(ErrorCode::EmptyFraction)),
            ("e", Err(ErrorCode::EmptyFraction)),
            ("E", Err(ErrorCode::EmptyFraction)),
            ("e1", Err(ErrorCode::EmptyFraction)),
            ("e+1", Err(ErrorCode::EmptyFraction)),
            ("e-1", Err(ErrorCode::EmptyFraction)),
            (".e", Err(ErrorCode::EmptyFraction)),
            (".E", Err(ErrorCode::EmptyFraction)),
            (".e1", Err(ErrorCode::EmptyFraction)),
            (".e+1", Err(ErrorCode::EmptyFraction)),
            (".e-1", Err(ErrorCode::EmptyFraction)),
            (".3e", Err(ErrorCode::EmptyExponent))
        ].iter());
    }

    #[test]
    fn fast_data_interface_test() {
        type Data<'a> = StandardFastDataInterface<'a>;

        // Check "1.2345".
        let data = Data {
            integer: b"1",
            fraction: b"2345",
            exponent: b"",
            raw_exponent: 0
        };
        assert!(data.integer_iter().eq(b"1".iter()));
        assert!(data.fraction_iter().eq(b"2345".iter()));

        #[cfg(feature = "correct")]
        assert_eq!(data.digits_start(), 0);
    }

    #[cfg(feature = "correct")]
    #[test]
    fn slow_data_interface_test() {
        type Data<'a> = StandardSlowDataInterface<'a>;
        // Check "1.2345", simple.
        let data = Data {
            integer: b"1",
            fraction: b"2345",
            digits_start: 0,
            truncated_digits: 0,
            raw_exponent: 0
        };
        assert_eq!(data.integer_digits(), 1);
        assert!(data.integer_iter().eq(b"1".iter()));
        assert_eq!(data.fraction_digits(), 4);
        assert!(data.fraction_iter().eq(b"2345".iter()));
        assert_eq!(data.significant_fraction_digits(), 4);
        assert!(data.significant_fraction_iter().eq(b"2345".iter()));
        assert_eq!(data.mantissa_digits(), 5);
        assert_eq!(data.digits_start(), 0);
        assert_eq!(data.truncated_digits(), 0);
        assert_eq!(data.raw_exponent(), 0);
        assert_eq!(data.mantissa_exponent(), -4);
        assert_eq!(data.scientific_exponent(), 0);

        // Check "0.12345", simple.
        let data = Data {
            integer: b"",
            fraction: b"12345",
            digits_start: 0,
            truncated_digits: 0,
            raw_exponent: 0
        };
        assert_eq!(data.integer_digits(), 0);
        assert!(data.integer_iter().eq(b"".iter()));
        assert_eq!(data.fraction_digits(), 5);
        assert!(data.fraction_iter().eq(b"12345".iter()));
        assert_eq!(data.significant_fraction_digits(), 5);
        assert!(data.significant_fraction_iter().eq(b"12345".iter()));
        assert_eq!(data.mantissa_digits(), 5);
        assert_eq!(data.digits_start(), 0);
        assert_eq!(data.truncated_digits(), 0);
        assert_eq!(data.raw_exponent(), 0);
        assert_eq!(data.mantissa_exponent(), -5);
        assert_eq!(data.scientific_exponent(), -1);
    }
}
