//! Adaptation of the V8 ftoa algorithm with a custom radix.
//!
//! This algorithm is adapted from the V8 codebase,
//! and may be found [here](https://github.com/v8/v8).
//!
//! # Unsupported Features
//!
//! This does not support a few features from the format packed struct,
//! most notably, it will never write numbers in scientific notation.
//! Scientific notation must be disabled.

#![cfg(feature = "radix")]
#![doc(hidden)]

use crate::options::{Options, RoundMode};
use core::mem;
use lexical_util::algorithm::{ltrim_char_count, rtrim_char_count};
use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
use lexical_util::digit::{char_to_digit_const, digit_to_char_const};
use lexical_util::format::NumberFormat;
use lexical_util::num::Float;
use lexical_write_integer::write::WriteInteger;

// ALGORITHM
// ---------

/// Naive float-to-string algorithm for generic radixes.
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
/// Panics if exponent notation is used.
pub unsafe fn write_float<F: Float, const FORMAT: u128>(
    float: F,
    bytes: &mut [u8],
    options: &Options,
) -> usize
where
    <F as Float>::Unsigned: WriteInteger + FormattedSize,
{
    // PRECONDITIONS

    // Assert no special cases remain, no negative numbers, and a valid format.
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    debug_assert!(!float.is_special());
    debug_assert!(float >= F::ZERO);
    debug_assert!(F::BITS <= 64);

    // Validate our options: we don't support different exponent bases here.
    debug_assert!(format.mantissa_radix() == format.exponent_base());

    // Temporary buffer for the result. We start with the decimal point in the
    // middle and write to the left for the integer part and to the right for the
    // fractional part. 1024 characters for the exponent and 52 for the mantissa
    // either way, with additional space for sign, decimal point and string
    // termination should be sufficient.
    const SIZE: usize = 2200;
    let buffer: mem::MaybeUninit<[u8; SIZE]> = mem::MaybeUninit::uninit();
    // SAFETY: safe, since we never read bytes that weren't written.
    let mut buffer = unsafe { buffer.assume_init() };
    //let buffer = buffer.as_mut_ptr();
    let initial_cursor: usize = SIZE / 2;
    let mut integer_cursor = initial_cursor;
    let mut fraction_cursor = initial_cursor;
    let base = F::as_cast(format.radix());

    // Split the float into an integer part and a fractional part.
    let mut integer = float.floor();
    let mut fraction = float - integer;

    // We only compute fractional digits up to the input double's precision.
    let mut delta = F::as_cast(0.5) * (float.next_positive() - float);
    delta = F::ZERO.next_positive().max_finite(delta);
    debug_assert!(delta > F::ZERO);

    // Write our fraction digits.
    // SAFETY: we have 1100 digits, which is enough for any float f64 or smaller.
    if fraction > delta {
        loop {
            // Shift up by one digit.
            fraction *= base;
            delta *= base;
            // Write digit.
            let digit = fraction.as_u32();
            let c = digit_to_char_const(digit, format.radix());
            unsafe { index_unchecked_mut!(buffer[fraction_cursor]) = c };
            fraction_cursor += 1;
            // Calculate remainder.
            fraction -= F::as_cast(digit);
            // Round to even.
            if fraction > F::as_cast(0.5) || (fraction == F::as_cast(0.5) && (digit & 1) != 0) {
                if fraction + delta > F::ONE {
                    // We need to back trace already written digits in case of carry-over.
                    loop {
                        fraction_cursor -= 1;
                        if fraction_cursor == initial_cursor - 1 {
                            // Carry over to the integer part.
                            integer += F::ONE;
                            break;
                        }
                        // Reconstruct digit.
                        let c = unsafe { index_unchecked!(buffer[fraction_cursor]) };
                        if let Some(digit) = char_to_digit_const(c, format.radix()) {
                            let idx = digit + 1;
                            let c = digit_to_char_const(idx, format.radix());
                            unsafe { index_unchecked_mut!(buffer[fraction_cursor]) = c };
                            fraction_cursor += 1;
                            break;
                        }
                    }
                    break;
                }
            }

            if delta >= fraction {
                break;
            }
        }
    }

    // Compute integer digits. Fill unrepresented digits with zero.
    // SAFETY: we have 1100 digits, which is enough for any float f64 or smaller.
    // We do this first, so we can do extended precision control later.
    while (integer / base).exponent() > 0 {
        integer /= base;
        integer_cursor -= 1;
        unsafe { index_unchecked_mut!(buffer[integer_cursor]) = b'0' };
    }

    loop {
        let remainder = integer % base;
        integer_cursor -= 1;
        let idx = remainder.as_u32();
        let c = digit_to_char_const(idx, format.radix());
        unsafe { index_unchecked_mut!(buffer[integer_cursor]) = c };
        integer = (integer - remainder) / base;

        if integer <= F::ZERO {
            break;
        }
    }

    // Write our exponent.
    let sci_exp = if float == F::ZERO {
        0
    } else {
        naive_exponent(float, format.radix())
    };
    let min_exp = options.negative_exponent_break().map_or(-5, |x| x.get());
    let max_exp = options.positive_exponent_break().map_or(9, |x| x.get());
    if !format.no_exponent_notation()
        && (format.required_exponent_notation() || sci_exp < min_exp || sci_exp > max_exp)
    {
        // Write digits in scientific notation.
        unsafe {
            write_float_scientific::<FORMAT>(
                sci_exp,
                &mut buffer,
                bytes,
                initial_cursor,
                integer_cursor,
                fraction_cursor,
                options,
            )
        }
    } else {
        // Write digits without scientific notation.
        unsafe {
            write_float_nonscientific::<FORMAT>(
                &mut buffer,
                bytes,
                initial_cursor,
                integer_cursor,
                fraction_cursor,
                options,
            )
        }
    }
}

