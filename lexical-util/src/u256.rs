//! An unsigned 256-bit integer type.
//!
//! This aims to have feature parity with Rust's unsigned
//! integer types, such as [u32][core::u32]. The documentation
//! is based off of [u32][core::u32] for each method/member.

use core::cmp::Ordering;
use core::{fmt, mem};
use core::iter::{Product, Sum};
use core::ops::*;
use core::num::ParseIntError;
use core::str::FromStr;

use crate::error::TryFromIntError;
use crate::i256::i256;
use crate::numtypes::*;
use crate::math;
// TODO: Document
// TODO: Feature gate this...

// FIXME: Add support for [Saturating][core::num::Saturating] and
// [Wrapping][core::num::Wrapping] when we drop support for <1.74.0.

/// The 256-bit unsigned integer type.
///
/// This has a stable binary representation for C, but the
/// high and low words depend on the target endianness.
/// Conversion to and from big endian should be done via
/// [`to_le_bytes`] and [`to_be_bytes`], or using [`get_high`]
/// and [`get_low`].
///
/// [`to_le_bytes`]: [u256::to_le_bytes]
/// [`to_be_bytes`]: [u256::to_be_bytes]
/// [`get_high`]: [u256::get_high]
/// [`get_low`]: [u256::get_low]
#[repr(C)]
#[cfg(target_endian = "little")]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct u256 {
    pub(crate) lo: u128,
    pub(crate) hi: u128,
}

/// The 256-bit unsigned integer type.
///
/// This has a stable binary representation for C, but the
/// high and low words depend on the target endianness.
/// Conversion to and from big endian should be done via
/// [`to_le_bytes`] and [`to_be_bytes`].
///
/// [`to_le_bytes`]: [u256::to_le_bytes]
/// [`to_be_bytes`]: [u256::to_be_bytes]
#[repr(C)]
#[cfg(target_endian = "big")]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct u256 {
    pub(crate) hi: u128,
    pub(crate) lo: u128,
}

