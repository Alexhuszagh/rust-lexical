//! An unsigned 256-bit integer type.
//!
//! This aims to have feature parity with Rust's unsigned
//! integer types, such as [u32][core::u32]. The documentation
//! is based off of [u32][core::u32] for each method/member.

use core::cmp::Ordering;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::*;
use core::num::{ParseIntError, TryFromIntError};
use core::str::FromStr;

use crate::i256::i256;
use crate::numtypes::*;
// TODO: Document
// TODO: Feature gate this...

// FIXME: Add support for [Saturating][core::num::Saturating] and
// [Wrapping][core::num::Wrapping] when we drop support for <1.74.0.

/// The 256-bit unsigned integer type.
///
/// This has the same binary representation as Apache Arrow's types,
/// and therefore can safely be transmuted from one to the other.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct u256 {
    lo: u128,
    hi: u128,
}

impl u256 {
    /// The smallest value that can be represented by this integer type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(u256::MIN, 0);
    /// ```
    pub const MIN: u256 = u256 { lo: 0, hi: 0 };

    /// The largest value that can be represented by this integer type
    /// (2<sup>256</sup> - 1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(u256::MAX, 0);  // TODO, need to negate...
    /// ```
    pub const MAX: u256 = not(Self::MIN);

    /// The size of this integer type in bits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(u256::BITS, 256);
    /// ```
    pub const BITS: u32 = 256;

    /// Returns the number of ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(u256::BITS, 256);  // TODO: Fix...
    /// ```
    #[inline(always)]
    pub const fn count_ones(self) -> u32 {
        // TODO: `ctpop`
        todo!();
    }

    /// Returns the number of zeros in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(u256::BITS, 256);  // TODO: Fix...
    /// ```
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
    /// let n = u256::MAX >> 2;
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// let n = u256::MAX >> 2;  // TODO: This is wrong
    /// assert_eq!(n.trailing_zeros(), 2);  // TODO: This is wrong
    ///
    /// let zero = u256::MIN;
    /// assert_eq!(zero.trailing_zeros(), 256);
    ///
    /// let max = u256::MAX;
    /// assert_eq!(max.trailing_zeros(), 0);
    /// ```
    #[inline(always)]
    pub const fn trailing_zeros(self) -> u32 {
        let mut trailing = self.hi.trailing_zeros();
        if trailing == u128::BITS {
            trailing += self.lo.trailing_zeros()
        }
        trailing
    }

    /// Returns the number of leading ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn leading_ones(self) -> u32 {
       not(self).leading_zeros()
    }

    /// Returns the number of trailing ones in the binary representation of `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn trailing_ones(self) -> u32 {
       not(self).trailing_zeros()
    }

    /// Shifts the bits to the left by a specified amount, `n`,
    /// wrapping the truncated bits to the end of the resulting integer.
    ///
    /// Please note this isn't the same operation as the `<<` shifting operator!
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn rotate_left(self, n: u32) -> Self {
        // TODO: should just be able to rotate the bits and overflow...
       todo!();
    }

    /// Shifts the bits to the right by a specified amount, `n`,
    /// wrapping the truncated bits to the beginning of the resulting
    /// integer.
    ///
    /// Please note this isn't the same operation as the `>>` shifting operator!
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn rotate_right(self, n: u32) -> Self {
       todo!();
    }


    /// Reverses the byte order of the integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn swap_bytes(self) -> Self {
        todo!();
    }

    /// Reverses the order of bits in the integer. The least significant
    /// bit becomes the most significant bit, second least-significant bit
    /// becomes second most-significant bit, etc.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn reverse_bits(self) -> Self {
        todo!();
    }

    /// Converts an integer from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are
    /// swapped.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        todo!();
    }

    /// Checked addition with a signed integer. Computes `self + rhs`,
    /// returning `None` if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_add_signed(self, rhs: i256) -> Option<Self> {
        todo!();
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None`
    /// if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        todo!();
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning `None`
    /// if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        todo!();
    }

    /// Checked integer division. Computes `self / rhs`, returning `None`
    /// if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn div_euclid(self, rhs: Self) -> Self {
        div(self, rhs)
    }

    /// Checked Euclidean division. Computes `self.div_euclid(rhs)`,
    /// returning `None` if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn rem_euclid(self, rhs: Self) -> Self {
        rem(self, rhs)
    }

    /// Checked Euclidean modulo. Computes `self.rem_euclid(rhs)`,
    /// returning `None` if `rhs == 0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn ilog(self, base: Self) -> u32 {
        if let Some(log) = self.checked_ilog(base) {
            log
        } else {
            todo!();
        }
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn ilog2(self) -> u32 {
        if let Some(log) = self.checked_ilog2() {
            log
        } else {
            todo!();
        }
    }

    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// # Panics
    ///
    /// This function will panic if `self` is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn ilog10(self) -> u32 {
        if let Some(log) = self.checked_ilog10() {
            log
        } else {
            todo!();
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
        todo!();
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    ///
    /// Returns `None` if the number is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_ilog2(self) -> Option<u32> {
        todo!();
    }

    /// Returns the base 10 logarithm of the number, rounded down.
    ///
    /// Returns `None` if the number is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_ilog10(self) -> Option<u32> {
        todo!();
    }

    /// Checked negation. Computes `-self`, returning `None` unless `self ==
    /// 0`.
    ///
    /// Note that negating any positive integer will overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
        todo!();
    }

    /// Checked shift right. Computes `self >> rhs`, returning `None`
    /// if `rhs` is larger than or equal to the number of bits in `self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
        todo!();
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None`
    /// if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// TODO
    /// ```
    #[inline(always)]
    pub const fn checked_pow(self, base: u32) -> Option<Self> {
        todo!();
    }

    // TODO: More here...
}

