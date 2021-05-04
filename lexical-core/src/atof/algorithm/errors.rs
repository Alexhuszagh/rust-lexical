//! Estimate the error in an 80-bit approximation of a float.
//!
//! This estimates the error in a floating-point representation.
//!
//! This implementation is loosely based off the Golang implementation,
//! found here:
//!     https://golang.org/src/strconv/atof.go

use crate::float::*;
use crate::traits::*;
use crate::util::*;

// HELPERS
// -------

/// Check if the error is accurate with a round-nearest rounding scheme.
#[inline]
fn nearest_error_is_accurate<M>(errors: u32, fp: &ExtendedFloat<M>, extrabits: i32)
    -> bool
where
    M: Mantissa,
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
fn toward_error_is_accurate<M>(errors: u32, fp: &ExtendedFloat<M>, extrabits: i32)
    -> bool
where
    M: Mantissa,
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

// FLOAT ERRORS
// ------------

/// Calculate if the errors in calculating the extended-precision float.
///
/// Specifically, we want to know if we are close to a halfway representation,
/// or halfway between `b` and `b+1`, or `b+h`. The halfway representation
/// has the form:
///     SEEEEEEEHMMMMMMMMMMMMMMMMMMMMMMM100...
/// where:
///     S = Sign Bit
///     E = Exponent Bits
///     H = Hidden Bit
///     M = Mantissa Bits
///
/// The halfway representation has a bit set 1-after the mantissa digits,
/// and no bits set immediately afterward, making it impossible to
/// round between `b` and `b+1` with this representation.
pub trait FloatErrors: Mantissa {
    /// Get the full error scale.
    #[inline(always)]
    fn error_scale() -> u32 {
        8
    }

    /// Get the half error scale.
    #[inline]
    fn error_halfscale() -> u32 {
        Self::error_scale() / 2
    }

    /// Determine if the number of errors is tolerable for float precision.
    #[inline]
    #[allow(unused_variables)]
    fn error_is_accurate<F: Float>(
        errors: u32,
        fp: &ExtendedFloat<Self>,
        kind: RoundingKind,
    ) -> bool {
        // Determine if extended-precision float is a good approximation.
        // If the error has affected too many units, the float will be
        // inaccurate, or if the representation is too close to halfway
        // that any operations could affect this halfway representation.
        // See the documentation for dtoa for more information.
        let full = Self::FULL;
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
            nearest_error_is_accurate::<Self>(errors, fp, extrabits)
        }

        #[cfg(feature = "rounding")]
        {
            if kind.is_nearest() {
                nearest_error_is_accurate::<Self>(errors, fp, extrabits)
            } else {
                toward_error_is_accurate::<Self>(errors, fp, extrabits)
            }
        }
    }
}

impl FloatErrors for u64 {
}

#[cfg(feature = "f128")]
impl FloatErrors for u128 {
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::correct::moderate_path;

    #[test]
    fn test_halfway_round_down() {
        let radix = 10;
        let kind = RoundingKind::NearestTieEven;

        // Halfway, round-down tests
        assert!(moderate_path::<f64, _>(9007199254740992u64, radix, 0, false, kind).1);
        assert!(!moderate_path::<f64, _>(9007199254740993u64, radix, 0, false, kind).1);
        assert!(moderate_path::<f64, _>(9007199254740994u64, radix, 0, false, kind).1);

        assert!(moderate_path::<f64, _>(18014398509481984u64, radix, 0, false, kind).1);
        assert!(!moderate_path::<f64, _>(18014398509481986u64, radix, 0, false, kind).1);
        assert!(moderate_path::<f64, _>(18014398509481988u64, radix, 0, false, kind).1);

        assert!(moderate_path::<f64, _>(9223372036854775808u64, radix, 0, false, kind).1);
        assert!(!moderate_path::<f64, _>(9223372036854776832u64, radix, 0, false, kind).1);
        assert!(moderate_path::<f64, _>(9223372036854777856u64, radix, 0, false, kind).1);

        // Add a 0 but say we're truncated.
        assert!(moderate_path::<f64, _>(9007199254740992000u64, radix, -3, true, kind).1);
        assert!(!moderate_path::<f64, _>(9007199254740993000u64, radix, -3, true, kind).1);
        assert!(moderate_path::<f64, _>(9007199254740994000u64, radix, -3, true, kind).1);
    }

    #[test]
    fn test_halfway_round_up() {
        let radix = 10;
        let kind = RoundingKind::NearestTieEven;

        // Halfway, round-down tests
        assert!(moderate_path::<f64, _>(9007199254740994u64, radix, 0, false, kind).1);
        assert!(!moderate_path::<f64, _>(9007199254740995u64, radix, 0, false, kind).1);
        assert!(moderate_path::<f64, _>(9007199254740996u64, radix, 0, false, kind).1);

        assert!(moderate_path::<f64, _>(18014398509481988u64, radix, 0, false, kind).1);
        assert!(!moderate_path::<f64, _>(18014398509481990u64, radix, 0, false, kind).1);
        assert!(moderate_path::<f64, _>(18014398509481992u64, radix, 0, false, kind).1);

        assert!(moderate_path::<f64, _>(9223372036854777856u64, radix, 0, false, kind).1);
        assert!(!moderate_path::<f64, _>(9223372036854778880u64, radix, 0, false, kind).1);
        assert!(moderate_path::<f64, _>(9223372036854779904u64, radix, 0, false, kind).1);

        // Add a 0 but say we're truncated.
        assert!(moderate_path::<f64, _>(9007199254740994000u64, radix, -3, true, kind).1);
        assert!(!moderate_path::<f64, _>(9007199254740994990u64, radix, -3, true, kind).1);
        assert!(!moderate_path::<f64, _>(9007199254740995000u64, radix, -3, true, kind).1);
        assert!(!moderate_path::<f64, _>(9007199254740995010u64, radix, -3, true, kind).1);
        assert!(moderate_path::<f64, _>(9007199254740996000u64, radix, -3, true, kind).1);
    }
}
