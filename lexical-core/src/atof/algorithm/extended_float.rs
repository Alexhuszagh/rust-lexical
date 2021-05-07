//! Estimate the error in an 80-bit approximation of a float.
//!
//! This estimates the error in a floating-point representation.
//!
//! This implementation is loosely based off the Golang implementation,
//! found here:
//!     https://github.com/golang/go/blob/b10849fbb97a2244c086991b4623ae9f32c212d0/src/strconv/extfloat.go

use crate::util::*;

use super::alias::*;

// FLOAT ERRORS
// ------------

/// Check if the error is accurate with a round-nearest rounding scheme.
#[inline]
fn nearest_error_is_accurate<M>(errors: u32, fp: &ExtendedFloat<M>, extrabits: i32) -> bool
where
    M: MantissaType,
{
    let full = M::FULL;
    let maskbits: M = as_cast(extrabits);
    let errors: M = as_cast(errors);

    // Round-to-nearest, need to use the halfway point.
    if extrabits >= full + 1 {
        // Underflow, we have a shift larger than the mantissa.
        // Representation is valid **only** if the value is close enough
        // overflow to the next bit within errors. If it overflows,
        // the representation is **not** valid.
        !fp.mant.overflowing_add(errors).1
    } else {
        let mask = lower_n_mask(maskbits);
        let extra = fp.mant & mask;

        // Round-to-nearest, need to check if we're close to halfway.
        // IE, b10100 | 100000, where `|` signifies the truncation point.
        let halfway = lower_n_halfway(maskbits);
        let cmp1 = halfway.wrapping_sub(errors) < extra;
        let cmp2 = extra < halfway.wrapping_add(errors);

        // If both comparisons are true, we have significant rounding error,
        // and the value cannot be exactly represented. Otherwise, the
        // representation is valid.
        !(cmp1 && cmp2)
    }
}

/// Check if the error is accurate with a round-toward rounding scheme.
#[inline]
#[cfg(feature = "rounding")]
fn toward_error_is_accurate<M>(errors: u32, fp: &ExtendedFloat<M>, extrabits: i32) -> bool
where
    M: MantissaType,
{
    let full = M::FULL;
    let maskbits: M = as_cast(extrabits);
    let errors: M = as_cast(errors);

    if extrabits >= full + 1 {
        // Underflow, we have a literal 0.
        true
    } else {
        let mask: M = lower_n_mask(maskbits);
        let extra: M = fp.mant & mask;

        // Round-towards, need to use `1 << extrabits`.
        if extrabits == full {
            // Round toward something, we need to check if either operation can overflow,
            // since we cannot exactly represent the comparison point as the type
            // in question.
            let cmp1 = extra.checked_sub(errors).is_none();
            let cmp2 = extra.checked_add(errors).is_none();
            // If either comparison is true, we have significant rounding error,
            // since we cannot distinguish the value (1 << 64).
            cmp1 || cmp2
        } else {
            // Round toward something, need to check if we're close to
            // IE, b10101 | 000000, where `|` signifies the truncation point.
            // If the extract bits +/- the error can overflow, then  we have
            // an issue.
            let fullway: M = nth_bit(maskbits);
            let cmp1 = fullway.wrapping_sub(errors) < extra;
            let cmp2 = extra < fullway.wrapping_add(errors);

            // If both comparisons are true, we have significant rounding error,
            // and the value cannot be exactly represented. Otherwise, the
            // representation is valid.
            !(cmp1 && cmp2)
        }
    }
}

// Calculate if the errors in calculating the extended-precision float.
//
// Specifically, we want to know if we are close to a halfway representation,
// or halfway between `b` and `b+1`, or `b+h`. The halfway representation
// has the form:
//     SEEEEEEEHMMMMMMMMMMMMMMMMMMMMMMM100...
// where:
//     S = Sign Bit
//     E = Exponent Bits
//     H = Hidden Bit
//     M = Mantissa Bits
//
// The halfway representation has a bit set 1-after the mantissa digits,
// and no bits set immediately afterward, making it impossible to
// round between `b` and `b+1` with this representation.

