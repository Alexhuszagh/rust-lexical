//! Optimized float serializer for radixes powers of 2.

#![cfg(feature = "power_of_two")]

use lexical_util::num::UnsignedInteger;

// ALGORITHM
// ---------

// PARSER
//
// The simple, parser algorithm can be thought of like this:
// ```python
// import fractions
// import numpy as np
//
// def parse_binary(integer, fraction, exponent, radix, dtype):
//     '''Parse a binary (or power of 2) integer to float'''
//
//     # Parse our simple values
//     iint = int(integer, radix)
//     ifrac = int(fraction, radix)
//     iexp = int(exponent, radix)
//
//     # Calculate our actual values
//     fint_num = iint * 2**iexp
//     fint = fractions.Fraction(fint_num)
//
//     if len(fraction) > iexp:
//         ffrac_exp_num = 0
//         ffrac_exp_den = len(fraction) - iexp
//     else:
//         ffrac_exp_num = iexp - len(fraction)
//         ffrac_exp_den = 0
//     ffrac_num = ifrac * 2**ffrac_exp_num
//     ffrac_den = 2**ffrac_exp_den
//     ffrac = fractions.Fraction(ffrac_num, ffrac_den)
//
//     return dtype(fint + ffrac)
//
// parse_binary('1', '001111000000110010', '0', 2, np.float32)
// ```
//
// For binary floats, we can get a step further, assuming the value
// is in the proper range for the exponent.
//
// Please note that the real implementation is much faster,
// but for simple floats, this suffices.
//
// A another example is as follows. This closely tracks our internal logic
// for converting extended-precision floats to native floats and creates
// an exact result.
//
// ```python
// import sys
// import numpy as np
//
//
// def into_float_bits(mant, exp, finfo):
//     '''Converts a mantissa and exponent (binary) into a float.'''
//      # Export floating-point number.
//     if mant == 0 or exp < finfo.DENORMAL_EXPONENT:
//         # sub-denormal, underflow
//         return 0
//     elif exp >= finfo.MAX_EXPONENT:
//         return INFINITY_BITS
//     else:
//         if exp == finfo.DENORMAL_EXPONENT and mant & finfo.HIDDEN_BIT_MASK == 0:
//             exp = 0
//         else:
//             exp += finfo.EXPONENT_BIAS
//         exp <<= finfo.MANTISSA_SIZE
//         mant &= finfo.MANTISSA_MASK
//         return mant | exp
//
//
// def into_float(mant, exp, finfo, is_positive):
//     '''Converts a mantissa, exponent, and sign into a float.'''
//
//     bits = into_float_bits(mant, exp, finfo)
//     if not is_positive:
//         bits |= SIGN_MASK
//
//     as_bytes = bits.to_bytes(finfo.BITS // 8, sys.byteorder)
//     return np.frombuffer(as_bytes, dtype=finfo.ftype)[0]
//
//
// def ctlz(integer, finfo):
//     '''Count the leading zeros on an integer'''
//
//     bits = [0] * finfo.BITS
//     for bit in range(0, finfo.BITS):
//         bits[bit] = (integer >> bit) & 1
//
//     count = 0
//     bit = 31
//     while bit >= 0 and bits[bit] == 0:
//         count += 1
//         bit -= 1
//
//     return count
//
//
// def normalize(mant, exp, finfo):
//     '''Normalizes the float.'''
//
//     if mant == 0:
//         shift = 0
//     else:
//         shift = ctlz(mant, finfo)
//
//     mant <<= shift
//     exp -= shift
//
//     return mant, exp
//
//
// def lower_n_mask(n):
//     '''Generate a bitwise mask for the lower `n` bits.'''
//     return (1 << n) - 1
//
//
// def nth_bit(n):
//     '''Calculate a scalar factor of 2 above the halfway point.'''
//     return 1 << n
//
//
// def lower_n_halfway(n):
//     '''Calculate the halfway point for the lower `n` bits.'''
//
//     if n == 0:
//         return 0
//     return nth_bit(n - 1)
//
//
// def internal_n_mask(bit, n):
//     '''Calculate a bitwise mask with `n` 1 bits starting at the `bit` position.'''
//     return lower_n_mask(bit) ^ lower_n_mask(bit - n)
//
//
// def round_nearest(mant, exp, finfo, shift):
//     '''Shift right N-bytes and round to the nearest.'''
//
//     mask = lower_n_mask(shift)
//     halfway = lower_n_halfway(shift)
//
//     truncated_bits = mant & mask;
//     is_above = truncated_bits > halfway;
//     is_halfway = truncated_bits == halfway;
//
//     if shift == finfo.BITS:
//         mant = 0
//     else:
//         mant >>= shift
//     exp += shift
//
//     return (mant, exp, is_above, is_halfway)
//
//
// def tie_even(mant, exp, is_above, is_halfway):
//     '''Tie rounded floating point to even.'''
//
//     is_odd = mant & 0x1 == 0x1
//     if is_above or (is_odd and is_halfway):
//         mant += 1
//     return mant, exp
//
//
// def round_nearest_tie_even(mant, exp, finfo, shift):
//     '''Shift right N-bytes and round nearest, tie-to-even.'''
//
//     mant, exp, is_above, is_halfway = round_nearest(mant, exp, finfo, shift)
//     return tie_even(mant, exp, is_above, is_halfway)
//
//
// def round_to_float(mant, exp, finfo):
//     '''Round the float to native, using round-nearest, tie-even.'''
//
//     final_exp = exp + finfo.DEFAULT_SHIFT
//     if final_exp < finfo.DENORMAL_EXPONENT:
//         diff = finfo.DENORMAL_EXPONENT - exp
//         if diff < finfo.BITS:
//             mant, exp = round_nearest_tie_even(mant, exp, finfo, diff)
//         else:
//             return (0, 0)
//     else:
//         mant, exp = round_nearest_tie_even(mant, exp, finfo, finfo.DEFAULT_SHIFT)
//
//     if mant & finfo.CARRY_MASK == finfo.CARRY_MASK:
//         mant >>= 1
//         exp += 1
//
//     return mant, exp
//
//
// def avoid_overflow(mant, exp, finfo):
//     '''Avoid overflow for large values, shift left as needed.'''
//
//     if exp > finfo.MAX_EXPONENT:
//         diff = exp - MAX_EXPONENT
//         if diff < finfo.MANTISSA_SIZE:
//             bit = MANTISSA_SIZE + 1
//             n = diff + 1
//             mask = internal_n_mask(bit, n)
//             if mant & mask == 0:
//                 shift = diff + 1
//                 mant <<= shift
//                 exp -= shift
//     return mant, exp
//
//
// def round_to_native(mant, exp, finfo):
//     '''Round float to representation for conversion.'''
//
//     mant, exp = normalize(mant, exp, finfo)
//     mant, exp = round_to_float(mant, exp, finfo)
//     mant, exp = avoid_overflow(mant, exp, finfo)
//
//     return mant, exp
//
//
// def parse_binary(integer, fraction, exponent, finfo):
//     '''Parses a binary number'''
//
//     is_positive = True
//     if len(integer) > 0 and integer[0] == '-':
//         integer = integer[1:]
//         is_positive = False
//     elif len(integer) > 0 and integer[0] == '+':
//         integer = integer[1:]
//     mant = int(integer + fraction, 2)
//     exp = int(exponent, 2) - len(fraction)
//     mant, exp = round_to_native(mant, exp, finfo)
//     return into_float(mant, exp, finfo, is_positive)
//
//
// class f32info:
//     ftype = np.float32
//     itype = np.uint32
//     BITS = 32
//     SIGN_MASK = 0x80000000
//     EXPONENT_MASK = 0x7F800000
//     HIDDEN_BIT_MASK = 0x00800000
//     MANTISSA_MASK = 0x007FFFFF
//     INFINITY_BITS = 0x7F800000
//     MANTISSA_SIZE = 23
//     EXPONENT_BIAS = 127 + MANTISSA_SIZE
//     DENORMAL_EXPONENT = 1 - EXPONENT_BIAS
//     MAX_EXPONENT = 0xFF - EXPONENT_BIAS
//     DEFAULT_SHIFT = BITS - MANTISSA_SIZE - 1
//     CARRY_MASK = 0x1000000
//
// parse_binary('1', '001111000000110010', '0', f32info) -> 1.2345657
// parse_binary('1', '001111000000110001', '0', f32info) -> 1.2345619
// parse_binary('1', '001111000000110011', '0', f32info) -> 1.2345695
// parse_binary('1', '0011110000001100101', '0', f32info) -> 1.2345676
// parse_binary('1', '0011110000001100101001', '0', f32info) -> 1.2345679
// parse_binary('1100', '010110000111111', '0', f32info) -> 12.345673
// parse_binary('1100', '01011000011111100111', '0', f32info) -> 12.345673
// ```

