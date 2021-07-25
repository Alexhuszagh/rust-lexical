use lexical_parse_integer::algorithm;
use proptest::prelude::*;

#[test]
fn char_to_digit_test() {
    // char_to_digit doesn't care about the radix.
    for c in b'0'..=b'9' {
        assert_eq!(algorithm::char_to_digit::<10>(c), Some(c as u32 - b'0' as u32));
    }
    assert_eq!(algorithm::char_to_digit::<10>(0x29), None);
    assert_eq!(algorithm::char_to_digit::<10>(0x3A), None);
    assert_eq!(algorithm::char_to_digit::<10>(0x59), None);

    for c in b'0'..=b'8' {
        assert_eq!(algorithm::char_to_digit::<9>(c), Some(c as u32 - b'0' as u32));
    }
    assert_eq!(algorithm::char_to_digit::<9>(0x29), None);
    assert_eq!(algorithm::char_to_digit::<9>(0x39), None);
    assert_eq!(algorithm::char_to_digit::<9>(0x3A), None);
    assert_eq!(algorithm::char_to_digit::<9>(0x59), None);

    for c in b'0'..=b'9' {
        assert_eq!(algorithm::char_to_digit::<16>(c), Some(c as u32 - b'0' as u32));
    }
    for c in b'a'..=b'f' {
        assert_eq!(algorithm::char_to_digit::<16>(c), Some(c as u32 - b'a' as u32 + 10));
    }
    for c in b'A'..=b'F' {
        assert_eq!(algorithm::char_to_digit::<16>(c), Some(c as u32 - b'A' as u32 + 10));
    }
    assert_eq!(algorithm::char_to_digit::<16>(0x29), None);
    assert_eq!(algorithm::char_to_digit::<16>(0x40), None);
    assert_eq!(algorithm::char_to_digit::<16>(0x3A), None);
    assert_eq!(algorithm::char_to_digit::<16>(0x59), None);
    assert_eq!(algorithm::char_to_digit::<16>(0x41), Some(10));
    assert_eq!(algorithm::char_to_digit::<16>(0x47), None);
    assert_eq!(algorithm::char_to_digit::<16>(0x5A), None);
    assert_eq!(algorithm::char_to_digit::<16>(0x61), Some(10));
    assert_eq!(algorithm::char_to_digit::<16>(0x67), None);
    assert_eq!(algorithm::char_to_digit::<16>(0x7A), None);
}

#[test]
fn test_is_4digits() {
    let value: u32 = 0x31_32_33_34;
    assert!(!algorithm::is_4digits::<4>(value));
    assert!(algorithm::is_4digits::<5>(value));
    assert!(algorithm::is_4digits::<10>(value));

    let value: u32 = 0x29_30_39_38;
    assert!(!algorithm::is_4digits::<10>(value));

    let value: u32 = 0x31_32_33_40;
    assert!(!algorithm::is_4digits::<10>(value));

    let value: u32 = 0x31_32_33_39;
    assert!(!algorithm::is_4digits::<9>(value));
    assert!(algorithm::is_4digits::<10>(value));
}

#[test]
fn test_parse_4digits() {
    assert_eq!(algorithm::parse_4digits::<10>(0x31_32_33_34), 4321);
    assert_eq!(algorithm::parse_4digits::<5>(0x31_32_33_34), 586);
    assert_eq!(algorithm::parse_4digits::<10>(0x36_37_38_39), 9876);
}

#[test]
fn test_try_parse_4digits() {
    assert_eq!(algorithm::try_parse_4digits::<u32, _, 10>(&mut b"1234".iter()), Some(1234));
    assert_eq!(algorithm::try_parse_4digits::<u32, _, 10>(&mut b"123".iter()), None);
    assert_eq!(algorithm::try_parse_4digits::<u32, _, 10>(&mut b"123\x00".iter()), None);
    assert_eq!(algorithm::try_parse_4digits::<u32, _, 10>(&mut b"123.".iter()), None);
    assert_eq!(algorithm::try_parse_4digits::<u32, _, 10>(&mut b"123_".iter()), None);
    assert_eq!(algorithm::try_parse_4digits::<u32, _, 10>(&mut b"1234_".iter()), Some(1234));
}

