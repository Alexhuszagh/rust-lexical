//! Optimized float serializer for radixes powers of 2.
//!
//! Note: this requires the mantissa radix and the
//! exponent base to be the same. See [hex](crate::hex) for
//! when the mantissa radix and the exponent base are different.

#![cfg(feature = "power-of-two")]
#![doc(hidden)]

use crate::options::{Options, RoundMode};
use crate::shared;
use lexical_util::algorithm::rtrim_char_count;
use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
use lexical_util::format::NumberFormat;
use lexical_util::num::{as_cast, Float, Integer, UnsignedInteger};
use lexical_write_integer::write::WriteInteger;

/// Optimized float-to-string algorithm for power of 2 radixes.
///
/// This assumes the float is:
///     1). Non-special (NaN or Infinite).
///     2). Non-negative.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of
/// significant digits, any (optional) leading or trailing zeros,
/// and the scientific exponent.
///
/// # Panics
///
/// Panics if exponent notation is used, and the exponent base and
/// mantissa radix are not the same in `FORMAT`.
pub unsafe fn write_float<F: Float, const FORMAT: u128>(
    float: F,
    bytes: &mut [u8],
    options: &Options,
) -> usize
where
    <F as Float>::Unsigned: WriteInteger + FormattedSize,
{
    // PRECONDITIONS

    // Assert no special cases remain, no negative numbers,
    // and a valid format.
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    debug_assert!(!float.is_special());
    debug_assert!(float >= F::ZERO);

    // Quickly calculate the number of bits we would have written.
    // This simulates writing the digits, so we can calculate the
    // scientific exponent. Since this number is often constant
    // (except for denormal values), and doesn't describe
    // the actual shift or digits we use...
    //
    // Note:
    //      Except for denormal floats, this will always
    //      be `F::MANTISSA_SIZE`, unless we have special
    //      formatting write control.
    let mantissa = float.mantissa();
    let radix = format.mantissa_radix();
    let (mantissa, mantissa_bits) = truncate_and_round(mantissa, radix, options);

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
    // it would introduce, that is, scaled to bits/digit. The min exp must
    // be less than, and the max must be above 0.
    let exp = float.exponent();
    let mut sci_exp = exp + mantissa_bits as i32 - 1;

    // Normalize the exponent if we have an actual zero.
    if mantissa == <F as Float>::Unsigned::ZERO {
        sci_exp = 0;
    }

    write_float!(
        FORMAT,
        sci_exp,
        options,
        write_float_scientific,
        write_float_positive_exponent,
        write_float_negative_exponent,
        generic => _,
        args => mantissa, exp, sci_exp, bytes, options,
    )
}

