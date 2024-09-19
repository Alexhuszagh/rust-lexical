#![cfg(all(feature = "power-of-two", feature = "format"))]

use lexical_parse_integer::FromLexicalWithOptions;
use lexical_parse_integer::NumberFormatBuilder;
use lexical_parse_integer::Options;
use lexical_util::error::Error;

#[test]
fn issue_98_test() {
    const DECIMAL_FORMAT: u128 = NumberFormatBuilder::new()
        .required_digits(true)
        .no_positive_mantissa_sign(false)
        .no_special(true)
        .no_integer_leading_zeros(true)
        .no_float_leading_zeros(false)
        .build();
    let result = i64::from_lexical_with_options::<DECIMAL_FORMAT>(b"1.1.0", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidDigit(1));
    assert_eq!(
        i64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"1.1.0", &Options::new()),
        Ok((1, 1))
    );

    let result = i64::from_lexical_with_options::<DECIMAL_FORMAT>(b"1.1", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidDigit(1));
    assert!(i64::from_lexical_with_options::<DECIMAL_FORMAT>(b"1.1", &Options::new()).is_err());
    assert_eq!(
        i64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"1.1", &Options::new()),
        Ok((1, 1))
    );

    let result = i64::from_lexical_with_options::<DECIMAL_FORMAT>(b"0.1.0", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidDigit(1));
    assert_eq!(
        i64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"0.1.0", &Options::new()),
        Ok((0, 1))
    );

    let result = i64::from_lexical_with_options::<DECIMAL_FORMAT>(b"0.1", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidDigit(1));
    assert!(i64::from_lexical_with_options::<DECIMAL_FORMAT>(b"0.1", &Options::new()).is_err());
    assert_eq!(
        i64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"0.1", &Options::new()),
        Ok((0, 1))
    );

    let result = i64::from_lexical_with_options::<DECIMAL_FORMAT>(b"01.1", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidLeadingZeros(0));

    let result = i64::from_lexical_with_options::<DECIMAL_FORMAT>(b"00.1", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidLeadingZeros(0));

    assert_eq!(
        i64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"10.1", &Options::new()),
        Ok((10, 2))
    );
    assert_eq!(
        i64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"11.1", &Options::new()),
        Ok((11, 2))
    );
}
