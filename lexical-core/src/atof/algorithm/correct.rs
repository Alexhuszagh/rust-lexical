//! Correct algorithms for string-to-float conversions.
//!
//! This implementation is loosely based off the Golang implementation,
//! found here:
//!     https://golang.org/src/strconv/atof.go
//!
//! The extended-precision and decimal versions are highly
// Fix a compiler bug that thinks `ExactExponent` isn't used.
#![allow(unused_imports)]

use lib::ptr;

use atoi;
use float::*;
use util::*;
use super::alias::*;
use super::cached::ModeratePathCache;
use super::bhcomp;
use super::exponent::*;
use super::small_powers::get_small_powers_64;

// SHARED

// Left-trim leading 0s.
macro_rules! ltrim_0 {
    ($bytes:expr) => { ltrim_char_slice($bytes, b'0') };
}

// Right-trim leading 0s.
macro_rules! rtrim_0 {
    ($bytes:expr) => { rtrim_char_slice($bytes, b'0') };
}

// Fast path for the parse algorithm.
// In this case, the mantissa can be represented by an integer,
// which allows any value to be exactly reconstructed.

// FLOAT SLICE

/// Substrings and information from parsing the float.
#[cfg_attr(test, derive(Debug))]
pub(super) struct FloatSlice<'a> {
    /// Substring for the integer component of the mantissa.
    integer: &'a [u8],
    /// Substring for the fraction component of the mantissa.
    fraction: &'a [u8],
    /// Offset to where the digits start in either integer or fraction.
    digits_start: usize,
    /// Offset to where the digits end in the fraction.
    digits_end: usize,
    /// Number of truncated digits from the mantissa.
    truncated: usize,
    /// Raw exponent for the float.
    raw_exponent: i32,
}

impl<'a> FloatSlice<'a> {
    /// Create uninitialized slice.
    #[inline]
    pub(super) fn uninitialized() -> FloatSlice<'a> {
        FloatSlice {
            integer: &[],
            fraction: &[],
            digits_start: explicit_uninitialized(),
            digits_end: explicit_uninitialized(),
            truncated: explicit_uninitialized(),
            raw_exponent: explicit_uninitialized(),
        }
    }

    /// Get the length of the integer substring.
    #[inline]
    pub(super) fn integer_len(&self) -> usize {
        self.integer.len()
    }

    /// Get number of parsed integer digits.
    #[inline]
    pub(super) fn integer_digits(&self) -> usize {
        self.integer_len()
    }

    /// Iterate over the integer digits.
    #[inline]
    pub(super) fn integer_iter(&self) -> SliceIter<u8> {
        self.integer.iter()
    }

    /// Get the length of the fraction substring.
    #[inline]
    pub(super) fn fraction_len(&self) -> usize {
        self.digits_end
    }

    /// Iterate over the fraction digits.
    #[inline]
    pub(super) fn fraction_digits(&self) -> usize {
        self.fraction_len() - self.digits_start
    }

    /// Iterate over the digits, by chaining two slices.
    #[inline]
    pub(super) fn fraction_iter(&self) -> SliceIter<u8> {
        // We need to rtrim the zeros in the slice fraction.
        // These are useless and just add computational complexity later,
        // just like leading zeros in the integer.
        // We need them to calculate the number of truncated bytes,
        // but we should remove them before doing anything costly.
        // In practice, we only call `mantissa_iter()` once per parse,
        // so this is effectively free.
        self.fraction[self.digits_start..self.digits_end].iter()
    }

    /// Get the number of digits in the mantissa.
    /// Cannot overflow, since this is based off a single usize input string.
    #[inline]
    pub(super) fn mantissa_digits(&self) -> usize {
        self.integer_digits() + self.fraction_digits()
    }

    /// Iterate over the mantissa digits, by chaining two slices.
    #[inline]
    pub(super) fn mantissa_iter(&self) -> ChainedSliceIter<u8> {
        self.integer_iter().chain(self.fraction_iter())
    }

    /// Get number of truncated digits.
    #[inline]
    pub(super) fn truncated_digits(&self) -> usize {
        // If we have truncated digits, need to remove the number of
        // trailing zeros from that.
        let trailing = self.fraction.len() - self.digits_end;
        match self.truncated > trailing {
            true  => self.truncated - trailing,
            false => 0,
        }
    }

    /// Get the mantissa exponent from the raw exponent.
    #[inline]
    pub(super) fn mantissa_exponent(&self) -> i32 {
        mantissa_exponent(self.raw_exponent, self.fraction_len(), self.truncated_digits())
    }

    /// Get the scientific exponent from the raw exponent.
    #[inline]
    pub(super) fn scientific_exponent(&self) -> i32 {
        let fraction_start = match self.digits_start.is_zero() {
            true  => 0,
            false => self.digits_start,
        };
        scientific_exponent(self.raw_exponent, self.integer_digits(), fraction_start)
    }
}

// PARSE
// -----

// Need to adjust the mantissa if we over-parsed it.
#[inline]
fn adjust_truncated_mantissa<M>(mantissa: M, radix: u32, trimmed: usize, truncated: usize)
    -> M
    where M: Mantissa
{
    if trimmed > truncated {
        // We have truncated digits, need to adjust the mantissa.
        // This is because we have digits that are not counted in the
        // resulting fraction that are present otherwise, only include
        // non-truncated digits.
        let base: M = as_cast(radix);
        let pow: M = base.pow(as_cast(trimmed - truncated));
        mantissa / pow
    } else {
        mantissa
    }
}

