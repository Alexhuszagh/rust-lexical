//! Shared definitions for bigintegers.

use float::*;
use float::convert::*;
use float::rounding::*;
use util::*;
use super::alias::*;
use super::correct::FloatSlice;
use super::exponent::*;
use super::math::*;

// CONSTANTS

/// Maximum number of digits before reverting to bigcomp.
pub(super) const LARGE_POWER_MAX: usize = 1 << 15;

// DATA TYPE

cfg_if! {
if #[cfg(feature = "radix")] {
    use lib::Vec;
    type DataType = Vec<Limb>;
} else {
    // Maximum denominator is 767 mantissa digits + 324 exponent,
    // or 1091 digits, or approximately 3600 bits (round up to 4k).
    use arrayvec;

    #[cfg(limb_width_32)]
    type DataType = arrayvec::ArrayVec<[Limb; 128]>;

    #[cfg(limb_width_64)]
    type DataType = arrayvec::ArrayVec<[Limb; 64]>;
}}  // cfg_if

// BIGINT

/// Storage for a big integer type.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub(super) struct Bigint {
    /// Internal storage for the Bigint, in little-endian order.
    data: DataType,
}

impl Default for Bigint {
    fn default() -> Self {
        // We want to avoid lower-order
        let mut bigint = Bigint { data: DataType::default() };
        bigint.data.reserve(20);
        bigint
    }
}

impl SharedOps for Bigint {
    type StorageType = DataType;

    #[inline]
    fn data<'a>(&'a self) -> &'a Self::StorageType {
        &self.data
    }

    #[inline]
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType {
        &mut self.data
    }
}

impl SmallOps for Bigint {
}

impl LargeOps for Bigint {
}

// PARSE MANTISSA

