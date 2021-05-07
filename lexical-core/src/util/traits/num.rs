//! Utilities for Rust numbers.
//!
//! These traits define useful properties, methods, associated
//! types, and trait bounds, and conversions for working with
//! numbers in generic code.

use crate::lib::{f32, f64, fmt, iter, mem, ops};
use crate::options::*;  // Refactor when we move options.
use crate::util::config::*;
use crate::util::options::*;

use super::cast::AsCast;
use super::primitive::Primitive;
use super::sequence::CloneableVecLike;

#[cfg(any(not(feature = "no_alloc"), feature = "f128", feature = "radix"))]
use crate::lib::Vec;

// NUMBER
// ------

/// Numerical type trait.
#[doc(hidden)]
pub trait Number:
    Primitive +
    // Iteration
    iter::Product + iter::Sum +
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
    /// Maximum number of bytes required to serialize a number to string.
    const FORMATTED_SIZE: usize;
    /// Maximum number of bytes required to serialize a number to a decimal string.
    const FORMATTED_SIZE_DECIMAL: usize;
    /// Is the type signed.
    const IS_SIGNED: bool;

    // OPTIONS
    type WriteOptions;
    type ParseOptions;
}

macro_rules! number_impl {
    ($($t:tt $radix_size:ident $decimal_size:ident $is_signed:literal $write_options:ident $parse_options:ident ; )*) => ($(
        impl Number for $t {
            const FORMATTED_SIZE: usize = $radix_size;
            const FORMATTED_SIZE_DECIMAL: usize = $decimal_size;
            const IS_SIGNED: bool = $is_signed;

            type WriteOptions = $write_options;
            type ParseOptions = $parse_options;
        }
    )*)
}

number_impl! {
    u8 U8_FORMATTED_SIZE U8_FORMATTED_SIZE_DECIMAL false WriteIntegerOptions ParseIntegerOptions ;
    u16 U16_FORMATTED_SIZE U16_FORMATTED_SIZE_DECIMAL false WriteIntegerOptions ParseIntegerOptions ;
    u32 U32_FORMATTED_SIZE U32_FORMATTED_SIZE_DECIMAL false WriteIntegerOptions ParseIntegerOptions ;
    u64 U64_FORMATTED_SIZE U64_FORMATTED_SIZE_DECIMAL false WriteIntegerOptions ParseIntegerOptions ;
    u128 U128_FORMATTED_SIZE U128_FORMATTED_SIZE_DECIMAL false WriteIntegerOptions ParseIntegerOptions ;
    usize USIZE_FORMATTED_SIZE USIZE_FORMATTED_SIZE_DECIMAL false WriteIntegerOptions ParseIntegerOptions ;
    i8 I8_FORMATTED_SIZE I8_FORMATTED_SIZE_DECIMAL true WriteIntegerOptions ParseIntegerOptions ;
    i16 I16_FORMATTED_SIZE I16_FORMATTED_SIZE_DECIMAL true WriteIntegerOptions ParseIntegerOptions ;
    i32 I32_FORMATTED_SIZE I32_FORMATTED_SIZE_DECIMAL true WriteIntegerOptions ParseIntegerOptions ;
    i64 I64_FORMATTED_SIZE I64_FORMATTED_SIZE_DECIMAL true WriteIntegerOptions ParseIntegerOptions ;
    i128 I128_FORMATTED_SIZE I128_FORMATTED_SIZE_DECIMAL true WriteIntegerOptions ParseIntegerOptions ;
    isize ISIZE_FORMATTED_SIZE ISIZE_FORMATTED_SIZE_DECIMAL true WriteIntegerOptions ParseIntegerOptions ;
    // f16
    // bf16
    f32 F32_FORMATTED_SIZE F32_FORMATTED_SIZE_DECIMAL true WriteFloatOptions ParseFloatOptions ;
    f64 F64_FORMATTED_SIZE F64_FORMATTED_SIZE_DECIMAL true WriteFloatOptions ParseFloatOptions ;
    // f128
}

// INTEGER
// -------