// Store the first digit and up to `BUFFER_SIZE - 20` digits
// that occur from left-to-right in the decimal representation.
// For example, for the number 123.45, store the first digit `1`
// and `2345` as the remaining values. Then, decide on-the-fly
// if we need scientific or regular formatting.
//
//   BUFFER_SIZE
// - 1      # first digit
// - 1      # period
// - 1      # +/- sign
// - 2      # e and +/- sign
// - 9      # max exp is 308, in radix2 is 9
// - 1      # null terminator
// = 15 characters of formatting required
// Just pad it a bit, we don't want memory corruption.
const MAX_NONDIGIT_LENGTH: usize = 25;
const MAX_DIGIT_LENGTH: usize = BUFFER_SIZE - MAX_NONDIGIT_LENGTH;

/// Round-up the last digit.
///
/// # Safety
///
/// Safe as long as `count <= digits.len()`.
pub unsafe fn round_up(digits: &mut [u8], count: usize, radix: u32) -> usize {
    debug_assert!(count <= digits.len());

    let mut index = count;
    let max_digit = digit_to_char_const(radix - 1, radix);
    while index != 0 {
        // SAFETY: safe if `count <= digits.len()`, since then
        // `index > 0 && index <= digits.len()`.
        let c = unsafe { index_unchecked!(digits[index - 1]) };
        if c < max_digit {
            // SAFETY: safe since `index > 0 && index <= digits.len()`.
            unsafe { index_unchecked_mut!(digits[index - 1]) = c + 1 };
            return index;
        }
        // Don't have to assign b'0' otherwise, since we're just carrying
        // to the next digit.
        index -= 1;
    }

    // Means all digits were b'9': we need to round up.
    // SAFETY: safe since `digits.len() > 1`.
    unsafe { index_unchecked_mut!(digits[0]) = b'1' };

    1
}