/// Parse the mantissa from a string.
///
/// Returns the mantissa, the the number of parsed integer digits,
/// the number of parsed fraction digits, and the number of truncated
/// digits (including those in both the integer and fraction).
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
fn parse_mantissa<'a, M>(radix: u32, mut bytes: &'a [u8])
    -> (M, FloatSlice, &'a [u8], Option<&'a u8>)
    where M: Mantissa
{
    // Initialize our variables for the output.
    let mut mantissa: M = M::ZERO;
    let mut slc = FloatSlice::uninitialized();

    // Trim the leading 0s.
    // Need to force this here, since if not, conversion of usize dot to
    // i32 may truncate when mantissa does not, which would lead to faulty
    // results. If we trim the 0s here, we guarantee any time `dot as i32`
    // leads to a truncation, mantissa will overflow.
    bytes = ltrim_0!(bytes).0;
    let first = bytes.as_ptr();

    // Parse the integral value.
    // Use the checked parsers so the truncated value is valid even if
    // the entire value is not parsed.
    let (len, truncated) = atoi::checked_positive(&mut mantissa, as_cast(radix), bytes);
    // We know this is safe, since atoi returns a value <= bytes.len().
    bytes = &index!(bytes[len..]);
    slc.integer = slice_from_span(first, len);

    // Check for trailing digits.
    let has_fraction = Some(&b'.') == bytes.get(0);
    if has_fraction && truncated.is_none() {
        // Has a decimal, no truncation, calculate the rest of it.
        // We know this is safe, since we know we have a fraction.
        bytes = &index!(bytes[1..]);
        let first = bytes.as_ptr();
        if mantissa.is_zero() {
            // Can ignore the leading digits while the mantissa is 0.
            // This allows us to represent extremely small values
            // using the fast route in non-scientific notation.
            // For example, this allows us to use the fast path for
            // both "1e-29" and "0.0000000000000000000000000001",
            // otherwise, only the former would work.
            let trim = ltrim_0!(bytes);
            bytes = trim.0;
            slc.digits_start = trim.1;
        } else {
            slc.digits_start = 0;
        }

        // Parse the remaining decimal. Since the truncation is only in
        // the fraction, no decimal place affects it.
        let (len, truncated) = atoi::checked_positive(&mut mantissa, as_cast(radix), bytes);
        // We know this is safe, since atoi returns a a value <= bytes.len().
        let bytes = &index!(bytes[len..]);
        slc.fraction = slice_from_span(first, len + slc.digits_start);
        let trim = rtrim_0!(slc.fraction);
        slc.digits_end = len + slc.digits_start - trim.1;
        slc.truncated = truncated.map_or(0, |p| distance(p, bytes.as_ptr()));
        mantissa = adjust_truncated_mantissa(mantissa, radix, trim.1, slc.truncated);
        (mantissa, slc, bytes, truncated)
    } else if has_fraction {
        // Integral overflow occurred, cannot add more values, but a fraction exists.
        // Ignore the remaining characters, but factor them into the dot exponent.
        // We know this is safe, since we know we have a fraction.
        bytes = &index!(bytes[1..]);
        let first = bytes.as_ptr();
        let len = bytes.iter()
            .take_while(|&&c| char_to_digit(c).as_u32() < radix)
            .count();
        // We know this is safe, since it's generated from the iterator.
        bytes = &index!(bytes[len..]);
        slc.digits_start = 0;
        slc.fraction = slice_from_span(first, len);
        let trim = rtrim_0!(slc.fraction);
        slc.digits_end = len - trim.1;
        slc.truncated = distance(truncated.unwrap(), bytes.as_ptr()) - 1;
        mantissa = adjust_truncated_mantissa(mantissa, radix, trim.1, slc.truncated);
        (mantissa, slc, bytes, truncated)
    } else {
        // No decimal, return the number of truncated bytes.
        slc.digits_start = 0;
        slc.fraction = slice_from_span(bytes.as_ptr(), 0);
        slc.truncated = truncated.map_or(0, |p| distance(p, bytes.as_ptr()));
        slc.digits_end = 0;
        (mantissa, slc, bytes, truncated)
    }
}

/// Parse the mantissa and exponent from a string.
///
/// Returns the mantissa, the exponent, the scientific-notation exponent,
/// the number of parsed digits, and the current parser state.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
fn parse_float<'a, M>(radix: u32, bytes: &'a [u8])
    -> (M, FloatSlice, &'a [u8], Option<&'a u8>)
    where M: Mantissa
{
    let (mantissa, mut slc, bytes, truncated) = parse_mantissa::<M>(radix, bytes);
    let (raw_exponent, bytes) = parse_exponent(radix, bytes);
    slc.raw_exponent = raw_exponent;

    (mantissa, slc, bytes, truncated)
}

// FAST
// ----

// Generate exact representations of a float using solely native
// floating-point intermediates.

/// Check if value is power of 2 and get the power.
#[inline]
fn pow2_exponent(radix: u32) -> i32 {
    match radix {
        2  => 1,
        4  => 2,
        8  => 3,
        16 => 4,
        32 => 5,
        _  => 0,
    }
}

/// Detect if a float representation is exactly halfway after truncation.
#[cfg(feature = "radix")]
#[inline]
fn is_halfway<F: FloatType>(mantissa: u64)
    -> bool
{
    // Get the leading and trailing zeros from the least-significant bit.
    let bit_length: i32 = 64 - mantissa.leading_zeros().as_i32();
    let trailing_zeros: i32 = mantissa.trailing_zeros().as_i32();

    // We need exactly mantissa+2 elements between these if it is halfway.
    // The hidden bit is mantissa+1 elements away, which is the last non-
    // truncated bit, while mantissa+2
    bit_length - trailing_zeros == F::MANTISSA_SIZE + 2
}

/// Detect if a float representation is odd after truncation.
#[cfg(feature = "radix")]
#[inline]
fn is_odd<F: FloatType>(mantissa: u64)
    -> bool
{
    // Get the leading and trailing zeros from the least-significant bit.
    let bit_length: i32 = 64 - mantissa.leading_zeros().as_i32();
    let shift = bit_length - (F::MANTISSA_SIZE + 1);
    if shift >= 0 {
        // Have enough bits to have a full mantissa in the float, need to
        // check if that last bit is set.
        let mask = 1u64 << shift;
        mantissa & mask == mask
    } else {
        // Not enough bits for a full mantissa, must be even.
        false
    }
}

/// Convert power-of-two to exact value.
///
/// We will always get an exact representation.
///
/// This works since multiplying by the exponent will not affect the
/// mantissa unless the exponent is denormal, which will cause truncation
/// regardless.
#[cfg(feature = "radix")]
#[inline]
fn pow2_fast_path<F>(mantissa: u64, radix: u32, pow2_exp: i32, exponent: i32)
    -> F
    where F: FloatType
{
    debug_assert!(pow2_exp != 0, "Not a power of 2.");

    // As long as the value is within the bounds, we can get an exact value.
    // Since any power of 2 only affects the exponent, we should be able to get
    // any exact value.

    // We know that if any value is > than max_exp, we get infinity, since
    // the mantissa must be positive. We know that the actual value that
    // causes underflow is 64, use 65 since that prevents inaccurate
    // rounding for any pow2_exp.
    let (min_exp, max_exp) = F::exponent_limit(radix);
    let underflow_exp = min_exp - (65 / pow2_exp);
    if exponent > max_exp {
        F::INFINITY
    } else if exponent < underflow_exp{
        F::ZERO
    } else if exponent < min_exp {
        // We know the mantissa is somewhere <= 65 below min_exp.
        // May still underflow, but it's close. Use the first multiplication
        // which guarantees no truncation, and then the second multiplication
        // which will round to the accurate representation.
        let remainder = exponent - min_exp;
        let float: F = as_cast(mantissa);
        let float = float.pow2(pow2_exp * remainder).pow2(pow2_exp * min_exp);
        float
    } else {
        let float: F = as_cast(mantissa);
        let float = float.pow2(pow2_exp * exponent);
        float
    }
}

