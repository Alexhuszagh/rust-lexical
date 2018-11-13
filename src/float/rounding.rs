//! Defines rounding schemes for floating-point numbers.

use util::*;
use super::float_type::FloatType;
use super::shift::{shl, shr};

// GENERIC
// -------

/// Parameters for general rounding operations.
pub(super) struct RoundingParameters {
    /// Bits to truncate from the mantissa.
    mask: u64,
    /// Midway point for truncated bits.
    mid: u64,
    /// Number of bits to shift
    shift: i32,
}

// ROUND NEAREST TIE EVEN

/// Shift right N-bytes and round to the nearest.
///
/// Return if we are above halfway and if we are halfway.
#[inline]
pub(super) fn round_nearest(fp: &mut FloatType, params: &RoundingParameters)
    -> (bool, bool)
{
    // Extract the truncated bits using mask.
    // Calculate if the value of the truncated bits are either above
    // the mid-way point, or equal to it.
    //
    // For example, for 4 truncated bytes, the mask would be b1111
    // and the midway point would be b1000.
    let truncated_bits = fp.frac & params.mask;
    let is_above = truncated_bits > params.mid;
    let is_halfway = truncated_bits == params.mid;

    // Bit shift so the leading bit is in the hidden bit.
    shr(fp, params.shift);

    (is_above, is_halfway)
}

/// Shift right N-bytes and round nearest, tie-to-even.
///
/// Floating-point arithmetic uses round to nearest, ties to even,
/// which rounds to the nearest value, if the value is halfway in between,
/// round to an even value.
#[inline]
pub(super) fn round_nearest_tie_even(fp: &mut FloatType, params: &RoundingParameters)
{
    let (is_above, is_halfway) = round_nearest(fp, params);

    // Extract the last bit after shifting (and determine if it is odd).
    let is_odd = fp.frac & 0x1 == 0x1;

    // Calculate if we need to roundup.
    // We need to roundup if we are above halfway, or if we are odd
    // and at half-way (need to tie-to-even).
    let is_roundup = is_above || (is_odd && is_halfway);

    // Roundup as needed.
    fp.frac += is_roundup as u64;
}

/// Shift right N-bytes and round nearest, tie-to-even.
///
/// Floating-point arithmetic uses round to nearest, ties to even,
/// which rounds to the nearest value, if the value is halfway in between,
/// round to an even value.
#[inline]
#[allow(dead_code)]
pub(super) fn round_nearest_tie_away_zero(fp: &mut FloatType, params: &RoundingParameters)
{
    let (is_above, is_halfway) = round_nearest(fp, params);

    // Calculate if we need to roundup.
    // We need to roundup if we are halfway or above halfway,
    // since the value is always positive and we need to round away
    // from zero.
    let is_roundup = is_above || is_halfway;

    // Roundup as needed.
    fp.frac += is_roundup as u64;
}

// NATIVE FLOAT
// ------------

// FLOAT ROUNDING

/// Trait to round extended-precision floats to native representations.
pub(super) trait FloatRounding: Float {
    /// Bits to truncate from the mantissa.
    const TRUNCATE_MASK: u64;
    /// Midway point for truncated bits.
    const TRUNCATE_MID: u64;
    /// Number of bits to shift (or 64 - mantissa size - 1).
    const TRUNCATE_SHIFT: i32;
    /// Mask to determine if a full-carry occurred (1 in bit above hidden bit).
    const CARRY_MASK: u64;
    /// Mask from the hidden bit to the right, to see if we can prevent overflow.]
    const OVERFLOW_MASK: &'static [u64];
    /// Rounding parameters to convert to native float.
    const ROUNDING_PARAMS: RoundingParameters = RoundingParameters {
        mask: Self::TRUNCATE_MASK,
        mid: Self::TRUNCATE_MID,
        shift: Self::TRUNCATE_SHIFT,
    };
}

impl FloatRounding for f32 {
    const TRUNCATE_MASK: u64    = 0xFFFFFFFFFF;
    const TRUNCATE_MID: u64     = 0x8000000000;
    const TRUNCATE_SHIFT: i32   = 40;
    const CARRY_MASK: u64       = 0x1000000;
    const OVERFLOW_MASK: &'static [u64] = &[
        0x00800000, 0x00C00000, 0x00E00000, 0x00F00000, 0x00F80000, 0x00FC0000,
        0x00FE0000, 0x00FF0000, 0x00FF8000, 0x00FFC000, 0x00FFE000, 0x00FFF000,
        0x00FFF800, 0x00FFFC00, 0x00FFFE00, 0x00FFFF00, 0x00FFFF80, 0x00FFFFC0,
        0x00FFFFE0, 0x00FFFFF0, 0x00FFFFF8, 0x00FFFFFC, 0x00FFFFFE, 0x00FFFFFF
    ];
}

impl FloatRounding for f64 {
    const TRUNCATE_MASK: u64    = 0x7FF;
    const TRUNCATE_MID: u64     = 0x400;
    const TRUNCATE_SHIFT: i32   = 11;
    const CARRY_MASK: u64       = 0x20000000000000;
    const OVERFLOW_MASK: &'static [u64] = &[
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
}

// ROUND TO FLOAT

/// Shift the FloatType fraction to the fraction bits in a native float.
///
/// Floating-point arithmetic uses round to nearest, ties to even,
/// which rounds to the nearest value, if the value is halfway in between,
/// round to an even value.
#[inline]
pub(super) fn round_to_float<T: FloatRounding>(fp: &mut FloatType) {
    round_nearest_tie_even(fp, &T::ROUNDING_PARAMS);
    if fp.frac & T::CARRY_MASK == T::CARRY_MASK {
        // Roundup carried over to 1 past the hidden bit.
        shr(fp, 1);
    }
}

// AVOID OVERFLOW/UNDERFLOW

/// Avoid underflow for denormalized values.
///
/// Shift if the shift results in a non-zero mantissa and an exponent
/// >= denormal exponent.
#[inline]
pub(super) fn avoid_underflow<T: FloatRounding>(fp: &mut FloatType) {
    // Calculate the difference to allow a single calculation
    // rather than a loop, to minimize the number of ops required.
    if fp.exp < T::DENORMAL_EXPONENT {
        let diff = T::DENORMAL_EXPONENT - fp.exp;
        if fp.frac >> diff != 0 {
            fp.frac >>= diff;
            fp.exp += diff;
        }
    }
}

/// Avoid overflow for large values, shift left as needed.
///
/// Shift until a 1-bit is in the hidden bit, if the mantissa is not 0.
#[inline]
pub(super) fn avoid_overflow<T: FloatRounding>(fp: &mut FloatType) {
    // Calculate the difference to allow a single calculation
    // rather than a loop, using a precalculated bitmask table,
    // minimizing the number of ops required.
    if fp.exp >= T::MAX_EXPONENT {
        let diff = fp.exp - T::MAX_EXPONENT;
        let idx = diff as usize;
        if let Some(mask) = T::OVERFLOW_MASK.get(idx) {
            if fp.frac & mask == 0 {
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
pub(super) fn round_to_native<T: FloatRounding>(fp: &mut FloatType) {
    // Shift all the way left, to ensure a consistent representation.
    // The following right-shifts do not work for a non-normalized number.
    fp.normalize();

    // Round so the fraction is in a native mantissa representation,
    // and avoid overflow/underflow.
    round_to_float::<T>(fp);
    avoid_underflow::<T>(fp);
    avoid_overflow::<T>(fp)
}
