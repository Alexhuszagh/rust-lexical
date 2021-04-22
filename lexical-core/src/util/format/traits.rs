//! Traits for Formats that implement the builder pattern.

// TRAIT

/// Trait for type builders.
pub trait Builder {
    type Buildable: Buildable;

    /// Consume builder and create type.
    fn build(&self) -> Option<Self::Buildable>;
}

/// Trait for types that can be constructed through a builder.
pub trait Buildable {
    type Builder: Builder;

    /// Create builder to instantiate the class.
    fn builder() -> Self::Builder;

    /// Recreate builder using values from existing type.
    fn rebuild(&self) -> Self::Builder;
}

/// Trait to create parse and write formats.
pub trait Format: Default + Copy + Clone + Send {
    // PROPERTIES

    /// Get the flag bits from the compiled float format.
    fn flags(self) -> Self;

    /// Get the interface flag bits from the compiled float format.
    fn interface_flags(self) -> Self;

    /// Get the radix for number encoding or decoding.
    fn radix(self) -> u8;

    /// Get the digit separator for the number format.
    fn digit_separator(self) -> u8;

    /// Get the decimal point character from the compiled float format.
    fn decimal_point(self) -> u8;

    /// Get the exponent character from the compiled float format.
    fn exponent(self) -> u8;

    /// Get the exponent backup character from the compiled float format.
    fn exponent_backup(self) -> u8;

    /// Get if digits are required before the decimal point.
    fn required_integer_digits(self) -> bool;

    /// Get if digits are required after the decimal point.
    fn required_fraction_digits(self) -> bool;

    /// Get if digits are required after the exponent character.
    fn required_exponent_digits(self) -> bool;

    /// Get if digits are required before or after the decimal point.
    fn required_digits(self) -> bool;

    /// Get if positive sign before the mantissa is not allowed.
    fn no_positive_mantissa_sign(self) -> bool;

    /// Get if positive sign before the mantissa is required.
    fn required_mantissa_sign(self) -> bool;

    /// Get if exponent notation is not allowed.
    fn no_exponent_notation(self) -> bool;

    /// Get if positive sign before the exponent is not allowed.
    fn no_positive_exponent_sign(self) -> bool;

    /// Get if sign before the exponent is required.
    fn required_exponent_sign(self) -> bool;

    /// Get if exponent without fraction is not allowed.
    fn no_exponent_without_fraction(self) -> bool;

    /// Get if special (non-finite) values are not allowed.
    fn no_special(self) -> bool;

    /// Get if special (non-finite) values are case-sensitive.
    fn case_sensitive_special(self) -> bool;

    /// Get if leading zeros before an integer are not allowed.
    fn no_integer_leading_zeros(self) -> bool;

    /// Get if leading zeros before a float are not allowed.
    fn no_float_leading_zeros(self) -> bool;

    /// Get if digit separators are allowed between integer digits.
    fn integer_internal_digit_separator(self) -> bool;

    /// Get if digit separators are allowed between fraction digits.
    fn fraction_internal_digit_separator(self) -> bool;

    /// Get if digit separators are allowed between exponent digits.
    fn exponent_internal_digit_separator(self) -> bool;

    /// Get if digit separators are allowed between digits.
    fn internal_digit_separator(self) -> bool;

    /// Get if a digit separator is allowed before any integer digits.
    fn integer_leading_digit_separator(self) -> bool;

    /// Get if a digit separator is allowed before any fraction digits.
    fn fraction_leading_digit_separator(self) -> bool;

    /// Get if a digit separator is allowed before any exponent digits.
    fn exponent_leading_digit_separator(self) -> bool;

    /// Get if a digit separator is allowed before any digits.
    fn leading_digit_separator(self) -> bool;

    /// Get if a digit separator is allowed after any integer digits.
    fn integer_trailing_digit_separator(self) -> bool;

    /// Get if a digit separator is allowed after any fraction digits.
    fn fraction_trailing_digit_separator(self) -> bool;

    /// Get if a digit separator is allowed after any exponent digits.
    fn exponent_trailing_digit_separator(self) -> bool;

    /// Get if a digit separator is allowed after any digits.
    fn trailing_digit_separator(self) -> bool;

    /// Get if multiple consecutive integer digit separators are allowed.
    fn integer_consecutive_digit_separator(self) -> bool;

    /// Get if multiple consecutive fraction digit separators are allowed.
    fn fraction_consecutive_digit_separator(self) -> bool;

    /// Get if multiple consecutive exponent digit separators are allowed.
    fn exponent_consecutive_digit_separator(self) -> bool;

    /// Get if multiple consecutive digit separators are allowed.
    fn consecutive_digit_separator(self) -> bool;

    /// Get if any digit separators are allowed in special (non-finite) values.
    fn special_digit_separator(self) -> bool;

    /// Get if using the incorrect, but fast conversion routines.
    fn incorrect(self) -> bool;

