//! Extended precision floating-point types.
//!
//! Also contains helpers to convert to and from native rust floats.
//! This representation stores the fraction as a 64-bit unsigned integer,
//! and the exponent as a 32-bit unsigned integer, allowed ~80 bits of
//! precision (only 16 bits of the 32-bit integer are used, u32 is used
//! for performance). Since there is no storage for the sign bit,
//! this only works for positive floats.
// Lot of useful algorithms in here, and helper utilities.
// We want to make sure this code is not accidentally deleted.
#![allow(dead_code)]

use util::*;
use super::convert::*;
use super::mantissa::Mantissa;
use super::rounding::*;
use super::shift::*;

// FLOAT TYPE

/// Extended precision floating-point type.
///
/// Private implementation, exposed only for testing purposes.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtendedFloat<M: Mantissa> {
    /// Has ~80 bits of precision (~16 for exponent).
    /// Use the 32-bit type first, for a packed alignment.
    pub exp: i32,
    pub frac: M,
}

impl<M: Mantissa> ExtendedFloat<M> {
    // PROPERTIES

    /// Get the mantissa component.
    #[inline(always)]
    pub fn mantissa(&self)
        -> M
    {
        self.frac
    }

    /// Get the exponent component.
    #[inline(always)]
    pub fn exponent(&self)
        -> i32
    {
        self.exp
    }

    // OPERATIONS

    /// Multiply two normalized extended-precision floats, as if by `a*b`.
    ///
    /// The precision is maximal when the numbers are normalized, however,
    /// decent precision will occur as long as both values have high bits
    /// set. The result is not normalized.
    ///
    /// Algorithm:
    ///     1. Non-signed multiplication of mantissas (requires 2x as many bits as input).
    ///     2. Normalization of the result (not done here).
    ///     3. Addition of exponents.
    #[inline]
    pub unsafe fn mul(&self, b: &ExtendedFloat<M>)
        -> ExtendedFloat<M>
    {
        // Logic check, values must be decently normalized prior to multiplication.
        debug_assert!((self.frac & M::HIMASK != M::ZERO) && (b.frac & M::HIMASK != M::ZERO));

        // Extract high-and-low masks.
        let ah = self.frac >> M::HALF;
        let al = self.frac & M::LOMASK;
        let bh = b.frac >> M::HALF;
        let bl = b.frac & M::LOMASK;

        // Get our products
        let ah_bl = ah * bl;
        let al_bh = al * bh;
        let al_bl = al * bl;
        let ah_bh = ah * bh;

        let mut tmp = (ah_bl & M::LOMASK) + (al_bh & M::LOMASK) + (al_bl >> M::HALF);
        // round up
        tmp += M::ONE << (M::HALF-1);

        ExtendedFloat {
            frac: ah_bh + (ah_bl >> M::HALF) + (al_bh >> M::HALF) + (tmp >> M::HALF),
            exp: self.exp + b.exp + M::FULL
        }
    }

    /// Multiply in-place, as if by `a*b`.
    ///
    /// The result is not normalized.
    #[inline]
    pub unsafe fn imul(&mut self, b: &ExtendedFloat<M>)
    {
        *self = self.mul(b);
    }

    // NORMALIZE

    /// Get if extended-float is normalized, MSB is set.
    pub fn is_normalized(&self)
        -> bool
    {
        self.frac & M::NORMALIZED_MASK == M::NORMALIZED_MASK
    }

    /// Normalize float-point number.
    ///
    /// Let the integer component of the mantissa (significand) to be exactly
    /// 1 and the decimal fraction and exponent to be whatever.
    ///
    /// Get the number of bytes shifted.
    #[inline]
    pub fn normalize(&mut self)
        -> u32
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

