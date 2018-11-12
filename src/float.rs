//! Extended precision floating-point types.
//!
//! Also contains helpers to convert to and from native rust floats.
//! This representation stores the fraction as a 64-bit unsigned integer,
//! and the exponent as a 32-bit unsigned integer, allowed ~80 bits of
//! precision (only 16 bits of the 32-bit integer are used, u32 is used
//! for performance). Since there is no storage for the sign bit,
//! this only works for positive floats.

use util::*;

// FLOAT TYPE

/// Extended precision floating-point type.
///
/// Private implementation, exposed only for testing purposes.
#[repr(C)]
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FloatType {
    /// Has ~80 bits of precision (~16 for exponent).
    /// Use the 32-bit type first, for a packed alignment.
    pub exp: i32,
    pub frac: u64,
}

impl FloatType {
    // OPERATIONS

    /// Multiply two normalized extended-precision floats, as if by `a*b`.
    ///
    /// About as fast as `mul_n`, but requires normalized values.
    ///
    /// Algorithm:
    ///     1. Non-signed multiplication of mantissas (requires 2x as many bits as input).
    ///     2. Normalization of the result (not done here).
    ///     3. Addition of exponents.
    #[inline]
    pub unsafe fn fast_multiply(&self, b: &FloatType) -> FloatType
    {
        const LOMASK: u64 = 0x00000000FFFFFFFF;

        let ah_bl = (self.frac >> 32)    * (b.frac & LOMASK);
        let al_bh = (self.frac & LOMASK) * (b.frac >> 32);
        let al_bl = (self.frac & LOMASK) * (b.frac & LOMASK);
        let ah_bh = (self.frac >> 32)    * (b.frac >> 32);

        let mut tmp = (ah_bl & LOMASK) + (al_bh & LOMASK) + (al_bl >> 32);
        // round up
        tmp += 1 << 31;

        FloatType {
            frac: ah_bh + (ah_bl >> 32) + (al_bh >> 32) + (tmp >> 32),
            exp: self.exp + b.exp + 64
        }
    }

    // NORMALIZE

    /// Normalize float-point number.
    ///
    /// Let the integer component of the mantissa (significand) to be exactly
    /// 1 and the decimal fraction and exponent to be whatever.
    #[inline]
    pub fn normalize(&mut self)
    {
        // Note:
        // Using the cltz intrinsic via leading_zeros is way faster (~10x)
        // than shifting 1-bit at a time, via while loop, and also way
        // faster (~2x) than an unrolled loop that checks at 32, 16, 4,
        // 2, and 1 bit.
        //
        // Using a modulus of pow2 (which will get optimized to a bitwise
        // and with 0x3F or faster) is slightly slower than an if/then,
        // however, removing the if/then will likely optimize more branched
        // code as it removes conditional logic.
        let shift = self.frac.leading_zeros();
        self.frac = self.frac.wrapping_shl(shift);
        self.exp -= shift as i32;
    }

    /// Get normalized boundaries for float.
    #[inline]
    pub fn normalized_boundaries(&self) -> (FloatType, FloatType) {
        let mut upper = FloatType {
            frac: (self.frac << 1) + 1,
            exp: self.exp - 1,
        };
        upper.normalize();

        // Use a boolean hack to get 2 if they're equal, else 1, without
        // any branching.
        let is_hidden = self.frac == F64_HIDDEN_BIT_MASK;
        let l_shift: i32 = is_hidden as i32 + 1;

        let mut lower = FloatType {
            frac: (self.frac << l_shift) - 1,
            exp: self.exp - l_shift,
        };
        lower.frac <<= lower.exp - upper.exp;
        lower.exp = upper.exp;

        (lower, upper)
    }

