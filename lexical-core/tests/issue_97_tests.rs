#![cfg(all(feature = "parse", feature = "format"))]

use core::num;

use lexical_core::{Error, FromLexical, FromLexicalWithOptions, NumberFormatBuilder};

#[test]
fn issue_97_test() {
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .internal_digit_separator(true)
        .build();

    let fopts = lexical_core::ParseFloatOptions::new();
    let iopts = lexical_core::ParseIntegerOptions::new();

    assert_eq!(
        i64::from_lexical_with_options::<FMT>(b"_1234", &iopts),
        Err(Error::InvalidDigit(0))
    );
    assert_eq!(
        i64::from_lexical_with_options::<FMT>(b"1234_", &iopts),
        Err(Error::InvalidDigit(4))
    );

    assert_eq!(
        f64::from_lexical_with_options::<FMT>(b"_1234", &fopts),
        Err(Error::InvalidDigit(0))
    );
    assert_eq!(
        f64::from_lexical_with_options::<FMT>(b"1234_", &fopts),
        Err(Error::InvalidDigit(4))
    );

    assert_eq!(
        f64::from_lexical_with_options::<FMT>(b"_12.34", &fopts),
        Err(Error::InvalidDigit(0))
    );
    assert_eq!(
        f64::from_lexical_with_options::<FMT>(b"12.34_", &fopts),
        Err(Error::InvalidDigit(5))
    );

    assert_eq!(f64::from_lexical_with_options::<FMT>(b"1_2.34", &fopts), Ok(12.34));
}

#[test]
fn issue_97_nofmt_test() {
    assert_eq!(i64::from_lexical(b"_1234"), Err(Error::InvalidDigit(0)));
    assert_eq!(i64::from_lexical(b"1234_"), Err(Error::InvalidDigit(4)));

    assert_eq!(f64::from_lexical(b"_1234"), Err(Error::InvalidDigit(0)));
    assert_eq!(f64::from_lexical(b"1234_"), Err(Error::InvalidDigit(4)));

    assert_eq!(f64::from_lexical(b"_12.34"), Err(Error::InvalidDigit(0)));
    assert_eq!(f64::from_lexical(b"12.34_"), Err(Error::InvalidDigit(5)));

    assert_eq!(f64::from_lexical(b"_.34"), Err(Error::InvalidDigit(0)));
    assert_eq!(f64::from_lexical(b"0_0.34"), Err(Error::InvalidDigit(1)));

    assert_eq!(f64::from_lexical(b".34"), Ok(0.34));
}
