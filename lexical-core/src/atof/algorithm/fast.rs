//! Fast-path algorithm. Only works for a small subset of floats.
//!
//! The fast-path algorithm can only be used if the float can be exactly
//! represented using the mantissa digits, without truncation, or if
//! the exponent is positive and the digits can be exactly represented
//! by multiplying a valid mantissa representation along with digits
//! shifted from the exponent to the mantissa.
//!
//! See `traits/exact_float.rs` `ExactFloat` for detailed documentation
//! on these limits.

use crate::util::*;

use super::alias::*;

// FAST PATH
// ---------

/// Convert mantissa to exact value for a non-base2 power.
///
/// Returns the resulting float and if the value can be represented exactly.
pub(crate) fn fast_path<F>(mantissa: F::MantissaType, radix: u32, exponent: i32) -> Option<F>
where
    F: FloatType,
{
    debug_assert_radix!(radix);
    debug_assert!(log2(radix) == 0, "Cannot use `fast_path` with a power of 2.");

    // `mantissa >> (F::MANTISSA_SIZE+1) != 0` effectively checks if the
    // value has a no bits above the hidden bit, which is what we want.
    let (min_exp, max_exp) = F::exponent_limit(radix);
    let shift_exp = F::mantissa_limit(radix);
    let mantissa_size = F::MANTISSA_SIZE + 1;
    if mantissa >> mantissa_size != F::MantissaType::ZERO {
        // Would require truncation of the mantissa.
        None
    } else if exponent == 0 {
        // 0 exponent, same as value, exact representation.
        let float: F = as_cast(mantissa);
        Some(float)
    } else if exponent >= min_exp && exponent <= max_exp {
        // Value can be exactly represented, return the value.
        // Use powi, since it's correct, and faster on
        // the fast-path.
        let float: F = as_cast(mantissa);
        Some(float.pow(radix, exponent))
    } else if exponent >= 0 && exponent <= max_exp + shift_exp {
        // Check to see if we have a disguised fast-path, where the
        // number of digits in the mantissa is very small, but and
        // so digits can be shifted from the exponent to the mantissa.
        // https://www.exploringbinary.com/fast-path-decimal-to-floating-point-conversion/
        let small_powers = F::MantissaType::small_powers(radix);
        let shift = exponent - max_exp;
        let power = small_powers[shift.as_usize()];

        // Compute the product of the power, if it overflows,
        // prematurely return early, otherwise, if we didn't overshoot,
        // we can get an exact value.
        let value = mantissa.checked_mul(power)?;
        if value >> mantissa_size != F::MantissaType::ZERO {
            None
        } else {
            // Use powi, since it's correct, and faster on
            // the fast-path.
            let float: F = as_cast(value);
            Some(float.pow(radix, max_exp))
        }
    } else {
        // Cannot be exactly represented, exponent too small or too big,
        // would require truncation.
        None
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_fast_path_test() {
        // valid
        let mantissa = (1 << f32::MANTISSA_SIZE) - 1;
        for base in BASE_POWN.iter().cloned() {
            let (min_exp, max_exp) = f32::exponent_limit(base);
            for exp in min_exp..max_exp + 1 {
                let valid = fast_path::<f32>(mantissa, base, exp).is_some();
                assert!(valid, "should be valid {:?}.", (mantissa, base, exp));
            }
        }

        // Check slightly above valid exponents
        let f = fast_path::<f32>(123, 10, 15);
        assert_eq!(f, Some(1.23e+17));

        // Exponent is 1 too high, pushes over the mantissa.
        let f = fast_path::<f32>(123, 10, 16);
        assert!(f.is_none());

        // Mantissa is too large, checked_mul should overflow.
        let f = fast_path::<f32>(mantissa, 10, 11);
        assert!(f.is_none());

        // invalid mantissa
        #[cfg(feature = "radix")]
        {
            let (_, max_exp) = f64::exponent_limit(3);
            let f = fast_path::<f32>(1 << f32::MANTISSA_SIZE, 3, max_exp + 1);
            assert!(f.is_none(), "invalid mantissa");
        }

        // invalid exponents
        for base in BASE_POWN.iter().cloned() {
            let (min_exp, max_exp) = f32::exponent_limit(base);
            let f = fast_path::<f32>(mantissa, base, min_exp - 1);
            assert!(f.is_none(), "exponent under min_exp");

            let f = fast_path::<f32>(mantissa, base, max_exp + 1);
            assert!(f.is_none(), "exponent above max_exp");
        }
    }

    #[test]
    fn double_fast_path_test() {
        // valid
        let mantissa = (1 << f64::MANTISSA_SIZE) - 1;
        for base in BASE_POWN.iter().cloned() {
            let (min_exp, max_exp) = f64::exponent_limit(base);
            for exp in min_exp..max_exp + 1 {
                let f = fast_path::<f64>(mantissa, base, exp);
                assert!(f.is_some(), "should be valid {:?}.", (mantissa, base, exp));
            }
        }

        // invalid mantissa
        #[cfg(feature = "radix")]
        {
            let (_, max_exp) = f64::exponent_limit(3);
            let f = fast_path::<f64>(1 << f64::MANTISSA_SIZE, 3, max_exp + 1);
            assert!(f.is_none(), "invalid mantissa");
        }

        // invalid exponents
        for base in BASE_POWN.iter().cloned() {
            let (min_exp, max_exp) = f64::exponent_limit(base);
            let f = fast_path::<f64>(mantissa, base, min_exp - 1);
            assert!(f.is_none(), "exponent under min_exp");

            let f = fast_path::<f64>(mantissa, base, max_exp + 1);
            assert!(f.is_none(), "exponent above max_exp");
        }
    }
}
