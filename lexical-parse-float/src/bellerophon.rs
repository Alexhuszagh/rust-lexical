//! An implementation of Clinger's Bellerophon algorithm.
//!
//! This is a moderate path algorithm that uses an extended-precision
//! float, represented in 80 bits, by calculating the bits of slop
//! and determining if those bits could prevent unambiguous rounding.
//!
//! This algorithm requires less static storage than the Lemire algorithm,
//! and has decent performance, and is therefore used when non-decimal,
//! non-power-of-two strings need to be parsed. Clinger's algorithm
//! is described in depth in "How to Read Floating Point Numbers Accurately.",
//! available online [here](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.45.4152&rep=rep1&type=pdf).
//!
//! This implementation is loosely based off the Golang implementation,
//! found [here](https://github.com/golang/go/blob/b10849fbb97a2244c086991b4623ae9f32c212d0/src/strconv/extfloat.go).
//! This code is therefore subject to a 3-clause BSD license.

#![cfg(not(feature = "compact"))]
#![cfg(feature = "radix")]
#![doc(hidden)]

use crate::float::{ExtendedFloat80, LemireFloat};
use crate::number::Number;
use crate::table::bellerophon_powers;
use lexical_util::format::NumberFormat;

// ALGORITHM
// ---------

/// Core implementation of the Bellerophon algorithm.
///
/// Create an extended-precision float, scale it to the proper radix power,
/// calculate the bits of slop, and return the representation. The value
/// will always be guaranteed to be within 1 bit, rounded-down, of the real
/// value. If a negative exponent is returned, this represents we were
/// unable to unambiguously round the significant digits.
///
/// This has been modified to return a biased, rather than unbiased exponent.
#[inline]
pub fn bellerophon<F: LemireFloat, const FORMAT: u128>(num: &Number) -> ExtendedFloat80 {
    let format = NumberFormat::<{ FORMAT }> {};
    let fp_zero = ExtendedFloat80 {
        mant: 0,
        exp: 0,
    };
    let fp_inf = ExtendedFloat80 {
        mant: 0,
        exp: F::INFINITE_POWER,
    };

    // Calculate our indexes for our extended-precision multiplication.
    let powers = bellerophon_powers(format.radix());
    let exponent = (num.exponent as i32).saturating_add(powers.bias);
    let small_index = exponent % powers.step;
    let large_index = exponent / powers.step;

    if exponent < 0 {
        // Guaranteed underflow (assign 0).
        return fp_zero;
    }
    if large_index as usize >= powers.large.len() {
        // Overflow (assign infinity)
        return fp_inf;
    }

    // Within the valid exponent range, multiply by the large and small
    // exponents and return the resulting value.

    // Track errors to as a factor of unit in last-precision.
    let mut errors: u32 = 0;
    if num.many_digits {
        errors += error_halfscale();
    }

    // Multiply by the small power.
    // Check if we can directly multiply by an integer, if not,
    // use extended-precision multiplication.
    let mut fp = ExtendedFloat80 {
        mant: num.mantissa,
        exp: 0,
    };
    match fp.mant.overflowing_mul(powers.get_small_int(small_index as usize)) {
        // Overflow, multiplication unsuccessful, go slow path.
        (_, true) => {
            normalize(&mut fp);
            fp = mul(&fp, &powers.get_small(small_index as usize));
            errors += error_halfscale();
        },
        // No overflow, multiplication successful.
        (mant, false) => {
            fp.mant = mant;
            normalize(&mut fp);
        },
    }

    // Multiply by the large power.
    fp = mul(&fp, &powers.get_large(large_index as usize));
    if errors > 0 {
        errors += 1;
    }
    errors += error_halfscale();

    // Normalize the floating point (and the errors).
    let shift = normalize(&mut fp);
    errors <<= shift;

    if !error_is_accurate::<F>(errors, &fp) {
        fp.exp = -1;
    }
    fp
}

// ERRORS
// ------

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
const fn error_scale() -> u32 {
    8
}

/// Get the half error scale.
#[inline]
const fn error_halfscale() -> u32 {
    error_scale() / 2
}

