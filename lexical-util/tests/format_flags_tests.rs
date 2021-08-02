#[cfg(feature = "format")]
use core::num;
use lexical_util::format;

#[cfg(feature = "format")]
const fn from_digit_separator(digit_separator: u8) -> u128 {
    format::NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(digit_separator))
        .digit_separator_flags(true)
        .build()
}

#[cfg(feature = "format")]
fn is_valid_digit_separator(digit_separator: u8) -> bool {
    let format = from_digit_separator(digit_separator);
    format::is_valid_digit_separator(format)
}

#[test]
#[cfg(feature = "format")]
fn test_is_valid_digit_separator() {
    assert_eq!(is_valid_digit_separator(b'_'), true);
    assert_eq!(is_valid_digit_separator(b'\''), true);
    assert_eq!(is_valid_digit_separator(b'.'), true);
    assert_eq!(is_valid_digit_separator(b'e'), true);
    assert_eq!(is_valid_digit_separator(b'0'), false);
    assert_eq!(is_valid_digit_separator(128), false);

    // Try with a custom radix.
    #[cfg(feature = "radix")]
    {
        let format =
            format::NumberFormat::<{ from_digit_separator(b'e') }>::rebuild().radix(16).build();
        assert_eq!(format::is_valid_digit_separator(format), false);
    }
}

const fn from_decimal_point(decimal_point: u8) -> u128 {
    format::NumberFormatBuilder::new().decimal_point(decimal_point).build()
}

fn is_valid_decimal_point(decimal_point: u8) -> bool {
    let format = from_decimal_point(decimal_point);
    format::is_valid_decimal_point(format)
}

#[test]
fn test_is_valid_decimal_point() {
    assert_eq!(is_valid_decimal_point(b'_'), true);
    assert_eq!(is_valid_decimal_point(b'\''), true);
    assert_eq!(is_valid_decimal_point(b'.'), true);
    assert_eq!(is_valid_decimal_point(b'e'), true);
    assert_eq!(is_valid_decimal_point(b'0'), false);
    assert_eq!(is_valid_decimal_point(128), false);

    // Try with a custom radix.
    #[cfg(feature = "radix")]
    {
        let format =
            format::NumberFormat::<{ from_decimal_point(b'e') }>::rebuild().radix(16).build();
        assert_eq!(format::is_valid_decimal_point(format), false);
    }
}

const fn from_exponent(exponent: u8) -> u128 {
    format::NumberFormatBuilder::new().exponent(exponent).build()
}

fn is_valid_exponent(exponent: u8) -> bool {
    let format = from_exponent(exponent);
    format::is_valid_exponent(format)
}

#[test]
fn test_is_valid_exponent() {
    assert_eq!(is_valid_exponent(b'_'), true);
    assert_eq!(is_valid_exponent(b'\''), true);
    assert_eq!(is_valid_exponent(b'.'), true);
    assert_eq!(is_valid_exponent(b'e'), true);
    assert_eq!(is_valid_exponent(b'0'), false);
    assert_eq!(is_valid_exponent(128), false);

    // Try with a custom radix.
    #[cfg(feature = "radix")]
    {
        let format = format::NumberFormat::<{ from_exponent(b'e') }>::rebuild().radix(16).build();
        assert_eq!(format::is_valid_exponent(format), false);
    }
}

#[cfg(feature = "format")]
fn is_valid_punctuation(
    digit_separator: u8,
    decimal_point: u8,
    exponent: u8,
    base_prefix: u8,
    base_suffix: u8,
) -> bool {
    let fmt = format::NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(digit_separator))
        .digit_separator_flags(true)
        .decimal_point(decimal_point)
        .exponent(exponent)
        .base_prefix(num::NonZeroU8::new(base_prefix))
        .base_suffix(num::NonZeroU8::new(base_suffix))
        .build();
    format::is_valid_punctuation(fmt)
}

#[test]
#[cfg(feature = "format")]
fn test_is_valid_punctuation() {
    assert_eq!(is_valid_punctuation(b'_', b'.', b'e', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'_', b'.', b'^', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'_', b'e', b'^', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'e', b'.', b'^', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'e', b'.', b'e', b'h', 0), false);
    assert_eq!(is_valid_punctuation(b'^', b'.', b'e', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'\'', b'.', b'e', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'\'', b'^', b'^', b'h', 0), false);
    assert_eq!(is_valid_punctuation(b'\'', b'e', b'e', b'h', 0), false);
    assert_eq!(is_valid_punctuation(b'\'', b'.', b'e', b'h', b'h'), false);
}
