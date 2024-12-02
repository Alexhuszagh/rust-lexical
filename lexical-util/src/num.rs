//! Utilities for Rust numbers.
//!
//! These traits define useful properties, methods, associated
//! types, and trait bounds, and conversions for working with
//! numbers in generic code.

#![cfg_attr(any(), rustfmt::skip)]

use core::{fmt, mem, ops};

#[cfg(feature = "f16")]
use crate::bf16::bf16;
#[cfg(feature = "f16")]
use crate::f16::f16;

// AS PRIMITIVE
// ------------

/// Type that can be converted to primitive with `as`.
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
    fn from_u64(value: u64) -> Self;

    #[cfg(feature = "f16")]
    fn as_f16(self) -> f16;

    #[cfg(feature = "f16")]
    fn as_bf16(self) -> bf16;
}

macro_rules! as_primitive {
    ($($t:ty)*) => ($(
        impl AsPrimitive for $t {
            #[inline(always)]
            fn as_u8(self) -> u8 {
                self as u8
            }

            #[inline(always)]
            fn as_u16(self) -> u16 {
                self as u16
            }

            #[inline(always)]
            fn as_u32(self) -> u32 {
                self as u32
            }

            #[inline(always)]
            fn as_u64(self) -> u64 {
                self as u64
            }

            #[inline(always)]
            fn as_u128(self) -> u128 {
                self as u128
            }

            #[inline(always)]
            fn as_usize(self) -> usize {
                self as usize
            }

            #[inline(always)]
            fn as_i8(self) -> i8 {
                self as i8
            }

            #[inline(always)]
            fn as_i16(self) -> i16 {
                self as i16
            }

            #[inline(always)]
            fn as_i32(self) -> i32 {
                self as i32
            }

            #[inline(always)]
            fn as_i64(self) -> i64 {
                self as i64
            }

            #[inline(always)]
            fn as_i128(self) -> i128 {
                self as i128
            }

            #[inline(always)]
            fn as_isize(self) -> isize {
                self as isize
            }

            #[inline(always)]
            fn as_f32(self) -> f32 {
                self as f32
            }

            #[inline(always)]
            fn as_f64(self) -> f64 {
                self as f64
            }

            #[inline(always)]
            fn from_u32(value: u32) -> Self {
                value as Self
            }

            #[inline(always)]
            fn from_u64(value: u64) -> Self {
                value as Self
            }

            #[cfg(feature = "f16")]
            #[inline(always)]
            fn as_f16(self) -> f16 {
                f16::from_f32(self as f32)
            }

            #[cfg(feature = "f16")]
            #[inline(always)]
            fn as_bf16(self) -> bf16 {
                bf16::from_f32(self as f32)
            }
        }
    )*)
}

as_primitive! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

#[cfg(feature = "f16")]
macro_rules! half_as_primitive {
    ($($t:ty)*) => ($(
        impl AsPrimitive for $t {
            #[inline(always)]
            fn as_u8(self) -> u8 {
                self.as_f32() as u8
            }

            #[inline(always)]
            fn as_u16(self) -> u16 {
                self.as_f32() as u16
            }

            #[inline(always)]
            fn as_u32(self) -> u32 {
                self.as_f32() as u32
            }

            #[inline(always)]
            fn as_u64(self) -> u64 {
                self.as_f32() as u64
            }

            #[inline(always)]
            fn as_u128(self) -> u128 {
                self.as_f32() as u128
            }

            #[inline(always)]
            fn as_usize(self) -> usize {
                self.as_f32() as usize
            }

            #[inline(always)]
            fn as_i8(self) -> i8 {
                self.as_f32() as i8
            }

            #[inline(always)]
            fn as_i16(self) -> i16 {
                self.as_f32() as i16
            }

            #[inline(always)]
            fn as_i32(self) -> i32 {
                self.as_f32() as i32
            }

            #[inline(always)]
            fn as_i64(self) -> i64 {
                self.as_f32() as i64
            }

            #[inline(always)]
            fn as_i128(self) -> i128 {
                self.as_f32() as i128
            }

            #[inline(always)]
            fn as_isize(self) -> isize {
                self.as_f32() as isize
            }

            #[inline(always)]
            fn as_f32(self) -> f32 {
                self.as_f32() as f32
            }

            #[inline(always)]
            fn as_f64(self) -> f64 {
                self.as_f32() as f64
            }

            #[inline(always)]
            #[allow(clippy::as_underscore)] // reason="intentionally used in a generic sense"
            fn from_u32(value: u32) -> Self {
                Self::from_f32(value as _)
            }

            #[inline(always)]
            fn from_u64(value: u64) -> Self {
                _ = value;
                unimplemented!()
            }

            #[inline(always)]
            fn as_f16(self) -> f16 {
                f16::from_f32(self.as_f32())
            }

            #[inline(always)]
            fn as_bf16(self) -> bf16 {
                bf16::from_f32(self.as_f32())
            }
        }
    )*)
}

