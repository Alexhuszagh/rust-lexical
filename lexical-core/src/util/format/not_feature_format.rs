//! Dummy implementation of `format` without the feature enabled.

#![cfg(not(feature = "format"))]

use bitflags::bitflags;

use super::super::config;
use super::flags;
use super::traits::*;

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
            flags::radix_to_flags(10)
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
    pub(crate) fn new(bits: u64) -> Self {
        Self { bits }
    }

    /// Create new format from bits.
    /// This method should **NEVER** be public, use the builder API.
    #[inline]
    pub(crate) fn from_radix(radix: u8) -> Self {
        Self::new(flags::radix_to_flags(radix))
    }
}

impl Format for NumberFormat {
    #[inline]
    fn flags(self) -> Self {
        self
    }

    #[inline]
    fn interface_flags(self) -> Self {
        self
    }

    #[inline]
    fn radix(self) -> u8 {
        flags::radix_from_flags(self.bits)
    }

    #[inline]
    fn digit_separator(self) -> u8 {
        b'\x00'
    }

    #[inline]
    fn decimal_point(self) -> u8 {
        flags::decimal_point_from_flags(self.bits)
    }

    #[inline]
    fn exponent(self) -> u8 {
        flags::exponent_from_flags(self.bits)
    }

    #[inline]
    fn exponent_backup(self) -> u8 {
        flags::exponent_backup_from_flags(self.bits)
    }

    #[inline]
    fn required_integer_digits(self) -> bool {
        false
    }

    #[inline]
    fn required_fraction_digits(self) -> bool {
        false
    }

    #[inline]
    fn required_exponent_digits(self) -> bool {
        true
    }

    #[inline]
    fn required_digits(self) -> bool {
        false
    }

    #[inline]
    fn no_positive_mantissa_sign(self) -> bool {
        false
    }

    #[inline]
    fn required_mantissa_sign(self) -> bool {
        false
    }

    #[inline]
    fn no_exponent_notation(self) -> bool {
        false
    }

    #[inline]
    fn no_positive_exponent_sign(self) -> bool {
        false
    }

    #[inline]
    fn required_exponent_sign(self) -> bool {
        false
    }

    #[inline]
    fn no_exponent_without_fraction(self) -> bool {
        false
    }

    #[inline]
    fn no_special(self) -> bool {
        false
    }

    #[inline]
    fn case_sensitive_special(self) -> bool {
        false
    }

    #[inline]
    fn no_integer_leading_zeros(self) -> bool {
        false
    }

    #[inline]
    fn no_float_leading_zeros(self) -> bool {
        false
    }

    #[inline]
    fn integer_internal_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn fraction_internal_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn exponent_internal_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn internal_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn integer_leading_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn fraction_leading_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn exponent_leading_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn leading_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn integer_trailing_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn fraction_trailing_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn exponent_trailing_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn trailing_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn integer_consecutive_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn fraction_consecutive_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn exponent_consecutive_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn consecutive_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn special_digit_separator(self) -> bool {
        false
    }

    #[inline]
    fn incorrect(self) -> bool {
        self.intersects(Self::INCORRECT)
    }

    #[inline]
    fn lossy(self) -> bool {
        self.intersects(Self::LOSSY)
    }

    #[inline]
    fn compile(
        radix: u8,
        digit_separator: u8,
        decimal_point: u8,
        exponent: u8,
        exponent_backup: u8,
        required_integer_digits: bool,
        required_fraction_digits: bool,
        required_exponent_digits: bool,
        no_positive_mantissa_sign: bool,
        required_mantissa_sign: bool,
        no_exponent_notation: bool,
        no_positive_exponent_sign: bool,
        required_exponent_sign: bool,
        no_exponent_without_fraction: bool,
        no_special: bool,
        case_sensitive_special: bool,
        no_integer_leading_zeros: bool,
        no_float_leading_zeros: bool,
        integer_internal_digit_separator: bool,
        fraction_internal_digit_separator: bool,
        exponent_internal_digit_separator: bool,
        integer_leading_digit_separator: bool,
        fraction_leading_digit_separator: bool,
        exponent_leading_digit_separator: bool,
        integer_trailing_digit_separator: bool,
        fraction_trailing_digit_separator: bool,
        exponent_trailing_digit_separator: bool,
        integer_consecutive_digit_separator: bool,
        fraction_consecutive_digit_separator: bool,
        exponent_consecutive_digit_separator: bool,
        special_digit_separator: bool,
        incorrect: bool,
        lossy: bool
    ) -> Option<Self> {
        let builder = NumberFormatBuilder {
            radix,
            decimal_point,
            exponent,
            exponent_backup,
            incorrect,
            lossy
        };

        // Need to do our own validation since we ignore
        // most fields: these can't be set.
        let invalid = digit_separator != b'\x00'
            || required_integer_digits
            || required_fraction_digits
            || !required_exponent_digits
            || no_positive_mantissa_sign
            || required_mantissa_sign
            || no_exponent_notation
            || no_positive_exponent_sign
            || required_exponent_sign
            || no_exponent_without_fraction
            || no_special
            || case_sensitive_special
            || no_integer_leading_zeros
            || no_float_leading_zeros
            || integer_internal_digit_separator
            || fraction_internal_digit_separator
            || exponent_internal_digit_separator
            || integer_leading_digit_separator
            || fraction_leading_digit_separator
            || exponent_leading_digit_separator
            || integer_trailing_digit_separator
            || fraction_trailing_digit_separator
            || exponent_trailing_digit_separator
            || integer_consecutive_digit_separator
            || fraction_consecutive_digit_separator
            || exponent_consecutive_digit_separator
            || special_digit_separator;

        if invalid {
            None
        } else {
            builder.build()
        }
    }

