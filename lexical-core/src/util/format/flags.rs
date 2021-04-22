//! Shared flags for number formats.

#![allow(dead_code)]

// We have a lot of flags that may not be enabled when the format
// feature is off, but we don't want to add cfg_if feature gates
// for every one.

use static_assertions::const_assert;

// MACROS
// ------

/// Add flag to flags
macro_rules! add_flag {
    ($flags:ident, $bool:expr, $flag:ident) => {
        if $bool {
            $flags |= NumberFormat::$flag;
        }
    };
}

// NON-DIGIT SEPARATOR FLAGS & MASKS
// ---------------------------------

/// Digits are required before the decimal point.
pub(super) const REQUIRED_INTEGER_DIGITS: u64              = 0b0000000000000000000000000000000000000000000000000000000000000001;

/// Digits are required after the decimal point.
/// This check will only occur if the decimal point is present.
pub(super) const REQUIRED_FRACTION_DIGITS: u64             = 0b0000000000000000000000000000000000000000000000000000000000000010;

/// Digits are required after the exponent character.
/// This check will only occur if the exponent character is present.
pub(super) const REQUIRED_EXPONENT_DIGITS: u64             = 0b0000000000000000000000000000000000000000000000000000000000000100;

/// Digits are required before or after the control characters.
pub(super) const REQUIRED_DIGITS: u64                      =
    REQUIRED_INTEGER_DIGITS
    | REQUIRED_FRACTION_DIGITS
    | REQUIRED_EXPONENT_DIGITS;

/// Positive sign before the mantissa is not allowed.
pub(super) const NO_POSITIVE_MANTISSA_SIGN: u64            = 0b0000000000000000000000000000000000000000000000000000000000001000;

/// Positive sign before the mantissa is required.
pub(super) const REQUIRED_MANTISSA_SIGN: u64               = 0b0000000000000000000000000000000000000000000000000000000000010000;

/// Exponent notation is not allowed.
pub(super) const NO_EXPONENT_NOTATION: u64                 = 0b0000000000000000000000000000000000000000000000000000000000100000;

/// Positive sign before the exponent is not allowed.
pub(super) const NO_POSITIVE_EXPONENT_SIGN: u64            = 0b0000000000000000000000000000000000000000000000000000000001000000;

/// Positive sign before the exponent is required.
pub(super) const REQUIRED_EXPONENT_SIGN: u64               = 0b0000000000000000000000000000000000000000000000000000000010000000;

/// Exponent without a fraction component is not allowed.
///
/// This only checks if a decimal point precedes the exponent character.
/// To require fraction digits or exponent digits with this check,
/// please use the appropriate flags.
pub(super) const NO_EXPONENT_WITHOUT_FRACTION: u64         = 0b0000000000000000000000000000000000000000000000000000000100000000;

/// Special (non-finite) values are not allowed.
pub(super) const NO_SPECIAL: u64                           = 0b0000000000000000000000000000000000000000000000000000001000000000;

/// Special (non-finite) values are case-sensitive.
pub(super) const CASE_SENSITIVE_SPECIAL: u64               = 0b0000000000000000000000000000000000000000000000000000010000000000;

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
pub(super) const NO_INTEGER_LEADING_ZEROS: u64             = 0b0000000000000000000000000000000000000000000000000000100000000000;

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
pub(super) const NO_FLOAT_LEADING_ZEROS: u64               = 0b0000000000000000000000000000000000000000000000000001000000000000;

// DIGIT SEPARATOR FLAGS & MASKS
// -----------------------------

/// Digit separators are allowed between integer digits.
pub(super) const INTEGER_INTERNAL_DIGIT_SEPARATOR: u64     = 0b0000000000000000000000000000000100000000000000000000000000000000;

/// A digit separator is allowed before any integer digits.
pub(super) const INTEGER_LEADING_DIGIT_SEPARATOR: u64      = 0b0000000000000000000000000000001000000000000000000000000000000000;

/// A digit separator is allowed after any integer digits.
pub(super) const INTEGER_TRAILING_DIGIT_SEPARATOR: u64     = 0b0000000000000000000000000000010000000000000000000000000000000000;

/// Multiple consecutive integer digit separators are allowed.
pub(super) const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR: u64  = 0b0000000000000000000000000000100000000000000000000000000000000000;

/// Digit separators are allowed between fraction digits.
pub(super) const FRACTION_INTERNAL_DIGIT_SEPARATOR: u64    = 0b0000000000000000000000000001000000000000000000000000000000000000;

/// A digit separator is allowed before any fraction digits.
pub(super) const FRACTION_LEADING_DIGIT_SEPARATOR: u64     = 0b0000000000000000000000000010000000000000000000000000000000000000;

/// A digit separator is allowed after any fraction digits.
pub(super) const FRACTION_TRAILING_DIGIT_SEPARATOR: u64    = 0b0000000000000000000000000100000000000000000000000000000000000000;

