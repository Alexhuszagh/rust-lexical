//! Brain Floating Point implementation, a 16-bit type used in machine learning.
//!
//! bf16 is meant as an interchange format, and therefore there may be
//! rounding error in using it for fast-path algorithms. Since there
//! are no native operations using `bf16`, this is of minimal concern.
//!
//! Most of this code has been implemented from [`half`], to enable simple
//! conversions to and from f32. This is provided as standalone code
//! to avoid any external dependencies and avoid the serialization logic
//! for serde, etc.
//!
//! There is no unsafety in this module other than a manual implementation
//! of sync/send for a primitive type.
//!
//! The documentation and implementation for other parts of this code is
//! derived from the Rust standard library, as is some of the more complex
//! functionality.
//!
//! [`half`] is dual licensed under an Apache 2.0 and MIT license.
//!
//! [`half`]: https://github.com/starkat99/half-rs

#![cfg(feature = "f16")]
#![doc(hidden)]

use core::cmp::Ordering;
use core::iter::{Product, Sum};
use core::num::FpCategory;
use core::ops::*;
use core::{fmt, mem, num, str};

use crate::num::Float;
use crate::numtypes::*;

/// Brain floating point type.
///
/// This has the same representation as [`bf16`] from [`half`], and
/// is guaranteed to be supported as a `u16` to/from C.
///
/// [`bf16`]: https://docs.rs/half/latest/half/struct.bf16.html
/// [`half`]: https://docs.rs/half/latest/half/
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone)]
pub struct bf16(u16);

// # SAFETY: bf16 is a trivial type internally represented by u16.
unsafe impl Send for bf16 {
}
// # SAFETY: bf16 is a trivial type internally represented by u16.
unsafe impl Sync for bf16 {
}

impl bf16 {
    /// The radix or base of the internal representation of `bf16`.
    pub const RADIX: u32 = 2;

    /// Number of significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = <Self as Float>::MANTISSA_SIZE as u32 + 1;

    /// Approximate number of significant digits in base 10.
    ///
    /// This is the maximum <i>x</i> such that any decimal number with <i>x</i>
    /// significant digits can be converted to `bf16` and back without loss.
    ///
    /// Equal to floor(log<sub>10</sub>&nbsp;2<sup>[`MANTISSA_DIGITS`]&nbsp;&
    /// minus;&nbsp;1</sup>).
    ///
    /// [`MANTISSA_DIGITS`]: bf16::MANTISSA_DIGITS
    pub const DIGITS: u32 = 2;

    /// [Machine epsilon] value for `bf16`.
    ///
    /// This is the difference between `1.0` and the next larger representable
    /// number.
    ///
    /// Equal to 2<sup>1&nbsp;&minus;&nbsp;[`MANTISSA_DIGITS`]</sup>.
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    /// [`MANTISSA_DIGITS`]: bf16::MANTISSA_DIGITS
    pub const EPSILON: Self = bf16(0x3C00u16);

    /// Smallest finite `bf16` value.
    ///
    /// Equal to &minus;[`MAX`].
    ///
    /// [`MAX`]: bf16::MAX
    pub const MIN: Self = <Self as Float>::MIN;

    /// Smallest positive normal `bf16` value.
    ///
    /// Equal to 2<sup>[`MIN_EXP`]&nbsp;&minus;&nbsp;1</sup>.
    ///
    /// [`MIN_EXP`]: bf16::MIN_EXP
    pub const MIN_POSITIVE: Self = Self(0x0080u16);

    /// Largest finite `bf16` value.
    ///
    /// Equal to
    /// (1&nbsp;&minus;&nbsp;2<sup>&minus;[`MANTISSA_DIGITS`]</sup>)&nbsp;
    /// 2<sup>[`MAX_EXP`]</sup>.
    ///
    /// [`MANTISSA_DIGITS`]: bf16::MANTISSA_DIGITS
    /// [`MAX_EXP`]: bf16::MAX_EXP
    pub const MAX: Self = <Self as Float>::MAX;