impl u256 {
    /// Create the 256-bit unsigned integer to a `u8`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Self {
        Self { lo: value as u128, hi: 0 }
    }

    /// Create the 256-bit unsigned integer from a `u16`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u16(value: u16) -> Self {
        Self { lo: value as u128, hi: 0 }
    }

    /// Create the 256-bit unsigned integer from a `u32`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self { lo: value as u128, hi: 0 }
    }

    /// Create the 256-bit unsigned integer from a `u64`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u64(value: u64) -> Self {
        Self { lo: value as u128, hi: 0 }
    }

    /// Create the 256-bit unsigned integer from a `u128`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_u128(value: u128) -> Self {
        Self { lo: value as u128, hi: 0 }
    }

    /// Create the 256-bit unsigned integer to an `i8`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i8(value: i8) -> Self {
        todo!();
    }

    /// Create the 256-bit unsigned integer from an `i16`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i16(value: i16) -> Self {
        todo!();
    }

    /// Create the 256-bit unsigned integer from an `i32`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i32(value: i32) -> Self {
        todo!();
    }

    /// Create the 256-bit unsigned integer from an `i64`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i64(value: i64) -> Self {
        todo!();
    }

    /// Create the 256-bit unsigned integer from an `i128`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i128(value: i128) -> Self {
        todo!();
    }

    /// Create the 256-bit unsigned integer from an `i256`, as if by an `as` cast.
    #[inline(always)]
    pub const fn from_i256(value: i256) -> Self {
        todo!();
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
        self.lo as u128
    }

    /// Convert the 256-bit unsigned integer to an `i8`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i8(&self) -> i8 {
        self.lo as i8
    }

    /// Convert the 256-bit unsigned integer to an `i16`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i16(&self) -> i16 {
        self.lo as i16
    }

    /// Convert the 256-bit unsigned integer to an `i32`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i32(&self) -> i32 {
        self.lo as i32
    }

    /// Convert the 256-bit unsigned integer to an `i64`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i64(&self) -> i64 {
        self.lo as i64
    }

    /// Convert the 256-bit unsigned integer to an `i128`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i128(&self) -> i128 {
        self.lo as i128
    }

    /// Convert the 256-bit unsigned integer to an `i256`, as if by an `as` cast.
    #[inline(always)]
    pub const fn as_i256(&self) -> i256 {
        todo!();
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
        todo!();
    }
}

impl fmt::Display for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!();
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

impl Not for &u256 {
    type Output = <u256 as Not>::Output;

    #[inline(always)]
    fn not(self) -> Self::Output {
        not(*self)
    }
}

impl fmt::Octal for u256 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!();
    }
}

impl Ord for u256 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        todo!();
    }
}

impl PartialOrd for u256 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!();
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
        todo!();
    }};

    (@shr $value:ident, $shift:ident) => {{
        todo!();
    }};

    (@nomod $t:ty, $shl:ident, $shr:ident) => (
        /// Const evaluation of shl.
        #[inline(always)]
        pub const fn $shl(self, other: $t) -> Self {
            shift_const_impl!(@shl self, other)
        }

        /// Const evaluation of shr.
        pub const fn $shr(self, other: $t) -> Self {
            shift_const_impl!(@shr self, other)
        }
    );

    ($t:ty, $shl:ident, $shr:ident) => (
        /// Const evaluation of shl.
        ///
        /// This behavior is wrapping: if `other >= 256`, this wraps.
        #[inline(always)]
        pub const fn $shl(self, other: $t) -> Self {
            let shift = other % 256;
            shift_const_impl!(@shl self, shift)
        }

        /// Const evaluation of shr.
        ///
        /// This behavior is wrapping: if `other >= 256`, this wraps.
        pub const fn $shr(self, other: $t) -> Self {
            let shift = other % 256;
            shift_const_impl!(@shr self, shift)
        }
    );

    (@256 $t:ty, $shl:ident, $shr:ident) => (
        /// Const evaluation of shl.
        ///
        /// This behavior is wrapping: if `other >= 256`, this wraps.
        #[inline(always)]
        pub const fn $shl(self, other: $t) -> Self {
            let shift = rem(other, Self::from_u16(256));
            shift_const_impl!(@shl self, shift)
        }

        /// Const evaluation of shr.
        ///
        /// This behavior is wrapping: if `other >= 256`, this wraps.
        pub const fn $shr(self, other: $t) -> Self {
            let shift = rem(other, Self::from_u16(256));
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
        let shift = rem(other, Self::from_u16(256));
        todo!();
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
        let shift = rem(other, Self::from_u16(256));
        todo!();
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
            fn try_from(u: $t) -> Result<Self, <Self as TryFrom<$t>>::Error> {
                todo!();
            }
        }
    )*);
}

try_from_impl! { i8 i16 i32 i64 i128 i256 isize }

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
    todo!();
}

/// Const implementation of `Div` for internal algorithm use.
const fn div(lhs: u256, rhs: u256) -> u256 {
    todo!();
}

/// Const implementation of `Mul` for internal algorithm use.
const fn mul(lhs: u256, rhs: u256) -> u256 {
    todo!();
}

/// Const implementation of `Rem` for internal algorithm use.
const fn rem(lhs: u256, rhs: u256) -> u256 {
    todo!();
}

/// Const implementation of `Sub` for internal algorithm use.
const fn sub(lhs: u256, rhs: u256) -> u256 {
    todo!();
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
