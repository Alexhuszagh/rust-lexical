//! Defines rounding schemes for floating-point numbers.

use util::*;
use super::float::ExtendedFloat;
use super::mantissa::Mantissa;
use super::shift::*;

// ENUM
// ----

/// Rounding type for float rounding.
#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RoundingKind {
    /// Round to the nearest, tie to even.
    NearestTieEven,
    /// Round to the nearest, tie away from zero.
    NearestTieAwayZero,
    /// Round up (toward infinity, since ExtendedFloat is always positive).
    TowardInfinity,
    /// Round down (toward zero, since ExtendedFloat is always positive).
    TowardZero,
}

// GENERIC
// -------

// NEAREST ROUNDING

/// Shift right N-bytes and round to the nearest.
///
/// Return if we are above halfway and if we are halfway.
#[inline]
pub(crate) fn round_nearest<M>(fp: &mut ExtendedFloat<M>, shift: i32)
    -> (bool, bool)
    where M: Mantissa
{
    // Extract the truncated bits using mask.
    // Calculate if the value of the truncated bits are either above
    // the mid-way point, or equal to it.
    //
    // For example, for 4 truncated bytes, the mask would be b1111
    // and the midway point would be b1000.
    let mask: M = lower_n_mask(as_cast(shift));
    let halfway: M = lower_n_halfway(as_cast(shift));

    let truncated_bits = fp.frac & mask;
    let is_above = truncated_bits > halfway;
    let is_halfway = truncated_bits == halfway;

    // Bit shift so the leading bit is in the hidden bit.
    overflowing_shr(fp, shift);

    (is_above, is_halfway)
}

/// Tie rounded floating point to event.
#[inline]
pub(crate) fn tie_even<M>(fp: &mut ExtendedFloat<M>, is_above: bool, is_halfway: bool)
    where M: Mantissa
{
    // Extract the last bit after shifting (and determine if it is odd).
    let is_odd = fp.frac & M::ONE == M::ONE;

    // Calculate if we need to roundup.
    // We need to roundup if we are above halfway, or if we are odd
    // and at half-way (need to tie-to-even).
    if is_above || (is_odd && is_halfway) {
        fp.frac += M::ONE;
    }
}

/// Shift right N-bytes and round nearest, tie-to-even.
///
/// Floating-point arithmetic uses round to nearest, ties to even,
/// which rounds to the nearest value, if the value is halfway in between,
/// round to an even value.
#[inline]
pub(crate) fn round_nearest_tie_even<M>(fp: &mut ExtendedFloat<M>, shift: i32)
    where M: Mantissa
{
    let (is_above, is_halfway) = round_nearest(fp, shift);
    tie_even(fp, is_above, is_halfway);
}

/// Tie rounded floating point away from zero.
#[inline]
pub(crate) fn tie_away_zero<M>(fp: &mut ExtendedFloat<M>, is_above: bool, is_halfway: bool)
    where M: Mantissa
{
    // Calculate if we need to roundup.
    // We need to roundup if we are halfway or above halfway,
    // since the value is always positive and we need to round away
    // from zero.
    if is_above || is_halfway {
        fp.frac += M::ONE;
    }
}

/// Shift right N-bytes and round nearest, tie-away-zero.
///
/// Floating-point arithmetic defines round to nearest, ties away from zero,
/// which rounds to the nearest value, if the value is halfway in between,
/// ties away from zero.
#[inline]
#[allow(dead_code)]
pub(crate) fn round_nearest_tie_away_zero<M>(fp: &mut ExtendedFloat<M>, shift: i32)
    where M: Mantissa
{
    let (is_above, is_halfway) = round_nearest(fp, shift);
    tie_away_zero(fp, is_above, is_halfway);
}

// DIRECTED ROUNDING