    /// Maximum possible power of 2 exponent.
    ///
    /// If <i>x</i>&nbsp;=&nbsp;`MAX_EXP`, then normal numbers
    /// &lt;&nbsp;1&nbsp;×&nbsp;2<sup><i>x</i></sup>.
    pub const MAX_EXP: i32 = Self::MAX_EXPONENT + Self::MANTISSA_SIZE;

    /// One greater than the minimum possible normal power of 2 exponent.
    ///
    /// If <i>x</i>&nbsp;=&nbsp;`MIN_EXP`, then normal numbers
    /// ≥&nbsp;0.5&nbsp;×&nbsp;2<sup><i>x</i></sup>.
    // NOTE: This is -MAX_EXP + 3
    pub const MIN_EXP: i32 = -Self::MAX_EXP + 3;

    /// Minimum <i>x</i> for which 10<sup><i>x</i></sup> is normal.
    ///
    /// Equal to ceil(log<sub>10</sub>&nbsp;[`MIN_POSITIVE`]).
    ///
    /// [`MIN_POSITIVE`]: bf16::MIN_POSITIVE
    pub const MIN_10_EXP: i32 = -37;

    /// Maximum <i>x</i> for which 10<sup><i>x</i></sup> is normal.
    ///
    /// Equal to floor(log<sub>10</sub>&nbsp;[`MAX`]).
    ///
    /// [`MAX`]: bf16::MAX
    pub const MAX_10_EXP: i32 = 38;

    /// Not a Number (NaN).
    ///
    /// Note that IEEE 754 doesn't define just a single NaN value;
    /// a plethora of bit patterns are considered to be NaN.
    /// Furthermore, the standard makes a difference
    /// between a "signaling" and a "quiet" NaN,
    /// and allows inspecting its "payload" (the unspecified bits in the bit
    /// pattern). This constant isn't guaranteed to equal to any specific
    /// NaN bitpattern, and the stability of its representation over Rust
    /// versions and target platforms isn't guaranteed.
    pub const NAN: Self = <Self as Float>::NAN;

    /// Infinity (∞).
    pub const INFINITY: Self = <Self as Float>::INFINITY;

    /// Negative infinity (−∞).
    pub const NEG_INFINITY: Self = <Self as Float>::NEG_INFINITY;

    /// Sign bit
    pub const SIGN_MASK: u16 = <Self as Float>::SIGN_MASK;

    /// Exponent mask
    pub const EXP_MASK: u16 = <Self as Float>::EXPONENT_MASK;

    /// Mantissa mask
    pub const MAN_MASK: u16 = <Self as Float>::MANTISSA_MASK;

    /// Minimum representable positive value (min subnormal)
    pub const TINY_BITS: u16 = 0x1;

    /// Minimum representable negative value (min negative subnormal)
    pub const NEG_TINY_BITS: u16 = Self::TINY_BITS | Self::SIGN_MASK;

    /// Returns `true` if this value is NaN.
    #[must_use]
    #[inline(always)]
    pub const fn is_nan(self) -> bool {
        let bits = self.to_bits();
        let is_special = bits & Self::EXPONENT_MASK == Self::EXPONENT_MASK;
        is_special && (bits & Self::MANTISSA_MASK) != 0
    }

    /// Computes the absolute value of `self`.
    #[must_use]
    #[inline(always)]
    pub const fn abs(self) -> Self {
        Self(self.0 & !Self::SIGN_MASK)
    }

    /// Returns `true` if this value is positive infinity or negative infinity,
    /// and `false` otherwise.
    #[must_use]
    #[inline(always)]
    pub const fn is_infinite(self) -> bool {
        eq(self, Self::INFINITY) | eq(self, Self::NEG_INFINITY)
    }

    /// Returns `true` if this number is neither infinite nor NaN.
    #[must_use]
    #[inline(always)]
    pub const fn is_finite(self) -> bool {
        self.to_bits() & Self::EXPONENT_MASK != Self::EXPONENT_MASK
    }

    /// Returns `true` if the number is [subnormal].
    ///
    /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
    #[must_use]
    #[inline(always)]
    pub const fn is_subnormal(self) -> bool {
        matches!(self.classify(), FpCategory::Subnormal)
    }

