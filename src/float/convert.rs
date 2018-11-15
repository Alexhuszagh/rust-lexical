//! Convert between extended-precision and native floats/integers.

use util::*;
use super::float::ExtendedFloat;
use super::mantissa::Mantissa;

// FROM INT

/// Import ExtendedFloat from integer.
///
/// This works because we call normalize before any operation, which
/// allows us to convert the integer representation to the float one.
#[inline(always)]
pub(super) fn from_int<M: Mantissa, T: Integer>(t: T) -> ExtendedFloat<M> {
    ExtendedFloat {
        frac: as_(t),
        exp: 0,
    }
}

// FROM FLOAT

/// Import ExtendedFloat from native float.
///
/// Generate fraction from mantissa and read exponent as signed magnitude value.
#[inline(always)]
pub(super) fn from_float<M: Mantissa, T: Float>(t: T)
    -> ExtendedFloat<M>
{
    ExtendedFloat {
        frac: as_(t.significand()),
        exp: t.exponent(),
    }
}

// AS FLOAT

/// Export extended-precision float to native float.
///
/// The extended-precision float must be in native float representation,
/// with overflow/underflow appropriately handled.
#[inline]
pub(super) fn as_float<M: Mantissa, T: Float>(fp: ExtendedFloat<M>)
    -> T
{
    // Export floating-point number.
    if fp.frac.is_zero() || fp.exp < T::DENORMAL_EXPONENT {
        // sub-denormal, underflow
        T::ZERO
    } else if fp.exp >= T::MAX_EXPONENT {
        // overflow
        T::from_bits(T::INFINITY_BITS)
    } else {
        // calculate the exp and fraction bits, and return a float from bits.
        let exp: M;
        if (fp.exp == T::DENORMAL_EXPONENT) && (fp.frac & as_::<M, _>(T::HIDDEN_BIT_MASK)).is_zero() {
            exp = M::ZERO;
        } else {
            exp = as_::<M, _>(fp.exp + T::EXPONENT_BIAS);
        }
        let exp = exp << T::MANTISSA_SIZE;
        let frac = fp.frac & as_::<M, _>(T::MANTISSA_MASK);
        T::from_bits(as_(frac | exp))
    }
}
