//! Brain Floating Point implementation, a 16-bit type used in machine learning.
//!
//! bf16 is meant as an interchange format, and therefore there may be
//! rounding error in using it for fast-path algorithms. Since there
//! are no native operations using `bf16`, this is of minimal concern.

#![cfg(feature = "f16")]
#![doc(hidden)]

use core::cmp::Ordering;
use core::{fmt, ops};

/// Brain floating point type.
#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone)]
pub struct bf16 {
    /// Raw bitwise representation of the float as a 16-bit type.
    bits: u16,
}

unsafe impl Send for bf16 {
}
unsafe impl Sync for bf16 {
}

impl bf16 {
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
        // This is super easy, since we have the same exponent bits:
        // just need to shift left 16.
        f32::from_bits((self.bits as u32) << 16)
    }

    #[inline(always)]
    pub fn from_f32(value: f32) -> Self {
        // Same number of exponent bits, less mantissa bits: simple conversion.
        // We want to round to the nearest float, so we'll check if it's odd
        // and above or equal to halfway. This also properly handled inf, denormal,
        // and NaN cases, since they're effectively the same.
        let bits = value.to_bits();
        let truncated = bits as u16;
        let bf16_bits = (bits >> 16) as u16;

        let halfway = 1u16 << 15;
        let is_odd = bf16_bits % 2 == 1;
        let is_halfway = truncated == halfway;
        let is_above = truncated > halfway;
        let round_up = is_above || (is_halfway && is_odd);

        Self::from_bits(bf16_bits + round_up as u16)
    }
}

impl PartialEq for bf16 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.as_f32().eq(&other.as_f32())
    }
}

impl PartialOrd for bf16 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_f32().partial_cmp(&other.as_f32())
    }
}

impl fmt::Debug for bf16 {
    #[inline(always)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_f32().fmt(formatter)
    }
}

impl fmt::Display for bf16 {
    #[inline(always)]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_f32().fmt(formatter)
    }
}

impl ops::Add for bf16 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() + rhs.as_f32())
    }
}

impl ops::Div for bf16 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() / rhs.as_f32())
    }
}

impl ops::Mul for bf16 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() * rhs.as_f32())
    }
}

impl ops::Sub for bf16 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() - rhs.as_f32())
    }
}

impl ops::Rem for bf16 {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() % rhs.as_f32())
    }
}

impl ops::Neg for bf16 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::from_bits(self.bits ^ (1 << 15))
    }
}

impl ops::AddAssign for bf16 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::DivAssign for bf16 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl ops::MulAssign for bf16 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::SubAssign for bf16 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::RemAssign for bf16 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}
