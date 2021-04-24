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
    /// a b'e' character for the exponent, a b'^' character for
    /// the exponent backup, and does not use the incorrect or
    /// lossy parser.
    ///
    /// Bit Flags Layout
    /// ----------------
    ///
    /// See `feature_format` for more in-depth information. This
    /// is a subset of those flags.
    ///
    /// The bitflags has the lower bits designated for flags that modify
    /// the parsing behavior of lexical, with 7 bits each set for the
    /// decimal point, exponent, and backup exponent, allowing any valid
    /// ASCII character as punctuation. Bits 12-18 are reserved for the
    /// radix (if the radix feature is enabled), bits 18-25 for the
    /// exponent character, bits 25-32 for the exponent backup character,
    /// bit 48 for the incorrect (fastest) float parser, bit 49 for the
    /// lossy (intermediate) float parser, bits 50-57 for the decimal
    /// point character, and the last 7 bits for the digit separator
    /// character.
    ///
    //
    /// ```text
    /// 0   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |                                               |      R/D      |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 16  17  18  19  20  21  22  23  24  25  26  27  28  29  30  31  32
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |  R/D  |         Exponent          |     Exponent Backup       |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 32  33  34  35  36  37  38  39  40  41 42  43  44  45  46  47   48
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |                                                               |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// 48  49  50  51  52  53  54  55  56  57  58  59  60  62  62  63  64
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |L/I|L/L|      Decimal Point        |                           |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// Where:
    ///     R/D = Radix (as a 6-bit integer).
    ///     L/I = Incorrect algorithm (everything done with native floats).
    ///     L/L = Lossy algorithm (using the fast and moderate paths).
    /// ```
    #[doc(hidden)]
    #[derive(Default)]
    pub struct NumberFormat: u64 {
        // CONVERSION PRECISION FLAGS & MASKS
        // See `flags` for documentation.

        #[doc(hidden)]
        const INCORRECT                             = flags::INCORRECT;

        #[doc(hidden)]
        const LOSSY                                 = flags::LOSSY;

        // HIDDEN DEFAULTS

        /// Standard float format.
        #[doc(hidden)]
        const STANDARD = (
            flags::radix_to_flags(10)   // TODO(ahuszagh) Remove.
            | flags::exponent_to_flags(b'e')
            | flags::exponent_backup_to_flags(b'^')
            | flags::decimal_point_to_flags(b'.')
        );
    }
}

impl NumberFormat {
    /// Create new format from bits.
    /// This method should **NEVER** be public, use the builder API.
    #[inline]
    pub(crate) const fn new(bits: u64) -> Self {
        Self { bits }
    }

    /// Create new format from bits.
    /// This method should **NEVER** be public, use the builder API.
    // TODO(ahuszagh) Remove: we should not have the radix here.
    #[inline]
    pub(crate) const fn from_radix(radix: u8) -> Self {
        Self::new(flags::radix_to_flags(radix))
    }

    // TODO(ahuszagh) Don't document these, since we'll be removing em soon.

    #[inline]
    pub const fn flags(self) -> Self {
        self
    }

    #[inline]
    pub const fn interface_flags(self) -> Self {
        self
    }

    // TODO(ahuszagh) Remove: we should not have the radix here.
    #[inline]
    pub const fn radix(self) -> u8 {
        flags::radix_from_flags(self.bits)
    }

    #[inline]
    pub const fn digit_separator(self) -> u8 {
        b'\x00'
    }

    #[inline]
    pub const fn decimal_point(self) -> u8 {
        flags::decimal_point_from_flags(self.bits)
    }

    #[inline]
    pub const fn exponent(self) -> u8 {
        flags::exponent_from_flags(self.bits)
    }

    #[inline]
    pub const fn exponent_backup(self) -> u8 {
        flags::exponent_backup_from_flags(self.bits)
    }

    #[inline]
    pub const fn required_integer_digits(self) -> bool {
        false
    }

    #[inline]
    pub const fn required_fraction_digits(self) -> bool {
        false
    }

    #[inline]
    pub const fn required_exponent_digits(self) -> bool {
        true
    }

    #[inline]
    pub const fn required_digits(self) -> bool {
        false
    }

    #[inline]
    pub const fn no_positive_mantissa_sign(self) -> bool {
        false
    }

    #[inline]
    pub const fn required_mantissa_sign(self) -> bool {
        false
    }

    #[inline]
    pub const fn no_exponent_notation(self) -> bool {
        false
    }

    #[inline]
    pub const fn no_positive_exponent_sign(self) -> bool {
        false
    }

    #[inline]
    pub const fn required_exponent_sign(self) -> bool {
        false
    }

    #[inline]
    pub const fn no_exponent_without_fraction(self) -> bool {
        false
    }

    #[inline]
    pub const fn no_special(self) -> bool {
        false
    }

    #[inline]
    pub const fn case_sensitive_special(self) -> bool {
        false
    }

    #[inline]
    pub const fn no_integer_leading_zeros(self) -> bool {
        false
    }

    #[inline]
    pub const fn no_float_leading_zeros(self) -> bool {
        false
    }

    #[inline]
    pub const fn integer_internal_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn fraction_internal_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn exponent_internal_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn internal_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn integer_leading_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn fraction_leading_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn exponent_leading_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn leading_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn integer_trailing_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn fraction_trailing_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn exponent_trailing_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn trailing_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn integer_consecutive_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn fraction_consecutive_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn exponent_consecutive_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn consecutive_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn special_digit_separator(self) -> bool {
        false
    }

