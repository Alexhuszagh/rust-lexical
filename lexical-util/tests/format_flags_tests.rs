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
            format::NumberFormat::<{ from_digit_separator(b'e') }>::builder().radix(16).build();
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
            format::NumberFormat::<{ from_decimal_point(b'e') }>::builder().radix(16).build();
        assert_eq!(format::is_valid_decimal_point(format), false);
    }
}

//#[test]
//fn test_is_valid_exponent_decimal() {
//    assert_eq!(is_valid_exponent_decimal(b'_'), true);
//    assert_eq!(is_valid_exponent_decimal(b'\''), true);
//    assert_eq!(is_valid_exponent_decimal(b'.'), true);
//    assert_eq!(is_valid_exponent_decimal(b'^'), true);
//    assert_eq!(is_valid_exponent_decimal(b'e'), true);
//    assert_eq!(is_valid_exponent_decimal(b'0'), false);
//    assert_eq!(is_valid_exponent_decimal(128), false);
//}
//
//#[test]
//fn test_is_valid_exponent_backup() {
//    assert_eq!(is_valid_exponent_backup(b'_'), true);
//    assert_eq!(is_valid_exponent_backup(b'\''), true);
//    assert_eq!(is_valid_exponent_backup(b'.'), true);
//    assert_eq!(is_valid_exponent_backup(b'^'), true);
//    assert_eq!(is_valid_exponent_backup(b'0'), false);
//    assert_eq!(is_valid_exponent_backup(128), false);
//
//    #[cfg(feature = "power_of_two")]
//    assert_eq!(is_valid_exponent_backup(b'e'), false);
//}
//
//#[test]
//fn test_is_valid_punctuation() {
//    assert_eq!(is_valid_punctuation(b'_', b'.', b'e', b'^'), true);
//    assert_eq!(is_valid_punctuation(b'_', b'.', b'^', b'^'), true);
//    assert_eq!(is_valid_punctuation(b'_', b'e', b'^', b'^'), true);
//    assert_eq!(is_valid_punctuation(b'e', b'.', b'^', b'^'), true);
//    assert_eq!(is_valid_punctuation(b'e', b'.', b'e', b'^'), false);
//    assert_eq!(is_valid_punctuation(b'^', b'.', b'e', b'^'), false);
//    assert_eq!(is_valid_punctuation(b'\'', b'^', b'e', b'^'), false);
//    assert_eq!(is_valid_punctuation(b'\'', b'e', b'e', b'^'), false);
//}
//
//#[test]
//fn test_exponent_decimal_to_flags() {
//    assert_eq!(exponent_decimal_to_flags(b'e'), 0x1940000);
//    assert_eq!(exponent_decimal_to_flags(b'^'), 0x1780000);
//    assert_eq!(exponent_decimal_to_flags(b'.'), 0xB80000);
//    assert_eq!(exponent_decimal_to_flags(b'\x00'), 0x0);
//}
//
//#[test]
//fn test_exponent_decimal_from_flags() {
//    assert_eq!(exponent_decimal_from_flags(0x1940000), b'e');
//    assert_eq!(exponent_decimal_from_flags(0x1780000), b'^');
//    assert_eq!(exponent_decimal_from_flags(0xB80000), b'.');
//    assert_eq!(exponent_decimal_from_flags(0x0), b'\x00');
//
//    // Test hybrid, to test mask
//    let flags = 0x1940000 | 0xBC000000 | 0xB8000000000000 | 0xBE00000000000000;
//    assert_eq!(exponent_decimal_from_flags(flags), b'e');
//}
//
//#[test]
//fn test_exponent_backup_to_flags() {
//    assert_eq!(exponent_backup_to_flags(b'e'), 0xCA000000);
//    assert_eq!(exponent_backup_to_flags(b'^'), 0xBC000000);
//    assert_eq!(exponent_backup_to_flags(b'.'), 0x5C000000);
//    assert_eq!(exponent_backup_to_flags(b'\x00'), 0x0);
//}
//
//#[test]
//fn test_exponent_backup_from_flags() {
//    assert_eq!(exponent_backup_from_flags(0xCA000000), b'e');
//    assert_eq!(exponent_backup_from_flags(0xBC000000), b'^');
//    assert_eq!(exponent_backup_from_flags(0x5C000000), b'.');
//    assert_eq!(exponent_backup_from_flags(0x0), b'\x00');
//
//    // Test hybrid, to test mask
//    let flags = 0x1940000 | 0xBC000000 | 0xB8000000000000 | 0xBE00000000000000;
//    assert_eq!(exponent_backup_from_flags(flags), b'^');
//}
//
//#[test]
//fn test_decimal_point_to_flags() {
//    assert_eq!(decimal_point_to_flags(b'e'), 0x194000000000000);
//    assert_eq!(decimal_point_to_flags(b'^'), 0x178000000000000);
//    assert_eq!(decimal_point_to_flags(b'.'), 0xB8000000000000);
//    assert_eq!(decimal_point_to_flags(b'\x00'), 0x0);
//}
//
//#[test]
//fn test_decimal_point_from_flags() {
//    assert_eq!(decimal_point_from_flags(0x194000000000000), b'e');
//    assert_eq!(decimal_point_from_flags(0x178000000000000), b'^');
//    assert_eq!(decimal_point_from_flags(0xB8000000000000), b'.');
//    assert_eq!(decimal_point_from_flags(0x0), b'\x00');
//
//    // Test hybrid, to test mask
//    let flags = 0x1940000 | 0xBC000000 | 0xB8000000000000 | 0xBE00000000000000;
//    assert_eq!(decimal_point_from_flags(flags), b'.');
//}
//
//#[test]
//fn test_digit_separator_to_flags() {
//    assert_eq!(digit_separator_to_flags(b'e'), 0xCA00000000000000);
//    assert_eq!(digit_separator_to_flags(b'^'), 0xBC00000000000000);
//    assert_eq!(digit_separator_to_flags(b'.'), 0x5C00000000000000);
//    assert_eq!(digit_separator_to_flags(b'\x00'), 0x0);
//}
//
//#[test]
//fn test_digit_separator_from_flags() {
//    assert_eq!(digit_separator_from_flags(0xCA00000000000000), b'e');
//    assert_eq!(digit_separator_from_flags(0xBC00000000000000), b'^');
//    assert_eq!(digit_separator_from_flags(0x5C00000000000000), b'.');
//    assert_eq!(digit_separator_from_flags(0x0), b'\x00');
//
//    // Test hybrid, to test mask
//    let flags = 0x1940000 | 0xBC000000 | 0xB8000000000000 | 0xBE00000000000000;
//    assert_eq!(digit_separator_from_flags(flags), b'_');
//}
//
//#[test]
//fn test_to_ascii_lowercase() {
//    assert_eq!(to_ascii_lowercase(b'E'), b'e');
//    assert_eq!(to_ascii_lowercase(b'e'), b'e');
//    assert_eq!(to_ascii_lowercase(b'Z'), b'z');
//    assert_eq!(to_ascii_lowercase(b'+'), b'+');
//    assert_eq!(to_ascii_lowercase(b'\t'), b'\t');
//}
