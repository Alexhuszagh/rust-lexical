//! Configuration for the syntax and valid characters of a number.

use super::lexer::*;
use super::syntax::*;

// TODO(ahuszagh) Need to have the builder here.
// The builders above shouldn't be used, for obvious reasons.

// NUMBER FORMAT
// -------------

// TODO(ahuszagh) Rename to NumberFormat
/// Specification of a numerical format.
///
/// This format comprises two discrete parts: those that affect lexing
/// (IE, which digits are valid, what's the decimal point character)
/// and those that affect syntax validation (IE, are leading zeros
/// in integers allowed).
///
/// This struct aims to be fast for copying, so it aims to contain
/// 2x 64-bit structs, aligned as a 128-bit type.
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NumberFormatV2 {
    pub(super) syntax: SyntaxFormat,
    pub(super) lexer: LexerFormat,
}

impl NumberFormatV2 {
    /// Get the syntax format.
    pub const fn syntax(&self) -> SyntaxFormat {
        self.syntax
    }

    /// Get the lexer format.
    pub const fn lexer(&self) -> LexerFormat {
        self.lexer
    }

    // FLAGS

    /// Get the flag bits from the compiled float format.
    #[inline(always)]
    pub const fn flags(&self) -> SyntaxFormat {
        self.syntax.flags()
    }

    /// Get the interface flag bits from the compiled float format.
    #[inline(always)]
    pub const fn interface_flags(&self) -> SyntaxFormat {
        self.syntax.interface_flags()
    }

    // CHARACTERS

    /// Get the digit separator for the number format.
    #[inline(always)]
    pub const fn digit_separator(&self) -> u8 {
        self.syntax.digit_separator()
    }

