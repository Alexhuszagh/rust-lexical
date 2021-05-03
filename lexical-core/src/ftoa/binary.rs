//! Optimized float serializer for radixes powers of 2.

#![cfg(feature = "binary")]

use crate::itoa;
use crate::traits::*;
use crate::util::*;

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

// The write is therefore quite simple:
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

/// Round up the value to the next multiple of base.
/// Preconditions: value and base must be >= 0.
#[inline(always)]
fn round_up(value: i32, base: i32) -> i32 {
    match value % base {
        0 => value,
        rem => value + base - rem,
    }
}

/// Calculate the fast ceiling, integral division.
#[inline(always)]
fn fast_ceildiv(value: i32, base: i32) -> i32 {
    round_up(value, base) / base
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

// FTOA
// ----

/// Write float to string with exponent notation.
#[inline]
fn ftoa_exponent<'a, Mant: UnsignedInteger>(
    radix: u32,
    exponent_base: u32,
    exponent_radix: u32,
    bits_per_digit: i32,
    bytes: &'a mut [u8],
    format: NumberFormat,
    mantissa: Mant,
    exp: i32,
    sci_exp: i32,
) -> usize
where
    Mant: itoa::Itoa,
{
    // Config options
    let decimal_point = format.decimal_point();
    let exponent_character = format.exponent_backup();

    // BUFFER

    // Write our digits to a temporary buffer.
    // We can assume we don't need more than 256, if we're future-proofing
    // for 256-bit integers. This is because although denormal values
    // can have more bits for the exponent, they lose bits
    // in the significand.
    const SIZE: usize = 260;
    let mut buffer: [u8; SIZE] = [b'\0'; SIZE];
    let shl = calculate_shl(exp, bits_per_digit);
    let value = mantissa << shl;
    let mut count = itoa::itoa_positive(value, radix, &mut buffer);
    count -= rtrim_char_slice(&buffer[..count], b'0').1;
    let digits = &buffer[..count];

    // Write our significant digits to the output.
    bytes[0] = digits[0];
    bytes[1] = decimal_point;
    let mut cursor: usize = 2;
    if digits.len() == 1 {
        bytes[2] = b'0';
        cursor += 1;
    } else {
        cursor += copy_to_dst(&mut bytes[2..], &digits[1..]);
    }

    // Now, write our exponent.
    let scaled_sci_exp = scale_sci_exp(sci_exp, exponent_base);
    bytes[cursor] = exponent_character;
    cursor += 1;
    let scaled_sci_exp_u32: u32;
    if sci_exp < 0 {
        bytes[cursor] = b'-';
        cursor += 1;
        scaled_sci_exp_u32 = scaled_sci_exp.wrapping_neg() as u32;
    } else {
        scaled_sci_exp_u32 = scaled_sci_exp as u32;
    }
    cursor += itoa::itoa_positive(scaled_sci_exp_u32, exponent_radix, &mut bytes[cursor..]);

    cursor
}

/// Write float to string without exponent notation.
///
/// Has a negative exponent (shift right) and no exponent notation.
#[inline]
fn ftoa_negative_no_exponent<'a, Mant: UnsignedInteger>(
    radix: u32,
    bits_per_digit: i32,
    bytes: &'a mut [u8],
    format: NumberFormat,
    mantissa: Mant,
    exp: i32,
    sci_exp: i32,
) -> usize
where
    Mant: itoa::Itoa,
{
    // Config options
    let decimal_point = format.decimal_point();
    let exponent_character = format.exponent_backup();

    // The number of 0 bits we need to pad left (reducing the
    // exponent) is just the negative scientific exponent.
    // We then need to calculate the number of zero digits
    // from this, remembering that we're padding left,
    // so for example, `1/2` in hex is represented as `0.8`.
    // That means we need the `⌈ zero_bits / bits_per_digit ⌉`.
    let zero_bits = sci_exp.wrapping_neg();
    let zero_digits = fast_ceildiv(zero_bits, bits_per_digit) as usize;

    // Write our 0 digits.
    bytes[0] = b'0';
    bytes[1] = decimal_point;
    bytes[2..zero_digits + 1].fill(b'0');
    let mut cursor = zero_digits + 1;

    // Generate our digits after the shift.
    let shl = calculate_shl(exp, bits_per_digit);
    cursor += itoa::itoa_positive(mantissa << shl, radix, &mut bytes[cursor..]);
    cursor -= rtrim_char_slice(&bytes[..cursor], b'0').1;

    // Write a decimal point if required for the notation.
    if cfg!(feature = "format") && format.required_exponent_notation() {
        bytes[cursor] = exponent_character;
        bytes[cursor + 1] = b'0';
        cursor += 2;
    }

    cursor
}