/// Shift right N-bytes and round toward infinity.
///
/// Floating-point arithmetic defines round toward infinity, which rounds
/// towards positive infinity.
#[inline]
#[allow(dead_code)]
pub(crate) fn round_toward_infinity<M>(fp: &mut ExtendedFloat<M>, shift: i32)
    where M: Mantissa
{
    // Extract the truncated bits using mask.
    let mask: M = lower_n_mask(as_cast(shift));
    let truncated_bits = fp.frac & mask;

    // Bit shift so the leading bit is in the hidden bit.
    overflowing_shr(fp, shift);

    // If the truncated bits are non-zero, that is, any rounding error occurred,
    // round-up.
    if truncated_bits != M::ZERO {
        fp.frac += M::ONE;
    }
}

/// Shift right N-bytes and round toward zero.
///
/// Floating-point arithmetic defines round toward zero, which rounds
/// towards positive zero.
#[inline]
#[allow(dead_code)]
pub(crate) fn round_toward_zero<M>(fp: &mut ExtendedFloat<M>, shift: i32)
    where M: Mantissa
{
    // Bit shift so the leading bit is in the hidden bit.
    // No rounding schemes, so we just ignore everything else.
    overflowing_shr(fp, shift);
}

// NATIVE FLOAT
// ------------

// FLOAT ROUNDING

/// Trait to round extended-precision floats to native representations.
pub trait FloatRounding<M: Mantissa>: Float {
    /// Default number of bits to shift (or 64 - mantissa size - 1).
    const DEFAULT_SHIFT: i32;
    /// Mask to determine if a full-carry occurred (1 in bit above hidden bit).
    const CARRY_MASK: M;
}

// Literals don't work for generic types, we need to use this as a hack.
macro_rules! float_rounding_f32 {
    ($($t:tt)*) => ($(
        impl FloatRounding<$t> for f32 {
            const DEFAULT_SHIFT: i32    = $t::FULL - f32::MANTISSA_SIZE - 1;
            const CARRY_MASK: $t        = 0x1000000;
        }
    )*)
}

float_rounding_f32! { u64 u128 }

// Literals don't work for generic types, we need to use this as a hack.
macro_rules! float_rounding_f64 {
    ($($t:tt)*) => ($(
        impl FloatRounding<$t> for f64 {
            const DEFAULT_SHIFT: i32    = $t::FULL - f64::MANTISSA_SIZE - 1;
            const CARRY_MASK: $t        = 0x20000000000000;
        }
    )*)
}

float_rounding_f64! { u64 u128 }

// ROUND TO FLOAT

/// Shift the ExtendedFloat fraction to the fraction bits in a native float.
///
/// Floating-point arithmetic uses round to nearest, ties to even,
/// which rounds to the nearest value, if the value is halfway in between,
/// round to an even value.
#[inline]
pub(crate) fn round_to_float<T, M, Cb>(fp: &mut ExtendedFloat<M>, cb: Cb)
    where T: FloatRounding<M>,
          M: Mantissa,
          Cb: FnOnce(&mut ExtendedFloat<M>, i32)
{
    // Calculate the difference to allow a single calculation
    // rather than a loop, to minimize the number of ops required.
    // This does underflow detection.
    let final_exp = fp.exp + T::DEFAULT_SHIFT;
    if final_exp < T::DENORMAL_EXPONENT {
        // We would end up with a denormal exponent, try to round to more
        // digits. Only shift right if we can avoid zeroing out the value,
        // which requires the exponent diff to be < M::BITS. The value
        // is already normalized, so we shouldn't have any issue zeroing
        // out the value.
        let diff = T::DENORMAL_EXPONENT - fp.exp;
        if diff <= M::FULL {
            // We can avoid underflow, can get a valid representation.
            cb(fp, diff);
        } else {
            // Certain underflow, assign literal 0s.
            fp.frac = M::ZERO;
            fp.exp = 0;
        }
    } else {
        cb(fp, T::DEFAULT_SHIFT);
    }

    if fp.frac & T::CARRY_MASK == T::CARRY_MASK {
        // Roundup carried over to 1 past the hidden bit.
        shr(fp, 1);
    }
}

// AVOID OVERFLOW/UNDERFLOW