    /// Returns `true` if the number is neither zero, infinite,
    /// [subnormal], or NaN.
    ///
    /// [subnormal]: https://en.wikipedia.org/wiki/Denormal_number
    #[must_use]
    #[inline(always)]
    pub const fn is_normal(self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    #[inline(always)]
    pub const fn classify(self) -> FpCategory {
        let b = self.to_bits();
        match (b & Self::MAN_MASK, b & Self::EXP_MASK) {
            (0, Self::EXP_MASK) => FpCategory::Infinite,
            (_, Self::EXP_MASK) => FpCategory::Nan,
            (0, 0) => FpCategory::Zero,
            (_, 0) => FpCategory::Subnormal,
            _ => FpCategory::Normal,
        }
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0`, NaNs
    /// with positive sign bit and positive infinity.
    ///
    /// Note that IEEE 754 doesn't assign any meaning to the sign bit in case of
    /// a NaN, and as Rust doesn't guarantee that the bit pattern of NaNs are
    /// conserved over arithmetic operations, the result of `is_sign_positive`
    /// on a NaN might produce an unexpected or non-portable result. See the
    /// [specification of NaN bit patterns](f32#nan-bit-patterns) for more
    /// info. Use `self.signum() == 1.0` if you need fully portable behavior
    /// (will return `false` for all NaNs).
    #[inline(always)]
    pub const fn is_sign_positive(self) -> bool {
        self.to_bits() & Self::SIGN_MASK == 0
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0`, NaNs
    /// with negative sign bit and negative infinity.
    ///
    /// Note that IEEE 754 doesn't assign any meaning to the sign bit in case of
    /// a NaN, and as Rust doesn't guarantee that the bit pattern of NaNs are
    /// conserved over arithmetic operations, the result of `is_sign_negative`
    /// on a NaN might produce an unexpected or non-portable result. See the
    /// [specification of NaN bit patterns](f32#nan-bit-patterns) for more
    /// info. Use `self.signum() == -1.0` if you need fully portable
    /// behavior (will return `false` for all NaNs).
    #[inline(always)]
    pub const fn is_sign_negative(self) -> bool {
        !self.is_sign_positive()
    }

    /// Takes the reciprocal (inverse) of a number, `1/x`.
    #[must_use]
    #[inline(always)]
    pub fn recip(self) -> Self {
        Self::ONE / self
    }

    /// Converts radians to degrees.
    #[must_use]
    #[inline(always)]
    pub fn to_degrees(self) -> Self {
        self * Self::from_u16(180) / Self::PI
    }

    /// Converts degrees to radians.
    #[must_use]
    #[inline(always)]
    pub fn to_radians(self) -> Self {
        self * Self::PI / Self::from_u16(180)
    }

    /// Returns the maximum of the two numbers, ignoring NaN.
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    #[must_use]
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        if other > self && !other.is_nan() {
            other
        } else {
            self
        }
    }

    /// Returns the minimum of the two numbers.
    ///
    /// If one of the arguments is NaN, then the other argument is returned.
    #[must_use]
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        if other < self && !other.is_nan() {
            other
        } else {
            self
        }
    }

    /// Raw transmutation to `u16`.
    ///
    /// This is currently identical to `transmute::<f16, u16>(self)` on all
    /// platforms.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    ///
    /// Note that this function is distinct from `as` casting, which attempts to
    /// preserve the *numeric* value, and not the bitwise value.
    #[must_use]
    #[inline(always)]
    pub const fn to_bits(self) -> u16 {
        self.0
    }

