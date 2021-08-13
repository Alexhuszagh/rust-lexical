//! Optimized float parser for radixes powers of 2.
//!
//! Note: this requires the mantissa radix and the
//! exponent base to be the same. See [hex](crate::hex) for
//! when the mantissa radix and the exponent base are different.

#![cfg(feature = "power-of-two")]
#![doc(hidden)]

use crate::float::{ExtendedFloat80, RawFloat};
use crate::mask::lower_n_halfway;
use crate::number::Number;
use lexical_util::format::NumberFormat;
use lexical_util::num::AsPrimitive;

/// Quick log2 that evaluates at compile time for the radix.
const fn log2(radix: u32) -> i32 {
    match radix {
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        _ => 5,
    }
}

/// Algorithm specialized for radixes of powers-of-two.
#[inline]
pub fn binary<F: RawFloat, const FORMAT: u128>(num: &Number, lossy: bool) -> ExtendedFloat80 {
    let format = NumberFormat::<{ FORMAT }> {};
    debug_assert!(matches!(format.radix(), 2 | 4 | 8 | 16 | 32));
    debug_assert!(format.mantissa_radix() == format.exponent_base());

    let fp_zero = ExtendedFloat80 {
        mant: 0,
        exp: 0,
    };
    let fp_inf = ExtendedFloat80 {
        mant: 0,
        exp: F::INFINITE_POWER,
    };
    let fp_error = ExtendedFloat80 {
        mant: 0,
        exp: -1,
    };

    // Normalize our mantissa for simpler results.
    let ctlz = num.mantissa.leading_zeros();
    let mut mantissa = num.mantissa << ctlz;

    // Quick check if we're close to a halfway point.
    // Since we're using powers-of-two, we can clearly tell if we're at
    // a halfway point, unless it's even and we're exactly halfway so far.
    // This is true even for radixes like 8 and 32, where `log2(radix)`
    // is not a power-of-two. If it's odd and we're at halfway, we'll
    // always round-up **anyway**.
    //
    // We need to check the truncated bits are equal to 0b100000....,
    // if it's above that, always round-up. If it's odd, we can always
    // disambiguate the float. If it's even, and exactly halfway, this
    // step fails.
    let mut power2 = num.exponent as i32 * log2(format.radix()) + F::EXPONENT_BIAS - ctlz as i32;
    if -power2 + 1 >= 64 {
        // Have more than 63 bits below the minimum exponent, must be 0.
        // Since we can't have partial digit rounding, this is true always
        // if the power-of-two >= 64.
        return fp_zero;
    }

    // Get our shift to shift the digits to the hidden bit, or correct spot.
    // This differs for denormal floats, so do that carefully, but that's
    // relative to the current leading zeros of the float.
    let mantissa_shift = 64 - F::MANTISSA_SIZE - 1;
    let shift = if -power2 >= mantissa_shift {
        -power2 + 1
    } else {
        mantissa_shift
    };

    // Determine if we can see if we're at a halfway point.
    let last_bit = 1u64 << shift;
    let truncated = last_bit - 1;
    let halfway = lower_n_halfway(shift as u64);
    let is_even = mantissa & last_bit == 0;
    let is_halfway = mantissa & truncated == halfway;
    if !lossy && is_even && is_halfway && num.many_digits {
        // Exactly halfway and even, cannot safely determine our representation.
        return fp_error;
    }

    // Shift our digits into place, and round up if needed.
    let is_above = mantissa & truncated > halfway;
    let round_up = is_above || (!is_even && is_halfway);
    mantissa >>= shift;
    power2 += shift;
    mantissa += round_up as u64;

    // Check if we carried, and if so, shift the bit to the hidden bit.
    let carry_mask = F::CARRY_MASK.as_u64();
    if mantissa & carry_mask == carry_mask {
        mantissa >>= 1;
        power2 += 1;
    }

    // Check for overflow.
    if power2 >= F::INFINITE_POWER {
        // Exponent is above largest normal value, must be infinite.
        return fp_inf;
    }

    // Remove the hidden bit.
    mantissa &= F::MANTISSA_MASK.as_u64();
    ExtendedFloat80 {
        mant: mantissa,
        exp: power2,
    }
}
