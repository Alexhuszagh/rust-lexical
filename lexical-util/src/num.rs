//! Utilities for Rust numbers.
//!
//! These traits define useful properties, methods, associated
//! types, and trait bounds, and conversions for working with
//! numbers in generic code.

use crate::lib::{fmt, mem, ops};

// AS PRIMITIVE
// ------------

/// Type that can be converted to primitive with `as`.
#[doc(hidden)]
pub trait AsPrimitive: Copy + PartialEq + PartialOrd + Send + Sync + Sized {
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
    fn as_u64(self) -> u64;
    fn as_u128(self) -> u128;
    fn as_usize(self) -> usize;
    fn as_i8(self) -> i8;
    fn as_i16(self) -> i16;
    fn as_i32(self) -> i32;
    fn as_i64(self) -> i64;
    fn as_i128(self) -> i128;
    fn as_isize(self) -> isize;
    fn as_f32(self) -> f32;
    fn as_f64(self) -> f64;
    fn from_u32(value: u32) -> Self;
}

macro_rules! as_primitive {
    ($($t:ty)*) => ($(
        impl AsPrimitive for $t {
            #[inline(always)]
            fn as_u8(self) -> u8 {
                self as _
            }

            #[inline(always)]
            fn as_u16(self) -> u16 {
                self as _
            }

            #[inline(always)]
            fn as_u32(self) -> u32 {
                self as _
            }

            #[inline(always)]
            fn as_u64(self) -> u64 {
                self as _
            }

            #[inline(always)]
            fn as_u128(self) -> u128 {
                self as _
            }

            #[inline(always)]
            fn as_usize(self) -> usize {
                self as _
            }

            #[inline(always)]
            fn as_i8(self) -> i8 {
                self as _
            }

            #[inline(always)]
            fn as_i16(self) -> i16 {
                self as _
            }

            #[inline(always)]
            fn as_i32(self) -> i32 {
                self as _
            }

            #[inline(always)]
            fn as_i64(self) -> i64 {
                self as _
            }

            #[inline(always)]
            fn as_i128(self) -> i128 {
                self as _
            }

            #[inline(always)]
            fn as_isize(self) -> isize {
                self as _
            }

            #[inline(always)]
            fn as_f32(self) -> f32 {
                self as _
            }

            #[inline(always)]
            fn as_f64(self) -> f64 {
                self as _
            }

            #[inline(always)]
            fn from_u32(value: u32) -> Self {
                value as _
            }
        }
    )*)
}

as_primitive! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

// AS CAST
// -------

/// An interface for casting between machine scalars.
pub trait AsCast: AsPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `AsPrimitive` trait.
    fn as_cast<N: AsPrimitive>(n: N) -> Self;
}

/// Allows the high-level conversion of generic types as if `as` was used.
#[inline]
pub fn as_cast<U: AsCast, T: AsCast>(t: T) -> U {
    U::as_cast(t)
}

macro_rules! as_cast {
    ($($t:ty, $meth:ident ; )*) => ($(
        impl AsCast for $t {
            #[inline]
            fn as_cast<N: AsPrimitive>(n: N) -> $t {
                n.$meth() as _
            }
        }
    )*);
}

as_cast!(
    u8, as_u8 ;
    u16, as_u16 ;
    u32, as_u32 ;
    u64, as_u64 ;
    u128, as_u128 ;
    usize, as_usize ;
    i8, as_i8 ;
    i16, as_i16 ;
    i32, as_i32 ;
    i64, as_i64 ;
    i128, as_i128 ;
    isize, as_isize ;
    f32, as_f32 ;
    f64, as_f64 ;
);

// PRIMITIVE
// ---------

/// Primitive type trait (which all have static lifetimes).
#[doc(hidden)]
pub trait Primitive: 'static + fmt::Debug + fmt::Display + AsCast {}

macro_rules! primitive {
    ($($t:ty)*) => ($(
        impl Primitive for $t {}
    )*)
}

primitive! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

// NUMBER
// ------

