//! Shared definitions for bigintegers.

use lib::iter;
use float::*;
use float::convert::*;
use float::rounding::*;
use util::*;
use super::bigcomp;
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
    use stackvector;

    #[cfg(target_pointer_width = "16")]
    type DataType = stackvector::StackVec<[Limb; 256]>;

    #[cfg(target_pointer_width = "32")]
    type DataType = stackvector::StackVec<[Limb; 128]>;

    #[cfg(target_pointer_width = "64")]
    type DataType = stackvector::StackVec<[Limb; 64]>;
}}  // cfg_if

// BIGINT

/// Storage for a big integer type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bigint {
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
pub fn parse_mantissa<Iter>(mut digits: Iter, radix: u32, max_digits: usize)
    -> Bigint
    where Iter: iter::Iterator<Item=u8>
{
    let small_powers = Bigint::small_powers(radix);
    let get_small = | i: usize | unsafe { *small_powers.get_unchecked(i) };
    let count = digits.size_hint().0;
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
    loop {
        // We've parsed the max digits using small values, add to bignum
        if counter == step {
            result.imul_small(get_small(counter));
            result.iadd_small(value);
            counter = 0;
            value = 0;
        }
        // Parse the next digit.
        let digit = match digits.next() {
            Some(v) => v,
            None    => break,
        };
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
        result.imul_small(get_small(counter));
        result.iadd_small(value);
    }

    // If we have any remaining digits after the last value, we need
    // to add a 1 after the rest of the array, it doesn't matter where,
    // just move it up. This is good for the worst-possible float
    // representation. We also need to return an index
    if digits.any(|v| v != b'0') {
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

/// Create a custom wrapper for big mantissa.
pub(super) fn bigint_rounding(is_truncated: bool)
    -> impl FnOnce(&mut ExtendedFloat80, i32)
{
    // Create our wrapper for round_nearest_tie_even.
    // If there are truncated bits, and we are exactly halfway,
    // then we need to set above to true and halfway to false.
    move | f: &mut ExtendedFloat80, shift: i32 | {
        let (mut is_above, mut is_halfway) = round_nearest(f, shift);
        if is_halfway && is_truncated {
            is_above = true;
            is_halfway = false;
        }
        tie_even(f, is_above, is_halfway);
    }
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

/// Use the bigcomp atof function.
#[inline(always)]
pub(super) unsafe fn bigcomp_atof<F, Iter>(digits: Iter, radix: u32, sci_exponent: i32, f: F)
    -> F
    where F: Float,
          F::Unsigned: Mantissa,
          ExtendedFloat<F::Unsigned>: bigcomp::ToBigInt<F::Unsigned>,
          Iter: iter::Iterator<Item=u8>
{
    bigcomp::slow_atof(digits, radix, sci_exponent, f)
}

/// Calculate the mantissa for a big integer with a positive exponent.
#[inline]
pub(super) unsafe fn positive_exponent_atof<F, Iter>(digits: Iter, radix: u32, max_digits: usize, exponent: i32)
    -> F
    where F: FloatRounding<u64>,
          F::Unsigned: Mantissa,
          Iter: iter::Iterator<Item=u8>
{
    // Simple, we just need to multiply by the power of the radix.
    // Now, we can calculate the mantissa and the exponent from this.
    // The binary exponent is the binary exponent for the mantissa
    // shifted to the hidden bit.
    let mut bigmant = parse_mantissa(digits, radix, max_digits);
    bigmant.imul_power(radix, exponent.as_u32());

    // Get the exact representation of the float from the big integer.
    let (mant, is_truncated) = bigmant.hi64();
    let exp = bigmant.bit_length().as_i32() - u64::BITS.as_i32();
    let mut fp = ExtendedFloat { mant: mant, exp: exp };
    fp.round_to_native::<F, _>(bigint_rounding(is_truncated));
    into_float(fp)
}