#[cfg(feature = "f16")]
half_as_primitive! { f16 bf16 }

// AS CAST
// -------

/// An interface for casting between machine scalars.
pub trait AsCast: AsPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `AsPrimitive` trait.
    fn as_cast<N: AsPrimitive>(n: N) -> Self;
}

/// Allows the high-level conversion of generic types as if `as` was used.
#[inline(always)]
pub fn as_cast<U: AsCast, T: AsCast>(t: T) -> U {
    U::as_cast(t)
}

macro_rules! as_cast {
    ($($t:ty, $meth:ident ; )*) => ($(
        impl AsCast for $t {
            #[inline(always)]
            #[allow(clippy::as_underscore)] // reason="intentional due to generic API"
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

#[cfg(feature = "f16")]
as_cast!(
    f16, as_f16 ;
    bf16, as_bf16 ;
);

// PRIMITIVE
// ---------

/// Primitive type trait (which all have static lifetimes).
pub trait Primitive: 'static + fmt::Debug + fmt::Display + AsCast {}

macro_rules! primitive {
    ($($t:ty)*) => ($(
        impl Primitive for $t {}
    )*)
}

primitive! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

#[cfg(feature = "f16")]
primitive! { f16 bf16 }

// NUMBER
// ------

/// Numerical type trait.
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
    f32 true ;
    f64 true ;
    // f128 true
}

#[cfg(feature = "f16")]
number_impl! {
    f16 true ;
    bf16 true ;
}

// INTEGER
// -------

/// Defines a trait that supports integral operations.
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
    ops::Not<Output=Self> +
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
    fn pow(self, exp: u32) -> Self;
    fn checked_pow(self, exp: u32) -> Option<Self>;
    fn overflowing_pow(self, exp: u32) -> (Self, bool);
    fn checked_add(self, i: Self) -> Option<Self>;
    fn checked_sub(self, i: Self) -> Option<Self>;
    fn checked_mul(self, i: Self) -> Option<Self>;
    fn overflowing_add(self, i: Self) -> (Self, bool);
    fn overflowing_sub(self, i: Self) -> (Self, bool);
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
    #[inline(always)]
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
    #[inline(always)]
    fn ceil_div(self, y: Self) -> Self {
        self.ceil_divmod(y).0
    }

    /// Get the fast ceiling modulus from integer division.
    /// Not safe, since the remainder can easily overflow.
    #[inline(always)]
    fn ceil_mod(self, y: Self) -> i32 {
        self.ceil_divmod(y).1
    }

    // PROPERTIES

    /// Get the number of bits in a value.
    #[inline(always)]
    fn bit_length(self) -> u32 {
        Self::BITS as u32 - self.leading_zeros()
    }

    /// Returns true if the least-significant bit is odd.
    #[inline(always)]
    fn is_odd(self) -> bool {
        self & Self::ONE == Self::ONE
    }

    /// Returns true if the least-significant bit is even.
    #[inline(always)]
    fn is_even(self) -> bool {
        !self.is_odd()
    }

    /// Get the maximum number of digits before the slice will overflow.
    ///
    /// This is effectively the floor(log(2**BITS-1, radix)), but we can
    /// try to go a bit lower without worrying too much.
    #[inline(always)]
    fn overflow_digits(radix: u32) -> usize {
        // this is heavily optimized for base10 and it's a way under estimate
        // that said, it's fast and works.
        if radix <= 16 {
            mem::size_of::<Self>() * 2 - Self::IS_SIGNED as usize
        } else {
            // way under approximation but always works and is fast
            mem::size_of::<Self>()
        }
    }
}

macro_rules! integer_impl {
($($t:tt)*) => ($(
    impl Integer for $t {
        const ZERO: $t = 0;
        const ONE: $t = 1;
        const TWO: $t = 2;
        const MAX: $t = $t::MAX;
        const MIN: $t = $t::MIN;
        const BITS: usize = $t::BITS as usize;

        #[inline(always)]
        fn leading_zeros(self) -> u32 {
            $t::leading_zeros(self)
        }

        #[inline(always)]
        fn trailing_zeros(self) -> u32 {
            $t::trailing_zeros(self)
        }

        #[inline(always)]
        fn checked_add(self, i: Self) -> Option<Self> {
            $t::checked_add(self, i)
        }

        #[inline(always)]
        fn checked_sub(self, i: Self) -> Option<Self> {
            $t::checked_sub(self, i)
        }

        #[inline(always)]
        fn checked_mul(self, i: Self) -> Option<Self> {
            $t::checked_mul(self, i)
        }

        #[inline(always)]
        fn overflowing_add(self, i: Self) -> (Self, bool) {
            $t::overflowing_add(self, i)
        }

        #[inline(always)]
        fn overflowing_sub(self, i: Self) -> (Self, bool) {
            $t::overflowing_sub(self, i)
        }

        #[inline(always)]
        fn overflowing_mul(self, i: Self) -> (Self, bool) {
            $t::overflowing_mul(self, i)
        }

        #[inline(always)]
        fn wrapping_add(self, i: Self) -> Self {
            $t::wrapping_add(self, i)
        }

        #[inline(always)]
        fn wrapping_sub(self, i: Self) -> Self {
            $t::wrapping_sub(self, i)
        }

        #[inline(always)]
        fn wrapping_mul(self, i: Self) -> Self {
            $t::wrapping_mul(self, i)
        }

        #[inline(always)]
        fn wrapping_neg(self) -> Self {
            $t::wrapping_neg(self)
        }

        #[inline(always)]
        fn pow(self, exp: u32) -> Self {
            Self::pow(self, exp)
        }

        #[inline(always)]
        fn checked_pow(self, exp: u32) -> Option<Self> {
            Self::checked_pow(self, exp)
        }

        #[inline(always)]
        fn overflowing_pow(self, exp: u32) -> (Self, bool) {
            Self::overflowing_pow(self, exp)
        }

        #[inline(always)]
        fn saturating_add(self, i: Self) -> Self {
            $t::saturating_add(self, i)
        }

        #[inline(always)]
        fn saturating_sub(self, i: Self) -> Self {
            $t::saturating_sub(self, i)
        }

        #[inline(always)]
        fn saturating_mul(self, i: Self) -> Self {
            $t::saturating_mul(self, i)
        }
    }
)*)
}

integer_impl! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize }

// SIGNED INTEGER
// --------------

/// Defines a trait that supports signed integral operations.
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
    /// Bitmask for the hidden bit in exponent, which is an implicit 1 in the
    /// fraction.
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
    fn ln(self) -> Self;
    fn floor(self) -> Self;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;

    /// Returns true if the float is a denormal.
    #[inline(always)]
    fn is_denormal(self) -> bool {
        self.to_bits() & Self::EXPONENT_MASK == Self::Unsigned::ZERO
    }

    /// Returns true if the float is a NaN or Infinite.
    #[inline(always)]
    fn is_special(self) -> bool {
        self.to_bits() & Self::EXPONENT_MASK == Self::EXPONENT_MASK
    }

    /// Returns true if the float is NaN.
    #[inline(always)]
    fn is_nan(self) -> bool {
        self.is_special() && (self.to_bits() & Self::MANTISSA_MASK) != Self::Unsigned::ZERO
    }

    /// Returns true if the float is infinite.
    #[inline(always)]
    fn is_inf(self) -> bool {
        self.is_special() && (self.to_bits() & Self::MANTISSA_MASK) == Self::Unsigned::ZERO
    }

    /// Returns true if the float's least-significant mantissa bit is odd.
    #[inline(always)]
    fn is_odd(self) -> bool {
        self.to_bits().is_odd()
    }

    /// Returns true if the float's least-significant mantissa bit is even.
    #[inline(always)]
    fn is_even(self) -> bool {
        !self.is_odd()
    }

    /// Returns true if the float needs a negative sign when serializing it.
    ///
    /// This is true if it's `-0.0` or it's below 0 and not NaN. But inf values
    /// need the sign.
    #[inline(always)]
    fn needs_negative_sign(self) -> bool {
        self.is_sign_negative() && !self.is_nan()
    }

    /// Get exponent component from the float.
    #[inline(always)]
    fn exponent(self) -> i32 {
        if self.is_denormal() {
            return Self::DENORMAL_EXPONENT;
        }

        let bits = self.to_bits();
        let biased_e = i32::as_cast((bits & Self::EXPONENT_MASK) >> Self::MANTISSA_SIZE).as_i32();
        biased_e - Self::EXPONENT_BIAS
    }

    /// Get mantissa (significand) component from float.
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn next_positive(self) -> Self {
        debug_assert!(self.is_sign_positive() && !self.is_inf());
        Self::from_bits(self.to_bits() + Self::Unsigned::ONE)
    }

    /// Get previous greater float, such that `self.prev().next() == self`.
    #[inline(always)]
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
    #[inline(always)]
    fn prev_positive(self) -> Self {
        debug_assert!(self.is_sign_positive() && self != Self::ZERO);
        Self::from_bits(self.to_bits() - Self::Unsigned::ONE)
    }

    /// Round a positive number to even.
    #[inline(always)]
    fn round_positive_even(self) -> Self {
        if self.mantissa().is_odd() {
            self.next_positive()
        } else {
            self
        }
    }

    /// Get the max of two finite numbers.
    #[inline(always)]
    fn max_finite(self, f: Self) -> Self {
        debug_assert!(!self.is_special() && !f.is_special(), "max_finite self={} f={}", self, f);
        if self < f {
            f
        } else {
            self
        }
    }

    /// Get the min of two finite numbers.
    #[inline(always)]
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
//      - f128

#[cfg(feature = "f16")]
macro_rules! float_one {
    ($f:ident) => {
        (($f::EXPONENT_BIAS - $f::MANTISSA_SIZE) as u16) << $f::MANTISSA_SIZE
    };
}

#[cfg(feature = "f16")]
macro_rules! float_two {
    ($f:ident) => {
        (($f::EXPONENT_BIAS - $f::MANTISSA_SIZE + 1) as u16) << $f::MANTISSA_SIZE
    };
}

#[cfg(feature = "f16")]
macro_rules! float_max {
    ($f:ident) => {
        ($f::EXPONENT_MASK ^ $f::HIDDEN_BIT_MASK) | $f::MANTISSA_MASK
    };
}

#[cfg(feature = "f16")]
macro_rules! float_min {
    ($f:ident) => {
        $f::MAX.to_bits() | $f::SIGN_MASK
    };
}

#[cfg(feature = "f16")]
macro_rules! float_nan {
    ($f:ident) => {
        $f::EXPONENT_MASK | ($f::HIDDEN_BIT_MASK >> 1)
    };
}

#[cfg(feature = "f16")]
impl Float for f16 {
    type Unsigned = u16;

    const ZERO: Self = Self::from_bits(0);
    const ONE: Self = Self::from_bits(float_one!(Self));
    const TWO: Self = Self::from_bits(float_two!(Self));
    const MAX: Self = Self::from_bits(float_max!(Self));
    const MIN: Self = Self::from_bits(float_min!(Self));
    const INFINITY: Self = Self::from_bits(Self::INFINITY_BITS);
    const NEG_INFINITY: Self = Self::from_bits(Self::NEGATIVE_INFINITY_BITS);
    const NAN: Self = Self::from_bits(float_nan!(Self));
    const BITS: usize = mem::size_of::<Self>() * 8;

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

    #[inline(always)]
    fn to_bits(self) -> u16 {
        f16::to_bits(self)
    }

    #[inline(always)]
    fn from_bits(u: u16) -> f16 {
        f16::from_bits(u)
    }

    #[inline(always)]
    fn ln(self) -> f16 {
        f16::from_f32(self.as_f32().ln())
    }

    #[inline(always)]
    fn floor(self) -> f16 {
        f16::from_f32(self.as_f32().floor())
    }

    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        self.to_bits() & Self::SIGN_MASK == 0
    }

    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        !self.is_sign_positive()
    }
}

