#![cfg(feature = "format")]

use core::num;

use lexical_parse_integer::{
    Error,
    FromLexical,
    FromLexicalWithOptions,
    NumberFormatBuilder,
    Options,
};

#[test]
fn issue_96_test() {
    let opts = Options::new();
    const NO_CONSECUTIVE: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .internal_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();
    const CONSECUTIVE: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .internal_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(true)
        .build();
    const NO_LEADING: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(false)
        .internal_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(true)
        .build();

    let result = i64::from_lexical(b"_-1234");
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    // NOTE: We need to make sure we're not skipping digit separators before the
    // sign, which is never allowed.
    let result = u64::from_lexical_with_options::<NO_CONSECUTIVE>(b"_-1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(1)));

    let result = i64::from_lexical_with_options::<NO_CONSECUTIVE>(b"_-1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(1)));

    let result = i64::from_lexical_with_options::<NO_LEADING>(b"^-1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    // NOTE: This uis correct, since it's "trailing"
    let result = i64::from_lexical_with_options::<NO_LEADING>(b"_-1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(1)));

    let result = i64::from_lexical_with_options::<NO_LEADING>(b"_1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    let result = i64::from_lexical_with_options::<NO_LEADING>(b"X1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    let result = i64::from_lexical_with_options::<NO_CONSECUTIVE>(b"__1__234__", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    let result = i64::from_lexical_with_options::<CONSECUTIVE>(b"__1__234__", &opts);
    assert_eq!(result, Ok(1234));
}

#[test]
fn issue_96_i_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .internal_digit_separator(true)
        .consecutive_digit_separator(false)
        .required_digits(true)
        .build();

    let result = i64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_1_", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((11, 3)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1_23", &opts);
    assert_eq!(result, Ok((1123, 6)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1__23", &opts);
    assert_eq!(result, Ok((11, 3)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1_23_", &opts);
    assert_eq!(result, Ok((1123, 6)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1_23.", &opts);
    assert_eq!(result, Ok((1123, 6)));
}

#[test]
fn issue_96_l_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();

    let result = i64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Ok((1, 2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Ok((1, 3)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::Empty(1)));
}

#[test]
fn issue_96_t_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .trailing_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();

    let result = i64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123, 5)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123, 4)));
}

#[test]
fn issue_96_il_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .internal_digit_separator(true)
        .leading_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();

    let result = i64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Ok((123, 5)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Ok((123, 6)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((11, 3)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1_", &opts);
    assert_eq!(result, Ok((11, 3)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123, 4)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123, 4)));
}

#[test]
fn issue_96_it_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .internal_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();

    let result = i64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((11, 3)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1_", &opts);
    assert_eq!(result, Ok((11, 4)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123, 5)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123, 4)));
}

#[test]
fn issue_96_lt_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();

    let result = i64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Ok((1, 2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Ok((1, 3)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1_", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_11_", &opts);
    assert_eq!(result, Ok((11, 4)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123, 5)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123, 4)));
}

#[test]
fn issue_96_no_required_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(false)
        .required_digits(false)
        .build();

    let result = i64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Ok((0, 0)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Ok((0, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Ok((0, 2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Ok((1, 2)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Ok((1, 3)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"1_1_", &opts);
    assert_eq!(result, Ok((1, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_11_", &opts);
    assert_eq!(result, Ok((11, 4)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Ok((0, 1)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123, 5)));

    let result = i64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123, 4)));
}