/// Round mantissa to the nearest value, returning only the number
/// of significant digits. Also returns the number of bits of the mantissa.
#[inline]
pub unsafe fn truncate_and_round(
    buffer: &mut [u8],
    start: usize,
    end: usize,
    radix: u32,
    options: &Options,
) -> usize {
    // Get the number of max digits, and then calculate if we need to round.
    let count = end - start;
    let max_digits = if let Some(digits) = options.max_significant_digits() {
        digits.get()
    } else {
        return count;
    };

    if max_digits >= count {
        return count;
    }
    if options.round_mode() == RoundMode::Truncate {
        // Don't round input, just shorten number of digits emitted.
        return max_digits;
    }

    // Need to add the number of leading zeros to the digits count.
    let max_digits = {
        let digits = unsafe { &mut index_unchecked_mut!(buffer[start..start + max_digits]) };
        max_digits + ltrim_char_count(digits, b'0')
    };

    // We need to round-nearest, tie-even, so we need to handle
    // the truncation **here**. If the representation is above
    // halfway at all, we need to round up, even if 1 bit.
    // SAFETY: safe since `max_digits < count`, and `max_digits > 0`.
    let last = unsafe { index_unchecked!(buffer[start + max_digits - 1]) };
    let first = unsafe { index_unchecked!(buffer[start + max_digits]) };
    let halfway = digit_to_char_const(radix / 2, radix);
    let rem = radix % 2;
    if first < halfway {
        // Just truncate, going to round-down anyway.
        max_digits
    } else if first > halfway {
        // Round-up always.
        // SAFETY: safe if `start <= end, because `max_digits < count`.
        let digits = unsafe { &mut index_unchecked_mut!(buffer[start..start + max_digits]) };
        unsafe { round_up(digits, max_digits, radix) }
    } else if rem == 0 {
        // Even radix, our halfway point `$c00000.....`.
        // SAFETY: safe if `start <= end, because `max_digits < count`.
        let truncated = unsafe { &index_unchecked!(buffer[start + max_digits + 1..end]) };
        if truncated.iter().all(|&x| x == b'0') && last & 1 == 0 {
            // At an exact halfway point, and even, round-down.
            max_digits
        } else {
            // Above halfway or at halfway and even, round-up
            // SAFETY: safe if `count <= digits.len()`, because `max_digits < count`.
            let digits = unsafe { &mut index_unchecked_mut!(buffer[start..start + max_digits]) };
            unsafe { round_up(digits, max_digits, radix) }
        }
    } else {
        // Odd radix, our halfway point is `$c$c$c$c$c$c....`. Cannot halfway points.
        // SAFETY: safe if `start <= end, because `max_digits < count`.
        let truncated = unsafe { &index_unchecked!(buffer[start + max_digits + 1..end]) };
        for &c in truncated.iter() {
            if c < halfway {
                return max_digits;
            } else if c > halfway {
                // Above halfway
                // SAFETY: safe if `count <= digits.len()`, because `max_digits < count`.
                let digits =
                    unsafe { &mut index_unchecked_mut!(buffer[start..start + max_digits]) };
                return unsafe { round_up(digits, max_digits, radix) };
            }
        }
        return max_digits;
    }
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
pub unsafe fn write_float_scientific<const FORMAT: u128>(
    sci_exp: i32,
    buffer: &mut [u8],
    bytes: &mut [u8],
    initial_cursor: usize,
    integer_cursor: usize,
    fraction_cursor: usize,
    options: &Options,
) -> usize {
    // PRECONDITIONS
    debug_assert!(bytes.len() >= BUFFER_SIZE);

    // Config options.
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    let decimal_point = format.decimal_point();
    let exponent_character = format.exponent();

    // Round and truncate the number of significant digits.
    let start: usize;
    let end: usize;
    if sci_exp <= 0 {
        start = ((initial_cursor as i32) - sci_exp - 1) as usize;
        end = fraction_cursor.min(start + MAX_DIGIT_LENGTH + 1);
    } else {
        start = integer_cursor;
        end = fraction_cursor.min(start + MAX_DIGIT_LENGTH + 1);
    }
    // SAFETY: safe since `start + count <= end && end <= buffer.len()`.
    let count = unsafe { truncate_and_round(buffer, start, end, format.radix(), options) };
    // SAFETY: safe since `start + count <= end`.
    let digits = unsafe { &index_unchecked!(buffer[start..start + count]) };

    // Non-exponent portion.
    // Get as many digits as possible, up to `MAX_DIGIT_LENGTH+1`
    // since we are ignoring the digit for the first digit,
    // or the number of written digits.
    // SAFETY: safe since the buffer must be larger than `M::FORMATTED_SIZE`.
    let count = unsafe {
        index_unchecked_mut!(bytes[0] = digits[0]);
        index_unchecked_mut!(bytes[1]) = decimal_point;
        let src = digits.as_ptr().add(1);
        let dst = &mut index_unchecked_mut!(bytes[2..count + 1]);
        copy_nonoverlapping_unchecked!(dst, src, count - 1);
        let zeros = rtrim_char_count(&index_unchecked!(bytes[2..count + 1]), b'0');
        count - zeros
    };
    // Extra 1 since we have the decimal point.
    let mut cursor = count + 1;

    // Determine if we need to add more trailing zeros.
    let mut exact_count: usize = count;
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(count);
    }

    // Write any trailing digits to the output.
    // SAFETY: bytes cannot be empty.
    if !format.no_exponent_without_fraction() && cursor == 2 && options.trim_floats() {
        // Need to trim floats from trailing zeros, and we have only a decimal.
        cursor -= 1;
    } else if exact_count < 2 {
        // Need to have at least 1 digit, the trailing `.0`.
        unsafe { index_unchecked_mut!(bytes[cursor]) = b'0' };
        cursor += 1;
    } else if exact_count > count {
        // NOTE: Neither `exact_count >= count >= 2`.
        // We need to write `exact_count - (cursor - 1)` digits, since
        // cursor includes the decimal point.
        let digits_end = exact_count + 1;
        // SAFETY: this is safe as long as the buffer was large enough
        // to hold `min_significant_digits + 1`.
        unsafe {
            slice_fill_unchecked!(&mut index_unchecked_mut!(bytes[cursor..digits_end]), b'0');
        }
        cursor = digits_end;
    }

    // Now, write our scientific notation.
    unsafe { index_unchecked_mut!(bytes[cursor]) = exponent_character };
    cursor += 1;

    // We've handled the zero case: write the sign for the exponent.
    let positive_exp: u32;
    if sci_exp < 0 {
        unsafe { index_unchecked_mut!(bytes[cursor]) = b'-' };
        cursor += 1;
        positive_exp = sci_exp.wrapping_neg() as u32;
    } else if cfg!(feature = "format") && format.required_exponent_sign() {
        unsafe { index_unchecked_mut!(bytes[cursor]) = b'+' };
        cursor += 1;
        positive_exp = sci_exp as u32;
    } else {
        positive_exp = sci_exp as u32;
    }

    // Write our exponent digits.
    // SAFETY: safe since bytes must be large enough to store all digits.
    cursor += unsafe {
        positive_exp.write_exponent::<u32, FORMAT>(&mut index_unchecked_mut!(bytes[cursor..]))
    };

    cursor
}

