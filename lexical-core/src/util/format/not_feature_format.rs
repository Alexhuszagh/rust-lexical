//! Dummy implementation of `format` without the feature enabled.

#![cfg(not(feature = "format"))]

use bitflags::bitflags;

use super::flags;

// NUMBER FORMAT

bitflags! {
    /// Bitflags for a number format.
    ///
    /// This is used to derive the high-level bitflags. The default
    /// representation has a b'.' character for the decimal point,
    /// a b'e' character for the exponent default, and a b'^' character
    /// for the exponent backup.
    ///
    /// Bit Flags Layout
    /// ----------------
    ///
    /// See `NumberFormat` when the `format` feature is enabled for
    /// more in-depth information. This is a subset of those flags.
    ///
    /// The bitflags has the lower bits designated for flags that modify
    /// the parsing behavior of lexical, with 7 bits each set for the
    /// decimal point, default exponent, and backup exponent, allowing
    /// any valid ASCII character as punctuation. Bits 18-25 for the
    /// exponent default character, bits 25-32 for the exponent backup
    /// character, bits 50-57 for the decimal point character, and the
    /// last 7 bits for the digit separator character.
    ///
    /// ```text
    /// 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |                                               |               |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |       |     Exponent Default      |     Exponent Backup       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 32  33  34  35  36  37  38  39  40  41 42  43  44  45  46  47   48
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |                                                               |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 48  49  50  51  52  53  54  55  56  57  58  59  60  62  62  63  64
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |       |      Decimal Point        |                           |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// ```
    #[derive(Default)]
    pub struct NumberFormat: u64 {
        // HIDDEN DEFAULTS

        /// Standard float format.
        const STANDARD = (
            flags::exponent_default_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
        );
    }
}

impl NumberFormat {
    /// Create new format from bits.
    /// This method should **NEVER** be public, use the builder API.
    #[inline(always)]
    pub(crate) const fn new(bits: u64) -> Self {
        Self { bits }
    }

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

    /// Get the digit separator for the number format.
    #[inline(always)]
    pub const fn digit_separator(self) -> u8 {
        b'\x00'
    }

    /// Get the decimal point character for the number format.
    #[inline(always)]
    pub const fn decimal_point(self) -> u8 {
        flags::decimal_point_from_flags(self.bits)
    }

    /// Get the default exponent character for the number format.
    #[inline(always)]
    pub const fn exponent_default(self) -> u8 {
        flags::exponent_default_from_flags(self.bits)
    }

    /// Get the backup exponent character for the number format.
    #[inline(always)]
    pub const fn exponent_backup(self) -> u8 {
        flags::exponent_backup_from_flags(self.bits)
    }

    const_fn!(
    /// Get the exponent character based on the radix.
    #[inline(always)]
    pub const fn exponent(self, radix: u32) -> u8 {
        if cfg!(feature = "radix") && radix >= 14 {
            self.exponent_backup()
        } else {
            self.exponent_default()
        }
    });

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
        false
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

    // BUILDERS

    /// Create new builder to instantiate `NumberFormat`.
    #[inline(always)]
    pub const fn builder() -> NumberFormatBuilder {
        NumberFormatBuilder::new()
    }

    /// Recreate `NumberFormatBuilder` using current `NumberFormat` values.
    #[inline(always)]
    pub const fn rebuild(self) -> NumberFormatBuilder {
        NumberFormatBuilder {
            decimal_point: self.decimal_point(),
            exponent_default: self.exponent_default(),
            exponent_backup: self.exponent_backup()
        }
    }
}

// NUMBER FORMAT BUILDER

/// Build float format value from specifications.
///
/// * `decimal_point`                           - Character to designate the decimal point.
/// * `exponent_default`                        - Default character to designate the exponent.
/// * `exponent_backup`                         - Backup character to designate the exponent for radix >= 0xE.
///
/// Returns the format on calling build if it was able to compile the format,
/// otherwise, returns None.
#[derive(Debug, Clone)]
pub struct NumberFormatBuilder {
    decimal_point: u8,
    exponent_default: u8,
    exponent_backup: u8,
}

