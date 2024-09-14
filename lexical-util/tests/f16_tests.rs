#![cfg(feature = "f16")]

mod util;

use lexical_util::f16::f16;
use lexical_util::num::Float;
use proptest::prelude::*;

use crate::util::default_proptest_config;

#[test]
fn as_f32_test() {
    assert_eq!(f16::from_bits(1).as_f32(), 0.000000059604645);
    assert_eq!(f16::ZERO.as_f32(), 0.0f32);
    assert_eq!(f16::ZERO.to_bits(), 0);
    assert_eq!(f16::ONE.as_f32(), 1.0f32);
    assert_eq!(f16::ONE.to_bits(), (15 << 10));
    assert_eq!(f16::TWO.as_f32(), 2.0f32);
    assert_eq!(f16::TWO.to_bits(), (16 << 10));
    assert_eq!(f16::from_bits(14 << 10).as_f32(), 0.5f32);
    assert!(f16::NAN.as_f32().is_nan());
    assert!(f16::INFINITY.as_f32().is_inf());
    assert!(f16::NEG_INFINITY.as_f32().is_inf());
}

#[test]
fn from_f32_test() {
    assert_eq!(f16::from_f32(2.980232e-08).to_bits(), 0);
    assert_eq!(f16::from_f32(2.9802322e-08).to_bits(), 0);
    assert_eq!(f16::from_f32(2.9802326e-08).to_bits(), 1);
    assert_eq!(f16::from_f32(5.960464e-08).to_bits(), 1);
    assert_eq!(f16::from_f32(5.9604645e-08).to_bits(), 1);
    assert_eq!(f16::from_f32(5.960465e-08).to_bits(), 1);
    assert!(f16::from_f32(f32::NAN).is_nan());
    assert!(f16::from_f32(f32::INFINITY).is_inf());
    assert!(f16::from_f32(f32::NEG_INFINITY).is_inf());
}

#[test]
fn math_tests() {
    assert_eq!(f16::ONE + f16::ONE, f16::TWO);
    assert_eq!(f16::ONE * f16::ONE, f16::ONE);
    assert_eq!(f16::ONE / f16::ONE, f16::ONE);
    assert_eq!(f16::ONE - f16::ONE, f16::ZERO);
    assert_eq!(f16::ONE % f16::ONE, f16::ZERO);
}

default_quickcheck! {
    fn f32_roundtrip_quickcheck(x: u16) -> bool {
        let f = f16::from_bits(x).as_f32();
        if f.is_nan() {
            f16::from_f32(f).is_nan()
        } else {
            f16::from_f32(f).to_bits() == x
        }
    }
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn f32_roundtrip_proptest(x in u16::MIN..u16::MAX) {
        let f = f16::from_bits(x).as_f32();
        if f.is_nan() {
            prop_assert!(f16::from_f32(f).is_nan());
        } else {
            prop_assert_eq!(f16::from_f32(f).to_bits(), x);
        }
    }
}
