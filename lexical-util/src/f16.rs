//! Half-precision IEEE-754 floating point implementation.
//!
//! f16 is meant as an interchange format, and therefore there may be
//! rounding error in using it for fast-path algorithms. Since there
//! are no native operations using `f16`, this is of minimal concern.
//!
//! Some of this code has been implemented from
//! [half-rs](https://github.com/starkat99/half-rs), to enable simple
//! conversions to and from f32.

#![cfg(feature = "f16")]
#![doc(hidden)]

use crate::num::Float;
use core::cmp::Ordering;
use core::{fmt, ops};

/// Half-precision IEEE-754 floating point type.
#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone)]
pub struct f16 {
    /// Raw bitwise representation of the float as a 16-bit type.
    bits: u16,
}

unsafe impl Send for f16 {
}
unsafe impl Sync for f16 {
}

impl f16 {
    #[inline(always)]
    pub const fn to_bits(self) -> u16 {
        self.bits
    }

    #[inline(always)]
    pub const fn from_bits(bits: u16) -> Self {
        Self {
            bits,
        }
    }

    #[inline(always)]
    pub fn as_f32(self) -> f32 {
        f16_to_f32(self)
    }

    #[inline(always)]
    pub fn from_f32(value: f32) -> Self {
        f32_to_f16(value)
    }
}

impl PartialEq for f16 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_f32().eq(&other.as_f32())
    }
}

impl PartialOrd for f16 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_f32().partial_cmp(&other.as_f32())
    }
}

impl fmt::Debug for f16 {
    #[inline(always)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_f32().fmt(formatter)
    }
}

impl fmt::Display for f16 {
    #[inline(always)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_f32().fmt(formatter)
    }
}

impl ops::Add for f16 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() + rhs.as_f32())
    }
}

impl ops::Div for f16 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() / rhs.as_f32())
    }
}

impl ops::Mul for f16 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() * rhs.as_f32())
    }
}

impl ops::Sub for f16 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() - rhs.as_f32())
    }
}

impl ops::Rem for f16 {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() % rhs.as_f32())
    }
}

impl ops::Neg for f16 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::from_bits(self.bits ^ (1 << 15))
    }
}

impl ops::AddAssign for f16 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::DivAssign for f16 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl ops::MulAssign for f16 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::SubAssign for f16 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::RemAssign for f16 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

// In the below functions, round to nearest, with ties to even.
// Let us call the most significant bit that will be shifted out the round_bit.
//
// Round up if either
//  a) Removed part > tie.
//     (mantissa & round_bit) != 0 && (mantissa & (round_bit - 1)) != 0
//  b) Removed part == tie, and retained part is odd.
//     (mantissa & round_bit) != 0 && (mantissa & (2 * round_bit)) != 0
// (If removed part == tie and retained part is even, do not round up.)
// These two conditions can be combined into one:
//     (mantissa & round_bit) != 0 && (mantissa & ((round_bit - 1) | (2 * round_bit))) != 0
// which can be simplified into
//     (mantissa & round_bit) != 0 && (mantissa & (3 * round_bit - 1)) != 0

