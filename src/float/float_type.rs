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
pub struct FloatType {
    /// Has ~80 bits of precision (~16 for exponent).
    /// Use the 32-bit type first, for a packed alignment.
    pub exp: i32,
    pub frac: u64,
}

impl FloatType {
    // OPERATIONS

    /// Multiply two normalized extended-precision floats, as if by `a*b`.
    ///
    /// The result is not normalized.
    ///
    /// Algorithm:
    ///     1. Non-signed multiplication of mantissas (requires 2x as many bits as input).
    ///     2. Normalization of the result (not done here).
    ///     3. Addition of exponents.
    #[inline]
    pub unsafe fn fast_multiply(&self, b: &FloatType) -> FloatType
    {
        const LOMASK: u64 = 0x00000000FFFFFFFF;

        // Extract high-and-low masks.
        let ah = self.frac >> 32;
        let al = self.frac & LOMASK;
        let bh = b.frac >> 32;
        let bl = b.frac & LOMASK;

        // Get our products
        let ah_bl = ah * bl;
        let al_bh = al * bh;
        let al_bl = al * bl;
        let ah_bh = ah * bh;

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
        shl!(self, shift);
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

    // ROUND

    /// Lossy round float-point number to f32 fraction boundaries.
    #[inline]
    fn round_to_f32(&mut self)
    {
        const DENORMAL: i32 = 1 - F32_EXPONENT_BIAS;
        const MAX: i32 = 0xFF - F32_EXPONENT_BIAS;
        // Every mask from the hidden bit over, to see if we can
        // shift-left in 1 operation.
        const MASKS: [u64; 24] = [
            0x00800000, 0x00C00000, 0x00E00000, 0x00F00000, 0x00F80000, 0x00FC0000,
            0x00FE0000, 0x00FF0000, 0x00FF8000, 0x00FFC000, 0x00FFE000, 0x00FFF000,
            0x00FFF800, 0x00FFFC00, 0x00FFFE00, 0x00FFFF00, 0x00FFFF80, 0x00FFFFC0,
            0x00FFFFE0, 0x00FFFFF0, 0x00FFFFF8, 0x00FFFFFC, 0x00FFFFFE, 0x00FFFFFF
        ];

        round_to_f32!(self, DENORMAL, MAX, MASKS)
    }

    /// Lossy round float-point number to f64 fraction boundaries.
    #[inline]
    fn round_to_f64(&mut self)
    {
        const DENORMAL: i32 = 1 - F64_EXPONENT_BIAS;
        const MAX: i32 = 0x7FF - F64_EXPONENT_BIAS;
        // Every mask from the hidden bit over, to see if we can
        // shift-left in 1 operation.
        const MASKS: [u64; 53] = [
            0x0010000000000000, 0x0018000000000000, 0x001C000000000000,
            0x001E000000000000, 0x001F000000000000, 0x001F800000000000,
            0x001FC00000000000, 0x001FE00000000000, 0x001FF00000000000,
            0x001FF80000000000, 0x001FFC0000000000, 0x001FFE0000000000,
            0x001FFF0000000000, 0x001FFF8000000000, 0x001FFFC000000000,
            0x001FFFE000000000, 0x001FFFF000000000, 0x001FFFF800000000,
            0x001FFFFC00000000, 0x001FFFFE00000000, 0x001FFFFF00000000,
            0x001FFFFF80000000, 0x001FFFFFC0000000, 0x001FFFFFE0000000,
            0x001FFFFFF0000000, 0x001FFFFFF8000000, 0x001FFFFFFC000000,
            0x001FFFFFFE000000, 0x001FFFFFFF000000, 0x001FFFFFFF800000,
            0x001FFFFFFFC00000, 0x001FFFFFFFE00000, 0x001FFFFFFFF00000,
            0x001FFFFFFFF80000, 0x001FFFFFFFFC0000, 0x001FFFFFFFFE0000,
            0x001FFFFFFFFF0000, 0x001FFFFFFFFF8000, 0x001FFFFFFFFFC000,
            0x001FFFFFFFFFE000, 0x001FFFFFFFFFF000, 0x001FFFFFFFFFF800,
            0x001FFFFFFFFFFC00, 0x001FFFFFFFFFFE00, 0x001FFFFFFFFFFF00,
            0x001FFFFFFFFFFF80, 0x001FFFFFFFFFFFC0, 0x001FFFFFFFFFFFE0,
            0x001FFFFFFFFFFFF0, 0x001FFFFFFFFFFFF8, 0x001FFFFFFFFFFFFC,
            0x001FFFFFFFFFFFFE, 0x001FFFFFFFFFFFFF
        ];

        round_to_f64!(self, DENORMAL, MAX, MASKS)
    }

    // FROM

    /// Create extended float from 8-bit unsigned integer.
    #[inline]
    pub fn from_u8(i: u8) -> FloatType {
        from_int!(i)
    }

    /// Create extended float from 16-bit unsigned integer.
    #[inline]
    pub fn from_u16(i: u16) -> FloatType {
        from_int!(i)
    }

    /// Create extended float from 32-bit unsigned integer.
    #[inline]
    pub fn from_u32(i: u32) -> FloatType {
        from_int!(i)
    }

    /// Create extended float from 64-bit unsigned integer.
    #[inline]
    pub fn from_u64(i: u64) -> FloatType {
        from_int!(i)
    }

    /// Create extended float from 32-bit float.
    #[inline]
    pub fn from_f32(f: f32) -> FloatType {
        const EXPONENT: u64 = F32_EXPONENT_MASK as u64;
        const HIDDEN: u64 = F32_HIDDEN_BIT_MASK as u64;
        const FRACTION: u64 = F32_FRACTION_MASK as u64;
        from_float!(f, EXPONENT, HIDDEN, FRACTION, F32_EXPONENT_BIAS, F32_SIGNIFICAND_SIZE)
    }

    /// Create extended float from 64-bit float.
    #[inline]
    pub fn from_f64(f: f64) -> FloatType {
        const EXPONENT: u64 = F64_EXPONENT_MASK;
        const HIDDEN: u64 = F64_HIDDEN_BIT_MASK;
        const FRACTION: u64 = F64_FRACTION_MASK;
        from_float!(f, EXPONENT, HIDDEN, FRACTION, F64_EXPONENT_BIAS, F64_SIGNIFICAND_SIZE)
    }

    // TO

    /// Convert to lower-precision 32-bit float.
    #[inline]
    pub fn as_f32(&self) -> f32 {
        const DENORMAL: i32 = 1 - F32_EXPONENT_BIAS;
        const MAX: i32 = 0xFF - F32_EXPONENT_BIAS;
        const HIDDEN: u64 = F32_HIDDEN_BIT_MASK as u64;
        const FRACTION: u64 = F32_FRACTION_MASK as u64;
        const BIAS: i32 = F32_EXPONENT_BIAS;
        const INF: u32 = U32_INFINITY;
        const SIG_SIZE: i32 = F32_SIGNIFICAND_SIZE;

        // Create a normalized fraction for export.
        let mut x = *self;
        x.round_to_f32();
        as_float!(x, f32, u32, DENORMAL, HIDDEN, FRACTION, BIAS, MAX, INF, SIG_SIZE)
    }

    /// Convert to lower-precision 64-bit float.
    #[inline]
    pub fn as_f64(&self) -> f64 {
        const DENORMAL: i32 = 1 - F64_EXPONENT_BIAS;
        const MAX: i32 = 0x7FF - F64_EXPONENT_BIAS;
        const HIDDEN: u64 = F64_HIDDEN_BIT_MASK;
        const FRACTION: u64 = F64_FRACTION_MASK;
        const BIAS: i32 = F64_EXPONENT_BIAS;
        const INF: u64 = U64_INFINITY;
        const SIG_SIZE: i32 = F64_SIGNIFICAND_SIZE;

        // Create a normalized fraction for export.
        let mut x = *self;
        x.round_to_f64();
        as_float!(x, f64, u64, DENORMAL, HIDDEN, FRACTION, BIAS, MAX, INF, SIG_SIZE)
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

    // ROUND

    #[test]
    fn round_to_f32_test() {
        // This is lossy, so some of these values are **slightly** rounded.

        // underflow
        let mut x = FloatType {frac: 9223372036854775808, exp: -213};
        x.round_to_f32();
        assert_eq!(x, FloatType {frac: 8388608, exp: -173});

        // min value
        let mut x = FloatType {frac: 9223372036854775808, exp: -212};
        x.round_to_f32();
        assert_eq!(x, FloatType {frac: 1, exp: -149});

        // 1.0e-40
        let mut x = FloatType {frac: 10043308644012916736, exp: -196};
        x.round_to_f32();
        assert_eq!(x, FloatType {frac: 71362, exp: -149});

        // 1.0e-20
        let mut x = FloatType {frac: 13611294244890214400, exp: -130};
        x.round_to_f32();
        assert_eq!(x, FloatType {frac: 12379400, exp: -90});

        // 1.0
        let mut x = FloatType {frac: 9223372036854775808, exp: -63};
        x.round_to_f32();
        assert_eq!(x, FloatType {frac: 8388608, exp: -23});

        // 1e20
        let mut x = FloatType {frac: 12500000250510966784, exp: 3};
        x.round_to_f32();
        assert_eq!(x, FloatType {frac: 11368684, exp: 43});

        // max value
        let mut x = FloatType {frac: 18446740775174668288, exp: 64};
        x.round_to_f32();
        assert_eq!(x, FloatType {frac: 16777213, exp: 104});

        // overflow
        let mut x = FloatType {frac: 18446740775174668288, exp: 65};
        x.round_to_f32();
        assert_eq!(x, FloatType {frac: 16777213, exp: 105});
    }

    #[test]
    fn round_to_f64_test() {
        // This is lossy, so some of these values are **slightly** rounded.

        // underflow
        let mut x = FloatType {frac: 9223372036854775808, exp: -1138};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 4503599627370496, exp: -1127});

        // min value
        let mut x = FloatType {frac: 9223372036854775808, exp: -1137};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 1, exp: -1074});

        // 1.0e-250
        let mut x = FloatType {frac: 13207363278391631872, exp: -894};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 6448907850777164, exp: -883});

        // 1.0e-150
        let mut x = FloatType {frac: 15095849699286165504, exp: -562};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 7371020360979573, exp: -551});

        // 1.0e-45
        let mut x = FloatType {frac: 13164036458569648128, exp: -213};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 6427752177035961, exp: -202});

        // 1.0e-40
        let mut x = FloatType {frac: 10043362776618688512, exp: -196};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 4903985730770844, exp: -185});

        // 1.0e-20
        let mut x = FloatType {frac: 13611294676837537792, exp: -130};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 6646139978924579, exp: -119});

        // 1.0
        let mut x = FloatType {frac: 9223372036854775808, exp: -63};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 4503599627370496, exp: -52});

        // 1e20
        let mut x = FloatType {frac: 12500000000000000000, exp: 3};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 6103515625000000, exp: 14});

        // 1e40
        let mut x = FloatType {frac: 16940658945086007296, exp: 69};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 8271806125530277, exp: 80});

        // 1e150
        let mut x = FloatType {frac: 11270725851789228032, exp: 435};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 5503284107318959, exp: 446});

        // 1e250
        let mut x = FloatType {frac: 12882297539194265600, exp: 767};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 6290184345309700, exp: 778});

        // max value
        let mut x = FloatType {frac: 18446744073709549568, exp: 960};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 9007199254740991, exp: 971});

        // overflow
        let mut x = FloatType {frac: 18446744073709549568, exp: 961};
        x.round_to_f64();
        assert_eq!(x, FloatType {frac: 9007199254740991, exp: 972});
    }

    // FROM

    #[test]
    fn from_int_test() {
        // 0
        assert_eq!(FloatType::from_u8(0), FloatType {frac: 0, exp: 0});
        assert_eq!(FloatType::from_u16(0), FloatType {frac: 0, exp: 0});
        assert_eq!(FloatType::from_u32(0), FloatType {frac: 0, exp: 0});
        assert_eq!(FloatType::from_u64(0), FloatType {frac: 0, exp: 0});

        // 1
        assert_eq!(FloatType::from_u8(1), FloatType {frac: 1, exp: 0});
        assert_eq!(FloatType::from_u16(1), FloatType {frac: 1, exp: 0});
        assert_eq!(FloatType::from_u32(1), FloatType {frac: 1, exp: 0});
        assert_eq!(FloatType::from_u64(1), FloatType {frac: 1, exp: 0});

        // (2^8-1) 255
        assert_eq!(FloatType::from_u8(255), FloatType {frac: 255, exp: 0});
        assert_eq!(FloatType::from_u16(255), FloatType {frac: 255, exp: 0});
        assert_eq!(FloatType::from_u32(255), FloatType {frac: 255, exp: 0});
        assert_eq!(FloatType::from_u64(255), FloatType {frac: 255, exp: 0});

        // (2^16-1) 65535
        assert_eq!(FloatType::from_u16(65535), FloatType {frac: 65535, exp: 0});
        assert_eq!(FloatType::from_u32(65535), FloatType {frac: 65535, exp: 0});
        assert_eq!(FloatType::from_u64(65535), FloatType {frac: 65535, exp: 0});

        // (2^32-1) 4294967295
        assert_eq!(FloatType::from_u32(4294967295), FloatType {frac: 4294967295, exp: 0});
        assert_eq!(FloatType::from_u64(4294967295), FloatType {frac: 4294967295, exp: 0});

        // (2^64-1) 18446744073709551615
        assert_eq!(FloatType::from_u64(18446744073709551615), FloatType {frac: 18446744073709551615, exp: 0});
    }

    #[test]
    fn from_f32_test() {
        assert_eq!(FloatType::from_f32(0.), FloatType {frac: 0, exp: -149});
        assert_eq!(FloatType::from_f32(-0.), FloatType {frac: 0, exp: -149});

        assert_eq!(FloatType::from_f32(1e-45), FloatType {frac: 1, exp: -149});
        assert_eq!(FloatType::from_f32(1e-40), FloatType {frac: 71362, exp: -149});
        assert_eq!(FloatType::from_f32(2e-40), FloatType {frac: 142725, exp: -149});
        assert_eq!(FloatType::from_f32(1e-20), FloatType {frac: 12379400, exp: -90});
        assert_eq!(FloatType::from_f32(2e-20), FloatType {frac: 12379400, exp: -89});
        assert_eq!(FloatType::from_f32(1.0), FloatType {frac: 8388608, exp: -23});
        assert_eq!(FloatType::from_f32(2.0), FloatType {frac: 8388608, exp: -22});
        assert_eq!(FloatType::from_f32(1e20), FloatType {frac: 11368684, exp: 43});
        assert_eq!(FloatType::from_f32(2e20), FloatType {frac: 11368684, exp: 44});
        assert_eq!(FloatType::from_f32(3.402823e38), FloatType {frac: 16777213, exp: 104});
    }

    #[test]
    fn from_f64_test() {
        assert_eq!(FloatType::from_f64(0.), FloatType {frac: 0, exp: -1074});
        assert_eq!(FloatType::from_f64(-0.), FloatType {frac: 0, exp: -1074});
        assert_eq!(FloatType::from_f64(5e-324), FloatType {frac: 1, exp: -1074});
        assert_eq!(FloatType::from_f64(1e-250), FloatType {frac: 6448907850777164, exp: -883});
        assert_eq!(FloatType::from_f64(1e-150), FloatType {frac: 7371020360979573, exp: -551});
        assert_eq!(FloatType::from_f64(1e-45), FloatType {frac: 6427752177035961, exp: -202});
        assert_eq!(FloatType::from_f64(1e-40), FloatType {frac: 4903985730770844, exp: -185});
        assert_eq!(FloatType::from_f64(2e-40), FloatType {frac: 4903985730770844, exp: -184});
        assert_eq!(FloatType::from_f64(1e-20), FloatType {frac: 6646139978924579, exp: -119});
        assert_eq!(FloatType::from_f64(2e-20), FloatType {frac: 6646139978924579, exp: -118});
        assert_eq!(FloatType::from_f64(1.0), FloatType {frac: 4503599627370496, exp: -52});
        assert_eq!(FloatType::from_f64(2.0), FloatType {frac: 4503599627370496, exp: -51});
        assert_eq!(FloatType::from_f64(1e20), FloatType {frac: 6103515625000000, exp: 14});
        assert_eq!(FloatType::from_f64(2e20), FloatType {frac: 6103515625000000, exp: 15});
        assert_eq!(FloatType::from_f64(1e40), FloatType {frac: 8271806125530277, exp: 80});
        assert_eq!(FloatType::from_f64(2e40), FloatType {frac: 8271806125530277, exp: 81});
        assert_eq!(FloatType::from_f64(1e150), FloatType {frac: 5503284107318959, exp: 446});
        assert_eq!(FloatType::from_f64(1e250), FloatType {frac: 6290184345309700, exp: 778});
        assert_eq!(FloatType::from_f64(1.7976931348623157e308), FloatType {frac: 9007199254740991, exp: 971});
    }

    fn assert_normalized_eq(mut x: FloatType, mut y: FloatType) {
        x.normalize();
        y.normalize();
        assert_eq!(x, y);
    }

    #[test]
    fn from_float() {
        let values: [f32; 26] = [
            1e-40,
            2e-40,
            1e-35,
            2e-35,
            1e-30,
            2e-30,
            1e-25,
            2e-25,
            1e-20,
            2e-20,
            1e-15,
            2e-15,
            1e-10,
            2e-10,
            1e-5,
            2e-5,
            1.0,
            2.0,
            1e5,
            2e5,
            1e10,
            2e10,
            1e15,
            2e15,
            1e20,
            2e20,
        ];
        for value in values.iter() {
            assert_normalized_eq(FloatType::from_f32(*value), FloatType::from_f64(*value as f64));
        }
    }

    // TO

    // Sample of interesting numbers to check during standard test builds.
    const INTEGERS: [u64; 32] = [
        0,                      // 0x0
        1,                      // 0x1
        7,                      // 0x7
        15,                     // 0xF
        112,                    // 0x70
        119,                    // 0x77
        127,                    // 0x7F
        240,                    // 0xF0
        247,                    // 0xF7
        255,                    // 0xFF
        2032,                   // 0x7F0
        2039,                   // 0x7F7
        2047,                   // 0x7FF
        4080,                   // 0xFF0
        4087,                   // 0xFF7
        4095,                   // 0xFFF
        65520,                  // 0xFFF0
        65527,                  // 0xFFF7
        65535,                  // 0xFFFF
        1048560,                // 0xFFFF0
        1048567,                // 0xFFFF7
        1048575,                // 0xFFFFF
        16777200,               // 0xFFFFF0
        16777207,               // 0xFFFFF7
        16777215,               // 0xFFFFFF
        268435440,              // 0xFFFFFF0
        268435447,              // 0xFFFFFF7
        268435455,              // 0xFFFFFFF
        4294967280,             // 0xFFFFFFF0
        4294967287,             // 0xFFFFFFF7
        4294967295,             // 0xFFFFFFFF
        18446744073709551615,   // 0xFFFFFFFFFFFFFFFF
    ];

    #[test]
    fn to_f32_test() {
        // underflow
        let x = FloatType {frac: 9223372036854775808, exp: -213};
        assert_eq!(x.as_f32(), 0.0);

        // min value
        let x = FloatType {frac: 9223372036854775808, exp: -212};
        assert_eq!(x.as_f32(), 1e-45);

        // 1.0e-40
        let x = FloatType {frac: 10043308644012916736, exp: -196};
        assert_eq!(x.as_f32(), 1e-40);

        // 1.0e-20
        let x = FloatType {frac: 13611294244890214400, exp: -130};
        assert_eq!(x.as_f32(), 1e-20);

        // 1.0
        let x = FloatType {frac: 9223372036854775808, exp: -63};
        assert_eq!(x.as_f32(), 1.0);

        // 1e20
        let x = FloatType {frac: 12500000250510966784, exp: 3};
        assert_eq!(x.as_f32(), 1e20);

        // max value
        let x = FloatType {frac: 18446740775174668288, exp: 64};
        assert_eq!(x.as_f32(), 3.402823e38);

        // almost max, high exp
        let x = FloatType {frac: 1048575, exp: 108};
        assert_eq!(x.as_f32(), 3.4028204e38);

        // max value + 1
        let x = FloatType {frac: 16777216, exp: 104};
        assert_eq!(x.as_f32(), F32_INFINITY);

        // max value + 1
        let x = FloatType {frac: 1048576, exp: 108};
        assert_eq!(x.as_f32(), F32_INFINITY);

        // 1e40
        let x = FloatType {frac: 16940658945086007296, exp: 69};
        assert_eq!(x.as_f32(), F32_INFINITY);

        // Integers.
        for int in INTEGERS.iter() {
            let fp = FloatType {frac: *int, exp: 0};
            assert_eq!(fp.as_f32(), *int as f32, "{:?} as f32", *int);
        }
    }

    #[test]
    fn to_f64_test() {
        // underflow
        let x = FloatType {frac: 9223372036854775808, exp: -1138};
        assert_relative_eq!(x.as_f64(), 0.0);

        // min value
        let x = FloatType {frac: 9223372036854775808, exp: -1137};
        assert_relative_eq!(x.as_f64(), 5e-324);

        // 1.0e-250
        let x = FloatType {frac: 13207363278391631872, exp: -894};
        assert_relative_eq!(x.as_f64(), 1e-250);

        // 1.0e-150
        let x = FloatType {frac: 15095849699286165504, exp: -562};
        assert_relative_eq!(x.as_f64(), 1e-150);

        // 1.0e-45
        let x = FloatType {frac: 13164036458569648128, exp: -213};
        assert_relative_eq!(x.as_f64(), 1e-45);

        // 1.0e-40
        let x = FloatType {frac: 10043362776618688512, exp: -196};
        assert_relative_eq!(x.as_f64(), 1e-40);

        // 1.0e-20
        let x = FloatType {frac: 13611294676837537792, exp: -130};
        assert_relative_eq!(x.as_f64(), 1e-20);

        // 1.0
        let x = FloatType {frac: 9223372036854775808, exp: -63};
        assert_relative_eq!(x.as_f64(), 1.0);

        // 1e20
        let x = FloatType {frac: 12500000000000000000, exp: 3};
        assert_relative_eq!(x.as_f64(), 1e20);

        // 1e40
        let x = FloatType {frac: 16940658945086007296, exp: 69};
        assert_relative_eq!(x.as_f64(), 1e40);

        // 1e150
        let x = FloatType {frac: 11270725851789228032, exp: 435};
        assert_relative_eq!(x.as_f64(), 1e150);

        // 1e250
        let x = FloatType {frac: 12882297539194265600, exp: 767};
        assert_relative_eq!(x.as_f64(), 1e250);

        // max value
        let x = FloatType {frac: 9007199254740991, exp: 971};
        assert_relative_eq!(x.as_f64(), 1.7976931348623157e308);

        // max value
        let x = FloatType {frac: 18446744073709549568, exp: 960};
        assert_relative_eq!(x.as_f64(), 1.7976931348623157e308);

        // overflow
        let x = FloatType {frac: 9007199254740992, exp: 971};
        assert_relative_eq!(x.as_f64(), F64_INFINITY);

        // overflow
        let x = FloatType {frac: 18446744073709549568, exp: 961};
        assert_relative_eq!(x.as_f64(), F64_INFINITY);

        // Integers.
        for int in INTEGERS.iter() {
            let fp = FloatType {frac: *int, exp: 0};
            assert_eq!(fp.as_f64(), *int as f64, "{:?} as f64", *int);
        }
    }

    #[test]
    #[ignore]
    fn to_f32_full_test() {
        // Use exhaustive search to ensure both lossy and unlossy items are checked.
        // 23-bits of precision, so go from 0-32.
        for int in 0..u32::max_value() {
            let fp = FloatType {frac: int as u64, exp: 0};
            assert_eq!(fp.as_f32(), int as f32, "{:?} as f32", int);
        }
    }

    #[test]
    #[ignore]
    fn to_f64_full_test() {
        // Use exhaustive search to ensure both lossy and unlossy items are checked.
        const U32_MAX: u64 = u32::max_value() as u64;
        const POW2_52: u64 = 4503599627370496;
        const START: u64 = POW2_52 - U32_MAX / 2;
        const END: u64 = START + U32_MAX;
        for int in START..END {
            let fp = FloatType {frac: int, exp: 0};
            assert_eq!(fp.as_f64(), int as f64, "{:?} as f64", int);
        }
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
        // Normalized
        let a = FloatType {frac: 6427752177035961, exp: -202};
        let b = FloatType {frac: 9223372036854775808, exp: -62};
        let c = FloatType {frac: 3213876088517981, exp: -200};
        check_fast_multiply(a, b, c);

        // Non-normalized
        unsafe {
            let mut a = FloatType::from_u8(10);
            let mut b = FloatType::from_u8(10);
            a.normalize();
            b.normalize();
            assert_eq!(a.fast_multiply(&b).as_f64(), 100.0);
        }
    }
}