/// Get the full error scale.
#[inline(always)]
fn error_scale() -> u32 {
    8
}

/// Get the half error scale.
#[inline]
fn error_halfscale() -> u32 {
    error_scale() / 2
}

/// Determine if the number of errors is tolerable for float precision.
#[inline]
#[allow(unused_variables)]
fn error_is_accurate<F: FloatType, M: MantissaType>(
    errors: u32,
    fp: &ExtendedFloat<M>,
    kind: RoundingKind,
) -> bool {
    // Determine if extended-precision float is a good approximation.
    // If the error has affected too many units, the float will be
    // inaccurate, or if the representation is too close to halfway
    // that any operations could affect this halfway representation.
    // See the documentation for dtoa for more information.
    let full = M::FULL;
    let nonsign_bits = full - 1;
    let bias = -(F::EXPONENT_BIAS - F::MANTISSA_SIZE);
    let denormal_exp = bias - nonsign_bits;
    // This is always a valid u32, since (denormal_exp - fp.exp)
    // will always be positive and the significand size is {23, 52}.
    let extrabits = match fp.exp <= denormal_exp {
        true => full - F::MANTISSA_SIZE + denormal_exp - fp.exp,
        false => nonsign_bits - F::MANTISSA_SIZE,
    };

    // Our logic is as follows: we want to determine if the actual
    // mantissa and the errors during calculation differ significantly
    // from the rounding point. The rounding point for round-nearest
    // is the halfway point, IE, this when the truncated bits start
    // with b1000..., while the rounding point for the round-toward
    // is when the truncated bits are equal to 0.
    // To do so, we can check whether the rounding point +/- the error
    // are >/< the actual lower n bits.
    //
    // For whether we need to use signed or unsigned types for this
    // analysis, see this example, using u8 rather than u64 to simplify
    // things.
    //
    // # Comparisons
    //      cmp1 = (halfway - errors) < extra
    //      cmp1 = extra < (halfway + errors)
    //
    // # Large Extrabits, Low Errors
    //
    //      extrabits = 8
    //      halfway          =  0b10000000
    //      extra            =  0b10000010
    //      errors           =  0b00000100
    //      halfway - errors =  0b01111100
    //      halfway + errors =  0b10000100
    //
    //      Unsigned:
    //          halfway - errors = 124
    //          halfway + errors = 132
    //          extra            = 130
    //          cmp1             = true
    //          cmp2             = true
    //      Signed:
    //          halfway - errors = 124
    //          halfway + errors = -124
    //          extra            = -126
    //          cmp1             = false
    //          cmp2             = true
    //
    // # Conclusion
    //
    // Since errors will always be small, and since we want to detect
    // if the representation is accurate, we need to use an **unsigned**
    // type for comparisons.

    #[cfg(not(feature = "rounding"))]
    {
        nearest_error_is_accurate::<M>(errors, fp, extrabits)
    }

    #[cfg(feature = "rounding")]
    {
        if kind.is_nearest() {
            nearest_error_is_accurate::<M>(errors, fp, extrabits)
        } else {
            toward_error_is_accurate::<M>(errors, fp, extrabits)
        }
    }
}

// MODERATE PATH
// -------------

