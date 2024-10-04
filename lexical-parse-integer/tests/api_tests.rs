mod util;

use lexical_parse_integer::{FromLexical, FromLexicalWithOptions, Options};
use lexical_util::error::Error;
#[cfg(feature = "format")]
use lexical_util::format::NumberFormatBuilder;
use lexical_util::format::STANDARD;
use proptest::prelude::*;
#[cfg(feature = "power-of-two")]
use util::from_radix;

use crate::util::default_proptest_config;

#[test]
fn u8_decimal_test() {
    assert_eq!(Ok(0), u8::from_lexical(b"0"));
    assert_eq!(Ok(127), u8::from_lexical(b"127"));
    assert_eq!(Ok(128), u8::from_lexical(b"128"));
    assert_eq!(Ok(255), u8::from_lexical(b"255"));
    assert_eq!(Err(Error::InvalidDigit(0)), u8::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), u8::from_lexical(b"1a"));
}

#[test]
fn i8_decimal_test() {
    assert_eq!(Ok(0), i8::from_lexical(b"0"));
    assert_eq!(Ok(127), i8::from_lexical(b"127"));
    assert_eq!(Err(Error::Overflow(2)), i8::from_lexical(b"128"));
    assert_eq!(Err(Error::Overflow(2)), i8::from_lexical(b"255"));
    assert_eq!(Ok(-1), i8::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), i8::from_lexical(b"1a"));

    assert_eq!(Ok((1, 1)), i8::from_lexical_partial(b"1"));
    assert_eq!(Ok((1, 1)), i8::from_lexical_partial(b"1a"));
    assert_eq!(Ok((-1, 2)), i8::from_lexical_partial(b"-1"));
    assert_eq!(Ok((-1, 2)), i8::from_lexical_partial(b"-1a"));
}

#[test]
fn u16_decimal_test() {
    assert_eq!(Ok(0), u16::from_lexical(b"0"));
    assert_eq!(Ok(32767), u16::from_lexical(b"32767"));
    assert_eq!(Ok(32768), u16::from_lexical(b"32768"));
    assert_eq!(Ok(65535), u16::from_lexical(b"65535"));
    assert_eq!(Err(Error::InvalidDigit(0)), u16::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), u16::from_lexical(b"1a"));
}

#[test]
fn i16_decimal_test() {
    assert_eq!(Ok(0), i16::from_lexical(b"0"));
    assert_eq!(Ok(32767), i16::from_lexical(b"32767"));
    assert_eq!(Err(Error::Overflow(4)), i16::from_lexical(b"32768"));
    assert_eq!(Err(Error::Overflow(4)), i16::from_lexical(b"65535"));
    assert_eq!(Ok(-1), i16::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), i16::from_lexical(b"1a"));
}

#[test]
fn u32_decimal_test() {
    assert_eq!(Ok(0), u32::from_lexical(b"0"));
    assert_eq!(Ok(2147483647), u32::from_lexical(b"2147483647"));
    assert_eq!(Ok(2147483648), u32::from_lexical(b"2147483648"));
    assert_eq!(Ok(4294967295), u32::from_lexical(b"4294967295"));
    assert_eq!(Err(Error::InvalidDigit(0)), u32::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), u32::from_lexical(b"1a"));
}

#[test]
fn i32_decimal_test() {
    assert_eq!(Ok(0), i32::from_lexical(b"0"));
    assert_eq!(Ok(2147483647), i32::from_lexical(b"2147483647"));
    assert_eq!(Err(Error::Overflow(9)), i32::from_lexical(b"2147483648"));
    assert_eq!(Err(Error::Overflow(9)), i32::from_lexical(b"4294967295"));
    assert_eq!(Ok(-1), i32::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), i32::from_lexical(b"1a"));
}

#[test]
fn u64_decimal_test() {
    assert_eq!(Ok(0), u64::from_lexical(b"0"));
    assert_eq!(Ok(9223372036854775807), u64::from_lexical(b"9223372036854775807"));
    assert_eq!(Ok(9223372036854775808), u64::from_lexical(b"9223372036854775808"));
    assert_eq!(Ok(18446744073709551615), u64::from_lexical(b"18446744073709551615"));
    assert_eq!(Err(Error::InvalidDigit(0)), u64::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), u64::from_lexical(b"1a"));
}