impl u256 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Self { lo: 0, hi: 0 };

    /// The largest value that can be represented by this integer type
    /// (2<sup>256</sup> - 1).
    pub const MAX: Self = not(Self::MIN);

    /// The size of this integer type in bits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use lexical_util::u256::u256;
    /// assert_eq!(u256::BITS, 256);
    /// ```
    pub const BITS: u32 = 256;

    /// Returns the number of ones in the binary representation of `self`.
    #[inline(always)]
    pub const fn count_ones(self) -> u32 {
        self.hi.count_ones() + self.lo.count_ones()
    }

    /// Returns the number of zeros in the binary representation of `self`.
    #[inline(always)]
    pub const fn count_zeros(self) -> u32 {
        not(self).count_ones()
    }

    /// Returns the number of leading zeros in the binary representation of `self`.
    ///
    /// Depending on what you're doing with the value, you might also be
    /// interested in the `ilog2` function which returns a consistent
    /// number, even if the type widens.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use lexical_util::u256::u256;
    /// let n = u256::MAX >> 2i32;
    /// assert_eq!(n.leading_zeros(), 2);
    ///
    /// let zero = u256::MIN;
    /// assert_eq!(zero.leading_zeros(), 256);
    ///
    /// let max = u256::MAX;
    /// assert_eq!(max.leading_zeros(), 0);
    /// ```
    #[inline(always)]
    pub const fn leading_zeros(self) -> u32 {
        let mut leading = self.hi.leading_zeros();
        if leading == u128::BITS {
            leading += self.lo.leading_zeros()
        }
        leading
    }

    /// Returns the number of trailing zeros in the binary representation of `self`.
    #[inline(always)]
    pub const fn trailing_zeros(self) -> u32 {
        let mut trailing = self.hi.trailing_zeros();
        if trailing == u128::BITS {
            trailing += self.lo.trailing_zeros()
        }
        trailing
    }

    /// Returns the number of leading ones in the binary representation of `self`.
    #[inline(always)]
    pub const fn leading_ones(self) -> u32 {
       not(self).leading_zeros()
    }

    /// Returns the number of trailing ones in the binary representation of `self`.
    #[inline(always)]
    pub const fn trailing_ones(self) -> u32 {
       not(self).trailing_zeros()
    }

    /// Shifts the bits to the left by a specified amount, `n`,
    /// wrapping the truncated bits to the end of the resulting integer.
    ///
    /// Please note this isn't the same operation as the `<<` shifting operator!
    #[inline(always)]
    pub const fn rotate_left(self, n: u32) -> Self {
        let (lo, hi) = math::rotate_left_u128(self.lo, self.hi, n);
        Self { lo, hi }
    }

    /// Shifts the bits to the right by a specified amount, `n`,
    /// wrapping the truncated bits to the beginning of the resulting
    /// integer.
    ///
    /// Please note this isn't the same operation as the `>>` shifting operator!
    #[inline(always)]
    pub const fn rotate_right(self, n: u32) -> Self {
        let (lo, hi) = math::rotate_right_u128(self.lo, self.hi, n);
        Self { lo, hi }
    }

    /// Reverses the byte order of the integer.
    #[inline(always)]
    pub const fn swap_bytes(self) -> Self {
        Self { hi: self.lo.swap_bytes(), lo: self.hi.swap_bytes() }
    }

    /// Reverses the order of bits in the integer. The least significant
    /// bit becomes the most significant bit, second least-significant bit
    /// becomes second most-significant bit, etc.
    #[inline(always)]
    pub const fn reverse_bits(self) -> Self {
        Self { hi: self.lo.reverse_bits(), lo: self.hi.reverse_bits() }
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are
    /// swapped.
    #[inline(always)]
    pub const fn from_be(x: Self) -> Self {
        if cfg!(target_endian = "big") {
            x
        } else {
            x.swap_bytes()
        }
    }

    /// Converts an integer from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are
    /// swapped.
    #[inline(always)]
    pub const fn from_le(x: Self) -> Self {
        if cfg!(target_endian = "little") {
            x
        } else {
            x.swap_bytes()
        }
    }

    /// Converts `self` to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are
    /// swapped.
    #[inline(always)]
    pub const fn to_be(self) -> Self {
        if cfg!(target_endian = "big") {
            self
        } else {
            self.swap_bytes()
        }
    }

    /// Converts `self` to little endian from the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are
    /// swapped.
    #[inline(always)]
    pub const fn to_le(self) -> Self {
        if cfg!(target_endian = "little") {
            self
        } else {
            self.swap_bytes()
        }
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None`
    /// if overflow occurred.
    #[inline(always)]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let (value, overflowed) = self.overflowing_add(rhs);
        if !overflowed {
            Some(value)
        } else {
            None
        }
    }

    /// Checked addition with a signed integer. Computes `self + rhs`,
    /// returning `None` if overflow occurred.
    #[inline(always)]
    pub const fn checked_add_signed(self, rhs: i256) -> Option<Self> {
        let (value, overflowed) = self.overflowing_add_signed(rhs);
        if !overflowed {
            Some(value)
        } else {
            None
        }
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None`
    /// if overflow occurred.
    #[inline(always)]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        let (value, overflowed) = self.overflowing_sub(rhs);
        if !overflowed {
            Some(value)
        } else {
            None
        }
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning `None`
    /// if overflow occurred.
    #[inline(always)]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        let (value, overflowed) = self.overflowing_mul(rhs);
        if !overflowed {
            Some(value)
        } else {
            None
        }
    }

    /// Checked integer division. Computes `self / rhs`, returning `None`
    /// if `rhs == 0`.
    #[inline(always)]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        if eq(rhs, Self::MIN) {
            None
        } else {
            Some(div(self, rhs))
        }
    }

    /// Performs Euclidean division.
    ///
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this
    /// is exactly equal to `self / rhs`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        div(self, rhs)
    }

    /// Checked Euclidean division. Computes `self.div_euclid(rhs)`,
    /// returning `None` if `rhs == 0`.
    #[inline(always)]
    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        if eq(rhs, Self::MIN) {
            None
        } else {
            Some(self.div_euclid(rhs))
        }
    }

    /// Checked integer division. Computes `self % rhs`, returning `None`
    /// if `rhs == 0`.
    #[inline(always)]
    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
        if eq(rhs, Self::MIN) {
            None
        } else {
            Some(rem(self, rhs))
        }
    }

    /// Calculates the least remainder of `self (mod rhs)`.
    ///
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this
    /// is exactly equal to `self % rhs`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        rem(self, rhs)
    }

    /// Checked Euclidean modulo. Computes `self.rem_euclid(rhs)`,
    /// returning `None` if `rhs == 0`.
    #[inline(always)]
    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        if eq(rhs, Self::MIN) {
            None
        } else {
            Some(self.rem_euclid(rhs))
        }
    }

    /// Returns the logarithm of the number with respect to an arbitrary base,
    /// rounded down.
    ///
    /// This method might not be optimized owing to implementation details;
    /// `ilog2` can produce results more efficiently for base 2, and `ilog10`
    /// can produce results more efficiently for base 10.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero, or if `base` is less than 2.
    #[inline(always)]
    pub const fn ilog(self, base: Self) -> u32 {
        if let Some(log) = self.checked_ilog(base) {
            log
        } else {
            panic!("argument of integer logarithm must be positive")
        }
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero.
    #[inline(always)]
    pub const fn ilog2(self) -> u32 {
        if let Some(log) = self.checked_ilog2() {
            log
        } else {
            panic!("argument of integer logarithm must be positive")
        }
    }

    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero.
    #[inline(always)]
    pub const fn ilog10(self) -> u32 {
        if let Some(log) = self.checked_ilog10() {
            log
        } else {
            panic!("argument of integer logarithm must be positive")
        }
    }

    /// Returns the logarithm of the number with respect to an arbitrary base,
    /// rounded down.
    ///
    /// Returns `None` if the number is zero, or if the base is not at least 2.
    ///
    /// This method might not be optimized owing to implementation details;
    /// `checked_ilog2` can produce results more efficiently for base 2, and
    /// `checked_ilog10` can produce results more efficiently for base 10.
    #[inline(always)]
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        todo!();
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// Returns `None` if the number is zero.
    #[inline(always)]
    pub const fn checked_ilog2(self) -> Option<u32> {
        todo!();
    }

    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// Returns `None` if the number is zero.
    #[inline(always)]
    pub const fn checked_ilog10(self) -> Option<u32> {
        match eq(self, Self::from_u8(0)) {
            true => None,
            false => Some(todo!()),
        }
    }

    /// Checked negation. Computes `-self`, returning `None` unless `self ==
    /// 0`.
    ///
    /// Note that negating any positive integer will overflow.
    #[inline(always)]
    pub const fn checked_neg(self) -> Option<Self> {
        if eq(self, Self::MIN) {
            Some(self)
        } else {
            None
        }
    }

    /// Checked shift left. Computes `self << rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    #[inline(always)]
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        if rhs < Self::BITS {
            Some(self.shl_u32(rhs))
        } else {
            None
        }
    }

    /// Checked shift right. Computes `self >> rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    #[inline(always)]
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        if rhs < Self::BITS {
            Some(self.shr_u32(rhs))
        } else {
            None
        }
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None`
    /// if overflow occurred.
    #[inline(always)]
    pub const fn checked_pow(self, base: u32) -> Option<Self> {
        todo!();
    }

    /// Saturating integer addition. Computes `self + rhs`, saturating at
    /// the numeric bounds instead of overflowing.
    #[inline(always)]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        match self.checked_add(rhs) {
            Some(v) => v,
            None => Self::MAX,
        }
    }

    /// Saturating addition with a signed integer. Computes `self + rhs`,
    /// saturating at the numeric bounds instead of overflowing.
    #[inline(always)]
    pub const fn saturating_add_signed(self, rhs: i256) -> Self {
        let is_negative = rhs.hi < 0;
        let (r, overflowed) = self.overflowing_add(Self::from_i256(rhs));
        if overflowed == is_negative {
            r
        } else if overflowed {
            Self::MAX
        } else {
            Self::MIN
        }
    }

    /// Saturating integer subtraction. Computes `self - rhs`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline(always)]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        match self.checked_sub(rhs) {
            Some(v) => v,
            None => Self::MAX,
        }
    }

    /// Saturating integer multiplication. Computes `self * rhs`,
    /// saturating at the numeric bounds instead of overflowing.
    #[inline(always)]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        match self.checked_mul(rhs) {
            Some(v) => v,
            None => Self::MAX,
        }
    }

    /// Saturating integer division. Computes `self / rhs`, saturating at the
    /// numeric bounds instead of overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn saturating_div(self, rhs: Self) -> Self {
        // on unsigned types, there is no overflow in integer division
        self.wrapping_div(rhs)
    }

    /// Saturating integer exponentiation. Computes `self.pow(exp)`,
    /// saturating at the numeric bounds instead of overflowing.
    #[inline]
    pub const fn saturating_pow(self, exp: u32) -> Self {
        match self.checked_pow(exp) {
            Some(x) => x,
            None => Self::MAX,
        }
    }

    /// Wrapping (modular) addition. Computes `self + rhs`,
    /// wrapping around at the boundary of the type.
    #[inline(always)]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        add(self, rhs)
    }

    /// Wrapping (modular) addition with a signed integer. Computes
    /// `self + rhs`, wrapping around at the boundary of the type.
    #[inline(always)]
    pub const fn wrapping_add_signed(self, rhs: i256) -> Self {
        self.wrapping_add(Self::from_i256(rhs))
    }

    /// Wrapping (modular) subtraction. Computes `self - rhs`,
    /// wrapping around at the boundary of the type.
    #[inline(always)]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        sub(self, rhs)
    }

    /// Wrapping (modular) multiplication. Computes `self *
    /// rhs`, wrapping around at the boundary of the type.
    #[inline(always)]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        mul(self, rhs)
    }

    /// Wrapping (modular) division. Computes `self / rhs`.
    ///
    /// Wrapped division on unsigned types is just normal division. There's
    /// no way wrapping could ever happen. This function exists so that all
    /// operations are accounted for in the wrapping operations.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        div(self, rhs)
    }

    /// Wrapping Euclidean division. Computes `self.div_euclid(rhs)`.
    ///
    /// Wrapped division on unsigned types is just normal division. There's
    /// no way wrapping could ever happen. This function exists so that all
    /// operations are accounted for in the wrapping operations. Since, for
    /// the positive integers, all common definitions of division are equal,
    /// this is exactly equal to `self.wrapping_div(rhs)`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }

    /// Wrapping (modular) remainder. Computes `self % rhs`.
    ///
    /// Wrapped remainder calculation on unsigned types is just the regular
    /// remainder calculation. There's no way wrapping could ever happen.
    /// This function exists so that all operations are accounted for in the
    /// wrapping operations.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn wrapping_rem(self, rhs: Self) -> Self {
        rem(self, rhs)
    }

    /// Wrapping Euclidean modulo. Computes `self.rem_euclid(rhs)`.
    ///
    /// Wrapped modulo calculation on unsigned types is just the regular
    /// remainder calculation. There's no way wrapping could ever happen.
    /// This function exists so that all operations are accounted for in the
    /// wrapping operations. Since, for the positive integers, all common
    /// definitions of division are equal, this is exactly equal to
    /// `self.wrapping_rem(rhs)`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.wrapping_rem(rhs)
    }

    /// Wrapping (modular) negation. Computes `-self`,
    /// wrapping around at the boundary of the type.
    ///
    /// Since unsigned types do not have negative equivalents
    /// all applications of this function will wrap (except for `-0`).
    /// For values smaller than the corresponding signed type's maximum
    /// the result is the same as casting the corresponding signed value.
    /// Any larger values are equivalent to `MAX + 1 - (val - MAX - 1)` where
    /// `MAX` is the corresponding signed type's maximum.
    #[inline(always)]
    pub const fn wrapping_neg(self) -> Self {
        Self::MIN.wrapping_sub(self)
    }

    /// Panic-free bitwise shift-left; yields `self << mask(rhs)`,
    /// where `mask` removes any high-order bits of `rhs` that
    /// would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is *not* the same as a rotate-left; the
    /// RHS of a wrapping shift-left is restricted to the range
    /// of the type, rather than the bits shifted out of the LHS
    /// being returned to the other end. The primitive integer
    /// types all implement a [`rotate_left`](Self::rotate_left) function,
    /// which may be what you want instead.
    #[inline(always)]
    pub const fn wrapping_shl(self, rhs: u32) -> Self {
        let (lo, hi) = math::shl_u128(self.lo, self.hi, rhs % 256);
        Self { hi, lo }
    }

    /// Panic-free bitwise shift-right; yields `self >> mask(rhs)`,
    /// where `mask` removes any high-order bits of `rhs` that
    /// would cause the shift to exceed the bitwidth of the type.
    ///
    /// Note that this is *not* the same as a rotate-right; the
    /// RHS of a wrapping shift-right is restricted to the range
    /// of the type, rather than the bits shifted out of the LHS
    /// being returned to the other end. The primitive integer
    /// types all implement a [`rotate_right`](Self::rotate_right) function,
    /// which may be what you want instead.
    #[inline(always)]
    pub const fn wrapping_shr(self, rhs: u32) -> Self {
        let (lo, hi) = math::shr_u128(self.lo, self.hi, rhs % 256);
        Self { hi, lo }
    }

    /// Wrapping (modular) exponentiation. Computes `self.pow(exp)`,
    /// wrapping around at the boundary of the type.
    #[inline]
    pub const fn wrapping_pow(self, mut exp: u32) -> Self {
        todo!();
    }

    /// Calculates `self` + `rhs`.
    ///
    /// Returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    #[inline(always)]
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (lo, hi, overflowed) = math::add_u128(self.lo, self.hi, rhs.lo, rhs.hi);
        (Self { lo, hi }, overflowed)
    }

    /// Calculates `self` + `rhs` with a signed `rhs`.
    ///
    /// Returns a tuple of the addition along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    #[inline(always)]
    pub const fn overflowing_add_signed(self, rhs: i256) -> (Self, bool) {
        let is_negative = rhs.hi < 0;
        let (r, overflowed) = self.overflowing_add(Self::from_i256(rhs));
        (r, overflowed ^ is_negative)
    }

    /// Calculates `self` - `rhs`.
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating
    /// whether an arithmetic overflow would occur. If an overflow would
    /// have occurred then the wrapped value is returned.
    #[inline(always)]
    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (lo, hi, overflowed) = math::sub_u128(self.lo, self.hi, rhs.lo, rhs.hi);
        (Self { lo, hi }, overflowed)
    }

    /// Computes the absolute difference between `self` and `other`.
    #[inline(always)]
    pub const fn abs_diff(self, other: Self) -> Self {
        if lt(self, other) {
            sub(other, self)
        } else {
            sub(self, other)
        }
    }

    /// Calculates the multiplication of `self` and `rhs`.
    ///
    /// Returns a tuple of the multiplication along with a boolean
    /// indicating whether an arithmetic overflow would occur. If an
    /// overflow would have occurred then the wrapped value is returned.
    #[inline(always)]
    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (lo, hi, overflowed) = math::mul_u128(self.lo, self.hi, rhs.lo, rhs.hi);
        (Self { lo, hi }, overflowed)
    }

    /// Calculates the divisor when `self` is divided by `rhs`.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating
    /// whether an arithmetic overflow would occur. Note that for unsigned
    /// integers overflow never occurs, so the second value is always
    /// `false`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        let (lo, hi) = math::div_u128(self.lo, self.hi, rhs.lo, rhs.hi);
        (Self { lo, hi }, false)
    }

    /// Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating
    /// whether an arithmetic overflow would occur. Note that for unsigned
    /// integers overflow never occurs, so the second value is always
    /// `false`.
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this
    /// is exactly equal to `self.overflowing_div(rhs)`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_div(rhs)
    }

    /// Calculates the remainder when `self` is divided by `rhs`.
    ///
    /// Returns a tuple of the remainder after dividing along with a boolean
    /// indicating whether an arithmetic overflow would occur. Note that for
    /// unsigned integers overflow never occurs, so the second value is
    /// always `false`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        let (lo, hi) = math::rem_u128(self.lo, self.hi, rhs.lo, rhs.hi);
        (Self { lo, hi }, false)
    }

    /// Calculates the remainder `self.rem_euclid(rhs)` as if by Euclidean division.
    ///
    /// Returns a tuple of the modulo after dividing along with a boolean
    /// indicating whether an arithmetic overflow would occur. Note that for
    /// unsigned integers overflow never occurs, so the second value is
    /// always `false`.
    /// Since, for the positive integers, all common
    /// definitions of division are equal, this operation
    /// is exactly equal to `self.overflowing_rem(rhs)`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        self.overflowing_rem(rhs)
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// Returns a tuple of the exponentiation along with a bool indicating
    /// whether an overflow happened.
    #[inline]
    pub const fn overflowing_pow(self, mut exp: u32) -> (Self, bool) {
        todo!();
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    #[inline]
    pub const fn pow(self, mut exp: u32) -> Self {
        todo!();
    }

    /// Returns the square root of the number, rounded down.
    #[inline]
    pub const fn isqrt(self) -> Self {
        todo!();
    }

    /// Calculates the quotient of `self` and `rhs`, rounding the result towards negative infinity.
    ///
    /// This is the same as performing `self / rhs` for all unsigned integers.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn div_floor(self, rhs: Self) -> Self {
        div(self, rhs)
    }

    /// Calculates the quotient of `self` and `rhs`, rounding the result towards positive infinity.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline(always)]
    pub const fn div_ceil(self, rhs: Self) -> Self {
        let d = div(self, rhs);
        let r = rem(self, rhs);
        if r.lo > 0 || r.hi > 0 {
            let (lo, hi, _) = math::add_u128(d.lo, d.hi, 1, 0);
            u256 { lo, hi }
        } else {
            d
        }
    }

    /// Calculates the smallest value greater than or equal to `self` that
    /// is a multiple of `rhs`.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    ///
    /// ## Overflow behavior
    ///
    /// On overflow, this function will panic if overflow checks are enabled (default in debug
    /// mode) and wrap if overflow checks are disabled (default in release mode).
    #[inline]
    pub const fn next_multiple_of(self, rhs: Self) -> Self {
        match rem(self, rhs) {
            Self::MIN => self,
            r => add(self, sub(rhs, r)),
        }
    }

    /// Calculates the smallest value greater than or equal to `self` that
    /// is a multiple of `rhs`. Returns `None` if `rhs` is zero or the
    /// operation would result in overflow.
    #[inline]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
        match self.checked_rem(rhs) {
            None => None,
            Some(Self::MIN) => Some(self),
            // rhs - r cannot overflow because r is smaller than rhs
            Some(r) => self.checked_add(sub(rhs, r)),
        }
    }

    /// Returns `true` if `self` is an integer multiple of `rhs`, and false otherwise.
    ///
    /// This function is equivalent to `self % rhs == 0`, except that it will not panic
    /// for `rhs == 0`. Instead, `0.is_multiple_of(0) == true`, and for any non-zero `n`,
    /// `n.is_multiple_of(0) == false`.
    #[inline]
    pub const fn is_multiple_of(self, rhs: Self) -> bool {
        match rhs {
            Self::MIN => eq(self, Self::MIN),
            _ => eq(rem(self, rhs), Self::MIN),
        }
    }

    /// Returns `true` if and only if `self == 2^k` for some `k`.
    #[inline(always)]
    pub const fn is_power_of_two(self) -> bool {
        self.count_ones() == 1
    }

    #[inline]
    const fn one_less_than_next_power_of_two(self) -> Self {
        if eq(self, Self::MIN) {
            return Self::MIN;
        }
        let p = sub(self, Self::from_u8(1));
        let z = p.leading_zeros();
        Self::MAX.shr_u32(z)
    }

    /// Returns the smallest power of two greater than or equal to `self`.
    ///
    /// When return value overflows (i.e., `self > (1 << (N-1))` for type
    /// `uN`), it panics in debug mode and the return value is wrapped to 0 in
    /// release mode (the only situation in which this method can return 0).
    #[inline]
    pub const fn next_power_of_two(self) -> Self {
        add(self.one_less_than_next_power_of_two(), Self::from_u8(1))
    }

    /// Returns the smallest power of two greater than or equal to `self`. If
    /// the next power of two is greater than the type's maximum value,
    /// `None` is returned, otherwise the power of two is wrapped in `Some`.
    #[inline]
    pub const fn checked_next_power_of_two(self) -> Option<Self> {
        self.one_less_than_next_power_of_two().checked_add(Self::from_u8(1))
    }

    /// Returns the memory representation of this integer as a byte array in
    /// big-endian (network) byte order.
    #[inline(always)]
    pub const fn to_be_bytes(self) -> [u8; 32] {
        self.to_be().to_ne_bytes()
    }

    /// Returns the memory representation of this integer as a byte array in
    /// little-endian byte order.
    #[inline(always)]
    pub const fn to_le_bytes(self) -> [u8; 32] {
        self.to_le().to_ne_bytes()
    }

    /// Returns the memory representation of this integer as a byte array in
    /// native byte order.
    ///
    /// As the target platform's native endianness is used, portable code
    /// should use [`to_be_bytes`] or [`to_le_bytes`], as appropriate,
    /// instead.
    #[inline(always)]
    pub const fn to_ne_bytes(self) -> [u8; 32] {
        // SAFETY: integers are plain old datatypes so we can always transmute them to
        // arrays of bytes
        unsafe { mem::transmute(self) }
    }

    /// Creates a native endian integer value from its representation
    /// as a byte array in big endian.
    #[inline(always)]
    pub const fn from_be_bytes(bytes: [u8; 32]) -> Self {
        Self::from_be(Self::from_ne_bytes(bytes))
    }

    /// Creates a native endian integer value from its representation
    /// as a byte array in little endian.
    pub const fn from_le_bytes(bytes: [u8; 32]) -> Self {
        Self::from_le(Self::from_ne_bytes(bytes))
    }

    /// Creates a native endian integer value from its memory representation
    /// as a byte array in native endianness.
    ///
    /// As the target platform's native endianness is used, portable code
    /// likely wants to use [`from_be_bytes`] or [`from_le_bytes`], as
    /// appropriate instead.
    ///
    /// [`from_be_bytes`]: Self::from_be_bytes
    /// [`from_le_bytes`]: Self::from_le_bytes
    #[inline(always)]
    pub const fn from_ne_bytes(bytes: [u8; 32]) -> Self {
        // SAFETY: integers are plain old datatypes so we can always transmute to them
        unsafe { mem::transmute(bytes) }
    }
}