/// Multiply the floating-point by the exponent.
///
/// Multiply by pre-calculated powers of the base, modify the extended-
/// float, and return if new value and if the value can be represented
/// accurately.
pub(crate) fn multiply_exponent_extended<F, M>(
    fp: &mut ExtendedFloat<M>,
    radix: u32,
    exponent: i32,
    truncated: bool,
    kind: RoundingKind,
) -> bool
where
    M: MantissaType,
    F: FloatType,
{
    let powers = M::get_powers(radix);
    let exponent = exponent.saturating_add(powers.bias);
    let small_index = exponent % powers.step;
    let large_index = exponent / powers.step;
    if exponent < 0 {
        // Guaranteed underflow (assign 0).
        fp.mant = M::ZERO;
        true
    } else if large_index as usize >= powers.large.len() {
        // Overflow (assign infinity)
        fp.mant = M::ONE << (M::FULL - 1);
        fp.exp = M::MAX_EXPONENT;
        true
    } else {
        // Within the valid exponent range, multiply by the large and small
        // exponents and return the resulting value.

        // Track errors to as a factor of unit in last-precision.
        let mut errors: u32 = 0;
        if truncated {
            errors += error_halfscale();
        }

        // Multiply by the small power.
        // Check if we can directly multiply by an integer, if not,
        // use extended-precision multiplication.
        match fp.mant.overflowing_mul(powers.get_small_int(small_index as usize)) {
            // Overflow, multiplication unsuccessful, go slow path.
            (_, true) => {
                fp.normalize();
                fp.imul(&powers.get_small(small_index as usize));
                errors += error_halfscale();
            },
            // No overflow, multiplication successful.
            (mant, false) => {
                fp.mant = mant;
                fp.normalize();
            },
        }

        // Multiply by the large power.
        fp.imul(&powers.get_large(large_index as usize));
        if errors > 0 {
            errors += 1;
        }
        errors += error_halfscale();

        // Normalize the floating point (and the errors).
        let shift = fp.normalize();
        errors <<= shift;

        error_is_accurate::<F, _>(errors, &fp, kind)
    }
}

