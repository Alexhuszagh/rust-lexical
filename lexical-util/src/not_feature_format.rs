//! Bare bones implementation of the format packed struct without feature
//! `format`.
//!
//! See `feature_format` for detailed documentation.

#![cfg(not(feature = "format"))]

use crate::error::Error;
use crate::format_builder::NumberFormatBuilder;
use crate::format_flags as flags;

/// Wrapper for the 128-bit packed struct.
///
/// The following values are explicitly set, and therefore not configurable:
///     1. required_integer_digits
///     2. required_fraction_digits
///     3. required_exponent_digits
///     4. required_mantissa_digits
///     5. required_digits
///     6. no_positive_mantissa_sign
///     7. required_mantissa_sign
///     8. no_exponent_notation
///     9. no_positive_exponent_sign
///     10. required_exponent_sign
///     11. no_exponent_without_fraction
///     12. no_special
///     13. case_sensitive_special
///     14. no_integer_leading_zeros
///     15. no_float_leading_zeros
///     16. required_exponent_notation
///     17. case_sensitive_exponent
///     18. case_sensitive_base_prefix
///     19. case_sensitive_base_suffix
///     20. integer_internal_digit_separator
///     21. fraction_internal_digit_separator
///     22. exponent_internal_digit_separator
///     23. internal_digit_separator
///     24. integer_leading_digit_separator
///     25. fraction_leading_digit_separator
///     26. exponent_leading_digit_separator
///     27. leading_digit_separator
///     28. integer_trailing_digit_separator
///     29. fraction_trailing_digit_separator
///     30. exponent_trailing_digit_separator
///     31. trailing_digit_separator
///     32. integer_consecutive_digit_separator
///     33. fraction_consecutive_digit_separator
///     34. exponent_consecutive_digit_separator
///     35. consecutive_digit_separator
///     36. special_digit_separator
///     37. digit_separator
///     38. base_prefix
///     39. base_suffix
///     40. exponent_base
///     41. exponent_radix
///
/// See `NumberFormatBuilder` for the `FORMAT` fields
/// for the packed struct.
#[doc(hidden)]
pub struct NumberFormat<const FORMAT: u128>;

impl<const FORMAT: u128> NumberFormat<FORMAT> {
    // CONSTRUCTORS

    /// Create new instance (for methods and validation).
    pub const fn new() -> Self {
        Self {}
    }

    // VALIDATION

    /// Determine if the number format is valid.
    pub const fn is_valid(&self) -> bool {
        self.error().is_success()
    }

    /// Get the error type from the format.
    pub const fn error(&self) -> Error {
        let valid_flags = flags::REQUIRED_EXPONENT_DIGITS | flags::REQUIRED_MANTISSA_DIGITS;
        if !flags::is_valid_radix(self.mantissa_radix()) {
            Error::InvalidMantissaRadix
        } else if !flags::is_valid_radix(self.exponent_base()) {
            Error::InvalidExponentBase
        } else if !flags::is_valid_radix(self.exponent_radix()) {
            Error::InvalidExponentRadix
        } else if !flags::is_valid_digit_separator(FORMAT) {
            Error::InvalidDigitSeparator
        } else if !flags::is_valid_base_prefix(FORMAT) {
            Error::InvalidBasePrefix
        } else if !flags::is_valid_base_suffix(FORMAT) {
            Error::InvalidBaseSuffix
        } else if !flags::is_valid_punctuation(FORMAT) {
            Error::InvalidPunctuation
        } else if self.flags() != valid_flags {
            Error::InvalidFlags
        } else {
            Error::Success
        }
    }

    // NON-DIGIT SEPARATOR FLAGS & MASKS

    /// If digits are required before the decimal point.
    pub const REQUIRED_INTEGER_DIGITS: bool = false;

    /// Get if digits are required before the decimal point.
    #[inline(always)]
    pub const fn required_integer_digits(&self) -> bool {
        Self::REQUIRED_INTEGER_DIGITS
    }

    /// If digits are required after the decimal point.
    pub const REQUIRED_FRACTION_DIGITS: bool = false;

    /// Get if digits are required after the decimal point.
    #[inline(always)]
    pub const fn required_fraction_digits(&self) -> bool {
        Self::REQUIRED_FRACTION_DIGITS
    }

    /// If digits are required after the exponent character.
    pub const REQUIRED_EXPONENT_DIGITS: bool = true;

    /// Get if digits are required after the exponent character.
    #[inline(always)]
    pub const fn required_exponent_digits(&self) -> bool {
        Self::REQUIRED_EXPONENT_DIGITS
    }

    /// If significant digits are required.
    pub const REQUIRED_MANTISSA_DIGITS: bool = true;