impl u256 {
    /// Get the high 128 bits of the signed integer.
    #[inline(always)]
    pub const fn get_high(self) -> u128 {
        self.hi
    }

    /// Get the low 128 bits of the signed integer.
    #[inline(always)]
    pub const fn get_low(self) -> u128 {
        self.lo
    }

    /// Create the 256-bit unsigned integer to a `u8`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Self {
        Self::from_u128(value as u128)
    }

    /// Create the 256-bit unsigned integer from a `u16`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u16(value: u16) -> Self {
        Self::from_u128(value as u128)
    }

    /// Create the 256-bit unsigned integer from a `u32`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self::from_u128(value as u128)
    }

    /// Create the 256-bit unsigned integer from a `u64`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u64(value: u64) -> Self {
        Self::from_u128(value as u128)
    }

    /// Create the 256-bit unsigned integer from a `u128`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u128(value: u128) -> Self {
        let (lo, hi) = math::as_uwide_u128(value);
        Self { lo, hi }
    }

    /// Create the 256-bit unsigned integer to an `i8`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i8(value: i8) -> Self {
        Self::from_i128(value as i128)
    }

    /// Create the 256-bit unsigned integer from an `i16`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i16(value: i16) -> Self {
        Self::from_i128(value as i128)
    }

    /// Create the 256-bit unsigned integer from an `i32`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i32(value: i32) -> Self {
        Self::from_i128(value as i128)
    }

    /// Create the 256-bit unsigned integer from an `i64`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i64(value: i64) -> Self {
        Self::from_i128(value as i128)
    }

    /// Create the 256-bit unsigned integer from an `i128`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i128(value: i128) -> Self {
        let (lo, hi) = math::as_iwide_u128(value);
        Self { lo, hi }
    }

    /// Create the 256-bit unsigned integer from an `i256`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i256(value: i256) -> Self {
        value.as_u256()
    }

    /// Convert the 256-bit unsigned integer to an `u8`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_u8(&self) -> u8 {
        self.lo as u8
    }

    /// Convert the 256-bit unsigned integer to an `u16`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_u16(&self) -> u16 {
        self.lo as u16
    }

    /// Convert the 256-bit unsigned integer to an `u32`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_u32(&self) -> u32 {
        self.lo as u32
    }

    /// Convert the 256-bit unsigned integer to an `u64`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_u64(&self) -> u64 {
        self.lo as u64
    }

    /// Convert the 256-bit unsigned integer to an `u128`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_u128(&self) -> u128 {
        math::as_unarrow_u128(self.lo, self.hi)
    }

    /// Convert the 256-bit unsigned integer to an `i8`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i8(&self) -> i8 {
        self.as_i128() as i8
    }

    /// Convert the 256-bit unsigned integer to an `i16`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i16(&self) -> i16 {
        self.as_i128() as i16
    }

    /// Convert the 256-bit unsigned integer to an `i32`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i32(&self) -> i32 {
        self.as_i128() as i32
    }

    /// Convert the 256-bit unsigned integer to an `i64`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i64(&self) -> i64 {
        self.as_i128() as i64
    }

    /// Convert the 256-bit unsigned integer to an `i128`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i128(&self) -> i128 {
        math::as_inarrow_u128(self.lo, self.hi)
    }

    /// Convert the 256-bit unsigned integer to an `i256`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i256(&self) -> i256 {
        // TODO: Validate this is true...
        // NOTE: This should be valid since we just want the same
        // bitwise representation as it.
        i256 { hi: self.hi as i128, lo: self.lo }
    }
}