/// Defines a trait that supports integral operations.
#[doc(hidden)]
pub trait Integer:
    // Basic
    Number + Eq + Ord +
    // Display
    fmt::Binary + fmt::Octal + fmt::LowerHex + fmt::UpperHex +
    fmt::Display + fmt::Debug +
    //Operations
    ops::BitAnd<Output=Self> +
    ops::BitAndAssign +
    ops::BitOr<Output=Self> +
    ops::BitOrAssign +
    ops::BitXor<Output=Self> +
    ops::BitXorAssign +
    ops::Not +
    ops::Shl<Self, Output=Self> +
    ops::Shl<u8, Output=Self> +
    ops::Shl<u16, Output=Self> +
    ops::Shl<u32, Output=Self> +
    ops::Shl<u64, Output=Self> +
    ops::Shl<usize, Output=Self> +
    ops::Shl<i8, Output=Self> +
    ops::Shl<i16, Output=Self> +
    ops::Shl<i64, Output=Self> +
    ops::Shl<isize, Output=Self> +
    ops::Shl<i32, Output=Self> +
    ops::ShlAssign<Self> +
    ops::ShlAssign<u8> +
    ops::ShlAssign<u16> +
    ops::ShlAssign<u32> +
    ops::ShlAssign<u64> +
    ops::ShlAssign<usize> +
    ops::ShlAssign<i8> +
    ops::ShlAssign<i16> +
    ops::ShlAssign<i32> +
    ops::ShlAssign<i64> +
    ops::ShlAssign<isize> +
    ops::Shr<Self, Output=Self> +
    ops::Shr<u8, Output=Self> +
    ops::Shr<u16, Output=Self> +
    ops::Shr<u32, Output=Self> +
    ops::Shr<u64, Output=Self> +
    ops::Shr<usize, Output=Self> +
    ops::Shr<i8, Output=Self> +
    ops::Shr<i16, Output=Self> +
    ops::Shr<i64, Output=Self> +
    ops::Shr<isize, Output=Self> +
    ops::Shr<i32, Output=Self> +
    ops::ShrAssign<Self> +
    ops::ShrAssign<u8> +
    ops::ShrAssign<u16> +
    ops::ShrAssign<u32> +
    ops::ShrAssign<u64> +
    ops::ShrAssign<usize> +
    ops::ShrAssign<i8> +
    ops::ShrAssign<i16> +
    ops::ShrAssign<i32> +
    ops::ShrAssign<i64> +
    ops::ShrAssign<isize>
{
    // CONSTANTS
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;
    const MIN: Self;
    const BITS: usize;

    // FUNCTIONS (INHERITED)
    fn max_value() -> Self;
    fn min_value() -> Self;
    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn pow(self, i: u32) -> Self;
    fn checked_add(self, i: Self) -> Option<Self>;
    fn checked_sub(self, i: Self) -> Option<Self>;
    fn checked_mul(self, i: Self) -> Option<Self>;
    fn checked_div(self, i: Self) -> Option<Self>;
    fn checked_rem(self, i: Self) -> Option<Self>;
    fn checked_neg(self) -> Option<Self>;
    fn checked_shl(self, i: u32) -> Option<Self>;
    fn checked_shr(self, i: u32) -> Option<Self>;
    fn wrapping_add(self, i: Self) -> Self;
    fn wrapping_sub(self, i: Self) -> Self;
    fn wrapping_mul(self, i: Self) -> Self;
    fn wrapping_div(self, i: Self) -> Self;
    fn wrapping_rem(self, i: Self) -> Self;
    fn wrapping_neg(self) -> Self;
    fn wrapping_shl(self, i: u32) -> Self;
    fn wrapping_shr(self, i: u32) -> Self;
    fn overflowing_add(self, i: Self) -> (Self, bool);
    fn overflowing_sub(self, i: Self) -> (Self, bool);
    fn overflowing_mul(self, i: Self) -> (Self, bool);
    fn overflowing_div(self, i: Self) -> (Self, bool);
    fn overflowing_rem(self, i: Self) -> (Self, bool);
    fn overflowing_neg(self) -> (Self, bool);
    fn overflowing_shl(self, i: u32) -> (Self, bool);
    fn overflowing_shr(self, i: u32) -> (Self, bool);
    fn saturating_add(self, i: Self) -> Self;
    fn saturating_sub(self, i: Self) -> Self;
    fn saturating_mul(self, i: Self) -> Self;

    /// Create literal zero.
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    /// Create literal one.
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    /// Create literal two.
    #[inline]
    fn two() -> Self {
        Self::TWO
    }

    /// Check if value is equal to zero.
    #[inline]
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    /// Check if value is equal to one.
    #[inline]
    fn is_one(self) -> bool {
        self == Self::ONE
    }

    // OPERATIONS

    /// Get the fast ceiling of the quotient from integer division.
    /// Not safe, since the remainder can easily overflow.
    #[inline]
    fn ceil_divmod(self, y: Self) -> (Self, i32) {
        let q = self / y;
        let r = self % y;
        match r.is_zero() {
            true  => (q, r.as_i32()),
            false => (q + Self::ONE, r.as_i32() - y.as_i32())
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
}

macro_rules! integer_impl {
    ($($t:tt)*) => ($(
        impl Integer for $t {
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TWO: $t = 2;
            const MAX: $t = $t::max_value();
            const MIN: $t = $t::min_value();
            const BITS: usize = mem::size_of::<$t>() * 8;

            #[inline]
            fn max_value() -> Self {
                Self::MAX
            }

            #[inline]
            fn min_value() -> Self {
                Self::MIN
            }

            #[inline]
            fn count_ones(self) -> u32 {
                $t::count_ones(self)
            }

            #[inline]
            fn count_zeros(self) -> u32 {
                $t::count_zeros(self)
            }

            #[inline]
            fn leading_zeros(self) -> u32 {
                $t::leading_zeros(self)
            }

            #[inline]
            fn trailing_zeros(self) -> u32 {
                $t::trailing_zeros(self)
            }

            #[inline]
            fn pow(self, i: u32) -> Self {
                $t::pow(self, i)
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
            fn checked_div(self, i: Self) -> Option<Self> {
                $t::checked_div(self, i)
            }

            #[inline]
            fn checked_rem(self, i: Self) -> Option<Self> {
                $t::checked_rem(self, i)
            }

            #[inline]
            fn checked_neg(self) -> Option<Self> {
                $t::checked_neg(self)
            }

            #[inline]
            fn checked_shl(self, i: u32) -> Option<Self> {
                $t::checked_shl(self,i)
            }

            #[inline]
            fn checked_shr(self, i: u32) -> Option<Self> {
                $t::checked_shr(self,i)
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
            fn wrapping_div(self, i: Self) -> Self {
                $t::wrapping_div(self, i)
            }

            #[inline]
            fn wrapping_rem(self, i: Self) -> Self {
                $t::wrapping_rem(self, i)
            }

            #[inline]
            fn wrapping_neg(self) -> Self {
                $t::wrapping_neg(self)
            }

            #[inline]
            fn wrapping_shl(self, i: u32) -> Self {
                $t::wrapping_shl(self,i)
            }

            #[inline]
            fn wrapping_shr(self, i: u32) -> Self {
                $t::wrapping_shr(self,i)
            }

            #[inline]
            fn overflowing_add(self, i: Self) -> (Self, bool) {
                $t::overflowing_add(self, i)
            }

            #[inline]
            fn overflowing_sub(self, i: Self) -> (Self, bool) {
                $t::overflowing_sub(self, i)
            }

            #[inline]
            fn overflowing_mul(self, i: Self) -> (Self, bool) {
                $t::overflowing_mul(self, i)
            }

            #[inline]
            fn overflowing_div(self, i: Self) -> (Self, bool) {
                $t::overflowing_div(self, i)
            }

            #[inline]
            fn overflowing_rem(self, i: Self) -> (Self, bool) {
                $t::overflowing_rem(self, i)
            }

            #[inline]
            fn overflowing_neg(self) -> (Self, bool) {
                $t::overflowing_neg(self)
            }

            #[inline]
            fn overflowing_shl(self, i: u32) -> (Self, bool) {
                $t::overflowing_shl(self,i)
            }

            #[inline]
            fn overflowing_shr(self, i: u32) -> (Self, bool) {
                $t::overflowing_shr(self,i)
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

/// Unwrap or get T::max_value().
#[inline]
pub(crate) fn unwrap_or_max<T: Integer>(t: Option<T>) -> T {
    t.unwrap_or(T::max_value())
}

// SIGNED INTEGER
// --------------

/// Defines a trait that supports signed integral operations.
#[doc(hidden)]
pub trait SignedInteger: Integer + ops::Neg<Output = Self> {
    // FUNCTIONS (INHERITED)
    fn checked_abs(self) -> Option<Self>;
    fn wrapping_abs(self) -> Self;
    fn overflowing_abs(self) -> (Self, bool);
}

macro_rules! signed_integer_impl {
    ($($t:tt)*) => ($(
        impl SignedInteger for $t {
            fn checked_abs(self) -> Option<Self> {
                $t::checked_abs(self)
            }

            fn wrapping_abs(self) -> Self {
                $t::wrapping_abs(self)
            }

            fn overflowing_abs(self) -> (Self, bool) {
                $t::overflowing_abs(self)
            }
        }
    )*)
}

signed_integer_impl! { i8 i16 i32 i64 i128 isize }

// UNSIGNED INTEGER
// ----------------

/// Defines a trait that supports unsigned integral operations.
#[doc(hidden)]
pub trait UnsignedInteger: Integer {
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
pub trait Float: Number + ops::Neg<Output = Self> {
    /// Unsigned type of the same size.
    type Unsigned: UnsignedInteger;

    /// The mantissa type for fast and moderate path parsing algorithms.
    ///
    /// This type is guaranteed to be at least as large as Unsigned,
    /// although for implementation reasons, it may be larger.
    type Mantissa: UnsignedInteger;

    /// Number of limbs in a Bigint.
    ///
    /// This number is somewhat arbitrary, but needs
    /// to be at least the number of bits required to store
    /// a Bigint, which is log2(10) * digits, adjusted to the limb size.
    ///
    /// Since we reserve at least 20 digits in the default constructor,
    /// this must be at least 20. This constant is mostly present
    /// to ensure BigintStorage is correct.
    const BIGINT_LIMBS: usize;

    /// Number of limbs in a Bigfloat.
    ///
    /// This number is somewhat arbitrary, but needs
    /// to be at least the number of bits required to store
    /// a Bigfloat, which is log2(10) * digits, adjusted to the limb size.
    ///
    /// Since we reserve at least 10 digits in the default constructor,
    /// this must be at least 10. This constant is mostly present
    /// to ensure BigfloatStorage is correct.
    const BIGFLOAT_LIMBS: usize;

    /// The storage type for the Bigint.
    type BigintStorage: CloneableVecLike<Limb>;
    /// The storage type for the Bigfloat.
    type BigfloatStorage: CloneableVecLike<Limb>;

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
    const CARRY_MASK: Self::Mantissa;

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
    fn abs(self) -> Self;
    fn ceil(self) -> Self;
    fn exp(self) -> Self;
    fn floor(self) -> Self;
    fn ln(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, f: Self) -> Self;
    fn round(self) -> Self;
    fn to_bits(self) -> Self::Unsigned;
    fn from_bits(u: Self::Unsigned) -> Self;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;

    // FUNCTIONS

    /// Check if value is equal to zero.
    #[inline]
    fn is_zero(self) -> bool {
        // IEEE754 guarantees `+0.0 == -0.0`, and Rust respects this,
        // unlike some other languages.
        self == Self::ZERO
    }

    /// Check if value is equal to one.
    #[inline]
    fn is_one(self) -> bool {
        self == Self::ONE
    }

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
        self.is_special() && !(self.to_bits() & Self::MANTISSA_MASK).is_zero()
    }

    /// Returns true if the float is infinite.
    #[inline]
    fn is_inf(self) -> bool {
        self.is_special() && (self.to_bits() & Self::MANTISSA_MASK).is_zero()
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
        let biased_e: i32 = AsCast::as_cast((bits & Self::EXPONENT_MASK) >> Self::MANTISSA_SIZE);
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
        if self.is_sign_negative() && self.is_zero() {
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
        if self.is_sign_positive() && self.is_zero() {
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
        debug_assert!(self.is_sign_positive() && !self.is_zero());
        return Self::from_bits(self.to_bits() - Self::Unsigned::ONE);
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

/// Wrap float method for `std` and `no_std` context.
macro_rules! float_method {
    ($f:ident, $t:tt, $meth:ident, $libm:ident $(,$i:expr)*) => ({
        #[cfg(feature = "std")]
        return $t::$meth($f $(,$i)*);

        #[cfg(not(feature = "std"))]
        return libm::$libm($f $(,$i)*);
    })
}

/// Define the float literals.
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
        const CARRY_MASK: <$float>::Mantissa = $hidden << 1;
        // Infinity is always every exponent bit set.
        const INFINITY_BITS: <$float>::Unsigned = $exponent;
        // Negative infinity is just infinity + sign.
        const NEGATIVE_INFINITY_BITS: <$float>::Unsigned = $exponent | $sign;
    };
}

/// Define the limb sizes.
macro_rules! float_limbs {
    (
        bigint32_size =>
        $bigint32:literal,bigint64_size =>
        $bigint64:literal,bigfloat32_size =>
        $bigfloat32:literal,bigfloat64_size =>
        $bigfloat64:literal,
    ) => {
        cfg_if! {
        if #[cfg(limb_width_64)] {
            const BIGINT_LIMBS: usize = $bigint64;
            const BIGFLOAT_LIMBS: usize = $bigfloat64;
        } else {
            const BIGINT_LIMBS: usize = $bigint32;
            const BIGFLOAT_LIMBS: usize = $bigfloat64;
        }} // cfg_if
    };
}

/// Define the float storage types.
macro_rules! float_storage {
    (
        bigint32_size => $bigint32:literal,
        bigint64_size => $bigint64:literal,
        bigfloat32_size => $bigfloat32:literal,
        bigfloat64_size => $bigfloat64:literal,
    ) => (
        cfg_if! {
        if #[cfg(any(not(feature = "no_alloc"), feature = "radix"))] {
            type BigintStorage = Vec<Limb>;
        } else if #[cfg(limb_width_64)] {
            type BigintStorage = arrayvec::ArrayVec<[Limb; $bigint64]>;
        } else {
            type BigintStorage = arrayvec::ArrayVec<[Limb; $bigint32]>;
        }} // cfg_if

        cfg_if! {
        if #[cfg(not(feature = "no_alloc"))] {
            type BigfloatStorage = Vec<Limb>;
        } else if #[cfg(limb_width_64)] {
            type BigfloatStorage = arrayvec::ArrayVec<[Limb; $bigfloat64]>;
        } else {
            type BigfloatStorage = arrayvec::ArrayVec<[Limb; $bigfloat64]>;
        }} // cfg_if

        float_limbs!(
            bigint32_size => $bigint32,
            bigint64_size => $bigint64,
            bigfloat32_size => $bigfloat32,
            bigfloat64_size => $bigfloat64,
        );
    );
}

// TODO(ahuszagh)
//  Due to missing specifics or types for the following float types,
//  `Float` is not yet fully implemented for:
//      - f16
//      - bf16
//      - f128

#[cfg(feature = "f16")]
impl Float for f16 {
    type Unsigned = u16;
    type Mantissa = u64;
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

    float_storage!(
        bigint32_size => 20,
        bigint64_size => 20,
        bigfloat32_size => 10,
        bigfloat64_size => 10,
    );
}

#[cfg(feature = "f16")]
impl Float for bf16 {
    type Unsigned = u16;
    type Mantissa = u64;
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

    float_storage!(
        bigint32_size => 20,
        bigint64_size => 20,
        bigfloat32_size => 20,
        bigfloat64_size => 10,
    );
}

impl Float for f32 {
    type Unsigned = u32;
    type Mantissa = u64;
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

    float_storage!(
        bigint32_size => 20,
        bigint64_size => 20,
        bigfloat32_size => 20,
        bigfloat64_size => 10,
    );

    #[inline]
    fn abs(self) -> f32 {
        float_method!(self, f32, abs, fabsf)
    }

    #[inline]
    fn ceil(self) -> f32 {
        float_method!(self, f32, ceil, ceilf)
    }

    #[inline]
    fn exp(self) -> f32 {
        float_method!(self, f32, exp, expf)
    }

    #[inline]
    fn floor(self) -> f32 {
        float_method!(self, f32, floor, floorf)
    }

    #[inline]
    fn ln(self) -> f32 {
        float_method!(self, f32, ln, logf)
    }

    #[inline]
    fn powi(self, n: i32) -> f32 {
        cfg_if! {
        if #[cfg(not(feature = "std"))] {
            self.powf(n as f32)
        } else {
            f32::powi(self, n)
        }} // cfg_if
    }

    #[inline]
    fn powf(self, n: f32) -> f32 {
        float_method!(self, f32, powf, powf, n)
    }

    #[inline]
    fn round(self) -> f32 {
        float_method!(self, f32, round, roundf)
    }

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

impl Float for f64 {
    type Unsigned = u64;
    type Mantissa = u64;
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

    float_storage!(
        bigint32_size => 128,
        bigint64_size => 64,
        bigfloat32_size => 36,
        bigfloat64_size => 20,
    );

    #[inline]
    fn abs(self) -> f64 {
        float_method!(self, f64, abs, fabs)
    }

    #[inline]
    fn ceil(self) -> f64 {
        float_method!(self, f64, ceil, ceil)
    }

    #[inline]
    fn exp(self) -> f64 {
        float_method!(self, f64, exp, exp)
    }

    #[inline]
    fn floor(self) -> f64 {
        float_method!(self, f64, floor, floor)
    }

    #[inline]
    fn ln(self) -> f64 {
        float_method!(self, f64, ln, log)
    }

    #[inline]
    fn powi(self, n: i32) -> f64 {
        cfg_if! {
        if #[cfg(not(feature = "std"))] {
            self.powf(n as f64)
        } else {
            f64::powi(self, n)
        }} // cfg_if
    }

    #[inline]
    fn powf(self, n: f64) -> f64 {
        float_method!(self, f64, powf, pow, n)
    }

    #[inline]
    fn round(self) -> f64 {
        float_method!(self, f64, round, round)
    }

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

#[cfg(feature = "f128")]
impl Float for f128 {
    type Unsigned = u128;
    type Mantissa = u128;
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

    type BigintStorage = Vec<Limb>;
    type BigfloatStorage = Vec<Limb>;

    float_limbs!(
        bigint32_size => 1800,
        bigint64_size => 900,
        bigfloat32_size => 1400,
        bigfloat64_size => 700,
    );
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    fn check_number<T: Number>(x: T, mut y: T) {
        // Copy, partialeq, partialord
        let _ = x;
        assert!(x < y);
        assert!(x != y);

        // Operations
        let _ = y + x;
        let _ = y - x;
        let _ = y * x;
        let _ = y / x;
        let _ = y % x;
        y += x;
        y -= x;
        y *= x;
        y /= x;
        y %= x;

        // Conversions already tested.
    }

    #[test]
    fn number_test() {
        check_number(1u8, 5);
        check_number(1u16, 5);
        check_number(1u32, 5);
        check_number(1u64, 5);
        check_number(1u128, 5);
        check_number(1usize, 5);
        check_number(1i8, 5);
        check_number(1i16, 5);
        check_number(1i32, 5);
        check_number(1i64, 5);
        check_number(1i128, 5);
        check_number(1isize, 5);
        check_number(1f32, 5.0);
        check_number(1f64, 5.0);
    }

    fn check_integer<T: Integer>(mut x: T) {
        // Copy, partialeq, partialord, ord, eq
        let _ = x;
        assert!(x > T::ONE);
        assert!(x != T::ONE);
        assert_eq!(x.min(T::ONE), T::ONE);
        assert_eq!(x.max(T::ONE), x);

        // Operations
        let _ = x + T::ONE;
        let _ = x - T::ONE;
        let _ = x * T::ONE;
        let _ = x / T::ONE;
        let _ = x % T::ONE;
        x += T::ONE;
        x -= T::ONE;
        x *= T::ONE;
        x /= T::ONE;
        x %= T::ONE;

        // Bitwise operations
        let _ = x & T::ONE;
        let _ = x | T::ONE;
        let _ = x ^ T::ONE;
        let _ = !x;
        x &= T::ONE;
        x |= T::ONE;
        x ^= T::ONE;

        // Bitshifts
        let _ = x << 1u8;
        let _ = x << 1u16;
        let _ = x << 1u32;
        let _ = x << 1u64;
        let _ = x << 1usize;
        let _ = x << 1i8;
        let _ = x << 1i16;
        let _ = x << 1i32;
        let _ = x << 1i64;
        let _ = x << 1isize;
        let _ = x >> 1u8;
        let _ = x >> 1u16;
        let _ = x >> 1u32;
        let _ = x >> 1u64;
        let _ = x >> 1usize;
        let _ = x >> 1i8;
        let _ = x >> 1i16;
        let _ = x >> 1i32;
        let _ = x >> 1i64;
        let _ = x >> 1isize;
        x <<= 1u8;
        x <<= 1u16;
        x <<= 1u32;
        x <<= 1u64;
        x <<= 1usize;
        x <<= 1i8;
        x <<= 1i16;
        x <<= 1i32;
        x <<= 1i64;
        x <<= 1isize;
        x >>= 1u8;
        x >>= 1u16;
        x >>= 1u32;
        x >>= 1u64;
        x >>= 1usize;
        x >>= 1i8;
        x >>= 1i16;
        x >>= 1i32;
        x >>= 1i64;
        x >>= 1isize;

        // Functions
        assert!(T::ZERO.is_zero());
        assert!(!T::ONE.is_zero());
        assert!(T::ONE.is_one());
        assert!(!T::ZERO.is_one());

        // Conversions already tested.
    }

    #[test]
    fn integer_test() {
        check_integer(65u8);
        check_integer(65u16);
        check_integer(65u32);
        check_integer(65u64);
        check_integer(65u128);
        check_integer(65usize);
        check_integer(65i8);
        check_integer(65i16);
        check_integer(65i32);
        check_integer(65i64);
        check_integer(65i128);
        check_integer(65isize);
    }

    #[test]
    fn ceil_divmod_test() {
        assert_eq!(5usize.ceil_divmod(7), (1, -2));
        assert_eq!(0usize.ceil_divmod(7), (0, 0));
        assert_eq!(35usize.ceil_divmod(7), (5, 0));
        assert_eq!(36usize.ceil_divmod(7), (6, -6));
    }

    #[test]
    fn unwrap_or_max_test() {
        let x: Option<u8> = None;
        assert_eq!(unwrap_or_max(x), u8::max_value());

        let x: Option<u8> = Some(1);
        assert_eq!(unwrap_or_max(x), 1);
    }

    fn check_float<T: Float>(mut x: T) {
        // Copy, partialeq, partialord
        let _ = x;
        assert!(x > T::ONE);
        assert!(x != T::ONE);

        // Operations
        let _ = x + T::ONE;
        let _ = x - T::ONE;
        let _ = x * T::ONE;
        let _ = x / T::ONE;
        let _ = x % T::ONE;
        let _ = -x;
        x += T::ONE;
        x -= T::ONE;
        x *= T::ONE;
        x /= T::ONE;
        x %= T::ONE;

        // Check functions
        let _ = x.abs();
        let _ = x.ceil();
        let _ = x.exp();
        let _ = x.floor();
        let _ = x.ln();
        let _ = x.powi(5);
        let _ = x.powf(T::ONE);
        let _ = x.round();
        let _ = x.to_bits();
        assert_eq!(T::from_bits(x.to_bits()), x);
        let _ = x.is_sign_positive();
        let _ = x.is_sign_negative();

        // Check properties
        let _ = x.to_bits() & T::SIGN_MASK;
        let _ = x.to_bits() & T::EXPONENT_MASK;
        let _ = x.to_bits() & T::HIDDEN_BIT_MASK;
        let _ = x.to_bits() & T::MANTISSA_MASK;
        assert!(T::from_bits(T::INFINITY_BITS).is_special());
    }

    #[test]
    fn float_test() {
        check_float(123f32);
        check_float(123f64);

        // b00000000000000000000000000000001
        let f: f32 = 1e-45;
        assert!(f.is_odd());
        assert!(f.next().is_even());
        assert!(f.next_positive().is_even());
        assert!(f.prev().is_even());
        assert!(f.prev_positive().is_even());
        assert!(f.round_positive_even().is_even());
        assert_eq!(f.prev().next(), f);
        assert_eq!(f.prev_positive().next_positive(), f);
        assert_eq!(f.round_positive_even(), f.next());

        // b00111101110011001100110011001101
        let f: f32 = 0.1;
        assert!(f.is_odd());
        assert!(f.next().is_even());
        assert!(f.next_positive().is_even());
        assert!(f.prev().is_even());
        assert!(f.prev_positive().is_even());
        assert!(f.round_positive_even().is_even());
        assert_eq!(f.prev().next(), f);
        assert_eq!(f.prev_positive().next_positive(), f);
        assert_eq!(f.round_positive_even(), f.next());

        // b01000000000000000000000000000000
        let f: f32 = 1.0;
        assert!(f.is_even());
        assert!(f.next().is_odd());
        assert!(f.next_positive().is_odd());
        assert!(f.prev().is_odd());
        assert!(f.prev_positive().is_odd());
        assert!(f.round_positive_even().is_even());
        assert_eq!(f.prev().next(), f);
        assert_eq!(f.prev_positive().next_positive(), f);
        assert_ne!(f.round_positive_even(), f.next());
    }
}