/// Numerical type trait.
#[doc(hidden)]
pub trait Number:
    Default +
    Primitive +
    // Operations
    ops::Add<Output=Self> +
    ops::AddAssign +
    ops::Div<Output=Self> +
    ops::DivAssign +
    ops::Mul<Output=Self> +
    ops::MulAssign +
    ops::Rem<Output=Self> +
    ops::RemAssign +
    ops::Sub<Output=Self> +
    ops::SubAssign
{
    /// If the number is a signed type.
    const IS_SIGNED: bool;
}

macro_rules! number_impl {
    ($($t:tt $is_signed:literal ; )*) => ($(
        impl Number for $t {
            const IS_SIGNED: bool = $is_signed;
        }
    )*)
}

number_impl! {
    u8 false ;
    u16 false ;
    u32 false ;
    u64 false ;
    u128 false ;
    usize false ;
    i8 true ;
    i16 true ;
    i32 true ;
    i64 true ;
    i128 true ;
    isize true ;
    // f16 true
    // bf16 true
    f32 true ;
    f64 true ;
    // f128 true
}

// INTEGER
// -------

/// Defines a trait that supports integral operations.
#[doc(hidden)]
pub trait Integer:
    // Basic
    Number + Eq + Ord +
    // Operations
    ops::BitAnd<Output=Self> +
    ops::BitAndAssign +
    ops::BitOr<Output=Self> +
    ops::BitOrAssign +
    ops::BitXor<Output=Self> +
    ops::BitXorAssign +
    ops::Shl<Self, Output=Self> +
    ops::Shl<i32, Output=Self> +
    ops::ShlAssign<i32> +
    ops::Shr<i32, Output=Self> +
    ops::ShrAssign<i32> +
{
    // CONSTANTS
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;
    const MIN: Self;
    const BITS: usize;

    // FUNCTIONS (INHERITED)
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn checked_add(self, i: Self) -> Option<Self>;
    fn checked_sub(self, i: Self) -> Option<Self>;
    fn checked_mul(self, i: Self) -> Option<Self>;
    fn overflowing_add(self, i: Self) -> (Self, bool);
    fn overflowing_mul(self, i: Self) -> (Self, bool);
    fn wrapping_add(self, i: Self) -> Self;
    fn wrapping_sub(self, i: Self) -> Self;
    fn wrapping_mul(self, i: Self) -> Self;
    fn wrapping_neg(self) -> Self;
    fn saturating_add(self, i: Self) -> Self;
    fn saturating_sub(self, i: Self) -> Self;
    fn saturating_mul(self, i: Self) -> Self;

    /// Get the fast ceiling of the quotient from integer division.
    /// Not safe, since the remainder can easily overflow.
    #[inline]
    fn ceil_divmod(self, y: Self) -> (Self, i32) {
        let q = self / y;
        let r = self % y;
        match r == Self::ZERO {
            true  => (q, i32::as_cast(r)),
            false => (q + Self::ONE, i32::as_cast(r) - i32::as_cast(y))
        }
    }

    /// Get the fast ceiling of the quotient from integer division.
    /// Not safe, since the remainder can easily overflow.
    #[inline]
    fn ceil_div(self, y: Self) -> Self {
        self.ceil_divmod(y).0
    }

    /// Get the fast ceiling modulus from integer division.
    /// Not safe, since the remainder can easily overflow.
    #[inline]
    fn ceil_mod(self, y: Self) -> i32 {
        self.ceil_divmod(y).1
    }

    // PROPERTIES

    /// Get the number of bits in a value.
    #[inline]
    fn bit_length(self) -> u32 {
        Self::BITS as u32 - self.leading_zeros()
    }

    /// Returns true if the least-significant bit is odd.
    #[inline]
    fn is_odd(self) -> bool {
        self & Self::ONE == Self::ONE
    }

    /// Returns true if the least-significant bit is even.
    #[inline]
    fn is_even(self) -> bool {
        !self.is_odd()
    }
}

