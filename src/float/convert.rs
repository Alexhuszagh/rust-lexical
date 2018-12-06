//! Convert between extended-precision and native floats/integers.

use lib::mem;
use util::*;
use super::float::ExtendedFloat;
use super::mantissa::Mantissa;

// FROM INT

/// Import ExtendedFloat from integer.
///
/// This works because we call normalize before any operation, which
/// allows us to convert the integer representation to the float one.
#[inline(always)]
pub(crate) fn from_int<T, M>(t: T)
    -> ExtendedFloat<M>
    where T: Integer,
          M: Mantissa
{
    debug_assert!(mem::size_of::<T>() <= mem::size_of::<M>(), "Possible truncation in ExtendedFloat::from_int.");

    ExtendedFloat {
        mant: as_cast(t),
        exp: 0,
    }
}

// FROM FLOAT

/// Import ExtendedFloat from native float.
#[inline(always)]
pub(crate) fn from_float<T, M>(t: T)
    -> ExtendedFloat<M>
    where T: Float,
          M: Mantissa
{
    ExtendedFloat {
        mant: as_cast(t.mantissa()),
        exp: t.exponent(),
    }
}

// AS FLOAT

/// Export extended-precision float to native float.
///
/// The extended-precision float must be in native float representation,
/// with overflow/underflow appropriately handled.
#[inline]
pub(crate) fn into_float<T, M>(fp: ExtendedFloat<M>)
    -> T
    where T: Float,
          M: Mantissa
{
    // Export floating-point number.
    if fp.mant.is_zero() || fp.exp < T::DENORMAL_EXPONENT {
        // sub-denormal, underflow
        T::ZERO
    } else if fp.exp >= T::MAX_EXPONENT {
        // overflow
        T::from_bits(T::INFINITY_BITS)
    } else {
        // calculate the exp and fraction bits, and return a float from bits.
        let exp: M;
        if (fp.exp == T::DENORMAL_EXPONENT) && (fp.mant & as_cast::<M, _>(T::HIDDEN_BIT_MASK)).is_zero() {
            exp = M::ZERO;
        } else {
            exp = as_cast::<M, _>(fp.exp + T::EXPONENT_BIAS);
        }
        let exp = exp << T::MANTISSA_SIZE;
        let mant = fp.mant & as_cast::<M, _>(T::MANTISSA_MASK);
        T::from_bits(as_cast(mant | exp))
    }
}