    /// Get if using the lossy, but moderately fast, conversion routines.
    fn lossy(self) -> bool;

    // BUILDERS
    // These methods are deprecated. Use the Builder API instead.

    /// Compile float format value from specifications.
    ///
    /// * `radix`                                   - Radix for number encoding or decoding.
    /// * `digit_separator`                         - Character to separate digits.
    /// * `decimal_point`                           - Character to designate the decimal point.
    /// * `exponent`                                - Character to designate the exponent.
    /// * `exponent_backup`                         - Backup character to designate the exponent for radix >= 0xE.
    /// * `required_fraction_digits`                - If digits are required after the decimal point.
    /// * `required_exponent_digits`                - If digits are required after the exponent character.
    /// * `no_positive_mantissa_sign`               - If positive sign before the mantissa is not allowed.
    /// * `required_mantissa_sign`                  - If positive sign before the mantissa is required.
    /// * `no_exponent_notation`                    - If exponent notation is not allowed.
    /// * `no_positive_exponent_sign`               - If positive sign before the exponent is not allowed.
    /// * `required_exponent_sign`                  - If sign before the exponent is required.
    /// * `no_exponent_without_fraction`            - If exponent without fraction is not allowed.
    /// * `no_special`                              - If special (non-finite) values are not allowed.
    /// * `case_sensitive_special`                  - If special (non-finite) values are case-sensitive.
    /// * `no_integer_leading_zeros`                - If leading zeros before an integer are not allowed.
    /// * `no_float_leading_zeros`                  - If leading zeros before a float are not allowed.
    /// * `integer_internal_digit_separator`        - If digit separators are allowed between integer digits.
    /// * `fraction_internal_digit_separator`       - If digit separators are allowed between fraction digits.
    /// * `exponent_internal_digit_separator`       - If digit separators are allowed between exponent digits.
    /// * `integer_leading_digit_separator`         - If a digit separator is allowed before any integer digits.
    /// * `fraction_leading_digit_separator`        - If a digit separator is allowed before any fraction digits.
    /// * `exponent_leading_digit_separator`        - If a digit separator is allowed before any exponent digits.
    /// * `integer_trailing_digit_separator`        - If a digit separator is allowed after any integer digits.
    /// * `fraction_trailing_digit_separator`       - If a digit separator is allowed after any fraction digits.
    /// * `exponent_trailing_digit_separator`       - If a digit separator is allowed after any exponent digits.
    /// * `integer_consecutive_digit_separator`     - If multiple consecutive integer digit separators are allowed.
    /// * `fraction_consecutive_digit_separator`    - If multiple consecutive fraction digit separators are allowed.
    /// * `special_digit_separator`                 - If any digit separators are allowed in special (non-finite) values.
    /// * `incorrect`                               - Use incorrect, but fast conversion routines.
    /// * `lossy`                                   - Use lossy, but moderately fast, conversion routines.
    ///
    /// Returns the value if it was able to compile the format,
    /// otherwise, returns None.
    #[cfg_attr(feature = "radix", doc = " Digit separators must not be in the character group `[A-Za-z0-9+-]`, nor be equal to")]
    #[cfg_attr(feature = "radix", doc = " `decimal_point`, `exponent`, or `exponent_backup`.")]
    #[cfg_attr(not(feature = "radix"), doc = " Digit separators must not be in the character group `[0-9+-]`, nor be equal to")]
    #[cfg_attr(not(feature = "radix"), doc = " `decimal_point` or `exponent`.")]
    ///
    /// # Versioning
    ///
    /// Due to the potential addition of bitflags required to parse a given
    /// number, this function is not considered stable and will not
    /// be stabilized. Any changes will ensure they introduce compile
    /// errors in existing code, and will not require increments
    /// in the current major/minor version.
    #[deprecated(
        since = "0.7.7",
        note = "Will be removed with 1.0. Use the Builder API instead."
    )]
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
    ) -> Option<Self>;

    /// Compile permissive number format.
    ///
    /// The permissive number format does not require any control
    /// grammar, besides the presence of mantissa digits.
    ///
    /// This function cannot fail, but returns an option for consistency
    /// with other grammar compilers.
    fn permissive() -> Option<Self>;

    /// Compile standard number format.
    ///
    /// The standard number format is guaranteed to be identical
    /// to the format expected by Rust's string to number parsers.
    ///
    /// This function cannot fail, but returns an option for consistency
    /// with other grammar compilers.
    fn standard() -> Option<Self>;

    /// Compile ignore number format.
    ///
    /// The ignore number format ignores all digit separators,
    /// and is permissive for all other control grammar, so
    /// implements a fast parser.
    ///
    /// * `digit_separator`                         - Character to separate digits.
    ///
    /// Returns the value if it was able to compile the format,
    /// otherwise, returns None.
    fn ignore(digit_separator: u8) -> Option<Self>;

    /// Create float format directly from digit separator for unittests.
    #[cfg(test)]
    fn from_separator(digit_separator: u8) -> Self;
}
