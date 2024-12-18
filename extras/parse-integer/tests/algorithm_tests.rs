#![cfg(not(feature = "compact"))]

mod util;

use lexical_parse_integer::algorithm;
use lexical_util::format::STANDARD;
use proptest::prelude::*;

use crate::util::default_proptest_config;

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