/// Write float to string in scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of digits
/// and the scientific notation's exponent digits.
///
/// # Preconditions
///
/// The mantissa must be truncated and rounded, prior to calling this,
/// based on the number of maximum digits. In addition, `exponent_base`
/// and `mantissa_radix` in `FORMAT` must be identical.
#[inline]
pub unsafe fn write_float_scientific<M, const FORMAT: u128>(
    mantissa: M,
    exp: i32,
    sci_exp: i32,
    bytes: &mut [u8],
    options: &Options,
) -> usize
where
    M: WriteInteger + FormattedSize,
{
    // Just decent size bounds checks to ensure we have a lot of space.
    assert!(M::FORMATTED_SIZE < BUFFER_SIZE - 2);
    debug_assert!(bytes.len() >= BUFFER_SIZE);

    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    let bits_per_digit = fast_log2(format.mantissa_radix());
    let decimal_point = options.decimal_point();

    // Validate our options: we don't support different exponent bases here.
    debug_assert!(format.mantissa_radix() == format.exponent_base());

    // Write our value, then trim trailing zeros, before we check the exact
    // bounds of the digits, to avoid accidentally choosing too many digits.
    let shl = calculate_shl(exp, bits_per_digit);
    let value = mantissa << shl;

    // SAFETY: safe since the buffer must be larger than `M::FORMATTED_SIZE`.
    let digit_count = unsafe {
        let count = value.write_mantissa::<M, FORMAT>(&mut index_unchecked_mut!(bytes[1..]));
        index_unchecked_mut!(bytes[0] = bytes[1]);
        index_unchecked_mut!(bytes[1]) = decimal_point;
        let zeros = rtrim_char_count(&index_unchecked!(bytes[2..count + 1]), b'0');
        count - zeros
    };
    // Extra 1 since we have the decimal point.
    let mut cursor = digit_count + 1;

    // Determine if we need to add more trailing zeros.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Write any trailing digits to the output.
    // SAFETY: bytes since cannot be empty.
    if !format.no_exponent_without_fraction() && cursor == 2 && options.trim_floats() {
        // Need to trim floats from trailing zeros, and we have only a decimal.
        cursor -= 1;
    } else if exact_count < 2 {
        // Need to have at least 1 digit, the trailing `.0`.
        unsafe { index_unchecked_mut!(bytes[cursor]) = b'0' };
        cursor += 1;
    } else if exact_count > digit_count {
        // NOTE: Neither `exact_count >= digit_count >= 2`.
        // We need to write `exact_count - (cursor - 1)` digits, since
        // cursor includes the decimal point.
        let digits_end = exact_count + 1;
        // SAFETY: this is safe as long as the buffer was large enough
        // to hold `min_significant_digits + 1`.
        unsafe {
            slice_fill_unchecked!(index_unchecked_mut!(bytes[cursor..digits_end]), b'0');
        }
        cursor = digits_end;
    }

    // Now, write our scientific notation.
    let scaled_sci_exp = scale_sci_exp(sci_exp, bits_per_digit);
    // SAFETY: safe if the buffer is large enough to hold the maximum written float.
    unsafe {
        shared::write_exponent::<FORMAT>(bytes, &mut cursor, scaled_sci_exp, options.exponent())
    };

    cursor
}

/// Write negative float to string without scientific notation.
/// Has a negative exponent (shift right) and no scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of
/// significant digits and the leading zeros.
#[inline]
pub unsafe fn write_float_negative_exponent<M, const FORMAT: u128>(
    mantissa: M,
    exp: i32,
    sci_exp: i32,
    bytes: &mut [u8],
    options: &Options,
) -> usize
where
    M: WriteInteger + FormattedSize,
{
    // NOTE:
    //  This cannot trim trailing zeros, since the exponent **must**
    //  be less than 0 and the value cannot be zero.
    debug_assert!(mantissa != M::ZERO);
    debug_assert!(sci_exp < 0);

    // Just decent size bounds checks to ensure we have a lot of space.
    assert!(M::FORMATTED_SIZE < BUFFER_SIZE - 2);
    debug_assert!(bytes.len() >= BUFFER_SIZE);

    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    let bits_per_digit = fast_log2(format.mantissa_radix());
    let decimal_point = options.decimal_point();

    // The number of 0 bits we need to pad left (reducing the
    // exponent) is just the negative scientific exponent.
    // We then need to calculate the number of zero digits
    // from this, remembering that we're padding left,
    // so for example, `1/2` in hex is represented as `0.8`.
    // That means we need the `⌈ zero_bits / bits_per_digit ⌉`.
    let zero_bits = sci_exp.wrapping_neg();
    let zero_digits = fast_ceildiv(zero_bits, bits_per_digit) as usize;

    // Write our 0 digits.
    // SAFETY: safe if `bytes.len() > BUFFER_SIZE - 2`.
    unsafe {
        index_unchecked_mut!(bytes[0]) = b'0';
        index_unchecked_mut!(bytes[1]) = decimal_point;
        let digits = &mut index_unchecked_mut!(bytes[2..zero_digits + 1]);
        slice_fill_unchecked!(digits, b'0');
    }
    let mut cursor = zero_digits + 1;

    // Generate our digits after the shift. Store the number of written
    // digits, so we can adjust the end-point accordingly.
    let shl = calculate_shl(exp, bits_per_digit);
    let value = mantissa << shl;

    // SAFETY: both are safe, if the buffer is large enough to hold the significant digits.
    let digit_count = unsafe {
        let count = value.write_mantissa::<M, FORMAT>(&mut index_unchecked_mut!(bytes[cursor..]));
        let zeros = rtrim_char_count(&index_unchecked!(bytes[cursor..cursor + count]), b'0');
        count - zeros
    };
    cursor += digit_count;

    // Determine if we need to add more trailing zeros.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Check if we need to write more trailing digits.
    // NOTE: we cannot have a "0.1" case here, since we've previous truncated
    // the significant digits, and the result is < 1.
    if exact_count > digit_count {
        let zeros = exact_count - digit_count;
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let digits = &mut index_unchecked_mut!(bytes[cursor..cursor + zeros]);
            slice_fill_unchecked!(digits, b'0');
        }
        cursor += zeros;
    }

    cursor
}