    /// Raw transmutation from `u16`.
    ///
    /// This is currently identical to `transmute::<u16, bf16>(v)` on all
    /// platforms. It turns out this is incredibly portable, for two
    /// reasons:
    ///
    /// * Floats and Ints have the same endianness on all supported platforms.
    /// * IEEE 754 very precisely specifies the bit layout of floats.
    ///
    /// However there is one caveat: prior to the 2008 version of IEEE 754, how
    /// to interpret the NaN signaling bit wasn't actually specified. Most
    /// platforms (notably x86 and ARM) picked the interpretation that was
    /// ultimately standardized in 2008, but some didn't (notably MIPS). As
    /// a result, all signaling NaNs on MIPS are quiet NaNs on x86, and
    /// vice-versa.
    ///
    /// Rather than trying to preserve signaling-ness cross-platform, this
    /// implementation favors preserving the exact bits. This means that
    /// any payloads encoded in NaNs will be preserved even if the result of
    /// this method is sent over the network from an x86 machine to a MIPS one.
    ///
    /// If the results of this method are only manipulated by the same
    /// architecture that produced them, then there is no portability concern.
    ///
    /// If the input isn't NaN, then there is no portability concern.
    ///
    /// If you don't care about signalingness (very likely), then there is no
    /// portability concern.
    ///
    /// Note that this function is distinct from `as` casting, which attempts to
    /// preserve the *numeric* value, and not the bitwise value.
    #[must_use]
    #[inline(always)]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// Returns the memory representation of this floating point number as a
    /// byte array in big-endian (network) byte order.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    #[must_use]
    #[inline(always)]
    pub const fn to_be_bytes(self) -> [u8; 2] {
        self.to_bits().to_be_bytes()
    }

    /// Returns the memory representation of this floating point number as a
    /// byte array in little-endian byte order.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    #[must_use]
    #[inline(always)]
    pub const fn to_le_bytes(self) -> [u8; 2] {
        self.to_bits().to_le_bytes()
    }

    /// Returns the memory representation of this floating point number as a
    /// byte array in native byte order.
    ///
    /// As the target platform's native endianness is used, portable code
    /// should use [`to_be_bytes`] or [`to_le_bytes`], as appropriate, instead.
    ///
    /// [`to_be_bytes`]: Self::to_be_bytes
    /// [`to_le_bytes`]: Self::to_le_bytes
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    #[must_use]
    #[inline(always)]
    pub const fn to_ne_bytes(self) -> [u8; 2] {
        self.to_bits().to_ne_bytes()
    }

    /// Creates a floating point value from its representation as a byte array
    /// in big endian.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    #[must_use]
    #[inline(always)]
    pub const fn from_be_bytes(bytes: [u8; 2]) -> Self {
        Self::from_bits(u16::from_be_bytes(bytes))
    }

    /// Creates a floating point value from its representation as a byte array
    /// in little endian.
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    #[must_use]
    #[inline(always)]
    pub const fn from_le_bytes(bytes: [u8; 2]) -> Self {
        Self::from_bits(u16::from_le_bytes(bytes))
    }

    /// Creates a floating point value from its representation as a byte array
    /// in native endian.
    ///
    /// As the target platform's native endianness is used, portable code
    /// likely wants to use [`from_be_bytes`] or [`from_le_bytes`], as
    /// appropriate instead.
    ///
    /// [`from_be_bytes`]: Self::from_be_bytes
    /// [`from_le_bytes`]: Self::from_le_bytes
    ///
    /// See [`from_bits`](Self::from_bits) for some discussion of the
    /// portability of this operation (there are almost no issues).
    #[must_use]
    #[inline(always)]
    pub const fn from_ne_bytes(bytes: [u8; 2]) -> Self {
        Self::from_bits(u16::from_ne_bytes(bytes))
    }