    /// Get the exponent character for the lexer format.
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        self.lexer.exponent()
    }

    /// Get the decimal point character for the lexer format.
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        self.lexer.decimal_point()
    }

    /// Get the character for the base prefix.
    ///
    /// If not provided, base prefixes are not allowed.
    /// The number will have then have the format `0$base_prefix...`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    #[inline(always)]
    pub const fn base_prefix(&self) -> OptionU8 {
        self.lexer.base_prefix()
    }

    /// Character for the base suffix.
    ///
    /// If not provided, base suffixes are not allowed.
    /// The number will have then have the format `...$base_suffix`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    #[inline(always)]
    pub const fn base_suffix(&self) -> OptionU8 {
        self.lexer.base_suffix()
    }

    // RADIX

    /// Get the radix for the mantissa digits for the lexer format.
    #[inline(always)]
    pub const fn mantissa_radix(&self) -> u8 {
        self.lexer.mantissa_radix()
    }

    /// Get the base for the exponent for the lexer format.
    ///
    /// IE, a base of 2 means we have `mantissa * 2^exponent`.
    /// If not provided, it defaults to `mantissa_radix`.
    #[inline(always)]
    pub const fn exponent_base(&self) -> OptionU8 {
        self.lexer.exponent_base()
    }

    /// Get the radix for the exponent digits.
    ///
    /// If not provided, defaults to `mantissa_radix`.
    #[inline(always)]
    pub const fn exponent_radix(&self) -> OptionU8 {
        self.lexer.exponent_radix()
    }

    // NON-DIGIT SEPARATOR FLAGS & MASKS

    /// Get if digits are required before the decimal point.
    #[inline(always)]
    pub const fn required_integer_digits(&self) -> bool {
        self.syntax.required_integer_digits()
    }

    /// Get if digits are required after the decimal point.
    #[inline(always)]
    pub const fn required_fraction_digits(&self) -> bool {
        self.syntax.required_fraction_digits()
    }

    /// Get if digits are required after the exponent character.
    #[inline(always)]
    pub const fn required_exponent_digits(&self) -> bool {
        self.syntax.required_exponent_digits()
    }

    /// Get if digits are required before or after the decimal point.
    #[inline(always)]
    pub const fn required_digits(&self) -> bool {
        self.syntax.required_digits()
    }

    /// Get if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn no_positive_mantissa_sign(&self) -> bool {
        self.syntax.no_positive_mantissa_sign()
    }

    /// Get if a sign symbol before the mantissa is required.
    #[inline(always)]
    pub const fn required_mantissa_sign(&self) -> bool {
        self.syntax.required_mantissa_sign()
    }

    /// Get if exponent notation is not allowed.
    #[inline(always)]
    pub const fn no_exponent_notation(&self) -> bool {
        self.syntax.no_exponent_notation()
    }

    /// Get if a positive sign before the exponent is not allowed.
    #[inline(always)]
    pub const fn no_positive_exponent_sign(&self) -> bool {
        self.syntax.no_positive_exponent_sign()
    }

    /// Get if a sign symbol before the exponent is required.
    #[inline(always)]
    pub const fn required_exponent_sign(&self) -> bool {
        self.syntax.required_exponent_sign()
    }

    /// Get if an exponent without fraction is not allowed.
    #[inline(always)]
    pub const fn no_exponent_without_fraction(&self) -> bool {
        self.syntax.no_exponent_without_fraction()
    }

    /// Get if special (non-finite) values are not allowed.
    #[inline(always)]
    pub const fn no_special(&self) -> bool {
        self.syntax.no_special()
    }

    /// Get if special (non-finite) values are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_special(&self) -> bool {
        self.syntax.case_sensitive_special()
    }

    /// Get if leading zeros before an integer are not allowed.
    #[inline(always)]
    pub const fn no_integer_leading_zeros(&self) -> bool {
        self.syntax.no_integer_leading_zeros()
    }

    /// Get if leading zeros before a float are not allowed.
    #[inline(always)]
    pub const fn no_float_leading_zeros(&self) -> bool {
        self.syntax.no_float_leading_zeros()
    }

    /// Get if exponent notation is required.
    #[inline(always)]
    pub const fn required_exponent_notation(&self) -> bool {
        self.syntax.required_exponent_notation()
    }

    /// Get if exponent characters are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_exponent(&self) -> bool {
        self.syntax.case_sensitive_exponent()
    }

    /// Get if base prefixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_prefix(&self) -> bool {
        self.syntax.case_sensitive_base_prefix()
    }

    /// Get if base suffixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_suffix(&self) -> bool {
        self.syntax.case_sensitive_base_suffix()
    }

    // DIGIT SEPARATOR FLAGS & MASKS

    /// Get if digit separators are allowed between integer digits.
    #[inline(always)]
    pub const fn integer_internal_digit_separator(&self) -> bool {
        self.syntax.integer_internal_digit_separator()
    }

    /// Get if digit separators are allowed between fraction digits.
    #[inline(always)]
    pub const fn fraction_internal_digit_separator(&self) -> bool {
        self.syntax.fraction_internal_digit_separator()
    }

    /// Get if digit separators are allowed between exponent digits.
    #[inline(always)]
    pub const fn exponent_internal_digit_separator(&self) -> bool {
        self.syntax.exponent_internal_digit_separator()
    }

    /// Get if digit separators are allowed between digits.
    #[inline(always)]
    pub const fn internal_digit_separator(&self) -> bool {
        self.syntax.internal_digit_separator()
    }

    /// Get if a digit separator is allowed before any integer digits.
    #[inline(always)]
    pub const fn integer_leading_digit_separator(&self) -> bool {
        self.syntax.integer_leading_digit_separator()
    }

    /// Get if a digit separator is allowed before any fraction digits.
    #[inline(always)]
    pub const fn fraction_leading_digit_separator(&self) -> bool {
        self.syntax.fraction_leading_digit_separator()
    }

    /// Get if a digit separator is allowed before any exponent digits.
    #[inline(always)]
    pub const fn exponent_leading_digit_separator(&self) -> bool {
        self.syntax.exponent_leading_digit_separator()
    }

    /// Get if a digit separator is allowed before any digits.
    #[inline(always)]
    pub const fn leading_digit_separator(&self) -> bool {
        self.syntax.leading_digit_separator()
    }

    /// Get if a digit separator is allowed after any integer digits.
    #[inline(always)]
    pub const fn integer_trailing_digit_separator(&self) -> bool {
        self.syntax.integer_trailing_digit_separator()
    }

    /// Get if a digit separator is allowed after any fraction digits.
    #[inline(always)]
    pub const fn fraction_trailing_digit_separator(&self) -> bool {
        self.syntax.fraction_trailing_digit_separator()
    }

    /// Get if a digit separator is allowed after any exponent digits.
    #[inline(always)]
    pub const fn exponent_trailing_digit_separator(&self) -> bool {
        self.syntax.exponent_trailing_digit_separator()
    }

    /// Get if a digit separator is allowed after any digits.
    #[inline(always)]
    pub const fn trailing_digit_separator(&self) -> bool {
        self.syntax.trailing_digit_separator()
    }

    /// Get if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    pub const fn integer_consecutive_digit_separator(&self) -> bool {
        self.syntax.integer_consecutive_digit_separator()
    }

    /// Get if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    pub const fn fraction_consecutive_digit_separator(&self) -> bool {
        self.syntax.fraction_consecutive_digit_separator()
    }

    /// Get if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    pub const fn exponent_consecutive_digit_separator(&self) -> bool {
        self.syntax.exponent_consecutive_digit_separator()
    }

    /// Get if multiple consecutive digit separators are allowed.
    #[inline(always)]
    pub const fn consecutive_digit_separator(&self) -> bool {
        self.syntax.consecutive_digit_separator()
    }

    /// Get if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    pub const fn special_digit_separator(&self) -> bool {
        self.syntax.special_digit_separator()
    }

    // HIDDEN DEFAULTS

    /// Standard number format.
    #[doc(hidden)]
    pub const STANDARD: Self = Self {
        syntax: SyntaxFormat::STANDARD,
        lexer: LexerFormat::STANDARD,
    };

    // BUILDERS

    // BUILDERS

    /// Create new builder to instantiate `NumberFormat`.
    #[inline(always)]
    pub const fn builder() -> NumberFormatBuilderV2 {
        NumberFormatBuilderV2::new()
    }

    /// Recreate `NumberFormatBuilder` using current `NumberFormat` values.
    #[inline(always)]
    pub const fn rebuild(&self) -> NumberFormatBuilderV2 {
        NumberFormatBuilderV2 {
            digit_separator: self.digit_separator(),
            exponent: self.exponent(),
            decimal_point: self.decimal_point(),
            mantissa_radix: self.mantissa_radix(),
            exponent_base: self.exponent_base(),
            exponent_radix: self.exponent_radix(),
            base_prefix: self.base_prefix(),
            base_suffix: self.base_suffix(),
            required_integer_digits: self.required_integer_digits(),
            required_fraction_digits: self.required_fraction_digits(),
            required_exponent_digits: self.required_exponent_digits(),
            no_positive_mantissa_sign: self.no_positive_mantissa_sign(),
            required_mantissa_sign: self.required_mantissa_sign(),
            no_exponent_notation: self.no_exponent_notation(),
            no_positive_exponent_sign: self.no_positive_exponent_sign(),
            required_exponent_sign: self.required_exponent_sign(),
            no_exponent_without_fraction: self.no_exponent_without_fraction(),
            no_special: self.no_special(),
            case_sensitive_special: self.case_sensitive_special(),
            no_integer_leading_zeros: self.no_integer_leading_zeros(),
            no_float_leading_zeros: self.no_float_leading_zeros(),
            required_exponent_notation: self.required_exponent_notation(),
            case_sensitive_exponent: self.case_sensitive_exponent(),
            case_sensitive_base_prefix: self.case_sensitive_base_prefix(),
            case_sensitive_base_suffix: self.case_sensitive_base_suffix(),
            integer_internal_digit_separator: self.integer_internal_digit_separator(),
            fraction_internal_digit_separator: self.fraction_internal_digit_separator(),
            exponent_internal_digit_separator: self.exponent_internal_digit_separator(),
            integer_leading_digit_separator: self.integer_leading_digit_separator(),
            fraction_leading_digit_separator: self.fraction_leading_digit_separator(),
            exponent_leading_digit_separator: self.exponent_leading_digit_separator(),
            integer_trailing_digit_separator: self.integer_trailing_digit_separator(),
            fraction_trailing_digit_separator: self.fraction_trailing_digit_separator(),
            exponent_trailing_digit_separator: self.exponent_trailing_digit_separator(),
            integer_consecutive_digit_separator: self.integer_consecutive_digit_separator(),
            fraction_consecutive_digit_separator: self.fraction_consecutive_digit_separator(),
            exponent_consecutive_digit_separator: self.exponent_consecutive_digit_separator(),
            special_digit_separator: self.special_digit_separator(),
        }
    }
}

