//! Dispatcher for the non-power-of-two algorithms.

use crate::util::*;

use super::alias::*;
use super::bhcomp;
use super::extended_float;
use super::fast::fast_path;
use super::format::*;
use super::incorrect;
use super::mantissa::*;

#[cfg(feature = "lemire")]
use super::lemire;

// TO NATIVE
// ---------

/// Shallow wrapper for the specialized moderate paths.
#[inline(always)]
fn moderate_path<F>(
    mantissa: F::MantissaType,
    radix: u32,
    exponent: i32,
    is_truncated: bool,
    is_lossy: bool,
    kind: RoundingKind,
) -> (F, bool)
where
    F: FloatType,
{
    cfg_if! {
    if #[cfg(feature = "lemire")] {
        if cfg!(feature = "radix") {
            // Only use lamire if 64-bit mantissa (evaluated at compile-time)
            // and if the radix is 10.
            if radix == 10 && F::MantissaType::BITS <= 64 {
                lemire::moderate_path::<F>(mantissa, radix, exponent, is_truncated, is_lossy, kind)
            } else {
                extended_float::moderate_path::<F>(mantissa, radix, exponent, is_truncated, is_lossy, kind)
            }
        } else if cfg!(feature = "f128") {
            // Only use the lamire algorithm if we have a 64-bit mantissa.
            if F::MantissaType::BITS <= 64 {
                return lemire::moderate_path::<F>(mantissa, radix, exponent, is_truncated, is_lossy, kind);
            } else {
                return extended_float::moderate_path::<F>(mantissa, radix, exponent, is_truncated, is_lossy, kind);
            }
        } else {
            // Cannot support 128-bit floats or non-base 10 radixes here.
            // Always use lamire.
            return lemire::moderate_path::<F>(mantissa, radix, exponent, is_truncated, is_lossy, kind);
        }
    } else {
        extended_float::moderate_path::<F>(mantissa, radix, exponent, is_truncated, is_lossy, kind)
    }} // cfg_if
}

/// Fallback method. Do not inline for performance reasons.
fn fallback<'a, F, Data>(
    data: Data,
    mantissa: F::MantissaType,
    radix: u32,
    is_lossy: bool,
    sign: Sign,
    rounding: RoundingKind,
) -> F
where
    F: FloatType,
    Data: SlowDataInterface<'a>,
{
    let kind = internal_rounding(rounding, sign);

    // Moderate path (use an extended 80-bit representation).
    let exponent = data.mantissa_exponent();
    let is_truncated = data.truncated_digits() != 0;
    let (float, valid) =
        moderate_path::<F>(mantissa, radix, exponent, is_truncated, is_lossy, kind);

    // Check if we can return early, or use slow-path.
    if valid || float.is_special() {
        float
    } else {
        bhcomp::atof(data, radix, float, kind)
    }
}

/// Parse non-power-of-two radix string to native float.
pub(crate) fn to_native<'a, F, Data>(
    mut data: Data,
    bytes: &'a [u8],
    radix: u32,
    is_incorrect: bool,
    is_lossy: bool,
    sign: Sign,
    rounding: RoundingKind,
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    // Parse the mantissa and exponent.
    let ptr = data.extract(bytes, radix)?;
    let (mantissa, truncated) = process_mantissa::<F::MantissaType, _>(&data, radix);

    // Process the state to a float.
    let float = if mantissa.is_zero() {
        // Literal 0, return early.
        // Value cannot be truncated, since truncation only occurs on
        // overflow or underflow.
        F::ZERO
    } else if truncated.is_zero() {
        // Try the fast path, no mantissa truncation.
        let mant_exp = data.mantissa_exponent(0);
        if let Some(float) = fast_path::<F>(mantissa, radix, mant_exp) {
            float
        } else if is_incorrect {
            incorrect::to_native::<F, _>(data, radix)
        } else {
            let slow = data.to_slow(truncated);
            fallback(slow, mantissa, radix, is_lossy, sign, rounding)
        }
    } else if is_incorrect {
        incorrect::to_native::<F, _>(data, radix)
    } else {
        // Can only use the moderate/slow path.
        let slow = data.to_slow(truncated);
        fallback(slow, mantissa, radix, is_lossy, sign, rounding)
    };
    Ok((float, ptr))
}
