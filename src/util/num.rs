//! Utilities for Rust numbers.

pub(crate) use lib::{f32, f64};
use lib::{fmt, iter, ops};
use super::cast::{AsCast, TryCast};
use super::primitive::Primitive;

// NUMBER

/// Numerical type trait.
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
{}

macro_rules! number_impl {
    ($($t:ty)*) => ($(
        impl Number for $t {}
    )*)
}

number_impl! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }

// INTEGER

/// Defines a trait that supports integral operations.
pub trait Integer:
    // Basic
    Number + Eq + Ord +
    // Display
    fmt::Octal + fmt::LowerHex + fmt::UpperHex +
    //Operations
    ops::BitAnd<Output=Self> +
    ops::BitAndAssign +
    ops::BitOr<Output=Self> +
    ops::BitOrAssign +
    ops::BitXor<Output=Self> +
    ops::BitXorAssign +
    ops::Not +
    ops::Shl<Output=Self> +
    ops::Shl<u8, Output=Self> +
    ops::Shl<u16, Output=Self> +
    ops::Shl<u32, Output=Self> +
    ops::Shl<u64, Output=Self> +
    ops::Shl<usize, Output=Self> +
    ops::Shl<i8, Output=Self> +
    ops::Shl<i16, Output=Self> +
    ops::Shl<i32, Output=Self> +
    ops::Shl<i64, Output=Self> +
    ops::Shl<isize, Output=Self> +
    ops::ShlAssign +
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
    ops::Shr<Output=Self> +
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
    ops::ShrAssign +
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
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const MAX: Self;
    const MIN: Self;

    // FUNCTIONS (INHERITED)
    fn max_value() -> Self;
    fn min_value() -> Self;
    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
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

    /// Create literal zero.
    #[inline(always)]
    fn zero() -> Self {
        Self::ZERO
    }

    /// Create literal one.
    #[inline(always)]
    fn one() -> Self {
        Self::ONE
    }

    /// Create literal two.
    #[inline(always)]
    fn two() -> Self {
        Self::TWO
    }

    /// Check if value is equal to zero.
    #[inline(always)]
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    /// Check if value is equal to one.
    #[inline(always)]
    fn is_one(self) -> bool {
        self == Self::ONE
    }

    // TRY CAST OR MAX

    #[inline(always)]
    fn try_u8_or_max(self) -> u8 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_u16_or_max(self) -> u16 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_u32_or_max(self) -> u32 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_u64_or_max(self) -> u64 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_u128_or_max(self) -> u128 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_usize_or_max(self) -> usize {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_i8_or_max(self) -> i8 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_i16_or_max(self) -> i16 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_i32_or_max(self) -> i32 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_i64_or_max(self) -> i64 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_i128_or_max(self) -> i128 {
        try_cast_or_max(self)
    }

    #[inline(always)]
    fn try_isize_or_max(self) -> isize {
        try_cast_or_max(self)
    }

    // TRY CAST OR MIN

    #[inline(always)]
    fn try_u8_or_min(self) -> u8 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_u16_or_min(self) -> u16 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_u32_or_min(self) -> u32 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_u64_or_min(self) -> u64 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_u128_or_min(self) -> u128 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_usize_or_min(self) -> usize {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_i8_or_min(self) -> i8 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_i16_or_min(self) -> i16 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_i32_or_min(self) -> i32 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_i64_or_min(self) -> i64 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_i128_or_min(self) -> i128 {
        try_cast_or_min(self)
    }

    #[inline(always)]
    fn try_isize_or_min(self) -> isize {
        try_cast_or_min(self)
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

            #[inline(always)]
            fn max_value() -> Self {
                Self::MAX
            }

            #[inline(always)]
            fn min_value() -> Self {
                Self::MIN
            }

            #[inline(always)]
            fn count_ones(self) -> u32 {
                $t::count_ones(self)
            }

            #[inline(always)]
            fn count_zeros(self) -> u32 {
                $t::count_zeros(self)
            }

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
            fn checked_div(self, i: Self) -> Option<Self> {
                $t::checked_div(self, i)
            }

            #[inline(always)]
            fn checked_rem(self, i: Self) -> Option<Self> {
                $t::checked_rem(self, i)
            }

            #[inline(always)]
            fn checked_neg(self) -> Option<Self> {
                $t::checked_neg(self)
            }

            #[inline(always)]
            fn checked_shl(self, i: u32) -> Option<Self> {
                $t::checked_shl(self,i)
            }

            #[inline(always)]
            fn checked_shr(self, i: u32) -> Option<Self> {
                $t::checked_shr(self,i)
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
            fn wrapping_div(self, i: Self) -> Self {
                $t::wrapping_div(self, i)
            }

            #[inline(always)]
            fn wrapping_rem(self, i: Self) -> Self {
                $t::wrapping_rem(self, i)
            }

            #[inline(always)]
            fn wrapping_neg(self) -> Self {
                $t::wrapping_neg(self)
            }

            #[inline(always)]
            fn wrapping_shl(self, i: u32) -> Self {
                $t::wrapping_shl(self,i)
            }

            #[inline(always)]
            fn wrapping_shr(self, i: u32) -> Self {
                $t::wrapping_shr(self,i)
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
            fn overflowing_div(self, i: Self) -> (Self, bool) {
                $t::overflowing_div(self, i)
            }

            #[inline(always)]
            fn overflowing_rem(self, i: Self) -> (Self, bool) {
                $t::overflowing_rem(self, i)
            }

            #[inline(always)]
            fn overflowing_neg(self) -> (Self, bool) {
                $t::overflowing_neg(self)
            }

            #[inline(always)]
            fn overflowing_shl(self, i: u32) -> (Self, bool) {
                $t::overflowing_shl(self,i)
            }

            #[inline(always)]
            fn overflowing_shr(self, i: u32) -> (Self, bool) {
                $t::overflowing_shr(self,i)
            }
        }
    )*)
}

integer_impl! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

/// Unwrap or get T::max_value().
#[inline(always)]
pub fn unwrap_or_max<T: Integer>(t: Option<T>) -> T {
    t.unwrap_or(T::max_value())
}

/// Unwrap or get T::min_value().
#[inline(always)]
pub fn unwrap_or_min<T: Integer>(t: Option<T>) -> T {
    t.unwrap_or(T::min_value())
}

/// Try to convert to U, if not, return U::max_value().
#[inline(always)]
#[allow(dead_code)]
pub fn try_cast_or_max<U: Integer, T: TryCast<U>>(t: T) -> U {
    unwrap_or_max(TryCast::try_cast(t))
}

/// Try to convert to U, if not, return U::min_value().
#[inline(always)]
#[allow(dead_code)]
pub fn try_cast_or_min<U: Integer, T: TryCast<U>>(t: T) -> U {
    unwrap_or_min(TryCast::try_cast(t))
}

// SIGNED INTEGER

/// Defines a trait that supports signed integral operations.
pub trait SignedInteger: Integer + ops::Neg<Output=Self>
{
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

/// Defines a trait that supports unsigned integral operations.
pub trait UnsignedInteger: Integer
{}

macro_rules! unsigned_integer_impl {
    ($($t:ty)*) => ($(
        impl UnsignedInteger for $t {}
    )*)
}

unsigned_integer_impl! { u8 u16 u32 u64 u128 usize }

// FLOAT

/// Float information for native float types.
pub trait Float: Number + ops::Neg<Output=Self>
{
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

    /// Bitmask for the sign bit.
    const SIGN_MASK: Self::Unsigned;
    /// Bitmask for the exponent, including the hidden bit.
    const EXPONENT_MASK: Self::Unsigned;
    /// Bitmask for the hidden bit in exponent, which is an implicit 1 in the fraction.
    const HIDDEN_BIT_MASK: Self::Unsigned;
    /// Bitmask for the mantissa (fraction), excluding the hidden bit.
    const MANTISSA_MASK: Self::Unsigned;

    // PROPERTIES

    /// Positive infinity as bits.
    const INFINITY_BITS: Self::Unsigned;
    /// Size of the significand (mantissa) without hidden bit.
    const MANTISSA_SIZE: i32;
    /// Bias of the exponet
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
        if self.is_sign_negative() && self.mantissa().is_zero() {
            // -0.0
            Self::ZERO
        } else if self.is_sign_negative() {
            Self::from_bits(bits - Self::Unsigned::ONE)
        } else {
            Self::from_bits(bits + Self::Unsigned::ONE)
        }
    }

    /// Get next greater float for a positive float.
    #[inline]
    fn next_positive(self) -> Self {
        debug_assert!(self.is_sign_positive());

        let bits = self.to_bits();
        if bits == Self::INFINITY_BITS {
            return Self::from_bits(Self::INFINITY_BITS);
        }
        return Self::from_bits(bits + Self::Unsigned::ONE);
    }

    /// Get the max of two finite numbers.
    #[inline]
    fn max_finite(self, f: Self) -> Self {
        debug_assert!(!self.is_special() && !f.is_special(), "max_finite self={} f={}",  self, f);
        if self < f { f } else { self }
    }

    /// Get the min of two finite numbers.
    #[inline]
    fn min_finite(self, f: Self) -> Self {
        debug_assert!(!self.is_special() && !f.is_special(), "min_finite self={} f={}",  self, f);
        if self < f { self } else { f }
    }
}

/// Wrap float method for `std` and `no_std` context.
macro_rules! float_method {
    ($f:ident, $t:tt, $meth:ident, $intr:ident $(,$i:expr)*) => ({
        #[cfg(feature = "std")]
        return $t::$meth($f $(,$i)*);

        #[cfg(not(feature = "std"))]
        return unsafe { core::intrinsics::$intr($f $(,$i)*) };
    })
}

/// Wrap float method with special conditions for MSVC.
///
/// This is because MSVC wraps these as inline functions, with no actual
/// ABI for the LLVM intrinsic.
macro_rules! float_method_msvc {
    ($f:ident, $ts:tt, $tl:tt, $meth:ident, $intr:ident $(,$i:expr)*) => ({
        #[cfg(feature = "std")]
        return $ts::$meth($f $(,$i)*);

        #[cfg(all(not(feature = "std"), not(target_env = "msvc")))]
        return unsafe { core::intrinsics::$intr($f $(,$i)*) };

        #[cfg(all(not(feature = "std"), target_env = "msvc"))]
        return ($f as $tl).$meth() as $ts;
    })
}

/// Wrap float log method for `no_std` context, with special conditions for Solaris.
///
/// Solaris has a standard non-conforming log implementation, we need
/// to wrap this cheaply.
macro_rules! float_method_log_solaris {
    ($f:ident, $t:tt, $meth:ident, $intr:ident $(,$i:expr)*) => ({
        #[cfg(feature = "std")]
        return $t::$meth($f $(,$i)*);

        #[cfg(all(not(feature = "std"), not(target_os = "solaris")))]
        return unsafe { core::intrinsics::$intr($f $(,$i)*) };

        // Workaround for Solaris/Illumos due to log(-value) == -Inf, not NaN.
        #[cfg(all(not(feature = "std"), target_os = "solaris"))] {
            if $f.is_nan() {
                $f
            } else if $f.is_special() {
                if $f > $t::ZERO { $f } else { $t::NAN }
            } else if $f > $t::ZERO {
                unsafe { core::intrinsics::$intr($f $(,$i)*) }
            } else if $f.is_zero() {
                $t::NEG_INFINITY
            } else {
                $t::NAN
            }
        }
    })
}