/// Write float to string without exponent notation.
///
/// Has a positive exponent (shift left) and no exponent notation.
#[inline]
fn ftoa_positive_no_exponent<'a, Mant: UnsignedInteger>(
    radix: u32,
    bits_per_digit: i32,
    bytes: &'a mut [u8],
    format: NumberFormat,
    mantissa: Mant,
    exp: i32,
    sci_exp: i32,
) -> usize
where
    Mant: itoa::Itoa,
{
    // Config options
    let decimal_point = format.decimal_point();
    let exponent_character = format.exponent_backup();

    // BUFFER

    // Shift our mantissa into place and write our digits to a temporary buffer.
    // We can assume we don't need more than 256, if we're future-proofing
    // for 256-bit integers. This is because although denormal values
    // can have more bits for the exponent, they lose bits
    // in the significand.
    const SIZE: usize = 260;
    let mut buffer: [u8; SIZE] = [b'\0'; SIZE];
    let shl = calculate_shl(exp, bits_per_digit);
    let value = mantissa << shl;
    let mut count = itoa::itoa_positive(value, radix, &mut buffer);
    count -= rtrim_char_slice(&buffer[..count], b'0').1;
    let digits = &buffer[..count];

    // Calculate the number of digits we can write left of the decimal point.
    // If we have a scientific exp of 0, we still need
    // to write 1 digit before, so it's ⌊ leading_bits / bits_per_digit ⌋ + 1.
    let leading_bits = sci_exp;
    let leading_digits = (leading_bits / bits_per_digit) as usize + 1;

    // Write our significant digits to the output.
    let mut cursor: usize = 0;
    if leading_digits >= digits.len() {
        // Need to write our digits, then trailing 0s, and a decimal point.
        copy_to_dst(bytes, &digits);
        bytes[digits.len()..leading_digits].fill(b'0');

        // Write the decimal point and trailing 0.
        cursor = leading_digits;
        bytes[cursor] = decimal_point;
        cursor += 1;
        bytes[cursor] = b'0';
        cursor += 1;
    } else {
        // No trailing digits, write to buffer then continue.
        cursor += copy_to_dst(bytes, &digits[..leading_digits]);
        bytes[cursor] = decimal_point;
        cursor += 1;
        cursor += copy_to_dst(&mut bytes[cursor..], &digits[leading_digits..]);
    }

    // Write a decimal point if required for the notation.
    if cfg!(feature = "format") && format.required_exponent_notation() {
        bytes[cursor] = exponent_character;
        bytes[cursor + 1] = b'0';
        cursor += 2;
    }

    cursor
}

