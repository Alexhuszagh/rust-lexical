//! Convert between extended-precision and native floats/integers.

use sealed::ops;
use util::*;
use super::float_type::FloatType;

// FROM INT

/// Import FloatType from integer.
///
/// This works because we call normalize before any operation, which
/// allows us to convert the integer representation to the float one.
#[inline(always)]
pub(super) fn from_int<T: Integer>(t: T) -> FloatType {
    FloatType {
        frac: t.cast(),
        exp: 0,
    }
}

// FROM FLOAT

/// Import FloatType from native float.
///
/// Generate fraction from mantissa and read exponent as signed magnitude value.
#[inline(always)]
pub(super) fn from_float<T: Float>(t: T)
    -> FloatType
    where T::Unsigned: PrimitiveCast<u64>
{
    FloatType {
        frac: t.significand().cast(),
        exp: t.exponent(),
    }
}

// AS FLOAT

/// Export extended-precision float to native float.
///
/// The extended-precision float must be in native float representation,
/// with overflow/underflow appropriately handled.
#[inline]
pub(super) fn as_float<T: Float>(fp: FloatType)
    -> T
    where T::Unsigned: PrimitiveCast<u64>,
          T::Unsigned: ops::Shl<i32>,
          u32: PrimitiveCast<T::Unsigned>,
          u64: PrimitiveCast<T::Unsigned>
{
    // Export floating-point number.
    if fp.frac == 0 || fp.exp < T::DENORMAL_EXPONENT {
        // sub-denormal, underflow
        T::ZERO
    } else if fp.exp >= T::MAX_EXPONENT {
        // overflow
        T::from_bits(T::INFINITY_BITS)
    } else {
        // calculate the exp and fraction bits, and return a float from bits.
        let exp: u64;
        if (fp.exp == T::DENORMAL_EXPONENT) && (fp.frac & T::HIDDEN_BIT_MASK.cast()) == 0 {
            exp = 0;
        } else {
            exp = (fp.exp + T::EXPONENT_BIAS) as u64;
        }
        let exp = exp << T::SIGNIFICAND_SIZE;
        let frac = fp.frac & T::FRACTION_MASK.cast();
        T::from_bits((frac | exp).cast())
    }
}
