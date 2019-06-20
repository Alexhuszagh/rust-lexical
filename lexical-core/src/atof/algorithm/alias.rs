//! Aliases and traits to simplify float-parsing.

use lib::{iter, slice};
use float::*;
use util::*;
use super::bigcomp::ToBigInt;
use super::correct::FloatErrors;

pub type SliceIter<'a, T> = slice::Iter<'a, T>;
pub type ChainedSliceIter<'a, T> = iter::Chain<SliceIter<'a, T>, SliceIter<'a, T>>;

// TRAITS

macro_rules! def_float_type {
    ($($t:ty)*) => (
        /// Trait to simplify type signatures for atof.
        pub(super) trait FloatType:
            $(FloatRounding<$t> +)*
            StablePower
        {
            type Mantissa: Mantissa;
            type ExtendedFloat: ExtendedFloatType<Self>;
        }
    );
}

#[cfg(has_i128)]
def_float_type!(u64 u128);

#[cfg(not(has_i128))]
def_float_type!(u64);

impl FloatType for f32 {
    type Mantissa = Self::Unsigned;
    type ExtendedFloat = ExtendedFloat<Self::Mantissa>;
}

impl FloatType for f64 {
    type Mantissa = Self::Unsigned;
    type ExtendedFloat = ExtendedFloat<Self::Mantissa>;
}

/// Trait for a useable mantissa.
pub(super) trait MantissaType:
    Mantissa +
    FloatErrors
{}

impl MantissaType for u64 {
}

#[cfg(has_i128)]
impl MantissaType for u128 {
}

/// Trait for extended-float types.
pub(super) trait ExtendedFloatType<F: FloatType>:
    ToBigInt<F::Mantissa> +
    From<F>
{
    // I really wish I had any other choice **other** than getters and setters,
    // but since we can't specify fields in traits, and we can't use properties...
    // C'est la vie.
    fn mant(&self) -> F::Mantissa;
    fn exp(&self) -> i32;
    fn set_mant(&mut self, F::Mantissa);
    fn set_exp(&mut self, i32);
}

impl ExtendedFloatType<f32> for ExtendedFloat<u32> {
    #[inline]
    fn mant(&self) -> u32 {
        self.mant
    }

    #[inline]
    fn exp(&self) -> i32 {
        self.exp
    }

    #[inline]
    fn set_mant(&mut self, mant: u32) {
        self.mant = mant;
    }

    #[inline]
    fn set_exp(&mut self, exp: i32) {
        self.exp = exp;
    }
}

impl ExtendedFloatType<f64> for ExtendedFloat<u64> {
    #[inline]
    fn mant(&self) -> u64 {
        self.mant
    }

    #[inline]
    fn exp(&self) -> i32 {
        self.exp
    }

    #[inline]
    fn set_mant(&mut self, mant: u64) {
        self.mant = mant;
    }

    #[inline]
    fn set_exp(&mut self, exp: i32) {
        self.exp = exp;
    }
}
