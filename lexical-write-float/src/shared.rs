//! Shared utilities for writing floats.

// TODO(ahuszagh) Remove this when we implement algorithm.
#![cfg_attr(not(feature = "compact"), allow(dead_code, unused_macros))]

#[cfg(any(feature = "radix", feature = "compact"))]
use lexical_util::digit::digit_to_char_const;
use lexical_util::format::NumberFormat;
use lexical_write_integer::write::WriteInteger;

/// Round-up the last digit, from a buffer of digits.
///
/// Round up the last digit, incrementally handling all subsequent
/// digits in case of overflow.
///
/// # Safety
///
/// Safe as long as `count <= digits.len()`.
#[cfg(any(feature = "radix", feature = "compact"))]
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
        let format = NumberFormat::<{ $format }> {};
        let min_exp = $options.negative_exponent_break().map_or(-5, |x| x.get());
        let max_exp = $options.positive_exponent_break().map_or(9, |x| x.get());

        let outside_break = $sci_exp < min_exp || $sci_exp > max_exp;
        let require_exponent = format.required_exponent_notation() || outside_break;
        if !format.no_exponent_notation() && require_exponent {
            // Validate our input: check if the format is invalid.
            assert_eq!(
                NumberFormat::<$format>::RADIX,
                NumberFormat::<$format>::EXPONENT_BASE,
                "If using exponent notation, the mantissa radix must equal the exponent base."
            );

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
