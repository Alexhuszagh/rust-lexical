//! Shared utilities for writing floats.

use lexical_util::digit::{char_to_valid_digit_const, digit_to_char_const};
use lexical_util::format::NumberFormat;
use lexical_write_integer::write::WriteInteger;

use crate::options::{Options, RoundMode};

/// Get the exact number of digits from a minimum bound.
#[inline(always)]
pub fn min_exact_digits(digit_count: usize, options: &Options) -> usize {
    let mut exact_count: usize = digit_count;
    if let Some(min_digits) = options.min_significant_digits() {
        exact_count = min_digits.get().max(exact_count);
    }
    exact_count
}

/// Round-up the last digit, from a buffer of digits.
///
/// Round up the last digit, incrementally handling all subsequent
/// digits in case of overflow.
#[cfg_attr(not(feature = "compact"), inline(always))]
pub fn round_up(digits: &mut [u8], count: usize, radix: u32) -> (usize, bool) {
    debug_assert!(count <= digits.len(), "rounding up requires digits.len() >= count");

    let mut index = count;
    let max_char = digit_to_char_const(radix - 1, radix);
    while index != 0 {
        let c = digits[index - 1];
        if c < max_char {
            let digit = char_to_valid_digit_const(c, radix);
            let rounded = digit_to_char_const(digit + 1, radix);
            // Won't panic since `index > 0 && index <= digits.len()`.
            digits[index - 1] = rounded;
            return (index, false);
        }
        // Don't have to assign `b'0'` otherwise, since we're just carrying
        // to the next digit.
        index -= 1;
    }

    // Means all digits were max digit: we need to round up.
    digits[0] = b'1';

    (1, true)
}

/// Round the number of digits based on the maximum digits, for decimal digits.
///
/// `digits` is a mutable buffer of the current digits, `digit_count` is the
/// length of the written digits in `digits`, and `exp` is the decimal exponent
/// relative to the digits. Returns the digit count, resulting exp, and if
/// the input carried to the next digit.
#[cfg_attr(not(feature = "compact"), inline(always))]
#[allow(clippy::comparison_chain)] // reason="conditions are different logical concepts"
pub fn truncate_and_round_decimal(
    digits: &mut [u8],
    digit_count: usize,
    options: &Options,
) -> (usize, bool) {
    debug_assert!(digit_count <= digits.len());

    let max_digits = if let Some(digits) = options.max_significant_digits() {
        digits.get()
    } else {
        return (digit_count, false);
    };
    if max_digits >= digit_count {
        return (digit_count, false);
    }

    // Check if we're truncating, if so, shorten the digits in the input.
    if options.round_mode() == RoundMode::Truncate {
        // Don't round input, just shorten number of digits emitted.
        return (max_digits, false);
    }

    // We need to round-nearest, tie-even, so we need to handle
    // the truncation **here**. If the representation is above
    // halfway at all, we need to round up, even if 1 digit.

    // Get the last non-truncated digit, and the remaining ones.
    // Won't panic if `digit_count < digits.len()`, since `max_digits <
    // digit_count`.
    let truncated = digits[max_digits];
    let (digits, carried) = if truncated < b'5' {
        // Just truncate, going to round-down anyway.
        (max_digits, false)
    } else if truncated > b'5' {
        // Round-up always.
        round_up(digits, max_digits, 10)
    } else {
        // Have a near-halfway case, resolve it.
        let to_round = &digits[max_digits - 1..digit_count];
        let is_odd = to_round[0] % 2 == 1;
        let is_above = to_round[2..].iter().any(|&x| x != b'0');
        if is_odd || is_above {
            // Won't panic `digit_count <= digits.len()`, because `max_digits <
            // digit_count`.
            round_up(digits, max_digits, 10)
        } else {
            (max_digits, false)
        }
    };

    (digits, carried)
}

/// Write the sign for the exponent.
#[cfg_attr(not(feature = "compact"), inline(always))]
pub fn write_exponent_sign<const FORMAT: u128>(
    bytes: &mut [u8],
    cursor: &mut usize,
    exp: i32,
) -> u32 {
    let format = NumberFormat::<{ FORMAT }> {};
    if exp < 0 {
        bytes[*cursor] = b'-';
        *cursor += 1;
        exp.wrapping_neg() as u32
    } else if cfg!(feature = "format") && format.required_exponent_sign() {
        bytes[*cursor] = b'+';
        *cursor += 1;
        exp as u32
    } else {
        exp as u32
    }
}

/// Write the symbol, sign, and digits for the exponent.
#[cfg_attr(not(feature = "compact"), inline(always))]
pub fn write_exponent<const FORMAT: u128>(
    bytes: &mut [u8],
    cursor: &mut usize,
    exp: i32,
    exponent_character: u8,
) {
    bytes[*cursor] = exponent_character;
    *cursor += 1;
    let positive_exp: u32 = write_exponent_sign::<FORMAT>(bytes, cursor, exp);
    *cursor += positive_exp.write_exponent_signed::<FORMAT>(&mut bytes[*cursor..]);
}

/// Detect the notation to use for the float formatter and call the appropriate
/// function.
///
/// The float must be positive. This doesn't affect the safety guarantees but
/// all algorithms assume a float >0 or that is not negative 0.
///
/// - `float` - The float to write to string.
/// - `format` - The formatting specification for the float.
/// - `sci_exp` - The scientific exponents describing the float.
/// - `options` - Options configuring how to serialize the float.
/// - `write_scientific` - The callback to write scientific notation numbers.
/// - `write_positive` - The callback to write non-scientific, positive numbers.
/// - `write_negative` - The callback to write non-scientific, negative numbers.
/// - `bytes` - The output buffer to write to.
/// - `args` - Additional arguments to pass to our internal writers.
macro_rules! write_float {
    (
        $float:ident,
        $format:ident,
        $sci_exp:ident,
        $options:ident,
        $write_scientific:ident,
        $write_positive:ident,
        $write_negative:ident,
        $(generic => $generic:tt,)?
        bytes => $bytes:ident,
        args => $($args:expr,)*
    ) => {{
        use lexical_util::format::NumberFormat;

        debug_assert!($float.is_sign_positive());

        let format = NumberFormat::<{ $format }> {};
        let min_exp = $options.negative_exponent_break().map_or(-5, |x| x.get());
        let max_exp = $options.positive_exponent_break().map_or(9, |x| x.get());

        let outside_break = $sci_exp < min_exp || $sci_exp > max_exp;
        let require_exponent = format.required_exponent_notation() || outside_break;
        if !format.no_exponent_notation() && require_exponent {
            // Write digits in scientific notation.
            $write_scientific::<$($generic,)? FORMAT>($bytes, $($args,)*)
        } else if $sci_exp < 0 {
            // Write negative exponent without scientific notation.
            $write_negative::<$($generic,)? FORMAT>($bytes, $($args,)*)
        } else {
            // Write positive exponent without scientific notation.
            $write_positive::<$($generic,)? FORMAT>($bytes, $($args,)*)
        }
    }};
}