impl Add for u256 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        if cfg!(not(debug_assertions)) {
            add(self, rhs)
        } else {
            self.checked_add(rhs).unwrap()
        }
    }
}

op_impl!(u256, Add, AddAssign, add, add_assign);

impl fmt::Binary for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!();
    }
}

impl BitAnd for u256 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        bitand(self, rhs)
    }
}

op_impl!(u256, BitAnd, BitAndAssign, bitand, bitand_assign);

impl BitOr for u256 {
    type Output = u256;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        bitor(self, rhs)
    }
}

op_impl!(u256, BitOr, BitOrAssign, bitor, bitor_assign);

impl BitXor for u256 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        bitxor(self, rhs)
    }
}

op_impl!(u256, BitXor, BitXorAssign, bitxor, bitxor_assign);

impl fmt::Debug for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.hi, f)?;
        fmt::Display::fmt(&self.lo, f)
    }
}

impl Div for u256 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        if cfg!(not(debug_assertions)) {
            div(self, rhs)
        } else {
            self.checked_div(rhs).unwrap()
        }
    }
}

op_impl!(u256, Div, DivAssign, div, div_assign);

impl From<bool> for u256 {
    #[inline(always)]
    fn from(small: bool) -> Self {
        Self { lo: small as u128, hi: 0 }
    }
}