/// Write float to string without scientific notation.
///
/// # Safety
///
/// Safe as long as `bytes` is large enough to hold the number of
/// significant digits and the leading zeros.
#[inline]
pub unsafe fn write_float_nonscientific<const FORMAT: u128>(
    buffer: &mut [u8],
    bytes: &mut [u8],
    initial_cursor: usize,
    integer_cursor: usize,
    fraction_cursor: usize,
    options: &Options,
) -> usize {
    // PRECONDITIONS
    debug_assert!(bytes.len() >= BUFFER_SIZE);

    // Config options.
    let format = NumberFormat::<{ FORMAT }> {};
    assert!(format.is_valid());
    let decimal_point = format.decimal_point();

    // Round and truncate the number of significant digits.
    let start = integer_cursor;
    let end = fraction_cursor.min(start + MAX_DIGIT_LENGTH + 1);
    // SAFETY: safe since `start + count <= end && end <= buffer.len()`.
    let mut count = unsafe { truncate_and_round(buffer, start, end, format.radix(), options) };
    // SAFETY: safe since `start + count <= end`.
    let digits = unsafe { &index_unchecked!(buffer[start..start + count]) };

    // Write the integer component.
    let integer_length = initial_cursor - start;
    let integer_count = count.min(integer_length);
    // SAFETY: safe if the buffer is large enough to hold the significant digits.
    unsafe {
        let src = digits.as_ptr();
        let dst = &mut index_unchecked_mut!(bytes[..integer_count]);
        copy_nonoverlapping_unchecked!(dst, src, integer_count);
    }
    if integer_count < integer_length {
        // We have more leading digits than digits we wrote: can write
        // any additional digits, and then just write the remaining zeros.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let digits = &mut index_unchecked_mut!(bytes[integer_count..integer_length]);
            slice_fill_unchecked!(digits, b'0');
        }
    }
    let mut cursor = integer_length;

    // SAFETY: safe if the buffer is large enough to hold the significant digits.
    unsafe { index_unchecked_mut!(bytes[cursor]) = decimal_point };
    cursor += 1;

    // Write the fraction component.
    let digits = unsafe { &index_unchecked!(digits[integer_count..]) };
    let fraction_count = count.saturating_sub(integer_length);
    if fraction_count > 0 {
        // Need to write additional fraction digits.
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        unsafe {
            let src = digits.as_ptr();
            let dst = &mut index_unchecked_mut!(bytes[cursor..cursor + fraction_count]);
            copy_nonoverlapping_unchecked!(dst, src, fraction_count);
            let zeros = rtrim_char_count(&index_unchecked!(bytes[cursor..cursor + count]), b'0');
            cursor += fraction_count - zeros;
        }
    } else if options.trim_floats() {
        // Remove the decimal point, went too far.
        cursor -= 1;
    } else {
        unsafe { index_unchecked_mut!(bytes[cursor]) = b'0' };
        cursor += 1;
        count += 1;
    }

    // Determine if we need to add more trailing zeros.
    let mut exact_count: usize = count;
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(count);
    }

    // Write any trailing digits to the output.
    // SAFETY: bytes cannot be empty.
    if (fraction_count > 0 || !options.trim_floats()) && exact_count > count {
        // NOTE: Neither `exact_count >= count >= 2`.
        // We need to write `exact_count - (cursor - 1)` digits, since
        // cursor includes the decimal point.
        let digits_end = cursor + exact_count - count;
        // SAFETY: this is safe as long as the buffer was large enough
        // to hold `min_significant_digits + 1`.
        unsafe {
            slice_fill_unchecked!(&mut index_unchecked_mut!(bytes[cursor..digits_end]), b'0');
        }
        cursor = digits_end;
    }

    cursor
}

// MATH
// ----

/// Calculate the naive exponent from a minimal value.
///
/// Don't export this for float, since it's specialized for radix.
#[inline]
fn naive_exponent<F: Float>(float: F, radix: u32) -> i32 {
    // floor returns the minimal value, which is our
    // desired exponent
    // ln(1.1e-5) -> -4.95 -> -5
    // ln(1.1e5) -> -5.04 -> 5
    debug_assert!(float != F::ZERO);
    (float.ln() / F::as_cast(radix).ln()).floor().as_i32()
}