#[cfg(feature = "f16")]
impl Float for bf16 {
    type Unsigned = u16;

    const ZERO: Self = Self::from_bits(0);
    const ONE: Self = Self::from_bits(float_one!(Self));
    const TWO: Self = Self::from_bits(float_two!(Self));
    const MAX: Self = Self::from_bits(float_max!(Self));
    const MIN: Self = Self::from_bits(float_min!(Self));
    const INFINITY: Self = Self::from_bits(Self::INFINITY_BITS);
    const NEG_INFINITY: Self = Self::from_bits(Self::NEGATIVE_INFINITY_BITS);
    const NAN: Self = Self::from_bits(float_nan!(Self));
    const BITS: usize = mem::size_of::<Self>() * 8;

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

    #[inline(always)]
    fn to_bits(self) -> u16 {
        bf16::to_bits(self)
    }

    #[inline(always)]
    fn from_bits(u: u16) -> bf16 {
        bf16::from_bits(u)
    }

    #[inline(always)]
    fn ln(self) -> bf16 {
        bf16::from_f32(self.as_f32().ln())
    }

    #[inline(always)]
    fn floor(self) -> bf16 {
        bf16::from_f32(self.as_f32().floor())
    }

    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        self.to_bits() & Self::SIGN_MASK == 0
    }

    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        !self.is_sign_positive()
    }
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

    #[inline(always)]
    fn to_bits(self) -> u32 {
        f32::to_bits(self)
    }

    #[inline(always)]
    fn from_bits(u: u32) -> f32 {
        f32::from_bits(u)
    }

    #[inline(always)]
    fn ln(self) -> f32 {
        #[cfg(feature = "std")]
        return f32::ln(self);

        #[cfg(not(feature = "std"))]
        return logf(self);
    }

    #[inline(always)]
    fn floor(self) -> f32 {
        #[cfg(feature = "std")]
        return f32::floor(self);

        #[cfg(not(feature = "std"))]
        return floorf(self);
    }

    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        f32::is_sign_positive(self)
    }

    #[inline(always)]
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

    #[inline(always)]
    fn to_bits(self) -> u64 {
        f64::to_bits(self)
    }

    #[inline(always)]
    fn from_bits(u: u64) -> f64 {
        f64::from_bits(u)
    }

    #[inline(always)]
    fn ln(self) -> f64 {
        #[cfg(feature = "std")]
        return f64::ln(self);

        #[cfg(not(feature = "std"))]
        return logd(self);
    }

    #[inline(always)]
    fn floor(self) -> f64 {
        #[cfg(feature = "std")]
        return f64::floor(self);

        #[cfg(not(feature = "std"))]
        return floord(self);
    }

    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        f64::is_sign_positive(self)
    }

    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        f64::is_sign_negative(self)
    }
}

