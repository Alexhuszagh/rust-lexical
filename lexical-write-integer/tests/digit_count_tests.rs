#![cfg(not(feature = "compact"))]

mod util;

use lexical_write_integer::decimal::DecimalCount;
use lexical_write_integer::digit_count::{self, DigitCount};
use proptest::prelude::*;

use crate::util::default_proptest_config;

#[test]
fn fast_log2_test() {
    // Check the first, even if illogical case works.
    assert_eq!(digit_count::fast_log2(0u32), 0);
    assert_eq!(digit_count::fast_log2(1u32), 0);
    assert_eq!(digit_count::fast_log2(2u32), 1);
    assert_eq!(digit_count::fast_log2(3u32), 1);

    assert_eq!(digit_count::fast_log2((1u32 << 16) - 1), 15);
    assert_eq!(digit_count::fast_log2(1u32 << 16), 16);
    assert_eq!(digit_count::fast_log2((1u32 << 16) + 1), 16);

    assert_eq!(digit_count::fast_log2(u32::MAX), 31);
}

fn slow_log2(x: u32) -> usize {
    // Slow approach to calculating a log2, using floats.
    if x == 0 {
        0
    } else {
        (x as f64).log2().floor() as usize
    }
}

#[test]
fn base10_count_test() {
    assert_eq!(1, 0u32.digit_count(10));
    assert_eq!(1, 9u32.digit_count(10));
    assert_eq!(2, 10u32.digit_count(10));
    assert_eq!(2, 11u32.digit_count(10));
    assert_eq!(2, 99u32.digit_count(10));
    assert_eq!(3, 100u32.digit_count(10));
    assert_eq!(3, 101u32.digit_count(10));
}

#[test]
fn base2_count_test() {
    assert_eq!(1, 0u32.digit_count(2));
    assert_eq!(1, 1u32.digit_count(2));
    assert_eq!(2, 2u32.digit_count(2));
    assert_eq!(2, 3u32.digit_count(2));
    assert_eq!(3, 4u32.digit_count(2));

    if cfg!(feature = "power-of-two") {
        for i in 1usize..=127 {
            let value = 2u128.pow(i as u32);
            assert_eq!(i + 1, value.digit_count(2));
            assert_eq!(i + 1, (value + 1).digit_count(2));
            assert_eq!(i, (value - 1).digit_count(2));
        }
    }
}

#[test]
fn base4_count_test() {
    assert_eq!(1, 0u32.digit_count(4));
    assert_eq!(1, 1u32.digit_count(4));
    assert_eq!(1, 3u32.digit_count(4));
    assert_eq!(2, 4u32.digit_count(4));
    assert_eq!(2, 5u32.digit_count(4));
    assert_eq!(2, 15u32.digit_count(4));
    assert_eq!(3, 16u32.digit_count(4));
    assert_eq!(3, 17u32.digit_count(4));

    if cfg!(feature = "power-of-two") {
        for i in 1usize..=63 {
            let value = 4u128.pow(i as u32);
            assert_eq!(i + 1, value.digit_count(4));
            assert_eq!(i + 1, (value + 1).digit_count(4));
            assert_eq!(i, (value - 1).digit_count(4));

            let halfway = value + 2u128.pow(i as u32);
            assert_eq!(i + 1, halfway.digit_count(4));
            assert_eq!(i + 1, (halfway + 1).digit_count(4));
            assert_eq!(i + 1, (halfway - 1).digit_count(4));
        }
    }
}

#[test]
fn base8_count_test() {
    assert_eq!(1, 0u32.digit_count(8));
    assert_eq!(1, 1u32.digit_count(8));
    assert_eq!(1, 7u32.digit_count(8));
    assert_eq!(2, 8u32.digit_count(8));
    assert_eq!(2, 9u32.digit_count(8));
    assert_eq!(2, 63u32.digit_count(8));
    assert_eq!(3, 64u32.digit_count(8));
    assert_eq!(3, 65u32.digit_count(8));

    if cfg!(feature = "power-of-two") {
        for i in 1usize..=31 {
            let value = 8u128.pow(i as u32);
            assert_eq!(i + 1, value.digit_count(8));
            assert_eq!(i + 1, (value + 1).digit_count(8));
            assert_eq!(i, (value - 1).digit_count(8));

            let halfway = value + 4u128.pow(i as u32);
            assert_eq!(i + 1, halfway.digit_count(8));
            assert_eq!(i + 1, (halfway + 1).digit_count(8));
            assert_eq!(i + 1, (halfway - 1).digit_count(8));
        }
    }
}