/// Determine if the number of errors is tolerable for float precision.
#[inline]
fn error_is_accurate<F: LemireFloat>(errors: u32, fp: &ExtendedFloat80) -> bool {
    // Determine if extended-precision float is a good approximation.
    // If the error has affected too many units, the float will be
    // inaccurate, or if the representation is too close to halfway
    // that any operations could affect this halfway representation.
    // See the documentation for dtoa for more information.
    let full = 64;
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
    let maskbits = extrabits as u64;
    let errors = errors as u64;

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

// MATH
// ----

/// Normalize float-point number.
///
/// Shift the mantissa so the number of leading zeros is 0, or the value
/// itself is 0.
///
/// Get the number of bytes shifted.
pub fn normalize(fp: &mut ExtendedFloat80) -> i32 {
    // Note:
    // Using the ctlz intrinsic via leading_zeros is way faster (~10x)
    // than shifting 1-bit at a time, via while loop, and also way
    // faster (~2x) than an unrolled loop that checks at 32, 16, 4,
    // 2, and 1 bit.
    //
    // Using a modulus of pow2 (which will get optimized to a bitwise
    // and with 0x3F or faster) is slightly slower than an if/then,
    // however, removing the if/then will likely optimize more branched
    // code as it removes conditional logic.

    // Calculate the number of leading zeros, and then zero-out
    // any overflowing bits, to avoid shl overflow when self.mant == 0.
    if fp.mant != 0 {
        let shift = fp.mant.leading_zeros() as i32;
        fp.mant <<= shift;
        fp.exp -= shift;
        shift
    } else {
        0
    }
}

/// Multiply two normalized extended-precision floats, as if by `a*b`.
///
/// The precision is maximal when the numbers are normalized, however,
/// decent precision will occur as long as both values have high bits
/// set. The result is not normalized.
///
/// Algorithm:
///     1. Non-signed multiplication of mantissas (requires 2x as many bits as input).
///     2. Normalization of the result (not done here).
///     3. Addition of exponents.
pub fn mul(x: &ExtendedFloat80, y: &ExtendedFloat80) -> ExtendedFloat80 {
    // Logic check, values must be decently normalized prior to multiplication.
    debug_assert!(x.mant >> 32 != 0);
    debug_assert!(y.mant >> 32 != 0);

    // Extract high-and-low masks.
    const LOMASK: u64 = u32::MAX as u64;
    let x1 = x.mant >> 32;
    let x0 = x.mant & LOMASK;
    let y1 = y.mant >> 32;
    let y0 = y.mant & LOMASK;

    // Get our products
    let x1_y0 = x1 * y0;
    let x0_y1 = x0 * y1;
    let x0_y0 = x0 * y0;
    let x1_y1 = x1 * y1;

    let mut tmp = (x1_y0 & LOMASK) + (x0_y1 & LOMASK) + (x0_y0 >> 32);
    // round up
    tmp += 1 << (32 - 1);

    ExtendedFloat80 {
        mant: x1_y1 + (x1_y0 >> 32) + (x0_y1 >> 32) + (tmp >> 32),
        exp: x.exp + y.exp + u64::BITS as i32,
    }
}

/// Generate a bitwise mask for the lower `n` bits.
///
/// # Examples
///
/// ```
/// # use lexical_parse_float::bellerophon::lower_n_mask;
/// # pub fn main() {
/// assert_eq!(lower_n_mask(2), 0b11);
/// # }
/// ```
#[inline]
pub fn lower_n_mask(n: u64) -> u64 {
    debug_assert!(n <= 64, "lower_n_mask() overflow in shl.");

    match n == 64 {
        true => u64::MAX,
        false => (1 << n) - 1,
    }
}

/// Calculate the halfway point for the lower `n` bits.
///
/// # Examples
///
/// ```
/// # use lexical_parse_float::bellerophon::lower_n_halfway;
/// # pub fn main() {
/// assert_eq!(lower_n_halfway(2), 0b10);
/// # }
/// ```
#[inline]
pub fn lower_n_halfway(n: u64) -> u64 {
    debug_assert!(n <= 64, "lower_n_halfway() overflow in shl.");

    match n == 0 {
        true => 0,
        false => nth_bit(n - 1),
    }
}

/// Calculate a scalar factor of 2 above the halfway point.
///
/// # Examples
///
/// ```text
/// # use lexical_parse_float::bellerophon::nth_bit;
/// # pub fn main() {
/// assert_eq!(nth_bit(2), 0b100);
/// # }
/// ```
#[inline]
pub fn nth_bit(n: u64) -> u64 {
    debug_assert!(n < 64, "nth_bit() overflow in shl.");
    1 << n
}