        // Calculate the number of leading zeros, and then zero-out
        // any overflowing bits, to avoid shl overflow when self.frac == 0.
        let shift = self.frac.leading_zeros();
        let shift = shift & (M::BITS as u32 - 1);
        shl(self, shift);
        shift
    }

    /// Get normalized boundaries for float.
    #[inline]
    pub fn normalized_boundaries(&self)
        -> (ExtendedFloat<M>, ExtendedFloat<M>)
    {
        let mut upper = ExtendedFloat {
            frac: (self.frac << 1) + M::ONE,
            exp: self.exp - 1,
        };
        upper.normalize();

        // Use a boolean hack to get 2 if they're equal, else 1, without
        // any branching.
        let is_hidden = self.frac == as_cast::<M, _>(f64::HIDDEN_BIT_MASK);
        let l_shift: i32 = is_hidden as i32 + 1;

        let mut lower = ExtendedFloat {
            frac: (self.frac << l_shift) - M::ONE,
            exp: self.exp - l_shift,
        };
        lower.frac <<= lower.exp - upper.exp;
        lower.exp = upper.exp;

        (lower, upper)
    }

    // ROUND

    /// Lossy round float-point number to native fraction boundaries.
    #[inline]
    fn round_to_native<F, Cb>(&mut self, cb: Cb)
        where F: FloatRounding<M>,
              Cb: FnOnce(&mut ExtendedFloat<M>, i32)
    {
        round_to_native::<F, M, _>(self, cb)
    }

    /// Lossy round float-point number to f32 fraction boundaries.
    #[inline]
    fn round_to_f32<Cb>(&mut self, cb: Cb)
        where f32: FloatRounding<M>,
              Cb: FnOnce(&mut ExtendedFloat<M>, i32)
    {
        self.round_to_native::<f32, Cb>(cb)
    }

    /// Lossy round float-point number to f64 fraction boundaries.
    #[inline]
    fn round_to_f64<Cb>(&mut self, cb: Cb)
        where f64: FloatRounding<M>,
              Cb: FnOnce(&mut ExtendedFloat<M>, i32)
    {
        self.round_to_native::<f64, Cb>(cb)
    }

    // FROM

    /// Create extended float from 8-bit unsigned integer.
    #[inline]
    pub fn from_int<T: Integer>(i: T)
        -> ExtendedFloat<M>
    {
        from_int(i)
    }

    /// Create extended float from 8-bit unsigned integer.
    #[inline]
    pub fn from_u8(i: u8)
        -> ExtendedFloat<M>
    {
        Self::from_int(i)
    }

    /// Create extended float from 16-bit unsigned integer.
    #[inline]
    pub fn from_u16(i: u16)
        -> ExtendedFloat<M>
    {
        Self::from_int(i)
    }

    /// Create extended float from 32-bit unsigned integer.
    #[inline]
    pub fn from_u32(i: u32)
        -> ExtendedFloat<M>
    {
        Self::from_int(i)
    }

    /// Create extended float from 64-bit unsigned integer.
    #[inline]
    pub fn from_u64(i: u64)
        -> ExtendedFloat<M>
    {
        Self::from_int(i)
    }

    /// Create extended float from native float.
    #[inline]
    pub fn from_float<F: Float>(f: F)
        -> ExtendedFloat<M>
    {
        from_float(f)
    }

    /// Create extended float from 32-bit float.
    #[inline]
    pub fn from_f32(f: f32)
        -> ExtendedFloat<M>
    {
        Self::from_float(f)
    }

    /// Create extended float from 64-bit float.
    #[inline]
    pub fn from_f64(f: f64)
        -> ExtendedFloat<M>
    {
        Self::from_float(f)
    }

    // INTO

    /// Convert into lower-precision native float.
    #[inline]
    pub fn into_float<F: FloatRounding<M>>(mut self)
        -> F
    {
        self.round_to_native::<F, _>(round_nearest_tie_even);
        into_float(self)
    }

    /// Convert into lower-precision 32-bit float.
    #[inline]
    pub fn into_f32(self)
        -> f32
        where f32: FloatRounding<M>
    {
        self.into_float()
    }

    /// Convert into lower-precision 64-bit float.
    #[inline]
    pub fn into_f64(self)
        -> f64
        where f64: FloatRounding<M>
    {
        self.into_float()
    }

    // INTO ROUNDED

    /// Convert into lower-precision native float with custom rounding rules.
    #[inline]
    pub fn into_rounded_float<F>(mut self, kind: RoundingKind)
        -> F
        where F: FloatRounding<M>
    {
        let cb = match kind {
            RoundingKind::NearestTieEven     => round_nearest_tie_even,
            RoundingKind::NearestTieAwayZero => round_nearest_tie_even,
            RoundingKind::TowardInfinity     => round_toward_infinity,
            RoundingKind::TowardZero         => round_toward_zero,
        };

        self.round_to_native::<F, _>(cb);
        into_float(self)
    }

    /// Convert into lower-precision 32-bit float with custom rounding rules.
    #[inline]
    pub fn into_rounded_f32(self, kind: RoundingKind)
        -> f32
        where f32: FloatRounding<M>
    {
        self.into_rounded_float(kind)
    }

    /// Convert into lower-precision 64-bit float with custom rounding rules.
    #[inline]
    pub fn into_rounded_f64(self, kind: RoundingKind)
        -> f64
        where f64: FloatRounding<M>
    {
        self.into_rounded_float(kind)
    }

    // AS

    /// Convert to lower-precision native float.
    #[inline]
    pub fn as_float<F: FloatRounding<M>>(&self)
        -> F
    {
        self.clone().into_float::<F>()
    }

    /// Convert to lower-precision 32-bit float.
    #[inline]
    pub fn as_f32(&self)
        -> f32
        where f32: FloatRounding<M>
    {
        self.as_float()
    }

    /// Convert to lower-precision 64-bit float.
    #[inline]
    pub fn as_f64(&self)
        -> f64
        where f64: FloatRounding<M>
    {
        self.as_float()
    }

    // AS ROUNDED

    /// Convert to lower-precision native float with custom rounding rules.
    #[inline]
    pub fn as_rounded_float<F>(&self, kind: RoundingKind)
        -> F
        where F: FloatRounding<M>
    {
        self.clone().into_rounded_float::<F>(kind)
    }

    /// Convert to lower-precision 32-bit float with custom rounding rules.
    #[inline]
    pub fn as_rounded_f32(&self, kind: RoundingKind)
        -> f32
        where f32: FloatRounding<M>
    {
        self.as_rounded_float(kind)
    }

    /// Convert to lower-precision 64-bit float with custom rounding rules.
    #[inline]
    pub fn as_rounded_f64(&self, kind: RoundingKind)
        -> f64
        where f64: FloatRounding<M>
    {
        self.as_rounded_float(kind)
    }
}

