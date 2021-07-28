#![cfg(not(feature = "compact"))]

mod util;

use lexical_parse_integer::algorithm;
use lexical_util::error::ErrorCode;
use lexical_util::iterator::Byte;
use lexical_util::noskip::AsNoSkip;
use proptest::prelude::*;
use util::to_format;

#[test]
fn test_is_4digits() {
    let value: u32 = 0x31_32_33_34;
    assert!(!algorithm::is_4digits::<{ to_format(4) }>(value));
    assert!(algorithm::is_4digits::<{ to_format(5) }>(value));
    assert!(algorithm::is_4digits::<{ to_format(10) }>(value));

    let value: u32 = 0x29_30_39_38;
    assert!(!algorithm::is_4digits::<{ to_format(10) }>(value));

    let value: u32 = 0x31_32_33_40;
    assert!(!algorithm::is_4digits::<{ to_format(10) }>(value));

    let value: u32 = 0x31_32_33_39;
    assert!(!algorithm::is_4digits::<{ to_format(9) }>(value));
    assert!(algorithm::is_4digits::<{ to_format(10) }>(value));
}

#[test]
fn test_parse_4digits() {
    assert_eq!(algorithm::parse_4digits::<{ to_format(10) }>(0x31_32_33_34), 4321);
    assert_eq!(algorithm::parse_4digits::<{ to_format(5) }>(0x31_32_33_34), 586);
    assert_eq!(algorithm::parse_4digits::<{ to_format(10) }>(0x36_37_38_39), 9876);
}

#[test]
fn test_try_parse_4digits() {
    const FORMAT: u128 = to_format(10);
    let parse = |digits: &[u8]| {
        let mut bytes = digits.noskip();
        algorithm::try_parse_4digits::<u32, _, FORMAT>(&mut bytes.integer_iter())
    };
    assert_eq!(parse(b"1234"), Some(1234));
    assert_eq!(parse(b"123"), None);
    assert_eq!(parse(b"123\x00"), None);
    assert_eq!(parse(b"123."), None);
    assert_eq!(parse(b"123_"), None);
    assert_eq!(parse(b"1234_"), Some(1234));
}

#[test]
fn test_is_8digits() {
    let value: u64 = 0x31_32_33_34_35_36_37_38;
    assert!(!algorithm::is_8digits::<{ to_format(4) }>(value));
    assert!(!algorithm::is_8digits::<{ to_format(5) }>(value));
    assert!(algorithm::is_8digits::<{ to_format(10) }>(value));

    let value: u64 = 0x29_30_31_32_33_34_35_36;
    assert!(!algorithm::is_8digits::<{ to_format(10) }>(value));

    let value: u64 = 0x30_31_32_33_34_35_36_40;
    assert!(!algorithm::is_8digits::<{ to_format(10) }>(value));

    let value: u64 = 0x31_32_33_34_35_36_37_39;
    assert!(!algorithm::is_8digits::<{ to_format(9) }>(value));
    assert!(algorithm::is_8digits::<{ to_format(10) }>(value));
}

#[test]
fn test_parse_8digits() {
    // 10000000
    let value: u64 = 0x30_30_30_30_30_30_30_31;
    assert_eq!(algorithm::parse_8digits::<{ to_format(10) }>(value), 10000000);
    assert_eq!(algorithm::parse_8digits::<{ to_format(5) }>(value), 78125);

    // 00000010
    let value: u64 = 0x30_31_30_30_30_30_30_30;
    assert_eq!(algorithm::parse_8digits::<{ to_format(10) }>(value), 10);
    assert_eq!(algorithm::parse_8digits::<{ to_format(5) }>(value), 5);

    // 12344321
    let value: u64 = 0x31_32_33_34_34_33_32_31;
    assert_eq!(algorithm::parse_8digits::<{ to_format(10) }>(value), 12344321);
    assert_eq!(algorithm::parse_8digits::<{ to_format(9) }>(value), 6052420);
    assert_eq!(algorithm::parse_8digits::<{ to_format(8) }>(value), 2738385);
    assert_eq!(algorithm::parse_8digits::<{ to_format(7) }>(value), 1120400);
    assert_eq!(algorithm::parse_8digits::<{ to_format(6) }>(value), 402745);
    assert_eq!(algorithm::parse_8digits::<{ to_format(5) }>(value), 121836);
}

#[test]
fn test_try_parse_8digits() {
    const FORMAT: u128 = to_format(10);
    let parse = |digits: &[u8]| {
        let mut bytes = digits.noskip();
        algorithm::try_parse_8digits::<u64, _, FORMAT>(&mut bytes.integer_iter())
    };

    assert_eq!(parse(b"12345678"), Some(12345678));
    assert_eq!(parse(b"1234567"), None);
    assert_eq!(parse(b"1234567\x00"), None);
    assert_eq!(parse(b"1234567."), None);
    assert_eq!(parse(b"1234567_"), None);
    assert_eq!(parse(b"12345678"), Some(12345678));
}

#[test]
fn algorithm_test() {
    const FORMAT: u128 = to_format(10);
    let parse_u32 = |digits: &[u8]| {
        let mut bytes = digits.noskip();
        algorithm::algorithm::<u32, _, FORMAT>(bytes.integer_iter())
    };
    let parse_i32 = |digits: &[u8]| {
        let mut bytes = digits.noskip();
        algorithm::algorithm::<i32, _, FORMAT>(bytes.integer_iter())
    };

    assert_eq!(parse_u32(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_u32(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_u32(b"-12345"), Err(ErrorCode::InvalidNegativeSign.into()));
    assert_eq!(parse_i32(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_i32(b"-12345"), Ok((-12345, 6)));
    assert_eq!(parse_i32(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_i32(b"+123.45"), Ok((123, 4)));
}

#[test]
fn algorithm_128_test() {
    const FORMAT: u128 = to_format(10);
    let parse_u128 = |digits: &[u8]| {
        let mut bytes = digits.noskip();
        algorithm::algorithm_128::<u128, _, FORMAT>(bytes.integer_iter())
    };
    let parse_i128 = |digits: &[u8]| {
        let mut bytes = digits.noskip();
        algorithm::algorithm_128::<i128, _, FORMAT>(bytes.integer_iter())
    };

    assert_eq!(parse_u128(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_u128(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_u128(b"-12345"), Err(ErrorCode::InvalidNegativeSign.into()));
    assert_eq!(parse_i128(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_i128(b"-12345"), Ok((-12345, 6)));
    assert_eq!(parse_i128(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_i128(b"+123.45"), Ok((123, 4)));
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
        let actual = algorithm::parse_4digits::<{ to_format(10) }>(v);
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
        let actual = algorithm::parse_8digits::<{ to_format(10) }>(v);
        let e1 = (a - 0x30) + 10 * (b - 0x30) + 100 * (c - 0x30) + 1000 * (d - 0x30);
        let e2 = (e - 0x30) + 10 * (f - 0x30) + 100 * (g - 0x30) + 1000 * (h - 0x30);
        let expected = e1 + 10000 * e2;
        prop_assert_eq!(actual, expected);
    }
}
