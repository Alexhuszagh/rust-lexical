#![cfg(feature = "write")]

use lexical_util::div128;
use proptest::{prop_assert_eq, proptest};

proptest! {
    #[test]
    fn u128_divrem_proptest(i in u128::min_value()..u128::max_value()) {
        let (hi, lo, step) = div128::u128_divrem(i, 10);
        let d = 10u128.pow(step as u32);
        let expected = (i / d, (i % d) as u64);
        prop_assert_eq!((hi, lo), expected);
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u128_divrem_radix_proptest(i in u128::min_value()..u128::max_value(), radix in 2u32..=36) {
        let (hi, lo, step) = div128::u128_divrem(i, radix);
        let d = (radix as u128).pow(step as u32);
        let expected = (i / d, (i % d) as u64);
        prop_assert_eq!((hi, lo), expected);
    }
}