    /// Get if significant digits are required.
    #[inline(always)]
    pub const fn required_mantissa_digits(&self) -> bool {
        Self::REQUIRED_MANTISSA_DIGITS
    }

    /// If at least 1 digit in the number is required.
    pub const REQUIRED_DIGITS: bool = true;

    /// Get if at least 1 digit in the number is required.
    #[inline(always)]
    pub const fn required_digits(&self) -> bool {
        Self::REQUIRED_DIGITS
    }

    /// If a positive sign before the mantissa is not allowed.
    pub const NO_POSITIVE_MANTISSA_SIGN: bool = false;

    /// Get if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn no_positive_mantissa_sign(&self) -> bool {
        Self::NO_POSITIVE_MANTISSA_SIGN
    }

    /// If a sign symbol before the mantissa is required.
    pub const REQUIRED_MANTISSA_SIGN: bool = false;

    /// Get if a sign symbol before the mantissa is required.
    #[inline(always)]
    pub const fn required_mantissa_sign(&self) -> bool {
        Self::REQUIRED_MANTISSA_SIGN
    }

    /// If exponent notation is not allowed.
    pub const NO_EXPONENT_NOTATION: bool = false;

    /// Get if exponent notation is not allowed.
    #[inline(always)]
    pub const fn no_exponent_notation(&self) -> bool {
        Self::NO_EXPONENT_NOTATION
    }

    /// If a positive sign before the exponent is not allowed.
    pub const NO_POSITIVE_EXPONENT_SIGN: bool = false;

    /// Get if a positive sign before the exponent is not allowed.
    #[inline(always)]
    pub const fn no_positive_exponent_sign(&self) -> bool {
        Self::NO_POSITIVE_EXPONENT_SIGN
    }

    /// If a sign symbol before the exponent is required.
    pub const REQUIRED_EXPONENT_SIGN: bool = false;

    /// Get if a sign symbol before the exponent is required.
    #[inline(always)]
    pub const fn required_exponent_sign(&self) -> bool {
        Self::REQUIRED_EXPONENT_SIGN
    }

    /// If an exponent without fraction is not allowed.
    pub const NO_EXPONENT_WITHOUT_FRACTION: bool = false;

    /// Get if an exponent without fraction is not allowed.
    #[inline(always)]
    pub const fn no_exponent_without_fraction(&self) -> bool {
        Self::NO_EXPONENT_WITHOUT_FRACTION
    }

    /// If special (non-finite) values are not allowed.
    pub const NO_SPECIAL: bool = false;

    /// Get if special (non-finite) values are not allowed.
    #[inline(always)]
    pub const fn no_special(&self) -> bool {
        Self::NO_SPECIAL
    }

    /// If special (non-finite) values are case-sensitive.
    pub const CASE_SENSITIVE_SPECIAL: bool = false;

    /// Get if special (non-finite) values are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_special(&self) -> bool {
        Self::CASE_SENSITIVE_SPECIAL
    }

    /// If leading zeros before an integer are not allowed.
    pub const NO_INTEGER_LEADING_ZEROS: bool = false;

    /// Get if leading zeros before an integer are not allowed.
    #[inline(always)]
    pub const fn no_integer_leading_zeros(&self) -> bool {
        Self::NO_INTEGER_LEADING_ZEROS
    }

    /// If leading zeros before a float are not allowed.
    pub const NO_FLOAT_LEADING_ZEROS: bool = false;

    /// Get if leading zeros before a float are not allowed.
    #[inline(always)]
    pub const fn no_float_leading_zeros(&self) -> bool {
        Self::NO_FLOAT_LEADING_ZEROS
    }

    /// If exponent notation is required.
    pub const REQUIRED_EXPONENT_NOTATION: bool = false;

    /// Get if exponent notation is required.
    #[inline(always)]
    pub const fn required_exponent_notation(&self) -> bool {
        Self::REQUIRED_EXPONENT_NOTATION
    }

    /// If exponent characters are case-sensitive.
    pub const CASE_SENSITIVE_EXPONENT: bool = false;

    /// Get if exponent characters are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_exponent(&self) -> bool {
        Self::CASE_SENSITIVE_EXPONENT
    }

    /// If base prefixes are case-sensitive.
    pub const CASE_SENSITIVE_BASE_PREFIX: bool = false;

    /// Get if base prefixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_prefix(&self) -> bool {
        Self::CASE_SENSITIVE_BASE_PREFIX
    }

    /// If base suffixes are case-sensitive.
    pub const CASE_SENSITIVE_BASE_SUFFIX: bool = false;

    /// Get if base suffixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_suffix(&self) -> bool {
        Self::CASE_SENSITIVE_BASE_SUFFIX
    }

    // DIGIT SEPARATOR FLAGS & MASKS

    // If digit separators are allowed between integer digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    pub const INTEGER_INTERNAL_DIGIT_SEPARATOR: bool = false;

    /// Get if digit separators are allowed between integer digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    pub const fn integer_internal_digit_separator(&self) -> bool {
        Self::INTEGER_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between fraction digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    pub const FRACTION_INTERNAL_DIGIT_SEPARATOR: bool = false;

    /// Get if digit separators are allowed between fraction digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    pub const fn fraction_internal_digit_separator(&self) -> bool {
        Self::FRACTION_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between exponent digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    pub const EXPONENT_INTERNAL_DIGIT_SEPARATOR: bool = false;

    /// Get if digit separators are allowed between exponent digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    pub const fn exponent_internal_digit_separator(&self) -> bool {
        Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    pub const INTERNAL_DIGIT_SEPARATOR: bool = false;

    /// Get if digit separators are allowed between digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    pub const fn internal_digit_separator(&self) -> bool {
        Self::INTERNAL_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    pub const INTEGER_LEADING_DIGIT_SEPARATOR: bool = false;

    /// Get if a digit separator is allowed before any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn integer_leading_digit_separator(&self) -> bool {
        Self::INTEGER_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    pub const FRACTION_LEADING_DIGIT_SEPARATOR: bool = false;

    /// Get if a digit separator is allowed before any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn fraction_leading_digit_separator(&self) -> bool {
        Self::FRACTION_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    pub const EXPONENT_LEADING_DIGIT_SEPARATOR: bool = false;

    /// Get if a digit separator is allowed before any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn exponent_leading_digit_separator(&self) -> bool {
        Self::EXPONENT_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    pub const LEADING_DIGIT_SEPARATOR: bool = false;

    /// Get if a digit separator is allowed before any digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn leading_digit_separator(&self) -> bool {
        Self::LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    pub const INTEGER_TRAILING_DIGIT_SEPARATOR: bool = false;

    /// Get if a digit separator is allowed after any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn integer_trailing_digit_separator(&self) -> bool {
        Self::INTEGER_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    pub const FRACTION_TRAILING_DIGIT_SEPARATOR: bool = false;

    /// Get if a digit separator is allowed after any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn fraction_trailing_digit_separator(&self) -> bool {
        Self::FRACTION_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    pub const EXPONENT_TRAILING_DIGIT_SEPARATOR: bool = false;

    /// Get if a digit separator is allowed after any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn exponent_trailing_digit_separator(&self) -> bool {
        Self::EXPONENT_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    pub const TRAILING_DIGIT_SEPARATOR: bool = false;

    /// Get if a digit separator is allowed after any digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn trailing_digit_separator(&self) -> bool {
        Self::TRAILING_DIGIT_SEPARATOR
    }

    /// If multiple consecutive integer digit separators are allowed.
    pub const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR: bool = false;

    /// Get if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    pub const fn integer_consecutive_digit_separator(&self) -> bool {
        Self::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive fraction digit separators are allowed.
    pub const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR: bool = false;

    /// Get if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    pub const fn fraction_consecutive_digit_separator(&self) -> bool {
        Self::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive exponent digit separators are allowed.
    pub const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR: bool = false;

    /// Get if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    pub const fn exponent_consecutive_digit_separator(&self) -> bool {
        Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive digit separators are allowed.
    pub const CONSECUTIVE_DIGIT_SEPARATOR: bool = false;

    /// Get if multiple consecutive digit separators are allowed.
    #[inline(always)]
    pub const fn consecutive_digit_separator(&self) -> bool {
        Self::CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If any digit separators are allowed in special (non-finite) values.
    pub const SPECIAL_DIGIT_SEPARATOR: bool = false;

    /// Get if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    pub const fn special_digit_separator(&self) -> bool {
        Self::SPECIAL_DIGIT_SEPARATOR
    }

    // CHARACTERS

    /// The digit separator character in the packed struct.
    pub const DIGIT_SEPARATOR: u8 = 0;

    /// Get the digit separator character.
    ///
    /// If the digit separator is 0, digit separators are not allowed.
    #[inline(always)]
    pub const fn digit_separator(&self) -> u8 {
        Self::DIGIT_SEPARATOR
    }

    /// The base prefix character in the packed struct.
    pub const BASE_PREFIX: u8 = 0;

    /// Get the character for the base prefix.
    ///
    /// If the base prefix is 0, base prefixes are not allowed.
    /// The number will have then have the format `0$base_prefix...`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    #[inline(always)]
    pub const fn base_prefix(&self) -> u8 {
        Self::BASE_PREFIX
    }

    /// Get if the format has a base prefix.
    #[inline(always)]
    pub const fn has_base_prefix(&self) -> bool {
        false
    }

    /// The base suffix character in the packed struct.
    pub const BASE_SUFFIX: u8 = 0;

    /// Character for the base suffix.
    ///
    /// If not provided, base suffixes are not allowed.
    /// The number will have then have the format `...$base_suffix`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    #[inline(always)]
    pub const fn base_suffix(&self) -> u8 {
        Self::BASE_SUFFIX
    }

    /// Get if the format has a base suffix.
    #[inline(always)]
    pub const fn has_base_suffix(&self) -> bool {
        false
    }

    // RADIX

    /// The radix for the significant digits in the packed struct.
    pub const MANTISSA_RADIX: u32 = flags::mantissa_radix(FORMAT);

    /// Get the radix for the mantissa digits.
    #[inline(always)]
    pub const fn mantissa_radix(&self) -> u32 {
        Self::MANTISSA_RADIX
    }

    /// The radix for the significant digits in the packed struct.
    /// Alias for `MANTISSA_RADIX`.
    pub const RADIX: u32 = Self::MANTISSA_RADIX;

    /// Get the radix for the significant digits.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        Self::RADIX
    }

    /// Get the radix**2 for the significant digits.
    #[inline(always)]
    pub const fn radix2(&self) -> u32 {
        self.radix().wrapping_mul(self.radix())
    }

    /// Get the radix**4 for the significant digits.
    #[inline(always)]
    pub const fn radix4(&self) -> u32 {
        self.radix2().wrapping_mul(self.radix2())
    }

    /// Get the radix*** for the significant digits.
    #[inline(always)]
    pub const fn radix8(&self) -> u32 {
        self.radix4().wrapping_mul(self.radix4())
    }

    /// The base for the exponent.
    pub const EXPONENT_BASE: u32 = flags::exponent_base(FORMAT);

    /// Get the base for the exponent.
    ///
    /// IE, a base of 2 means we have `mantissa * 2^exponent`.
    /// If not provided, it defaults to `radix`.
    #[inline(always)]
    pub const fn exponent_base(&self) -> u32 {
        Self::EXPONENT_BASE
    }

    /// The radix for the exponent digits.
    pub const EXPONENT_RADIX: u32 = flags::exponent_radix(FORMAT);

    /// Get the radix for the exponent digits.
    #[inline(always)]
    pub const fn exponent_radix(&self) -> u32 {
        Self::EXPONENT_RADIX
    }

    // FLAGS

    /// Get the flags from the number format.
    #[inline(always)]
    pub const fn flags(&self) -> u128 {
        FORMAT & flags::FLAG_MASK
    }

    /// Get the interface flags from the number format.
    #[inline(always)]
    pub const fn interface_flags(&self) -> u128 {
        FORMAT & flags::INTERFACE_FLAG_MASK
    }

    /// Get the digit separator flags from the number format.
    #[inline(always)]
    pub const fn digit_separator_flags(&self) -> u128 {
        FORMAT & flags::DIGIT_SEPARATOR_FLAG_MASK
    }

    /// Get the exponent flags from the number format.
    #[inline(always)]
    pub const fn exponent_flags(&self) -> u128 {
        FORMAT & flags::EXPONENT_FLAG_MASK
    }

    /// Get the integer digit separator flags from the number format.
    #[inline(always)]
    pub const fn integer_digit_separator_flags(&self) -> u128 {
        FORMAT & flags::INTEGER_DIGIT_SEPARATOR_FLAG_MASK
    }

    /// Get the fraction digit separator flags from the number format.
    #[inline(always)]
    pub const fn fraction_digit_separator_flags(&self) -> u128 {
        FORMAT & flags::FRACTION_DIGIT_SEPARATOR_FLAG_MASK
    }

    /// Get the exponent digit separator flags from the number format.
    #[inline(always)]
    pub const fn exponent_digit_separator_flags(&self) -> u128 {
        FORMAT & flags::EXPONENT_DIGIT_SEPARATOR_FLAG_MASK
    }

    // BUILDER

    /// Get the number format builder from the format.
    #[inline]
    pub const fn builder() -> NumberFormatBuilder {
        NumberFormatBuilder::new()
    }

    /// Get the number format builder from the format.
    #[inline]
    pub const fn rebuild() -> NumberFormatBuilder {
        NumberFormatBuilder::rebuild(FORMAT)
    }
}

impl<const FORMAT: u128> Default for NumberFormat<FORMAT> {
    fn default() -> Self {
        Self::new()
    }
}