/// Convert mantissa to exact value for a non-base2 power.
///
/// Returns the resulting float and if the value can be represented exactly.
#[inline]
fn fast_path<F>(mantissa: u64, radix: u32, exponent: i32)
    -> (F, bool)
    where F: FloatType
{
    debug_assert_radix!(radix);
    debug_assert!(pow2_exponent(radix) == 0, "Cannot use `fast_path` with a power of 2.");

    // `mantissa >> (F::MANTISSA_SIZE+1) != 0` effectively checks if the
    // value has a no bits above the hidden bit, which is what we want.
    let (min_exp, max_exp) = F::exponent_limit(radix);
    let shift_exp = F::mantissa_limit(radix);
    let mantissa_size = F::MANTISSA_SIZE + 1;
    if mantissa >> mantissa_size != 0 {
        // Would require truncation of the mantissa.
        (F::ZERO, false)
    } else {
        if exponent == 0 {
            // 0 exponent, same as value, exact representation.
            let float: F = as_cast(mantissa);
            (float,  true)
        } else if exponent >= min_exp && exponent <= max_exp {
            // Value can be exactly represented, return the value.
            let float: F = as_cast(mantissa);
            let float = float.pow(radix, exponent);
            (float, true)
        } else if exponent >= 0 && exponent <= max_exp + shift_exp {
            // Check to see if we have a disguised fast-path, where the
            // number of digits in the mantissa is very small, but and
            // so digits can be shifted from the exponent to the mantissa.
            // https://www.exploringbinary.com/fast-path-decimal-to-floating-point-conversion/
            let small_powers = get_small_powers_64(radix);
            let shift = exponent - max_exp;
            let power = small_powers[shift.as_usize()];

            // Compute the product of the power, if it overflows,
            // prematurely return early, otherwise, if we didn't overshoot,
            // we can get an exact value.
            mantissa.checked_mul(power)
                .map_or((F::ZERO, false), |v| {
                    if v >> mantissa_size != 0 {
                        (F::ZERO, false)
                    } else {
                        let float: F = as_cast(v);
                        let float = float.pow(radix, max_exp);
                        (float, true)
                    }
                })
        } else {
            // Cannot be exactly represented, exponent too small or too big,
            // would require truncation.
            (F::ZERO, false)
        }
    }
}

// MODERATE
// --------

// Moderate path for the parse algorithm.
//
// In this case, the mantissa can be (partially) represented by an integer,
// however, the exponent or mantissa cannot be fully represented without
// truncating bytes. The moderate path uses a 64-bit integer, while
// the slow path uses a 128-bit integer.
//
// If the value represents only one possible floating-point number, then
// the moderate path is a good approximation. Otherwise, if the generated
// value is close to a halfway representation, use the slow path for
// an exact representation.

// EXTENDED

pub trait FloatErrors: Mantissa {
    /// Get the full error scale.
    fn error_scale() -> u32;
    /// Get the half error scale.
    fn error_halfscale() -> u32;
    /// Determine if the number of errors is tolerable for float precision.
    fn error_is_accurate<F: Float>(count: u32, fp: &ExtendedFloat<Self>, kind: RoundingKind) -> bool;
}

/// Check if the error is accurate with a round-nearest rounding scheme.
#[inline]
fn nearest_error_is_accurate(errors: u64, fp: &ExtendedFloat<u64>, extrabits: u64)
    -> bool
{
    // Round-to-nearest, need to use the halfway point.
    if extrabits == 65 {
        // Underflow, we have a shift larger than the mantissa.
        // Representation is valid **only** if the value is close enough
        // overflow to the next bit within errors. If it overflows,
        // the representation is **not** valid.
        !fp.mant.overflowing_add(errors).1
    } else {
        let mask: u64 = lower_n_mask(extrabits);
        let extra: u64 = fp.mant & mask;

        // Round-to-nearest, need to check if we're close to halfway.
        // IE, b10100 | 100000, where `|` signifies the truncation point.
        let halfway: u64 = lower_n_halfway(extrabits);
        let cmp1 = halfway.wrapping_sub(errors) < extra;
        let cmp2 = extra < halfway.wrapping_add(errors);

        // If both comparisons are true, we have significant rounding error,
        // and the value cannot be exactly represented. Otherwise, the
        // representation is valid.
        !(cmp1 && cmp2)
    }
}

/// Check if the error is accurate with a round-toward rounding scheme.
#[cfg(feature = "rounding")]
#[inline]
fn toward_error_is_accurate(errors: u64, fp: &ExtendedFloat<u64>, extrabits: u64)
    -> bool
{
    if extrabits == 65 {
        // Underflow, we have a literal 0.
        true
    } else {
        let mask: u64 = lower_n_mask(extrabits);
        let extra: u64 = fp.mant & mask;

        // Round-towards, need to use `1 << extrabits`.
        if extrabits == 64 {
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
            let fullway: u64 = nth_bit(extrabits);
            let cmp1 = fullway.wrapping_sub(errors) < extra;
            let cmp2 = extra < fullway.wrapping_add(errors);

            // If both comparisons are true, we have significant rounding error,
            // and the value cannot be exactly represented. Otherwise, the
            // representation is valid.
            !(cmp1 && cmp2)
        }
    }
}

impl FloatErrors for u64 {
    #[inline]
    fn error_scale() -> u32 {
        8
    }

    #[inline]
    fn error_halfscale() -> u32 {
        u64::error_scale() / 2
    }

    #[inline]
    #[allow(unused_variables)]
    fn error_is_accurate<F: Float>(count: u32, fp: &ExtendedFloat<u64>, kind: RoundingKind)
        -> bool
    {
        // Determine if extended-precision float is a good approximation.
        // If the error has affected too many units, the float will be
        // inaccurate, or if the representation is too close to halfway
        // that any operations could affect this halfway representation.
        // See the documentation for dtoa for more information.
        let bias = -(F::EXPONENT_BIAS - F::MANTISSA_SIZE);
        let denormal_exp = bias - 63;
        // This is always a valid u32, since (denormal_exp - fp.exp)
        // will always be positive and the significand size is {23, 52}.
        let extrabits = match fp.exp <= denormal_exp {
            true  => 64 - F::MANTISSA_SIZE + denormal_exp - fp.exp,
            false => 63 - F::MANTISSA_SIZE,
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

        let extrabits = extrabits.as_u64();
        let errors = count.as_u64();
        if extrabits > 65 {
            // Underflow, we have a literal 0.
            return true;
        }

        #[cfg(not(feature = "rounding"))] {
            nearest_error_is_accurate(errors, fp, extrabits)
        }

        #[cfg(feature = "rounding")] {
            if is_nearest(kind) {
                nearest_error_is_accurate(errors, fp, extrabits)
            } else {
                toward_error_is_accurate(errors, fp, extrabits)
            }
        }
    }
}

// 128-bit representation is always accurate, ignore this.
#[cfg(has_i128)]
impl FloatErrors for u128 {
    #[inline]
    fn error_scale() -> u32 {
        0
    }

    #[inline]
    fn error_halfscale() -> u32 {
        0
    }

    #[inline]
    fn error_is_accurate<F: Float>(_: u32, _: &ExtendedFloat<u128>, _: RoundingKind) -> bool {
        // Ignore the halfway problem, use more bits to aim for accuracy,
        // but short-circuit to avoid extremely slow operations.
        true
    }
}

