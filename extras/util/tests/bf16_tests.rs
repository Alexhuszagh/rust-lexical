#![cfg(feature = "f16")]

mod util;

use lexical_util::bf16::bf16;
use proptest::prelude::*;

use crate::util::default_proptest_config;

default_quickcheck! {
    fn f32_roundtrip_quickcheck(x: u16) -> bool {
        let f = bf16::from_bits(x).as_f32_const();
        if f.is_nan() {
            bf16::from_f32_const(f).is_nan()
        } else {
            bf16::from_f32_const(f).to_bits() == x
        }
    }
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn f32_roundtrip_proptest(x in u16::MIN..u16::MAX) {
        let f = bf16::from_bits(x).to_f32_const();
        if f.is_nan() {
            prop_assert!(bf16::from_f32_const(f).is_nan());
        } else {
            prop_assert_eq!(bf16::from_f32_const(f).to_bits(), x);
        }
    }
}