/// Write positive float to string without scientific notation.
/// Has a positive exponent (shift left) and no scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of
/// significant digits and the (optional) trailing zeros.
#[inline]
pub unsafe fn write_float_positive_exponent<M, const FORMAT: u128>(
    mantissa: M,
    exp: i32,
    sci_exp: i32,
    bytes: &mut [u8],
    options: &Options,
) -> usize
where
    M: WriteInteger + FormattedSize,
{
    debug_assert!(sci_exp >= 0 || mantissa == M::ZERO);

    // Config options
    let format = NumberFormat::<{ FORMAT }> {};
    let bits_per_digit = fast_log2(format.mantissa_radix());
    let decimal_point = options.decimal_point();

    // Write our value, then trim trailing zeros, before we check the exact
    // bounds of the digits, to avoid accidentally choosing too many digits.
    let shl = calculate_shl(exp, bits_per_digit);
    let value = mantissa << shl;

    // SAFETY: safe since the buffer must be larger than `M::FORMATTED_SIZE`.
    let mut digit_count = unsafe {
        let count = value.write_mantissa::<M, FORMAT>(bytes);
        let zeros = rtrim_char_count(&index_unchecked!(bytes[..count]), b'0');
        count - zeros
    };

    // Write the significant digits.
    // Calculate the number of digits we can write left of the decimal point.
    // If we have a scientific exp of 0, we still need
    // to write 1 digit before, so it's ⌊ leading_bits / bits_per_digit ⌋ + 1.
    let leading_bits = sci_exp;
    let leading_digits = (leading_bits / bits_per_digit) as usize + 1;

    // Now need to write our decimal point and add any additional significant digits.
    let mut cursor: usize;
    let mut trimmed = false;
    if leading_digits >= digit_count {
        // We have more leading digits than digits we wrote: can write
        // any additional digits, and then just write the remaining zeros.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let digits = &mut index_unchecked_mut!(bytes[digit_count..leading_digits]);
            slice_fill_unchecked!(digits, b'0');
        }
        cursor = leading_digits;
        // Only write decimal point if we're not trimming floats.
        if !options.trim_floats() {
            // SAFETY: safe if `cursor +2 <= bytes.len()`.
            unsafe { index_unchecked_mut!(bytes[cursor]) = decimal_point };
            cursor += 1;
            unsafe { index_unchecked_mut!(bytes[cursor]) = b'0' };
            cursor += 1;
            digit_count += 1;
        } else {
            trimmed = true;
        }
    } else {
        // We have less leading digits than digits we wrote: find the
        // decimal point index, shift all digits right by 1, then write it.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        let shifted = digit_count - leading_digits;
        unsafe {
            let buf = &mut index_unchecked_mut!(bytes[leading_digits..digit_count + 1]);
            safe_assert!(buf.len() > shifted);
            for i in (0..shifted).rev() {
                index_unchecked_mut!(buf[i + 1] = buf[i]);
            }
            index_unchecked_mut!(bytes[leading_digits]) = decimal_point;
            cursor = digit_count + 1;
        }
    }

    // Determine if we need to add more trailing zeros after a decimal point.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Change the number of digits written, if we need to add more or trim digits.
    if !trimmed && exact_count > digit_count {
        // Check if we need to write more trailing digits.
        let zeros = exact_count - digit_count;
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let digits = &mut index_unchecked_mut!(bytes[cursor..cursor + zeros]);
            slice_fill_unchecked!(digits, b'0');
        }
        cursor += zeros;
    }

    cursor
}

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

