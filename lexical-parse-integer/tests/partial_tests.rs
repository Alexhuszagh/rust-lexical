use lexical_parse_integer::{FromLexical, FromLexicalWithOptions, Options};
use lexical_util::error::Error;
#[cfg(all(feature = "format", feature = "power-of-two"))]
use lexical_util::format::NumberFormatBuilder;
use lexical_util::format::STANDARD;

#[test]
fn u8_decimal_test() {
    assert_eq!(Ok((0, 1)), u8::from_lexical_partial(b"0"));
    assert_eq!(Ok((127, 3)), u8::from_lexical_partial(b"127"));
    assert_eq!(Ok((128, 3)), u8::from_lexical_partial(b"128"));
    assert_eq!(Ok((255, 3)), u8::from_lexical_partial(b"255"));
    assert_eq!(Err(Error::InvalidDigit(0)), u8::from_lexical(b"-1"));
    assert_eq!(Ok((1, 1)), u8::from_lexical_partial(b"1a"));

    let options = Options::default();
    assert_eq!(Ok((0, 1)), u8::from_lexical_partial_with_options::<{ STANDARD }>(b"0", &options));
}

#[test]
#[cfg(all(feature = "format", feature = "power-of-two"))]
fn u8_decimal_format_test() {
    // Test an invalid format.
    const FORMAT: u128 = NumberFormatBuilder::from_radix(1);
    let options = Options::default();
    assert_eq!(
        Err(Error::InvalidMantissaRadix),
        u8::from_lexical_with_options::<FORMAT>(b"0", &options)
    );
    assert_eq!(
        Err(Error::InvalidMantissaRadix),
        u8::from_lexical_partial_with_options::<FORMAT>(b"0", &options)
    );
}
