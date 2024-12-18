#![cfg(feature = "f16")]

use lexical_util::f16::f16;
use lexical_util::num::Float;

#[test]
fn as_f32_test() {
    assert_eq!(f16::from_bits(1).as_f32_const(), 0.000000059604645);
    assert_eq!(f16::ZERO.as_f32_const(), 0.0f32);
    assert_eq!(f16::ZERO.to_bits(), 0);
    assert_eq!(f16::ONE.as_f32_const(), 1.0f32);
    assert_eq!(f16::ONE.to_bits(), (15 << 10));
    assert_eq!(f16::TWO.as_f32_const(), 2.0f32);
    assert_eq!(f16::TWO.to_bits(), (16 << 10));
    assert_eq!(f16::from_bits(14 << 10).as_f32_const(), 0.5f32);
    assert!(f16::NAN.as_f32_const().is_nan());
    assert!(f16::INFINITY.as_f32_const().is_inf());
    assert!(f16::NEG_INFINITY.as_f32_const().is_inf());
}

#[test]
fn from_f32_test() {
    assert_eq!(f16::from_f32_const(2.980232e-08).to_bits(), 0);
    assert_eq!(f16::from_f32_const(2.9802322e-08).to_bits(), 0);
    assert_eq!(f16::from_f32_const(2.9802326e-08).to_bits(), 1);
    assert_eq!(f16::from_f32_const(5.960464e-08).to_bits(), 1);
    assert_eq!(f16::from_f32_const(5.9604645e-08).to_bits(), 1);
    assert_eq!(f16::from_f32_const(5.960465e-08).to_bits(), 1);
    assert!(f16::from_f32_const(f32::NAN).is_nan());
    assert!(f16::from_f32_const(f32::INFINITY).is_inf());
    assert!(f16::from_f32_const(f32::NEG_INFINITY).is_inf());
}

#[test]
fn math_tests() {
    assert_eq!(f16::ONE + f16::ONE, f16::TWO);
    assert_eq!(f16::ONE * f16::ONE, f16::ONE);
    assert_eq!(f16::ONE / f16::ONE, f16::ONE);
    assert_eq!(f16::ONE - f16::ONE, f16::ZERO);
    assert_eq!(f16::ONE % f16::ONE, f16::ZERO);
}
