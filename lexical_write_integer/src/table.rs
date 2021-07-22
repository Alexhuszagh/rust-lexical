//! Pre-computed tables for writing integral strings.

use crate::lib::hint;
use lexical_util::assert::debug_assert_radix;

// Re-export all the feature-specific files.
#[cfg(feature = "power_of_two")]
pub use crate::table_binary::*;
pub use crate::table_decimal::*;
#[cfg(feature = "radix")]
pub use crate::table_radix::*;

/// Precalculated table for a digit to a character.
///
/// Unoptimized table for radix N always, which translates a single digit to a
/// character, and also useful for radix-N float encoding.
pub const DIGIT_TO_CHAR: [u8; 36] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z',
];

/// Get character from digit.
///
/// # Safety
///
/// Safe as long as `digit < 36`.
#[inline]
pub unsafe fn digit_to_char(digit: usize) -> u8 {
    debug_assert!(digit < 36, "digit_to_char() invalid character.");
    unsafe { *DIGIT_TO_CHAR.get_unchecked(digit) }
}

/// Get lookup table for 2 digit radix conversions.
///
/// # Safety
///
/// Safe as long as the radix provided is valid.
#[inline]
#[cfg(feature = "radix")]
pub unsafe fn get_table(radix: u32) -> &'static [u8] {
    debug_assert_radix(radix);
    match radix {
        2 => &DIGIT_TO_BASE2_SQUARED,
        3 => &DIGIT_TO_BASE3_SQUARED,
        4 => &DIGIT_TO_BASE4_SQUARED,
        5 => &DIGIT_TO_BASE5_SQUARED,
        6 => &DIGIT_TO_BASE6_SQUARED,
        7 => &DIGIT_TO_BASE7_SQUARED,
        8 => &DIGIT_TO_BASE8_SQUARED,
        9 => &DIGIT_TO_BASE9_SQUARED,
        10 => &DIGIT_TO_BASE10_SQUARED,
        11 => &DIGIT_TO_BASE11_SQUARED,
        12 => &DIGIT_TO_BASE12_SQUARED,
        13 => &DIGIT_TO_BASE13_SQUARED,
        14 => &DIGIT_TO_BASE14_SQUARED,
        15 => &DIGIT_TO_BASE15_SQUARED,
        16 => &DIGIT_TO_BASE16_SQUARED,
        17 => &DIGIT_TO_BASE17_SQUARED,
        18 => &DIGIT_TO_BASE18_SQUARED,
        19 => &DIGIT_TO_BASE19_SQUARED,
        20 => &DIGIT_TO_BASE20_SQUARED,
        21 => &DIGIT_TO_BASE21_SQUARED,
        22 => &DIGIT_TO_BASE22_SQUARED,
        23 => &DIGIT_TO_BASE23_SQUARED,
        24 => &DIGIT_TO_BASE24_SQUARED,
        25 => &DIGIT_TO_BASE25_SQUARED,
        26 => &DIGIT_TO_BASE26_SQUARED,
        27 => &DIGIT_TO_BASE27_SQUARED,
        28 => &DIGIT_TO_BASE28_SQUARED,
        29 => &DIGIT_TO_BASE29_SQUARED,
        30 => &DIGIT_TO_BASE30_SQUARED,
        31 => &DIGIT_TO_BASE31_SQUARED,
        32 => &DIGIT_TO_BASE32_SQUARED,
        33 => &DIGIT_TO_BASE33_SQUARED,
        34 => &DIGIT_TO_BASE34_SQUARED,
        35 => &DIGIT_TO_BASE35_SQUARED,
        36 => &DIGIT_TO_BASE36_SQUARED,
        // SAFETY: This is safe as long as the radix is valid.
        _ => unsafe { hint::unreachable_unchecked() },
    }
}

/// Get lookup table for 2 digit radix conversions.
///
/// # Safety
///
/// Safe as long as the radix provided is valid.
#[inline]
#[cfg(all(feature = "power_of_two", not(feature = "radix")))]
pub unsafe fn get_table(radix: u32) -> &'static [u8] {
    debug_assert_radix(radix);
    match radix {
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