/// Optimized float-to-string algorithm for power of 2 radixes.
///
/// This assumes the float is:
///     1). Non-zero
///     2). Non-special (NaN or Infinite).
///     3). Non-negative.
fn ftoa<'a, F: Float>(
    float: F,
    radix: u32,
    exponent_base: u32,
    exponent_radix: u32,
    bytes: &'a mut [u8],
    format: NumberFormat,
) -> usize
where
    <F as Float>::Unsigned: itoa::Itoa,
{
    // PRECONDITIONS

    // Assert no special cases remain, no non-zero values,
    // and no negative numbers, and a valid radix.
    debug_assert_radix!(radix);
    debug_assert!(!float.is_special());
    debug_assert!(!float.is_zero());
    debug_assert!(float > Float::ZERO);

    // Quickly calculate the number of bits we would have written.
    // This simulates writing the digits, so we can calculate the
    // scientific exponent. Since this number is often constant
    // (except for denormal values), and doesn't describe
    // the actual shift or digits we use...
    //
    // Note:
    //      Except for denormal floats, this will always
    //      be `F::MANTISSA_SIZE`.
    let mantissa = float.mantissa();
    let mantissa_bits = significant_bits(mantissa) as i32;

    // See if we should use an exponent if the number was represented
    // in scientific notation, AKA, `I.FFFF^EEE`. If the exponent is above
    // a certain value, then use scientific notation. We therefore have
    // to adjust the exponent by the number of mantissa bits, and shift
    // by 1 (since a scientific exponent of 0 should have 1 digit ahead).
    // This is a binary exp, so we need to how large our
    // adjusted exp to the radix is.
    //
    // The scientific exponent is always this way: it's the float exponent
    // (binary) + mantissa bits (binary) - 1 (for the first bit, binary),
    // since 1.0 is scientific exponent 0. We need to check the scientific
    // exponent relative to the number of leading or trailing 0s
    // it would introduce, that is, scaled to bits/digit.
    let bits_per_digit = log2(radix) as i32;
    let exp = float.exponent();
    let sci_exp = exp + mantissa_bits - 1;
    if float <= as_cast(1e-5) || float >= as_cast(1e9) {
        ftoa_exponent(
            radix,
            exponent_base,
            exponent_radix,
            bits_per_digit,
            bytes,
            format,
            mantissa,
            exp,
            sci_exp,
        )
    } else {
        // Don't use an exponent. Write the digits, scaled to the exponent.
        if sci_exp < 0 {
            ftoa_negative_no_exponent(radix, bits_per_digit, bytes, format, mantissa, exp, sci_exp)
        } else {
            ftoa_positive_no_exponent(radix, bits_per_digit, bytes, format, mantissa, exp, sci_exp)
        }
    }
}

/// Fast implementation for f32. Names exist so we don't need trait dependencies.
#[inline(always)]
pub(crate) fn float_binary<'a>(
    float: f32,
    radix: u32,
    bytes: &'a mut [u8],
    format: NumberFormat,
) -> usize {
    ftoa(float, radix, radix, radix, bytes, format)
}