/// Avoid overflow for large values, shift left as needed.
///
/// Shift until a 1-bit is in the hidden bit, if the mantissa is not 0.
#[inline]
pub(crate) fn avoid_overflow<T, M>(fp: &mut ExtendedFloat<M>)
    where T: FloatRounding<M>,
          M: Mantissa
{
    // Calculate the difference to allow a single calculation
    // rather than a loop, minimizing the number of ops required.
    if fp.exp >= T::MAX_EXPONENT {
        let diff = fp.exp - T::MAX_EXPONENT;
        if diff <= T::MANTISSA_SIZE {
            // Our overflow mask needs to start at the hidden bit, or at
            // `T::MANTISSA_SIZE+1`, and needs to have `diff+1` bits set,
            // to see if our value overflows.
            let bit = as_cast(T::MANTISSA_SIZE+1);
            let n = as_cast(diff+1);
            let mask: M = internal_n_mask(bit, n);
            if (fp.frac & mask).is_zero() {
                // If we have no 1-bit in the hidden-bit position,
                // which is index 0, we need to shift 1.
                let shift = diff + 1;
                shl(fp, shift);
            }
        }
    }
}

// ROUND TO NATIVE

/// Round an extended-precision float to a native float representation.
#[inline]
pub(crate) fn round_to_native<T, M, Cb>(fp: &mut ExtendedFloat<M>, cb: Cb)
    where T: FloatRounding<M>,
          M: Mantissa,
          Cb: FnOnce(&mut ExtendedFloat<M>, i32)
{
    // Shift all the way left, to ensure a consistent representation.
    // The following right-shifts do not work for a non-normalized number.
    fp.normalize();

    // Round so the fraction is in a native mantissa representation,
    // and avoid overflow/underflow.
    round_to_float::<T, M, _>(fp, cb);
    avoid_overflow::<T, M>(fp);
}


// TESTS
// -----

#[cfg(test)]
mod tests {
    use float::ExtendedFloat80;
    use super::*;

    // NEAREST ROUNDING

    #[test]
    fn round_nearest_test() {
        // Check exactly halfway (b'1100000')
        let mut fp = ExtendedFloat80 { frac: 0x60, exp: 0 };
        let (above, halfway) = round_nearest(&mut fp, 6);
        assert!(!above);
        assert!(halfway);
        assert_eq!(fp.frac, 1);

        // Check above halfway (b'1100001')
        let mut fp = ExtendedFloat80 { frac: 0x61, exp: 0 };
        let (above, halfway) = round_nearest(&mut fp, 6);
        assert!(above);
        assert!(!halfway);
        assert_eq!(fp.frac, 1);

        // Check below halfway (b'1011111')
        let mut fp = ExtendedFloat80 { frac: 0x5F, exp: 0 };
        let (above, halfway) = round_nearest(&mut fp, 6);
        assert!(!above);
        assert!(!halfway);
        assert_eq!(fp.frac, 1);
    }

    #[test]
    fn round_nearest_tie_even_test() {
        // Check round-up, halfway
        let mut fp = ExtendedFloat80 { frac: 0x60, exp: 0 };
        round_nearest_tie_even(&mut fp, 6);
        assert_eq!(fp.frac, 2);

        // Check round-down, halfway
        let mut fp = ExtendedFloat80 { frac: 0x20, exp: 0 };
        round_nearest_tie_even(&mut fp, 6);
        assert_eq!(fp.frac, 0);

        // Check round-up, above halfway
        let mut fp = ExtendedFloat80 { frac: 0x61, exp: 0 };
        round_nearest_tie_even(&mut fp, 6);
        assert_eq!(fp.frac, 2);

        let mut fp = ExtendedFloat80 { frac: 0x21, exp: 0 };
        round_nearest_tie_even(&mut fp, 6);
        assert_eq!(fp.frac, 1);

        // Check round-down, below halfway
        let mut fp = ExtendedFloat80 { frac: 0x5F, exp: 0 };
        round_nearest_tie_even(&mut fp, 6);
        assert_eq!(fp.frac, 1);

        let mut fp = ExtendedFloat80 { frac: 0x1F, exp: 0 };
        round_nearest_tie_even(&mut fp, 6);
        assert_eq!(fp.frac, 0);
    }