macro_rules! integer_impl {
    ($($t:tt)*) => ($(
        impl Integer for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TWO: $t = 2;
            const MAX: $t = $t::max_value();
            const MIN: $t = $t::min_value();
            // DEPRECATE: when we drop support for <= 1.53.0, change to `<$t>::BITS`
            const BITS: usize = mem::size_of::<$t>() * 8;

            #[inline]
            fn leading_zeros(self) -> u32 {
                $t::leading_zeros(self)
            }

            #[inline]
            fn trailing_zeros(self) -> u32 {
                $t::trailing_zeros(self)
            }

            #[inline]
            fn checked_add(self, i: Self) -> Option<Self> {
                $t::checked_add(self, i)
            }

            #[inline]
            fn checked_sub(self, i: Self) -> Option<Self> {
                $t::checked_sub(self, i)
            }

            #[inline]
            fn checked_mul(self, i: Self) -> Option<Self> {
                $t::checked_mul(self, i)
            }

            #[inline]
            fn overflowing_add(self, i: Self) -> (Self, bool) {
                $t::overflowing_add(self, i)
            }

            #[inline]
            fn overflowing_mul(self, i: Self) -> (Self, bool) {
                $t::overflowing_mul(self, i)
            }

            #[inline]
            fn wrapping_add(self, i: Self) -> Self {
                $t::wrapping_add(self, i)
            }

            #[inline]
            fn wrapping_sub(self, i: Self) -> Self {
                $t::wrapping_sub(self, i)
            }

            #[inline]
            fn wrapping_mul(self, i: Self) -> Self {
                $t::wrapping_mul(self, i)
            }

            #[inline]
            fn wrapping_neg(self) -> Self {
                $t::wrapping_neg(self)
            }

            #[inline]
            fn saturating_add(self, i: Self) -> Self {
                $t::saturating_add(self, i)
            }

            #[inline]
            fn saturating_sub(self, i: Self) -> Self {
                $t::saturating_sub(self, i)
            }

            #[inline]
            fn saturating_mul(self, i: Self) -> Self {
                $t::saturating_mul(self, i)
            }
        }
    )*)
}

integer_impl! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

// SIGNED INTEGER
// --------------

/// Defines a trait that supports signed integral operations.
#[doc(hidden)]
pub trait SignedInteger: Integer + ops::Neg<Output = Self> {}

macro_rules! signed_integer_impl {
    ($($t:tt)*) => ($(
        impl SignedInteger for $t {}
    )*)
}

signed_integer_impl! { i8 i16 i32 i64 i128 isize }

// UNSIGNED INTEGER
// ----------------

/// Defines a trait that supports unsigned integral operations.
#[doc(hidden)]
pub trait UnsignedInteger: Integer {}

macro_rules! unsigned_integer_impl {
    ($($t:ty)*) => ($(
        impl UnsignedInteger for $t {}
    )*)
}

unsigned_integer_impl! { u8 u16 u32 u64 u128 usize }

// FLOAT
// -----

/// Float information for native float types.
#[doc(hidden)]
#[cfg(feature = "floats")]
pub trait Float: Number + ops::Neg<Output = Self> {
    /// Unsigned type of the same size.
    type Unsigned: UnsignedInteger;

    // CONSTANTS
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;
    const MIN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;
    const BITS: usize;

    /// Bitmask for the sign bit.
    const SIGN_MASK: Self::Unsigned;
    /// Bitmask for the exponent, including the hidden bit.
    const EXPONENT_MASK: Self::Unsigned;
    /// Bitmask for the hidden bit in exponent, which is an implicit 1 in the fraction.
    const HIDDEN_BIT_MASK: Self::Unsigned;
    /// Bitmask for the mantissa (fraction), excluding the hidden bit.
    const MANTISSA_MASK: Self::Unsigned;
    /// Mask to determine if a full-carry occurred (1 in bit above hidden bit).
    const CARRY_MASK: Self::Unsigned;

    // PROPERTIES