#[test]
fn base16_count_test() {
    assert_eq!(1, 0u32.digit_count(16));
    assert_eq!(1, 1u32.digit_count(16));
    assert_eq!(1, 15u32.digit_count(16));
    assert_eq!(2, 16u32.digit_count(16));
    assert_eq!(2, 17u32.digit_count(16));
    assert_eq!(2, 255u32.digit_count(16));
    assert_eq!(3, 256u32.digit_count(16));
    assert_eq!(3, 257u32.digit_count(16));

    if cfg!(feature = "power-of-two") {
        for i in 1usize..=15 {
            let value = 16u128.pow(i as u32);
            assert_eq!(i + 1, value.digit_count(16));
            assert_eq!(i + 1, (value + 1).digit_count(16));
            assert_eq!(i, (value - 1).digit_count(16));

            let halfway = value + 8u128.pow(i as u32);
            assert_eq!(i + 1, halfway.digit_count(16));
            assert_eq!(i + 1, (halfway + 1).digit_count(16));
            assert_eq!(i + 1, (halfway - 1).digit_count(16));
        }
    }
}

#[test]
fn base32_count_test() {
    assert_eq!(1, 0u32.digit_count(32));
    assert_eq!(1, 1u32.digit_count(32));
    assert_eq!(1, 31u32.digit_count(32));
    assert_eq!(2, 32u32.digit_count(32));
    assert_eq!(2, 33u32.digit_count(32));
    assert_eq!(2, 1023u32.digit_count(32));
    assert_eq!(3, 1024u32.digit_count(32));
    assert_eq!(3, 1025u32.digit_count(32));

    if cfg!(feature = "power-of-two") {
        for i in 1usize..=7 {
            let value = 32u128.pow(i as u32);
            assert_eq!(i + 1, value.digit_count(32));
            assert_eq!(i + 1, (value + 1).digit_count(32));
            assert_eq!(i, (value - 1).digit_count(32));

            let halfway = value + 16u128.pow(i as u32);
            assert_eq!(i + 1, halfway.digit_count(32));
            assert_eq!(i + 1, (halfway + 1).digit_count(32));
            assert_eq!(i + 1, (halfway - 1).digit_count(32));
        }
    }
}

default_quickcheck! {
    fn decimal_count_quickcheck(x: u32) -> bool {
        x.digit_count(10) == x.decimal_count()
    }

    fn fast_log2_quickcheck(x: u32) -> bool {
        slow_log2(x) == digit_count::fast_log2(x)
    }
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn decimal_slow_u64_test(x: u64) {
        prop_assert_eq!(x.digit_count(10), x.slow_digit_count(10));
    }

    #[test]
    fn basen_slow_u64_test(x: u64, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        prop_assert_eq!(x.digit_count(radix), x.slow_digit_count(radix));
    }

    #[test]
    fn decimal_slow_u128_test(x: u128) {
        prop_assert_eq!(x.digit_count(10), x.slow_digit_count(10));
    }

    #[test]
    #[cfg(feature = "power-of-two")]
    fn basen_slow_u128_test(x: u128, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        prop_assert_eq!(x.digit_count(radix), x.slow_digit_count(radix));
    }
}

#[rustversion::since(1.67)]
macro_rules! ilog {
    ($x:ident, $radix:expr) => {{
        if $x > 0 {
            $x.ilog($radix as _) as usize
        } else {
            0usize
        }
    }};
}

#[rustversion::since(1.67)]
proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn basen_u64_test(x: u64, radix in 2u32..=36) {
        prop_assert_eq!(x.digit_count(radix), ilog!(x, radix) + 1);
    }

    #[test]
    #[cfg(feature = "radix")]
    fn basen_u128_test(x: u128, radix in 2u32..=36) {
        prop_assert_eq!(x.digit_count(radix), ilog!(x, radix) + 1);
    }

    #[test]
    #[cfg(all(feature = "power-of-two", not(feature = "radix")))]
    fn basen_u128_test(x: u128, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        prop_assert_eq!(x.digit_count(radix), ilog!(x, radix) + 1);
    }

    #[test]
    #[cfg(not(feature = "power-of-two"))]
    fn basen_u128_test(x: u128) {
        prop_assert_eq!(x.digit_count(10), ilog!(x, 10) + 1);
    }
}
