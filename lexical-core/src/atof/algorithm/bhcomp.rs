//! Compare the mantissa to the halfway representation of the float.
//!
//! Compares the actual significant digits of the mantissa to the
//! theoretical digits from `b+h`, scaled into the proper range.

use crate::float::*;
use crate::traits::*;
use crate::util::*;

use super::alias::*;
use super::bigcomp;
use super::bignum::*;
use super::format::*;
use super::math::*;

// PARSE MANTISSA
// --------------

/// Iteratively add small digits to the mantissa and increment the counter.
macro_rules! add_digits {
    (
        $iter:expr,
        $result:ident,
        $value:ident,
        $i:ident,
        $counter:ident,
        $step:ident,
        $small_powers:ident,
        $base:ident,
        $radix:ident,
        $max_digits:ident
    ) => {
        while let Some(&digit) = $iter.next() {
            // We've parsed the max digits using small values, add to bignum
            if $counter == $step {
                $result.imul_small($small_powers[$counter]);
                $result.iadd_small($value);
                $counter = 0;
                $value = 0;
            }

            $value *= $base;
            $value += as_limb(to_digit(digit, $radix).unwrap());

            // Check if we've parsed all our possible digits.
            $i += 1;
            $counter += 1;
            if $i == $max_digits {
                break;
            }
        }
    };
}

/// Parse the full mantissa into a big integer.
///
/// Max digits is the maximum number of digits plus one.
pub(super) fn parse_mantissa<'a, F, Data>(data: Data, radix: u32, max_digits: usize) -> Bigint<F>
where
    F: FloatType,
    Data: SlowDataInterface<'a>,
{
    let small_powers = Bigint::<F>::small_powers(radix);
    let count = data.mantissa_digits();
    let bits = count / integral_binary_factor(radix).as_usize();
    let bytes = bits / <Limb as Integer>::BITS;

    // Main loop
    let step = small_powers.len() - 2;
    let base = as_limb(radix);
    let max_digits = max_digits - 1;
    let mut counter = 0;
    let mut value: Limb = 0;
    let mut i: usize = 0;
    let mut result = Bigint::<F>::default();
    result.data.reserve(bytes);

    // Iteratively process all the data in the mantissa.
    let mut integer_iter = data.integer_iter();
    let mut fraction_iter = data.significant_fraction_iter();
    add_digits!(
        integer_iter,
        result,
        value,
        i,
        counter,
        step,
        small_powers,
        base,
        radix,
        max_digits
    );
    if integer_iter.consumed() {
        // Continue if we haven't already processed the max digits.
        add_digits!(
            fraction_iter,
            result,
            value,
            i,
            counter,
            step,
            small_powers,
            base,
            radix,
            max_digits
        );
    }

    // We will always have a remainder, as long as we entered the loop
    // once, or counter % step is 0.
    if counter != 0 {
        result.imul_small(small_powers[counter]);
        result.iadd_small(value);
    }

    // If we have any remaining digits after the last value, we need
    // to add a 1 after the rest of the array, it doesn't matter where,
    // just move it up. This is good for the worst-possible float
    // representation. We also need to return an index.
    // Since we already trimmed trailing zeros, we know there has
    // to be a non-zero digit if there are any left.
    let is_consumed = integer_iter.consumed() && fraction_iter.consumed();
    if !is_consumed {
        result.imul_small(base);
        result.iadd_small(1);
    }

    result
}

// ROUNDING
// --------

/// Custom rounding for round-nearest algorithms.
macro_rules! nearest_cb {
    ($m:tt, $is_truncated:ident, $cb:ident) => {
        // Create our wrapper for round_nearest_tie_*.
        // If there are truncated bits, and we are exactly halfway,
        // then we need to set above to true and halfway to false.
        move |f: &mut ExtendedFloat<$m>, shift: i32| {
            let (mut is_above, mut is_halfway) = round_nearest(f, shift);
            if is_halfway && $is_truncated {
                is_above = true;
                is_halfway = false;
            }
            $cb::<$m>(f, is_above, is_halfway);
        }
    };
}

/// Custom rounding for round-toward algorithms.
#[cfg(feature = "rounding")]
macro_rules! toward_cb {
    ($m:tt, $is_truncated:ident, $cb:ident) => {
        // Create our wrapper for round_towards_tie_*.
        // If there are truncated bits, and truncated is not set, set it.
        move |f: &mut ExtendedFloat<$m>, shift: i32| {
            let truncated = round_toward(f, shift);
            $cb::<$m>(f, $is_truncated | truncated);
        }
    };
}

