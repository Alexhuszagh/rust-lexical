mod util;

use lexical_parse_integer::{FromLexical, FromLexicalWithOptions, Options};
use lexical_util::error::Error;
#[cfg(feature = "format")]
use lexical_util::format::NumberFormatBuilder;
use lexical_util::format::STANDARD;
#[cfg(feature = "power-of-two")]
use util::from_radix;

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
    const OPTIONS: Options = Options::new();
    assert_eq!(Ok(0), i128::from_lexical_with_options::<STANDARD>(b"0", &OPTIONS));
}

#[test]
#[cfg(feature = "power-of-two")]
fn i32_binary_test() {
    const OPTIONS: Options = Options::new();
    const FORMAT: u128 = from_radix(2);
    assert_eq!(i32::from_lexical_with_options::<FORMAT>(b"11", &OPTIONS), Ok(3));
    assert_eq!(i32::from_lexical_with_options::<FORMAT>(b"-11", &OPTIONS), Ok(-3));
}

#[cfg(feature = "radix")]
fn radix_to_u32<const FORMAT: u128>(bytes: &[u8], expected: u32) {
    const OPTIONS: Options = Options::new();
    let result = u32::from_lexical_with_options::<{ FORMAT }>(bytes, &OPTIONS);
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
    const OPTIONS: Options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new().no_integer_leading_zeros(true).build_strict();
    assert!(i32::from_lexical_with_options::<FORMAT>(b"1", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"01", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"10", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"010", &OPTIONS).is_err());
}

#[test]
#[cfg(feature = "format")]
fn i32_integer_internal_digit_separator_test() {
    const OPTIONS: Options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(std::num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .build_strict();
    assert!(i32::from_lexical_with_options::<FORMAT>(b"3_1", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"_31", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"31_", &OPTIONS).is_err());
}

#[test]
#[cfg(feature = "format")]
fn i32_integer_leading_digit_separator_test() {
    const OPTIONS: Options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(std::num::NonZeroU8::new(b'_'))
        .integer_leading_digit_separator(true)
        .build_strict();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"3_1", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"_31", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"31_", &OPTIONS).is_err());
}

#[test]
#[cfg(feature = "format")]
fn i32_integer_trailing_digit_separator_test() {
    const OPTIONS: Options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(std::num::NonZeroU8::new(b'_'))
        .integer_trailing_digit_separator(true)
        .build_strict();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"3_1", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"_31", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"31_", &OPTIONS).is_ok());
}

#[test]
#[cfg(feature = "format")]
fn i32_integer_consecutive_digit_separator_test() {
    const OPTIONS: Options = Options::new();
    const FORMAT: u128 = NumberFormatBuilder::new()
        .digit_separator(std::num::NonZeroU8::new(b'_'))
        .integer_internal_digit_separator(true)
        .integer_consecutive_digit_separator(true)
        .build_strict();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"3_1", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"3__1", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"_31", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"31_", &OPTIONS).is_err());
}

#[test]
#[cfg(feature = "format")]
fn i32_json_no_leading_zero() {
    const OPTIONS: Options = Options::new();
    use lexical_util::format::JSON;

    assert!(i32::from_lexical_with_options::<{ JSON }>(b"12", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<{ JSON }>(b"-12", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<{ JSON }>(b"012", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<{ JSON }>(b"-012", &OPTIONS).is_err());
}

#[test]
#[cfg(all(feature = "power-of-two", feature = "format"))]
fn base_prefix_test() {
    use core::num;

    const FORMAT: u128 =
        NumberFormatBuilder::new().base_prefix(num::NonZeroU8::new(b'x')).build_strict();
    const OPTIONS: Options = Options::new();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x1", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x12", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"12", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x12", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x-12", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"012", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012h", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012h ", &OPTIONS).is_err());

    assert!(i32::from_lexical_partial_with_options::<FORMAT>(b"-0x012h", &OPTIONS).is_ok());
    assert!(i32::from_lexical_partial_with_options::<FORMAT>(b"-0x012h ", &OPTIONS).is_ok());
}

#[test]
#[cfg(all(feature = "power-of-two", feature = "format"))]
fn base_suffix_test() {
    use core::num;

    const FORMAT: u128 =
        NumberFormatBuilder::new().base_suffix(num::NonZeroU8::new(b'h')).build_strict();
    const OPTIONS: Options = Options::new();

    assert!(i32::from_lexical_with_options::<FORMAT>(b"h", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-h", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-1h", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"12h", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"12", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-12h", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x-12", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"0x12", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"012h", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-012", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012h", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"-0x012h ", &OPTIONS).is_err());

    assert!(i32::from_lexical_partial_with_options::<FORMAT>(b"-0x012h", &OPTIONS).is_ok());
    assert!(i32::from_lexical_partial_with_options::<FORMAT>(b"-0x012h ", &OPTIONS).is_ok());
}

#[test]
#[cfg(all(feature = "power-of-two", feature = "format"))]
fn base_prefix_and_suffix_test() {
    use core::num;

    const FORMAT: u128 = NumberFormatBuilder::new()
        .base_prefix(num::NonZeroU8::new(b'x'))
        .base_suffix(num::NonZeroU8::new(b'h'))
        .build_strict();
    const OPTIONS: Options = Options::new();
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+3h", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0x3", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0x3h", &OPTIONS).is_ok());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0x3h ", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0xh", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+h", &OPTIONS).is_err());
    assert!(i32::from_lexical_with_options::<FORMAT>(b"+0x", &OPTIONS).is_err());
}