/// Multiply the floating-point by the exponent.
///
/// Multiply by pre-calculated powers of the base, modify the extended-
/// float, and return if new value and if the value can be represented
/// accurately.
#[inline]
fn multiply_exponent_extended<F, M>(fp: &mut ExtendedFloat<M>, radix: u32, exponent: i32, truncated: bool, kind: RoundingKind)
    -> bool
    where M: FloatErrors,
          F: FloatRounding<M>,
          ExtendedFloat<M>: ModeratePathCache<M>
{
    let powers = ExtendedFloat::<M>::get_powers(radix);
    let exponent = exponent + powers.bias;
    let small_index = exponent % powers.step;
    let large_index = exponent / powers.step;
    if exponent < 0 {
        // Guaranteed underflow (assign 0).
        fp.mant = M::ZERO;
        true
    } else if large_index as usize >= powers.large.len() {
        // Overflow (assign infinity)
        fp.mant = M::ONE << 63;
        fp.exp = 0x7FF;
        true
    } else {
        // Within the valid exponent range, multiply by the large and small
        // exponents and return the resulting value.

        // Track errors to as a factor of unit in last-precision.
        let mut errors: u32 = 0;
        if truncated {
            errors += M::error_halfscale();
        }

        // Multiply by the small power.
        // Check if we can directly multiply by an integer, if not,
        // use extended-precision multiplication.
        match fp.mant.overflowing_mul(powers.get_small_int(small_index.as_usize())) {
            // Overflow, multiplication unsuccessful, go slow path.
            (_, true)     => {
                fp.normalize();
                fp.imul(&powers.get_small(small_index.as_usize()));
                errors += M::error_halfscale();
            },
            // No overflow, multiplication successful.
            (mant, false) => {
                fp.mant = mant;
                fp.normalize();
            },
        }

        // Multiply by the large power
        fp.imul(&powers.get_large(large_index.as_usize()));
        if errors > 0 {
            errors += 1;
        }
        errors += M::error_halfscale();

        // Normalize the floating point (and the errors).
        let shift = fp.normalize();
        errors <<= shift;

        M::error_is_accurate::<F>(errors, &fp, kind)
    }
}

/// Create a precise native float using an intermediate extended-precision float.
///
/// Return the float approximation and if the value can be accurately
/// represented with mantissa bits of precision.
#[inline]
pub(super) fn moderate_path<F, M>(mantissa: M, radix: u32, exponent: i32, truncated: bool, kind: RoundingKind)
    -> (ExtendedFloat<M>, bool)
    where M: FloatErrors,
          F: FloatRounding<M> + StablePower,
          ExtendedFloat<M>: ModeratePathCache<M>
{
    let mut fp = ExtendedFloat { mant: mantissa, exp: 0 };
    let valid = multiply_exponent_extended::<F, M>(&mut fp, radix, exponent, truncated, kind);
    (fp, valid)
}

// ATOF/ATOD

/// Parse power-of-two radix string to native float.
#[cfg(feature = "radix")]
#[inline]
fn pow2_to_native<'a, F>(radix: u32, pow2_exp: i32, bytes: &'a [u8], sign: Sign)
    -> (F, &'a [u8])
    where F: FloatType
{
    let (mut mantissa, slc, bytes, truncated) = parse_float::<u64>(radix, bytes);

    // We have a power of 2, can get an exact value even if the mantissa
    // was truncated. Check to see if there are any truncated digits, depending
    // on our rounding scheme.
    let kind = global_rounding(sign);
    let mantissa_size = F::MANTISSA_SIZE + 1;
    if truncated.is_some() {
        if kind != RoundingKind::Downward {
            // See if we need to round-up.
            let bytes = slice_from_range(truncated.unwrap(), bytes.as_ptr());
            let count = bytes.iter().take_while(|&&c| c == b'0' || c == b'.').count();
            let bytes = &bytes[count..];
            let is_truncated = bytes.get(0).map_or(false, |&c| char_to_digit(c).as_u32() < radix);
            if cfg!(feature = "rounding") || kind == RoundingKind::NearestTieEven {
                // Need to check if we're exactly halfway and if there are truncated digits.
                if is_halfway::<F>(mantissa) && is_odd::<F>(mantissa) {
                    mantissa += 1;
                }
            } else if kind == RoundingKind::NearestTieAwayZero {
                // Need to check if we're exactly halfway and if there are truncated digits.
                if is_halfway::<F>(mantissa) {
                    mantissa += 1;
                }
            } else {
                // Need to check if there are any bytes present.
                // Check if there were any truncated bytes.
                if is_truncated {
                    mantissa += 1;
                }
            }
        }

        // Create exact representation and return.
        let exponent = slc.mantissa_exponent().saturating_mul(pow2_exp);
        let fp = ExtendedFloat { mant: mantissa, exp: exponent };
        (fp.into_rounded_float_impl::<F>(kind), bytes)
    } else if mantissa >> mantissa_size != 0 {
        // Would be truncated, use the extended float.
        let exponent = slc.mantissa_exponent().saturating_mul(pow2_exp);
        let fp = ExtendedFloat { mant: mantissa, exp: exponent };
        (fp.into_rounded_float_impl::<F>(kind), bytes)
    } else {
        // Nothing above the hidden bit, so no rounding-error, can use the fast path.
        let float = pow2_fast_path(mantissa, radix, pow2_exp, slc.mantissa_exponent());
        (float, bytes)
    }
}

/// Parse non-power-of-two radix string to native float.
#[inline]
fn pown_to_native<'a, F>(radix: u32, bytes: &'a [u8], lossy: bool, sign: Sign)
    -> (F, &'a [u8])
    where F: FloatType
{
    let (mantissa, slc, bytes, _) = parse_float::<u64>(radix, bytes);
    let exponent = slc.mantissa_exponent();
    let kind = global_rounding(sign);

    if mantissa == 0 {
        // Literal 0, return early.
        // Value cannot be truncated, since we discard leading 0s.
        return (F::ZERO, bytes);
    } else if exponent > 0x40000000 {
        // Extremely large exponent, will always be infinity.
        // Avoid potential overflows in exponent addition.
        return (F::INFINITY, bytes);
    } else if exponent < -0x40000000 {
        // Extremely small exponent, will always be zero.
        // Avoid potential overflows in exponent addition.
        return (F::ZERO, bytes);
    } else if slc.truncated.is_zero() {
        // Try last fast path to exact, no mantissa truncation
        let (float, valid) = fast_path::<F>(mantissa, radix, exponent);
        if valid {
            return (float, bytes);
        }
    }

    // Moderate path (use an extended 80-bit representation).
    let (fp, valid) = moderate_path::<F, _>(mantissa, radix, exponent, slc.truncated != 0, kind);
    if valid || lossy {
        let float = fp.into_rounded_float_impl::<F>(kind);
        return (float, bytes);
    }

    // Slow path
    let b = fp.into_rounded_float_impl::<F>(RoundingKind::Downward);
    if b.is_special() {
        // We have a non-finite number, we get to leave early.
        return (b, bytes);
    } else {
        let float = bhcomp::atof(slc, radix, b, kind);
        return (float, bytes);
    }
}

/// Parse native float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
fn to_native<F>(radix: u32, bytes: &[u8], lossy: bool, sign: Sign)
    -> (F, usize)
    where F: FloatType
{
    #[cfg(not(feature = "radix"))] {
        let (f, slc) = pown_to_native(radix, bytes, lossy, sign);
        (f, bytes.len() - slc.len())
    }

    #[cfg(feature = "radix")] {
        let pow2_exp = pow2_exponent(radix);
        let (f, slc) = match pow2_exp {
            0 => pown_to_native(radix, bytes, lossy, sign),
            _ => pow2_to_native(radix, pow2_exp, bytes, sign),
        };
        (f, bytes.len() - slc.len())
    }
}

/// Parse 32-bit float from string.
#[inline]
pub(crate) fn atof(radix: u32, bytes: &[u8], sign: Sign)
    -> (f32, usize)
{
    to_native::<f32>(radix, bytes, false, sign)
}

/// Parse 64-bit float from string.
#[inline]
pub(crate) fn atod(radix: u32, bytes: &[u8], sign: Sign)
    -> (f64, usize)
{
    to_native::<f64>(radix, bytes, false, sign)
}