/// Custom rounding for truncated mantissa.
///
/// Respect rounding rules in the config file.
#[inline]
#[allow(unused_variables)]
pub(super) fn round_to_native<F, M>(fp: &mut ExtendedFloat<M>, is_truncated: bool, kind: RoundingKind)
where
    F: FloatType,
    M: Mantissa,
{
    #[cfg(feature = "rounding")]
    match kind {
        RoundingKind::NearestTieEven => fp.round_to_native::<F, _>(nearest_cb!(M, is_truncated, tie_even)),
        RoundingKind::NearestTieAwayZero => {
            fp.round_to_native::<F, _>(nearest_cb!(M, is_truncated, tie_away_zero))
        },
        RoundingKind::Upward => fp.round_to_native::<F, _>(toward_cb!(M, is_truncated, upward)),
        RoundingKind::Downward => fp.round_to_native::<F, _>(toward_cb!(M, is_truncated, downard)),
        _ => unreachable!(),
    };

    #[cfg(not(feature = "rounding"))]
    fp.round_to_native::<F, _>(nearest_cb!(M, is_truncated, tie_even));
}

/// BIGCOMP PATH
// -------------

/// Maximum number of digits before reverting to bigcomp.
const LARGE_POWER_MAX: usize = 1 << 15;

/// Check if we need to use bigcomp.
#[inline]
pub(super) fn use_bigcomp(radix: u32, count: usize) -> bool {
    // When we have extremely large values, it makes a lot more sense to
    // use am algorithm that scales linearly with input size. We
    // only precompute exponent up to 2^15 anyway for a given radix, so
    // use it. If the radix is not odd, we know the finite number of digits
    // for the worst-case representation, so we can create a valid ratio
    // and ignore the remaining digits.
    radix.is_odd() && count > LARGE_POWER_MAX
}

/// Calculate the mantissa for a big integer with a positive exponent.
pub(super) fn large_atof<'a, F, Data>(
    data: Data,
    radix: u32,
    max_digits: usize,
    exponent: i32,
    kind: RoundingKind,
) -> F
where
    F: FloatType,
    Data: SlowDataInterface<'a>,
{
    // Simple, we just need to multiply by the power of the radix.
    // Now, we can calculate the mantissa and the exponent from this.
    // The binary exponent is the binary exponent for the mantissa
    // shifted to the hidden bit.
    let mut bigmant = parse_mantissa::<F, Data>(data, radix, max_digits);
    bigmant.imul_power(radix, exponent.as_u32());

    // Get the exact representation of the float from the big integer.
    let (mant, is_truncated) = bigmant.hi64();
    let exp = bigmant.bit_length().as_i32() - <u64 as Integer>::BITS.as_i32();
    let mut fp = ExtendedFloat {
        mant,
        exp,
    };
    round_to_native::<F, _>(&mut fp, is_truncated, kind);
    into_float(fp)
}

// BHCOMP
// ------

/// Calculate the mantissa for a big integer with a negative exponent.
///
/// This invokes the comparison with `b+h`.
pub(super) fn small_atof<'a, F, Data>(
    data: Data,
    radix: u32,
    max_digits: usize,
    exponent: i32,
    f: F,
    kind: RoundingKind,
) -> F
where
    F: FloatType,
    Data: SlowDataInterface<'a>,
{
    // Get the significant digits and radix exponent for the real digits.
    let mut real_digits = parse_mantissa::<F, Data>(data, radix, max_digits);
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
        true => (theor_exp - real_exp, -real_exp, 0),
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
///     `FloatState2`).
///     sci_exponent and digits.size_hint() must not overflow i32.
pub(super) fn atof<'a, F, Data>(data: Data, radix: u32, f: F, kind: RoundingKind) -> F
where
    F: FloatType,
    Data: SlowDataInterface<'a>,
{
    // We have a finite conversions number of digits for base10.
    // In order for a float in radix `b` with a finite number of digits
    // to have a finite representation in radix `y`, `b` should divide
    // an integer power of `y`. This means for binary, all even radixes
    // have finite representations, and all odd ones do not.
    let max_digits = unwrap_or_max(F::max_correct_digits(radix));
    let count = max_digits.min(data.mantissa_digits());
    let exponent = data.scientific_exponent() + 1 - count.as_i32();

    if cfg!(feature = "radix") && use_bigcomp(radix, count) {
        // Use the slower algorithm for giant data, since we use a lot less memory.
        bigcomp::atof(data, radix, f, kind)
    } else if exponent >= 0 {
        large_atof(data, radix, max_digits, exponent, kind)
    } else {
        small_atof(data, radix, max_digits, exponent, f, kind)
    }
}
