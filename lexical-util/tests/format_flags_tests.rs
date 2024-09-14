#[cfg(feature = "format")]
use core::num;

#[cfg(feature = "format")]
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

#[cfg(all(feature = "power-of-two", feature = "format"))]
fn is_valid_punctuation(digit_separator: u8, base_prefix: u8, base_suffix: u8) -> bool {
    let fmt = format::NumberFormatBuilder::new()
        .digit_separator(num::NonZeroU8::new(digit_separator))
        .digit_separator_flags(true)
        .base_prefix(num::NonZeroU8::new(base_prefix))
        .base_suffix(num::NonZeroU8::new(base_suffix))
        .build();
    format::is_valid_punctuation(fmt)
}

#[test]
#[cfg(all(feature = "power-of-two", feature = "format"))]
fn test_is_valid_punctuation() {
    assert_eq!(is_valid_punctuation(b'_', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'e', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'^', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'\'', b'h', 0), true);
    assert_eq!(is_valid_punctuation(b'\'', b'h', b'h'), false);
}
