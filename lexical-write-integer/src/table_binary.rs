//! Pre-computed tables for writing non-decimal strings.

#![cfg(not(feature = "compact"))]
#![cfg(feature = "power-of-two")]
#![doc(hidden)]

#[cfg(not(feature = "radix"))]
use crate::table_decimal::*;
#[cfg(not(feature = "radix"))]
use core::hint;
#[cfg(not(feature = "radix"))]
use lexical_util::assert::debug_assert_radix;
#[cfg(not(feature = "radix"))]
use lexical_util::format::radix_from_flags;

/// Get lookup table for 2 digit radix conversions.
///
/// * `FORMAT` - Number format.
/// * `MASK` - Mask to extract the radix value.
/// * `SHIFT` - Shift to normalize the radix value in `[0, 0x3f]`.
///
/// # Safety
///
/// Safe as long as the radix provided is valid.
#[inline]
#[cfg(not(feature = "radix"))]
pub unsafe fn get_table<const FORMAT: u128, const MASK: u128, const SHIFT: i32>() -> &'static [u8] {
    debug_assert_radix(radix_from_flags(FORMAT, MASK, SHIFT));
    match radix_from_flags(FORMAT, MASK, SHIFT) {
        2 => &DIGIT_TO_BASE2_SQUARED,
        4 => &DIGIT_TO_BASE4_SQUARED,
        8 => &DIGIT_TO_BASE8_SQUARED,
        10 => &DIGIT_TO_BASE10_SQUARED,
        16 => &DIGIT_TO_BASE16_SQUARED,
        32 => &DIGIT_TO_BASE32_SQUARED,
        // SAFETY: This is safe as long as the radix is valid.
        _ => unsafe { hint::unreachable_unchecked() },
    }
}

// RADIX^2 TABLES
// --------------

