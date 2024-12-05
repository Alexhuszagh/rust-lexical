//! Optimized float serializer for hexadecimal floats.
//!
//! This actually works for any case where we can exactly represent
//! any power of the mantissa radix using the exponent base. For example,
//! given a mantissa radix of `16`, and an exponent base of `8`,
//! `16^2` cannot be exactly represented in octal. In short:
//! `⌊log2(r) / log2(b)⌋ == ⌈log2(r) / log2(b)⌉`.
//!
//! This gives us the following mantissa radix/exponent base combinations:
//!
//! - 4, 2
//! - 8, 2
//! - 16, 2
//! - 32, 2
//! - 16, 4

#![cfg(feature = "power-of-two")]
#![doc(hidden)]

use lexical_util::algorithm::rtrim_char_count;
use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
use lexical_util::format::NumberFormat;
use lexical_util::num::{Float, Integer};
use lexical_write_integer::write::WriteInteger;

use crate::binary::{
    calculate_shl,
    fast_ceildiv,
    fast_log2,
    truncate_and_round,
    write_float_negative_exponent,
    write_float_positive_exponent,
};
use crate::options::Options;
use crate::shared;

/// Optimized float-to-string algorithm for hexadecimal strings.
///
/// This assumes the float is:
///     1). Non-special (NaN or Infinite).
///     2). Non-negative.
///
/// # Safety
///
/// Safe as long as the float isn't special (NaN or Infinity), and `bytes`
/// is large enough to hold the significant digits.
///
/// # Panics
///
/// Panics if the radix for the significant digits is not 16, if
/// the exponent base is not 2, or if the radix for the exponent
/// digits is not 10.
pub fn write_float<F: Float, const FORMAT: u128>(
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
    debug_assert!(matches!(
        (format.radix(), format.exponent_base()),
        (4, 2) | (8, 2) | (16, 2) | (32, 2) | (16, 4)
    ));

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

    // SAFETY: Safe, just other API methods need to be migrated in.
    write_float!(
        float,
        FORMAT,
        sci_exp,
        options,
        write_float_scientific,
        write_float_positive_exponent,
        write_float_negative_exponent,
        generic => _,
        bytes => bytes,
        args => mantissa, exp, sci_exp, options,
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
/// based on the number of maximum digits.
#[cfg_attr(not(feature = "compact"), inline(always))]
pub fn write_float_scientific<M, const FORMAT: u128>(
    bytes: &mut [u8],
    mantissa: M,
    exp: i32,
    sci_exp: i32,
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
    let bits_per_base = fast_log2(format.exponent_base());
    let decimal_point = options.decimal_point();

    // Write our value, then trim trailing zeros, before we check the exact
    // bounds of the digits, to avoid accidentally choosing too many digits.
    // shl is the powers of two we have missing from our exponent that nee
    // to be transferred to our significant digits. Since the all mantissa
    // radix powers can be **exactly** represented by exponent bases,
    // we can just shift this into the mantissa.
    let shl = calculate_shl(exp, bits_per_digit);
    let value = mantissa << shl;

    let count = value.write_mantissa::<FORMAT>(&mut bytes[1..]);
    bytes[0] = bytes[1];
    bytes[1] = decimal_point;
    let zeros = rtrim_char_count(&bytes[2..count + 1], b'0');
    let digit_count = count - zeros;
    // Extra 1 since we have the decimal point.
    let mut cursor = digit_count + 1;

    // Determine if we need to add more trailing zeros.
    let exact_count = shared::min_exact_digits(digit_count, options);

    // Write any trailing digits to the output.
    // Won't panic safe if the buffer is large enough to hold the significant
    // digits.
    if !format.no_exponent_without_fraction() && cursor == 2 && options.trim_floats() {
        // Need to trim floats from trailing zeros, and we have only a decimal.
        cursor -= 1;
    } else if exact_count < 2 {
        // Need to have at least 1 digit, the trailing `.0`.
        bytes[cursor] = b'0';
        cursor += 1;
    } else if exact_count > digit_count {
        // NOTE: Neither `exact_count >= digit_count >= 2`.
        // We need to write `exact_count - (cursor - 1)` digits, since
        // cursor includes the decimal point.
        let digits_end = exact_count + 1;
        bytes[cursor..digits_end].fill(b'0');
        cursor = digits_end;
    }

    // Now, write our scientific notation.
    // Won't panic safe if bytes is large enough to store all digits.
    let scaled_sci_exp = scale_sci_exp(sci_exp, bits_per_digit, bits_per_base);
    shared::write_exponent::<FORMAT>(bytes, &mut cursor, scaled_sci_exp, options.exponent());

    cursor
}

// ALGORITHM
// ---------

/// We need to scale the scientific exponent for writing.
///
/// This is similar to [`binary::scale_sci_exp`](crate::binary::scale_sci_exp),
/// however, we need to effectively have the same algorithm with `bits_per_base`
/// instead of `bits_per_digit`. However, `bits_per_base` is smaller, and
/// will not properly floor the values, so we add in an extra step.
#[inline(always)]
pub fn scale_sci_exp(sci_exp: i32, bits_per_digit: i32, bits_per_base: i32) -> i32 {
    if sci_exp < 0 {
        let neg_sci_exp = sci_exp.wrapping_neg();
        let floor = fast_ceildiv(neg_sci_exp, bits_per_digit);
        (floor * bits_per_digit / bits_per_base).wrapping_neg()
    } else {
        let floor = sci_exp / bits_per_digit;
        floor * bits_per_digit / bits_per_base
    }
}
