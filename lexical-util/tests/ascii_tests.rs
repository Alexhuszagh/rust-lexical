use lexical_util::ascii;

#[test]
fn is_valid_ascii_test() {
    assert_eq!(ascii::is_valid_ascii(b'\x00'), false);
    assert_eq!(ascii::is_valid_ascii(b'\n'), true);
    assert_eq!(ascii::is_valid_ascii(b'\r'), true);
    assert_eq!(ascii::is_valid_ascii(b'\x1b'), false);
    assert_eq!(ascii::is_valid_ascii(b' '), true);
    assert_eq!(ascii::is_valid_ascii(b'0'), true);
    assert_eq!(ascii::is_valid_ascii(b'9'), true);
    assert_eq!(ascii::is_valid_ascii(b':'), true);
    assert_eq!(ascii::is_valid_ascii(b'A'), true);
    assert_eq!(ascii::is_valid_ascii(b'Z'), true);
    assert_eq!(ascii::is_valid_ascii(b']'), true);
    assert_eq!(ascii::is_valid_ascii(b'a'), true);
    assert_eq!(ascii::is_valid_ascii(b'z'), true);
    assert_eq!(ascii::is_valid_ascii(b'~'), true);
    assert_eq!(ascii::is_valid_ascii(b'\x7f'), false);
}

#[test]
fn is_valid_ascii_slice_test() {
    assert_eq!(ascii::is_valid_ascii_slice(b" 09a"), true);
    assert_eq!(ascii::is_valid_ascii_slice(b" 09a\x1b"), false);
}

#[test]
fn is_valid_letter_test() {
    assert_eq!(ascii::is_valid_letter(b'\x00'), false);
    assert_eq!(ascii::is_valid_letter(b'\n'), false);
    assert_eq!(ascii::is_valid_letter(b'\r'), false);
    assert_eq!(ascii::is_valid_letter(b'\x1b'), false);
    assert_eq!(ascii::is_valid_letter(b' '), false);
    assert_eq!(ascii::is_valid_letter(b'0'), false);
    assert_eq!(ascii::is_valid_letter(b'9'), false);
    assert_eq!(ascii::is_valid_letter(b':'), false);
    assert_eq!(ascii::is_valid_letter(b'A'), true);
    assert_eq!(ascii::is_valid_letter(b'Z'), true);
    assert_eq!(ascii::is_valid_letter(b']'), false);
    assert_eq!(ascii::is_valid_letter(b'a'), true);
    assert_eq!(ascii::is_valid_letter(b'z'), true);
    assert_eq!(ascii::is_valid_letter(b'~'), false);
    assert_eq!(ascii::is_valid_letter(b'\x7f'), false);
}

#[test]
fn is_valid_letter_slice_test() {
    assert_eq!(ascii::is_valid_letter_slice(b" 09a"), false);
    assert_eq!(ascii::is_valid_letter_slice(b"aZAz"), true);
}
