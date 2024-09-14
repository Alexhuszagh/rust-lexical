#![cfg(not(feature = "compact"))]

mod util;

use lexical_parse_integer::algorithm;
use lexical_parse_integer::options::SMALL_NUMBERS;
use lexical_util::format::STANDARD;
use lexical_util::iterator::AsBytes;
use proptest::prelude::*;
#[cfg(feature = "power-of-two")]
use util::from_radix;

use crate::util::default_proptest_config;

#[test]
fn test_is_4digits() {
    let value: u32 = 0x31_32_33_34;
    #[cfg(feature = "power-of-two")]
    assert!(!algorithm::is_4digits::<{ from_radix(4) }>(value));
    #[cfg(feature = "radix")]
    assert!(algorithm::is_4digits::<{ from_radix(5) }>(value));
    assert!(algorithm::is_4digits::<{ STANDARD }>(value));

    let value: u32 = 0x29_30_39_38;
    assert!(!algorithm::is_4digits::<{ STANDARD }>(value));

    let value: u32 = 0x31_32_33_40;
    assert!(!algorithm::is_4digits::<{ STANDARD }>(value));

    let value: u32 = 0x31_32_33_39;
    #[cfg(feature = "radix")]
    assert!(!algorithm::is_4digits::<{ from_radix(9) }>(value));
    assert!(algorithm::is_4digits::<{ STANDARD }>(value));
}

#[test]
fn test_parse_4digits() {
    assert_eq!(algorithm::parse_4digits::<{ STANDARD }>(0x31_32_33_34), 4321);
    #[cfg(feature = "radix")]
    assert_eq!(algorithm::parse_4digits::<{ from_radix(5) }>(0x31_32_33_34), 586);
    assert_eq!(algorithm::parse_4digits::<{ STANDARD }>(0x36_37_38_39), 9876);
}