// #[cfg(feature = "f128")]
// impl Float for f128 {
//     type Unsigned = u128;
//     float_literals!(f128);
//     float_masks!(
//         float => Self,
//         sign_mask => 0x80000000000000000000000000000000,
//         exponent_mask => 0x7FFF0000000000000000000000000000,
//         hidden_bit_mask => 0x00010000000000000000000000000000,
//         mantissa_mask => 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFF,
//     );
//     const EXPONENT_SIZE: i32 = 15;
//     const MANTISSA_SIZE: i32 = 112;
//     const EXPONENT_BIAS: i32 = 16383 + Self::MANTISSA_SIZE;
//     const DENORMAL_EXPONENT: i32 = 1 - Self::EXPONENT_BIAS;
//     const MAX_EXPONENT: i32 = 0x7FFF - Self::EXPONENT_BIAS;
// }

// FLOAT HELPERS
// -------------

// These are adapted from libm, a port of musl libc's libm to Rust.
// libm can be found online [here](https://github.com/rust-lang/libm),
// and is similarly licensed under an Apache2.0/MIT license

/// # Safety
///
/// Safe as long as `e` is properly initialized.
#[cfg(all(not(feature = "std"), feature = "floats"))]
macro_rules! volatile {
($e:expr) => {
    // SAFETY: safe as long as `$e` has been properly initialized.
    unsafe {
        core::ptr::read_volatile(&$e);
    }
};
}

