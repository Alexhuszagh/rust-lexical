//! Pre-computed tables for writing integral strings.

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