#[test]
fn test_try_parse_4digits() {
    let parse = |bytes: &[u8]| {
        let mut digits = bytes.bytes::<{ STANDARD }>();
        algorithm::try_parse_4digits::<u32, _, STANDARD>(&mut digits.integer_iter())
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
    #[cfg(feature = "power-of-two")]
    assert!(!algorithm::is_8digits::<{ from_radix(4) }>(value));
    #[cfg(feature = "radix")]
    assert!(!algorithm::is_8digits::<{ from_radix(5) }>(value));
    assert!(algorithm::is_8digits::<{ STANDARD }>(value));

    let value: u64 = 0x29_30_31_32_33_34_35_36;
    assert!(!algorithm::is_8digits::<{ STANDARD }>(value));

    let value: u64 = 0x30_31_32_33_34_35_36_40;
    assert!(!algorithm::is_8digits::<{ STANDARD }>(value));

    let value: u64 = 0x31_32_33_34_35_36_37_39;
    #[cfg(feature = "radix")]
    assert!(!algorithm::is_8digits::<{ from_radix(9) }>(value));
    assert!(algorithm::is_8digits::<{ STANDARD }>(value));
}

#[test]
fn test_parse_8digits() {
    // 10000000
    let value: u64 = 0x30_30_30_30_30_30_30_31;
    assert_eq!(algorithm::parse_8digits::<{ STANDARD }>(value), 10000000);
    #[cfg(feature = "radix")]
    assert_eq!(algorithm::parse_8digits::<{ from_radix(5) }>(value), 78125);

    // 00000010
    let value: u64 = 0x30_31_30_30_30_30_30_30;
    assert_eq!(algorithm::parse_8digits::<{ STANDARD }>(value), 10);
    #[cfg(feature = "radix")]
    assert_eq!(algorithm::parse_8digits::<{ from_radix(5) }>(value), 5);

    // 12344321
    let value: u64 = 0x31_32_33_34_34_33_32_31;
    assert_eq!(algorithm::parse_8digits::<{ STANDARD }>(value), 12344321);
    #[cfg(feature = "power-of-two")]
    assert_eq!(algorithm::parse_8digits::<{ from_radix(8) }>(value), 2738385);

    #[cfg(feature = "radix")]
    {
        assert_eq!(algorithm::parse_8digits::<{ from_radix(9) }>(value), 6052420);
        assert_eq!(algorithm::parse_8digits::<{ from_radix(7) }>(value), 1120400);
        assert_eq!(algorithm::parse_8digits::<{ from_radix(6) }>(value), 402745);
        assert_eq!(algorithm::parse_8digits::<{ from_radix(5) }>(value), 121836);
    }
}

#[test]
fn test_try_parse_8digits() {
    let parse = |bytes: &[u8]| {
        let mut digits = bytes.bytes::<{ STANDARD }>();
        algorithm::try_parse_8digits::<u64, _, STANDARD>(&mut digits.integer_iter())
    };

    assert_eq!(parse(b"12345678"), Some(12345678));
    assert_eq!(parse(b"1234567"), None);
    assert_eq!(parse(b"1234567\x00"), None);
    assert_eq!(parse(b"1234567."), None);
    assert_eq!(parse(b"1234567_"), None);
    assert_eq!(parse(b"12345678"), Some(12345678));
}

#[cfg(feature = "power-of-two")]
macro_rules! parse_radix {
    ($i:literal) => {
        |bytes: &[u8]| {
            algorithm::algorithm_partial::<u32, { from_radix($i) }>(bytes, &SMALL_NUMBERS)
        }
    };
}

#[test]
fn algorithm_test() {
    let parse_u32 =
        |bytes: &[u8]| algorithm::algorithm_partial::<u32, STANDARD>(bytes, &SMALL_NUMBERS);
    let parse_i32 =
        |bytes: &[u8]| algorithm::algorithm_partial::<i32, STANDARD>(bytes, &SMALL_NUMBERS);

    assert_eq!(parse_u32(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_u32(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_u32(b"-12345"), Ok((0, 0)));
    assert_eq!(parse_i32(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_i32(b"-12345"), Ok((-12345, 6)));
    assert_eq!(parse_i32(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_i32(b"+123.45"), Ok((123, 4)));

    // Need to try with other radixes here, especially to ensure no regressions with
    // #71. Issue: https://github.com/Alexhuszagh/rust-lexical/issues/71
    #[cfg(feature = "power-of-two")]
    {
        // This should try to invoke `parse_4digits` since it's more than
        // 4 digits, and unsigned.
        assert_eq!(parse_radix!(4)(b"12345"), Ok((27, 3)));
        assert_eq!(parse_radix!(8)(b"12345"), Ok((5349, 5)));
        assert_eq!(parse_radix!(16)(b"12345"), Ok((74565, 5)));
        assert_eq!(parse_radix!(32)(b"12345"), Ok((1117317, 5)));
    }

    #[cfg(feature = "radix")]
    {
        assert_eq!(parse_radix!(6)(b"12345"), Ok((1865, 5)));
        assert_eq!(parse_radix!(12)(b"12345"), Ok((24677, 5)));
        assert_eq!(parse_radix!(24)(b"12345"), Ok((361253, 5)));
    }
}

#[test]
fn algorithm_128_test() {
    let parse_u128 =
        |bytes: &[u8]| algorithm::algorithm_partial::<u128, STANDARD>(bytes, &SMALL_NUMBERS);
    let parse_i128 =
        |bytes: &[u8]| algorithm::algorithm_partial::<i128, STANDARD>(bytes, &SMALL_NUMBERS);

    assert_eq!(parse_u128(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_u128(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_u128(b"-12345"), Ok((0, 0)));
    assert_eq!(parse_i128(b"12345"), Ok((12345, 5)));
    assert_eq!(parse_i128(b"-12345"), Ok((-12345, 6)));
    assert_eq!(parse_i128(b"+12345"), Ok((12345, 6)));
    assert_eq!(parse_i128(b"+123.45"), Ok((123, 4)));
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn parse_4digits_proptest(
        a in 0x30u32..0x39,
        b in 0x30u32..0x39,
        c in 0x30u32..0x39,
        d in 0x30u32..0x39,
    )
    {
        let v = (a << 24) | (b << 16) | (c << 8) | d;
        let actual = algorithm::parse_4digits::<{ STANDARD }>(v);
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
        let actual = algorithm::parse_8digits::<{ STANDARD }>(v);
        let e1 = (a - 0x30) + 10 * (b - 0x30) + 100 * (c - 0x30) + 1000 * (d - 0x30);
        let e2 = (e - 0x30) + 10 * (f - 0x30) + 100 * (g - 0x30) + 1000 * (h - 0x30);
        let expected = e1 + 10000 * e2;
        prop_assert_eq!(actual, expected);
    }
}
