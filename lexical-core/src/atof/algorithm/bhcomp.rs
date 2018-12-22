//! Compare the mantissa to the halfway representation of the float.
//!
//! Compares the actual significant digits of the mantissa to the
//! theoretical digits from `b+h`, scaled into the proper range.

use util::*;
use super::alias::*;
use super::bigcomp;
use super::correct::FloatSlice;
use super::bigint::*;
use super::math::*;

/// Calculate the mantissa for a big integer with a negative exponent.
///
/// This invokes the comparison with `b+h`.
#[inline]
pub(super) fn small_atof<F>(slc: FloatSlice, radix: u32, max_digits: usize, exponent: i32, f: F, kind: RoundingKind)
    -> F
    where F: FloatType
{
    // Get the significant digits and radix exponent for the real digits.
    let mut real_digits = parse_mantissa(slc, radix, max_digits);
    let real_exp = exponent;
    debug_assert!(real_exp < 0);

    // Get the significant digits and the binary exponent for `b+h`.
    let theor = bigcomp::theoretical_float(f, kind);
    let mut theor_digits = Bigint::from_u64(theor.mant().as_u64());
    let theor_exp = theor.exp();

    // We need to scale the real digits and `b+h` digits to be the same
    // order. We currently have `real_exp`, in `radix`, that needs to be
    // shifted to `theor_digits` (since it is negative), and `theor_exp`
    // to either `theor_digits` or `real_digits` as a power of 2 (since it
    // may be positive or negative). Try to remove as many powers of 2
    // as possible. All values are relative to `theor_digits`, that is,
    // reflect the power you need to multiply `theor_digits` by.
    let (binary_exp, halfradix_exp, radix_exp) = match radix.is_even() {
        // Can remove a power-of-two.
        // Both are on opposite-sides of equation, can factor out a
        // power of two.
        //
        // Example: 10^-10, 2^-10   -> ( 0, 10, 0)
        // Example: 10^-10, 2^-15   -> (-5, 10, 0)
        // Example: 10^-10, 2^-5    -> ( 5, 10, 0)
        // Example: 10^-10, 2^5 -> (15, 10, 0)
        true  => (theor_exp - real_exp, -real_exp, 0),
        // Cannot remove a power-of-two.
        false => (theor_exp, 0, -real_exp),
    };

    // Carry out our multiplication.
    if halfradix_exp != 0 {
        theor_digits.imul_power(radix / 2, halfradix_exp.as_u32());
    }
    if radix_exp != 0 {
        theor_digits.imul_power(radix, radix_exp.as_u32());
    }
    if binary_exp > 0 {
        theor_digits.imul_power(2, binary_exp.as_u32());
    } else if binary_exp < 0 {
        real_digits.imul_power(2, (-binary_exp).as_u32());
    }

    bigcomp::round_to_native(f, real_digits.compare(&theor_digits), kind)
}

/// Calculate the exact value of the float.
///
/// Notes:
///     The digits iterator must not have any trailing zeros (true for
///     `FloatSlice`).
///     sci_exponent and digits.size_hint() must not overflow i32.
pub(super) fn atof<'a, F>(slc: FloatSlice, radix: u32, f: F, kind: RoundingKind)
    -> F
    where F: FloatType
{
    // We have a finite conversions number of digits for base10.
    // In order for a float in radix `b` with a finite number of digits
    // to have a finite representation in radix `y`, `b` should divide
    // an integer power of `y`. This means for binary, all even radixes
    // have finite representations, and all odd ones do not.
    let max_digits = unwrap_or_max(max_digits::<F>(radix));
    let count = max_digits.min(slc.mantissa_digits());
    let exponent = slc.scientific_exponent() + 1 - count.as_i32();

    if cfg!(feature = "radix") && use_bigcomp(radix, count) {
        // Use the slower algorithm for giant data, since we use a lot less memory.
        bigcomp::atof(slc, radix, f, kind)
    } else if exponent >= 0 {
        large_atof(slc, radix, max_digits, exponent, kind)
    } else {
        small_atof(slc, radix, max_digits, exponent, f, kind)
    }
}