    #[test]
    fn round_nearest_tie_away_zero_test() {
        // Check round-up, halfway
        let mut fp = ExtendedFloat80 { frac: 0x60, exp: 0 };
        round_nearest_tie_away_zero(&mut fp, 6);
        assert_eq!(fp.frac, 2);

        let mut fp = ExtendedFloat80 { frac: 0x20, exp: 0 };
        round_nearest_tie_away_zero(&mut fp, 6);
        assert_eq!(fp.frac, 1);

        // Check round-up, above halfway
        let mut fp = ExtendedFloat80 { frac: 0x61, exp: 0 };
        round_nearest_tie_away_zero(&mut fp, 6);
        assert_eq!(fp.frac, 2);

        let mut fp = ExtendedFloat80 { frac: 0x21, exp: 0 };
        round_nearest_tie_away_zero(&mut fp, 6);
        assert_eq!(fp.frac, 1);

        // Check round-down, below halfway
        let mut fp = ExtendedFloat80 { frac: 0x5F, exp: 0 };
        round_nearest_tie_away_zero(&mut fp, 6);
        assert_eq!(fp.frac, 1);

        let mut fp = ExtendedFloat80 { frac: 0x1F, exp: 0 };
        round_nearest_tie_away_zero(&mut fp, 6);
        assert_eq!(fp.frac, 0);
    }

    // DIRECTED ROUNDING

    #[test]
    fn round_toward_infinity_test() {
        // b0000000
        let mut fp = ExtendedFloat80 { frac: 0x00, exp: 0 };
        round_toward_infinity(&mut fp, 6);
        assert_eq!(fp.frac, 0);

        // b1000000
        let mut fp = ExtendedFloat80 { frac: 0x40, exp: 0 };
        round_toward_infinity(&mut fp, 6);
        assert_eq!(fp.frac, 1);

        // b1100000
        let mut fp = ExtendedFloat80 { frac: 0x60, exp: 0 };
        round_toward_infinity(&mut fp, 6);
        assert_eq!(fp.frac, 2);

        // b1110000
        let mut fp = ExtendedFloat80 { frac: 0x70, exp: 0 };
        round_toward_infinity(&mut fp, 6);
        assert_eq!(fp.frac, 2);
    }

    #[test]
    fn round_toward_zero_test() {
        // b0000000
        let mut fp = ExtendedFloat80 { frac: 0x00, exp: 0 };
        round_toward_zero(&mut fp, 6);
        assert_eq!(fp.frac, 0);

        // b1000000
        let mut fp = ExtendedFloat80 { frac: 0x40, exp: 0 };
        round_toward_zero(&mut fp, 6);
        assert_eq!(fp.frac, 1);

        // b1100000
        let mut fp = ExtendedFloat80 { frac: 0x60, exp: 0 };
        round_toward_zero(&mut fp, 6);
        assert_eq!(fp.frac, 1);

        // b1110000
        let mut fp = ExtendedFloat80 { frac: 0x70, exp: 0 };
        round_toward_zero(&mut fp, 6);
        assert_eq!(fp.frac, 1);
    }

    // HIGH-LEVEL

    #[test]
    fn round_to_float_test() {
        // Denormal
        let mut fp = ExtendedFloat80 { frac: 1<<63, exp: f64::DENORMAL_EXPONENT - 15 };
        round_to_float::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 1<<48);
        assert_eq!(fp.exp, f64::DENORMAL_EXPONENT);