    /// Returns the ordering between `self` and `other`.
    ///
    /// Unlike the standard partial comparison between floating point numbers,
    /// this comparison always produces an ordering in accordance to
    /// the `totalOrder` predicate as defined in the IEEE 754 (2008 revision)
    /// floating point standard. The values are ordered in the following
    /// sequence:
    ///
    /// - negative quiet NaN
    /// - negative signaling NaN
    /// - negative infinity
    /// - negative numbers
    /// - negative subnormal numbers
    /// - negative zero
    /// - positive zero
    /// - positive subnormal numbers
    /// - positive numbers
    /// - positive infinity
    /// - positive signaling NaN
    /// - positive quiet NaN.
    ///
    /// The ordering established by this function does not always agree with the
    /// [`PartialOrd`] and [`PartialEq`] implementations of `bf16`. For example,
    /// they consider negative and positive zero equal, while `total_cmp`
    /// doesn't.
    ///
    /// The interpretation of the signaling NaN bit follows the definition in
    /// the IEEE 754 standard, which may not match the interpretation by some of
    /// the older, non-conformant (e.g. MIPS) hardware implementations.
    ///
    /// # Examples
    /// ```
    /// # use half::bf16;
    /// let mut v: Vec<bf16> = vec![];
    /// v.push(bf16::ONE);
    /// v.push(bf16::INFINITY);
    /// v.push(bf16::NEG_INFINITY);
    /// v.push(bf16::NAN);
    /// v.push(bf16::MAX_SUBNORMAL);
    /// v.push(-bf16::MAX_SUBNORMAL);
    /// v.push(bf16::ZERO);
    /// v.push(bf16::NEG_ZERO);
    /// v.push(bf16::NEG_ONE);
    /// v.push(bf16::MIN_POSITIVE);
    ///
    /// v.sort_by(|a, b| a.total_cmp(&b));
    ///
    /// assert!(v
    ///     .into_iter()
    ///     .zip(
    ///         [
    ///             bf16::NEG_INFINITY,
    ///             bf16::NEG_ONE,
    ///             -bf16::MAX_SUBNORMAL,
    ///             bf16::NEG_ZERO,
    ///             bf16::ZERO,
    ///             bf16::MAX_SUBNORMAL,
    ///             bf16::MIN_POSITIVE,
    ///             bf16::ONE,
    ///             bf16::INFINITY,
    ///             bf16::NAN
    ///         ]
    ///         .iter()
    ///     )
    ///     .all(|(a, b)| a.to_bits() == b.to_bits()));
    /// ```
    // Implementation based on: https://doc.rust-lang.org/std/primitive.f32.html#method.total_cmp
    #[must_use]
    #[inline(always)]
    pub fn total_cmp(&self, other: &Self) -> Ordering {
        let mut left = self.to_bits() as i16;
        let mut right = other.to_bits() as i16;
        left ^= (((left >> 15) as u16) >> 1) as i16;
        right ^= (((right >> 15) as u16) >> 1) as i16;
        left.cmp(&right)
    }

    /// Restrict a value to a certain interval unless it is NaN.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise this returns `self`.
    ///
    /// Note that this function returns NaN if the initial value was NaN as
    /// well.
    ///
    /// # Panics
    /// Panics if `min > max`, `min` is NaN, or `max` is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use half::prelude::*;
    /// assert!(bf16::from_f32(-3.0).clamp(bf16::from_f32(-2.0), bf16::from_f32(1.0)) == bf16::from_f32(-2.0));
    /// assert!(bf16::from_f32(0.0).clamp(bf16::from_f32(-2.0), bf16::from_f32(1.0)) == bf16::from_f32(0.0));
    /// assert!(bf16::from_f32(2.0).clamp(bf16::from_f32(-2.0), bf16::from_f32(1.0)) == bf16::from_f32(1.0));
    /// assert!(bf16::NAN.clamp(bf16::from_f32(-2.0), bf16::from_f32(1.0)).is_nan());
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn clamp(self, min: bf16, max: bf16) -> bf16 {
        assert!(min <= max);
        let mut x = self;
        if x < min {
            x = min;
        }
        if x > max {
            x = max;
        }
        x
    }
}

macro_rules! from_int_impl {
    ($t:ty, $func:ident) => {
        /// Create from the integral type, as if by an `as` cast.
        #[inline(always)]
        pub const fn $func(value: $t) -> Self {
            Self::from_f32(value as f32)
        }
    };
}

// Non-standard extensions to simplify working with `f16`.
impl bf16 {
    /// [`bf16`] 1
    pub const ONE: bf16 = bf16(0x3F80u16);

    /// [`bf16`] 0
    pub const ZERO: bf16 = bf16(0x0000u16);

    /// [`bf16`] -0
    pub const NEG_ZERO: bf16 = bf16(0x8000u16);

