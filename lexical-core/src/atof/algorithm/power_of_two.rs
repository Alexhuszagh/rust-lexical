//! Correct, fast algorithms for conversions of power-of-two radix floats.

#![cfg(feature = "power_of_two")]

use crate::util::*;

use super::alias::*;
use super::format::*;
use super::mantissa::*;

// FAST PATH
// ---------

/// Convert power-of-two to exact value.
///
/// We will always get an exact representation.
///
/// This works since multiplying by the exponent will not affect the
/// mantissa unless the exponent is denormal, which will cause truncation
/// regardless.
fn fast_path<F, M>(mantissa: M, radix: u32, radix_log2: i32, exponent: i32) -> F
where
    F: FloatType,
    M: MantissaType,
{
    debug_assert!(radix_log2 != 0, "Not a power of 2.");

    // As long as the value is within the bounds, we can get an exact value.
    // Since any power of 2 only affects the exponent, we should be able to get
    // any exact value.

    // We know that if any value is > than max_exp, we get infinity, since
    // the mantissa must be positive. We know that the actual value that
    // causes underflow is 64, use 65 since that prevents inaccurate
    // rounding for any log2(radix).
    let (min_exp, max_exp) = F::exponent_limit(radix);
    let underflow_exp = min_exp - (65 / radix_log2);
    if exponent > max_exp {
        F::INFINITY
    } else if exponent < underflow_exp {
        F::ZERO
    } else if exponent < min_exp {
        // We know the mantissa is somewhere <= 65 below min_exp.
        // May still underflow, but it's close. Use the first multiplication
        // which guarantees no truncation, and then the second multiplication
        // which will round to the accurate representation.
        let remainder = exponent - min_exp;
        let float: F = as_cast(mantissa);
        let float = float.pow2(radix_log2 * remainder).pow2(radix_log2 * min_exp);
        float
    } else {
        let float: F = as_cast(mantissa);
        let float = float.pow2(radix_log2 * exponent);
        float
    }
}

// MODERATE PATH
// -------------

/// Extended-precision float for when the when the mantissa cannot fit.
///
/// This requires no truncated bits, but a mantissa that is greater
/// than F::MANTISSA_SIZE.
#[inline(always)]
fn moderate_path<'a, F, M, Data>(
    data: Data,
    mantissa: M,
    truncated: usize,
    radix_log2: i32,
    sign: Sign,
    rounding: RoundingKind,
) -> F
where
    F: FloatType,
    M: MantissaType,
    Data: FastDataInterface<'a>,
{
    let kind = internal_rounding(rounding, sign);
    let slow = data.to_slow(truncated);
    let exponent = slow.mantissa_exponent().saturating_mul(radix_log2);
    let fp = ExtendedFloat {
        mant: mantissa,
        exp: exponent,
    };
    fp.into_rounded_float_impl::<F>(kind)
}

// ROUNDING
// --------

/// Round upward if the value is above the current representation.
#[inline(always)]
#[cfg(feature = "rounding")]
fn round_upward<M>(mut mantissa: M, is_truncated: bool) -> M
where
    M: MantissaType,
{
    if is_truncated {
        mantissa += M::ONE;
    }
    mantissa
}

// SLOW PATH
// ---------

/// Slow path for when we need to determine the rounding.
///
/// Have a truncated mantissa, need to solve halfway rounding cases.
#[inline(always)]
#[allow(unused_variables, unused_mut)]
fn slow_path<'a, F, Data>(
    data: Data,
    mut mantissa: F::MantissaType,
    truncated: usize,
    radix_log2: i32,
    sign: Sign,
    rounding: RoundingKind,
) -> F
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    // Truncated mantissa.
    let kind = internal_rounding(rounding, sign);
    let slow = data.to_slow(truncated);

    #[cfg(feature = "rounding")]
    {
        // Only need to check if we round-upward, to see if we have any
        // truncation. Only add a single bit so then the impl rounding
        // handles everything.
        if kind == RoundingKind::Upward {
            // Need to check if there are any bytes present.
            // Check if there were any truncated bytes.
            let index = slow.mantissa_digits() - slow.truncated_digits();
            let iter = slow.integer_iter().chain(slow.fraction_iter()).skip(index);
            let count = iter.take_while(|&&c| c == b'0').count();
            let is_truncated = count < slow.truncated_digits();
            mantissa = round_upward(mantissa, is_truncated)
        }
    }

    // Create exact representation and return.
    let exponent = slow.mantissa_exponent().saturating_mul(radix_log2);
    let fp = ExtendedFloat {
        mant: mantissa,
        exp: exponent,
    };
    fp.into_rounded_float_impl::<F>(kind)
}

// ALGORITHM
// ---------

/// Parse power-of-two radix string to native float.
pub(crate) fn to_native<'a, F, Data>(
    mut data: Data,
    bytes: &'a [u8],
    radix: u32,
    radix_log2: i32,
    sign: Sign,
    rounding: RoundingKind,
) -> ParseTupleResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    // Parse the mantissa and exponent.
    let ptr = data.extract(bytes, radix)?;
    let (mantissa, truncated) = process_mantissa::<F::MantissaType, _>(&data, radix);

    // We have a power of 2, can get an exact value even if the mantissa
    // was truncated. Check to see if there are any truncated digits, depending
    // on our rounding scheme.
    let mantissa_size = F::MANTISSA_SIZE + 1;
    let float = if !truncated.is_zero() {
        slow_path(data, mantissa, truncated, radix_log2, sign, rounding)
    } else if mantissa >> mantissa_size != F::MantissaType::ZERO {
        // Would be truncated, use the extended float.
        moderate_path(data, mantissa, truncated, radix_log2, sign, rounding)
    } else {
        // Nothing above the hidden bit, so no rounding-error, can use the fast path.
        let mant_exp = data.mantissa_exponent(0);
        fast_path(mantissa, radix, radix_log2, mant_exp)
    };
    Ok((float, ptr))
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_fast_path() {
        // Everything is valid.
        let mantissa = 1 << 63;
        for base in BASE_POW2.iter().cloned() {
            let (min_exp, max_exp) = f32::exponent_limit(base);
            let pow2_exp = log2(base);
            for exp in min_exp - 20..max_exp + 30 {
                // Always valid, ignore result
                fast_path::<f32, _>(mantissa, base, pow2_exp, exp);
            }
        }
    }

    #[test]
    fn double_fast_path_test() {
        // Everything is valid.
        let mantissa = 1 << 63;
        for base in BASE_POW2.iter().cloned() {
            let (min_exp, max_exp) = f64::exponent_limit(base);
            let pow2_exp = log2(base);
            for exp in min_exp - 20..max_exp + 30 {
                // Ignore result, always valid
                fast_path::<f64, _>(mantissa, base, pow2_exp, exp);
            }
        }
    }
}