pub const DIGIT_TO_BASE2_SQUARED: [u8; 8] = [b'0', b'0', b'0', b'1', b'1', b'0', b'1', b'1'];
pub const DIGIT_TO_BASE4_SQUARED: [u8; 32] = [
    b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3',
    b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3', b'3', b'0', b'3', b'1', b'3', b'2', b'3', b'3',
];
pub const DIGIT_TO_BASE8_SQUARED: [u8; 128] = [
    b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6', b'0', b'7',
    b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3', b'1', b'4', b'1', b'5', b'1', b'6', b'1', b'7',
    b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3', b'2', b'4', b'2', b'5', b'2', b'6', b'2', b'7',
    b'3', b'0', b'3', b'1', b'3', b'2', b'3', b'3', b'3', b'4', b'3', b'5', b'3', b'6', b'3', b'7',
    b'4', b'0', b'4', b'1', b'4', b'2', b'4', b'3', b'4', b'4', b'4', b'5', b'4', b'6', b'4', b'7',
    b'5', b'0', b'5', b'1', b'5', b'2', b'5', b'3', b'5', b'4', b'5', b'5', b'5', b'6', b'5', b'7',
    b'6', b'0', b'6', b'1', b'6', b'2', b'6', b'3', b'6', b'4', b'6', b'5', b'6', b'6', b'6', b'7',
    b'7', b'0', b'7', b'1', b'7', b'2', b'7', b'3', b'7', b'4', b'7', b'5', b'7', b'6', b'7', b'7',
];
pub const DIGIT_TO_BASE16_SQUARED: [u8; 512] = [
    b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6', b'0', b'7',
    b'0', b'8', b'0', b'9', b'0', b'A', b'0', b'B', b'0', b'C', b'0', b'D', b'0', b'E', b'0', b'F',
    b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3', b'1', b'4', b'1', b'5', b'1', b'6', b'1', b'7',
    b'1', b'8', b'1', b'9', b'1', b'A', b'1', b'B', b'1', b'C', b'1', b'D', b'1', b'E', b'1', b'F',
    b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3', b'2', b'4', b'2', b'5', b'2', b'6', b'2', b'7',
    b'2', b'8', b'2', b'9', b'2', b'A', b'2', b'B', b'2', b'C', b'2', b'D', b'2', b'E', b'2', b'F',
    b'3', b'0', b'3', b'1', b'3', b'2', b'3', b'3', b'3', b'4', b'3', b'5', b'3', b'6', b'3', b'7',
    b'3', b'8', b'3', b'9', b'3', b'A', b'3', b'B', b'3', b'C', b'3', b'D', b'3', b'E', b'3', b'F',
    b'4', b'0', b'4', b'1', b'4', b'2', b'4', b'3', b'4', b'4', b'4', b'5', b'4', b'6', b'4', b'7',
    b'4', b'8', b'4', b'9', b'4', b'A', b'4', b'B', b'4', b'C', b'4', b'D', b'4', b'E', b'4', b'F',
    b'5', b'0', b'5', b'1', b'5', b'2', b'5', b'3', b'5', b'4', b'5', b'5', b'5', b'6', b'5', b'7',
    b'5', b'8', b'5', b'9', b'5', b'A', b'5', b'B', b'5', b'C', b'5', b'D', b'5', b'E', b'5', b'F',
    b'6', b'0', b'6', b'1', b'6', b'2', b'6', b'3', b'6', b'4', b'6', b'5', b'6', b'6', b'6', b'7',
    b'6', b'8', b'6', b'9', b'6', b'A', b'6', b'B', b'6', b'C', b'6', b'D', b'6', b'E', b'6', b'F',
    b'7', b'0', b'7', b'1', b'7', b'2', b'7', b'3', b'7', b'4', b'7', b'5', b'7', b'6', b'7', b'7',
    b'7', b'8', b'7', b'9', b'7', b'A', b'7', b'B', b'7', b'C', b'7', b'D', b'7', b'E', b'7', b'F',
    b'8', b'0', b'8', b'1', b'8', b'2', b'8', b'3', b'8', b'4', b'8', b'5', b'8', b'6', b'8', b'7',
    b'8', b'8', b'8', b'9', b'8', b'A', b'8', b'B', b'8', b'C', b'8', b'D', b'8', b'E', b'8', b'F',
    b'9', b'0', b'9', b'1', b'9', b'2', b'9', b'3', b'9', b'4', b'9', b'5', b'9', b'6', b'9', b'7',
    b'9', b'8', b'9', b'9', b'9', b'A', b'9', b'B', b'9', b'C', b'9', b'D', b'9', b'E', b'9', b'F',
    b'A', b'0', b'A', b'1', b'A', b'2', b'A', b'3', b'A', b'4', b'A', b'5', b'A', b'6', b'A', b'7',
    b'A', b'8', b'A', b'9', b'A', b'A', b'A', b'B', b'A', b'C', b'A', b'D', b'A', b'E', b'A', b'F',
    b'B', b'0', b'B', b'1', b'B', b'2', b'B', b'3', b'B', b'4', b'B', b'5', b'B', b'6', b'B', b'7',
    b'B', b'8', b'B', b'9', b'B', b'A', b'B', b'B', b'B', b'C', b'B', b'D', b'B', b'E', b'B', b'F',
    b'C', b'0', b'C', b'1', b'C', b'2', b'C', b'3', b'C', b'4', b'C', b'5', b'C', b'6', b'C', b'7',
    b'C', b'8', b'C', b'9', b'C', b'A', b'C', b'B', b'C', b'C', b'C', b'D', b'C', b'E', b'C', b'F',
    b'D', b'0', b'D', b'1', b'D', b'2', b'D', b'3', b'D', b'4', b'D', b'5', b'D', b'6', b'D', b'7',
    b'D', b'8', b'D', b'9', b'D', b'A', b'D', b'B', b'D', b'C', b'D', b'D', b'D', b'E', b'D', b'F',
    b'E', b'0', b'E', b'1', b'E', b'2', b'E', b'3', b'E', b'4', b'E', b'5', b'E', b'6', b'E', b'7',
    b'E', b'8', b'E', b'9', b'E', b'A', b'E', b'B', b'E', b'C', b'E', b'D', b'E', b'E', b'E', b'F',
    b'F', b'0', b'F', b'1', b'F', b'2', b'F', b'3', b'F', b'4', b'F', b'5', b'F', b'6', b'F', b'7',
    b'F', b'8', b'F', b'9', b'F', b'A', b'F', b'B', b'F', b'C', b'F', b'D', b'F', b'E', b'F', b'F',
];
pub const DIGIT_TO_BASE32_SQUARED: [u8; 2048] = [
    b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6', b'0', b'7',
    b'0', b'8', b'0', b'9', b'0', b'A', b'0', b'B', b'0', b'C', b'0', b'D', b'0', b'E', b'0', b'F',
    b'0', b'G', b'0', b'H', b'0', b'I', b'0', b'J', b'0', b'K', b'0', b'L', b'0', b'M', b'0', b'N',
    b'0', b'O', b'0', b'P', b'0', b'Q', b'0', b'R', b'0', b'S', b'0', b'T', b'0', b'U', b'0', b'V',
    b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3', b'1', b'4', b'1', b'5', b'1', b'6', b'1', b'7',
    b'1', b'8', b'1', b'9', b'1', b'A', b'1', b'B', b'1', b'C', b'1', b'D', b'1', b'E', b'1', b'F',
    b'1', b'G', b'1', b'H', b'1', b'I', b'1', b'J', b'1', b'K', b'1', b'L', b'1', b'M', b'1', b'N',
    b'1', b'O', b'1', b'P', b'1', b'Q', b'1', b'R', b'1', b'S', b'1', b'T', b'1', b'U', b'1', b'V',
    b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3', b'2', b'4', b'2', b'5', b'2', b'6', b'2', b'7',
    b'2', b'8', b'2', b'9', b'2', b'A', b'2', b'B', b'2', b'C', b'2', b'D', b'2', b'E', b'2', b'F',
    b'2', b'G', b'2', b'H', b'2', b'I', b'2', b'J', b'2', b'K', b'2', b'L', b'2', b'M', b'2', b'N',
    b'2', b'O', b'2', b'P', b'2', b'Q', b'2', b'R', b'2', b'S', b'2', b'T', b'2', b'U', b'2', b'V',
    b'3', b'0', b'3', b'1', b'3', b'2', b'3', b'3', b'3', b'4', b'3', b'5', b'3', b'6', b'3', b'7',
    b'3', b'8', b'3', b'9', b'3', b'A', b'3', b'B', b'3', b'C', b'3', b'D', b'3', b'E', b'3', b'F',
    b'3', b'G', b'3', b'H', b'3', b'I', b'3', b'J', b'3', b'K', b'3', b'L', b'3', b'M', b'3', b'N',
    b'3', b'O', b'3', b'P', b'3', b'Q', b'3', b'R', b'3', b'S', b'3', b'T', b'3', b'U', b'3', b'V',
    b'4', b'0', b'4', b'1', b'4', b'2', b'4', b'3', b'4', b'4', b'4', b'5', b'4', b'6', b'4', b'7',
    b'4', b'8', b'4', b'9', b'4', b'A', b'4', b'B', b'4', b'C', b'4', b'D', b'4', b'E', b'4', b'F',
    b'4', b'G', b'4', b'H', b'4', b'I', b'4', b'J', b'4', b'K', b'4', b'L', b'4', b'M', b'4', b'N',
    b'4', b'O', b'4', b'P', b'4', b'Q', b'4', b'R', b'4', b'S', b'4', b'T', b'4', b'U', b'4', b'V',
    b'5', b'0', b'5', b'1', b'5', b'2', b'5', b'3', b'5', b'4', b'5', b'5', b'5', b'6', b'5', b'7',
    b'5', b'8', b'5', b'9', b'5', b'A', b'5', b'B', b'5', b'C', b'5', b'D', b'5', b'E', b'5', b'F',
    b'5', b'G', b'5', b'H', b'5', b'I', b'5', b'J', b'5', b'K', b'5', b'L', b'5', b'M', b'5', b'N',
    b'5', b'O', b'5', b'P', b'5', b'Q', b'5', b'R', b'5', b'S', b'5', b'T', b'5', b'U', b'5', b'V',
    b'6', b'0', b'6', b'1', b'6', b'2', b'6', b'3', b'6', b'4', b'6', b'5', b'6', b'6', b'6', b'7',
    b'6', b'8', b'6', b'9', b'6', b'A', b'6', b'B', b'6', b'C', b'6', b'D', b'6', b'E', b'6', b'F',
    b'6', b'G', b'6', b'H', b'6', b'I', b'6', b'J', b'6', b'K', b'6', b'L', b'6', b'M', b'6', b'N',
    b'6', b'O', b'6', b'P', b'6', b'Q', b'6', b'R', b'6', b'S', b'6', b'T', b'6', b'U', b'6', b'V',
    b'7', b'0', b'7', b'1', b'7', b'2', b'7', b'3', b'7', b'4', b'7', b'5', b'7', b'6', b'7', b'7',
    b'7', b'8', b'7', b'9', b'7', b'A', b'7', b'B', b'7', b'C', b'7', b'D', b'7', b'E', b'7', b'F',
    b'7', b'G', b'7', b'H', b'7', b'I', b'7', b'J', b'7', b'K', b'7', b'L', b'7', b'M', b'7', b'N',
    b'7', b'O', b'7', b'P', b'7', b'Q', b'7', b'R', b'7', b'S', b'7', b'T', b'7', b'U', b'7', b'V',
    b'8', b'0', b'8', b'1', b'8', b'2', b'8', b'3', b'8', b'4', b'8', b'5', b'8', b'6', b'8', b'7',
    b'8', b'8', b'8', b'9', b'8', b'A', b'8', b'B', b'8', b'C', b'8', b'D', b'8', b'E', b'8', b'F',
    b'8', b'G', b'8', b'H', b'8', b'I', b'8', b'J', b'8', b'K', b'8', b'L', b'8', b'M', b'8', b'N',
    b'8', b'O', b'8', b'P', b'8', b'Q', b'8', b'R', b'8', b'S', b'8', b'T', b'8', b'U', b'8', b'V',
    b'9', b'0', b'9', b'1', b'9', b'2', b'9', b'3', b'9', b'4', b'9', b'5', b'9', b'6', b'9', b'7',
    b'9', b'8', b'9', b'9', b'9', b'A', b'9', b'B', b'9', b'C', b'9', b'D', b'9', b'E', b'9', b'F',
    b'9', b'G', b'9', b'H', b'9', b'I', b'9', b'J', b'9', b'K', b'9', b'L', b'9', b'M', b'9', b'N',
    b'9', b'O', b'9', b'P', b'9', b'Q', b'9', b'R', b'9', b'S', b'9', b'T', b'9', b'U', b'9', b'V',
    b'A', b'0', b'A', b'1', b'A', b'2', b'A', b'3', b'A', b'4', b'A', b'5', b'A', b'6', b'A', b'7',
    b'A', b'8', b'A', b'9', b'A', b'A', b'A', b'B', b'A', b'C', b'A', b'D', b'A', b'E', b'A', b'F',
    b'A', b'G', b'A', b'H', b'A', b'I', b'A', b'J', b'A', b'K', b'A', b'L', b'A', b'M', b'A', b'N',
    b'A', b'O', b'A', b'P', b'A', b'Q', b'A', b'R', b'A', b'S', b'A', b'T', b'A', b'U', b'A', b'V',
    b'B', b'0', b'B', b'1', b'B', b'2', b'B', b'3', b'B', b'4', b'B', b'5', b'B', b'6', b'B', b'7',
    b'B', b'8', b'B', b'9', b'B', b'A', b'B', b'B', b'B', b'C', b'B', b'D', b'B', b'E', b'B', b'F',
    b'B', b'G', b'B', b'H', b'B', b'I', b'B', b'J', b'B', b'K', b'B', b'L', b'B', b'M', b'B', b'N',
    b'B', b'O', b'B', b'P', b'B', b'Q', b'B', b'R', b'B', b'S', b'B', b'T', b'B', b'U', b'B', b'V',
    b'C', b'0', b'C', b'1', b'C', b'2', b'C', b'3', b'C', b'4', b'C', b'5', b'C', b'6', b'C', b'7',
    b'C', b'8', b'C', b'9', b'C', b'A', b'C', b'B', b'C', b'C', b'C', b'D', b'C', b'E', b'C', b'F',
    b'C', b'G', b'C', b'H', b'C', b'I', b'C', b'J', b'C', b'K', b'C', b'L', b'C', b'M', b'C', b'N',
    b'C', b'O', b'C', b'P', b'C', b'Q', b'C', b'R', b'C', b'S', b'C', b'T', b'C', b'U', b'C', b'V',
    b'D', b'0', b'D', b'1', b'D', b'2', b'D', b'3', b'D', b'4', b'D', b'5', b'D', b'6', b'D', b'7',
    b'D', b'8', b'D', b'9', b'D', b'A', b'D', b'B', b'D', b'C', b'D', b'D', b'D', b'E', b'D', b'F',
    b'D', b'G', b'D', b'H', b'D', b'I', b'D', b'J', b'D', b'K', b'D', b'L', b'D', b'M', b'D', b'N',
    b'D', b'O', b'D', b'P', b'D', b'Q', b'D', b'R', b'D', b'S', b'D', b'T', b'D', b'U', b'D', b'V',
    b'E', b'0', b'E', b'1', b'E', b'2', b'E', b'3', b'E', b'4', b'E', b'5', b'E', b'6', b'E', b'7',
    b'E', b'8', b'E', b'9', b'E', b'A', b'E', b'B', b'E', b'C', b'E', b'D', b'E', b'E', b'E', b'F',
    b'E', b'G', b'E', b'H', b'E', b'I', b'E', b'J', b'E', b'K', b'E', b'L', b'E', b'M', b'E', b'N',
    b'E', b'O', b'E', b'P', b'E', b'Q', b'E', b'R', b'E', b'S', b'E', b'T', b'E', b'U', b'E', b'V',
    b'F', b'0', b'F', b'1', b'F', b'2', b'F', b'3', b'F', b'4', b'F', b'5', b'F', b'6', b'F', b'7',
    b'F', b'8', b'F', b'9', b'F', b'A', b'F', b'B', b'F', b'C', b'F', b'D', b'F', b'E', b'F', b'F',
    b'F', b'G', b'F', b'H', b'F', b'I', b'F', b'J', b'F', b'K', b'F', b'L', b'F', b'M', b'F', b'N',
    b'F', b'O', b'F', b'P', b'F', b'Q', b'F', b'R', b'F', b'S', b'F', b'T', b'F', b'U', b'F', b'V',
    b'G', b'0', b'G', b'1', b'G', b'2', b'G', b'3', b'G', b'4', b'G', b'5', b'G', b'6', b'G', b'7',
    b'G', b'8', b'G', b'9', b'G', b'A', b'G', b'B', b'G', b'C', b'G', b'D', b'G', b'E', b'G', b'F',
    b'G', b'G', b'G', b'H', b'G', b'I', b'G', b'J', b'G', b'K', b'G', b'L', b'G', b'M', b'G', b'N',
    b'G', b'O', b'G', b'P', b'G', b'Q', b'G', b'R', b'G', b'S', b'G', b'T', b'G', b'U', b'G', b'V',
    b'H', b'0', b'H', b'1', b'H', b'2', b'H', b'3', b'H', b'4', b'H', b'5', b'H', b'6', b'H', b'7',
    b'H', b'8', b'H', b'9', b'H', b'A', b'H', b'B', b'H', b'C', b'H', b'D', b'H', b'E', b'H', b'F',
    b'H', b'G', b'H', b'H', b'H', b'I', b'H', b'J', b'H', b'K', b'H', b'L', b'H', b'M', b'H', b'N',
    b'H', b'O', b'H', b'P', b'H', b'Q', b'H', b'R', b'H', b'S', b'H', b'T', b'H', b'U', b'H', b'V',
    b'I', b'0', b'I', b'1', b'I', b'2', b'I', b'3', b'I', b'4', b'I', b'5', b'I', b'6', b'I', b'7',
    b'I', b'8', b'I', b'9', b'I', b'A', b'I', b'B', b'I', b'C', b'I', b'D', b'I', b'E', b'I', b'F',
    b'I', b'G', b'I', b'H', b'I', b'I', b'I', b'J', b'I', b'K', b'I', b'L', b'I', b'M', b'I', b'N',
    b'I', b'O', b'I', b'P', b'I', b'Q', b'I', b'R', b'I', b'S', b'I', b'T', b'I', b'U', b'I', b'V',
    b'J', b'0', b'J', b'1', b'J', b'2', b'J', b'3', b'J', b'4', b'J', b'5', b'J', b'6', b'J', b'7',
    b'J', b'8', b'J', b'9', b'J', b'A', b'J', b'B', b'J', b'C', b'J', b'D', b'J', b'E', b'J', b'F',
    b'J', b'G', b'J', b'H', b'J', b'I', b'J', b'J', b'J', b'K', b'J', b'L', b'J', b'M', b'J', b'N',
    b'J', b'O', b'J', b'P', b'J', b'Q', b'J', b'R', b'J', b'S', b'J', b'T', b'J', b'U', b'J', b'V',
    b'K', b'0', b'K', b'1', b'K', b'2', b'K', b'3', b'K', b'4', b'K', b'5', b'K', b'6', b'K', b'7',
    b'K', b'8', b'K', b'9', b'K', b'A', b'K', b'B', b'K', b'C', b'K', b'D', b'K', b'E', b'K', b'F',
    b'K', b'G', b'K', b'H', b'K', b'I', b'K', b'J', b'K', b'K', b'K', b'L', b'K', b'M', b'K', b'N',
    b'K', b'O', b'K', b'P', b'K', b'Q', b'K', b'R', b'K', b'S', b'K', b'T', b'K', b'U', b'K', b'V',
    b'L', b'0', b'L', b'1', b'L', b'2', b'L', b'3', b'L', b'4', b'L', b'5', b'L', b'6', b'L', b'7',
    b'L', b'8', b'L', b'9', b'L', b'A', b'L', b'B', b'L', b'C', b'L', b'D', b'L', b'E', b'L', b'F',
    b'L', b'G', b'L', b'H', b'L', b'I', b'L', b'J', b'L', b'K', b'L', b'L', b'L', b'M', b'L', b'N',
    b'L', b'O', b'L', b'P', b'L', b'Q', b'L', b'R', b'L', b'S', b'L', b'T', b'L', b'U', b'L', b'V',
    b'M', b'0', b'M', b'1', b'M', b'2', b'M', b'3', b'M', b'4', b'M', b'5', b'M', b'6', b'M', b'7',
    b'M', b'8', b'M', b'9', b'M', b'A', b'M', b'B', b'M', b'C', b'M', b'D', b'M', b'E', b'M', b'F',
    b'M', b'G', b'M', b'H', b'M', b'I', b'M', b'J', b'M', b'K', b'M', b'L', b'M', b'M', b'M', b'N',
    b'M', b'O', b'M', b'P', b'M', b'Q', b'M', b'R', b'M', b'S', b'M', b'T', b'M', b'U', b'M', b'V',
    b'N', b'0', b'N', b'1', b'N', b'2', b'N', b'3', b'N', b'4', b'N', b'5', b'N', b'6', b'N', b'7',
    b'N', b'8', b'N', b'9', b'N', b'A', b'N', b'B', b'N', b'C', b'N', b'D', b'N', b'E', b'N', b'F',
    b'N', b'G', b'N', b'H', b'N', b'I', b'N', b'J', b'N', b'K', b'N', b'L', b'N', b'M', b'N', b'N',
    b'N', b'O', b'N', b'P', b'N', b'Q', b'N', b'R', b'N', b'S', b'N', b'T', b'N', b'U', b'N', b'V',
    b'O', b'0', b'O', b'1', b'O', b'2', b'O', b'3', b'O', b'4', b'O', b'5', b'O', b'6', b'O', b'7',
    b'O', b'8', b'O', b'9', b'O', b'A', b'O', b'B', b'O', b'C', b'O', b'D', b'O', b'E', b'O', b'F',
    b'O', b'G', b'O', b'H', b'O', b'I', b'O', b'J', b'O', b'K', b'O', b'L', b'O', b'M', b'O', b'N',
    b'O', b'O', b'O', b'P', b'O', b'Q', b'O', b'R', b'O', b'S', b'O', b'T', b'O', b'U', b'O', b'V',
    b'P', b'0', b'P', b'1', b'P', b'2', b'P', b'3', b'P', b'4', b'P', b'5', b'P', b'6', b'P', b'7',
    b'P', b'8', b'P', b'9', b'P', b'A', b'P', b'B', b'P', b'C', b'P', b'D', b'P', b'E', b'P', b'F',
    b'P', b'G', b'P', b'H', b'P', b'I', b'P', b'J', b'P', b'K', b'P', b'L', b'P', b'M', b'P', b'N',
    b'P', b'O', b'P', b'P', b'P', b'Q', b'P', b'R', b'P', b'S', b'P', b'T', b'P', b'U', b'P', b'V',
    b'Q', b'0', b'Q', b'1', b'Q', b'2', b'Q', b'3', b'Q', b'4', b'Q', b'5', b'Q', b'6', b'Q', b'7',
    b'Q', b'8', b'Q', b'9', b'Q', b'A', b'Q', b'B', b'Q', b'C', b'Q', b'D', b'Q', b'E', b'Q', b'F',
    b'Q', b'G', b'Q', b'H', b'Q', b'I', b'Q', b'J', b'Q', b'K', b'Q', b'L', b'Q', b'M', b'Q', b'N',
    b'Q', b'O', b'Q', b'P', b'Q', b'Q', b'Q', b'R', b'Q', b'S', b'Q', b'T', b'Q', b'U', b'Q', b'V',
    b'R', b'0', b'R', b'1', b'R', b'2', b'R', b'3', b'R', b'4', b'R', b'5', b'R', b'6', b'R', b'7',
    b'R', b'8', b'R', b'9', b'R', b'A', b'R', b'B', b'R', b'C', b'R', b'D', b'R', b'E', b'R', b'F',
    b'R', b'G', b'R', b'H', b'R', b'I', b'R', b'J', b'R', b'K', b'R', b'L', b'R', b'M', b'R', b'N',
    b'R', b'O', b'R', b'P', b'R', b'Q', b'R', b'R', b'R', b'S', b'R', b'T', b'R', b'U', b'R', b'V',
    b'S', b'0', b'S', b'1', b'S', b'2', b'S', b'3', b'S', b'4', b'S', b'5', b'S', b'6', b'S', b'7',
    b'S', b'8', b'S', b'9', b'S', b'A', b'S', b'B', b'S', b'C', b'S', b'D', b'S', b'E', b'S', b'F',
    b'S', b'G', b'S', b'H', b'S', b'I', b'S', b'J', b'S', b'K', b'S', b'L', b'S', b'M', b'S', b'N',
    b'S', b'O', b'S', b'P', b'S', b'Q', b'S', b'R', b'S', b'S', b'S', b'T', b'S', b'U', b'S', b'V',
    b'T', b'0', b'T', b'1', b'T', b'2', b'T', b'3', b'T', b'4', b'T', b'5', b'T', b'6', b'T', b'7',
    b'T', b'8', b'T', b'9', b'T', b'A', b'T', b'B', b'T', b'C', b'T', b'D', b'T', b'E', b'T', b'F',
    b'T', b'G', b'T', b'H', b'T', b'I', b'T', b'J', b'T', b'K', b'T', b'L', b'T', b'M', b'T', b'N',
    b'T', b'O', b'T', b'P', b'T', b'Q', b'T', b'R', b'T', b'S', b'T', b'T', b'T', b'U', b'T', b'V',
    b'U', b'0', b'U', b'1', b'U', b'2', b'U', b'3', b'U', b'4', b'U', b'5', b'U', b'6', b'U', b'7',
    b'U', b'8', b'U', b'9', b'U', b'A', b'U', b'B', b'U', b'C', b'U', b'D', b'U', b'E', b'U', b'F',
    b'U', b'G', b'U', b'H', b'U', b'I', b'U', b'J', b'U', b'K', b'U', b'L', b'U', b'M', b'U', b'N',
    b'U', b'O', b'U', b'P', b'U', b'Q', b'U', b'R', b'U', b'S', b'U', b'T', b'U', b'U', b'U', b'V',
    b'V', b'0', b'V', b'1', b'V', b'2', b'V', b'3', b'V', b'4', b'V', b'5', b'V', b'6', b'V', b'7',
    b'V', b'8', b'V', b'9', b'V', b'A', b'V', b'B', b'V', b'C', b'V', b'D', b'V', b'E', b'V', b'F',
    b'V', b'G', b'V', b'H', b'V', b'I', b'V', b'J', b'V', b'K', b'V', b'L', b'V', b'M', b'V', b'N',
    b'V', b'O', b'V', b'P', b'V', b'Q', b'V', b'R', b'V', b'S', b'V', b'T', b'V', b'U', b'V', b'V',
];