    /// [`bf16`] -1
    pub const NEG_ONE: bf16 = bf16(0xBF80u16);

    /// [`bf16`] Euler's number (ℯ)
    pub const E: bf16 = bf16(0x402Eu16);

    /// [`bf16`] Archimedes' constant (π)
    pub const PI: bf16 = bf16(0x4049u16);

    /// [`bf16`] 1/π
    pub const FRAC_1_PI: bf16 = bf16(0x3EA3u16);

    /// [`bf16`] 1/√2
    pub const FRAC_1_SQRT_2: bf16 = bf16(0x3F35u16);

    /// [`bf16`] 2/π
    pub const FRAC_2_PI: bf16 = bf16(0x3F23u16);

    /// [`bf16`] 2/√π
    pub const FRAC_2_SQRT_PI: bf16 = bf16(0x3F90u16);

    /// [`bf16`] π/2
    pub const FRAC_PI_2: bf16 = bf16(0x3FC9u16);

    /// [`bf16`] π/3
    pub const FRAC_PI_3: bf16 = bf16(0x3F86u16);

    /// [`bf16`] π/4
    pub const FRAC_PI_4: bf16 = bf16(0x3F49u16);

    /// [`bf16`] π/6
    pub const FRAC_PI_6: bf16 = bf16(0x3F06u16);

    /// [`bf16`] π/8
    pub const FRAC_PI_8: bf16 = bf16(0x3EC9u16);

    /// [`bf16`] 𝗅𝗇 10
    pub const LN_10: bf16 = bf16(0x4013u16);

    /// [`bf16`] 𝗅𝗇 2
    pub const LN_2: bf16 = bf16(0x3F31u16);

    /// [`bf16`] 𝗅𝗈𝗀₁₀ℯ
    pub const LOG10_E: bf16 = bf16(0x3EDEu16);

    /// [`bf16`] 𝗅𝗈𝗀₁₀2
    pub const LOG10_2: bf16 = bf16(0x3E9Au16);

    /// [`bf16`] 𝗅𝗈𝗀₂ℯ
    pub const LOG2_E: bf16 = bf16(0x3FB9u16);

    /// [`bf16`] 𝗅𝗈𝗀₂10
    pub const LOG2_10: bf16 = bf16(0x4055u16);

    /// [`bf16`] √2
    pub const SQRT_2: bf16 = bf16(0x3FB5u16);

    #[inline(always)]
    pub const fn as_f32(self) -> f32 {
        // This is super easy, since we have the same exponent bits:
        // just need to shift left 16.
        f32_from_bits((self.0 as u32) << 16)
    }

    #[inline(always)]
    pub const fn from_f32(value: f32) -> Self {
        // Same number of exponent bits, less mantissa bits: simple conversion.
        // We want to round to the nearest float, so we'll check if it's odd
        // and above or equal to halfway. This also properly handled inf, denormal,
        // and NaN cases, since they're effectively the same.
        let bits = f32_to_bits(value);
        let truncated = bits as u16;
        let bf16_bits = (bits >> 16) as u16;

        let halfway = 1u16 << 15;
        let is_odd = bf16_bits % 2 == 1;
        let is_halfway = truncated == halfway;
        let is_above = truncated > halfway;
        let round_up = is_above || (is_halfway && is_odd);

        Self::from_bits(bf16_bits + round_up as u16)
    }

    from_int_impl!(u8, from_u8);
    from_int_impl!(u16, from_u16);
    from_int_impl!(u32, from_u32);
    from_int_impl!(u64, from_u64);
    from_int_impl!(u128, from_u128);
    from_int_impl!(i8, from_i8);
    from_int_impl!(i16, from_i16);
    from_int_impl!(i32, from_i32);
    from_int_impl!(i64, from_i64);
    from_int_impl!(i128, from_i128);
    from_int_impl!(f64, from_f64);
}

impl PartialEq for bf16 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        eq(*self, *other)
    }
}