    // The following constants can be calculated as follows:
    //  - `INFINITY_BITS`: EXPONENT_MASK
    //  - `NEGATIVE_INFINITY_BITS`: INFINITY_BITS | SIGN_MASK
    //  - `EXPONENT_BIAS`: `2^(EXPONENT_SIZE-1) - 1 + MANTISSA_SIZE`
    //  - `DENORMAL_EXPONENT`: `1 - EXPONENT_BIAS`
    //  - `MAX_EXPONENT`: `2^EXPONENT_SIZE - 1 - EXPONENT_BIAS`

    /// Positive infinity as bits.
    const INFINITY_BITS: Self::Unsigned;
    /// Positive infinity as bits.
    const NEGATIVE_INFINITY_BITS: Self::Unsigned;
    /// Size of the exponent.
    const EXPONENT_SIZE: i32;
    /// Size of the significand (mantissa) without hidden bit.
    const MANTISSA_SIZE: i32;
    /// Bias of the exponent.
    const EXPONENT_BIAS: i32;
    /// Exponent portion of a denormal float.
    const DENORMAL_EXPONENT: i32;
    /// Maximum exponent value in float.
    const MAX_EXPONENT: i32;

    // FUNCTIONS (INHERITED)

    // Re-export the to and from bits methods.
    fn to_bits(self) -> Self::Unsigned;
    fn from_bits(u: Self::Unsigned) -> Self;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;

    /// Returns true if the float is a denormal.
    #[inline]
    fn is_denormal(self) -> bool {
        self.to_bits() & Self::EXPONENT_MASK == Self::Unsigned::ZERO
    }

    /// Returns true if the float is a NaN or Infinite.
    #[inline]
    fn is_special(self) -> bool {
        self.to_bits() & Self::EXPONENT_MASK == Self::EXPONENT_MASK
    }

    /// Returns true if the float is NaN.
    #[inline]
    fn is_nan(self) -> bool {
        self.is_special() && (self.to_bits() & Self::MANTISSA_MASK) != Self::Unsigned::ZERO
    }

    /// Returns true if the float is infinite.
    #[inline]
    fn is_inf(self) -> bool {
        self.is_special() && (self.to_bits() & Self::MANTISSA_MASK) == Self::Unsigned::ZERO
    }

    /// Returns true if the float's least-significant mantissa bit is odd.
    #[inline]
    fn is_odd(self) -> bool {
        self.to_bits().is_odd()
    }

    /// Returns true if the float's least-significant mantissa bit is even.
    #[inline]
    fn is_even(self) -> bool {
        !self.is_odd()
    }

    /// Get exponent component from the float.
    #[inline]
    fn exponent(self) -> i32 {
        if self.is_denormal() {
            return Self::DENORMAL_EXPONENT;
        }

        let bits = self.to_bits();
        let biased_e = i32::as_cast((bits & Self::EXPONENT_MASK) >> Self::MANTISSA_SIZE).as_i32();
        biased_e - Self::EXPONENT_BIAS
    }

    /// Get mantissa (significand) component from float.
    #[inline]
    fn mantissa(self) -> Self::Unsigned {
        let bits = self.to_bits();
        let s = bits & Self::MANTISSA_MASK;
        if !self.is_denormal() {
            s + Self::HIDDEN_BIT_MASK
        } else {
            s
        }
    }

    /// Get next greater float.
    #[inline]
    fn next(self) -> Self {
        let bits = self.to_bits();
        if self.is_sign_negative() && self == Self::ZERO {
            // -0.0
            Self::ZERO
        } else if bits == Self::INFINITY_BITS {
            Self::from_bits(Self::INFINITY_BITS)
        } else if self.is_sign_negative() {
            Self::from_bits(bits.saturating_sub(Self::Unsigned::ONE))
        } else {
            Self::from_bits(bits.saturating_add(Self::Unsigned::ONE))
        }
    }

    /// Get next greater float for a positive float.
    /// Value must be >= 0.0 and < INFINITY.
    #[inline]
    fn next_positive(self) -> Self {
        debug_assert!(self.is_sign_positive() && !self.is_inf());
        Self::from_bits(self.to_bits() + Self::Unsigned::ONE)
    }

