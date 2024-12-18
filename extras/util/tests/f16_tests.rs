#![cfg(feature = "f16")]

mod util;

use lexical_util::f16::f16;
use proptest::prelude::*;

use crate::util::default_proptest_config;

default_quickcheck! {
    fn f32_roundtrip_quickcheck(x: u16) -> bool {
        let f = f16::from_bits(x).as_f32_const();
        if f.is_nan() {
            f16::from_f32_const(f).is_nan()
        } else {
            f16::from_f32_const(f).to_bits() == x
        }
    }
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn f32_roundtrip_proptest(x in u16::MIN..u16::MAX) {
        let f = f16::from_bits(x).as_f32_const();
        if f.is_nan() {
            prop_assert!(f16::from_f32_const(f).is_nan());
        } else {
            prop_assert_eq!(f16::from_f32_const(f).to_bits(), x);
        }
    }
}