impl PartialOrd for bf16 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        partial_cmp(*self, *other)
    }

    #[inline(always)]
    fn lt(&self, other: &bf16) -> bool {
        lt(*self, *other)
    }

    #[inline(always)]
    fn le(&self, other: &bf16) -> bool {
        le(*self, *other)
    }

    #[inline(always)]
    fn gt(&self, other: &bf16) -> bool {
        gt(*self, *other)
    }

    #[inline(always)]
    fn ge(&self, other: &bf16) -> bool {
        ge(*self, *other)
    }
}

impl Add for bf16 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() + rhs.as_f32())
    }
}

op_impl!(bf16, Add, AddAssign, add, add_assign);

impl Div for bf16 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() / rhs.as_f32())
    }
}

op_impl!(bf16, Div, DivAssign, div, div_assign);

impl Mul for bf16 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() * rhs.as_f32())
    }
}

op_impl!(bf16, Mul, MulAssign, mul, mul_assign);

impl Sub for bf16 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() - rhs.as_f32())
    }
}

op_impl!(bf16, Sub, SubAssign, sub, sub_assign);

impl Rem for bf16 {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_f32(self.as_f32() % rhs.as_f32())
    }
}

op_impl!(bf16, Rem, RemAssign, rem, rem_assign);

impl Neg for bf16 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::from_bits(self.0 ^ (1 << 15))
    }
}

ref_impl!(bf16, Neg, neg);

// NOTE: This is not optimized, and is optimized in `lexical`.
impl str::FromStr for bf16 {
    type Err = num::ParseFloatError;

    fn from_str(src: &str) -> Result<bf16, num::ParseFloatError> {
        f32::from_str(src).map(bf16::from_f32)
    }
}

impl fmt::Debug for bf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self.as_f32(), f)
    }
}

impl fmt::Display for bf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.as_f32(), f)
    }
}

impl fmt::LowerExp for bf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:e}", self.as_f32())
    }
}

impl fmt::UpperExp for bf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:E}", self.as_f32())
    }
}

impl fmt::Binary for bf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::Octal for bf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:o}", self.0)
    }
}

impl fmt::LowerHex for bf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:x}", self.0)
    }
}

impl fmt::UpperHex for bf16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:X}", self.0)
    }
}

impl Product for bf16 {
    #[inline(always)]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        product_bf16(iter.map(|f| f.to_bits()))
    }
}

impl<'a> Product<&'a bf16> for bf16 {
    #[inline]
    fn product<I: Iterator<Item = &'a bf16>>(iter: I) -> Self {
        product_bf16(iter.map(|f| f.to_bits()))
    }
}

impl Sum for bf16 {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        sum_bf16(iter.map(|f| f.to_bits()))
    }
}

impl<'a> Sum<&'a bf16> for bf16 {
    #[inline]
    fn sum<I: Iterator<Item = &'a bf16>>(iter: I) -> Self {
        sum_bf16(iter.map(|f| f.to_bits()))
    }
}

from_impl!(bf16, u8, from_u8);
from_impl!(bf16, u16, from_u16);
from_impl!(bf16, u32, from_u32);
from_impl!(bf16, u64, from_u64);
from_impl!(bf16, u128, from_u128);
from_impl!(bf16, i8, from_i8);
from_impl!(bf16, i16, from_i16);
from_impl!(bf16, i32, from_i32);
from_impl!(bf16, i64, from_i64);
from_impl!(bf16, i128, from_i128);
from_impl!(bf16, f64, from_f64);

// Private functions for const expr.
#[must_use]
#[inline(always)]
const fn eq(lhs: bf16, rhs: bf16) -> bool {
    // NOTE: This optimizes identically at opt levels 1+ to asm
    //  https://godbolt.org/z/bW7s7o5M6
    if lhs.is_nan() {
        false
    } else if lhs.to_bits() & !bf16::SIGN_MASK == 0 {
        rhs.to_bits() & !bf16::SIGN_MASK == 0
    } else {
        lhs.to_bits() == rhs.to_bits()
    }
}