    #[inline]
    fn standard() -> Option<Self> {
        Some(Self::STANDARD)
    }

    #[inline]
    fn permissive() -> Option<Self> {
        None
    }

    #[inline]
    fn ignore(_: u8) -> Option<Self> {
        None
    }

    #[cfg(test)]
    #[inline]
    fn from_separator(_: u8) -> Self {
        panic!("Not implemented with the format feature.");
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
    #[allow(deprecated)]    // Remove when we deprecate these methods.
    fn new() -> Self {
        #[cfg(feature = "radix")]
        let exponent_backup = config::get_exponent_backup_char();
        #[cfg(not(feature = "radix"))]
        let exponent_backup = b'^';

        Self {
            radix: 10,
            decimal_point: b'.',
            exponent: config::get_exponent_default_char(),
            exponent_backup,
            incorrect: false,
            lossy: false
        }
    }

    #[inline(always)]
    pub fn radix(&mut self, radix: u8) -> &mut Self {
        self.radix = radix;
        self
    }

    #[inline(always)]
    pub fn digit_separator(&mut self, _: u8) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn decimal_point(&mut self, decimal_point: u8) -> &mut Self {
        self.decimal_point = decimal_point;
        self
    }

    #[inline(always)]
    pub fn exponent(&mut self, exponent: u8) -> &mut Self {
        self.exponent = exponent;
        self
    }

    #[inline(always)]
    pub fn exponent_backup(&mut self, exponent_backup: u8) -> &mut Self {
        self.exponent_backup = exponent_backup;
        self
    }

    #[inline(always)]
    pub fn required_integer_digits(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn required_fraction_digits(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn required_exponent_digits(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn no_positive_mantissa_sign(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn required_mantissa_sign(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn no_exponent_notation(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn no_positive_exponent_sign(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn required_exponent_sign(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn no_exponent_without_fraction(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn no_special(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn case_sensitive_special(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn no_integer_leading_zeros(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn no_float_leading_zeros(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn integer_internal_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn fraction_internal_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn exponent_internal_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn integer_leading_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn fraction_leading_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn exponent_leading_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn integer_trailing_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn fraction_trailing_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn exponent_trailing_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn integer_consecutive_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn fraction_consecutive_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn exponent_consecutive_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn special_digit_separator(&mut self, _: bool) -> &mut Self {
        unimplemented!()
    }

    #[inline(always)]
    pub fn incorrect(&mut self, incorrect: bool) -> &mut Self {
        self.incorrect = incorrect;
        self
    }

    #[inline(always)]
    pub fn lossy(&mut self, lossy: bool) -> &mut Self {
        self.lossy = lossy;
        self
    }
}

impl Default for NumberFormatBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for NumberFormatBuilder {
    type Buildable = NumberFormat;

    #[inline]
    fn build(&self) -> Option<Self::Buildable> {
        let mut format = Self::Buildable::default();

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

impl Buildable for NumberFormat {
    type Builder = NumberFormatBuilder;

    #[inline(always)]
    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    #[inline]
    fn rebuild(&self) -> Self::Builder {
        Self::Builder {
            radix: self.radix(),
            decimal_point: self.decimal_point(),
            exponent: self.exponent(),
            exponent_backup: self.exponent_backup(),
            incorrect: self.incorrect(),
            lossy: self.lossy()
        }
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
    #[allow(deprecated)]
    fn test_compilers() {
        assert_eq!(NumberFormat::standard(), Some(NumberFormat::STANDARD));
        assert_eq!(NumberFormat::permissive(), None);
        assert_eq!(NumberFormat::ignore(b'\x00'), None);

        // Test compile.
        assert_eq!(None, NumberFormat::compile(
            10,
            b'\x00',
            b'\x00',
            b'\x00',
            b'\x00',
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false
        ));
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