impl From<char> for u256 {
    #[inline(always)]
    fn from(c: char) -> Self {
        Self { lo: c as u128, hi: 0 }
    }
}

from_impl!(u256, u8, from_u8);
from_impl!(u256, u16, from_u16);
from_impl!(u256, u32, from_u32);
from_impl!(u256, u64, from_u64);
from_impl!(u256, u128, from_u128);

impl FromStr for u256 {
    type Err = ParseIntError;

    /// Parses a string s to return a value of this type.
    ///
    /// This is not optimized, since all optimization is done in
    /// the lexical implementation.
    #[inline(always)]
    fn from_str(src: &str) -> Result<u256, ParseIntError> {
        todo!();
    }
}

impl fmt::LowerExp for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!();
    }
}

impl fmt::LowerHex for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!();
    }
}

impl Mul for u256 {
    type Output = u256;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        if cfg!(not(debug_assertions)) {
            mul(self, rhs)
        } else {
            self.checked_mul(rhs).unwrap()
        }
    }
}

op_impl!(u256, Mul, MulAssign, mul, mul_assign);

impl Not for u256 {
    type Output = u256;

    #[inline(always)]
    fn not(self) -> Self::Output {
        not(self)
    }
}

ref_impl!(u256, Not, not);

impl fmt::Octal for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!();
    }
}