// NUMBER FORMAT BUILDER
// ---------------------

// TODO(ahuszagh) Rename to NumberFormatBuilder
/// Build float format value from specifications.
///
/// * `digit_separator`                         - Character to separate digits.
/// * `exponent`                                - Character to designate exponent notation.
/// * `decimal_point`                           - Character to designate the decimal point.
/// * `mantissa_radix`                          - Radix for mantissa digits.
/// * `exponent_base`                           - Base for the exponent.
/// * `exponent_radix`                          - Radix for the exponent digits.
/// * `base_prefix`                             - Optional character for the base prefix.
/// * `base_suffix`                             - Optional character for the base suffix.
/// * `required_integer_digits`                 - If digits are required before the decimal point.
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
/// * `required_exponent_notation`              - If exponent notation is required.
/// * `case_sensitive_exponent`                 - If exponent characters are case-sensitive.
/// * `case_sensitive_base_prefix`              - If base prefixes are case-sensitive.
/// * `case_sensitive_base_suffix`              - If base suffixes are case-sensitive.
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
///
/// Returns the format on calling build if it was able to compile the format,
/// otherwise, returns None.
pub struct NumberFormatBuilderV2 {
    digit_separator: u8,
    exponent: u8,
    decimal_point: u8,
    mantissa_radix: u8,
    exponent_base: OptionU8,
    exponent_radix: OptionU8,
    base_prefix: OptionU8,
    base_suffix: OptionU8,
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
    required_exponent_notation: bool,
    case_sensitive_exponent: bool,
    case_sensitive_base_prefix: bool,
    case_sensitive_base_suffix: bool,
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
}

