use crate::util::*;

use super::common::*;
use super::float::*;
use super::options::*;

#[inline]
pub fn compute_float<F>(q: i64, w: F::Mantissa, options: &Options)
    -> AdjustedMantissa<F>
where
    F: FloatType,
{
    debug_assert!(F::Unsigned::BITS <= 64, "Cannot use with f128");

    let mut w = w.as_u64();
    if w == 0 || q < options.smallest_power as i64 {
        return AdjustedMantissa::zero();
    } else if q > options.largest_power as i64 {
        return AdjustedMantissa::inf();
    }

    let ctlz = w.leading_zeros();
    w <<= ctlz;
    let (lo, hi) = compute_product_approx::<F>(q, w, F::MANTISSA_SIZE as usize + 3, options);
    if lo == 0xFFFF_FFFF_FFFF_FFFF {
        let inside_safe_exponent = (q >= options.min_safe_exponent) && (q <= options.max_safe_exponent);
        if !inside_safe_exponent {
            return AdjustedMantissa::error();
        }
    }
    let upperbit = (hi >> 63) as i32;
    let mut mantissa = hi >> (upperbit + 64 - F::MANTISSA_SIZE - 3);
    let min_exp = -(F::EXPONENT_BIAS - F::MANTISSA_SIZE);
    let mut power2 = F::power(q as i32, options) + upperbit - ctlz as i32 - min_exp;
    if power2 <= 0 {
        if -power2 + 1 >= 64 {
            return AdjustedMantissa::zero();
        }
        mantissa >>= -power2 + 1;
        mantissa += mantissa & 1;
        mantissa >>= 1;
        power2 = (mantissa >= (1_u64 << F::MANTISSA_SIZE)) as i32;
        return AdjustedMantissa {
            mantissa: as_cast(mantissa),
            power2
        };
    }
    if lo <= 1
        && q >= F::min_exponent_round_to_even(options) as i64
        && q <= F::max_exponent_round_to_even(options) as i64
        && mantissa & 3 == 1
        && (mantissa << (upperbit + 64 - F::MANTISSA_SIZE - 3)) == hi
    {
        mantissa &= !1_u64;
    }
    mantissa += mantissa & 1;
    mantissa >>= 1;
    if mantissa >= (2_u64 << F::MANTISSA_SIZE) {
        mantissa = 1_u64 << F::MANTISSA_SIZE;
        power2 += 1;
    }
    mantissa &= !(1_u64 << F::MANTISSA_SIZE);
    if power2 >= AdjustedMantissa::<F>::inf_exp() {
        return AdjustedMantissa::inf();
    }
    AdjustedMantissa::<F> {
        mantissa: as_cast(mantissa),
        power2
    }
}

/// Multiply two unsigned, integral values, and return the hi and lo product.
#[inline(always)]
pub(crate) fn mul<M: Mantissa>(x: M, y: M) -> (M, M) {
    // Extract high-and-low masks.
    let x1 = x >> M::HALF;
    let x0 = x & M::LOMASK;
    let y1 = y >> M::HALF;
    let y0 = y & M::LOMASK;

    // Get our products
    let w0 = x0 * y0;
    let tmp = (x1 * y0) + (w0 >> M::HALF);
    let w1 = tmp & M::LOMASK;
    let w2 = tmp >> M::HALF;
    let w1 = w1 + x0 * y1;
    let hi = (x1 * y1) + w2 + (w1 >> M::HALF);
    let lo = x.wrapping_mul(y);

    (hi, lo)
}

// This will compute or rather approximate w * 5**q and return a pair of 64-bit words
// approximating the result, with the "high" part corresponding to the most significant
// bits and the low part corresponding to the least significant bits.
#[inline]
fn compute_product_approx<F>(q: i64, w: u64, precision: usize, options: &Options)
    -> (u64, u64)
where
    F: FloatType
{
    debug_assert!(q >= F::min_exp(options) as i64);
    debug_assert!(q <= F::max_exp(options) as i64);
    debug_assert!(precision <= 64);

    let mask = if precision < 64 {
        0xFFFF_FFFF_FFFF_FFFF_u64 >> precision
    } else {
        0xFFFF_FFFF_FFFF_FFFF_u64
    };
    let index = (q - F::min_exp(options) as i64) as usize;
    let (lo, hi) = options.power_of_x_128[index];
    let (mut first_lo, mut first_hi) = mul(w, lo);
    if first_hi & mask == mask {
        let (_, second_hi) = mul(w, hi);
        first_lo = first_lo.wrapping_add(second_hi);
        if second_hi > first_lo {
            first_hi += 1;
        }
    }
    (first_lo, first_hi)
}