#[test]
fn i64_decimal_test() {
    assert_eq!(Ok(0), i64::from_lexical(b"0"));
    assert_eq!(Ok(9223372036854775807), i64::from_lexical(b"9223372036854775807"));
    assert_eq!(Err(Error::Overflow(18)), i64::from_lexical(b"9223372036854775808"));
    assert_eq!(Err(Error::Overflow(19)), i64::from_lexical(b"18446744073709551615"));
    assert_eq!(Ok(-1), i64::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), i64::from_lexical(b"1a"));

    // Add tests discovered via fuzzing. This won't necessarily be the
    // proper index, since we use multi-digit parsing.
    assert!(i64::from_lexical(b"406260572150672006000066000000060060007667760000000000000000000+00000006766767766666767665670000000000000000000000666").err().unwrap().is_overflow());
    assert!(i64::from_lexical(b"406260572150672006000066000000060060007667760000000000000000000")
        .err()
        .unwrap()
        .is_overflow());
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
    assert_eq!(Err(Error::InvalidDigit(0)), u128::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), u128::from_lexical(b"1a"));
}

#[test]
fn i128_decimal_test() {
    assert_eq!(Ok(0), i128::from_lexical(b"0"));
    assert_eq!(
        Ok(170141183460469231731687303715884105727),
        i128::from_lexical(b"170141183460469231731687303715884105727")
    );
    assert_eq!(
        Err(Error::Overflow(38)),
        i128::from_lexical(b"170141183460469231731687303715884105728")
    );
    assert_eq!(
        Err(Error::Overflow(38)),
        i128::from_lexical(b"340282366920938463463374607431768211455")
    );
    assert_eq!(Ok(-1), i128::from_lexical(b"-1"));
    assert_eq!(Err(Error::InvalidDigit(1)), i128::from_lexical(b"1a"));
}

#[test]
fn double_sign_test() {
    assert_eq!(Err(Error::InvalidDigit(1)), i16::from_lexical(b"+-0000"));
    assert_eq!(Err(Error::InvalidDigit(1)), i128::from_lexical(b"+-0000"));
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
    const FORMAT: u128 = from_radix(2);
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
    radix_to_u32::<{ from_radix(2) }>(b"100101", 37);
    radix_to_u32::<{ from_radix(3) }>(b"1101", 37);
    radix_to_u32::<{ from_radix(4) }>(b"211", 37);
    radix_to_u32::<{ from_radix(5) }>(b"122", 37);
    radix_to_u32::<{ from_radix(6) }>(b"101", 37);
    radix_to_u32::<{ from_radix(7) }>(b"52", 37);
    radix_to_u32::<{ from_radix(8) }>(b"45", 37);
    radix_to_u32::<{ from_radix(9) }>(b"41", 37);
    radix_to_u32::<{ from_radix(10) }>(b"37", 37);
    radix_to_u32::<{ from_radix(11) }>(b"34", 37);
    radix_to_u32::<{ from_radix(12) }>(b"31", 37);
    radix_to_u32::<{ from_radix(13) }>(b"2B", 37);
    radix_to_u32::<{ from_radix(14) }>(b"29", 37);
    radix_to_u32::<{ from_radix(15) }>(b"27", 37);
    radix_to_u32::<{ from_radix(16) }>(b"25", 37);
    radix_to_u32::<{ from_radix(17) }>(b"23", 37);
    radix_to_u32::<{ from_radix(18) }>(b"21", 37);
    radix_to_u32::<{ from_radix(19) }>(b"1I", 37);
    radix_to_u32::<{ from_radix(20) }>(b"1H", 37);
    radix_to_u32::<{ from_radix(21) }>(b"1G", 37);
    radix_to_u32::<{ from_radix(22) }>(b"1F", 37);
    radix_to_u32::<{ from_radix(23) }>(b"1E", 37);
    radix_to_u32::<{ from_radix(24) }>(b"1D", 37);
    radix_to_u32::<{ from_radix(25) }>(b"1C", 37);
    radix_to_u32::<{ from_radix(26) }>(b"1B", 37);
    radix_to_u32::<{ from_radix(27) }>(b"1A", 37);
    radix_to_u32::<{ from_radix(28) }>(b"19", 37);
    radix_to_u32::<{ from_radix(29) }>(b"18", 37);
    radix_to_u32::<{ from_radix(30) }>(b"17", 37);
    radix_to_u32::<{ from_radix(31) }>(b"16", 37);
    radix_to_u32::<{ from_radix(32) }>(b"15", 37);
    radix_to_u32::<{ from_radix(33) }>(b"14", 37);
    radix_to_u32::<{ from_radix(34) }>(b"13", 37);
    radix_to_u32::<{ from_radix(35) }>(b"12", 37);
    radix_to_u32::<{ from_radix(36) }>(b"11", 37);
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

#[test]
#[cfg(all(feature = "power-of-two", feature = "format"))]
fn base_prefix_test() {
    use core::num;

    const FORMAT: u128 = NumberFormatBuilder::new().base_prefix(num::NonZeroU8::new(b'x')).build();
    let options = Options::new();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x1", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x12", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"12", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x12", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x-12", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"012", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012h", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012h ", &options).is_err());

    assert!(i32::from_lexical_partial_with_options::<FORMAT>(b"-0x012h", &options).is_ok());
    assert!(i32::from_lexical_partial_with_options::<FORMAT>(b"-0x012h ", &options).is_ok());
}

