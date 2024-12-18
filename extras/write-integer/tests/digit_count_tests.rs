#![cfg(not(feature = "compact"))]

mod util;

use lexical_write_integer::decimal::DecimalCount;
use lexical_write_integer::digit_count::{self, DigitCount};
use proptest::prelude::*;

use crate::util::default_proptest_config;

fn slow_log2(x: u32) -> usize {
    // Slow approach to calculating a log2, using floats.
    if x == 0 {
        0
    } else {
        (x as f64).log2().floor() as usize
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