    /// Get previous greater float, such that `self.prev().next() == self`.
    #[inline]
    fn prev(self) -> Self {
        let bits = self.to_bits();
        if self.is_sign_positive() && self == Self::ZERO {
            // +0.0
            -Self::ZERO
        } else if bits == Self::NEGATIVE_INFINITY_BITS {
            Self::from_bits(Self::NEGATIVE_INFINITY_BITS)
        } else if self.is_sign_negative() {
            Self::from_bits(bits.saturating_add(Self::Unsigned::ONE))
        } else {
            Self::from_bits(bits.saturating_sub(Self::Unsigned::ONE))
        }
    }

    /// Get previous greater float for a positive float.
    /// Value must be > 0.0.
    #[inline]
    fn prev_positive(self) -> Self {
        debug_assert!(self.is_sign_positive() && self != Self::ZERO);
        Self::from_bits(self.to_bits() - Self::Unsigned::ONE)
    }

    /// Round a positive number to even.
    #[inline]
    fn round_positive_even(self) -> Self {
        if self.mantissa().is_odd() {
            self.next_positive()
        } else {
            self
        }
    }

    /// Get the max of two finite numbers.
    #[inline]
    fn max_finite(self, f: Self) -> Self {
        debug_assert!(!self.is_special() && !f.is_special(), "max_finite self={} f={}", self, f);
        if self < f {
            f
        } else {
            self
        }
    }

    /// Get the min of two finite numbers.
    #[inline]
    fn min_finite(self, f: Self) -> Self {
        debug_assert!(!self.is_special() && !f.is_special(), "min_finite self={} f={}", self, f);
        if self < f {
            self
        } else {
            f
        }
    }
}

/// Define the float literals.
#[cfg(feature = "floats")]
macro_rules! float_literals {
    ($float:ty) => {
        const ZERO: $float = 0.0;
        const ONE: $float = 1.0;
        const TWO: $float = 2.0;
        const MAX: $float = <$float>::MAX;
        const MIN: $float = <$float>::MIN;
        const INFINITY: $float = <$float>::INFINITY;
        const NEG_INFINITY: $float = <$float>::NEG_INFINITY;
        const NAN: $float = <$float>::NAN;
        const BITS: usize = mem::size_of::<$float>() * 8;
    };
}

/// Define the float masks.
#[cfg(feature = "floats")]
macro_rules! float_masks {
    (
        float =>
        $float:ty,sign_mask =>
        $sign:literal,exponent_mask =>
        $exponent:literal,hidden_bit_mask =>
        $hidden:literal,mantissa_mask =>
        $mantissa:literal,
    ) => {
        const SIGN_MASK: <$float>::Unsigned = $sign;
        const EXPONENT_MASK: <$float>::Unsigned = $exponent;
        const HIDDEN_BIT_MASK: <$float>::Unsigned = $hidden;
        const MANTISSA_MASK: <$float>::Unsigned = $mantissa;
        // The carry mask is always 1 bit above the hidden bit.
        const CARRY_MASK: <$float>::Unsigned = $hidden << 1;
        // Infinity is always every exponent bit set.
        const INFINITY_BITS: <$float>::Unsigned = $exponent;
        // Negative infinity is just infinity + sign.
        const NEGATIVE_INFINITY_BITS: <$float>::Unsigned = $exponent | $sign;
    };
}

//  Due to missing specifics or types for the following float types,
//  `Float` is not yet fully implemented for:
//      - f16
//      - bf16
//      - f128

#[cfg(all(feature = "f16", feature = "floats"))]
impl Float for f16 {
    type Unsigned = u16;
    float_literals!(f16);
    float_masks!(
        float => Self,
        sign_mask => 0x8000,
        exponent_mask => 0x7C00,
        hidden_bit_mask => 0x0400,
        mantissa_mask => 0x03FF,
    );
    const EXPONENT_SIZE: i32 = 5;
    const MANTISSA_SIZE: i32 = 10;
    const EXPONENT_BIAS: i32 = 15 + Self::MANTISSA_SIZE;
    const DENORMAL_EXPONENT: i32 = 1 - Self::EXPONENT_BIAS;
    const MAX_EXPONENT: i32 = 0x1F - Self::EXPONENT_BIAS;
}

