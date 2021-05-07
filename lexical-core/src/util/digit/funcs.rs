//! Utilities to convert and identify digits.

#![cfg_attr(rustfmt, rustfmt::skip)]

use super::decimal::*;

// DIGITS
// ------

const_fn!(
/// Get if the character is a digit.
/// Optimize for case when we have a radix <= 10.
#[inline(always)]
#[cfg(feature = "power_of_two")]
pub(crate) const fn is_digit(c: u8, radix: u32) -> bool {
    let digit = if radix <= 10 {
        match c {
            b'0'..=b'9' => c - b'0',
            _ => return false,
        }
    } else {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'z' => c - b'a' + 10,
            b'A'..=b'Z' => c - b'A' + 10,
            _ => return false,
        }
    };
    (digit as u32) < radix
});

const_fn!(
/// Get if the character is a digit.
/// Optimize for case when we have a radix == 10.
#[inline(always)]
#[cfg(not(feature = "power_of_two"))]
pub(crate) const fn is_digit(c: u8, _: u32) -> bool {
    let digit = match c {
        b'0'..=b'9' => c - b'0',
        _ => return false,
    };
    (digit as u32) < 10
});

/// Get if the character is not a digit.
#[inline(always)]
pub(crate) fn is_not_digit_char(c: u8, radix: u32) -> bool {
    !is_digit(c, radix)
}

// Convert character to digit.
#[inline(always)]
pub(crate) fn to_digit(c: u8, radix: u32) -> Option<u32> {
    (c as char).to_digit(radix)
}

// Convert character to digit.
#[inline(always)]
pub(crate) fn to_digit_err<'a>(c: &'a u8, radix: u32) -> Result<u32, &'a u8> {
    match to_digit(*c, radix) {
        Some(v) => Ok(v),
        None => Err(c),
    }
}

/// Get character from digit.
#[inline(always)]
pub(crate) fn digit_to_char(digit: usize) -> u8 {
    debug_assert!(digit < 36, "digit_to_char() invalid character.");
    DIGIT_TO_CHAR[digit]
}