    #[inline]
    pub const fn incorrect(self) -> bool {
        self.intersects(Self::INCORRECT)
    }

    #[inline]
    pub const fn lossy(self) -> bool {
        self.intersects(Self::LOSSY)
    }

    #[inline(always)]
    pub const fn builder() -> NumberFormatBuilder {
        NumberFormatBuilder::new()
    }

    #[inline]
    pub const fn rebuild(self) -> NumberFormatBuilder {
        NumberFormatBuilder {
            radix: self.radix(),
            decimal_point: self.decimal_point(),
            exponent: self.exponent(),
            exponent_backup: self.exponent_backup(),
            incorrect: self.incorrect(),
            lossy: self.lossy()
        }
    }
}

// NUMBER FORMAT BUILDER

/// Build float format value from specifications.
///
/// * `radix`                                   - Radix for number encoding or decoding.
/// * `decimal_point`                           - Character to designate the decimal point.
/// * `exponent`                                - Character to designate the exponent.
/// * `exponent_backup`                         - Backup character to designate the exponent for radix >= 0xE.
/// * `incorrect`                               - Use incorrect, but fast conversion routines.
/// * `lossy`                                   - Use lossy, but moderately fast, conversion routines.
///
/// Returns the format on calling build if it was able to compile the format,
/// otherwise, returns None.
#[derive(Debug, Clone)]
pub struct NumberFormatBuilder {
    radix: u8,
    decimal_point: u8,
    exponent: u8,
    exponent_backup: u8,
    incorrect: bool,
    lossy: bool,
}

impl NumberFormatBuilder {
    /// Create new NumberFormatBuilder with default arguments.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            radix: 10,
            decimal_point: b'.',
            exponent: b'e',
            exponent_backup: b'^',
            incorrect: false,
            lossy: false
        }
    }

    #[inline(always)]
    pub const fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    #[inline(always)]
    pub const fn decimal_point(mut self, decimal_point: u8) -> Self {
        self.decimal_point = decimal_point;
        self
    }

    #[inline(always)]
    pub const fn exponent(mut self, exponent: u8) -> Self {
        self.exponent = exponent;
        self
    }

    #[inline(always)]
    pub const fn exponent_backup(mut self, exponent_backup: u8) -> Self {
        self.exponent_backup = exponent_backup;
        self
    }

    #[inline(always)]
    pub const fn incorrect(mut self, incorrect: bool) -> Self {
        self.incorrect = incorrect;
        self
    }

    #[inline(always)]
    pub const fn lossy(mut self, lossy: bool) -> Self {
        self.lossy = lossy;
        self
    }

    #[inline]
    pub const fn build(&self) -> Option<NumberFormat> {
        let mut format = NumberFormat::new(0);

        // Add conversion precision flags.
        add_flag!(format, self.incorrect, INCORRECT);
        add_flag!(format, self.lossy, LOSSY);

        // Add punctuation characters.
        format.bits |= flags::decimal_point_to_flags(self.decimal_point);
        format.bits |= flags::exponent_to_flags(self.exponent);
        format.bits |= flags::exponent_backup_to_flags(self.exponent_backup);

        // Add radix
        format.bits |= flags::radix_to_flags(self.radix);

        // Validation.
        let is_invalid =
            !flags::is_valid_decimal_point(self.decimal_point)
            || !flags::is_valid_exponent(self.exponent)
            || !flags::is_valid_exponent_backup(self.exponent_backup)
            || !flags::is_valid_punctuation(b'\x00', self.decimal_point, self.exponent, self.exponent_backup)
            || !flags::is_valid_radix(self.radix)
            || self.incorrect && self.lossy;

        match is_invalid {
            true  => None,
            false => Some(format)
        }
    }
}

impl Default for NumberFormatBuilder {
    #[inline]
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
        assert_eq!(flag.exponent(), b'e');
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
        assert_eq!(flag.incorrect(), false);
        assert_eq!(flag.lossy(), false);

        #[cfg(feature ="radix")]
        assert_eq!(flag.radix(), 10);

        #[cfg(feature ="radix")]
        assert_eq!(flag.exponent_backup(), b'^');
    }

    #[test]
    fn test_builder() {
        // Test a few invalid ones.
        let flag = NumberFormat::builder().incorrect(true).lossy(true).build();
        assert_eq!(flag, None);

        let flag = NumberFormat::builder().exponent(b'.').build();
        assert_eq!(flag, None);

        // Test a few valid ones.
        let flag = NumberFormat::builder().incorrect(true).build();
        assert!(flag.is_some());
        let flag = flag.unwrap();
        assert_eq!(flag.radix(), 10);
        assert_eq!(flag.decimal_point(), b'.');
        assert_eq!(flag.exponent(), b'e');
        assert_eq!(flag.exponent_backup(), b'^');
        assert_eq!(flag.incorrect(), true);
        assert_eq!(flag.lossy(), false);
    }

    #[test]
    fn test_rebuild() {
        let flag = NumberFormat::STANDARD;
        let flag = flag.rebuild().lossy(true).build().unwrap();
        assert_eq!(flag.radix(), 10);
        assert_eq!(flag.lossy(), true);
    }
}