// WRITER

// The writer is therefore quite simple:
//  1). Write the significant digits, using an itoa algorithm for the radix.
//      We never need to backtrack, because we cannot have rounding error.
//  2). Calculate the scientific exponent for the value.
//      This is exponent + mantissa digits.
//  3). Determine if we should use exponent notation.
//  4). Write the digits, scaled by the exponent, to the buffer.
//  5). Write the exponent character and the scientific exponent, if applicable.
//
// We can validate a written value (without an exponent)
// with the following:
//
// ```python
// def to_float(value, radix, exp_base=None, exp_radix=None):
//     '''Convert a string to float.'''
//
//     if exp_base is None:
//         exp_base = radix
//     if exp_radix is None:
//         exp_radix = radix
//
//     integer_digits, fraction_digits = value.split('.')
//     integer = int(integer_digits, radix)
//
//     split = fraction_digits.split('^')
//     if len(split) == 2:
//         fraction_digits, exponent_digits = split
//         fraction = int(fraction_digits, radix)
//         exponent = int(exponent_digits, exp_radix)
//     else:
//         fraction = int(fraction_digits, radix)
//         exponent = 0
//
//     mantissa = integer + (fraction * radix**(-len(fraction_digits)))
//     return mantissa * exp_base**exponent
// ```