impl Ord for u256 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        cmp(*self, *other)
    }
}

impl PartialOrd for u256 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        lt(*self, *other)
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        le(*self, *other)
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        gt(*self, *other)
    }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        ge(*self, *other)
    }
}

impl Product for u256 {
    #[inline(always)]
    fn product<I: Iterator<Item = u256>>(iter: I) -> Self {
        todo!();
    }
}

impl Rem for u256 {
    type Output = u256;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        if cfg!(not(debug_assertions)) {
            rem(self, rhs)
        } else {
            self.checked_rem(rhs).unwrap()
        }
    }
}

op_impl!(u256, Rem, RemAssign, rem, rem_assign);

macro_rules! shift_const_impl {
    (@shl $value:ident, $shift:ident) => {{
        let (lo, hi) = math::shl_u128($value.lo, $value.hi, $shift as u32);
        Self { hi, lo }
    }};

    (@shr $value:ident, $shift:ident) => {{
        let (lo, hi) = math::shr_u128($value.lo, $value.hi, $shift as u32);
        Self { hi, lo }
    }};

    (@nomod $t:ty, $shl:ident, $shr:ident) => (
        /// Const evaluation of shl.
        #[inline(always)]
        pub const fn $shl(self, other: $t) -> Self {
            let other = other as u32;
            shift_const_impl!(@shl self, other)
        }

        /// Const evaluation of shr.
        pub const fn $shr(self, other: $t) -> Self {
            let other = other as u32;
            shift_const_impl!(@shr self, other)
        }
    );

    ($t:ty, $shl:ident, $shr:ident) => (
        /// Const evaluation of shl.
        ///
        /// This behavior is wrapping: if `other >= 256`, this wraps.
        #[inline(always)]
        pub const fn $shl(self, other: $t) -> Self {
            debug_assert!(other < 256, "attempt to shift left with overflow");
            let other = other as u32;
            shift_const_impl!(@shl self, other)
        }

        /// Const evaluation of shr.
        ///
        /// This behavior is wrapping: if `other >= 256`, this wraps.
        pub const fn $shr(self, other: $t) -> Self {
            debug_assert!(other < 256, "attempt to shift right with overflow");
            let other = other as u32;
            shift_const_impl!(@shr self, other)
        }
    );

    (@256 $t:ty, $shl:ident, $shr:ident) => (
        /// Const evaluation of shl.
        ///
        /// This behavior is wrapping: if `other >= 256`, this wraps.
        #[inline(always)]
        pub const fn $shl(self, other: $t) -> Self {
            let max = Self::from_u16(256);
            debug_assert!(lt(other, max), "attempt to shift left with overflow");
            let shift = (self.lo & (u32::MAX as u128)) as u32;
            shift_const_impl!(@shl self, shift)
        }

        /// Const evaluation of shr.
        ///
        /// This behavior is wrapping: if `other >= 256`, this wraps.
        pub const fn $shr(self, other: $t) -> Self {
            let max = Self::from_u16(256);
            debug_assert!(lt(other, max), "attempt to shift right with overflow");
            let shift = self.lo & (u32::MAX as u128);
            shift_const_impl!(@shr self, shift)
        }
    );
}