fn f16_to_f32(half: f16) -> f32 {
    let man_shift = f32::MANTISSA_SIZE - f16::MANTISSA_SIZE;
    let f16_bias = f16::EXPONENT_BIAS - f16::MANTISSA_SIZE;
    let f32_bias = f32::EXPONENT_BIAS - f32::MANTISSA_SIZE;

    // Check for signed zero
    if half.bits & (f16::SIGN_MASK - 1) == 0 {
        return f32::from_bits((half.bits as u32) << 16);
    }

    let half_sign = (half.bits & f16::SIGN_MASK) as u32;
    let half_exp = (half.bits & f16::EXPONENT_MASK) as u32;
    let half_man = (half.bits & f16::MANTISSA_MASK) as u32;

    if half.is_nan() {
        return f32::from_bits((half_sign << 16) | 0x7FC0_0000u32 | (half_man << man_shift));
    } else if half.is_inf() {
        return f32::from_bits((half_sign << 16) | f32::INFINITY_BITS);
    }

    // Calculate single-precision components with adjusted exponent
    let sign = half_sign << 16;
    // Unbias exponent
    let unbiased_exp = ((half_exp as i32) >> f16::MANTISSA_SIZE) - f16_bias;

    // Check for subnormals, which will be normalized by adjusting exponent
    if half_exp == 0 {
        // Calculate how much to adjust the exponent by
        let e = (half_man as u16).leading_zeros() - (16 - f16::MANTISSA_SIZE as u32);

        // Rebias and adjust exponent
        let exp = (f32_bias as u32 - f16_bias as u32 - e) << f32::MANTISSA_SIZE;
        let man = (half_man << (f16_bias as u32 - 1 + e)) & f32::MANTISSA_MASK;
        return f32::from_bits(sign | exp | man);
    }

    // Rebias exponent for a normalized normal
    let exp = ((unbiased_exp + f32_bias) as u32) << f32::MANTISSA_SIZE;
    let man = (half_man & f16::MANTISSA_MASK as u32) << man_shift;
    f32::from_bits(sign | exp | man)
}

fn f32_to_f16(value: f32) -> f16 {
    let man_shift = f32::MANTISSA_SIZE - f16::MANTISSA_SIZE;
    let f16_bias = f16::EXPONENT_BIAS - f16::MANTISSA_SIZE;
    let f32_bias = f32::EXPONENT_BIAS - f32::MANTISSA_SIZE;

    // Convert to raw bytes
    let x = value.to_bits();

    // Extract IEEE754 components
    let sign = x & f32::SIGN_MASK;
    let exp = x & f32::EXPONENT_MASK;
    let man = x & f32::MANTISSA_MASK;

    // Check for all exponent bits being set, which is Infinity or NaN
    if value.is_nan() {
        return f16::from_bits((sign >> 16) as u16 | 0x7e00 | (man >> man_shift) as u16);
    } else if value.is_inf() {
        return f16::from_bits((sign >> 16) as u16 | f16::INFINITY_BITS);
    }

    // The number is normalized, start assembling half precision version
    let half_sign = sign >> 16;
    // Unbias the exponent, then bias for half precision
    let unbiased_exp = ((exp >> f32::MANTISSA_SIZE) as i32) - f32_bias;
    let half_exp = unbiased_exp + f16_bias;

    // Check for exponent overflow, return +infinity
    if unbiased_exp >= 0x1F {
        return f16::from_bits(half_sign as u16 | f16::INFINITY_BITS);
    }

    // Check for underflow
    if half_exp <= 0 {
        // Check mantissa for what we can do
        if f16_bias - 1 - half_exp > f32::MANTISSA_SIZE + 1 {
            // No rounding possibility, so this is a full underflow, return signed zero
            return f16::from_bits(half_sign as u16);
        }
        // Don't forget about hidden leading mantissa bit when assembling mantissa
        let man = man | f32::HIDDEN_BIT_MASK;
        let mut half_man = man >> (f16_bias - 1 - half_exp);
        // Check for rounding (see comment above functions)
        let round_bit = 1 << (man_shift - half_exp);
        if (man & round_bit) != 0 && (man & (3 * round_bit - 1)) != 0 {
            half_man += 1;
        }
        // No exponent for subnormals
        return f16::from_bits((half_sign | half_man) as u16);
    }

    // Rebias the exponent
    let half_exp = (half_exp as u32) << f16::MANTISSA_SIZE;
    let half_man = man >> man_shift;
    let round_bit = 1 << (man_shift - 1);
    if (man & round_bit) != 0 && (man & (3 * round_bit - 1)) != 0 {
        // Round it
        f16::from_bits(((half_sign | half_exp | half_man) + 1) as u16)
    } else {
        f16::from_bits((half_sign | half_exp | half_man) as u16)
    }
}