// TODO(ahuszagh) Rename to NumberFormatBuilder
impl NumberFormatBuilderV2 {
    /// Create new NumberFormatBuilder with default arguments.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            digit_separator: b'\x00',
            exponent: b'e',
            decimal_point: b'.',
            mantissa_radix: 10,
            exponent_base: None,
            exponent_radix: None,
            base_prefix: None,
            base_suffix: None,
            required_integer_digits: false,
            required_fraction_digits: false,
            required_exponent_digits: false,
            no_positive_mantissa_sign: false,
            required_mantissa_sign: false,
            no_exponent_notation: false,
            no_positive_exponent_sign: false,
            required_exponent_sign: false,
            no_exponent_without_fraction: false,
            no_special: false,
            case_sensitive_special: false,
            no_integer_leading_zeros: false,
            no_float_leading_zeros: false,
            required_exponent_notation: false,
            case_sensitive_exponent: false,
            case_sensitive_base_prefix: false,
            case_sensitive_base_suffix: false,
            integer_internal_digit_separator: false,
            fraction_internal_digit_separator: false,
            exponent_internal_digit_separator: false,
            integer_leading_digit_separator: false,
            fraction_leading_digit_separator: false,
            exponent_leading_digit_separator: false,
            integer_trailing_digit_separator: false,
            fraction_trailing_digit_separator: false,
            exponent_trailing_digit_separator: false,
            integer_consecutive_digit_separator: false,
            fraction_consecutive_digit_separator: false,
            exponent_consecutive_digit_separator: false,
            special_digit_separator: false,
        }
    }

    // GETTERS

    /// Get the digit separator for the number format.
    #[inline(always)]
    pub const fn get_digit_separator(&self) -> u8 {
        self.digit_separator
    }

    /// Get the exponent character for the number format.
    #[inline(always)]
    pub const fn get_exponent(&self) -> u8 {
        self.exponent
    }

    /// Get the decimal point character for the number format.
    #[inline(always)]
    pub const fn get_decimal_point(&self) -> u8 {
        self.decimal_point
    }

    /// Get the radix for mantissa digits.
    #[inline(always)]
    pub const fn get_mantissa_radix(&self) -> u8 {
        self.mantissa_radix
    }

    /// Get the radix for the exponent.
    #[inline(always)]
    pub const fn get_exponent_base(&self) -> OptionU8 {
        self.exponent_base
    }

    /// Get the radix for exponent digits.
    #[inline(always)]
    pub const fn get_exponent_radix(&self) -> OptionU8 {
        self.exponent_radix
    }

    /// Get the optional character for the base prefix.
    #[inline(always)]
    pub const fn get_base_prefix(&self) -> OptionU8 {
        self.base_prefix
    }

    /// Get the optional character for the base suffix.
    #[inline(always)]
    pub const fn get_base_suffix(&self) -> OptionU8 {
        self.base_suffix
    }

    /// Get if digits are required before the decimal point.
    #[inline(always)]
    pub const fn get_required_integer_digits(&self) -> bool {
        self.required_integer_digits
    }

    /// Get if digits are required after the decimal point.
    #[inline(always)]
    pub const fn get_required_fraction_digits(&self) -> bool {
        self.required_fraction_digits
    }

    /// Get if digits are required after the exponent character.
    #[inline(always)]
    pub const fn get_required_exponent_digits(&self) -> bool {
        self.required_exponent_digits
    }

    /// Get if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn get_no_positive_mantissa_sign(&self) -> bool {
        self.no_positive_mantissa_sign
    }

    /// Get if a sign symbol before the mantissa is required.
    #[inline(always)]
    pub const fn get_required_mantissa_sign(&self) -> bool {
        self.required_mantissa_sign
    }

    /// Get if exponent notation is not allowed.
    #[inline(always)]
    pub const fn get_no_exponent_notation(&self) -> bool {
        self.no_exponent_notation
    }

    /// Get if a positive sign before the exponent is not allowed.
    #[inline(always)]
    pub const fn get_no_positive_exponent_sign(&self) -> bool {
        self.no_positive_exponent_sign
    }

    /// Get if a sign symbol before the exponent is required.
    #[inline(always)]
    pub const fn get_required_exponent_sign(&self) -> bool {
        self.required_exponent_sign
    }

    /// Get if an exponent without fraction is not allowed.
    #[inline(always)]
    pub const fn get_no_exponent_without_fraction(&self) -> bool {
        self.no_exponent_without_fraction
    }

    /// Get if special (non-finite) values are not allowed.
    #[inline(always)]
    pub const fn get_no_special(&self) -> bool {
        self.no_special
    }

    /// Get if special (non-finite) values are case-sensitive.
    #[inline(always)]
    pub const fn get_case_sensitive_special(&self) -> bool {
        self.case_sensitive_special
    }

    /// Get if leading zeros before an integer are not allowed.
    #[inline(always)]
    pub const fn get_no_integer_leading_zeros(&self) -> bool {
        self.no_integer_leading_zeros
    }

    /// Get if leading zeros before a float are not allowed.
    #[inline(always)]
    pub const fn get_no_float_leading_zeros(&self) -> bool {
        self.no_float_leading_zeros
    }

    /// Get if exponent notation is required.
    #[inline(always)]
    pub const fn get_required_exponent_notation(&self) -> bool {
        self.required_exponent_notation
    }

    /// Get if exponent characters are case-sensitive.
    #[inline(always)]
    pub const fn get_case_sensitive_exponent(&self) -> bool {
        self.case_sensitive_exponent
    }

    /// Get if base prefixes are case-sensitive.
    #[inline(always)]
    pub const fn get_case_sensitive_base_prefix(&self) -> bool {
        self.case_sensitive_base_prefix
    }

    /// Get if base suffixes are case-sensitive.
    #[inline(always)]
    pub const fn get_case_sensitive_base_suffix(&self) -> bool {
        self.case_sensitive_base_suffix
    }

    /// Get if digit separators are allowed between integer digits.
    #[inline(always)]
    pub const fn get_integer_internal_digit_separator(&self) -> bool {
        self.integer_internal_digit_separator
    }

    /// Get if digit separators are allowed between fraction digits.
    #[inline(always)]
    pub const fn get_fraction_internal_digit_separator(&self) -> bool {
        self.fraction_internal_digit_separator
    }

    /// Get if digit separators are allowed between exponent digits.
    #[inline(always)]
    pub const fn get_exponent_internal_digit_separator(&self) -> bool {
        self.exponent_internal_digit_separator
    }

    /// Get if a digit separator is allowed before any integer digits.
    #[inline(always)]
    pub const fn get_integer_leading_digit_separator(&self) -> bool {
        self.integer_leading_digit_separator
    }

    /// Get if a digit separator is allowed before any fraction digits.
    #[inline(always)]
    pub const fn get_fraction_leading_digit_separator(&self) -> bool {
        self.fraction_leading_digit_separator
    }

    /// Get if a digit separator is allowed before any exponent digits.
    #[inline(always)]
    pub const fn get_exponent_leading_digit_separator(&self) -> bool {
        self.exponent_leading_digit_separator
    }

    /// Get if a digit separator is allowed after any integer digits.
    #[inline(always)]
    pub const fn get_integer_trailing_digit_separator(&self) -> bool {
        self.integer_trailing_digit_separator
    }

    /// Get if a digit separator is allowed after any fraction digits.
    #[inline(always)]
    pub const fn get_fraction_trailing_digit_separator(&self) -> bool {
        self.fraction_trailing_digit_separator
    }

    /// Get if a digit separator is allowed after any exponent digits.
    #[inline(always)]
    pub const fn get_exponent_trailing_digit_separator(&self) -> bool {
        self.exponent_trailing_digit_separator
    }

    /// Get if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    pub const fn get_integer_consecutive_digit_separator(&self) -> bool {
        self.integer_consecutive_digit_separator
    }

    /// Get if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    pub const fn get_fraction_consecutive_digit_separator(&self) -> bool {
        self.fraction_consecutive_digit_separator
    }

    /// Get if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    pub const fn get_exponent_consecutive_digit_separator(&self) -> bool {
        self.exponent_consecutive_digit_separator
    }

    /// Get if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    pub const fn get_special_digit_separator(&self) -> bool {
        self.special_digit_separator
    }

    // SETTERS
    // TODO(ahuszagh) Feature-gate!!!

    /// Set the digit separator for the number format.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn digit_separator(mut self, character: u8) -> Self {
        self.digit_separator = character;
        self
    }

    /// Set the exponent character for the number format.
    #[inline(always)]
    pub const fn exponent(mut self, character: u8) -> Self {
        self.exponent = character;
        self
    }

    /// Set the decimal point character for the number format.
    #[inline(always)]
    pub const fn decimal_point(mut self, character: u8) -> Self {
        self.decimal_point = character;
        self
    }

    /// Set the radix for mantissa digits.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn mantissa_radix(mut self, radix: u8) -> Self {
        self.mantissa_radix = radix;
        self
    }

    /// Set the radix for the exponent.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn exponent_base(mut self, base: OptionU8) -> Self {
        self.exponent_base = base;
        self
    }

    /// Set the radix for exponent digits.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn exponent_radix(mut self, radix: OptionU8) -> Self {
        self.exponent_radix = radix;
        self
    }

    /// Set the optional character for the base prefix.
    #[inline(always)]
    #[cfg(all(feature = "power_of_two", feature = "format"))]
    pub const fn base_prefix(mut self, base_prefix: OptionU8) -> Self {
        self.base_prefix = base_prefix;
        self
    }

    /// Set the optional character for the base suffix.
    #[inline(always)]
    #[cfg(all(feature = "power_of_two", feature = "format"))]
    pub const fn base_suffix(mut self, base_suffix: OptionU8) -> Self {
        self.base_suffix = base_suffix;
        self
    }

    /// Set if digits are required before the decimal point.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_integer_digits(mut self, flag: bool) -> Self {
        self.required_integer_digits = flag;
        self
    }

    /// Set if digits are required after the decimal point.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_fraction_digits(mut self, flag: bool) -> Self {
        self.required_fraction_digits = flag;
        self
    }

    /// Set if digits are required after the exponent character.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_exponent_digits(mut self, flag: bool) -> Self {
        self.required_exponent_digits = flag;
        self
    }

    /// Set if digits are required for all float components.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_digits(mut self, flag: bool) -> Self {
        self = self.required_integer_digits(flag);
        self = self.required_fraction_digits(flag);
        self = self.required_exponent_digits(flag);
        self
    }

    /// Set if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn no_positive_mantissa_sign(mut self, flag: bool) -> Self {
        self.no_positive_mantissa_sign = flag;
        self
    }

    /// Set if a sign symbol before the mantissa is required.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_mantissa_sign(mut self, flag: bool) -> Self {
        self.required_mantissa_sign = flag;
        self
    }

    /// Set if exponent notation is not allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_exponent_notation(mut self, flag: bool) -> Self {
        self.no_exponent_notation = flag;
        self
    }

    /// Set if a positive sign before the exponent is not allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_positive_exponent_sign(mut self, flag: bool) -> Self {
        self.no_positive_exponent_sign = flag;
        self
    }

    /// Set if a sign symbol before the exponent is required.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_exponent_sign(mut self, flag: bool) -> Self {
        self.required_exponent_sign = flag;
        self
    }

    /// Set if an exponent without fraction is not allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_exponent_without_fraction(mut self, flag: bool) -> Self {
        self.no_exponent_without_fraction = flag;
        self
    }

    /// Set if special (non-finite) values are not allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_special(mut self, flag: bool) -> Self {
        self.no_special = flag;
        self
    }

    /// Set if special (non-finite) values are case-sensitive.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn case_sensitive_special(mut self, flag: bool) -> Self {
        self.case_sensitive_special = flag;
        self
    }

    /// Set if leading zeros before an integer are not allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_integer_leading_zeros(mut self, flag: bool) -> Self {
        self.no_integer_leading_zeros = flag;
        self
    }

    /// Set if leading zeros before a float are not allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn no_float_leading_zeros(mut self, flag: bool) -> Self {
        self.no_float_leading_zeros = flag;
        self
    }

    /// Set if exponent notation is required.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_exponent_notation(mut self, flag: bool) -> Self {
        self.required_exponent_notation = flag;
        self
    }

    /// Set if exponent characters are case-sensitive.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn case_sensitive_exponent(mut self, flag: bool) -> Self {
        self.case_sensitive_exponent = flag;
        self
    }

    /// Set if base prefixes are case-sensitive.
    #[inline(always)]
    #[cfg(all(feature = "power_of_two", feature = "format"))]
    pub const fn case_sensitive_base_prefix(mut self, flag: bool) -> Self {
        self.case_sensitive_base_prefix = flag;
        self
    }

    /// Set if base suffixes are case-sensitive.
    #[inline(always)]
    #[cfg(all(feature = "power_of_two", feature = "format"))]
    pub const fn case_sensitive_base_suffix(mut self, flag: bool) -> Self {
        self.case_sensitive_base_suffix = flag;
        self
    }

    /// Set if digit separators are allowed between integer digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_internal_digit_separator(mut self, flag: bool) -> Self {
        self.integer_internal_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed between fraction digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_internal_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_internal_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed between exponent digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_internal_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_internal_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before any integer digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_leading_digit_separator(mut self, flag: bool) -> Self {
        self.integer_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before any fraction digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_leading_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before any exponent digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_leading_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after any integer digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.integer_trailing_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after any fraction digits.
    #[inline(always)]
    pub const fn fraction_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_trailing_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after any exponent digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_trailing_digit_separator = flag;
        self
    }

    /// Set if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_consecutive_digit_separator(mut self, flag: bool) -> Self {
        self.integer_consecutive_digit_separator = flag;
        self
    }

    /// Set if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_consecutive_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_consecutive_digit_separator = flag;
        self
    }

    /// Set if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_consecutive_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_consecutive_digit_separator = flag;
        self
    }

    /// Set if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn special_digit_separator(mut self, flag: bool) -> Self {
        self.special_digit_separator = flag;
        self
    }

    /// Set all integer digit separator flag masks.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn digit_separator_flag_mask(mut self, flag: bool) -> Self {
        self = self.integer_digit_separator_flag_mask(flag);
        self = self.fraction_digit_separator_flag_mask(flag);
        self = self.exponent_digit_separator_flag_mask(flag);
        self = self.special_digit_separator(flag);
        self
    }

    /// Set all integer digit separator flag masks.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_digit_separator_flag_mask(mut self, flag: bool) -> Self {
        self = self.integer_internal_digit_separator(flag);
        self = self.integer_leading_digit_separator(flag);
        self = self.integer_trailing_digit_separator(flag);
        self = self.integer_consecutive_digit_separator(flag);
        self
    }

    /// Set all fraction digit separator flag masks.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_digit_separator_flag_mask(mut self, flag: bool) -> Self {
        self = self.fraction_internal_digit_separator(flag);
        self = self.fraction_leading_digit_separator(flag);
        self = self.fraction_trailing_digit_separator(flag);
        self = self.fraction_consecutive_digit_separator(flag);
        self
    }

    /// Set all exponent digit separator flag masks.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_digit_separator_flag_mask(mut self, flag: bool) -> Self {
        self = self.exponent_internal_digit_separator(flag);
        self = self.exponent_leading_digit_separator(flag);
        self = self.exponent_trailing_digit_separator(flag);
        self = self.exponent_consecutive_digit_separator(flag);
        self
    }

    // BUILDER

    // TODO(ahuszagh) Here...
}