#[test]
fn test_is_8digits() {
    let value: u64 = 0x31_32_33_34_35_36_37_38;
    assert!(!algorithm::is_8digits::<4>(value));
    assert!(!algorithm::is_8digits::<5>(value));
    assert!(algorithm::is_8digits::<10>(value));

    let value: u64 = 0x29_30_31_32_33_34_35_36;
    assert!(!algorithm::is_8digits::<10>(value));

    let value: u64 = 0x30_31_32_33_34_35_36_40;
    assert!(!algorithm::is_8digits::<10>(value));

    let value: u64 = 0x31_32_33_34_35_36_37_39;
    assert!(!algorithm::is_8digits::<9>(value));
    assert!(algorithm::is_8digits::<10>(value));
}

#[test]
fn test_parse_8digits() {
    // 10000000
    let value: u64 = 0x30_30_30_30_30_30_30_31;
    assert_eq!(algorithm::parse_8digits::<10>(value), 10000000);
    assert_eq!(algorithm::parse_8digits::<5>(value), 78125);

    // 00000010
    let value: u64 = 0x30_31_30_30_30_30_30_30;
    assert_eq!(algorithm::parse_8digits::<10>(value), 10);
    assert_eq!(algorithm::parse_8digits::<5>(value), 5);

    // 12344321
    let value: u64 = 0x31_32_33_34_34_33_32_31;
    assert_eq!(algorithm::parse_8digits::<10>(value), 12344321);
    assert_eq!(algorithm::parse_8digits::<9>(value), 6052420);
    assert_eq!(algorithm::parse_8digits::<8>(value), 2738385);
    assert_eq!(algorithm::parse_8digits::<7>(value), 1120400);
    assert_eq!(algorithm::parse_8digits::<6>(value), 402745);
    assert_eq!(algorithm::parse_8digits::<5>(value), 121836);
}

#[test]
fn test_try_parse_8digits() {
    assert_eq!(algorithm::try_parse_8digits::<u64, _, 10>(&mut b"12345678".iter()), Some(12345678));
    assert_eq!(algorithm::try_parse_8digits::<u64, _, 10>(&mut b"1234567".iter()), None);
    assert_eq!(algorithm::try_parse_8digits::<u64, _, 10>(&mut b"1234567\x00".iter()), None);
    assert_eq!(algorithm::try_parse_8digits::<u64, _, 10>(&mut b"1234567.".iter()), None);
    assert_eq!(algorithm::try_parse_8digits::<u64, _, 10>(&mut b"1234567_".iter()), None);
    assert_eq!(algorithm::try_parse_8digits::<u64, _, 10>(&mut b"12345678".iter()), Some(12345678));
}

proptest! {
    #[test]
    fn parse_4digits_proptest(
        a in 0x30u32..0x39,
        b in 0x30u32..0x39,
        c in 0x30u32..0x39,
        d in 0x30u32..0x39,
    )
    {
        let v = (a << 24) | (b << 16) | (c << 8) | d;
        let actual = algorithm::parse_4digits::<10>(v);
        let expected = (a - 0x30) + 10 * (b - 0x30) + 100 * (c - 0x30) + 1000 * (d - 0x30);
        prop_assert_eq!(actual, expected);
    }

    #[test]
    fn parse_8digits_proptest(
        a in 0x30u64..0x39,
        b in 0x30u64..0x39,
        c in 0x30u64..0x39,
        d in 0x30u64..0x39,
        e in 0x30u64..0x39,
        f in 0x30u64..0x39,
        g in 0x30u64..0x39,
        h in 0x30u64..0x39,
    )
    {
        let v1 = (a << 24) | (b << 16) | (c << 8) | d;
        let v2 = (e << 24) | (f << 16) | (g << 8) | h;
        let v = (v1 << 32) | v2;
        let actual = algorithm::parse_8digits::<10>(v);
        let e1 = (a - 0x30) + 10 * (b - 0x30) + 100 * (c - 0x30) + 1000 * (d - 0x30);
        let e2 = (e - 0x30) + 10 * (f - 0x30) + 100 * (g - 0x30) + 1000 * (h - 0x30);
        let expected = e1 + 10000 * e2;
        prop_assert_eq!(actual, expected);
    }
}