#[cfg(all(feature = "f16", feature = "floats"))]
impl Float for bf16 {
    type Unsigned = u16;
    float_literals!(bf16);
    float_masks!(
        float => Self,
        sign_mask => 0x8000,
        exponent_mask => 0x7F80,
        hidden_bit_mask => 0x0080,
        mantissa_mask => 0x007F,
    );
    const EXPONENT_SIZE: i32 = 8;
    const MANTISSA_SIZE: i32 = 7;
    const EXPONENT_BIAS: i32 = 127 + Self::MANTISSA_SIZE;
    const DENORMAL_EXPONENT: i32 = 1 - Self::EXPONENT_BIAS;
    const MAX_EXPONENT: i32 = 0xFF - Self::EXPONENT_BIAS;
}

#[cfg(feature = "floats")]
impl Float for f32 {
    type Unsigned = u32;
    float_literals!(f32);
    float_masks!(
        float => Self,
        sign_mask => 0x80000000,
        exponent_mask => 0x7F800000,
        hidden_bit_mask => 0x00800000,
        mantissa_mask => 0x007FFFFF,
    );
    const EXPONENT_SIZE: i32 = 8;
    const MANTISSA_SIZE: i32 = 23;
    const EXPONENT_BIAS: i32 = 127 + Self::MANTISSA_SIZE;
    const DENORMAL_EXPONENT: i32 = 1 - Self::EXPONENT_BIAS;
    const MAX_EXPONENT: i32 = 0xFF - Self::EXPONENT_BIAS;

    #[inline]
    fn to_bits(self) -> u32 {
        f32::to_bits(self)
    }

    #[inline]
    fn from_bits(u: u32) -> f32 {
        f32::from_bits(u)
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        f32::is_sign_positive(self)
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        f32::is_sign_negative(self)
    }
}

#[cfg(feature = "floats")]
impl Float for f64 {
    type Unsigned = u64;
    float_literals!(f64);
    float_masks!(
        float => Self,
        sign_mask => 0x8000000000000000,
        exponent_mask => 0x7FF0000000000000,
        hidden_bit_mask => 0x0010000000000000,
        mantissa_mask => 0x000FFFFFFFFFFFFF,
    );
    const EXPONENT_SIZE: i32 = 11;
    const MANTISSA_SIZE: i32 = 52;
    const EXPONENT_BIAS: i32 = 1023 + Self::MANTISSA_SIZE;
    const DENORMAL_EXPONENT: i32 = 1 - Self::EXPONENT_BIAS;
    const MAX_EXPONENT: i32 = 0x7FF - Self::EXPONENT_BIAS;

    #[inline]
    fn to_bits(self) -> u64 {
        f64::to_bits(self)
    }

    #[inline]
    fn from_bits(u: u64) -> f64 {
        f64::from_bits(u)
    }

    #[inline]
    fn is_sign_positive(self) -> bool {
        f64::is_sign_positive(self)
    }

    #[inline]
    fn is_sign_negative(self) -> bool {
        f64::is_sign_negative(self)
    }
}

#[cfg(all(feature = "f128", feature = "floats"))]
impl Float for f128 {
    type Unsigned = u128;
    float_literals!(f128);
    float_masks!(
        float => Self,
        sign_mask => 0x80000000000000000000000000000000,
        exponent_mask => 0x7FFF0000000000000000000000000000,
        hidden_bit_mask => 0x00010000000000000000000000000000,
        mantissa_mask => 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFF,
    );
    const EXPONENT_SIZE: i32 = 15;
    const MANTISSA_SIZE: i32 = 112;
    const EXPONENT_BIAS: i32 = 16383 + Self::MANTISSA_SIZE;
    const DENORMAL_EXPONENT: i32 = 1 - Self::EXPONENT_BIAS;
    const MAX_EXPONENT: i32 = 0x7FFF - Self::EXPONENT_BIAS;
}