/// Floor (f64)
///
/// Finds the nearest integer less than or equal to `x`.
#[cfg(all(not(feature = "std"), feature = "floats"))]
fn floord(x: f64) -> f64 {
    const TOINT: f64 = 1. / f64::EPSILON;

    let ui = x.to_bits();
    let e = ((ui >> 52) & 0x7ff) as i32;

    if (e >= 0x3ff + 52) || (x == 0.) {
        return x;
    }
    /* y = int(x) - x, where int(x) is an integer neighbor of x */
    let y = if (ui >> 63) != 0 {
        x - TOINT + TOINT - x
    } else {
        x + TOINT - TOINT - x
    };
    /* special case because of non-nearest rounding modes */
    if e < 0x3ff {
        volatile!(y);
        return if (ui >> 63) != 0 {
            -1.
        } else {
            0.
        };
    }
    if y > 0. {
        x + y - 1.
    } else {
        x + y
    }
}

/// Floor (f32)
///
/// Finds the nearest integer less than or equal to `x`.
#[cfg(all(not(feature = "std"), feature = "floats"))]
fn floorf(x: f32) -> f32 {
    let mut ui = x.to_bits();
    let e = (((ui >> 23) as i32) & 0xff) - 0x7f;

    if e >= 23 {
        return x;
    }
    if e >= 0 {
        let m: u32 = 0x007fffff >> e;
        if (ui & m) == 0 {
            return x;
        }
        volatile!(x + f32::from_bits(0x7b800000));
        if ui >> 31 != 0 {
            ui += m;
        }
        ui &= !m;
    } else {
        volatile!(x + f32::from_bits(0x7b800000));
        if ui >> 31 == 0 {
            ui = 0;
        } else if ui << 1 != 0 {
            return -1.0;
        }
    }
    f32::from_bits(ui)
}

