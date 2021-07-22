use lexical_util::div128;
use proptest::{prop_assert_eq, proptest};

proptest! {
    #[test]
    fn u128_divrem_proptest(i in u128::min_value()..u128::max_value()) {
        let (d, _, d_ctlz) = div128::u128_divisor(10);
        let expected = (i / d as u128, (i % d as u128) as u64);
        let actual = div128::u128_divrem(i, d, d_ctlz);
        prop_assert_eq!(actual, expected);
    }
}