#[must_use]
#[inline(always)]
fn partial_cmp(lhs: bf16, rhs: bf16) -> Option<Ordering> {
    if lhs.is_nan() || rhs.is_nan() {
        None
    } else {
        let neg = lhs.0 & 0x8000u16 != 0;
        let rhs_neg = rhs.0 & 0x8000u16 != 0;
        match (neg, rhs_neg) {
            (false, false) => Some(lhs.0.cmp(&rhs.0)),
            (false, true) => {
                if (lhs.0 | rhs.0) & 0x7FFFu16 == 0 {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Greater)
                }
            },
            (true, false) => {
                if (lhs.0 | rhs.0) & 0x7FFFu16 == 0 {
                    Some(Ordering::Equal)
                } else {
                    Some(Ordering::Less)
                }
            },
            (true, true) => Some(rhs.0.cmp(&lhs.0)),
        }
    }
}

#[must_use]
#[inline(always)]
const fn lt(lhs: bf16, rhs: bf16) -> bool {
    if lhs.is_nan() || rhs.is_nan() {
        false
    } else {
        let neg = lhs.0 & 0x8000u16 != 0;
        let rhs_neg = rhs.0 & 0x8000u16 != 0;
        match (neg, rhs_neg) {
            (false, false) => lhs.0 < rhs.0,
            (false, true) => false,
            (true, false) => (lhs.0 | rhs.0) & 0x7FFFu16 != 0,
            (true, true) => lhs.0 > rhs.0,
        }
    }
}

#[must_use]
#[inline(always)]
const fn le(lhs: bf16, rhs: bf16) -> bool {
    if lhs.is_nan() || rhs.is_nan() {
        false
    } else {
        let neg = lhs.0 & 0x8000u16 != 0;
        let rhs_neg = rhs.0 & 0x8000u16 != 0;
        match (neg, rhs_neg) {
            (false, false) => lhs.0 <= rhs.0,
            (false, true) => (lhs.0 | rhs.0) & 0x7FFFu16 == 0,
            (true, false) => true,
            (true, true) => lhs.0 >= rhs.0,
        }
    }
}

#[must_use]
#[inline(always)]
const fn gt(lhs: bf16, rhs: bf16) -> bool {
    if lhs.is_nan() || rhs.is_nan() {
        false
    } else {
        let neg = lhs.0 & 0x8000u16 != 0;
        let rhs_neg = rhs.0 & 0x8000u16 != 0;
        match (neg, rhs_neg) {
            (false, false) => lhs.0 > rhs.0,
            (false, true) => (lhs.0 | rhs.0) & 0x7FFFu16 != 0,
            (true, false) => false,
            (true, true) => lhs.0 < rhs.0,
        }
    }
}

#[must_use]
#[inline(always)]
const fn ge(lhs: bf16, rhs: bf16) -> bool {
    if lhs.is_nan() || rhs.is_nan() {
        false
    } else {
        let neg = lhs.0 & 0x8000u16 != 0;
        let rhs_neg = rhs.0 & 0x8000u16 != 0;
        match (neg, rhs_neg) {
            (false, false) => lhs.0 >= rhs.0,
            (false, true) => true,
            (true, false) => (lhs.0 | rhs.0) & 0x7FFFu16 == 0,
            (true, true) => lhs.0 <= rhs.0,
        }
    }
}

// NOTE: taken from the core implementation.
#[must_use]
#[inline(always)]
const fn f32_from_bits(v: u32) -> f32 {
    // SAFETY: The type is POD
    unsafe { mem::transmute(v) }
}

#[must_use]
#[inline(always)]
const fn f32_to_bits(v: f32) -> u32 {
    // SAFETY: The type is POD
    unsafe { mem::transmute(v) }
}

#[must_use]
#[inline(always)]
const fn u16_to_f32(v: u16) -> f32 {
    bf16(v).as_f32()
}

#[must_use]
#[inline(always)]
fn product_bf16<I: Iterator<Item = u16>>(iter: I) -> bf16 {
    bf16::from_f32(iter.map(u16_to_f32).product())
}

#[must_use]
#[inline(always)]
fn sum_bf16<I: Iterator<Item = u16>>(iter: I) -> bf16 {
    bf16::from_f32(iter.map(u16_to_f32).sum())
}