/// Parse the full mantissa into a big integer.
///
/// Max digits is the maximum number of digits plus one.
pub(super) fn parse_mantissa(slc: FloatSlice, radix: u32, max_digits: usize)
    -> Bigint
{
    let small_powers = Bigint::small_powers(radix);
    let count = slc.mantissa_digits();
    let bits = count / integral_binary_factor(radix).as_usize();
    let bytes = bits / Limb::BITS;

    // Main loop
    let step = small_powers.len() - 2;
    let base = as_limb(radix);
    let max_digits = max_digits - 1;
    let mut counter = 0;
    let mut value: Limb = 0;
    let mut i: usize = 0;
    let mut result = Bigint::default();
    result.data.reserve(bytes);

    let mut iter = slc.mantissa_iter();
    while let Some(&digit) = iter.next() {
        // We've parsed the max digits using small values, add to bignum
        if counter == step {
            result.imul_small(small_powers[counter]);
            result.iadd_small(value);
            counter = 0;
            value = 0;
        }

        value *= base;
        value += as_limb(char_to_digit(digit));

        // Check if we've parsed all our possible digits.
        i += 1;
        counter += 1;
        if i == max_digits {
            break;
        }
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
    if let Some(_) = iter.next() {
        result.imul_small(base);
        result.iadd_small(1);
    }

    result
}

/// Implied method to calculate the number of digits from a 32-bit float.
fn max_digits_f32(radix: u32) -> Option<usize> {
    match radix {
        6  => Some(103),
        10 => Some(114),
        12 => Some(117),
        14 => Some(119),
        18 => Some(122),
        20 => Some(123),
        22 => Some(123),
        24 => Some(124),
        26 => Some(125),
        28 => Some(125),
        30 => Some(126),
        34 => Some(127),
        36 => Some(127),
        // Powers of two and odd numbers should be unreachable
        _  => None,
    }
}

/// Implied method to calculate the number of digits from a 64-bit float.
fn max_digits_f64(radix: u32) -> Option<usize> {
    match radix {
        6  => Some(682),
        10 => Some(769),
        12 => Some(792),
        14 => Some(808),
        18 => Some(832),
        20 => Some(840),
        22 => Some(848),
        24 => Some(854),
        26 => Some(859),
        28 => Some(864),
        30 => Some(868),
        34 => Some(876),
        36 => Some(879),
        // Powers of two and odd numbers should be unreachable
        _  => None,
    }
}

/// Calculate the maximum number of digits possible in the mantissa.
///
/// Returns the maximum number of digits plus one.
///
/// We can exactly represent a float in radix `b` from radix 2 if
/// `b` is divisible by 2. This function calculates the exact number of
/// digits required to exactly represent that float.
///
/// According to the "Handbook of Floating Point Arithmetic",
/// for IEEE754, with emin being the min exponent, p2 being the
/// precision, and b being the radix, the number of digits follows as:
///
/// `−emin + p2 + ⌊(emin + 1) log(2, b) − log(1 − 2^(−p2), b)⌋`
///
/// For f32, this follows as:
///     emin = -126
///     p2 = 24
///
/// For f64, this follows as:
///     emin = -1022
///     p2 = 53
///
/// In Python:
///     `-emin + p2 + math.floor((emin+1)*math.log(2, b) - math.log(1-2**(-p2), b))`
///
/// This was used to calculate the maximum number of digits for [2, 36].
pub(super) fn max_digits<F>(radix: u32)
    -> Option<usize>
    where F: Float
{
    match F::BITS {
        32 => max_digits_f32(radix),
        64 => max_digits_f64(radix),
        _  => unreachable!(),
    }
}

// ROUNDING

// Returning impl Trait is not supported prior to Rust 1.26.
// Use a macro and store to a variable, rather than actually
// using the `impl Trait` with an inlined function.

/// Custom rounding for round-nearest algorithms.
macro_rules! nearest_cb {
    ($m:tt, $is_truncated:ident, $cb:ident) => {
        // Create our wrapper for round_nearest_tie_*.
        // If there are truncated bits, and we are exactly halfway,
        // then we need to set above to true and halfway to false.
        move | f: &mut ExtendedFloat<$m>, shift: i32 | {
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
        move | f: &mut ExtendedFloat<$m>, shift: i32 | {
            let truncated = round_toward(f, shift);
            $cb::<$m>(f, $is_truncated | truncated);
        }
    };
}

/// Custom rounding for truncated mantissa.
///
/// Respect rounding rules in the config file.
#[allow(unused_variables)]
pub(super) fn round_to_native<F>(fp: &mut ExtendedFloat80, is_truncated: bool, kind: RoundingKind)
    where F: FloatType
{
    type M = u64;

    // Define a simplified function, since we can't store the callback to
    // a variable without `impl Trait`, which requires 1.26.0.
    #[inline(always)]
    fn round<F, Cb>(fp: &mut ExtendedFloat80, cb: Cb)
        where F: FloatRounding<M>,
              Cb: FnOnce(&mut ExtendedFloat<M>, i32)
    {
        fp.round_to_native::<F, _>(cb);
    }

    #[cfg(feature = "rounding")]
    match kind {
        RoundingKind::NearestTieEven     => round::<F, _>(fp, nearest_cb!(M, is_truncated, tie_even)),
        RoundingKind::NearestTieAwayZero => round::<F, _>(fp, nearest_cb!(M, is_truncated, tie_away_zero)),
        RoundingKind::Upward             => round::<F, _>(fp, toward_cb!(M, is_truncated, upward)),
        RoundingKind::Downward           => round::<F, _>(fp, toward_cb!(M, is_truncated, downard)),
        _                                => unreachable!(),
    };

    #[cfg(not(feature = "rounding"))]
    round::<F, _>(fp, nearest_cb!(M, is_truncated, tie_even));
}

/// BIGCOMP PATH

/// Check if we need to use bigcomp.
#[inline]
pub(super) fn use_bigcomp(radix: u32, count: usize)
    -> bool
{
    // When we have extremely large values, it makes a lot more sense to
    // use am algorithm that scales linearly with input size. We
    // only precompute exponent up to 2^15 anyway for a given radix, so
    // use it. If the radix is not odd, we know the finite number of digits
    // for the worst-case representation, so we can create a valid ratio
    // and ignore the remaining digits.
    radix.is_odd() && count > LARGE_POWER_MAX
}

/// Calculate the mantissa for a big integer with a positive exponent.
#[inline]
pub(super) fn large_atof<F>(slc: FloatSlice, radix: u32, max_digits: usize, exponent: i32, kind: RoundingKind)
    -> F
    where F: FloatType
{
    // Simple, we just need to multiply by the power of the radix.
    // Now, we can calculate the mantissa and the exponent from this.
    // The binary exponent is the binary exponent for the mantissa
    // shifted to the hidden bit.
    let mut bigmant = parse_mantissa(slc, radix, max_digits);
    bigmant.imul_power(radix, exponent.as_u32());

    // Get the exact representation of the float from the big integer.
    let (mant, is_truncated) = bigmant.hi64();
    let exp = bigmant.bit_length().as_i32() - u64::BITS.as_i32();
    let mut fp = ExtendedFloat { mant: mant, exp: exp };
    round_to_native::<F>(&mut fp, is_truncated, kind);
    into_float(fp)
}