// MATH
// ----

/// Calculate the number of significant bits in an integer.
/// This is just `1-ctlz(value)`.
#[inline(always)]
fn significant_bits<T: UnsignedInteger>(value: T) -> u32 {
    T::BITS as u32 - value.leading_zeros()
}

/// Calculate the fast ceiling, integral division.
#[inline(always)]
fn fast_ceildiv(value: i32, base: i32) -> i32 {
    debug_assert!(value >= 0);
    (value + base - 1) / base
}

/// Get the inverse remainder to calculate the modulus.
///
/// For negative numbers, the remainder is is essentially:
///     `⌊-x / y⌋ = z, r = -x - (y * z)`
///     `⌊-21 / 4⌋ = -5, r = -21 - (4 * -5), or r = -1`
///
/// We, however, want the modulus to calculate the left-shift
/// necessary. The modulus for negative numbers is effectively:
///     `⌈-x / y⌉ = z, r = -x - (y * z)`
///     `⌈-21 / 4⌉ = -6, r = -21 - (4 * -6), or r = 3`
#[inline(always)]
fn inverse_remainder(remainder: i32, base: i32) -> i32 {
    debug_assert!(remainder >= 0);
    match remainder {
        0 => 0,
        rem => base - rem,
    }
}

/// We need to calculate the shift to align the
/// highest mantissa bit. This is based off aligning
/// the bits to the exponent, so we can get it on
/// an exponent boundary. For example, at exp = -55,
/// we get a shl of 1 from the modulus `(-55 % 4)`.
/// Since rust doesn't support the modulus operation,
/// just remainder, we can use the `inverse_remainder`
/// function to swap between the remainder of the inverse
/// to the modulus. Likewise, at `(-53 % 4) == 3`,
/// `-55 % 3 == 2`, and `-53 % 5 == 2`, all numbers
/// we expect.
///
/// For positive numbers, this is quite simple: `(48 % 5) => == 3`,
/// which is just the modulus operation as well (which for positive
/// numbers, this is identical to Rust's remainder operator).
#[inline(always)]
fn calculate_shl(exp: i32, bits_per_digit: i32) -> i32 {
    if exp < 0 {
        let shr = exp.wrapping_neg() % bits_per_digit;
        inverse_remainder(shr, bits_per_digit)
    } else {
        exp % bits_per_digit
    }
}

/// We need to scale the scientific exponent for writing.
///
/// If we have a negative exp, then when scaling that,
/// we need to consider that an exp of -1 with 5 bits
/// per base is still <0, IE, the sci exp we write has
/// to be: ⌊sci_exp / bits_per_base⌋, where ceil is
/// wrapping towards greatest magnitude.
///
/// If we have a positive exp, we just need the floor of the
/// `sci_exp / bits_per_base`, because if we had an exp of 1 with
/// 5 bits, that would behind the decimal point.
#[inline(always)]
pub fn scale_sci_exp(sci_exp: i32, exponent_base: u32) -> i32 {
    let bits_per_base = log2(exponent_base);
    if sci_exp < 0 {
        let neg_sci_exp = sci_exp.wrapping_neg();
        fast_ceildiv(neg_sci_exp, bits_per_base).wrapping_neg()
    } else {
        sci_exp / bits_per_base
    }
}

// ALGORITHM
// ---------
