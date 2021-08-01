#[cfg(feature = "power-of-two")]
mod util;

use lexical_parse_integer::{FromLexical, FromLexicalWithOptions, Options};
use lexical_util::error::ErrorCode;
#[cfg(feature = "format")]
use lexical_util::format::NumberFormatBuilder;
use lexical_util::format::STANDARD;
use proptest::prelude::*;
#[cfg(feature = "power-of-two")]
use util::to_format;

#[test]
fn u8_decimal_test() {
    assert_eq!(Ok(0), u8::from_lexical(b"0"));
    assert_eq!(Ok(127), u8::from_lexical(b"127"));
    assert_eq!(Ok(128), u8::from_lexical(b"128"));
    assert_eq!(Ok(255), u8::from_lexical(b"255"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u8::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u8::from_lexical(b"1a"));
}

#[test]
fn i8_decimal_test() {
    assert_eq!(Ok(0), i8::from_lexical(b"0"));
    assert_eq!(Ok(127), i8::from_lexical(b"127"));
    assert_eq!(Err((ErrorCode::Overflow, 2).into()), i8::from_lexical(b"128"));
    assert_eq!(Err((ErrorCode::Overflow, 2).into()), i8::from_lexical(b"255"));
    assert_eq!(Ok(-1), i8::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i8::from_lexical(b"1a"));
}

#[test]
fn u16_decimal_test() {
    assert_eq!(Ok(0), u16::from_lexical(b"0"));
    assert_eq!(Ok(32767), u16::from_lexical(b"32767"));
    assert_eq!(Ok(32768), u16::from_lexical(b"32768"));
    assert_eq!(Ok(65535), u16::from_lexical(b"65535"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u16::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u16::from_lexical(b"1a"));
}

#[test]
fn i16_decimal_test() {
    assert_eq!(Ok(0), i16::from_lexical(b"0"));
    assert_eq!(Ok(32767), i16::from_lexical(b"32767"));
    assert_eq!(Err((ErrorCode::Overflow, 4).into()), i16::from_lexical(b"32768"));
    assert_eq!(Err((ErrorCode::Overflow, 4).into()), i16::from_lexical(b"65535"));
    assert_eq!(Ok(-1), i16::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i16::from_lexical(b"1a"));
}

#[test]
fn u32_decimal_test() {
    assert_eq!(Ok(0), u32::from_lexical(b"0"));
    assert_eq!(Ok(2147483647), u32::from_lexical(b"2147483647"));
    assert_eq!(Ok(2147483648), u32::from_lexical(b"2147483648"));
    assert_eq!(Ok(4294967295), u32::from_lexical(b"4294967295"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u32::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u32::from_lexical(b"1a"));
}

#[test]
fn i32_decimal_test() {
    assert_eq!(Ok(0), i32::from_lexical(b"0"));
    assert_eq!(Ok(2147483647), i32::from_lexical(b"2147483647"));
    assert_eq!(Err((ErrorCode::Overflow, 9).into()), i32::from_lexical(b"2147483648"));
    assert_eq!(Err((ErrorCode::Overflow, 9).into()), i32::from_lexical(b"4294967295"));
    assert_eq!(Ok(-1), i32::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i32::from_lexical(b"1a"));
}

#[test]
fn u64_decimal_test() {
    assert_eq!(Ok(0), u64::from_lexical(b"0"));
    assert_eq!(Ok(9223372036854775807), u64::from_lexical(b"9223372036854775807"));
    assert_eq!(Ok(9223372036854775808), u64::from_lexical(b"9223372036854775808"));
    assert_eq!(Ok(18446744073709551615), u64::from_lexical(b"18446744073709551615"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u64::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u64::from_lexical(b"1a"));
}

#[test]
fn i64_decimal_test() {
    assert_eq!(Ok(0), i64::from_lexical(b"0"));
    assert_eq!(Ok(9223372036854775807), i64::from_lexical(b"9223372036854775807"));
    assert_eq!(Err((ErrorCode::Overflow, 18).into()), i64::from_lexical(b"9223372036854775808"));
    assert_eq!(Err((ErrorCode::Overflow, 19).into()), i64::from_lexical(b"18446744073709551615"));
    assert_eq!(Ok(-1), i64::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i64::from_lexical(b"1a"));

    // Add tests discovered via fuzzing. This won't necessarily be the
    // proper index, since we use multi-digit parsing.
    assert_eq!(ErrorCode::Overflow, i64::from_lexical(b"406260572150672006000066000000060060007667760000000000000000000+00000006766767766666767665670000000000000000000000666").err().unwrap().code);
}

#[test]
fn u128_decimal_test() {
    assert_eq!(Ok(0), u128::from_lexical(b"0"));
    assert_eq!(
        Ok(170141183460469231731687303715884105727),
        u128::from_lexical(b"170141183460469231731687303715884105727")
    );
    assert_eq!(
        Ok(170141183460469231731687303715884105728),
        u128::from_lexical(b"170141183460469231731687303715884105728")
    );
    assert_eq!(
        Ok(340282366920938463463374607431768211455),
        u128::from_lexical(b"340282366920938463463374607431768211455")
    );
    assert_eq!(Err((ErrorCode::InvalidDigit, 0).into()), u128::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), u128::from_lexical(b"1a"));
}

#[test]
fn i128_decimal_test() {
    assert_eq!(Ok(0), i128::from_lexical(b"0"));
    assert_eq!(
        Ok(170141183460469231731687303715884105727),
        i128::from_lexical(b"170141183460469231731687303715884105727")
    );
    assert_eq!(
        Err((ErrorCode::Overflow, 38).into()),
        i128::from_lexical(b"170141183460469231731687303715884105728")
    );
    assert_eq!(
        Err((ErrorCode::Overflow, 38).into()),
        i128::from_lexical(b"340282366920938463463374607431768211455")
    );
    assert_eq!(Ok(-1), i128::from_lexical(b"-1"));
    assert_eq!(Err((ErrorCode::InvalidDigit, 1).into()), i128::from_lexical(b"1a"));
}

#[test]
fn options_test() {
    let options = Options::new();
    assert_eq!(Ok(0), i128::from_lexical_with_options::<STANDARD>(b"0", &options));
}

#[test]
#[cfg(feature = "power-of-two")]
fn i32_binary_test() {
    let options = Options::new();
    const FORMAT: u128 = to_format(2);
    assert_eq!(i32::from_lexical_with_options::<FORMAT>(b"11", &options), Ok(3));
    assert_eq!(i32::from_lexical_with_options::<FORMAT>(b"-11", &options), Ok(-3));
}

#[cfg(feature = "radix")]
fn radix_to_u32<const FORMAT: u128>(bytes: &[u8], expected: u32) {
    let options = Options::new();
    let result = u32::from_lexical_with_options::<{ FORMAT }>(bytes, &options);
    assert_eq!(result, Ok(expected));
}

#[test]
#[cfg(feature = "radix")]
fn radix_test() {
    radix_to_u32::<{ to_format(2) }>(b"100101", 37);
    radix_to_u32::<{ to_format(3) }>(b"1101", 37);
    radix_to_u32::<{ to_format(4) }>(b"211", 37);
    radix_to_u32::<{ to_format(5) }>(b"122", 37);
    radix_to_u32::<{ to_format(6) }>(b"101", 37);
    radix_to_u32::<{ to_format(7) }>(b"52", 37);
    radix_to_u32::<{ to_format(8) }>(b"45", 37);
    radix_to_u32::<{ to_format(9) }>(b"41", 37);
    radix_to_u32::<{ to_format(10) }>(b"37", 37);
    radix_to_u32::<{ to_format(11) }>(b"34", 37);
    radix_to_u32::<{ to_format(12) }>(b"31", 37);
    radix_to_u32::<{ to_format(13) }>(b"2B", 37);
    radix_to_u32::<{ to_format(14) }>(b"29", 37);
    radix_to_u32::<{ to_format(15) }>(b"27", 37);
    radix_to_u32::<{ to_format(16) }>(b"25", 37);
    radix_to_u32::<{ to_format(17) }>(b"23", 37);
    radix_to_u32::<{ to_format(18) }>(b"21", 37);
    radix_to_u32::<{ to_format(19) }>(b"1I", 37);
    radix_to_u32::<{ to_format(20) }>(b"1H", 37);
    radix_to_u32::<{ to_format(21) }>(b"1G", 37);
    radix_to_u32::<{ to_format(22) }>(b"1F", 37);
    radix_to_u32::<{ to_format(23) }>(b"1E", 37);
    radix_to_u32::<{ to_format(24) }>(b"1D", 37);
    radix_to_u32::<{ to_format(25) }>(b"1C", 37);
    radix_to_u32::<{ to_format(26) }>(b"1B", 37);
    radix_to_u32::<{ to_format(27) }>(b"1A", 37);
    radix_to_u32::<{ to_format(28) }>(b"19", 37);
    radix_to_u32::<{ to_format(29) }>(b"18", 37);
    radix_to_u32::<{ to_format(30) }>(b"17", 37);
    radix_to_u32::<{ to_format(31) }>(b"16", 37);
    radix_to_u32::<{ to_format(32) }>(b"15", 37);
    radix_to_u32::<{ to_format(33) }>(b"14", 37);
    radix_to_u32::<{ to_format(34) }>(b"13", 37);
    radix_to_u32::<{ to_format(35) }>(b"12", 37);
    radix_to_u32::<{ to_format(36) }>(b"11", 37);
}

#[test]
#[cfg(feature = "format")]
fn i32_no_leading_zeros_test() {
    let options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new().no_integer_leading_zeros(true).build();
    assert!(i32::from_lexical_with_options::<FORMAT>(b"1", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"01", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"10", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"010", &options).is_err());
}

#[test]
#[cfg(feature = "format")]
fn i32_integer_internal_digit_separator_test() {
    let options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(std::num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .build();
    assert!(i32::from_lexical_with_options::<FORMAT>(b"3_1", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"_31", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"31_", &options).is_err());
}

#[test]
#[cfg(feature = "format")]
fn i32_integer_leading_digit_separator_test() {
    let options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(std::num::NonZeroU8::new(b'_'))
        .integer_leading_digit_separator(true)
        .build();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"3_1", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"_31", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"31_", &options).is_err());
}

#[test]
#[cfg(feature = "format")]
fn i32_integer_trailing_digit_separator_test() {
    let options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(std::num::NonZeroU8::new(b'_'))
        .integer_trailing_digit_separator(true)
        .build();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"3_1", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"_31", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"31_", &options).is_ok());
}

