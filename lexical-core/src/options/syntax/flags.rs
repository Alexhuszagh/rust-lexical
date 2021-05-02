//! Shared flags for syntax formats.

#![cfg_attr(not(feature = "format"), allow(dead_code))]

// We have a lot of flags that may not be enabled when the format
// feature is off, but we don't want to add cfg_if feature gates
// for every one.

use static_assertions::const_assert;

// NON-DIGIT SEPARATOR FLAGS & MASKS
// ---------------------------------

/// Digits are required before the decimal point.
pub(crate) const REQUIRED_INTEGER_DIGITS: u64              = 0b0000000000000000000000000000000000000000000000000000000100000000;

/// Digits are required after the decimal point.
/// This check will only occur if the decimal point is present.
pub(crate) const REQUIRED_FRACTION_DIGITS: u64             = 0b0000000000000000000000000000000000000000000000000000001000000000;

/// Digits are required after the exponent character.
/// This check will only occur if the exponent character is present.
pub(crate) const REQUIRED_EXPONENT_DIGITS: u64             = 0b0000000000000000000000000000000000000000000000000000010000000000;

/// Digits are required before or after the control characters.
pub(crate) const REQUIRED_DIGITS: u64                      =
    REQUIRED_INTEGER_DIGITS
    | REQUIRED_FRACTION_DIGITS
    | REQUIRED_EXPONENT_DIGITS;

/// Positive sign before the mantissa is not allowed.
pub(crate) const NO_POSITIVE_MANTISSA_SIGN: u64            = 0b0000000000000000000000000000000000000000000000000000100000000000;

/// Positive sign before the mantissa is required.
pub(crate) const REQUIRED_MANTISSA_SIGN: u64               = 0b0000000000000000000000000000000000000000000000000001000000000000;

/// Exponent notation is not allowed.
pub(crate) const NO_EXPONENT_NOTATION: u64                 = 0b0000000000000000000000000000000000000000000000000010000000000000;

/// Positive sign before the exponent is not allowed.
pub(crate) const NO_POSITIVE_EXPONENT_SIGN: u64            = 0b0000000000000000000000000000000000000000000000000100000000000000;

/// Positive sign before the exponent is required.
pub(crate) const REQUIRED_EXPONENT_SIGN: u64               = 0b0000000000000000000000000000000000000000000000001000000000000000;

/// Exponent without a fraction component is not allowed.
///
/// This only checks if a decimal point precedes the exponent character.
/// To require fraction digits or exponent digits with this check,
/// please use the appropriate flags.
pub(crate) const NO_EXPONENT_WITHOUT_FRACTION: u64         = 0b0000000000000000000000000000000000000000000000010000000000000000;

/// Special (non-finite) values are not allowed.
pub(crate) const NO_SPECIAL: u64                           = 0b0000000000000000000000000000000000000000000000100000000000000000;

/// Special (non-finite) values are case-sensitive.
pub(crate) const CASE_SENSITIVE_SPECIAL: u64               = 0b0000000000000000000000000000000000000000000001000000000000000000;

/// Leading zeros before an integer value are not allowed.
///
/// If the value is a literal, then this distinction applies
/// when the value is treated like an integer literal, typically
/// when there is no decimal point. If the value is parsed,
/// then this distinction applies when the value as parsed
/// as an integer.
///
/// # Warning
///
/// This also does not mean that the value parsed will be correct,
/// for example, in languages like C, this will not auto-
/// deduce that the radix is 8 with leading zeros, for an octal
/// literal.
pub(crate) const NO_INTEGER_LEADING_ZEROS: u64             = 0b0000000000000000000000000000000000000000000010000000000000000000;

/// Leading zeros before a float value are not allowed.
///
/// If the value is a literal, then this distinction applies
/// when the value is treated like an integer float, typically
/// when there is a decimal point. If the value is parsed,
/// then this distinction applies when the value as parsed
/// as a float.
///
/// # Warning
///
/// This also does not mean that the value parsed will be correct,
/// for example, in languages like C, this will not auto-
/// deduce that the radix is 8 with leading zeros, for an octal
/// literal.
pub(crate) const NO_FLOAT_LEADING_ZEROS: u64               = 0b0000000000000000000000000000000000000000000100000000000000000000;

/// Exponent notation is required.
///
/// Valid floats must contain an exponent notation character, and if
/// applicable, a sign character and digits afterwards.
pub(crate) const REQUIRED_EXPONENT_NOTATION: u64           = 0b0000000000000000000000000000000000000000001000000000000000000000;

/// Exponent characters are case-sensitive.
pub(crate) const CASE_SENSITIVE_EXPONENT: u64              = 0b0000000000000000000000000000000000000000010000000000000000000000;