/* origin: FreeBSD /usr/src/lib/msun/src/e_log.c */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */
/* log(x)
 * Return the logarithm of x
 *
 * Method :
 *   1. Argument Reduction: find k and f such that
 *                      x = 2^k * (1+f),
 *         where  sqrt(2)/2 < 1+f < sqrt(2) .
 *
 *   2. Approximation of log(1+f).
 *      Let s = f/(2+f) ; based on log(1+f) = log(1+s) - log(1-s)
 *               = 2s + 2/3 s**3 + 2/5 s**5 + .....,
 *               = 2s + s*R
 *      We use a special Remez algorithm on [0,0.1716] to generate
 *      a polynomial of degree 14 to approximate R The maximum error
 *      of this polynomial approximation is bounded by 2**-58.45. In
 *      other words,
 *                      2      4      6      8      10      12      14
 *          R(z) ~ Lg1*s +Lg2*s +Lg3*s +Lg4*s +Lg5*s  +Lg6*s  +Lg7*s
 *      (the values of Lg1 to Lg7 are listed in the program)
 *      and
 *          |      2          14          |     -58.45
 *          | Lg1*s +...+Lg7*s    -  R(z) | <= 2
 *          |                             |
 *      Note that 2s = f - s*f = f - hfsq + s*hfsq, where hfsq = f*f/2.
 *      In order to guarantee error in log below 1ulp, we compute log
 *      by
 *              log(1+f) = f - s*(f - R)        (if f is not too large)
 *              log(1+f) = f - (hfsq - s*(hfsq+R)).     (better accuracy)
 *
 *      3. Finally,  log(x) = k*ln2 + log(1+f).
 *                          = k*ln2_hi+(f-(hfsq-(s*(hfsq+R)+k*ln2_lo)))
 *         Here ln2 is split into two floating point number:
 *                      ln2_hi + ln2_lo,
 *         where n*ln2_hi is always exact for |n| < 2000.
 *
 * Special cases:
 *      log(x) is NaN with signal if x < 0 (including -INF) ;
 *      log(+INF) is +INF; log(0) is -INF with signal;
 *      log(NaN) is that NaN with no signal.
 *
 * Accuracy:
 *      according to an error analysis, the error is always less than
 *      1 ulp (unit in the last place).
 *
 * Constants:
 * The hexadecimal values are the intended ones for the following
 * constants. The decimal values may be used, provided that the
 * compiler will convert from decimal to binary accurately enough
 * to produce the hexadecimal values shown.
 */

