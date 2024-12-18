#![cfg(feature = "f16")]

use lexical_util::bf16::bf16;
use lexical_util::num::Float;

#[test]
fn as_f32_test() {
    assert_eq!(bf16::from_bits(1).as_f32(), 9.18355e-41f32);
    assert_eq!(bf16::ZERO.as_f32_const(), 0.0f32);
    assert_eq!(bf16::ZERO.to_bits(), 0);
    assert_eq!(bf16::ONE.as_f32_const(), 1.0f32);
    assert_eq!(bf16::ONE.to_bits(), (127 << 7));
    assert_eq!(bf16::TWO.as_f32_const(), 2.0f32);
    assert_eq!(bf16::TWO.to_bits(), (128 << 7));
    assert_eq!(bf16::from_bits(126 << 7).as_f32_const(), 0.5f32);
    assert!(bf16::NAN.as_f32_const().is_nan());
    assert!(bf16::INFINITY.as_f32_const().is_inf());
    assert!(bf16::NEG_INFINITY.as_f32_const().is_inf());
}

#[test]
fn from_f32_test() {
    assert_eq!(bf16::from_f32_const(4.5917e-41f32).to_bits(), 0);
    assert_eq!(bf16::from_f32_const(4.5918e-41f32).to_bits(), 0);
    assert_eq!(bf16::from_f32_const(4.5919e-41f32).to_bits(), 1);
    assert_eq!(bf16::from_f32_const(9.18354e-41f32).to_bits(), 1);
    assert_eq!(bf16::from_f32_const(9.18355e-41f32).to_bits(), 1);
    assert_eq!(bf16::from_f32_const(9.18356e-41f32).to_bits(), 1);
    assert_eq!(bf16::from_f32_const(1.37752e-40f32).to_bits(), 1);
    assert_eq!(bf16::from_f32_const(1.37753e-40f32).to_bits(), 2);
    assert_eq!(bf16::from_f32_const(1.37754e-40f32).to_bits(), 2);
    assert!(bf16::from_f32_const(f32::NAN).is_nan());
    assert!(bf16::from_f32_const(f32::INFINITY).is_inf());
    assert!(bf16::from_f32_const(f32::NEG_INFINITY).is_inf());
}

#[test]
fn math_tests() {
    assert_eq!(bf16::ONE + bf16::ONE, bf16::TWO);
    assert_eq!(bf16::ONE * bf16::ONE, bf16::ONE);
    assert_eq!(bf16::ONE / bf16::ONE, bf16::ONE);
    assert_eq!(bf16::ONE - bf16::ONE, bf16::ZERO);
    assert_eq!(bf16::ONE % bf16::ONE, bf16::ZERO);
}
