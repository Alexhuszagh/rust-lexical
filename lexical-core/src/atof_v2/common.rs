use crate::util::*;
use super::float::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct AdjustedMantissa<F: FloatType> {
    // TODO(ahuszagh) Rename these.
    pub mantissa: F::Mantissa,
    // Unbias the power2? Maybe?
    // Do it after we finish everything else.
    pub power2: i32,
}

// TODO(ahuszagh) This is just ExtendedFloat with a biased exponent.
//  Remove the bias, and we're good to go.
impl<F: FloatType> AdjustedMantissa<F> {
    #[inline]
    pub fn zero_pow2(power2: i32) -> Self {
        Self {
            mantissa: F::Mantissa::ZERO,
            power2,
        }
    }

    #[inline]
    pub fn error() -> Self {
        Self {
            mantissa: F::Mantissa::ZERO,
            power2: -1,
        }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            mantissa: F::Mantissa::ZERO,
            power2: 0,
        }
    }

    #[inline]
    pub fn inf() -> Self {
        Self {
            mantissa: F::Mantissa::ZERO,
            power2: Self::inf_exp(),
        }
    }

    #[inline]
    pub fn inf_exp() -> i32 {
        F::MAX_EXPONENT + F::EXPONENT_BIAS
    }
}