// Const implementations for Shl
impl u256 {
    shift_const_impl!(@nomod i8, shl_i8, shr_i8);
    shift_const_impl!(i16, shl_i16, shr_i16);
    shift_const_impl!(i32, shl_i32, shr_i32);
    shift_const_impl!(i64, shl_i64, shr_i64);
    shift_const_impl!(i128, shl_i128, shr_i128);
    // TODO: Restore
    //shift_const_impl!(@256 i256, shl_i256, shr_i256);
    shift_const_impl!(isize, shl_isize, shr_isize);
    shift_const_impl!(@nomod u8, shl_u8, shr_u8);
    shift_const_impl!(u16, shl_u16, shr_u16);
    shift_const_impl!(u32, shl_u32, shr_u32);
    shift_const_impl!(u64, shl_u64, shr_u64);
    shift_const_impl!(u128, shl_u128, shr_u128);
    shift_const_impl!(@256 u256, shl_u256, shr_u256);
    shift_const_impl!(usize, shl_usize, shr_usize);
}

impl Shl for u256 {
    type Output = Self;

    #[inline(always)]
    fn shl(self, other: Self) -> Self::Output {
        let shift = other.lo & (u32::MAX as u128);
        shift_const_impl!(@shr self, shift)
    }
}

impl Shl<&u256> for u256 {
    type Output = <Self as Shl>::Output;

    #[inline(always)]
    fn shl(self, other: &u256) -> Self::Output {
        self.shl(*other)
    }
}

impl Shr for u256 {
    type Output = Self;

    #[inline(always)]
    fn shr(self, other: Self) -> Self::Output {
        let shift = other.lo & (u32::MAX as u128);
        shift_const_impl!(@shr self, shift)
    }
}

impl Shr<&u256> for u256 {
    type Output = <Self as Shr>::Output;

    #[inline(always)]
    fn shr(self, other: &u256) -> Self::Output {
        self.shr(*other)
    }
}

macro_rules! shift_impl {
    (@mod $($t:ty)*) => ($(
        impl Shl<$t> for u256 {
            type Output = Self;

            #[inline(always)]
            fn shl(self, other: $t) -> Self::Output {
                let shift = other % 256;
                shift_const_impl!(@shl self, shift)
            }
        }

        impl Shr<$t> for u256 {
            type Output = Self;

            #[inline(always)]
            fn shr(self, other: $t) -> Self::Output {
                let shift = other % 256;
                shift_const_impl!(@shr self, shift)
            }
        }
    )*);

    (@nomod $($t:ty)*) => ($(
        impl Shl<$t> for u256 {
            type Output = Self;

            #[inline(always)]
            fn shl(self, other: $t) -> Self::Output {
                shift_const_impl!(@shl self, other)
            }
        }

        impl Shr<$t> for u256 {
            type Output = Self;

            #[inline(always)]
            fn shr(self, other: $t) -> Self::Output {
                shift_const_impl!(@shr self, other)
            }
        }
    )*);

    (@256 $($t:ty)*) => ($(
        impl Shl<$t> for u256 {
            type Output = Self;

            #[inline(always)]
            fn shl(self, other: $t) -> Self::Output {
                let shift = rem(other, Self::from_u16(256));
                shift_const_impl!(@shl self, shift)
            }
        }

        impl Shr<$t> for u256 {
            type Output = Self;

            #[inline(always)]
            fn shr(self, other: $t) -> Self::Output {
                let shift = rem(other, Self::from_u16(256));
                shift_const_impl!(@shr self, shift)
            }
        }
    )*);

    ($($t:ty)*) => ($(
        impl Shl<&$t> for u256 {
            type Output = <Self as Shl>::Output;

            #[inline(always)]
            fn shl(self, other: &$t) -> Self::Output {
                self.shl(*other)
            }
        }

        impl ShlAssign<$t> for u256 {
            #[inline(always)]
            fn shl_assign(&mut self, other: $t) {
                *self = self.shl(other);
            }
        }

        impl ShlAssign<&$t> for u256 {
            #[inline(always)]
            fn shl_assign(&mut self, other: &$t) {
                *self = self.shl(other);
            }
        }

        impl Shr<&$t> for u256 {
            type Output = <Self as Shr>::Output;

            #[inline(always)]
            fn shr(self, other: &$t) -> Self::Output {
                self.shr(*other)
            }
        }

        impl ShrAssign<$t> for u256 {
            #[inline(always)]
            fn shr_assign(&mut self, other: $t) {
                *self = self.shr(other);
            }
        }

        impl ShrAssign<&$t> for u256 {
            #[inline(always)]
            fn shr_assign(&mut self, other: &$t) {
                *self = self.shr(other);
            }
        }
    )*);
}