        // Halfway, round-down (b'1000000000000000000000000000000000000000000000000000010000000000')
        let mut fp = ExtendedFloat80 { frac: 0x8000000000000400, exp: -63 };
        round_to_float::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 1<<52);
        assert_eq!(fp.exp, -52);

        // Halfway, round-up (b'1000000000000000000000000000000000000000000000000000110000000000')
        let mut fp = ExtendedFloat80 { frac: 0x8000000000000C00, exp: -63 };
        round_to_float::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, (1<<52) + 2);
        assert_eq!(fp.exp, -52);

        // Above halfway
        let mut fp = ExtendedFloat80 { frac: 0x8000000000000401, exp: -63 };
        round_to_float::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, (1<<52)+1);
        assert_eq!(fp.exp, -52);

        let mut fp = ExtendedFloat80 { frac: 0x8000000000000C01, exp: -63 };
        round_to_float::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, (1<<52) + 2);
        assert_eq!(fp.exp, -52);

        // Below halfway
        let mut fp = ExtendedFloat80 { frac: 0x80000000000003FF, exp: -63 };
        round_to_float::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 1<<52);
        assert_eq!(fp.exp, -52);

        let mut fp = ExtendedFloat80 { frac: 0x8000000000000BFF, exp: -63 };
        round_to_float::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, (1<<52) + 1);
        assert_eq!(fp.exp, -52);
    }

    #[test]
    fn avoid_overflow_test() {
        // Avoid overflow, fails by 1
        let mut fp = ExtendedFloat80 { frac: 0xFFFFFFFFFFFF, exp: f64::MAX_EXPONENT + 5 };
        avoid_overflow::<f64, _>(&mut fp);
        assert_eq!(fp.frac, 0xFFFFFFFFFFFF);
        assert_eq!(fp.exp, f64::MAX_EXPONENT+5);

        // Avoid overflow, succeeds
        let mut fp = ExtendedFloat80 { frac: 0xFFFFFFFFFFFF, exp: f64::MAX_EXPONENT + 4 };
        avoid_overflow::<f64, _>(&mut fp);
        assert_eq!(fp.frac, 0x1FFFFFFFFFFFE0);
        assert_eq!(fp.exp, f64::MAX_EXPONENT-1);
    }

    #[test]
    fn round_to_native_test() {
        // Overflow
        let mut fp = ExtendedFloat80 { frac: 0xFFFFFFFFFFFF, exp: f64::MAX_EXPONENT + 4 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 0x1FFFFFFFFFFFE0);
        assert_eq!(fp.exp, f64::MAX_EXPONENT-1);

        // Need denormal
        let mut fp = ExtendedFloat80 { frac: 1, exp: f64::DENORMAL_EXPONENT +48 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 1<<48);
        assert_eq!(fp.exp, f64::DENORMAL_EXPONENT);

        // Halfway, round-down (b'10000000000000000000000000000000000000000000000000000100000')
        let mut fp = ExtendedFloat80 { frac: 0x400000000000020, exp: -58 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 1<<52);
        assert_eq!(fp.exp, -52);

        // Halfway, round-up (b'10000000000000000000000000000000000000000000000000001100000')
        let mut fp = ExtendedFloat80 { frac: 0x400000000000060, exp: -58 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, (1<<52) + 2);
        assert_eq!(fp.exp, -52);

        // Above halfway
        let mut fp = ExtendedFloat80 { frac: 0x400000000000021, exp: -58 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, (1<<52)+1);
        assert_eq!(fp.exp, -52);

        let mut fp = ExtendedFloat80 { frac: 0x400000000000061, exp: -58 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, (1<<52) + 2);
        assert_eq!(fp.exp, -52);

        // Below halfway
        let mut fp = ExtendedFloat80 { frac: 0x40000000000001F, exp: -58 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 1<<52);
        assert_eq!(fp.exp, -52);

        let mut fp = ExtendedFloat80 { frac: 0x40000000000005F, exp: -58 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, (1<<52) + 1);
        assert_eq!(fp.exp, -52);

        // Underflow
        // Adapted from failures in strtod.
        let mut fp = ExtendedFloat80 { exp: -1139, frac: 18446744073709550712 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 0);
        assert_eq!(fp.exp, 0);

        let mut fp = ExtendedFloat80 { exp: -1139, frac: 18446744073709551460 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 0);
        assert_eq!(fp.exp, 0);

        let mut fp = ExtendedFloat80 { exp: -1138, frac: 9223372036854776103 };
        round_to_native::<f64, _, _>(&mut fp, round_nearest_tie_even);
        assert_eq!(fp.frac, 1);
        assert_eq!(fp.exp, -1074);
    }
}