/// Multiple consecutive fraction digit separators are allowed.
pub(super) const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR: u64 = 0b0000000000000000000000001000000000000000000000000000000000000000;

/// Digit separators are allowed between exponent digits.
pub(super) const EXPONENT_INTERNAL_DIGIT_SEPARATOR: u64    = 0b0000000000000000000000010000000000000000000000000000000000000000;

/// A digit separator is allowed before any exponent digits.
pub(super) const EXPONENT_LEADING_DIGIT_SEPARATOR: u64     = 0b0000000000000000000000100000000000000000000000000000000000000000;

/// A digit separator is allowed after any exponent digits.
pub(super) const EXPONENT_TRAILING_DIGIT_SEPARATOR: u64    = 0b0000000000000000000001000000000000000000000000000000000000000000;

/// Multiple consecutive exponent digit separators are allowed.
pub(super) const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR: u64 = 0b0000000000000000000010000000000000000000000000000000000000000000;

/// Digit separators are allowed between digits.
pub(super) const INTERNAL_DIGIT_SEPARATOR: u64             =
    INTEGER_INTERNAL_DIGIT_SEPARATOR
    | FRACTION_INTERNAL_DIGIT_SEPARATOR
    | EXPONENT_INTERNAL_DIGIT_SEPARATOR;

/// A digit separator is allowed before any digits.
pub(super) const LEADING_DIGIT_SEPARATOR: u64              =
    INTEGER_LEADING_DIGIT_SEPARATOR
    | FRACTION_LEADING_DIGIT_SEPARATOR
    | EXPONENT_LEADING_DIGIT_SEPARATOR;

/// A digit separator is allowed after any digits.
pub(super) const TRAILING_DIGIT_SEPARATOR: u64             =
    INTEGER_TRAILING_DIGIT_SEPARATOR
    | FRACTION_TRAILING_DIGIT_SEPARATOR
    | EXPONENT_TRAILING_DIGIT_SEPARATOR;

/// Multiple consecutive digit separators are allowed.
pub(super) const CONSECUTIVE_DIGIT_SEPARATOR: u64          =
    INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
    | FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
    | EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR;

/// Any digit separators are allowed in special (non-finite) values.
pub(super) const SPECIAL_DIGIT_SEPARATOR: u64              = 0b0000000000000000000100000000000000000000000000000000000000000000;

// CONVERSION PRECISION FLAGS & MASKS
// These control the precision and speed of the conversion
// routines used to parse or serialize the number.

/// Use the fastest, incorrect parsing algorithm.
pub(super) const INCORRECT: u64                            = 0b0000000000000001000000000000000000000000000000000000000000000000;

/// Use the intermediate, lossy parsing algorithm.
pub(super) const LOSSY: u64                                = 0b0000000000000010000000000000000000000000000000000000000000000000;

// ASSERTIONS
// ----------

// Ensure all our bit flags are valid.
macro_rules! check_subsequent_flags {
    ($x:ident, $y:ident) => (
        const_assert!($x << 1 == $y);
    );
}

// Non-digit separator flags.
const_assert!(REQUIRED_INTEGER_DIGITS == 1);
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
check_subsequent_flags!(CASE_SENSITIVE_SPECIAL, NO_INTEGER_LEADING_ZEROS);
check_subsequent_flags!(NO_INTEGER_LEADING_ZEROS, NO_FLOAT_LEADING_ZEROS);

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

// Conversion precision flags
const_assert!(INCORRECT == 1 << 48);
check_subsequent_flags!(INCORRECT, LOSSY);

// VALIDATORS
// ----------

// Determine if character is valid ASCII.
#[inline]
fn is_ascii(ch: u8) -> bool {
    ch.is_ascii()
}

/// Determine if the digit separator is valid.
#[inline]
#[cfg(not(feature = "radix"))]
pub(super) fn is_valid_digit_separator(ch: u8) -> bool {
    match ch {
        b'0' ..= b'9' => false,
        b'+' | b'-'   => false,
        _             => is_ascii(ch)
    }
}

/// Determine if the digit separator is valid.
#[inline]
#[cfg(feature = "radix")]
pub(super) fn is_valid_digit_separator(ch: u8) -> bool {
    match ch {
        b'A' ..= b'Z' => false,
        b'a' ..= b'z' => false,
        b'0' ..= b'9' => false,
        b'+' | b'-'   => false,
        _             => is_ascii(ch)
    }
}

/// Determine if the decimal point is valid.
#[inline]
pub(super) fn is_valid_decimal_point(ch: u8) -> bool {
    is_valid_digit_separator(ch)
}

/// Determine if exponent character is valid.
#[inline]
pub(super) fn is_valid_exponent(ch: u8) -> bool {
    match ch {
        b'0' ..= b'9' => false,
        b'+' | b'-'   => false,
        _             => is_ascii(ch)
    }
}

/// Determine if the exponent backup character is valid.
#[inline]
pub(super) fn is_valid_exponent_backup(ch: u8) -> bool {
    is_valid_digit_separator(ch)
}

