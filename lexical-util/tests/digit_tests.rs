#![cfg(any(feature = "parse", feature = "write"))]

use lexical_util::digit;

#[cfg(feature = "parse")]
fn char_to_digit(c: u8, radix: u32, expected: Option<u32>) {
    assert_eq!(digit::char_to_digit_const(c, radix), expected);
    assert_eq!(digit::char_to_digit(c, radix), expected);
}

#[test]
#[cfg(feature = "parse")]
fn char_to_digit_test() {
    char_to_digit(b'0', 2, Some(0));
    char_to_digit(b'1', 2, Some(1));
    char_to_digit(b'9', 2, None);
    char_to_digit(b'A', 2, None);
    char_to_digit(b'Z', 2, None);

    char_to_digit(b'0', 10, Some(0));
    char_to_digit(b'1', 10, Some(1));
    char_to_digit(b'9', 10, Some(9));
    char_to_digit(b'A', 10, None);
    char_to_digit(b'Z', 10, None);

    char_to_digit(b'0', 16, Some(0));
    char_to_digit(b'1', 16, Some(1));
    char_to_digit(b'9', 16, Some(9));
    char_to_digit(b'A', 16, Some(10));
    char_to_digit(b'Z', 16, None);

    // char_to_digit doesn't care about the radix.
    // check some more comprehensive cases.
    for c in b'0'..=b'9' {
        char_to_digit(c, 10, Some(c as u32 - b'0' as u32));
    }
    char_to_digit(0x29, 10, None);
    char_to_digit(0x3A, 10, None);
    char_to_digit(0x59, 10, None);

    for c in b'0'..=b'8' {
        char_to_digit(c, 9, Some(c as u32 - b'0' as u32));
    }
    char_to_digit(0x29, 9, None);
    char_to_digit(0x39, 9, None);
    char_to_digit(0x3A, 9, None);
    char_to_digit(0x59, 9, None);

    for c in b'0'..=b'9' {
        char_to_digit(c, 16, Some(c as u32 - b'0' as u32));
    }
    for c in b'a'..=b'f' {
        char_to_digit(c, 16, Some(c as u32 - b'a' as u32 + 10));
    }
    for c in b'A'..=b'F' {
        char_to_digit(c, 16, Some(c as u32 - b'A' as u32 + 10));
    }
    char_to_digit(0x29, 16, None);
    char_to_digit(0x40, 16, None);
    char_to_digit(0x3A, 16, None);
    char_to_digit(0x59, 16, None);
    char_to_digit(0x41, 16, Some(10));
    char_to_digit(0x47, 16, None);
    char_to_digit(0x5A, 16, None);
    char_to_digit(0x61, 16, Some(10));
    char_to_digit(0x67, 16, None);
    char_to_digit(0x7A, 16, None);
}

#[cfg(feature = "parse")]
fn char_is_digit(c: u8, radix: u32, expected: bool) {
    assert_eq!(digit::char_is_digit_const(c, radix), expected);
    assert_eq!(digit::char_is_digit(c, radix), expected);
}

#[test]
#[cfg(feature = "parse")]
fn char_is_digit_const_test() {
    char_is_digit(b'0', 2, true);
    char_is_digit(b'1', 2, true);
    char_is_digit(b'9', 2, false);
    char_is_digit(b'A', 2, false);
    char_is_digit(b'Z', 2, false);

    char_is_digit(b'0', 10, true);
    char_is_digit(b'1', 10, true);
    char_is_digit(b'9', 10, true);
    char_is_digit(b'A', 10, false);
    char_is_digit(b'Z', 10, false);

    char_is_digit(b'0', 16, true);
    char_is_digit(b'1', 16, true);
    char_is_digit(b'9', 16, true);
    char_is_digit(b'A', 16, true);
    char_is_digit(b'Z', 16, false);
}

#[cfg(feature = "write")]
fn digit_to_char(digit: u32, radix: u32, expected: u8) {
    assert_eq!(digit::digit_to_char_const(digit, radix), expected);
    assert_eq!(digit::digit_to_char(digit), expected);
}

#[test]
#[cfg(feature = "write")]
fn digit_to_char_const_test() {
    digit_to_char(9, 10, b'9');
    digit_to_char(10, 36, b'A');
    digit_to_char(11, 36, b'B');
}