    /// Create extended float from 64-bit float.
    #[inline]
    pub fn from_f64(f: f64) -> FloatType {
        let bits = f.to_bits() as u64;
        let mut fp = FloatType {
            frac: (bits & F64_FRACTION_MASK),
            exp: ((bits & F64_EXPONENT_MASK) >> F64_SIGNIFICAND_SIZE) as i32,
        };

        if fp.exp != 0 {
            fp.frac += F64_HIDDEN_BIT_MASK;
            fp.exp -= F64_EXPONENT_BIAS;
        } else {
            fp.exp = -F64_EXPONENT_BIAS + 1;
        }

        fp
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    // NORMALIZE

    #[test]
    fn normalize_test() {
        // F32
        // min value
        let mut x = FloatType {frac: 1, exp: -149};
        x.normalize();
        assert_eq!(x, FloatType {frac: 9223372036854775808, exp: -212});

        // 1.0e-40
        let mut x = FloatType {frac: 71362, exp: -149};
        x.normalize();
        assert_eq!(x, FloatType {frac: 10043308644012916736, exp: -196});

        // 1.0e-20
        let mut x = FloatType {frac: 12379400, exp: -90};
        x.normalize();
        assert_eq!(x, FloatType {frac: 13611294244890214400, exp: -130});

        // 1.0
        let mut x = FloatType {frac: 8388608, exp: -23};
        x.normalize();
        assert_eq!(x, FloatType {frac: 9223372036854775808, exp: -63});

        // 1e20
        let mut x = FloatType {frac: 11368684, exp: 43};
        x.normalize();
        assert_eq!(x, FloatType {frac: 12500000250510966784, exp: 3});

        // max value
        let mut x = FloatType {frac: 16777213, exp: 104};
        x.normalize();
        assert_eq!(x, FloatType {frac: 18446740775174668288, exp: 64});

        // F64

        // min value
        let mut x = FloatType {frac: 1, exp: -1074};
        x.normalize();
        assert_eq!(x, FloatType {frac: 9223372036854775808, exp: -1137});

        // 1.0e-250
        let mut x = FloatType {frac: 6448907850777164, exp: -883};
        x.normalize();
        assert_eq!(x, FloatType {frac: 13207363278391631872, exp: -894});

        // 1.0e-150
        let mut x = FloatType {frac: 7371020360979573, exp: -551};
        x.normalize();
        assert_eq!(x, FloatType {frac: 15095849699286165504, exp: -562});

        // 1.0e-45
        let mut x = FloatType {frac: 6427752177035961, exp: -202};
        x.normalize();
        assert_eq!(x, FloatType {frac: 13164036458569648128, exp: -213});

        // 1.0e-40
        let mut x = FloatType {frac: 4903985730770844, exp: -185};
        x.normalize();
        assert_eq!(x, FloatType {frac: 10043362776618688512, exp: -196});

        // 1.0e-20
        let mut x = FloatType {frac: 6646139978924579, exp: -119};
        x.normalize();
        assert_eq!(x, FloatType {frac: 13611294676837537792, exp: -130});

        // 1.0
        let mut x = FloatType {frac: 4503599627370496, exp: -52};
        x.normalize();
        assert_eq!(x, FloatType {frac: 9223372036854775808, exp: -63});

        // 1e20
        let mut x = FloatType {frac: 6103515625000000, exp: 14};
        x.normalize();
        assert_eq!(x, FloatType {frac: 12500000000000000000, exp: 3});

        // 1e40
        let mut x = FloatType {frac: 8271806125530277, exp: 80};
        x.normalize();
        assert_eq!(x, FloatType {frac: 16940658945086007296, exp: 69});

        // 1e150
        let mut x = FloatType {frac: 5503284107318959, exp: 446};
        x.normalize();
        assert_eq!(x, FloatType {frac: 11270725851789228032, exp: 435});

        // 1e250
        let mut x = FloatType {frac: 6290184345309700, exp: 778};
        x.normalize();
        assert_eq!(x, FloatType {frac: 12882297539194265600, exp: 767});

        // max value
        let mut x = FloatType {frac: 9007199254740991, exp: 971};
        x.normalize();
        assert_eq!(x, FloatType {frac: 18446744073709549568, exp: 960});
    }

    #[test]
    fn normalized_boundaries_test() {
        let fp = FloatType {frac: 4503599627370496, exp: -50};
        let u = FloatType {frac: 9223372036854775296, exp: -61};
        let l = FloatType {frac: 9223372036854776832, exp: -61};
        let (upper, lower) = fp.normalized_boundaries();
        assert_eq!(upper, u);
        assert_eq!(lower, l);
    }

    // FROM

    #[test]
    fn from_f64_test() {
        let fp = FloatType {frac: 4503599627370496, exp: -50};
        assert_eq!(FloatType::from_f64(4.0), fp);
    }

    // OPERATIONS

    fn check_fast_multiply(a: FloatType, b: FloatType, c: FloatType) {
        unsafe {
            let r = a.fast_multiply(&b);
            assert_eq!(r, c);
        }
    }

    #[test]
    fn fast_multiply_test() {
        let a = FloatType {frac: 6427752177035961, exp: -202};
        let b = FloatType {frac: 9223372036854775808, exp: -62};
        let c = FloatType {frac: 3213876088517981, exp: -200};
        check_fast_multiply(a, b, c);
    }
}