impl ExtendedFloat<u128> {
    /// Create extended float from 64-bit unsigned integer.
    #[inline]
    pub fn from_u128(i: u128) -> ExtendedFloat<u128> {
        Self::from_int(i)
    }
}

// ALIASES

/// Alias with ~80 bits of precision, 64 for the mantissa and 16 for exponent.
pub type ExtendedFloat80 = ExtendedFloat<u64>;

/// Alias with ~160 bits of precision, 128 for the mantissa and 32 for exponent.
pub type ExtendedFloat160 = ExtendedFloat<u128>;

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    // NORMALIZE

    fn check_normalize(frac: u64, exp: i32, shift: u32, r_frac: u64, r_exp: i32) {
        let mut x = ExtendedFloat {frac: frac, exp: exp};
        assert!(!x.is_normalized());
        assert_eq!(x.normalize(), shift);
        assert_eq!(x, ExtendedFloat {frac: r_frac, exp: r_exp});
        assert!(x.is_normalized() || x.frac.is_zero());

        let mut x = ExtendedFloat {frac: frac as u128, exp: exp};
        let shift = if shift == 0 { 0 } else { shift+64 };
        let r_exp = if r_exp == 0 { 0 } else { r_exp-64 };
        assert!(!x.is_normalized());
        assert_eq!(x.normalize(), shift);
        assert_eq!(x, ExtendedFloat {frac: (r_frac as u128) << 64, exp: r_exp});
        assert!(x.is_normalized() || x.frac.is_zero());
    }

    #[test]
    fn normalize_test() {
        // F32
        // 0
        check_normalize(0, 0, 0, 0, 0);

        // min value
        check_normalize(1, -149, 63, 9223372036854775808, -212);

        // 1.0e-40
        check_normalize(71362, -149, 47, 10043308644012916736, -196);

        // 1.0e-20
        check_normalize(12379400, -90, 40, 13611294244890214400, -130);

        // 1.0
        check_normalize(8388608, -23, 40, 9223372036854775808, -63);

        // 1e20
        check_normalize(11368684, 43, 40, 12500000250510966784, 3);

        // max value
        check_normalize(16777213, 104, 40, 18446740775174668288, 64);

        // F64

        // min value
        check_normalize(1, -1074, 63, 9223372036854775808, -1137);

        // 1.0e-250
        check_normalize(6448907850777164, -883, 11, 13207363278391631872, -894);

        // 1.0e-150
        check_normalize(7371020360979573, -551, 11, 15095849699286165504, -562);

        // 1.0e-45
        check_normalize(6427752177035961, -202, 11, 13164036458569648128, -213);

        // 1.0e-40
        check_normalize(4903985730770844, -185, 11, 10043362776618688512, -196);

        // 1.0e-20
        check_normalize(6646139978924579, -119, 11, 13611294676837537792, -130);

        // 1.0
        check_normalize(4503599627370496, -52, 11, 9223372036854775808, -63);

        // 1e20
        check_normalize(6103515625000000, 14, 11, 12500000000000000000, 3);

        // 1e40
        check_normalize(8271806125530277, 80, 11, 16940658945086007296, 69);

        // 1e150
        check_normalize(5503284107318959, 446, 11, 11270725851789228032, 435);

        // 1e250
        check_normalize(6290184345309700, 778, 11, 12882297539194265600, 767);

        // max value
        check_normalize(9007199254740991, 971, 11, 18446744073709549568, 960);
    }

    #[test]
    fn normalized_boundaries_test() {
        let fp = ExtendedFloat80 {frac: 4503599627370496, exp: -50};
        let u = ExtendedFloat80 {frac: 9223372036854775296, exp: -61};
        let l = ExtendedFloat80 {frac: 9223372036854776832, exp: -61};
        let (upper, lower) = fp.normalized_boundaries();
        assert_eq!(upper, u);
        assert_eq!(lower, l);
    }

    // ROUND

    fn check_round_to_f32(frac: u64, exp: i32, r_frac: u64, r_exp: i32)
    {
        let mut x = ExtendedFloat {frac: frac, exp: exp};
        x.round_to_f32(round_nearest_tie_even);
        assert_eq!(x, ExtendedFloat {frac: r_frac, exp: r_exp});

        let mut x = ExtendedFloat {frac: (frac as u128) << 64, exp: exp-64};
        x.round_to_f32(round_nearest_tie_even);
        assert_eq!(x, ExtendedFloat {frac: r_frac as u128, exp: r_exp});
    }

    #[test]
    fn round_to_f32_test() {
        // This is lossy, so some of these values are **slightly** rounded.

        // underflow
        check_round_to_f32(9223372036854775808, -213, 0, -149);

        // min value
        check_round_to_f32(9223372036854775808, -212, 1, -149);

        // 1.0e-40
        check_round_to_f32(10043308644012916736, -196, 71362, -149);

        // 1.0e-20
        check_round_to_f32(13611294244890214400, -130, 12379400, -90);

        // 1.0
        check_round_to_f32(9223372036854775808, -63, 8388608, -23);

        // 1e20
        check_round_to_f32(12500000250510966784, 3, 11368684, 43);

        // max value
        check_round_to_f32(18446740775174668288, 64, 16777213, 104);

        // overflow
        check_round_to_f32(18446740775174668288, 65, 16777213, 105);
    }

    fn check_round_to_f64(frac: u64, exp: i32, r_frac: u64, r_exp: i32)
    {
        let mut x = ExtendedFloat {frac: frac, exp: exp};
        x.round_to_f64(round_nearest_tie_even);
        assert_eq!(x, ExtendedFloat {frac: r_frac, exp: r_exp});

        let mut x = ExtendedFloat {frac: (frac as u128) << 64, exp: exp-64};
        x.round_to_f64(round_nearest_tie_even);
        assert_eq!(x, ExtendedFloat {frac: r_frac as u128, exp: r_exp});
    }

    #[test]
    fn round_to_f64_test() {
        // This is lossy, so some of these values are **slightly** rounded.

        // underflow
        check_round_to_f64(9223372036854775808, -1138, 0, -1074);

        // min value
        check_round_to_f64(9223372036854775808, -1137, 1, -1074);

        // 1.0e-250
        check_round_to_f64(15095849699286165504, -562, 7371020360979573, -551);

        // 1.0e-150
        check_round_to_f64(15095849699286165504, -562, 7371020360979573, -551);

        // 1.0e-45
        check_round_to_f64(13164036458569648128, -213, 6427752177035961, -202);

        // 1.0e-40
        check_round_to_f64(10043362776618688512, -196, 4903985730770844, -185);

        // 1.0e-20
        check_round_to_f64(13611294676837537792, -130, 6646139978924579, -119);

        // 1.0
        check_round_to_f64(9223372036854775808, -63, 4503599627370496, -52);

        // 1e20
        check_round_to_f64(12500000000000000000, 3, 6103515625000000, 14);

        // 1e40
        check_round_to_f64(16940658945086007296, 69, 8271806125530277, 80);

        // 1e150
        check_round_to_f64(11270725851789228032, 435, 5503284107318959, 446);

        // 1e250
        check_round_to_f64(12882297539194265600, 767, 6290184345309700, 778);

        // max value
        check_round_to_f64(18446744073709549568, 960, 9007199254740991, 971);

        // Bug fixes
        // 1.2345e-308
        check_round_to_f64(10234494226754558294, -1086, 2498655817078750, -1074)
    }

    // FROM

    #[test]
    fn from_int_test() {
        // 0
        assert_eq!(ExtendedFloat80::from_u8(0), ExtendedFloat80 {frac: 0, exp: 0});
        assert_eq!(ExtendedFloat80::from_u16(0), ExtendedFloat80 {frac: 0, exp: 0});
        assert_eq!(ExtendedFloat80::from_u32(0), ExtendedFloat80 {frac: 0, exp: 0});
        assert_eq!(ExtendedFloat80::from_u64(0), ExtendedFloat80 {frac: 0, exp: 0});
        assert_eq!(ExtendedFloat160::from_u128(0), ExtendedFloat160 {frac: 0, exp: 0});

        // 1
        assert_eq!(ExtendedFloat80::from_u8(1), ExtendedFloat80 {frac: 1, exp: 0});
        assert_eq!(ExtendedFloat80::from_u16(1), ExtendedFloat80 {frac: 1, exp: 0});
        assert_eq!(ExtendedFloat80::from_u32(1), ExtendedFloat80 {frac: 1, exp: 0});
        assert_eq!(ExtendedFloat80::from_u64(1), ExtendedFloat80 {frac: 1, exp: 0});
        assert_eq!(ExtendedFloat160::from_u128(1), ExtendedFloat160 {frac: 1, exp: 0});

        // (2^8-1) 255
        assert_eq!(ExtendedFloat80::from_u8(255), ExtendedFloat80 {frac: 255, exp: 0});
        assert_eq!(ExtendedFloat80::from_u16(255), ExtendedFloat80 {frac: 255, exp: 0});
        assert_eq!(ExtendedFloat80::from_u32(255), ExtendedFloat80 {frac: 255, exp: 0});
        assert_eq!(ExtendedFloat80::from_u64(255), ExtendedFloat80 {frac: 255, exp: 0});
        assert_eq!(ExtendedFloat160::from_u128(255), ExtendedFloat160 {frac: 255, exp: 0});

        // (2^16-1) 65535
        assert_eq!(ExtendedFloat80::from_u16(65535), ExtendedFloat80 {frac: 65535, exp: 0});
        assert_eq!(ExtendedFloat80::from_u32(65535), ExtendedFloat80 {frac: 65535, exp: 0});
        assert_eq!(ExtendedFloat80::from_u64(65535), ExtendedFloat80 {frac: 65535, exp: 0});
        assert_eq!(ExtendedFloat160::from_u128(65535), ExtendedFloat160 {frac: 65535, exp: 0});

        // (2^32-1) 4294967295
        assert_eq!(ExtendedFloat80::from_u32(4294967295), ExtendedFloat80 {frac: 4294967295, exp: 0});
        assert_eq!(ExtendedFloat80::from_u64(4294967295), ExtendedFloat80 {frac: 4294967295, exp: 0});
        assert_eq!(ExtendedFloat160::from_u128(4294967295), ExtendedFloat160 {frac: 4294967295, exp: 0});

        // (2^64-1) 18446744073709551615
        assert_eq!(ExtendedFloat80::from_u64(18446744073709551615), ExtendedFloat80 {frac: 18446744073709551615, exp: 0});
        assert_eq!(ExtendedFloat160::from_u128(18446744073709551615), ExtendedFloat160 {frac: 18446744073709551615, exp: 0});

        // (2^128-1) 340282366920938463463374607431768211455
        assert_eq!(ExtendedFloat160::from_u128(340282366920938463463374607431768211455), ExtendedFloat160 {frac: 340282366920938463463374607431768211455, exp: 0});
    }

    #[test]
    fn from_f32_test() {
        assert_eq!(ExtendedFloat80::from_f32(0.), ExtendedFloat80 {frac: 0, exp: -149});
        assert_eq!(ExtendedFloat80::from_f32(-0.), ExtendedFloat80 {frac: 0, exp: -149});

        assert_eq!(ExtendedFloat80::from_f32(1e-45), ExtendedFloat80 {frac: 1, exp: -149});
        assert_eq!(ExtendedFloat80::from_f32(1e-40), ExtendedFloat80 {frac: 71362, exp: -149});
        assert_eq!(ExtendedFloat80::from_f32(2e-40), ExtendedFloat80 {frac: 142725, exp: -149});
        assert_eq!(ExtendedFloat80::from_f32(1e-20), ExtendedFloat80 {frac: 12379400, exp: -90});
        assert_eq!(ExtendedFloat80::from_f32(2e-20), ExtendedFloat80 {frac: 12379400, exp: -89});
        assert_eq!(ExtendedFloat80::from_f32(1.0), ExtendedFloat80 {frac: 8388608, exp: -23});
        assert_eq!(ExtendedFloat80::from_f32(2.0), ExtendedFloat80 {frac: 8388608, exp: -22});
        assert_eq!(ExtendedFloat80::from_f32(1e20), ExtendedFloat80 {frac: 11368684, exp: 43});
        assert_eq!(ExtendedFloat80::from_f32(2e20), ExtendedFloat80 {frac: 11368684, exp: 44});
        assert_eq!(ExtendedFloat80::from_f32(3.402823e38), ExtendedFloat80 {frac: 16777213, exp: 104});
    }

    #[test]
    fn from_f64_test() {
        assert_eq!(ExtendedFloat80::from_f64(0.), ExtendedFloat80 {frac: 0, exp: -1074});
        assert_eq!(ExtendedFloat80::from_f64(-0.), ExtendedFloat80 {frac: 0, exp: -1074});
        assert_eq!(ExtendedFloat80::from_f64(5e-324), ExtendedFloat80 {frac: 1, exp: -1074});
        assert_eq!(ExtendedFloat80::from_f64(1e-250), ExtendedFloat80 {frac: 6448907850777164, exp: -883});
        assert_eq!(ExtendedFloat80::from_f64(1e-150), ExtendedFloat80 {frac: 7371020360979573, exp: -551});
        assert_eq!(ExtendedFloat80::from_f64(1e-45), ExtendedFloat80 {frac: 6427752177035961, exp: -202});
        assert_eq!(ExtendedFloat80::from_f64(1e-40), ExtendedFloat80 {frac: 4903985730770844, exp: -185});
        assert_eq!(ExtendedFloat80::from_f64(2e-40), ExtendedFloat80 {frac: 4903985730770844, exp: -184});
        assert_eq!(ExtendedFloat80::from_f64(1e-20), ExtendedFloat80 {frac: 6646139978924579, exp: -119});
        assert_eq!(ExtendedFloat80::from_f64(2e-20), ExtendedFloat80 {frac: 6646139978924579, exp: -118});
        assert_eq!(ExtendedFloat80::from_f64(1.0), ExtendedFloat80 {frac: 4503599627370496, exp: -52});
        assert_eq!(ExtendedFloat80::from_f64(2.0), ExtendedFloat80 {frac: 4503599627370496, exp: -51});
        assert_eq!(ExtendedFloat80::from_f64(1e20), ExtendedFloat80 {frac: 6103515625000000, exp: 14});
        assert_eq!(ExtendedFloat80::from_f64(2e20), ExtendedFloat80 {frac: 6103515625000000, exp: 15});
        assert_eq!(ExtendedFloat80::from_f64(1e40), ExtendedFloat80 {frac: 8271806125530277, exp: 80});
        assert_eq!(ExtendedFloat80::from_f64(2e40), ExtendedFloat80 {frac: 8271806125530277, exp: 81});
        assert_eq!(ExtendedFloat80::from_f64(1e150), ExtendedFloat80 {frac: 5503284107318959, exp: 446});
        assert_eq!(ExtendedFloat80::from_f64(1e250), ExtendedFloat80 {frac: 6290184345309700, exp: 778});
        assert_eq!(ExtendedFloat80::from_f64(1.7976931348623157e308), ExtendedFloat80 {frac: 9007199254740991, exp: 971});
    }

    fn assert_normalized_eq<M: Mantissa>(mut x: ExtendedFloat<M>, mut y: ExtendedFloat<M>) {
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
            assert_normalized_eq(ExtendedFloat80::from_f32(*value), ExtendedFloat80::from_f64(*value as f64));
            assert_normalized_eq(ExtendedFloat160::from_f32(*value), ExtendedFloat160::from_f64(*value as f64));
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
        let x = ExtendedFloat80 {frac: 9223372036854775808, exp: -213};
        assert_eq!(x.into_f32(), 0.0);

        // min value
        let x = ExtendedFloat80 {frac: 9223372036854775808, exp: -212};
        assert_eq!(x.into_f32(), 1e-45);

        // 1.0e-40
        let x = ExtendedFloat80 {frac: 10043308644012916736, exp: -196};
        assert_eq!(x.into_f32(), 1e-40);

        // 1.0e-20
        let x = ExtendedFloat80 {frac: 13611294244890214400, exp: -130};
        assert_eq!(x.into_f32(), 1e-20);

        // 1.0
        let x = ExtendedFloat80 {frac: 9223372036854775808, exp: -63};
        assert_eq!(x.into_f32(), 1.0);

        // 1e20
        let x = ExtendedFloat80 {frac: 12500000250510966784, exp: 3};
        assert_eq!(x.into_f32(), 1e20);

        // max value
        let x = ExtendedFloat80 {frac: 18446740775174668288, exp: 64};
        assert_eq!(x.into_f32(), 3.402823e38);

        // almost max, high exp
        let x = ExtendedFloat80 {frac: 1048575, exp: 108};
        assert_eq!(x.into_f32(), 3.4028204e38);

        // max value + 1
        let x = ExtendedFloat80 {frac: 16777216, exp: 104};
        assert_eq!(x.into_f32(), f32::INFINITY);

        // max value + 1
        let x = ExtendedFloat80 {frac: 1048576, exp: 108};
        assert_eq!(x.into_f32(), f32::INFINITY);

        // 1e40
        let x = ExtendedFloat80 {frac: 16940658945086007296, exp: 69};
        assert_eq!(x.into_f32(), f32::INFINITY);

        // Integers.
        for int in INTEGERS.iter() {
            let fp = ExtendedFloat80 {frac: *int, exp: 0};
            assert_eq!(fp.into_f32(), *int as f32, "{:?} as f32", *int);
        }
    }

    #[test]
    fn to_f64_test() {
        // underflow
        let x = ExtendedFloat80 {frac: 9223372036854775808, exp: -1138};
        assert_relative_eq!(x.into_f64(), 0.0);

        // min value
        let x = ExtendedFloat80 {frac: 9223372036854775808, exp: -1137};
        assert_relative_eq!(x.into_f64(), 5e-324);

        // 1.0e-250
        let x = ExtendedFloat80 {frac: 13207363278391631872, exp: -894};
        assert_relative_eq!(x.into_f64(), 1e-250);

        // 1.0e-150
        let x = ExtendedFloat80 {frac: 15095849699286165504, exp: -562};
        assert_relative_eq!(x.into_f64(), 1e-150);

        // 1.0e-45
        let x = ExtendedFloat80 {frac: 13164036458569648128, exp: -213};
        assert_relative_eq!(x.into_f64(), 1e-45);

        // 1.0e-40
        let x = ExtendedFloat80 {frac: 10043362776618688512, exp: -196};
        assert_relative_eq!(x.into_f64(), 1e-40);

        // 1.0e-20
        let x = ExtendedFloat80 {frac: 13611294676837537792, exp: -130};
        assert_relative_eq!(x.into_f64(), 1e-20);

        // 1.0
        let x = ExtendedFloat80 {frac: 9223372036854775808, exp: -63};
        assert_relative_eq!(x.into_f64(), 1.0);

        // 1e20
        let x = ExtendedFloat80 {frac: 12500000000000000000, exp: 3};
        assert_relative_eq!(x.into_f64(), 1e20);

        // 1e40
        let x = ExtendedFloat80 {frac: 16940658945086007296, exp: 69};
        assert_relative_eq!(x.into_f64(), 1e40);

        // 1e150
        let x = ExtendedFloat80 {frac: 11270725851789228032, exp: 435};
        assert_relative_eq!(x.into_f64(), 1e150);

        // 1e250
        let x = ExtendedFloat80 {frac: 12882297539194265600, exp: 767};
        assert_relative_eq!(x.into_f64(), 1e250);

        // max value
        let x = ExtendedFloat80 {frac: 9007199254740991, exp: 971};
        assert_relative_eq!(x.into_f64(), 1.7976931348623157e308);

        // max value
        let x = ExtendedFloat80 {frac: 18446744073709549568, exp: 960};
        assert_relative_eq!(x.into_f64(), 1.7976931348623157e308);

        // overflow
        let x = ExtendedFloat80 {frac: 9007199254740992, exp: 971};
        assert_relative_eq!(x.into_f64(), f64::INFINITY);

        // overflow
        let x = ExtendedFloat80 {frac: 18446744073709549568, exp: 961};
        assert_relative_eq!(x.into_f64(), f64::INFINITY);

        // Underflow
        // Adapted from failures in strtod.
        let x = ExtendedFloat80 { exp: -1139, frac: 18446744073709550712 };
        assert_relative_eq!(x.into_f64(), 0.0);

        let x = ExtendedFloat80 { exp: -1139, frac: 18446744073709551460 };
        assert_relative_eq!(x.into_f64(), 0.0);

        let x = ExtendedFloat80 { exp: -1138, frac: 9223372036854776103 };
        assert_relative_eq!(x.into_f64(), 5e-324);

        // Integers.
        for int in INTEGERS.iter() {
            let fp = ExtendedFloat80 {frac: *int, exp: 0};
            assert_eq!(fp.into_f64(), *int as f64, "{:?} as f64", *int);
        }
    }

    #[test]
    fn to_rounded_f32_test() {
        // Just check it compiles, we already check the underlying algorithms.
        let x = ExtendedFloat80 {frac: 9223372036854775808, exp: -63};
        assert_eq!(x.as_rounded_f32(RoundingKind::NearestTieEven), 1.0);
        assert_eq!(x.as_rounded_f32(RoundingKind::NearestTieAwayZero), 1.0);
        assert_eq!(x.as_rounded_f32(RoundingKind::TowardInfinity), 1.0);
        assert_eq!(x.as_rounded_f32(RoundingKind::TowardZero), 1.0);
    }

    #[test]
    fn to_rounded_f64_test() {
        // Just check it compiles, we already check the underlying algorithms.
        let x = ExtendedFloat80 {frac: 9223372036854775808, exp: -63};
        assert_eq!(x.as_rounded_f64(RoundingKind::NearestTieEven), 1.0);
        assert_eq!(x.as_rounded_f64(RoundingKind::NearestTieAwayZero), 1.0);
        assert_eq!(x.as_rounded_f64(RoundingKind::TowardInfinity), 1.0);
        assert_eq!(x.as_rounded_f64(RoundingKind::TowardZero), 1.0);
    }

    #[test]
    #[ignore]
    fn to_f32_full_test() {
        // Use exhaustive search to ensure both lossy and unlossy items are checked.
        // 23-bits of precision, so go from 0-32.
        for int in 0..u32::max_value() {
            let fp = ExtendedFloat80 {frac: int as u64, exp: 0};
            assert_eq!(fp.into_f32(), int as f32, "ExtendedFloat80 {:?} as f32", int);

            let fp = ExtendedFloat160 {frac: int as u128, exp: 0};
            assert_eq!(fp.into_f32(), int as f32, "ExtendedFloat160 {:?} as f64", int);
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
            let fp = ExtendedFloat80 {frac: int, exp: 0};
            assert_eq!(fp.into_f64(), int as f64, "ExtendedFloat80 {:?} as f64", int);

            let fp = ExtendedFloat160 {frac: int as u128, exp: 0};
            assert_eq!(fp.into_f64(), int as f64, "ExtendedFloat160 {:?} as f64", int);
        }
    }

    // OPERATIONS

    fn check_mul<M: Mantissa>(a: ExtendedFloat<M>, b: ExtendedFloat<M>, c: ExtendedFloat<M>) {
        unsafe {
            let r = a.mul(&b);
            assert_eq!(r, c);
        }
    }

    #[test]
    fn mul_test() {
        // Normalized (64-bit mantissa)
        let a = ExtendedFloat80 {frac: 13164036458569648128, exp: -213};
        let b = ExtendedFloat80 {frac: 9223372036854775808, exp: -62};
        let c = ExtendedFloat80 {frac: 6582018229284824064, exp: -211};
        check_mul(a, b, c);

        // Normalized (128-bit mantissa)
        let a = ExtendedFloat160 {frac: 242833611528216130005140556221773774848, exp: -277};
        let b = ExtendedFloat160 {frac: 170141183460469231731687303715884105728, exp: -126};
        let c = ExtendedFloat160 {frac: 121416805764108065002570278110886887424, exp: -275};
        check_mul(a, b, c);

        // Check with integers
        unsafe {
            // 64-bit mantissa
            let mut a = ExtendedFloat80::from_u8(10);
            let mut b = ExtendedFloat80::from_u8(10);
            a.normalize();
            b.normalize();
            assert_eq!(a.mul(&b).into_f64(), 100.0);

            // 128-bit mantissa
            let mut a = ExtendedFloat160::from_u8(10);
            let mut b = ExtendedFloat160::from_u8(10);
            a.normalize();
            b.normalize();
            assert_eq!(a.mul(&b).into_f64(), 100.0);

            // Check both values need high bits set.
            let a = ExtendedFloat80 { frac: 1 << 32, exp: -31 };
            let b = ExtendedFloat80 { frac: 1 << 32, exp: -31 };
            assert_eq!(a.mul(&b).into_f64(), 4.0);

            // Check both values need high bits set.
            let a = ExtendedFloat80 { frac: 10 << 31, exp: -31 };
            let b = ExtendedFloat80 { frac: 10 << 31, exp: -31 };
            assert_eq!(a.mul(&b).into_f64(), 100.0);
        }
    }

    fn check_imul<M: Mantissa>(mut a: ExtendedFloat<M>, b: ExtendedFloat<M>, c: ExtendedFloat<M>) {
        unsafe {
            a.imul(&b);
            assert_eq!(a, c);
        }
    }

    #[test]
    fn imul_test() {
        // Normalized (64-bit mantissa)
        let a = ExtendedFloat80 {frac: 13164036458569648128, exp: -213};
        let b = ExtendedFloat80 {frac: 9223372036854775808, exp: -62};
        let c = ExtendedFloat80 {frac: 6582018229284824064, exp: -211};
        check_imul(a, b, c);

        // Normalized (128-bit mantissa)
        let a = ExtendedFloat160 {frac: 242833611528216130005140556221773774848, exp: -277};
        let b = ExtendedFloat160 {frac: 170141183460469231731687303715884105728, exp: -126};
        let c = ExtendedFloat160 {frac: 121416805764108065002570278110886887424, exp: -275};
        check_imul(a, b, c);

        // Check with integers
        unsafe {
            // 64-bit mantissa
            let mut a = ExtendedFloat80::from_u8(10);
            let mut b = ExtendedFloat80::from_u8(10);
            a.normalize();
            b.normalize();
            a.imul(&b);
            assert_eq!(a.into_f64(), 100.0);

            // 128-bit mantissa
            let mut a = ExtendedFloat160::from_u8(10);
            let mut b = ExtendedFloat160::from_u8(10);
            a.normalize();
            b.normalize();
            a.imul(&b);
            assert_eq!(a.into_f64(), 100.0);

            // Check both values need high bits set.
            let mut a = ExtendedFloat80 { frac: 1 << 32, exp: -31 };
            let b = ExtendedFloat80 { frac: 1 << 32, exp: -31 };
            a.imul(&b);
            assert_eq!(a.into_f64(), 4.0);

            // Check both values need high bits set.
            let mut a = ExtendedFloat80 { frac: 10 << 31, exp: -31 };
            let b = ExtendedFloat80 { frac: 10 << 31, exp: -31 };
            a.imul(&b);
            assert_eq!(a.into_f64(), 100.0);
        }
    }
}