/// Parse 32-bit float from string.
#[inline]
pub(crate) fn atof_lossy(radix: u32, bytes: &[u8], sign: Sign)
    -> (f32, usize)
{
    to_native::<f32>(radix, bytes, true, sign)
}

/// Parse 64-bit float from string.
#[inline]
pub(crate) fn atod_lossy(radix: u32, bytes: &[u8], sign: Sign)
    -> (f64, usize)
{
    to_native::<f64>(radix, bytes, true, sign)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use arrayvec;
    use lib::str;
    use util::test::*;
    use super::*;

    #[test]
    fn scientific_exponent_test() {
        // Check "1.2345", simple.
        let slc = FloatSlice {
            integer: "1".as_bytes(),
            fraction: "2345".as_bytes(),
            digits_start: 0,
            digits_end: 4,
            truncated: 0,
            raw_exponent: 0,
        };
        assert_eq!(slc.scientific_exponent(), 0);

        // Check "0.12345", simple.
        let slc = FloatSlice {
            integer: "".as_bytes(),
            fraction: "12345".as_bytes(),
            digits_start: 0,
            digits_end: 5,
            truncated: 0,
            raw_exponent: 0,
        };
        assert_eq!(slc.scientific_exponent(), -1);
    }

    // PARSE MANTISSA

    fn check_parse_mantissa<M>(radix: u32, s: &str, tup: (M, usize, usize, usize, usize, &str))
        where M: Mantissa
    {
        let (value, slc, bytes, _) = parse_mantissa::<M>(radix, s.as_bytes());
        let digits: arrayvec::ArrayVec<[u8; 1024]> = slc.mantissa_iter().cloned().collect();
        let digits = str::from_utf8(&digits).unwrap();
        assert_eq!(value, tup.0);
        assert_eq!(slc.integer_len(), tup.1);
        assert_eq!(slc.fraction_len(), tup.2);
        assert_eq!(slc.truncated_digits(), tup.3);
        assert_eq!(distance(s.as_ptr(), bytes.as_ptr()), tup.4);
        assert_eq!(digits, tup.5);
    }

    #[test]
    fn parse_mantissa_test() {
        // 64-bit
        check_parse_mantissa::<u64>(10, "1.2345", (12345, 1, 4, 0, 6, "12345"));
        check_parse_mantissa::<u64>(10, "12.345", (12345, 2, 3, 0, 6, "12345"));
        check_parse_mantissa::<u64>(10, "12345.6789", (123456789, 5, 4, 0, 10, "123456789"));
        check_parse_mantissa::<u64>(10, "1.2345e10", (12345, 1, 4, 0, 6, "12345"));
        check_parse_mantissa::<u64>(10, "0.0000000000000000001", (1, 0, 19, 0, 21, "1"));
        check_parse_mantissa::<u64>(10, "0.00000000000000000000000000001", (1, 0, 29, 0, 31, "1"));
        check_parse_mantissa::<u64>(10, "100000000000000000000", (10000000000000000000, 21, 0, 1, 21, "100000000000000000000"));

        // Adapted from failures in strtod.
        check_parse_mantissa::<u64>(10, "179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", (17976931348623158079, 309, 70, 359, 380, "1797693134862315807937289714053034150799341327100378269361737789804449682927647509466490179775872070963302864166928879109465555478519404026306574886715058206819089020007083836762738548458177115317644757302700698555713669596228429148198608349364752927190741684443655107043427115596995080930428801779041744977919999999999999999999999999999999999999999999999999999999999999999999999"));

        // Rounding error
        // Adapted from test-float-parse failures.
        check_parse_mantissa::<u64>(10, "1009e-31", (1009, 4, 0, 0, 4, "1009"));

        // 128-bit
        check_parse_mantissa::<u128>(10, "1.2345", (12345, 1, 4, 0, 6,  "12345"));
        check_parse_mantissa::<u128>(10, "12.345", (12345, 2, 3, 0, 6,  "12345"));
        check_parse_mantissa::<u128>(10, "12345.6789", (123456789, 5, 4, 0, 10,  "123456789"));
        check_parse_mantissa::<u128>(10, "1.2345e10", (12345, 1, 4, 0, 6,  "12345"));
        check_parse_mantissa::<u128>(10, "0.0000000000000000001", (1, 0, 19, 0, 21,  "1"));
        check_parse_mantissa::<u128>(10, "0.00000000000000000000000000001", (1, 0, 29, 0, 31,  "1"));
        check_parse_mantissa::<u128>(10, "100000000000000000000", (100000000000000000000, 21, 0, 0, 21,  "100000000000000000000"));
    }

    fn check_parse_float<M>(radix: u32, s: &str, tup: (M, i32, i32, usize, usize, bool, &str))
        where M: Mantissa
    {
        let (value, slc, bytes, truncated) = parse_float::<M>(radix, s.as_bytes());
        let digits: arrayvec::ArrayVec<[u8; 1024]> = slc.mantissa_iter().cloned().collect();
        let digits = str::from_utf8(&digits).unwrap();
        assert_eq!(value, tup.0);
        assert_eq!(slc.mantissa_exponent(), tup.1);
        assert_eq!(slc.scientific_exponent(), tup.2);
        assert_eq!(slc.mantissa_digits(), tup.3);
        assert_eq!(distance(s.as_ptr(), bytes.as_ptr()), tup.4);
        assert_eq!(truncated.is_some(), tup.5);
        assert_eq!(digits, tup.6);
        assert_eq!(digits.len(), slc.mantissa_digits());
    }

    #[test]
    fn parse_float_test() {
        // 64-bit
        check_parse_float::<u64>(10, "1.2345", (12345, -4, 0, 5, 6, false, "12345"));
        check_parse_float::<u64>(10, "12.345", (12345, -3, 1, 5, 6, false, "12345"));
        check_parse_float::<u64>(10, "12345.6789", (123456789, -4, 4, 9, 10, false, "123456789"));
        check_parse_float::<u64>(10, "1.2345e10", (12345, 6, 10, 5, 9, false, "12345"));
        check_parse_float::<u64>(10, "100000000000000000000", (10000000000000000000, 1, 20, 21, 21, true, "100000000000000000000"));
        check_parse_float::<u64>(10, "100000000000000000001", (10000000000000000000, 1, 20, 21, 21, true, "100000000000000000001"));

        // Adapted from failures in strtod.
        check_parse_float::<u64>(10, "179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", (17976931348623158079, 289, 308, 379, 380, true, "1797693134862315807937289714053034150799341327100378269361737789804449682927647509466490179775872070963302864166928879109465555478519404026306574886715058206819089020007083836762738548458177115317644757302700698555713669596228429148198608349364752927190741684443655107043427115596995080930428801779041744977919999999999999999999999999999999999999999999999999999999999999999999999"));

        // Rounding error
        // Adapted from test-float-parse failures.
        check_parse_float::<u64>(10, "1009e-31", (1009, -31, -28, 4, 8, false, "1009"));

        // 128-bit
        check_parse_float::<u128>(10, "1.2345", (12345, -4, 0, 5, 6, false, "12345"));
        check_parse_float::<u128>(10, "12.345", (12345, -3, 1, 5, 6, false, "12345"));
        check_parse_float::<u128>(10, "12345.6789", (123456789, -4, 4, 9, 10, false, "123456789"));
        check_parse_float::<u128>(10, "1.2345e10", (12345, 6, 10, 5, 9, false, "12345"));
        check_parse_float::<u128>(10, "100000000000000000000", (100000000000000000000, 0, 20, 21, 21, false, "100000000000000000000"));
        check_parse_float::<u128>(10, "100000000000000000001", (100000000000000000001, 0, 20, 21, 21, false, "100000000000000000001"));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn is_odd_test() {
        // Variant of b1000000000000000000000001, a halfway value for f32.
        assert!(is_odd::<f32>(0x1000002));
        assert!(is_odd::<f32>(0x2000004));
        assert!(is_odd::<f32>(0x8000010000000000));
        assert!(!is_odd::<f64>(0x1000002));
        assert!(!is_odd::<f64>(0x2000004));
        assert!(!is_odd::<f64>(0x8000010000000000));

        assert!(!is_odd::<f32>(0x1000001));
        assert!(!is_odd::<f32>(0x2000002));
        assert!(!is_odd::<f32>(0x8000008000000000));
        assert!(!is_odd::<f64>(0x1000001));
        assert!(!is_odd::<f64>(0x2000002));
        assert!(!is_odd::<f64>(0x8000008000000000));

        // Variant of b100000000000000000000000000000000000000000000000000001,
        // a halfway value for f64
        assert!(!is_odd::<f32>(0x3f000000000002));
        assert!(!is_odd::<f32>(0x3f000000000003));
        assert!(!is_odd::<f32>(0xFC00000000000800));
        assert!(!is_odd::<f32>(0xFC00000000000C00));
        assert!(is_odd::<f64>(0x3f000000000002));
        assert!(is_odd::<f64>(0x3f000000000003));
        assert!(is_odd::<f64>(0xFC00000000000800));
        assert!(is_odd::<f64>(0xFC00000000000C00));

        assert!(!is_odd::<f32>(0x3f000000000001));
        assert!(!is_odd::<f32>(0x3f000000000004));
        assert!(!is_odd::<f32>(0xFC00000000000400));
        assert!(!is_odd::<f32>(0xFC00000000001000));
        assert!(!is_odd::<f64>(0x3f000000000001));
        assert!(!is_odd::<f64>(0x3f000000000004));
        assert!(!is_odd::<f64>(0xFC00000000000400));
        assert!(!is_odd::<f64>(0xFC00000000001000));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn is_halfway_test() {
        // Variant of b1000000000000000000000001, a halfway value for f32.
        assert!(is_halfway::<f32>(0x1000001));
        assert!(is_halfway::<f32>(0x2000002));
        assert!(is_halfway::<f32>(0x8000008000000000));
        assert!(!is_halfway::<f64>(0x1000001));
        assert!(!is_halfway::<f64>(0x2000002));
        assert!(!is_halfway::<f64>(0x8000008000000000));

        // Variant of b10000000000000000000000001, which is 1-off a halfway value.
        assert!(!is_halfway::<f32>(0x2000001));
        assert!(!is_halfway::<f64>(0x2000001));

        // Variant of b100000000000000000000000000000000000000000000000000001,
        // a halfway value for f64
        assert!(!is_halfway::<f32>(0x20000000000001));
        assert!(!is_halfway::<f32>(0x40000000000002));
        assert!(!is_halfway::<f32>(0x8000000000000400));
        assert!(is_halfway::<f64>(0x20000000000001));
        assert!(is_halfway::<f64>(0x40000000000002));
        assert!(is_halfway::<f64>(0x8000000000000400));

        // Variant of b111111000000000000000000000000000000000000000000000001,
        // a halfway value for f64.
        assert!(!is_halfway::<f32>(0x3f000000000001));
        assert!(!is_halfway::<f32>(0xFC00000000000400));
        assert!(is_halfway::<f64>(0x3f000000000001));
        assert!(is_halfway::<f64>(0xFC00000000000400));

        // Variant of b1000000000000000000000000000000000000000000000000000001,
        // which is 1-off a halfway value.
        assert!(!is_halfway::<f32>(0x40000000000001));
        assert!(!is_halfway::<f64>(0x40000000000001));
    }

    #[cfg(feature = "radix")]
    #[test]
    fn float_pow2_fast_path() {
        // Everything is valid.
        let mantissa = 1 << 63;
        for base in BASE_POW2.iter().cloned() {
            let (min_exp, max_exp) = f32::exponent_limit(base);
            let pow2_exp = pow2_exponent(base);
            for exp in min_exp-20..max_exp+30 {
                // Always valid, ignore result
                pow2_fast_path::<f32>(mantissa, base, pow2_exp, exp);
            }
        }
    }

    #[cfg(feature = "radix")]
    #[test]
    fn double_pow2_fast_path_test() {
        // Everything is valid.
        let mantissa = 1 << 63;
        for base in BASE_POW2.iter().cloned() {
            let (min_exp, max_exp) = f64::exponent_limit(base);
            let pow2_exp = pow2_exponent(base);
            for exp in min_exp-20..max_exp+30 {
                // Ignore result, always valid
                pow2_fast_path::<f64>(mantissa, base, pow2_exp, exp);
            }
        }
    }

    #[test]
    fn float_fast_path_test() {
        // valid
        let mantissa = (1 << f32::MANTISSA_SIZE) - 1;
        for base in BASE_POWN.iter().cloned() {
            let (min_exp, max_exp) = f32::exponent_limit(base);
            for exp in min_exp..max_exp+1 {
                let (_, valid) = fast_path::<f32>(mantissa, base, exp);
                assert!(valid, "should be valid {:?}.", (mantissa, base, exp));
            }
        }

        // Check slightly above valid exponents
        let (f, valid) = fast_path::<f32>(123, 10, 15);
        assert_eq!(f, 1.23e+17);
        assert!(valid);

        // Exponent is 1 too high, pushes over the mantissa.
        let (_, valid) = fast_path::<f32>(123, 10, 16);
        assert!(!valid);

        // Mantissa is too large, checked_mul should overflow.
        let (_, valid) = fast_path::<f32>(mantissa, 10, 11);
        assert!(!valid);

        // invalid mantissa
        #[cfg(feature = "radix")] {
            let (_, max_exp) = f64::exponent_limit(3);
            let (_, valid) = fast_path::<f32>(1<<f32::MANTISSA_SIZE, 3, max_exp+1);
            assert!(!valid, "invalid mantissa");
        }

        // invalid exponents
        for base in BASE_POWN.iter().cloned() {
            let (min_exp, max_exp) = f32::exponent_limit(base);
            let (_, valid) = fast_path::<f32>(mantissa, base, min_exp-1);
            assert!(!valid, "exponent under min_exp");

            let (_, valid) = fast_path::<f32>(mantissa, base, max_exp+1);
            assert!(!valid, "exponent above max_exp");
        }
    }

    #[test]
    fn double_fast_path_test() {
        // valid
        let mantissa = (1 << f64::MANTISSA_SIZE) - 1;
        for base in BASE_POWN.iter().cloned() {
            let (min_exp, max_exp) = f64::exponent_limit(base);
            for exp in min_exp..max_exp+1 {
                let (_, valid) = fast_path::<f64>(mantissa, base, exp);
                assert!(valid, "should be valid {:?}.", (mantissa, base, exp));
            }
        }

        // invalid mantissa
        #[cfg(feature = "radix")] {
            let (_, max_exp) = f64::exponent_limit(3);
            let (_, valid) = fast_path::<f64>(1<<f64::MANTISSA_SIZE, 3, max_exp+1);
            assert!(!valid, "invalid mantissa");
        }

        // invalid exponents
        for base in BASE_POWN.iter().cloned() {
            let (min_exp, max_exp) = f64::exponent_limit(base);
            let (_, valid) = fast_path::<f64>(mantissa, base, min_exp-1);
            assert!(!valid, "exponent under min_exp");

            let (_, valid) = fast_path::<f64>(mantissa, base, max_exp+1);
            assert!(!valid, "exponent above max_exp");
        }
    }

    #[cfg(feature = "radix")]
    #[test]
    fn float_moderate_path_test() {
        // valid (overflowing small mult)
        let mantissa: u64 = 1 << 63;
        let (f, valid) = moderate_path::<f32, _>(mantissa, 3, 1, false, RoundingKind::NearestTieEven);
        assert_eq!(f.into_f32(), 2.7670116e+19);
        assert!(valid, "exponent should be valid");

        let mantissa: u64 = 4746067219335938;
        let (f, valid) = moderate_path::<f32, _>(mantissa, 15, -9, false, RoundingKind::NearestTieEven);
        assert_eq!(f.into_f32(), 123456.1);
        assert!(valid, "exponent should be valid");
    }

    #[cfg(feature = "radix")]
    #[test]
    fn double_moderate_path_test() {
        // valid (overflowing small mult)
        let mantissa: u64 = 1 << 63;
        let (f, valid) = moderate_path::<f64, _>(mantissa, 3, 1, false, RoundingKind::NearestTieEven);
        assert_eq!(f.into_f64(), 2.7670116110564327e+19);
        assert!(valid, "exponent should be valid");

        // valid (ends of the earth, salting the earth)
        let (f, valid) = moderate_path::<f64, _>(mantissa, 3, -695, true, RoundingKind::NearestTieEven);
        assert_eq!(f.into_f64(), 2.32069302345e-313);
        assert!(valid, "exponent should be valid");

        // invalid ("268A6.177777778", base 15)
        let mantissa: u64 = 4746067219335938;
        let (_, valid) = moderate_path::<f64, _>(mantissa, 15, -9, false, RoundingKind::NearestTieEven);
        assert!(!valid, "exponent should be invalid");

        // valid ("268A6.177777778", base 15)
        // 123456.10000000001300614743687445, exactly, should not round up.
        let mantissa: u128 = 4746067219335938;
        let (f, valid) = moderate_path::<f64, _>(mantissa, 15, -9, false, RoundingKind::NearestTieEven);
        assert_eq!(f.into_f64(), 123456.1);
        assert!(valid, "exponent should be valid");

        // Rounding error
        // Adapted from test-float-parse failures.
        let mantissa: u64 = 1009;
        let (_, valid) = moderate_path::<f64, _>(mantissa, 10, -31, false, RoundingKind::NearestTieEven);
        assert!(!valid, "exponent should be valid");
    }

    fn check_atof(radix: u32, s: &str, tup: (f32, usize)) {
        let (value, len) = atof(radix, s.as_bytes(), Sign::Positive);
        assert_f32_eq!(value, tup.0);
        assert_eq!(len, tup.1);
    }

    #[test]
    fn atof_test() {
        check_atof(10, "1.2345", (1.2345, 6));
        check_atof(10, "12.345", (12.345, 6));
        check_atof(10, "12345.6789", (12345.6789, 10));
        check_atof(10, "1.2345e10", (1.2345e10, 9));
        check_atof(10, "1.2345e-38", (1.2345e-38, 10));

        // Check expected rounding, using borderline cases.
        // Round-down, halfway
        check_atof(10, "16777216", (16777216.0, 8));
        check_atof(10, "16777217", (16777216.0, 8));
        check_atof(10, "16777218", (16777218.0, 8));
        check_atof(10, "33554432", (33554432.0, 8));
        check_atof(10, "33554434", (33554432.0, 8));
        check_atof(10, "33554436", (33554436.0, 8));
        check_atof(10, "17179869184", (17179869184.0, 11));
        check_atof(10, "17179870208", (17179869184.0, 11));
        check_atof(10, "17179871232", (17179871232.0, 11));

        // Round-up, halfway
        check_atof(10, "16777218", (16777218.0, 8));
        check_atof(10, "16777219", (16777220.0, 8));
        check_atof(10, "16777220", (16777220.0, 8));
        check_atof(10, "33554436", (33554436.0, 8));
        check_atof(10, "33554438", (33554440.0, 8));
        check_atof(10, "33554440", (33554440.0, 8));
        check_atof(10, "17179871232", (17179871232.0, 11));
        check_atof(10, "17179872256", (17179873280.0, 11));
        check_atof(10, "17179873280", (17179873280.0, 11));

        // Round-up, above halfway
        check_atof(10, "33554435", (33554436.0, 8));
        check_atof(10, "17179870209", (17179871232.0, 11));

        // Check exactly halfway, round-up at halfway
        check_atof(10, "1.00000017881393432617187499", (1.0000001, 28));
        check_atof(10, "1.000000178813934326171875", (1.0000002, 26));
        check_atof(10, "1.00000017881393432617187501", (1.0000002, 28));
    }

    fn check_atod(radix: u32, s: &str, tup: (f64, usize)) {
        let (value, len) = atod(radix, s.as_bytes(), Sign::Positive);
        assert_f64_eq!(value, tup.0);
        assert_eq!(len, tup.1);
    }

    #[test]
    fn atod_test() {
        check_atod(10, "1.2345", (1.2345, 6));
        check_atod(10, "12.345", (12.345, 6));
        check_atod(10, "12345.6789", (12345.6789, 10));
        check_atod(10, "1.2345e10", (1.2345e10, 9));
        check_atod(10, "1.2345e-308", (1.2345e-308, 11));

        // Check expected rounding, using borderline cases.
        // Round-down, halfway
        check_atod(10, "9007199254740992", (9007199254740992.0, 16));
        check_atod(10, "9007199254740993", (9007199254740992.0, 16));
        check_atod(10, "9007199254740994", (9007199254740994.0, 16));
        check_atod(10, "18014398509481984", (18014398509481984.0, 17));
        check_atod(10, "18014398509481986", (18014398509481984.0, 17));
        check_atod(10, "18014398509481988", (18014398509481988.0, 17));
        check_atod(10, "9223372036854775808", (9223372036854775808.0, 19));
        check_atod(10, "9223372036854776832", (9223372036854775808.0, 19));
        check_atod(10, "9223372036854777856", (9223372036854777856.0, 19));
        check_atod(10, "11417981541647679048466287755595961091061972992", (11417981541647679048466287755595961091061972992.0, 47));
        check_atod(10, "11417981541647680316116887983825362587765178368", (11417981541647679048466287755595961091061972992.0, 47));
        check_atod(10, "11417981541647681583767488212054764084468383744", (11417981541647681583767488212054764084468383744.0, 47));

        // Round-up, halfway
        check_atod(10, "9007199254740994", (9007199254740994.0, 16));
        check_atod(10, "9007199254740995", (9007199254740996.0, 16));
        check_atod(10, "9007199254740996", (9007199254740996.0, 16));
        check_atod(10, "18014398509481988", (18014398509481988.0, 17));
        check_atod(10, "18014398509481990", (18014398509481992.0, 17));
        check_atod(10, "18014398509481992", (18014398509481992.0, 17));
        check_atod(10, "9223372036854777856", (9223372036854777856.0, 19));
        check_atod(10, "9223372036854778880", (9223372036854779904.0, 19));
        check_atod(10, "9223372036854779904", (9223372036854779904.0, 19));
        check_atod(10, "11417981541647681583767488212054764084468383744", (11417981541647681583767488212054764084468383744.0, 47));
        check_atod(10, "11417981541647682851418088440284165581171589120", (11417981541647684119068688668513567077874794496.0, 47));
        check_atod(10, "11417981541647684119068688668513567077874794496", (11417981541647684119068688668513567077874794496.0, 47));

        // Round-up, above halfway
        check_atod(10, "9223372036854776833", (9223372036854777856.0, 19));
        check_atod(10, "11417981541647680316116887983825362587765178369", (11417981541647681583767488212054764084468383744.0, 47));

        // Rounding error
        // Adapted from failures in strtod.
        check_atod(10, "2.2250738585072014e-308", (2.2250738585072014e-308, 23));
        check_atod(10, "2.2250738585072011360574097967091319759348195463516456480234261097248222220210769455165295239081350879141491589130396211068700864386945946455276572074078206217433799881410632673292535522868813721490129811224514518898490572223072852551331557550159143974763979834118019993239625482890171070818506906306666559949382757725720157630626906633326475653000092458883164330377797918696120494973903778297049050510806099407302629371289589500035837999672072543043602840788957717961509455167482434710307026091446215722898802581825451803257070188608721131280795122334262883686223215037756666225039825343359745688844239002654981983854879482922068947216898310996983658468140228542433306603398508864458040010349339704275671864433837704860378616227717385456230658746790140867233276367187499e-308", (2.225073858507201e-308, 776));
        check_atod(10, "2.22507385850720113605740979670913197593481954635164564802342610972482222202107694551652952390813508791414915891303962110687008643869459464552765720740782062174337998814106326732925355228688137214901298112245145188984905722230728525513315575501591439747639798341180199932396254828901710708185069063066665599493827577257201576306269066333264756530000924588831643303777979186961204949739037782970490505108060994073026293712895895000358379996720725430436028407889577179615094551674824347103070260914462157228988025818254518032570701886087211312807951223342628836862232150377566662250398253433597456888442390026549819838548794829220689472168983109969836584681402285424333066033985088644580400103493397042756718644338377048603786162277173854562306587467901408672332763671875e-308", (2.2250738585072014e-308, 774));
        check_atod(10, "2.2250738585072011360574097967091319759348195463516456480234261097248222220210769455165295239081350879141491589130396211068700864386945946455276572074078206217433799881410632673292535522868813721490129811224514518898490572223072852551331557550159143974763979834118019993239625482890171070818506906306666559949382757725720157630626906633326475653000092458883164330377797918696120494973903778297049050510806099407302629371289589500035837999672072543043602840788957717961509455167482434710307026091446215722898802581825451803257070188608721131280795122334262883686223215037756666225039825343359745688844239002654981983854879482922068947216898310996983658468140228542433306603398508864458040010349339704275671864433837704860378616227717385456230658746790140867233276367187501e-308", (2.2250738585072014e-308, 776));
        check_atod(10, "179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999", (1.7976931348623157e+308, 380));
        check_atod(10, "7.4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984374999e-324", (5e-324, 761));
        check_atod(10, "7.4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984375e-324", (1e-323, 758));
        check_atod(10, "7.4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984375001e-324", (1e-323, 761));

        // Rounding error
        // Adapted from:
        //  https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/
        #[cfg(feature = "radix")]
        check_atod(2, "1e-10000110010", (5e-324, 14));

        #[cfg(feature = "radix")]
        check_atod(2, "1e-10000110011", (0.0, 14));
        check_atod(10, "0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000024703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125", (0.0, 1077));

        // Rounding error
        // Adapted from:
        //  https://www.exploringbinary.com/how-glibc-strtod-works/
        check_atod(10, "0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072008890245868760858598876504231122409594654935248025624400092282356951787758888037591552642309780950434312085877387158357291821993020294379224223559819827501242041788969571311791082261043971979604000454897391938079198936081525613113376149842043271751033627391549782731594143828136275113838604094249464942286316695429105080201815926642134996606517803095075913058719846423906068637102005108723282784678843631944515866135041223479014792369585208321597621066375401613736583044193603714778355306682834535634005074073040135602968046375918583163124224521599262546494300836851861719422417646455137135420132217031370496583210154654068035397417906022589503023501937519773030945763173210852507299305089761582519159720757232455434770912461317493580281734466552734375", (2.2250738585072011e-308, 1076));

        // Rounding error
        // Adapted from test-float-parse failures.
        check_atod(10, "1009e-31", (1.009e-28, 8));
        check_atod(10, "18294e304", (f64::INFINITY, 9));

        // Rounding error
        // Adapted from a @dangrabcad's issue #20.
        check_atod(10, "7.689539722041643e164", (7.689539722041643e164, 21));
        check_atod(10, "768953972204164300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", (7.689539722041643e164, 165));
        check_atod(10, "768953972204164300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0", (7.689539722041643e164, 167));

        // Check other cases similar to @dangrabcad's issue #20.
        check_atod(10, "9223372036854776833.0", (9223372036854777856.0, 21));
        check_atod(10, "11417981541647680316116887983825362587765178369.0", (11417981541647681583767488212054764084468383744.0, 49));
        check_atod(10, "9007199254740995.0", (9007199254740996.0, 18));
        check_atod(10, "18014398509481990.0", (18014398509481992.0, 19));
        check_atod(10, "9223372036854778880.0", (9223372036854779904.0, 21));
        check_atod(10, "11417981541647682851418088440284165581171589120.0", (11417981541647684119068688668513567077874794496.0, 49));
    }

    // Lossy

    fn check_atof_lossy(radix: u32, s: &str, tup: (f32, usize)) {
        let (value, len) = atof_lossy(radix, s.as_bytes(), Sign::Positive);
        assert_f32_eq!(value, tup.0);
        assert_eq!(len, tup.1);
    }

    #[test]
    fn atof_lossy_test() {
        check_atof_lossy(10, "1.2345", (1.2345, 6));
        check_atof_lossy(10, "12.345", (12.345, 6));
        check_atof_lossy(10, "12345.6789", (12345.6789, 10));
        check_atof_lossy(10, "1.2345e10", (1.2345e10, 9));
    }

    fn check_atod_lossy(radix: u32, s: &str, tup: (f64, usize)) {
        let (value, len) = atod_lossy(radix, s.as_bytes(), Sign::Positive);
        assert_f64_eq!(value, tup.0);
        assert_eq!(len, tup.1);
    }

    #[test]
    fn atod_lossy_test() {
        check_atod_lossy(10, "1.2345", (1.2345, 6));
        check_atod_lossy(10, "12.345", (12.345, 6));
        check_atod_lossy(10, "12345.6789", (12345.6789, 10));
        check_atod_lossy(10, "1.2345e10", (1.2345e10, 9));
    }
}