impl Float for f32 {
    type Unsigned = u32;
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
    const TWO: f32 = 2.0;
    const MAX: f32 = f32::MAX;
    const MIN: f32 = f32::MIN;
    const INFINITY: f32 = f32::INFINITY;
    const NEG_INFINITY: f32 = f32::NEG_INFINITY;
    const NAN: f32 = f32::NAN;
    const SIGN_MASK: u32            = 0x80000000;
    const EXPONENT_MASK: u32        = 0x7F800000;
    const HIDDEN_BIT_MASK: u32      = 0x00800000;
    const MANTISSA_MASK: u32        = 0x007FFFFF;
    const INFINITY_BITS: u32        = 0x7F800000;
    const MANTISSA_SIZE: i32        = 23;
    const EXPONENT_BIAS: i32        = 127 + Self::MANTISSA_SIZE;
    const DENORMAL_EXPONENT: i32    = 1 - Self::EXPONENT_BIAS;
    const MAX_EXPONENT: i32         = 0xFF - Self::EXPONENT_BIAS;

    #[inline(always)]
    fn abs(self) -> f32 {
        float_method!(self, f32, abs, fabsf32)
    }

    #[inline(always)]
    fn ceil(self) -> f32 {
        float_method_msvc!(self, f32, f64, ceil, ceilf32)
    }