/// Base prefixes are case-sensitive.
pub(crate) const CASE_SENSITIVE_BASE_PREFIX: u64           = 0b0000000000000000000000000000000000000000100000000000000000000000;

/// Base suffixes are case-sensitive.
pub(crate) const CASE_SENSITIVE_BASE_SUFFIX: u64           = 0b0000000000000000000000000000000000000001000000000000000000000000;

// DIGIT SEPARATOR FLAGS & MASKS
// -----------------------------

/// Digit separators are allowed between integer digits.
pub(crate) const INTEGER_INTERNAL_DIGIT_SEPARATOR: u64     = 0b0000000000000000000000000000000100000000000000000000000000000000;

/// A digit separator is allowed before any integer digits.
pub(crate) const INTEGER_LEADING_DIGIT_SEPARATOR: u64      = 0b0000000000000000000000000000001000000000000000000000000000000000;

/// A digit separator is allowed after any integer digits.
pub(crate) const INTEGER_TRAILING_DIGIT_SEPARATOR: u64     = 0b0000000000000000000000000000010000000000000000000000000000000000;

/// Multiple consecutive integer digit separators are allowed.
pub(crate) const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR: u64  = 0b0000000000000000000000000000100000000000000000000000000000000000;

/// Digit separators are allowed between fraction digits.
pub(crate) const FRACTION_INTERNAL_DIGIT_SEPARATOR: u64    = 0b0000000000000000000000000001000000000000000000000000000000000000;

/// A digit separator is allowed before any fraction digits.
pub(crate) const FRACTION_LEADING_DIGIT_SEPARATOR: u64     = 0b0000000000000000000000000010000000000000000000000000000000000000;

/// A digit separator is allowed after any fraction digits.
pub(crate) const FRACTION_TRAILING_DIGIT_SEPARATOR: u64    = 0b0000000000000000000000000100000000000000000000000000000000000000;

/// Multiple consecutive fraction digit separators are allowed.
pub(crate) const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR: u64 = 0b0000000000000000000000001000000000000000000000000000000000000000;

/// Digit separators are allowed between exponent digits.
pub(crate) const EXPONENT_INTERNAL_DIGIT_SEPARATOR: u64    = 0b0000000000000000000000010000000000000000000000000000000000000000;

/// A digit separator is allowed before any exponent digits.
pub(crate) const EXPONENT_LEADING_DIGIT_SEPARATOR: u64     = 0b0000000000000000000000100000000000000000000000000000000000000000;

/// A digit separator is allowed after any exponent digits.
pub(crate) const EXPONENT_TRAILING_DIGIT_SEPARATOR: u64    = 0b0000000000000000000001000000000000000000000000000000000000000000;

/// Multiple consecutive exponent digit separators are allowed.
pub(crate) const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR: u64 = 0b0000000000000000000010000000000000000000000000000000000000000000;

/// Digit separators are allowed between digits.
pub(crate) const INTERNAL_DIGIT_SEPARATOR: u64             =
    INTEGER_INTERNAL_DIGIT_SEPARATOR
    | FRACTION_INTERNAL_DIGIT_SEPARATOR
    | EXPONENT_INTERNAL_DIGIT_SEPARATOR;

/// A digit separator is allowed before any digits.
pub(crate) const LEADING_DIGIT_SEPARATOR: u64              =
    INTEGER_LEADING_DIGIT_SEPARATOR
    | FRACTION_LEADING_DIGIT_SEPARATOR
    | EXPONENT_LEADING_DIGIT_SEPARATOR;

/// A digit separator is allowed after any digits.
pub(crate) const TRAILING_DIGIT_SEPARATOR: u64             =
    INTEGER_TRAILING_DIGIT_SEPARATOR
    | FRACTION_TRAILING_DIGIT_SEPARATOR
    | EXPONENT_TRAILING_DIGIT_SEPARATOR;

/// Multiple consecutive digit separators are allowed.
pub(crate) const CONSECUTIVE_DIGIT_SEPARATOR: u64          =
    INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
    | FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
    | EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR;

/// Any digit separators are allowed in special (non-finite) values.
pub(crate) const SPECIAL_DIGIT_SEPARATOR: u64              = 0b0000000000000000000100000000000000000000000000000000000000000000;

// FLAG ASSERTIONS
// ---------------

// Ensure all our bit flags are valid.
macro_rules! check_subsequent_flags {
    ($x:ident, $y:ident) => (
        const_assert!($x << 1 == $y);
    );
}