#[allow(clippy::eq_op, clippy::excessive_precision)] // reason="values need to be exact under all conditions"
#[cfg(all(not(feature = "std"), feature = "floats"))]
fn logd(mut x: f64) -> f64 {
    const LN2_HI: f64 = 6.93147180369123816490e-01; /* 3fe62e42 fee00000 */
    const LN2_LO: f64 = 1.90821492927058770002e-10; /* 3dea39ef 35793c76 */
    const LG1: f64 = 6.666666666666735130e-01; /* 3FE55555 55555593 */
    const LG2: f64 = 3.999999999940941908e-01; /* 3FD99999 9997FA04 */
    const LG3: f64 = 2.857142874366239149e-01; /* 3FD24924 94229359 */
    const LG4: f64 = 2.222219843214978396e-01; /* 3FCC71C5 1D8E78AF */
    const LG5: f64 = 1.818357216161805012e-01; /* 3FC74664 96CB03DE */
    const LG6: f64 = 1.531383769920937332e-01; /* 3FC39A09 D078C69F */
    const LG7: f64 = 1.479819860511658591e-01; /* 3FC2F112 DF3E5244 */

    let x1p54 = f64::from_bits(0x4350000000000000); // 0x1p54 === 2 ^ 54

    let mut ui = x.to_bits();
    let mut hx: u32 = (ui >> 32) as u32;
    let mut k: i32 = 0;

    if (hx < 0x00100000) || ((hx >> 31) != 0) {
        /* x < 2**-126 */
        if ui << 1 == 0 {
            return -1. / (x * x); /* log(+-0)=-inf */
        }
        if hx >> 31 != 0 {
            return (x - x) / 0.0; /* log(-#) = NaN */
        }
        /* subnormal number, scale x up */
        k -= 54;
        x *= x1p54;
        ui = x.to_bits();
        hx = (ui >> 32) as u32;
    } else if hx >= 0x7ff00000 {
        return x;
    } else if hx == 0x3ff00000 && ui << 32 == 0 {
        return 0.;
    }

    /* reduce x into [sqrt(2)/2, sqrt(2)] */
    hx += 0x3ff00000 - 0x3fe6a09e;
    k += ((hx >> 20) as i32) - 0x3ff;
    hx = (hx & 0x000fffff) + 0x3fe6a09e;
    ui = ((hx as u64) << 32) | (ui & 0xffffffff);
    x = f64::from_bits(ui);

    let f: f64 = x - 1.0;
    let hfsq: f64 = 0.5 * f * f;
    let s: f64 = f / (2.0 + f);
    let z: f64 = s * s;
    let w: f64 = z * z;
    let t1: f64 = w * (LG2 + w * (LG4 + w * LG6));
    let t2: f64 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    let r: f64 = t2 + t1;
    let dk: f64 = k as f64;
    s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI
}

/* origin: FreeBSD /usr/src/lib/msun/src/e_logf.c */
/*
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 */
/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

#[allow(clippy::eq_op, clippy::excessive_precision)] // reason="values need to be exact under all conditions"
#[cfg(all(not(feature = "std"), feature = "floats"))]
fn logf(mut x: f32) -> f32 {
    const LN2_HI: f32 = 6.9313812256e-01; /* 0x3f317180 */
    const LN2_LO: f32 = 9.0580006145e-06; /* 0x3717f7d1 */
    /* |(log(1+s)-log(1-s))/s - Lg(s)| < 2**-34.24 (~[-4.95e-11, 4.97e-11]). */
    const LG1: f32 = 0.66666662693; /* 0xaaaaaa.0p-24 */
    const LG2: f32 = 0.40000972152; /* 0xccce13.0p-25 */
    const LG3: f32 = 0.28498786688; /* 0x91e9ee.0p-25 */
    const LG4: f32 = 0.24279078841; /* 0xf89e26.0p-26 */

    let x1p25 = f32::from_bits(0x4c000000); // 0x1p25f === 2 ^ 25

    let mut ix = x.to_bits();
    let mut k = 0i32;

    if (ix < 0x00800000) || ((ix >> 31) != 0) {
        /* x < 2**-126 */
        if ix << 1 == 0 {
            return -1. / (x * x); /* log(+-0)=-inf */
        }
        if (ix >> 31) != 0 {
            return (x - x) / 0.; /* log(-#) = NaN */
        }
        /* subnormal number, scale up x */
        k -= 25;
        x *= x1p25;
        ix = x.to_bits();
    } else if ix >= 0x7f800000 {
        return x;
    } else if ix == 0x3f800000 {
        return 0.;
    }

    /* reduce x into [sqrt(2)/2, sqrt(2)] */
    ix += 0x3f800000 - 0x3f3504f3;
    k += ((ix >> 23) as i32) - 0x7f;
    ix = (ix & 0x007fffff) + 0x3f3504f3;
    x = f32::from_bits(ix);

    let f = x - 1.;
    let s = f / (2. + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG2 + w * LG4);
    let t2 = z * (LG1 + w * LG3);
    let r = t2 + t1;
    let hfsq = 0.5 * f * f;
    let dk = k as f32;
    s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI
}
