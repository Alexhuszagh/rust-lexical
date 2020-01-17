//! Specialized float parsers for different formats.

use crate::atof::algorithm::state::FloatState1;
use crate::lib::slice;
use super::consume::*;
use super::exponent::*;
use super::result::*;
use super::traits::*;
use super::trim::*;
use super::validate::*;


/// Standard float parser.
///
/// Guaranteed to parse `FloatFormat::RUST_STRING`, and
/// therefore will track that exact specification.
///
/// The requirements:
///     1). Must contain significant digits.
///     2). Must contain exponent digits if an exponent is present.
pub(crate) struct StandardParser;

impl FormatParser for StandardParser {
    perftools_inline!{
    fn consume_digits<'a>(
        digits: &'a [u8],
        radix: u32,
        character_separator: u8
    )
        -> (&'a [u8], &'a [u8])
    {
        consume_digits_no_separator(digits, radix, character_separator)
    }}

    perftools_inline!{
    fn parse_exponent<'a>(
        state: &mut FloatState1<'a>,
        bytes: &'a [u8],
        radix: u32,
        character_separator: u8
    )
        -> ParseResult<&'a [u8]>
    {
        parse_required_no_separator(state, bytes, radix, character_separator)
    }}

    perftools_inline!{
    fn validate_mantissa(
        state: &FloatState1,
        character_separator: u8
    )
        -> ParseResult<()>
    {
        validate_required_digits_no_separator(state, character_separator)
    }}

    perftools_inline!{
    fn validate_exponent(
        state: &FloatState1,
        character_separator: u8
    )
        -> ParseResult<()>
    {
        validate_required_exponent_no_separator(state, character_separator)
    }}

    perftools_inline!{
    fn ltrim<'a>(bytes: &'a [u8], digit_separator: u8) -> (&'a [u8], usize) {
        ltrim_no_separator(bytes, digit_separator)
    }}

    perftools_inline!{
    fn rtrim<'a>(bytes: &'a [u8], digit_separator: u8) -> (&'a [u8], usize) {
        rtrim_no_separator(bytes, digit_separator)
    }}
}

/// Standard float iterator.
///
/// Guaranteed to parse `FloatFormat::RUST_STRING`, and
/// therefore will track that exact specification.
///
/// The requirements:
///     1). Does not contain any digit separators
pub(crate) struct StandardIterator;

impl<'a> FormatIterator<'a> for StandardIterator {
    type IntegerIter = slice::Iter<'a, u8>;
    type FractionIter = slice::Iter<'a, u8>;

    perftools_inline!{
    fn integer_iter(
        integer: &'a [u8],
        _: u8
    )
        -> Self::IntegerIter
    {
        integer.iter()
    }}

    perftools_inline!{
    fn fraction_iter(
        fraction: &'a [u8],
        _: u8
    )
        -> Self::FractionIter
    {
        fraction.iter()
    }}
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn standard_parse_test() {
        StandardParser::run_tests([
            // Valid
            ("1.2345", Ok((b!("1"), b!("2345"), b!(""), 0).into())),
            ("12.345", Ok((b!("12"), b!("345"), b!(""), 0).into())),
            ("12345.6789", Ok((b!("12345"), b!("6789"), b!(""), 0).into())),
            ("1.2345e10", Ok((b!("1"), b!("2345"), b!("10"), 10).into())),
            ("1.2345e+10", Ok((b!("1"), b!("2345"), b!("+10"), 10).into())),
            ("1.2345e-10", Ok((b!("1"), b!("2345"), b!("-10"), -10).into())),
            ("100000000000000000000", Ok((b!("100000000000000000000"), b!(""), b!(""), 0).into())),
            ("100000000000000000001", Ok((b!("100000000000000000001"), b!(""), b!(""), 0).into())),
            ("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", Ok((b!("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791"), b!("9999999999999999999999999999999999999999999999999999999999999999999999"), b!(""), 0).into())),
            ("1009e-31", Ok((b!("1009"), b!(""), b!("-31"), -31).into())),
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
        ].iter().map(|x| (x.0, b'\x00', &x.1)));
    }
}
