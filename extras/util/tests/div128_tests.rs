#![cfg(not(feature = "compact"))]
#![cfg(feature = "write")]

mod util;

use lexical_util::div128::u128_divrem;
use lexical_util::step::u64_step;
use proptest::{prop_assert_eq, proptest};

use crate::util::default_proptest_config;

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn u128_divrem_proptest(i in u128::MIN..u128::MAX) {
        let (hi, lo) = u128_divrem(i, 10);
        let step = u64_step(10);
        let d = 10u128.pow(step as u32);
        let expected = (i / d, (i % d) as u64);
        prop_assert_eq!((hi, lo), expected);
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u128_divrem_radix_proptest(i in u128::MIN..u128::MAX, radix in 2u32..=36) {
        // Simulate a const expr.
        let (hi, lo) = u128_divrem(i, radix);
        let step = u64_step(radix);
        let d = (radix as u128).pow(step as u32);
        let expected = (i / d, (i % d) as u64);
        prop_assert_eq!((hi, lo), expected);
    }
}