#[test]
#[cfg(feature = "format")]
fn i32_integer_consecutive_digit_separator_test() {
    let options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(std::num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"3_1", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"3__1", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"_31", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"31_", &options).is_err());
}

#[test]
#[cfg(feature = "format")]
fn i32_json_no_leading_zero() {
    let options = Options::new();
    use lexical_util::format::JSON;

    assert!(i32::from_lexical_with_options::<{ JSON }>(b"12", &options).is_ok());
    assert!(i32::from_lexical_with_options::<{ JSON }>(b"-12", &options).is_ok());
    assert!(i32::from_lexical_with_options::<{ JSON }>(b"012", &options).is_err());
    assert!(i32::from_lexical_with_options::<{ JSON }>(b"-012", &options).is_err());
}

proptest! {
    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "power-of-two")]
    fn i32_binary_roundtrip_display_proptest(i in i32::MIN..i32::MAX) {
        let options = Options::new();
        const FORMAT: u128 = to_format(2);
        let digits = if i < 0 {
            format!("-{:b}", (i as i64).wrapping_neg())
        } else {
            format!("{:b}", i)
        };
        let result = i32::from_lexical_with_options::<FORMAT>(digits.as_bytes(), &options);
        prop_assert_eq!(i, result.unwrap());
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u8_invalid_proptest(i in r"[+]?[0-9]{2}\D") {
        let result = u8::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let index = result.err().unwrap().index;
        prop_assert!(index == 2 || index == 3);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}") {
        let result = u8::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Overflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u8_negative_proptest(i in r"[-][1-9][0-9]{2}") {
        let result = u8::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::InvalidDigit);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u8_double_sign_proptest(i in r"[+]{2}[0-9]{2}") {
        let result = u8::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u8_sign_only_proptest(i in r"[+]") {
        let result = u8::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Empty);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u8_trailing_digits_proptest(i in r"[+]?[0-9]{2}\D[0-9]{2}") {
        let result = u8::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 2 || error.index == 3);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i8_invalid_proptest(i in r"[+-]?[0-9]{2}\D") {
        let result = i8::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 2 || error.index == 3);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}\D") {
        let result = i8::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Overflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i8_underflow_proptest(i in r"[-][1-9][0-9]{3}\D") {
        let result = i8::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Underflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i8_double_sign_proptest(i in r"[+-]{2}[0-9]{2}") {
        let result = i8::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i8_sign_only_proptest(i in r"[+-]") {
        let result = i8::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::Empty);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i8_trailing_digits_proptest(i in r"[+-]?[0-9]{2}\D[0-9]{2}") {
        let result = i8::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 2 || error.index == 3);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u16_invalid_proptest(i in r"[+]?[0-9]{4}\D") {
        let result = u16::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 4 || error.index == 5);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}\D") {
        let result = u16::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Overflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u16_negative_proptest(i in r"[-][1-9][0-9]{4}") {
        let result = u16::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::InvalidDigit);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u16_double_sign_proptest(i in r"[+]{2}[0-9]{4}") {
        let result = u16::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u16_sign_only_proptest(i in r"[+]") {
        let result = u16::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Empty);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u16_trailing_digits_proptest(i in r"[+]?[0-9]{4}\D[0-9]{2}") {
        let result = u16::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 4 || error.index == 5);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i16_invalid_proptest(i in r"[+-]?[0-9]{4}\D") {
        let result = i16::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 4 || error.index == 5);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}\D") {
        let result = i16::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Overflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i16_underflow_proptest(i in r"[-][1-9][0-9]{5}\DD") {
        let result = i16::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Underflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i16_double_sign_proptest(i in r"[+-]{2}[0-9]{4}") {
        let result = i16::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i16_sign_only_proptest(i in r"[+-]") {
        let result = i16::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Empty);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i16_trailing_digits_proptest(i in r"[+-]?[0-9]{4}\D[0-9]{2}") {
        let result = i16::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 4 || error.index == 5);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u32_invalid_proptest(i in r"[+]?[0-9]{9}\D") {
        let result = u32::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 9 || error.index == 10);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}\D") {
        let result = u32::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Overflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u32_negative_proptest(i in r"[-][1-9][0-9]{9}") {
        let result = u32::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::InvalidDigit);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u32_double_sign_proptest(i in r"[+]{2}[0-9]{9}") {
        let result = u32::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u32_sign_only_proptest(i in r"[+]") {
        let result = u32::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Empty);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u32_trailing_digits_proptest(i in r"[+]?[0-9]{9}\D[0-9]{2}") {
        let result = u32::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 9 || error.index == 10);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i32_invalid_proptest(i in r"[+-]?[0-9]{9}\D") {
        let result = i32::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 9 || error.index == 10);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}\D") {
        let result = i32::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Overflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i32_underflow_proptest(i in r"-[1-9][0-9]{10}\D") {
        let result = i32::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Underflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i32_double_sign_proptest(i in r"[+-]{2}[0-9]{9}") {
        let result = i32::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i32_sign_only_proptest(i in r"[+-]") {
        let result = i32::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Empty);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i32_trailing_digits_proptest(i in r"[+-]?[0-9]{9}\D[0-9]{2}") {
        let result = i32::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 9 || error.index == 10);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u64_invalid_proptest(i in r"[+]?[0-9]{19}\D") {
        let result = u64::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 19 || error.index == 20);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u64_overflow_proptest(i in r"[+]?[1-9][0-9]{21}\D") {
        let result = u64::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Overflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u64_negative_proptest(i in r"[-][1-9][0-9]{21}") {
        let result = u64::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::InvalidDigit);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u64_double_sign_proptest(i in r"[+]{2}[0-9]{19}") {
        let result = u64::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u64_sign_only_proptest(i in r"[+]") {
        let result = u64::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Empty);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn u64_trailing_digits_proptest(i in r"[+]?[0-9]{19}\D[0-9]{2}") {
        let result = u64::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 19 || error.index == 20);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i64_invalid_proptest(i in r"[+-]?[0-9]{18}\D") {
        let result = i64::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 18 || error.index == 19);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i64_overflow_proptest(i in r"[+]?[1-9][0-9]{19}\D") {
        let result = i64::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Overflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i64_underflow_proptest(i in r"-[1-9][0-9]{19}\D") {
        let result = i64::from_lexical(i.as_bytes());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Underflow);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i64_double_sign_proptest(i in r"[+-]{2}[0-9]{18}") {
        let result = i64::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i64_sign_only_proptest(i in r"[+-]") {
        let result = i32::from_lexical(i.as_bytes());
        prop_assert!(result.is_err());
        let code = result.err().unwrap().code;
        prop_assert_eq!(code, ErrorCode::Empty);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn i64_trailing_digits_proptest(i in r"[+-]?[0-9]{18}\D[0-9]{2}") {
        let result = i64::from_lexical(i.as_bytes());
        let error = result.err().unwrap();
        prop_assert_eq!(error.code, ErrorCode::InvalidDigit);
        prop_assert!(error.index == 18 || error.index == 19);
    }
}
