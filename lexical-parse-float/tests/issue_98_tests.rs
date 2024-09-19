#![cfg(all(feature = "power-of-two", feature = "format"))]

use std::assert_eq;

use lexical_parse_float::FromLexicalWithOptions;
use lexical_parse_float::NumberFormatBuilder;
use lexical_parse_float::Options;
use lexical_util::error::Error;

#[test]
fn issue_98_test() {
    const DECIMAL_FORMAT: u128 = NumberFormatBuilder::new()
        .required_digits(true)
        .no_positive_mantissa_sign(false)
        .no_special(true)
        .no_integer_leading_zeros(true)
        .no_float_leading_zeros(true)
        .build();
    let result = f64::from_lexical_with_options::<DECIMAL_FORMAT>(b"1.1.0", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidDigit(3));
    assert_eq!(
        f64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"1.1.0", &Options::new()),
        Ok((1.1f64, 3))
    );
    assert_eq!(
        f64::from_lexical_with_options::<DECIMAL_FORMAT>(b"1.1", &Options::new()),
        Ok(1.1f64)
    );
    assert_eq!(
        f64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"1.1", &Options::new()),
        Ok((1.1f64, 3))
    );

    let result = f64::from_lexical_with_options::<DECIMAL_FORMAT>(b"0.1.0", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidDigit(3));
    assert_eq!(
        f64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"0.1.0", &Options::new()),
        Ok((0.1f64, 3))
    );
    assert_eq!(
        f64::from_lexical_with_options::<DECIMAL_FORMAT>(b"0.1", &Options::new()),
        Ok(0.1f64)
    );
    assert_eq!(
        f64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"0.1", &Options::new()),
        Ok((0.1f64, 3))
    );

    let result = f64::from_lexical_with_options::<DECIMAL_FORMAT>(b"01.1.0", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidLeadingZeros(0));

    let result = f64::from_lexical_with_options::<DECIMAL_FORMAT>(b"00.1", &Options::new());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::InvalidLeadingZeros(0));

    assert_eq!(
        f64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"10.1", &Options::new()),
        Ok((10.1, 4))
    );
    assert_eq!(
        f64::from_lexical_partial_with_options::<DECIMAL_FORMAT>(b"11.1", &Options::new()),
        Ok((11.1, 4))
    );
}