// Non-digit separator flags.
const_assert!(REQUIRED_INTEGER_DIGITS == 1 << 8);
check_subsequent_flags!(REQUIRED_INTEGER_DIGITS, REQUIRED_FRACTION_DIGITS);
check_subsequent_flags!(REQUIRED_FRACTION_DIGITS, REQUIRED_EXPONENT_DIGITS);
check_subsequent_flags!(REQUIRED_EXPONENT_DIGITS, NO_POSITIVE_MANTISSA_SIGN);
check_subsequent_flags!(NO_POSITIVE_MANTISSA_SIGN, REQUIRED_MANTISSA_SIGN);
check_subsequent_flags!(REQUIRED_MANTISSA_SIGN, NO_EXPONENT_NOTATION);
check_subsequent_flags!(NO_EXPONENT_NOTATION, NO_POSITIVE_EXPONENT_SIGN);
check_subsequent_flags!(NO_POSITIVE_EXPONENT_SIGN, REQUIRED_EXPONENT_SIGN);
check_subsequent_flags!(REQUIRED_EXPONENT_SIGN, NO_EXPONENT_WITHOUT_FRACTION);
check_subsequent_flags!(NO_EXPONENT_WITHOUT_FRACTION, NO_SPECIAL);
check_subsequent_flags!(NO_SPECIAL, CASE_SENSITIVE_SPECIAL);
check_subsequent_flags!(NO_SPECIAL, CASE_SENSITIVE_SPECIAL);
check_subsequent_flags!(CASE_SENSITIVE_SPECIAL, NO_INTEGER_LEADING_ZEROS);
check_subsequent_flags!(NO_INTEGER_LEADING_ZEROS, NO_FLOAT_LEADING_ZEROS);
check_subsequent_flags!(NO_FLOAT_LEADING_ZEROS, REQUIRED_EXPONENT_NOTATION);
check_subsequent_flags!(REQUIRED_EXPONENT_NOTATION, CASE_SENSITIVE_EXPONENT);
check_subsequent_flags!(CASE_SENSITIVE_EXPONENT, CASE_SENSITIVE_BASE_PREFIX);
check_subsequent_flags!(CASE_SENSITIVE_BASE_PREFIX, CASE_SENSITIVE_BASE_SUFFIX);

// Digit separator flags.
const_assert!(INTEGER_INTERNAL_DIGIT_SEPARATOR == 1 << 32);
check_subsequent_flags!(INTEGER_INTERNAL_DIGIT_SEPARATOR, INTEGER_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_LEADING_DIGIT_SEPARATOR, INTEGER_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_TRAILING_DIGIT_SEPARATOR, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_CONSECUTIVE_DIGIT_SEPARATOR, FRACTION_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_INTERNAL_DIGIT_SEPARATOR, FRACTION_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_LEADING_DIGIT_SEPARATOR, FRACTION_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_TRAILING_DIGIT_SEPARATOR, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_CONSECUTIVE_DIGIT_SEPARATOR, EXPONENT_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_INTERNAL_DIGIT_SEPARATOR, EXPONENT_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_LEADING_DIGIT_SEPARATOR, EXPONENT_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_TRAILING_DIGIT_SEPARATOR, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR, SPECIAL_DIGIT_SEPARATOR);

// FLAG FUNCTIONS
// --------------

/// Convert digit separator to flags.
#[inline]
pub(crate) const fn digit_separator_to_flags(ch: u8) -> u64 {
    ch as u64
}

/// Extract digit separator from flags.
#[inline]
#[cfg(any(test, feature = "format"))]
pub(crate) const fn digit_separator_from_flags(flag: u64) -> u8 {
    (flag & 0xFF) as u8
}

// MASK ASSERTIONS
// ---------------

// Ensure all our bit masks don't overlap with existing flags.
macro_rules! check_masks_and_flags {
    ($xm:expr, $xs:expr, $f:expr) => (
        const_assert!((($xm as u64) << $xs) & $f == 0);
    );
}

// Check masks don't overlap with neighboring flags.
check_masks_and_flags!(0xFF, 0, REQUIRED_INTEGER_DIGITS);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_separator_to_flags() {
        assert_eq!(digit_separator_to_flags(b'e'), 0x65);
        assert_eq!(digit_separator_to_flags(b'^'), 0x5E);
        assert_eq!(digit_separator_to_flags(b'.'), 0x2E);
        assert_eq!(digit_separator_to_flags(b'\x00'), 0x00);
    }

    #[test]
    fn test_digit_separator_from_flags() {
        assert_eq!(digit_separator_from_flags(0x65), b'e');
        assert_eq!(digit_separator_from_flags(0x5E), b'^');
        assert_eq!(digit_separator_from_flags(0x5F), b'_');
        assert_eq!(digit_separator_from_flags(0x2E), b'.');
        assert_eq!(digit_separator_from_flags(0x00), b'\x00');

        // Test hybrid, to test mask
        let flags = 0x5F | 0xF00;
        assert_eq!(digit_separator_from_flags(flags), b'_');
    }
}