#[test]
#[cfg(all(feature = "power-of-two", feature = "format"))]
fn base_suffix_test() {
    use core::num;

    const FORMAT: u128 = NumberFormatBuilder::new().base_suffix(num::NonZeroU8::new(b'h')).build();
    let options = Options::new();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"h", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-h", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-1h", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"12h", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"12", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-12h", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x-12", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x12", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"012h", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-012", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012h", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012h ", &options).is_err());

    assert!(i32::from_lexical_partial_with_options::<FORMAT>(b"-0x012h", &options).is_ok());
    assert!(i32::from_lexical_partial_with_options::<FORMAT>(b"-0x012h ", &options).is_ok());
}

#[test]
#[cfg(all(feature = "power-of-two", feature = "format"))]
fn base_prefix_and_suffix_test() {
    use core::num;

    const FORMAT: u128 = NumberFormatBuilder::new()
        .base_prefix(num::NonZeroU8::new(b'x'))
        .base_suffix(num::NonZeroU8::new(b'h'))
        .build();
    let options = Options::new();
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+3h", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0x3", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0x3h", &options).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0x3h ", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0xh", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+h", &options).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0x", &options).is_err());
}

macro_rules! is_error {
    ($result:expr, $check:ident) => {{
        let result = $result;
        prop_assert!(result.is_err());
        let err = result.err().unwrap();
        prop_assert!(err.$check());
    }};
}

macro_rules! is_invalid_digit {
    ($result:expr) => {
        is_error!($result, is_invalid_digit)
    };
}

macro_rules! is_empty {
    ($result:expr) => {
        is_error!($result, is_empty)
    };
}

macro_rules! is_overflow {
    ($result:expr) => {
        is_error!($result, is_overflow)
    };
}

macro_rules! is_underflow {
    ($result:expr) => {
        is_error!($result, is_underflow)
    };
}

