//! Implementation of `SyntaxFormat` with the `format` feature disabled.

#![cfg(not(feature = "format"))]

use bitflags::bitflags;

// SYNTAX FORMAT

bitflags! {
    /// Bitflags for a number format.
    ///
    /// This is used to derive the high-level bitflags. This is a dummy
    /// implementation for API-compatibility, and therefore contains no data.
    /// See `SyntaxFormat` when the `format` feature is enabled for
    /// an explanation of the syntax format layout.
    /// ```
    #[repr(C)]
    #[repr(align(8))]
    #[derive(Default)]
    pub struct SyntaxFormat: u64 {
        // HIDDEN DEFAULTS

        /// Standard float format.
        const STANDARD = 0;
    }
}

impl SyntaxFormat {
    /// Create new format from bits.
    /// This method should **NEVER** be public, use the builder API.
    #[inline(always)]
    pub(crate) const fn new(bits: u64) -> Self {
        Self {
            bits,
        }
    }

    // FLAGS

    /// Get the flag bits from the compiled float format.
    #[inline(always)]
    pub const fn flags(self) -> Self {
        self
    }

    /// Get the interface flag bits from the compiled float format.
    #[inline(always)]
    pub const fn interface_flags(self) -> Self {
        self
    }

    // DIGIT SEPARATOR

    /// Get the digit separator for the number format.
    #[inline(always)]
    pub const fn digit_separator(self) -> u8 {
        b'\x00'
    }

    // NON-DIGIT SEPARATOR FLAGS & MASKS

    /// Get if digits are required before the decimal point.
    #[inline(always)]
    pub const fn required_integer_digits(self) -> bool {
        false
    }

    /// Get if digits are required after the decimal point.
    #[inline(always)]
    pub const fn required_fraction_digits(self) -> bool {
        false
    }

    /// Get if digits are required after the exponent character.
    #[inline(always)]
    pub const fn required_exponent_digits(self) -> bool {
        true
    }

    /// Get if digits are required before or after the decimal point.
    #[inline(always)]
    pub const fn required_digits(self) -> bool {
        true
    }