impl NumberFormatBuilder {
    /// Create new NumberFormatBuilder with default arguments.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            decimal_point: b'.',
            exponent_default: b'e',
            exponent_backup: b'^',
        }
    }

    // SETTERS

    /// Set the decimal point character for the number format.
    #[inline(always)]
    pub const fn decimal_point(mut self, decimal_point: u8) -> Self {
        self.decimal_point = decimal_point;
        self
    }

    const_fn!(
    /// Set the default exponent character for the number format.
    #[inline(always)]
    pub const fn exponent_default(mut self, exponent_default: u8) -> Self {
        self.exponent_default = flags::to_ascii_lowercase(exponent_default);
        self
    });

    const_fn!(
    /// Set the backup exponent character for the number format.
    #[inline(always)]
    pub const fn exponent_backup(mut self, exponent_backup: u8) -> Self {
        self.exponent_backup = flags::to_ascii_lowercase(exponent_backup);
        self
    });

    // BUILDER

    const_fn!(
    /// Create `NumberFormat` from builder options.
    ///
    /// If the format is invalid, this function will return `None`.
    #[inline]
    pub const fn build(&self) -> Option<NumberFormat> {
        let mut format = NumberFormat::new(0);

        // Add punctuation characters.
        format.bits |= flags::decimal_point_to_flags(self.decimal_point);
        format.bits |= flags::exponent_default_to_flags(self.exponent_default);
        format.bits |= flags::exponent_backup_to_flags(self.exponent_backup);

        // Validation.
        let is_invalid =
            !flags::is_valid_decimal_point(self.decimal_point)
            || !flags::is_valid_exponent_default(self.exponent_default)
            || !flags::is_valid_exponent_backup(self.exponent_backup)
            || !flags::is_valid_punctuation(b'\x00', self.decimal_point, self.exponent_default, self.exponent_backup);

        match is_invalid {
            true  => None,
            false => Some(format)
        }
    });
}

impl Default for NumberFormatBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties() {
        let flag = NumberFormat::STANDARD;
        assert_eq!(flag.flags(), flag);
        assert_eq!(flag.interface_flags(), flag);
        assert_eq!(flag.digit_separator(), b'\x00');
        assert_eq!(flag.decimal_point(), b'.');
        assert_eq!(flag.exponent_default(), b'e');
        assert_eq!(flag.required_integer_digits(), false);
        assert_eq!(flag.required_fraction_digits(), false);
        assert_eq!(flag.required_exponent_digits(), true);
        assert_eq!(flag.required_digits(), false);
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

        #[cfg(feature ="radix")]
        assert_eq!(flag.exponent_backup(), b'^');
    }

    #[test]
    fn test_builder() {
        // Test a few invalid ones.
        let flag = NumberFormat::builder().exponent_default(b'.').build();
        assert_eq!(flag, None);

        // Test a few valid ones.
        let flag = NumberFormat::builder().decimal_point(b'.').build();
        assert!(flag.is_some());
        let flag = flag.unwrap();
        assert_eq!(flag.decimal_point(), b'.');
        assert_eq!(flag.exponent_default(), b'e');
        assert_eq!(flag.exponent_backup(), b'^');
    }

    #[test]
    fn test_rebuild() {
        let flag = NumberFormat::STANDARD;
        let flag = flag.rebuild().decimal_point(b',').build().unwrap();
        assert_eq!(flag.decimal_point(), b',');
        assert_eq!(flag.exponent_default(), b'e');
        assert_eq!(flag.exponent_backup(), b'^');

        let flag = flag.rebuild().exponent_default(b'f').build().unwrap();
        assert_eq!(flag.decimal_point(), b',');
        assert_eq!(flag.exponent_default(), b'f');
        assert_eq!(flag.exponent_backup(), b'^');

        let flag = flag.rebuild().exponent_backup(b'$').build().unwrap();
        assert_eq!(flag.decimal_point(), b',');
        assert_eq!(flag.exponent_default(), b'f');
        assert_eq!(flag.exponent_backup(), b'$');
    }
}