macro_rules! is_invalid_digit_match {
    ($result:expr, $p1:pat_param $(| $prest:pat_param)*) => {{
        let result = $result;
        prop_assert!(result.is_err());
        let err = result.err().unwrap();
        prop_assert!(err.is_invalid_digit());
        prop_assert!(matches!(*err.index().unwrap(), $p1 $(| $prest)*));
    }};
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    #[cfg(feature = "power-of-two")]
    fn i32_binary_roundtrip_display_proptest(i in i32::MIN..i32::MAX) {
        let options = Options::new();
        const FORMAT: u128 = from_radix(2);
        let digits = if i < 0 {
            format!("-{:b}", (i as i64).wrapping_neg())
        } else {
            format!("{:b}", i)
        };
        let result = i32::from_lexical_with_options::<FORMAT>(digits.as_bytes(), &options);
        prop_assert_eq!(i, result.unwrap());
    }

    #[test]
    fn u8_invalid_proptest(i in r"[+]?[0-9]{2}\D") {
        is_invalid_digit_match!(u8::from_lexical(i.as_bytes()), 2 | 3);
    }

    #[test]
    fn u8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}") {
        is_overflow!(u8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u8_negative_proptest(i in r"[-][1-9][0-9]{2}") {
        is_invalid_digit!(u8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u8_double_sign_proptest(i in r"[+]{2}[0-9]{2}") {
        is_invalid_digit_match!(u8::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u8_sign_only_proptest(i in r"[+]") {
        is_empty!(u8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u8_trailing_digits_proptest(i in r"[+]?[0-9]{2}\D[0-9]{2}") {
        is_invalid_digit_match!(u8::from_lexical(i.as_bytes()), 2 | 3);
    }

    #[test]
    fn i8_invalid_proptest(i in r"[+-]?[0-9]{2}\D") {
        is_invalid_digit_match!(i8::from_lexical(i.as_bytes()), 2 | 3);
    }

    #[test]
    fn i8_overflow_proptest(i in r"[+]?[1-9][0-9]{3}") {
        is_overflow!(i8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i8_underflow_proptest(i in r"[-][1-9][0-9]{3}") {
        is_underflow!(i8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i8_double_sign_proptest(i in r"[+-]{2}[0-9]{2}") {
        is_invalid_digit_match!(i8::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i8_sign_only_proptest(i in r"[+-]") {
        is_empty!(i8::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i8_trailing_digits_proptest(i in r"[+-]?[0-9]{2}\D[0-9]{2}") {
        is_invalid_digit_match!(i8::from_lexical(i.as_bytes()), 2 | 3);
    }

    #[test]
    fn u16_invalid_proptest(i in r"[+]?[0-9]{4}\D") {
        is_invalid_digit_match!(u16::from_lexical(i.as_bytes()), 4 | 5);
    }

    #[test]
    fn u16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}") {
        is_overflow!(u16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u16_negative_proptest(i in r"[-][1-9][0-9]{4}") {
        is_invalid_digit!(u16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u16_double_sign_proptest(i in r"[+]{2}[0-9]{4}") {
        is_invalid_digit_match!(u16::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u16_sign_only_proptest(i in r"[+]") {
        is_empty!(u16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u16_trailing_digits_proptest(i in r"[+]?[0-9]{4}\D[0-9]{2}") {
        is_invalid_digit_match!(u16::from_lexical(i.as_bytes()), 4 | 5);
    }

    #[test]
    fn i16_invalid_proptest(i in r"[+-]?[0-9]{4}\D") {
        is_invalid_digit_match!(i16::from_lexical(i.as_bytes()), 4 | 5);
    }

    #[test]
    fn i16_overflow_proptest(i in r"[+]?[1-9][0-9]{5}") {
        is_overflow!(i16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i16_underflow_proptest(i in r"[-][1-9][0-9]{5}") {
        is_underflow!(i16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i16_double_sign_proptest(i in r"[+-]{2}[0-9]{4}") {
        is_invalid_digit_match!(i16::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i16_sign_only_proptest(i in r"[+-]") {
        is_empty!(i16::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i16_trailing_digits_proptest(i in r"[+-]?[0-9]{4}\D[0-9]{2}") {
        is_invalid_digit_match!(i16::from_lexical(i.as_bytes()), 4 | 5);
    }

    #[test]
    fn u32_invalid_proptest(i in r"[+]?[0-9]{9}\D") {
        is_invalid_digit_match!(u32::from_lexical(i.as_bytes()), 9 | 10);
    }

    #[test]
    fn u32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}") {
        is_overflow!(u32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u32_negative_proptest(i in r"[-][1-9][0-9]{9}") {
        is_invalid_digit!(u32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u32_double_sign_proptest(i in r"[+]{2}[0-9]{9}") {
        is_invalid_digit_match!(u32::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u32_sign_only_proptest(i in r"[+]") {
        is_empty!(u32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u32_trailing_digits_proptest(i in r"[+]?[0-9]{9}\D[0-9]{2}") {
        is_invalid_digit_match!(u32::from_lexical(i.as_bytes()), 9 | 10);
    }

    #[test]
    fn i32_invalid_proptest(i in r"[+-]?[0-9]{9}\D") {
        is_invalid_digit_match!(i32::from_lexical(i.as_bytes()), 9 | 10);
    }

    #[test]
    fn i32_overflow_proptest(i in r"[+]?[1-9][0-9]{10}") {
        is_overflow!(i32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i32_underflow_proptest(i in r"-[1-9][0-9]{10}") {
        is_underflow!(i32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i32_double_sign_proptest(i in r"[+-]{2}[0-9]{9}") {
        is_invalid_digit_match!(i32::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i32_sign_only_proptest(i in r"[+-]") {
        is_empty!(i32::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i32_trailing_digits_proptest(i in r"[+-]?[0-9]{9}\D[0-9]{2}") {
        is_invalid_digit_match!(i32::from_lexical(i.as_bytes()), 9 | 10);
    }

    #[test]
    fn u64_invalid_proptest(i in r"[+]?[0-9]{19}\D") {
        is_invalid_digit_match!(u64::from_lexical(i.as_bytes()), 19 | 20);
    }

    #[test]
    fn u64_overflow_proptest(i in r"[+]?[1-9][0-9]{21}") {
        is_overflow!(u64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u64_negative_proptest(i in r"[-][1-9][0-9]{21}") {
        is_invalid_digit!(u64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u64_double_sign_proptest(i in r"[+]{2}[0-9]{19}") {
        is_invalid_digit_match!(u64::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u64_sign_only_proptest(i in r"[+]") {
        is_empty!(u64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u64_trailing_digits_proptest(i in r"[+]?[0-9]{19}\D[0-9]{2}") {
        is_invalid_digit_match!(u64::from_lexical(i.as_bytes()), 19 | 20);
    }

    #[test]
    fn i64_invalid_proptest(i in r"[+-]?[0-9]{18}\D") {
        is_invalid_digit_match!(i64::from_lexical(i.as_bytes()), 18 | 19);
    }

    #[test]
    fn i64_overflow_proptest(i in r"[+]?[1-9][0-9]{19}") {
        is_overflow!(i64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i64_underflow_proptest(i in r"-[1-9][0-9]{19}") {
        is_underflow!(i64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i64_double_sign_proptest(i in r"[+-]{2}[0-9]{18}") {
        is_invalid_digit_match!(i64::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i64_sign_only_proptest(i in r"[+-]") {
        is_empty!(i64::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i64_trailing_digits_proptest(i in r"[+-]?[0-9]{18}\D[0-9]{2}") {
        is_invalid_digit_match!(i64::from_lexical(i.as_bytes()), 18 | 19);
    }

    #[test]
    fn u128_invalid_proptest(i in r"[+]?[0-9]{38}\D") {
        is_invalid_digit_match!(u128::from_lexical(i.as_bytes()), 38 | 39);
    }

    #[test]
    fn u128_overflow_proptest(i in r"[+]?[1-9][0-9]{39}") {
        is_overflow!(u128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u128_negative_proptest(i in r"[-][1-9][0-9]{39}") {
        is_invalid_digit!(u128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u128_double_sign_proptest(i in r"[+]{2}[0-9]{38}") {
        is_invalid_digit_match!(u128::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn u128_sign_only_proptest(i in r"[+]") {
        is_empty!(u128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn u128_trailing_digits_proptest(i in r"[+]?[0-9]{38}\D[0-9]{2}") {
        is_invalid_digit_match!(u128::from_lexical(i.as_bytes()), 38 | 39);
    }

    #[test]
    fn i128_invalid_proptest(i in r"[+-]?[0-9]{38}\D") {
        is_invalid_digit_match!(i128::from_lexical(i.as_bytes()), 38 | 39);
    }

    #[test]
    fn i128_overflow_proptest(i in r"[+]?[1-9][0-9]{39}") {
        is_overflow!(i128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i128_underflow_proptest(i in r"-[1-9][0-9]{39}") {
        is_underflow!(i128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i128_double_sign_proptest(i in r"[+-]{2}[0-9]{38}") {
        is_invalid_digit_match!(i128::from_lexical(i.as_bytes()), 1);
    }

    #[test]
    fn i128_sign_only_proptest(i in r"[+-]") {
        is_empty!(i128::from_lexical(i.as_bytes()));
    }

    #[test]
    fn i128_trailing_digits_proptest(i in r"[+-]?[0-9]{38}\D[0-9]{2}") {
        is_invalid_digit_match!(i128::from_lexical(i.as_bytes()), 38 | 39);
    }
}