shift_impl! { @nomod i8 u8 }
shift_impl! { @mod i16 i32 i64 i128 isize u16 u32 u64 u128 usize }
shift_impl! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize } //  TODO: restore i256

impl Sub for u256 {
    type Output = u256;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        if cfg!(not(debug_assertions)) {
            sub(self, rhs)
        } else {
            self.checked_sub(rhs).unwrap()
        }
    }
}

op_impl!(u256, Sub, SubAssign, sub, sub_assign);

impl Sum for u256 {
    #[inline(always)]
    fn sum<I: Iterator<Item = u256>>(iter: I) -> Self {
        todo!();
    }
}

macro_rules! try_from_impl {
    ($($t:ty)*) => ($(
        impl TryFrom<$t> for u256 {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(u: $t) -> Result<Self, TryFromIntError> {
                if u >= 0 {
                    Ok(Self::from_u128(u as u128))
                } else {
                    Err(TryFromIntError {})
                }
            }
        }
    )*);
}

try_from_impl! { i8 i16 i32 i64 i128 isize }

impl TryFrom<i256> for u256 {
    type Error = TryFromIntError;

    #[inline(always)]
    fn try_from(u: i256) -> Result<Self, TryFromIntError> {
        if u.hi >= 0 {
            Ok(u.as_u256())
        } else {
            Err(TryFromIntError {})
        }
    }
}

impl fmt::UpperExp for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!();
    }
}

impl fmt::UpperHex for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!();
    }
}

/// Const implementation of `Add` for internal algorithm use.
const fn add(lhs: u256, rhs: u256) -> u256 {
    let (lo, hi, _) = math::add_u128(lhs.lo, lhs.hi, rhs.lo, rhs.hi);
    u256 { lo, hi }
}

/// Const implementation of `Div` for internal algorithm use.
const fn div(lhs: u256, rhs: u256) -> u256 {
    let (lo, hi) = math::div_u128(lhs.lo, lhs.hi, rhs.lo, rhs.hi);
    u256 { lo, hi }
}

/// Const implementation of `Mul` for internal algorithm use.
const fn mul(lhs: u256, rhs: u256) -> u256 {
    let (lo, hi, _) = math::mul_u128(lhs.lo, lhs.hi, rhs.lo, rhs.hi);
    u256 { lo, hi }
}

/// Const implementation of `Rem` for internal algorithm use.
const fn rem(lhs: u256, rhs: u256) -> u256 {
    let (lo, hi) = math::rem_u128(lhs.lo, lhs.hi, rhs.lo, rhs.hi);
    u256 { lo, hi }
}

/// Const implementation of `Sub` for internal algorithm use.
const fn sub(lhs: u256, rhs: u256) -> u256 {
    let (lo, hi, _) = math::sub_u128(lhs.lo, lhs.hi, rhs.lo, rhs.hi);
    u256 { lo, hi }
}

/// Const implementation of `BitAnd` for internal algorithm use.
const fn bitand(lhs: u256, rhs: u256) -> u256 {
    u256 { hi: lhs.hi & rhs.hi, lo: lhs.lo & rhs.lo }
}

/// Const implementation of `BitOr` for internal algorithm use.
const fn bitor(lhs: u256, rhs: u256) -> u256 {
    u256 { hi: lhs.hi | rhs.hi, lo: lhs.lo | rhs.lo }
}

/// Const implementation of `BitXor` for internal algorithm use.
const fn bitxor(lhs: u256, rhs: u256) -> u256 {
    u256 { hi: lhs.hi ^ rhs.hi, lo: lhs.lo ^ rhs.lo }
}

/// Const implementation of `Not` for internal algorithm use.
const fn not(n: u256) -> u256 {
    u256 { lo: !n.lo, hi: !n.hi }
}

/// Const implementation of `Eq` for internal algorithm use.
const fn eq(lhs: u256, rhs: u256) -> bool {
    lhs.lo == rhs.lo && lhs.hi == rhs.hi
}

/// Const implementation of `PartialOrd::lt` for internal algorithm use.
const fn lt(lhs: u256, rhs: u256) -> bool {
    lhs.hi < rhs.hi || (lhs.hi == rhs.hi && lhs.lo < rhs.lo)
}

/// Const implementation of `PartialOrd::le` for internal algorithm use.
const fn le(lhs: u256, rhs: u256) -> bool {
    lhs.hi < rhs.hi || (lhs.hi == rhs.hi && lhs.lo <= rhs.lo)
}

/// Const implementation of `PartialOrd::gt` for internal algorithm use.
const fn gt(lhs: u256, rhs: u256) -> bool {
    lhs.hi > rhs.hi || (lhs.hi == rhs.hi && lhs.lo > rhs.lo)
}

/// Const implementation of `PartialOrd::ge` for internal algorithm use.
const fn ge(lhs: u256, rhs: u256) -> bool {
    lhs.hi > rhs.hi || (lhs.hi == rhs.hi && lhs.lo >= rhs.lo)
}

/// Const implementation of `PartialOrd::cmp` for internal algorithm use.
const fn cmp(lhs: u256, rhs: u256) -> Ordering {
    if lt(lhs, rhs) {
        Ordering::Less
    } else if gt(lhs, rhs) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(u256::from_u8(1), u256::from_u8(1)), u256::from_u8(2));
        assert_eq!(add(u256::MAX, u256::MAX), u256 { hi: u128::MAX, lo: u128::MAX - 1 });
        // TODO: Add more here
    }
}
