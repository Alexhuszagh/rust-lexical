#![cfg(any(feature = "parse", feature = "write"))]

use lexical_util::digit;

#[cfg(feature = "parse")]
fn char_to_digit<const RADIX: u32>(c: u8, expected: Option<u32>) {
    assert_eq!(digit::char_to_digit_const::<RADIX>(c), expected);
    assert_eq!(digit::char_to_digit(c, RADIX), expected);
}

#[test]
#[cfg(feature = "parse")]
fn char_to_digit_test() {
    char_to_digit::<2>(b'0', Some(0));
    char_to_digit::<2>(b'1', Some(1));
    char_to_digit::<2>(b'9', None);
    char_to_digit::<2>(b'A', None);
    char_to_digit::<2>(b'Z', None);

    char_to_digit::<10>(b'0', Some(0));
    char_to_digit::<10>(b'1', Some(1));
    char_to_digit::<10>(b'9', Some(9));
    char_to_digit::<10>(b'A', None);
    char_to_digit::<10>(b'Z', None);

    char_to_digit::<16>(b'0', Some(0));
    char_to_digit::<16>(b'1', Some(1));
    char_to_digit::<16>(b'9', Some(9));
    char_to_digit::<16>(b'A', Some(10));
    char_to_digit::<16>(b'Z', None);

    // char_to_digit doesn't care about the radix.
    // check some more comprehensive cases.
    for c in b'0'..=b'9' {
        char_to_digit::<10>(c, Some(c as u32 - b'0' as u32));
    }
    char_to_digit::<10>(0x29, None);
    char_to_digit::<10>(0x3A, None);
    char_to_digit::<10>(0x59, None);

    for c in b'0'..=b'8' {
        char_to_digit::<9>(c, Some(c as u32 - b'0' as u32));
    }
    char_to_digit::<9>(0x29, None);
    char_to_digit::<9>(0x39, None);
    char_to_digit::<9>(0x3A, None);
    char_to_digit::<9>(0x59, None);

    for c in b'0'..=b'9' {
        char_to_digit::<16>(c, Some(c as u32 - b'0' as u32));
    }
    for c in b'a'..=b'f' {
        char_to_digit::<16>(c, Some(c as u32 - b'a' as u32 + 10));
    }
    for c in b'A'..=b'F' {
        char_to_digit::<16>(c, Some(c as u32 - b'A' as u32 + 10));
    }
    char_to_digit::<16>(0x29, None);
    char_to_digit::<16>(0x40, None);
    char_to_digit::<16>(0x3A, None);
    char_to_digit::<16>(0x59, None);
    char_to_digit::<16>(0x41, Some(10));
    char_to_digit::<16>(0x47, None);
    char_to_digit::<16>(0x5A, None);
    char_to_digit::<16>(0x61, Some(10));
    char_to_digit::<16>(0x67, None);
    char_to_digit::<16>(0x7A, None);
}

#[cfg(feature = "parse")]
fn char_is_digit<const RADIX: u32>(c: u8, expected: bool) {
    assert_eq!(digit::char_is_digit_const::<RADIX>(c), expected);
    assert_eq!(digit::char_is_digit(c, RADIX), expected);
}

#[test]
#[cfg(feature = "parse")]
fn char_is_digit_const_test() {
    char_is_digit::<2>(b'0', true);
    char_is_digit::<2>(b'1', true);
    char_is_digit::<2>(b'9', false);
    char_is_digit::<2>(b'A', false);
    char_is_digit::<2>(b'Z', false);

    char_is_digit::<10>(b'0', true);
    char_is_digit::<10>(b'1', true);
    char_is_digit::<10>(b'9', true);
    char_is_digit::<10>(b'A', false);
    char_is_digit::<10>(b'Z', false);

    char_is_digit::<16>(b'0', true);
    char_is_digit::<16>(b'1', true);
    char_is_digit::<16>(b'9', true);
    char_is_digit::<16>(b'A', true);
    char_is_digit::<16>(b'Z', false);
}

#[cfg(feature = "write")]
fn digit_to_char<const RADIX: u32>(digit: u32, expected: u8) {
    assert_eq!(digit::digit_to_char_const::<RADIX>(digit), expected);
    assert_eq!(unsafe { digit::digit_to_char(digit) }, expected);
}

#[test]
#[cfg(feature = "write")]
fn digit_to_char_const_test() {
    digit_to_char::<10>(9, b'9');
    digit_to_char::<36>(10, b'A');
    digit_to_char::<36>(11, b'B');
}
