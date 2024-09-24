#![cfg(feature = "format")]

use core::num;

use lexical_parse_float::{
    Error,
    FromLexical,
    FromLexicalWithOptions,
    NumberFormatBuilder,
    Options,
};
use lexical_util::format::STANDARD;

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

    let result = f64::from_lexical(b"_-1234");
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    let result = f64::from_lexical_with_options::<NO_CONSECUTIVE>(b"_-1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(1)));

    let result = f64::from_lexical_with_options::<NO_LEADING>(b"^-1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    // NOTE: This uis correct, since it's "trailing"
    let result = f64::from_lexical_with_options::<NO_LEADING>(b"_-1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(1)));

    let result = f64::from_lexical_with_options::<NO_LEADING>(b"_1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    let result = f64::from_lexical_with_options::<NO_LEADING>(b"X1234", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    let result = f64::from_lexical_with_options::<NO_CONSECUTIVE>(b"__1__234__", &opts);
    assert_eq!(result, Err(Error::InvalidDigit(0)));

    let result = f64::from_lexical_with_options::<CONSECUTIVE>(b"__1__234__", &opts);
    assert_eq!(result, Ok(1234f64));
}

#[test]
fn issue_96_i_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .internal_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();

    let result = f64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_1_", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((11f64, 3)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1_23", &opts);
    assert_eq!(result, Ok((1123f64, 6)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1__23", &opts);
    assert_eq!(result, Ok((11f64, 3)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1_23_", &opts);
    assert_eq!(result, Ok((1123f64, 6)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1_23.", &opts);
    assert_eq!(result, Ok((1123f64, 7)));
}

#[test]
fn issue_96_l_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();

    let result = f64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Ok((1f64, 2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Ok((1f64, 3)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));
}

#[test]
fn issue_96_t_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .trailing_digit_separator(true)
        .consecutive_digit_separator(false)
        .build();

    let result = f64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123f64, 5)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123f64, 4)));
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

    let result = f64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Ok((123f64, 5)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Ok((123f64, 6)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((11f64, 3)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1_", &opts);
    assert_eq!(result, Ok((11f64, 3)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123f64, 4)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123f64, 4)));
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

    let result = f64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(0)));

    let result: Result<(f64, usize), Error> =
        f64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((11f64, 3)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1_", &opts);
    assert_eq!(result, Ok((11f64, 4)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123f64, 5)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123f64, 4)));
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

    let result = f64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Err(Error::Empty(0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Err(Error::Empty(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Err(Error::Empty(2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Ok((1f64, 2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Ok((1f64, 3)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1_", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_11_", &opts);
    assert_eq!(result, Ok((11f64, 4)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Err(Error::EmptyMantissa(1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123f64, 5)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123f64, 4)));
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

    let result = f64::from_lexical_partial_with_options::<FMT>(b"", &opts);
    assert_eq!(result, Ok((0f64, 0)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_", &opts);
    assert_eq!(result, Ok((0f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_", &opts);
    assert_eq!(result, Ok((0f64, 2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_1_23", &opts);
    assert_eq!(result, Ok((1f64, 2)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+_1_23", &opts);
    assert_eq!(result, Ok((1f64, 3)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1__1_23", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"1_1_", &opts);
    assert_eq!(result, Ok((1f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_11_", &opts);
    assert_eq!(result, Ok((11f64, 4)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"_+1_23", &opts);
    assert_eq!(result, Ok((0f64, 1)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123_", &opts);
    assert_eq!(result, Ok((123f64, 5)));

    let result = f64::from_lexical_partial_with_options::<FMT>(b"+123__", &opts);
    assert_eq!(result, Ok((123f64, 4)));
}

#[test]
fn issue_96_rounding_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .internal_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(true)
        .build();
    let input = b"0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002225073858507200889024586876085859887650423112240959465493524802562440009228235695178775888803759155264230978095043431208587738715835729182199302029437922422355981982750124204178896957131179108226104397197960400045489739193807919893608152561311337614984204327175103362739154978273159414382813627511383860409424946494228631669542910508020181592664213499660651780309507591305871984642390606863710200510872328278467884363194451586613504122347901479236958520832159762106637540161373658304419360371477835530668283453563400507407304013560296804637591858316312422452159926254649430083685186171942241764645513713542013221703137049658321015465406803539741790602258950302350193751977303094576317321085250729930508976158251915";
    let result = f32::from_lexical_partial_with_options::<STANDARD>(input, &opts);
    assert_eq!(result, Ok((0f32, input.len())));
    let result = f32::from_lexical_partial_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok((0f32, input.len())));

    let result = f64::from_lexical_partial_with_options::<STANDARD>(input, &opts);
    assert_eq!(result, Ok((2.225073858507201e-308f64, input.len())));
    let result = f64::from_lexical_partial_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok((2.225073858507201e-308f64, input.len())));

    let input = b"_0e+___00";
    let result = f32::from_lexical_partial_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok((0f32, input.len())));

    let result = f32::from_lexical_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok(0f32));

    let result = f64::from_lexical_partial_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok((0f64, input.len())));

    let result = f64::from_lexical_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok(0f64));

    let input = b"323081493377685546875e-297";
    let result = f64::from_lexical_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok(3.2308149337768557e-277));

    let input = b"32308_1493_3776_8554_6875e-297";
    let result = f64::from_lexical_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok(3.2308149337768557e-277));
}

#[test]
fn issue_96_wuff_test() {
    let opts = Options::new();
    const FMT: u128 = NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(b'_'))
        .leading_digit_separator(true)
        .internal_digit_separator(true)
        .trailing_digit_separator(true)
        .consecutive_digit_separator(true)
        .build();
    let input = b"0.000061094760894775390625";
    let result = f32::from_lexical_partial_with_options::<STANDARD>(input, &opts);
    assert_eq!(result, Ok((6.109476e-5f32, input.len())));

    let result = f64::from_lexical_partial_with_options::<STANDARD>(input, &opts);
    assert_eq!(result, Ok((6.109476089477539e-5, input.len())));

    let input = b"0_.0000610_9476_0894775390_625";
    let result = f32::from_lexical_partial_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok((6.109476e-5f32, input.len())));

    let result = f64::from_lexical_partial_with_options::<FMT>(input, &opts);
    assert_eq!(result, Ok((6.109476089477539e-5, input.len())));
}