    /// Get if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn no_positive_mantissa_sign(self) -> bool {
        false
    }

    /// Get if a sign symbol before the mantissa is required.
    #[inline(always)]
    pub const fn required_mantissa_sign(self) -> bool {
        false
    }

    /// Get if exponent notation is not allowed.
    #[inline(always)]
    pub const fn no_exponent_notation(self) -> bool {
        false
    }

    /// Get if a positive sign before the exponent is not allowed.
    #[inline(always)]
    pub const fn no_positive_exponent_sign(self) -> bool {
        false
    }

    /// Get if a sign symbol before the exponent is required.
    #[inline(always)]
    pub const fn required_exponent_sign(self) -> bool {
        false
    }

    /// Get if an exponent without fraction is not allowed.
    #[inline(always)]
    pub const fn no_exponent_without_fraction(self) -> bool {
        false
    }

    /// Get if special (non-finite) values are not allowed.
    #[inline(always)]
    pub const fn no_special(self) -> bool {
        false
    }

    /// Get if special (non-finite) values are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_special(self) -> bool {
        false
    }

    /// Get if leading zeros before an integer are not allowed.
    #[inline(always)]
    pub const fn no_integer_leading_zeros(self) -> bool {
        false
    }

    /// Get if leading zeros before a float are not allowed.
    #[inline(always)]
    pub const fn no_float_leading_zeros(self) -> bool {
        false
    }

    /// Get if exponent notation is required.
    #[inline(always)]
    pub const fn required_exponent_notation(self) -> bool {
        false
    }

    /// Get if exponent characters are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_exponent(self) -> bool {
        false
    }

    /// Get if base prefixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_prefix(self) -> bool {
        false
    }

    /// Get if base suffixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_suffix(self) -> bool {
        false
    }

    // DIGIT SEPARATOR FLAGS & MASKS

    /// Get if digit separators are allowed between integer digits.
    #[inline(always)]
    pub const fn integer_internal_digit_separator(self) -> bool {
        false
    }

    /// Get if digit separators are allowed between fraction digits.
    #[inline(always)]
    pub const fn fraction_internal_digit_separator(self) -> bool {
        false
    }

    /// Get if digit separators are allowed between exponent digits.
    #[inline(always)]
    pub const fn exponent_internal_digit_separator(self) -> bool {
        false
    }

    /// Get if digit separators are allowed between digits.
    #[inline(always)]
    pub const fn internal_digit_separator(self) -> bool {
        false
    }

    /// Get if a digit separator is allowed before any integer digits.
    #[inline(always)]
    pub const fn integer_leading_digit_separator(self) -> bool {
        false
    }

    /// Get if a digit separator is allowed before any fraction digits.
    #[inline(always)]
    pub const fn fraction_leading_digit_separator(self) -> bool {
        false
    }

    /// Get if a digit separator is allowed before any exponent digits.
    #[inline(always)]
    pub const fn exponent_leading_digit_separator(self) -> bool {
        false
    }

    /// Get if a digit separator is allowed before any digits.
    #[inline(always)]
    pub const fn leading_digit_separator(self) -> bool {
        false
    }

    /// Get if a digit separator is allowed after any integer digits.
    #[inline(always)]
    pub const fn integer_trailing_digit_separator(self) -> bool {
        false
    }

    /// Get if a digit separator is allowed after any fraction digits.
    #[inline(always)]
    pub const fn fraction_trailing_digit_separator(self) -> bool {
        false
    }

    /// Get if a digit separator is allowed after any exponent digits.
    #[inline(always)]
    pub const fn exponent_trailing_digit_separator(self) -> bool {
        false
    }

    /// Get if a digit separator is allowed after any digits.
    #[inline(always)]
    pub const fn trailing_digit_separator(self) -> bool {
        false
    }

    /// Get if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    pub const fn integer_consecutive_digit_separator(self) -> bool {
        false
    }

    /// Get if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    pub const fn fraction_consecutive_digit_separator(self) -> bool {
        false
    }

    /// Get if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    pub const fn exponent_consecutive_digit_separator(self) -> bool {
        false
    }

    /// Get if multiple consecutive digit separators are allowed.
    #[inline(always)]
    pub const fn consecutive_digit_separator(self) -> bool {
        false
    }

    /// Get if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    pub const fn special_digit_separator(self) -> bool {
        false
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties() {
        let flag = SyntaxFormat::STANDARD;
        assert_eq!(flag.flags(), flag);
        assert_eq!(flag.interface_flags(), flag);
        assert_eq!(flag.digit_separator(), b'\x00');
        assert_eq!(flag.required_integer_digits(), false);
        assert_eq!(flag.required_fraction_digits(), false);
        assert_eq!(flag.required_exponent_digits(), true);
        assert_eq!(flag.required_digits(), true);
        assert_eq!(flag.no_positive_mantissa_sign(), false);
        assert_eq!(flag.required_mantissa_sign(), false);
        assert_eq!(flag.no_exponent_notation(), false);
        assert_eq!(flag.no_positive_exponent_sign(), false);
        assert_eq!(flag.required_exponent_sign(), false);
        assert_eq!(flag.no_exponent_without_fraction(), false);
        assert_eq!(flag.no_special(), false);
        assert_eq!(flag.case_sensitive_special(), false);
        assert_eq!(flag.no_integer_leading_zeros(), false);
        assert_eq!(flag.no_float_leading_zeros(), false);
        assert_eq!(flag.no_exponent_notation(), false);
        assert_eq!(flag.required_exponent_notation(), false);
        assert_eq!(flag.case_sensitive_exponent(), false);
        assert_eq!(flag.case_sensitive_base_prefix(), false);
        assert_eq!(flag.case_sensitive_base_suffix(), false);
        assert_eq!(flag.integer_internal_digit_separator(), false);
        assert_eq!(flag.fraction_internal_digit_separator(), false);
        assert_eq!(flag.exponent_internal_digit_separator(), false);
        assert_eq!(flag.internal_digit_separator(), false);
        assert_eq!(flag.integer_leading_digit_separator(), false);
        assert_eq!(flag.fraction_leading_digit_separator(), false);
        assert_eq!(flag.exponent_leading_digit_separator(), false);
        assert_eq!(flag.leading_digit_separator(), false);
        assert_eq!(flag.integer_trailing_digit_separator(), false);
        assert_eq!(flag.fraction_trailing_digit_separator(), false);
        assert_eq!(flag.exponent_trailing_digit_separator(), false);
        assert_eq!(flag.trailing_digit_separator(), false);
        assert_eq!(flag.integer_consecutive_digit_separator(), false);
        assert_eq!(flag.fraction_consecutive_digit_separator(), false);
        assert_eq!(flag.exponent_consecutive_digit_separator(), false);
        assert_eq!(flag.consecutive_digit_separator(), false);
        assert_eq!(flag.special_digit_separator(), false);
    }
}