/// Fast implementation for f64. Names exist so we don't need trait dependencies.
#[inline(always)]
pub(crate) fn double_binary<'a>(
    float: f64,
    radix: u32,
    bytes: &'a mut [u8],
    format: NumberFormat,
) -> usize {
    ftoa(float, radix, radix, radix, bytes, format)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftoa() {
        let mut buffer = new_buffer();
        let format = NumberFormat::STANDARD;

        // Check writing characters before and after the decimal point,
        // without an exponent.
        let count = ftoa(1.2345678901234567890f32, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.0011110000001100101001"));

        // Check writing multiple characters before the decimal point,
        // without an exponent.
        let count = ftoa(3.2345678901234567890f32, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("11.0011110000001100101001"));

        // Check writing characters before the decimal point, without
        // any filled zeros.
        let count = ftoa(1f32, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.0"));

        // Check writing values with a negative sci_exp, that is, writing
        // digits only after the decimal point, without an exponent.
        let count = ftoa(0.2345678901234567890f32, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.00111100000011001010010001"));

        // Check writing values with a negative sci_exp, that is, writing
        // digits only after the decimal point, without any filled zeros,
        // without an exponent.
        let count = ftoa(0.7345678901234567890f32, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.1011110000001100101001"));

        // Now need to write with an exponent.
        // Let's try a denormal first.
        let count = ftoa(1.4e-45f32, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.0^-10010101"));

        // Now need to write with an exponent.
        // Let's try the max value.
        let count = ftoa(3.4028234664e38f32, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.11111111111111111111111^1111111"));
    }

    #[test]
    fn test_dtoa() {
        let mut buffer = new_buffer();
        let format = NumberFormat::STANDARD;

        // All these tests check at least **5** multiples of 2
        // for each value, since that allows us to test all
        // shifts of a given value, since the maximum log2(radix)
        // is 5. All of these values are verified using the Python
        // `to_float` implementation above.

        // POSITIVE SCI EXP, NO EXPONENT

        // Check writing values with a positive sci_exp, that is, writing
        // digits before and after the decimal point, without an exponent.

        // Binary
        let count = ftoa(0.2345678901234567890e2f64, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("10111.0111010011110000000111111110110100110010011001"));

        let count = ftoa(0.1172839450617284e2f64, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1011.10111010011110000000111111110110100110010011001"));

        let count = ftoa(0.0586419725308642e2f64, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("101.110111010011110000000111111110110100110010011001"));

        let count = ftoa(0.0293209862654321e2f64, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("10.1110111010011110000000111111110110100110010011001"));

        let count = ftoa(0.01466049313271605e2f64, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.01110111010011110000000111111110110100110010011001"));

        // Base4
        let count = ftoa(0.2345678901234567890e2f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("113.13103300013332310302121"));

        let count = ftoa(0.1172839450617284e2f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("23.232213200033331221210302"));

        let count = ftoa(0.0586419725308642e2f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("11.313103300013332310302121"));

        let count = ftoa(0.0293209862654321e2f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("2.3232213200033331221210302"));

        let count = ftoa(0.01466049313271605e2f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.1313103300013332310302121"));

        // Octal
        let count = ftoa(0.2345678901234567890e2f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("27.3517003773231144"));

        let count = ftoa(0.1172839450617284e2f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("13.5647401775514462"));

        let count = ftoa(0.0586419725308642e2f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("5.6723600776646231"));

        let count = ftoa(0.0293209862654321e2f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("2.73517003773231144"));

        let count = ftoa(0.01466049313271605e2f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.35647401775514462"));

        // Hexadecimal
        let count = ftoa(0.2345678901234567890e2f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("17.74F01FED3264"));

        let count = ftoa(0.1172839450617284e2f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("B.BA780FF69932"));

        let count = ftoa(0.0586419725308642e2f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("5.DD3C07FB4C99"));

        let count = ftoa(0.0293209862654321e2f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("2.EE9E03FDA64C8"));

        let count = ftoa(0.01466049313271605e2f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.774F01FED3264"));

        // Base32
        let count = ftoa(0.2345678901234567890e2f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("N.EJO1VR9ICG"));

        let count = ftoa(0.1172839450617284e2f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("B.N9S0VTKP68"));

        let count = ftoa(0.0586419725308642e2f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("5.RKU0FUQCJ4"));

        let count = ftoa(0.0293209862654321e2f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("2.TQF07VD69I"));

        let count = ftoa(0.01466049313271605e2f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.ET7G3VMJ4P"));

        // Different exponent base.
        let count = ftoa(0.2345678901234567890e2f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("N.EJO1VR9ICG"));

        // Different exponent radix.
        let count = ftoa(0.2345678901234567890e2f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("N.EJO1VR9ICG"));

        // Need to test when we have more leading digits than digits.
        let count = ftoa(1024.0f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("100.0"));

        // NEGATIVE SCI EXP, NO EXPONENT

        // Check writing values with a negative sci_exp, that is, writing
        // digits only after the decimal point, without an exponent.

        // Binary
        let count = ftoa(0.2345678901234567890f64, 2, 2, 2, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.00111100000011001010010000101000110001011001111110111"));

        let count = ftoa(0.1172839450617284f64, 2, 2, 2, &mut buffer, format);
        assert_eq!(
            &buffer[..count],
            b!("0.000111100000011001010010000101000110001011001111110111")
        );

        let count = ftoa(0.0586419725308642f64, 2, 2, 2, &mut buffer, format);
        assert_eq!(
            &buffer[..count],
            b!("0.0000111100000011001010010000101000110001011001111110111")
        );

        // Base4
        let count = ftoa(0.2345678901234567890f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.033000302210022030112133232"));

        let count = ftoa(0.1172839450617284f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.013200121102011012023033313"));

        let count = ftoa(0.0586419725308642f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0033000302210022030112133232"));

        let count = ftoa(0.0293209862654321f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0013200121102011012023033313"));

        let count = ftoa(0.01466049313271605f64, 4, 4, 4, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.00033000302210022030112133232"));

        // Octal
        let count = ftoa(0.2345678901234567890f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.170062441214263756"));

        let count = ftoa(0.1172839450617284f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.074031220506131767"));

        let count = ftoa(0.0586419725308642f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0360145102430547734"));

        let count = ftoa(0.0293209862654321f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0170062441214263756"));

        let count = ftoa(0.01466049313271605f64, 8, 8, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0074031220506131767"));

        // Hexadecimal
        let count = ftoa(0.2345678901234567890f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.3C0CA428C59FB8"));

        let count = ftoa(0.1172839450617284f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.1E06521462CFDC"));

        let count = ftoa(0.0586419725308642f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0F03290A3167EE"));

        let count = ftoa(0.0293209862654321f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0781948518B3F7"));

        let count = ftoa(0.01466049313271605f64, 16, 16, 16, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.03C0CA428C59FB8"));

        // Base32
        let count = ftoa(0.2345678901234567890f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.7G6A8A65JUS"));

        let count = ftoa(0.1172839450617284f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.3O354532PVE"));

        let count = ftoa(0.0586419725308642f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.1S1II2HHCVN"));

        let count = ftoa(0.0293209862654321f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0U0P918OMFRG"));

        let count = ftoa(0.01466049313271605f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.0F0CKGKCB7TO"));

        // Different exponent base.
        let count = ftoa(0.2345678901234567890f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.7G6A8A65JUS"));

        // Different exponent radix.
        let count = ftoa(0.2345678901234567890f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("0.7G6A8A65JUS"));

        // NEGATIVE SCI EXP, WITH EXPONENT

        // Check writing a value with a negative scientific exponent,
        // where we need to use scientific notation.
        let count = ftoa(0.2345678901234567890e-40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.0M6KNHH73N8^-R"));

        let count = ftoa(0.1172839450617284e-40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("G.B3ABOOJHRO^-S"));

        let count = ftoa(0.0586419725308642e-40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("8.5HL5SC9OTS^-S"));

        let count = ftoa(0.0293209862654321e-40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("4.2OQIU64SEU^-S"));

        let count = ftoa(0.01466049313271605e-40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("2.1CD9F32E7F^-S"));

        // Different exponent base.
        let count = ftoa(0.2345678901234567890e-40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.0M6KNHH73N8^-47"));

        let count = ftoa(0.1172839450617284e-40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("G.B3ABOOJHRO^-48"));

        let count = ftoa(0.0586419725308642e-40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("8.5HL5SC9OTS^-49"));

        let count = ftoa(0.0293209862654321e-40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("4.2OQIU64SEU^-4A"));

        let count = ftoa(0.01466049313271605e-40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("2.1CD9F32E7F^-4B"));

        // Different exponent radix.
        let count = ftoa(0.2345678901234567890e-40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.0M6KNHH73N8^-207"));

        let count = ftoa(0.1172839450617284e-40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("G.B3ABOOJHRO^-210"));

        let count = ftoa(0.0586419725308642e-40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("8.5HL5SC9OTS^-211"));

        let count = ftoa(0.0293209862654321e-40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("4.2OQIU64SEU^-212"));

        let count = ftoa(0.01466049313271605e-40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("2.1CD9F32E7F^-213"));

        // POSITIVE SCI EXP, WITH EXPONENT

        // Check writing a value with a positive scientific exponent,
        // where we need to use scientific notation.
        let count = ftoa(0.2345678901234567890e40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.N4M59DCAVIO^Q"));

        let count = ftoa(0.1172839450617284e40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("R.IB2KMM5FPC^P"));

        let count = ftoa(0.0586419725308642e40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("D.P5HABB2NSM^P"));

        let count = ftoa(0.0293209862654321e40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("6.SIOL5LHBUB^P"));

        let count = ftoa(0.01466049313271605e40f64, 32, 32, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("3.E9CAIQOLV5G^P"));

        // Different exponent base.
        let count = ftoa(0.2345678901234567890e40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.N4M59DCAVIO^42"));

        let count = ftoa(0.1172839450617284e40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("R.IB2KMM5FPC^41"));

        let count = ftoa(0.0586419725308642e40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("D.P5HABB2NSM^40"));

        let count = ftoa(0.0293209862654321e40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("6.SIOL5LHBUB^3V"));

        let count = ftoa(0.01466049313271605e40f64, 32, 2, 32, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("3.E9CAIQOLV5G^3U"));

        // Different exponent radix.
        let count = ftoa(0.2345678901234567890e40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("1.N4M59DCAVIO^202"));

        let count = ftoa(0.1172839450617284e40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("R.IB2KMM5FPC^201"));

        let count = ftoa(0.0586419725308642e40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("D.P5HABB2NSM^200"));

        let count = ftoa(0.0293209862654321e40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("6.SIOL5LHBUB^177"));

        let count = ftoa(0.01466049313271605e40f64, 32, 2, 8, &mut buffer, format);
        assert_eq!(&buffer[..count], b!("3.E9CAIQOLV5G^176"));
    }
}