/// Create a precise native float using an intermediate extended-precision float.
///
/// Return the float approximation and if the value can be accurately
/// represented with mantissa bits of precision.
#[inline(always)]
pub(super) fn moderate_path<F>(
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
    let mut fp = ExtendedFloat {
        mant: mantissa,
        exp: 0,
    };
    let valid = multiply_exponent_extended::<F, _>(&mut fp, radix, exponent, is_truncated, kind);
    if valid || is_lossy {
        // Need to check if we have a 64-bit float, actually.
        let float = fp.into_rounded_float_impl::<F>(kind);
        (float, true)
    } else {
        // Need the slow-path algorithm.
        let float = fp.into_rounded_float_impl::<F>(RoundingKind::Downward);
        (float, false)
    }
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halfway_round_down() {
        let radix = 10;
        let kind = RoundingKind::NearestTieEven;

        // Halfway, round-down tests
        assert!(moderate_path::<f64>(9007199254740992u64, radix, 0, false, false, kind).1);
        assert!(!moderate_path::<f64>(9007199254740993u64, radix, 0, false, false, kind).1);
        assert!(moderate_path::<f64>(9007199254740994u64, radix, 0, false, false, kind).1);

        assert!(moderate_path::<f64>(18014398509481984u64, radix, 0, false, false, kind).1);
        assert!(!moderate_path::<f64>(18014398509481986u64, radix, 0, false, false, kind).1);
        assert!(moderate_path::<f64>(18014398509481988u64, radix, 0, false, false, kind).1);

        assert!(moderate_path::<f64>(9223372036854775808u64, radix, 0, false, false, kind).1);
        assert!(!moderate_path::<f64>(9223372036854776832u64, radix, 0, false, false, kind).1);
        assert!(moderate_path::<f64>(9223372036854777856u64, radix, 0, false, false, kind).1);

        // Add a 0 but say we're truncated.
        assert!(moderate_path::<f64>(9007199254740992000u64, radix, -3, true, false, kind).1);
        assert!(!moderate_path::<f64>(9007199254740993000u64, radix, -3, true, false, kind).1);
        assert!(moderate_path::<f64>(9007199254740994000u64, radix, -3, true, false, kind).1);
    }

    #[test]
    fn test_halfway_round_up() {
        let radix = 10;
        let kind = RoundingKind::NearestTieEven;

        // Halfway, round-down tests
        assert!(moderate_path::<f64>(9007199254740994u64, radix, 0, false, false, kind).1);
        assert!(!moderate_path::<f64>(9007199254740995u64, radix, 0, false, false, kind).1);
        assert!(moderate_path::<f64>(9007199254740996u64, radix, 0, false, false, kind).1);

        assert!(moderate_path::<f64>(18014398509481988u64, radix, 0, false, false, kind).1);
        assert!(!moderate_path::<f64>(18014398509481990u64, radix, 0, false, false, kind).1);
        assert!(moderate_path::<f64>(18014398509481992u64, radix, 0, false, false, kind).1);

        assert!(moderate_path::<f64>(9223372036854777856u64, radix, 0, false, false, kind).1);
        assert!(!moderate_path::<f64>(9223372036854778880u64, radix, 0, false, false, kind).1);
        assert!(moderate_path::<f64>(9223372036854779904u64, radix, 0, false, false, kind).1);

        // Add a 0 but say we're truncated.
        assert!(moderate_path::<f64>(9007199254740994000u64, radix, -3, true, false, kind).1);
        assert!(!moderate_path::<f64>(9007199254740994990u64, radix, -3, true, false, kind).1);
        assert!(!moderate_path::<f64>(9007199254740995000u64, radix, -3, true, false, kind).1);
        assert!(!moderate_path::<f64>(9007199254740995010u64, radix, -3, true, false, kind).1);
        assert!(moderate_path::<f64>(9007199254740996000u64, radix, -3, true, false, kind).1);
    }

    #[test]
    #[cfg(feature = "radix")]
    fn float_moderate_path_test() {
        // valid (overflowing small mult)
        let mantissa: u64 = 1 << 63;
        let (f, valid) =
            moderate_path::<f32>(mantissa, 3, 1, false, false, RoundingKind::NearestTieEven);
        assert_eq!(f, 2.7670116e+19);
        assert!(valid, "exponent should be valid");

        let mantissa: u64 = 4746067219335938;
        let (f, valid) =
            moderate_path::<f32>(mantissa, 15, -9, false, false, RoundingKind::NearestTieEven);
        assert_eq!(f, 123456.1);
        assert!(valid, "exponent should be valid");
    }

    #[test]
    #[cfg(feature = "radix")]
    fn double_moderate_path_test() {
        // valid (overflowing small mult)
        let mantissa: u64 = 1 << 63;
        let (f, valid) =
            moderate_path::<f64>(mantissa, 3, 1, false, false, RoundingKind::NearestTieEven);
        assert_eq!(f, 2.7670116110564327e+19);
        assert!(valid, "exponent should be valid");

        // valid (ends of the earth, salting the earth)
        let (f, valid) =
            moderate_path::<f64>(mantissa, 3, -695, true, false, RoundingKind::NearestTieEven);
        assert_eq!(f, 2.32069302345e-313);
        assert!(valid, "exponent should be valid");

        // invalid ("268A6.177777778", base 15)
        let mantissa: u64 = 4746067219335938;
        let (_, valid) =
            moderate_path::<f64>(mantissa, 15, -9, false, false, RoundingKind::NearestTieEven);
        assert!(!valid, "exponent should be invalid");

        // valid ("268A6.177777778", base 15)
        // 123456.10000000001300614743687445, exactly, should not round up.
        #[cfg(feature = "f128")]
        {
            let mantissa: u128 = 4746067219335938;
            let (f, valid) =
                moderate_path::<f64>(mantissa, 15, -9, false, false, RoundingKind::NearestTieEven);
            assert_eq!(f, 123456.1);
            assert!(valid, "exponent should be valid");
        }

        // Rounding error
        // Adapted from test-parse-random failures.
        let mantissa: u64 = 1009;
        let (_, valid) =
            moderate_path::<f64>(mantissa, 10, -31, false, false, RoundingKind::NearestTieEven);
        assert!(!valid, "exponent should be valid");
    }
}