    #[inline(always)]
    fn exp(self) -> f32 {
        float_method_msvc!(self, f32, f64, exp, expf32)
    }

    #[inline(always)]
    fn floor(self) -> f32 {
        float_method_msvc!(self, f32, f64, floor, floorf32)
    }

    #[inline(always)]
    fn ln(self) -> f32 {
        float_method_msvc!(self, f32, f64, ln, logf32)
    }

    #[inline(always)]
    fn powi(self, n: i32) -> f32 {
        float_method!(self, f32, powi, powif32, n)
    }

    #[inline(always)]
    fn powf(self, n: f32) -> f32 {
        float_method_msvc!(self, f32, f64, powf, powf32, n as f32)
    }

    #[inline(always)]
    fn round(self) -> f32 {
        float_method!(self, f32, round, roundf32)
    }

    #[inline(always)]
    fn to_bits(self) -> u32 {
        f32::to_bits(self)
    }

    #[inline(always)]
    fn from_bits(u: u32) -> f32 {
        f32::from_bits(u)
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

impl Float for f64 {
    type Unsigned = u64;
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
    const TWO: f64 = 2.0;
    const MAX: f64 = f64::MAX;
    const MIN: f64 = f64::MIN;
    const INFINITY: f64 = f64::INFINITY;
    const NEG_INFINITY: f64 = f64::NEG_INFINITY;
    const NAN: f64 = f64::NAN;
    const SIGN_MASK: u64            = 0x8000000000000000;
    const EXPONENT_MASK: u64        = 0x7FF0000000000000;
    const HIDDEN_BIT_MASK: u64      = 0x0010000000000000;
    const MANTISSA_MASK: u64        = 0x000FFFFFFFFFFFFF;
    const INFINITY_BITS: u64        = 0x7FF0000000000000;
    const MANTISSA_SIZE: i32        = 52;
    const EXPONENT_BIAS: i32        = 1023 + Self::MANTISSA_SIZE;
    const DENORMAL_EXPONENT: i32    = 1 - Self::EXPONENT_BIAS;
    const MAX_EXPONENT: i32         = 0x7FF - Self::EXPONENT_BIAS;

    #[inline(always)]
    fn abs(self) -> f64 {
        float_method!(self, f64, abs, fabsf64)
    }

    #[inline(always)]
    fn ceil(self) -> f64 {
        float_method!(self, f64, ceil, ceilf64)
    }

    #[inline(always)]
    fn exp(self) -> f64 {
        float_method!(self, f64, exp, expf64)
    }

    #[inline(always)]
    fn floor(self) -> f64 {
        float_method!(self, f64, floor, floorf64)
    }

    #[inline(always)]
    fn ln(self) -> f64 {
        float_method_log_solaris!(self, f64, ln, logf64)
    }

    #[inline(always)]
    fn powi(self, n: i32) -> f64 {
        float_method!(self, f64, powi, powif64, n)
    }

    #[inline(always)]
    fn powf(self, n: f64) -> f64 {
        float_method!(self, f64, powf, powf64, n)
    }

    #[inline(always)]
    fn round(self) -> f64 {
        float_method!(self, f64, round, roundf64)
    }

    #[inline(always)]
    fn to_bits(self) -> u64 {
        f64::to_bits(self)
    }

    #[inline(always)]
    fn from_bits(u: u64) -> f64 {
        f64::from_bits(u)
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
    fn unwrap_or_max_test() {
        let x: Option<u8> = None;
        assert_eq!(unwrap_or_max(x), u8::max_value());

        let x: Option<u8> = Some(1);
        assert_eq!(unwrap_or_max(x), 1);
    }

    #[test]
    fn unwrap_or_min_test() {
        let x: Option<u8> = None;
        assert_eq!(unwrap_or_min(x), u8::min_value());

        let x: Option<u8> = Some(1);
        assert_eq!(unwrap_or_min(x), 1);
    }

    #[test]
    fn try_cast_or_max_test() {
        let x: u8 = try_cast_or_max(u16::min_value());
        assert_eq!(x, u8::min_value());

        let x: u8 = try_cast_or_max(u8::max_value() as u16);
        assert_eq!(x, u8::max_value());

        let x: u8 = try_cast_or_max(u16::max_value());
        assert_eq!(x, u8::max_value());
    }

    #[test]
    fn try_cast_or_min_test() {
        let x: u8 = try_cast_or_min(u16::min_value());
        assert_eq!(x, u8::min_value());

        let x: u8 = try_cast_or_min(u8::max_value() as u16);
        assert_eq!(x, u8::max_value());

        let x: u8 = try_cast_or_min(u16::max_value());
        assert_eq!(x, u8::min_value());
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
    }
}