/// Determine if all of the "punctuation" characters are valid.
#[inline]
pub(super) fn is_valid_punctuation(digit_separator: u8, decimal_point: u8, exponent: u8, exponent_backup: u8) -> bool
{
    if digit_separator == decimal_point {
        false
    } else if digit_separator == exponent {
        false
    } else if digit_separator == exponent_backup {
        false
    } else if decimal_point == exponent {
        false
    } else if decimal_point == exponent_backup {
        false
    } else if exponent == exponent_backup {
        false
    } else {
        true
    }
}

/// Determine if the radix is valid.
#[cfg(not(feature = "radix"))]
#[inline]
pub(super) fn is_valid_radix(radix: u8) -> bool {
    radix == 10
}

/// Determine if the radix is valid.
#[cfg(feature = "radix")]
#[inline]
pub(super) fn is_valid_radix(radix: u8) -> bool {
    radix >= 2 && radix <= 36
}

// FLAG FUNCTIONS
// --------------

/// Convert radix to flags.
#[inline]
pub(super) const fn radix_to_flags(radix: u8) -> u64 {
    const MASK: u8 = 0x3F;
    ((radix & MASK) as u64) << 12
}

/// Extract radix from flags.
#[inline]
pub(super) const fn radix_from_flags(flag: u64) -> u8 {
    const MASK: u64 = 0x3F << 12;
    ((flag & MASK) >> 12) as u8
}

/// Convert exponent to flags.
#[inline]
pub(super) const fn exponent_to_flags(ch: u8) -> u64 {
    const MASK: u8 = 0x7F;
    ((ch & MASK) as u64) << 18
}

/// Extract exponent from flags.
#[inline]
pub(super) const fn exponent_from_flags(flag: u64) -> u8 {
    const MASK: u64 = 0x7F << 18;
    ((flag & MASK) >> 18) as u8
}

/// Convert exponent backup to flags.
#[inline]
pub(super) const fn exponent_backup_to_flags(ch: u8) -> u64 {
    const MASK: u8 = 0x7F;
    ((ch & MASK) as u64) << 25
}

/// Extract exponent backup from flags.
#[inline]
pub(super) const fn exponent_backup_from_flags(flag: u64) -> u8 {
    const MASK: u64 = 0x7F << 25;
    ((flag & MASK) >> 25) as u8
}

/// Convert decimal point to flags.
#[inline]
pub(super) const fn decimal_point_to_flags(ch: u8) -> u64 {
    const MASK: u8 = 0x7F;
    ((ch & MASK) as u64) << 50
}

/// Extract decimal point from flags.
#[inline]
pub(super) const fn decimal_point_from_flags(flag: u64) -> u8 {
    const MASK: u64 = 0x7F << 50;
    ((flag & MASK) >> 50) as u8
}

/// Convert digit separator to flags.
#[inline]
pub(super) const fn digit_separator_to_flags(ch: u8) -> u64 {
    const MASK: u8 = 0x7F;
    ((ch & MASK) as u64) << 57
}

/// Extract digit separator from flags.
#[cfg(feature = "format")]
#[inline]
pub(super) const fn digit_separator_from_flags(flag: u64) -> u8 {
    const MASK: u64 = 0x7F << 57;
    ((flag & MASK) >> 57) as u8
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_digit_separator() {
        assert_eq!(is_valid_digit_separator(b'_'), true);
        assert_eq!(is_valid_digit_separator(b'\''), true);
        assert_eq!(is_valid_digit_separator(b'.'), true);
        if cfg!(feature = "radix") {
            assert_eq!(is_valid_digit_separator(b'e'), false);
        } else {
            assert_eq!(is_valid_digit_separator(b'e'), true);
        }
        assert_eq!(is_valid_digit_separator(b'0'), false);
        assert_eq!(is_valid_digit_separator(128), false);
    }

    #[test]
    fn test_is_valid_decimal_point() {
        assert_eq!(is_valid_decimal_point(b'_'), true);
        assert_eq!(is_valid_decimal_point(b'\''), true);
        assert_eq!(is_valid_decimal_point(b'.'), true);
        if cfg!(feature = "radix") {
            assert_eq!(is_valid_decimal_point(b'e'), false);
        } else {
            assert_eq!(is_valid_decimal_point(b'e'), true);
        }
        assert_eq!(is_valid_decimal_point(b'0'), false);
        assert_eq!(is_valid_decimal_point(128), false);
    }

    #[test]
    fn test_is_valid_exponent() {
        assert_eq!(is_valid_exponent(b'_'), true);
        assert_eq!(is_valid_exponent(b'\''), true);
        assert_eq!(is_valid_exponent(b'.'), true);
        assert_eq!(is_valid_exponent(b'e'), true);
        assert_eq!(is_valid_exponent(b'0'), false);
        assert_eq!(is_valid_exponent(128), false);
    }
}
