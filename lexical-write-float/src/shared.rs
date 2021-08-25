//! Shared utilities for writing floats.

use crate::options::{Options, RoundMode};
use lexical_util::digit::digit_to_char_const;
use lexical_util::format::NumberFormat;
use lexical_write_integer::write::WriteInteger;

/// Debug assertion to ensure we properly rounded the significant digits.
#[inline(always)]
pub fn debug_assert_digits(digit_count: usize, options: &Options) {
    let max_digits = options.max_significant_digits().map_or(digit_count, |x| x.get());
    debug_assert!(digit_count <= max_digits);
}

/// Round-up the last digit, from a buffer of digits.
///
/// Round up the last digit, incrementally handling all subsequent
/// digits in case of overflow.
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
        let digit = unsafe { index_unchecked!(digits[index - 1]) };
        if digit < max_digit {
            // SAFETY: safe since `index > 0 && index <= digits.len()`.
            unsafe { index_unchecked_mut!(digits[index - 1]) = digit + 1 };
            return index;
        }
        // Don't have to assign b'0' otherwise, since we're just carrying
        // to the next digit.
        index -= 1;
    }

    // Means all digits were max digit: we need to round up.
    // SAFETY: safe since `digits.len() > 1`.
    unsafe { index_unchecked_mut!(digits[0]) = b'1' };

    1
}

/// Round the number of digits based on the maximum digits, for decimal digits.
/// `digits` is a mutable buffer of the current digits, `digit_count` is the
/// length of the written digits in `digits`, and `exp` is the decimal exponent
/// relative to the digits.
///
/// # Safety
///
/// Safe as long as `ndigits <= digits.len()`.
#[allow(clippy::comparison_chain)]
#[cfg_attr(not(feature = "compact"), inline)]
pub unsafe fn truncate_and_round_decimal(
    digits: &mut [u8],
    digit_count: usize,
    exp: i32,
    options: &Options,
) -> (usize, i32) {
    debug_assert!(digit_count <= digits.len());

    let max_digits = if let Some(digits) = options.max_significant_digits() {
        digits.get()
    } else {
        return (digit_count, exp);
    };
    if max_digits >= digit_count {
        return (digit_count, exp);
    }

    // Need to adjust `exp`, since we're shortening the digits in the input.
    let shift = digit_count - max_digits;
    let exp = exp + shift as i32;
    if options.round_mode() == RoundMode::Truncate {
        // Don't round input, just shorten number of digits emitted.
        return (max_digits, exp);
    }

    // We need to round-nearest, tie-even, so we need to handle
    // the truncation **here**. If the representation is above
    // halfway at all, we need to round up, even if 1 digit.

    // Get the last non-truncated digit, and the remaining ones.
    let truncated = unsafe { index_unchecked!(digits[max_digits]) };
    let digits = if truncated < b'5' {
        // Just truncate, going to round-down anyway.
        max_digits
    } else if truncated > b'5' {
        // Round-up always.
        // SAFETY: safe if `digit_count <= digits.len()`, because `max_digits < digit_count`.
        unsafe { round_up(digits, max_digits, 10) }
    } else {
        // Have a near-halfway case, resolve it.
        // SAFETY: safe if `digit_count < digits.len()`.
        let (is_odd, is_above) = unsafe {
            let to_round = &index_unchecked!(digits[max_digits - 1..digit_count]);
            let is_odd = index_unchecked!(to_round[0]) % 2 == 1;
            let is_above = index_unchecked!(to_round[2..]).iter().any(|&x| x != b'0');
            (is_odd, is_above)
        };
        if is_odd || is_above {
            // SAFETY: safe if `digit_count <= digits.len()`, because `max_digits < digit_count`.
            unsafe { round_up(digits, max_digits, 10) }
        } else {
            max_digits
        }
    };

    (digits, exp)
}

/// Write the sign for the exponent.
///
/// # Safety
///
/// Safe if `bytes.len() > cursor + 1`.
#[cfg_attr(not(feature = "compact"), inline)]
pub unsafe fn write_exponent_sign<const FORMAT: u128>(
    bytes: &mut [u8],
    cursor: &mut usize,
    exp: i32,
) -> u32 {
    let format = NumberFormat::<{ FORMAT }> {};
    if exp < 0 {
        // SAFETY: safe if bytes is large enough to hold the output
        unsafe { index_unchecked_mut!(bytes[*cursor]) = b'-' };
        *cursor += 1;
        exp.wrapping_neg() as u32
    } else if cfg!(feature = "format") && format.required_exponent_sign() {
        // SAFETY: safe if bytes is large enough to hold the output
        unsafe { index_unchecked_mut!(bytes[*cursor]) = b'+' };
        *cursor += 1;
        exp as u32
    } else {
        exp as u32
    }
}

/// Write the symbol, sign, and digits for the exponent.
///
/// # Safety
///
/// Safe if the buffer can hold all the significant digits and the sign
/// starting from cursor.
#[cfg_attr(not(feature = "compact"), inline)]
pub unsafe fn write_exponent<const FORMAT: u128>(
    bytes: &mut [u8],
    cursor: &mut usize,
    exp: i32,
    exponent_character: u8,
) {
    *cursor += unsafe {
        index_unchecked_mut!(bytes[*cursor]) = exponent_character;
        *cursor += 1;
        let positive_exp = write_exponent_sign::<FORMAT>(bytes, cursor, exp);
        positive_exp.write_exponent::<u32, FORMAT>(&mut index_unchecked_mut!(bytes[*cursor..]))
    };
}

/// Detect the notation to use for the float formatter and call the appropriate function..
macro_rules! write_float {
    (
        $format:ident,
        $sci_exp:ident,
        $options:ident,
        $write_scientific:ident,
        $write_positive:ident,
        $write_negative:ident,
        $(generic => $generic:tt,)?
        args => $($args:expr,)*
    ) => {{
        use lexical_util::format::NumberFormat;

        let format = NumberFormat::<{ $format }> {};
        let min_exp = $options.negative_exponent_break().map_or(-5, |x| x.get());
        let max_exp = $options.positive_exponent_break().map_or(9, |x| x.get());

        let outside_break = $sci_exp < min_exp || $sci_exp > max_exp;
        let require_exponent = format.required_exponent_notation() || outside_break;
        if !format.no_exponent_notation() && require_exponent {
            // Write digits in scientific notation.
            // SAFETY: safe as long as bytes is large enough to hold all the digits.
            unsafe { $write_scientific::<$($generic,)? FORMAT>($($args,)*) }
        } else if $sci_exp >= 0 {
            // Write positive exponent without scientific notation.
            // SAFETY: safe as long as bytes is large enough to hold all the digits.
            unsafe { $write_positive::<$($generic,)? FORMAT>($($args,)*) }
        } else {
            // Write negative exponent without scientific notation.
            // SAFETY: safe as long as bytes is large enough to hold all the digits.
            unsafe { $write_negative::<$($generic,)? FORMAT>($($args,)*) }
        }
    }};
}