/// Fast integral log2.
/// Only to be used on radixes.
#[inline]
pub fn fast_log2(x: u32) -> i32 {
    debug_assert!(matches!(x, 2 | 4 | 8 | 16 | 32));
    32 - 1 - (x | 1).leading_zeros() as i32
}

/// Calculate the number of significant bits in an integer.
/// This is just `1-ctlz(value)`.
#[inline(always)]
pub fn significant_bits<T: UnsignedInteger>(value: T) -> u32 {
    T::BITS as u32 - value.leading_zeros()
}

/// Calculate the fast ceiling, integral division.
#[inline(always)]
pub fn fast_ceildiv(value: i32, base: i32) -> i32 {
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
pub fn inverse_remainder(remainder: i32, base: i32) -> i32 {
    debug_assert!(remainder >= 0 && remainder < base);
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
///
/// NOTE: this can never overflow the significant digits, since
/// the mantissa must be at **least** 6-bits smaller than the mantissa
/// size (for `f16`, larger for anything else) and `bits_per_digit <= 5`.
#[inline(always)]
pub fn calculate_shl(exp: i32, bits_per_digit: i32) -> i32 {
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
pub fn scale_sci_exp(sci_exp: i32, bits_per_digit: i32) -> i32 {
    if sci_exp < 0 {
        let neg_sci_exp = sci_exp.wrapping_neg();
        fast_ceildiv(neg_sci_exp, bits_per_digit).wrapping_neg()
    } else {
        sci_exp / bits_per_digit
    }
}

// ALGORITHM
// ---------

/// Round mantissa to the nearest value, returning only the number
/// of significant digits. Also returns the number of bits of the mantissa.
#[inline]
pub fn truncate_and_round<M>(mantissa: M, radix: u32, options: &Options) -> (M, usize)
where
    M: UnsignedInteger,
{
    let mut mantissa_bits = significant_bits(mantissa) as usize;
    let bits_per_digit = fast_log2(radix);

    // Get the number of max digits, and then calculate if we need to round.
    let mut max_digits = usize::MAX;
    if let Some(digits) = options.max_significant_digits() {
        max_digits = digits.get();
    }
    let max_bits = max_digits.saturating_mul(bits_per_digit as usize);
    let mut shifted_mantissa = mantissa;

    // Need to truncate the number of significant digits.
    if max_bits < mantissa_bits {
        let shr = (mantissa_bits - max_bits) as i32;
        shifted_mantissa = mantissa >> shr;

        // We need to round-nearest, tie-even, so we need to handle
        // the truncation **here**. If the representation is above
        // halfway at all, we need to round up, even if 1 bit.
        if options.round_mode() == RoundMode::Round {
            let mask = (M::ONE << shr) - M::ONE;
            let halfway = M::ONE << (shr - 1);
            let above_halfway = (mantissa & mask) > halfway;
            let is_halfway = (mantissa & mask) == halfway;
            let is_odd = shifted_mantissa & M::ONE == M::ONE;

            // Round-up and calculate if we carry over 1-bit.
            // The built-in ctlz is very fast, so use that.
            // Add 1 to the mantissa bits if we carry.
            let initial_bits = shifted_mantissa.leading_zeros();
            shifted_mantissa += as_cast((above_halfway || (is_odd & is_halfway)) as u32);
            let final_bits = shifted_mantissa.leading_zeros();
            mantissa_bits += (final_bits - initial_bits) as usize;
        }
    }

    (shifted_mantissa, mantissa_bits)
}
