//! Builder for the number format.

use core::{mem, num};

use static_assertions::const_assert;

use crate::format_flags as flags;

/// Type with the exact same size as a `u8`.
pub type OptionU8 = Option<num::NonZeroU8>;

// Ensure the sizes are identical.
const_assert!(mem::size_of::<OptionU8>() == mem::size_of::<u8>());

/// Add single flag to `SyntaxFormat`.
macro_rules! add_flag {
    ($format:ident, $bool:expr, $flag:ident) => {
        if $bool {
            $format |= flags::$flag;
        }
    };
}

/// Add multiple flags to `SyntaxFormat`.
macro_rules! add_flags {
    ($format:ident ; $($bool:expr, $flag:ident ;)*) => {{
        $(add_flag!($format, $bool, $flag);)*
    }};
}

/// Determine if a flag is set in the format.
macro_rules! has_flag {
    ($format:ident, $flag:ident) => {
        $format & flags::$flag != 0
    };
}

/// Unwrap `Option` as a const fn.
#[inline(always)]
const fn unwrap_or_zero(option: OptionU8) -> u8 {
    match option {
        Some(x) => x.get(),
        None => 0,
    }
}

/// Build number format from specifications.
///
/// Returns the format on calling build if it was able to compile the format,
/// otherwise, returns None.
///
/// # Fields
///
/// * `digit_separator`                         - Character to separate digits.
/// * `mantissa_radix`                          - Radix for mantissa digits.
/// * `exponent_base`                           - Base for the exponent.
/// * `exponent_radix`                          - Radix for the exponent digits.
/// * `base_prefix`                             - Optional character for the
///   base prefix.
/// * `base_suffix`                             - Optional character for the
///   base suffix.
/// * `required_integer_digits`                 - If digits are required before
///   the decimal point.
/// * `required_fraction_digits`                - If digits are required after
///   the decimal point.
/// * `required_exponent_digits`                - If digits are required after
///   the exponent character.
/// * `required_mantissa_digits`                - If at least 1 significant
///   digit is required.
/// * `no_positive_mantissa_sign`               - If positive sign before the
///   mantissa is not allowed.
/// * `required_mantissa_sign`                  - If positive sign before the
///   mantissa is required.
/// * `no_exponent_notation`                    - If exponent notation is not
///   allowed.
/// * `no_positive_exponent_sign`               - If positive sign before the
///   exponent is not allowed.
/// * `required_exponent_sign`                  - If sign before the exponent is
///   required.
/// * `no_exponent_without_fraction`            - If exponent without fraction
///   is not allowed.
/// * `no_special`                              - If special (non-finite) values
///   are not allowed.
/// * `case_sensitive_special`                  - If special (non-finite) values
///   are case-sensitive.
/// * `no_integer_leading_zeros`                - If leading zeros before an
///   integer are not allowed.
/// * `no_float_leading_zeros`                  - If leading zeros before a
///   float are not allowed.
/// * `required_exponent_notation`              - If exponent notation is
///   required.
/// * `case_sensitive_exponent`                 - If exponent characters are
///   case-sensitive.
/// * `case_sensitive_base_prefix`              - If base prefixes are
///   case-sensitive.
/// * `case_sensitive_base_suffix`              - If base suffixes are
///   case-sensitive.
/// * `integer_internal_digit_separator`        - If digit separators are
///   allowed between integer digits.
/// * `fraction_internal_digit_separator`       - If digit separators are
///   allowed between fraction digits.
/// * `exponent_internal_digit_separator`       - If digit separators are
///   allowed between exponent digits.
/// * `integer_leading_digit_separator`         - If a digit separator is
///   allowed before any integer digits.
/// * `fraction_leading_digit_separator`        - If a digit separator is
///   allowed before any fraction digits.
/// * `exponent_leading_digit_separator`        - If a digit separator is
///   allowed before any exponent digits.
/// * `integer_trailing_digit_separator`        - If a digit separator is
///   allowed after any integer digits.
/// * `fraction_trailing_digit_separator`       - If a digit separator is
///   allowed after any fraction digits.
/// * `exponent_trailing_digit_separator`       - If a digit separator is
///   allowed after any exponent digits.
/// * `integer_consecutive_digit_separator`     - If multiple consecutive
///   integer digit separators are allowed.
/// * `fraction_consecutive_digit_separator`    - If multiple consecutive
///   fraction digit separators are allowed.
/// * `special_digit_separator`                 - If any digit separators are
///   allowed in special (non-finite) values.
///
/// # Write Integer Fields
///
/// No fields are used for writing integers.
///
/// # Parse Integer Fields
///
/// These fields are used for parsing integers:
///
/// * `digit_separator`
/// * `mantissa_radix`
/// * `base_prefix`
/// * `base_suffix`
/// * `no_positive_mantissa_sign`
/// * `required_mantissa_sign`
/// * `no_integer_leading_zeros`
/// * `integer_internal_digit_separator`
/// * `integer_leading_digit_separator`
/// * `integer_trailing_digit_separator`
/// * `integer_consecutive_digit_separator`
///
/// # Write Float Fields
///
/// These fields are used for writing floats:
///
/// * `mantissa_radix`
/// * `exponent_base`
/// * `exponent_radix`
/// * `no_positive_mantissa_sign`
/// * `required_mantissa_sign`
/// * `no_exponent_notation`
/// * `no_positive_exponent_sign`
/// * `required_exponent_sign`
/// * `required_exponent_notation`
///
/// # Parse Float Fields
///
/// These fields are used for parsing floats:
///
/// * `digit_separator`
/// * `mantissa_radix`
/// * `exponent_base`
/// * `exponent_radix`
/// * `base_prefix`
/// * `base_suffix`
/// * `required_integer_digits`
/// * `required_fraction_digits`
/// * `required_exponent_digits`
/// * `no_positive_mantissa_sign`
/// * `required_mantissa_sign`
/// * `no_exponent_notation`
/// * `no_positive_exponent_sign`
/// * `required_exponent_sign`
/// * `no_exponent_without_fraction`
/// * `no_special`
/// * `case_sensitive_special`
/// * `no_integer_leading_zeros`
/// * `no_float_leading_zeros`
/// * `required_exponent_notation`
/// * `case_sensitive_exponent`
/// * `case_sensitive_base_prefix`
/// * `case_sensitive_base_suffix`
/// * `integer_internal_digit_separator`
/// * `fraction_internal_digit_separator`
/// * `exponent_internal_digit_separator`
/// * `integer_leading_digit_separator`
/// * `fraction_leading_digit_separator`
/// * `exponent_leading_digit_separator`
/// * `integer_trailing_digit_separator`
/// * `fraction_trailing_digit_separator`
/// * `exponent_trailing_digit_separator`
/// * `integer_consecutive_digit_separator`
/// * `fraction_consecutive_digit_separator`
/// * `special_digit_separator`
pub struct NumberFormatBuilder {
    digit_separator: OptionU8,
    base_prefix: OptionU8,
    base_suffix: OptionU8,
    mantissa_radix: u8,
    exponent_base: OptionU8,
    exponent_radix: OptionU8,
    required_integer_digits: bool,
    required_fraction_digits: bool,
    required_exponent_digits: bool,
    required_mantissa_digits: bool,
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

impl NumberFormatBuilder {
    // CONSTRUCTORS

    /// Create new `NumberFormatBuilder` with default arguments.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            digit_separator: None,
            base_prefix: None,
            base_suffix: None,
            mantissa_radix: 10,
            exponent_base: None,
            exponent_radix: None,
            required_integer_digits: false,
            required_fraction_digits: false,
            required_exponent_digits: true,
            required_mantissa_digits: true,
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

    /// Create number format for standard, binary number.
    #[cfg(feature = "power-of-two")]
    pub const fn binary() -> u128 {
        Self::from_radix(2)
    }

    /// Create number format for standard, octal number.
    #[cfg(feature = "power-of-two")]
    pub const fn octal() -> u128 {
        Self::from_radix(8)
    }

    /// Create number format for standard, decimal number.
    pub const fn decimal() -> u128 {
        let mut builder = Self::new();
        builder.mantissa_radix = 10;
        builder.exponent_base = num::NonZeroU8::new(10);
        builder.exponent_radix = num::NonZeroU8::new(10);
        builder.build()
    }

    /// Create number format for standard, hexadecimal number.
    #[cfg(feature = "power-of-two")]
    pub const fn hexadecimal() -> u128 {
        Self::from_radix(16)
    }

    /// Create number format from radix.
    #[cfg(feature = "power-of-two")]
    pub const fn from_radix(radix: u8) -> u128 {
        Self::new()
            .radix(radix)
            .exponent_base(num::NonZeroU8::new(radix))
            .exponent_radix(num::NonZeroU8::new(radix))
            .build()
    }

    // GETTERS

    /// Get the digit separator for the number format.
    #[inline(always)]
    pub const fn get_digit_separator(&self) -> OptionU8 {
        self.digit_separator
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

    /// Get if at least 1 significant digit is required.
    #[inline(always)]
    pub const fn get_required_mantissa_digits(&self) -> bool {
        self.required_mantissa_digits
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
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    pub const fn get_integer_internal_digit_separator(&self) -> bool {
        self.integer_internal_digit_separator
    }

    /// Get if digit separators are allowed between fraction digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    pub const fn get_fraction_internal_digit_separator(&self) -> bool {
        self.fraction_internal_digit_separator
    }

    /// Get if digit separators are allowed between exponent digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    pub const fn get_exponent_internal_digit_separator(&self) -> bool {
        self.exponent_internal_digit_separator
    }

    /// Get if a digit separator is allowed before any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn get_integer_leading_digit_separator(&self) -> bool {
        self.integer_leading_digit_separator
    }

    /// Get if a digit separator is allowed before any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn get_fraction_leading_digit_separator(&self) -> bool {
        self.fraction_leading_digit_separator
    }

    /// Get if a digit separator is allowed before any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn get_exponent_leading_digit_separator(&self) -> bool {
        self.exponent_leading_digit_separator
    }

    /// Get if a digit separator is allowed after any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn get_integer_trailing_digit_separator(&self) -> bool {
        self.integer_trailing_digit_separator
    }

    /// Get if a digit separator is allowed after any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    pub const fn get_fraction_trailing_digit_separator(&self) -> bool {
        self.fraction_trailing_digit_separator
    }

    /// Get if a digit separator is allowed after any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
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

    /// Set the digit separator for the number format.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn digit_separator(mut self, character: OptionU8) -> Self {
        self.digit_separator = character;
        self
    }

    /// Alias for mantissa radix.
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn radix(self, radix: u8) -> Self {
        self.mantissa_radix(radix)
    }

    /// Set the radix for mantissa digits.
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn mantissa_radix(mut self, radix: u8) -> Self {
        self.mantissa_radix = radix;
        self
    }

    /// Set the radix for the exponent.
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn exponent_base(mut self, base: OptionU8) -> Self {
        self.exponent_base = base;
        self
    }

    /// Set the radix for exponent digits.
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn exponent_radix(mut self, radix: OptionU8) -> Self {
        self.exponent_radix = radix;
        self
    }

    /// Set the optional character for the base prefix.
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn base_prefix(mut self, base_prefix: OptionU8) -> Self {
        self.base_prefix = base_prefix;
        self
    }

    /// Set the optional character for the base suffix.
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
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

    /// Set if at least 1 significant digit is required.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_mantissa_digits(mut self, flag: bool) -> Self {
        self.required_mantissa_digits = flag;
        self
    }

    /// Set if digits are required for all float components.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn required_digits(mut self, flag: bool) -> Self {
        self = self.required_integer_digits(flag);
        self = self.required_fraction_digits(flag);
        self = self.required_exponent_digits(flag);
        self = self.required_mantissa_digits(flag);
        self
    }

    /// Set if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    #[cfg(feature = "format")]
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
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn case_sensitive_base_prefix(mut self, flag: bool) -> Self {
        self.case_sensitive_base_prefix = flag;
        self
    }

    /// Set if base suffixes are case-sensitive.
    #[inline(always)]
    #[cfg(all(feature = "power-of-two", feature = "format"))]
    pub const fn case_sensitive_base_suffix(mut self, flag: bool) -> Self {
        self.case_sensitive_base_suffix = flag;
        self
    }

    /// Set if digit separators are allowed between integer digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_internal_digit_separator(mut self, flag: bool) -> Self {
        self.integer_internal_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed between fraction digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_internal_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_internal_digit_separator = flag;
        self
    }

    /// Set if digit separators are allowed between exponent digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_internal_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_internal_digit_separator = flag;
        self
    }

    /// Set all internal digit separator flags.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn internal_digit_separator(mut self, flag: bool) -> Self {
        self = self.integer_internal_digit_separator(flag);
        self = self.fraction_internal_digit_separator(flag);
        self = self.exponent_internal_digit_separator(flag);
        self
    }

    /// Set if a digit separator is allowed before any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_leading_digit_separator(mut self, flag: bool) -> Self {
        self.integer_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_leading_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_leading_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed before any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_leading_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_leading_digit_separator = flag;
        self
    }

    /// Set all leading digit separator flags.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn leading_digit_separator(mut self, flag: bool) -> Self {
        self = self.integer_leading_digit_separator(flag);
        self = self.fraction_leading_digit_separator(flag);
        self = self.exponent_leading_digit_separator(flag);
        self
    }

    /// Set if a digit separator is allowed after any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.integer_trailing_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.fraction_trailing_digit_separator = flag;
        self
    }

    /// Set if a digit separator is allowed after any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_trailing_digit_separator(mut self, flag: bool) -> Self {
        self.exponent_trailing_digit_separator = flag;
        self
    }

    /// Set all trailing digit separator flags.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn trailing_digit_separator(mut self, flag: bool) -> Self {
        self = self.integer_trailing_digit_separator(flag);
        self = self.fraction_trailing_digit_separator(flag);
        self = self.exponent_trailing_digit_separator(flag);
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

    /// Set all consecutive digit separator flags.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn consecutive_digit_separator(mut self, flag: bool) -> Self {
        self = self.integer_consecutive_digit_separator(flag);
        self = self.fraction_consecutive_digit_separator(flag);
        self = self.exponent_consecutive_digit_separator(flag);
        self
    }

    /// Set if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn special_digit_separator(mut self, flag: bool) -> Self {
        self.special_digit_separator = flag;
        self
    }

    /// Set all digit separator flag masks.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.integer_digit_separator_flags(flag);
        self = self.fraction_digit_separator_flags(flag);
        self = self.exponent_digit_separator_flags(flag);
        self = self.special_digit_separator(flag);
        self
    }

    /// Set all integer digit separator flag masks.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn integer_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.integer_internal_digit_separator(flag);
        self = self.integer_leading_digit_separator(flag);
        self = self.integer_trailing_digit_separator(flag);
        self = self.integer_consecutive_digit_separator(flag);
        self
    }

    /// Set all fraction digit separator flag masks.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn fraction_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.fraction_internal_digit_separator(flag);
        self = self.fraction_leading_digit_separator(flag);
        self = self.fraction_trailing_digit_separator(flag);
        self = self.fraction_consecutive_digit_separator(flag);
        self
    }

    /// Set all exponent digit separator flag masks.
    #[inline(always)]
    #[cfg(feature = "format")]
    pub const fn exponent_digit_separator_flags(mut self, flag: bool) -> Self {
        self = self.exponent_internal_digit_separator(flag);
        self = self.exponent_leading_digit_separator(flag);
        self = self.exponent_trailing_digit_separator(flag);
        self = self.exponent_consecutive_digit_separator(flag);
        self
    }

    // BUILDER

    /// Create 128-bit, packed number format struct from builder options.
    ///
    /// NOTE: This function will never fail, due to issues with panicking
    /// (and therefore unwrapping Errors/Options) in const fns. It is
    /// therefore up to you to ensure the format is valid, called via the
    /// `is_valid` function on `NumberFormat`.
    #[inline(always)]
    pub const fn build(&self) -> u128 {
        let mut format: u128 = 0;
        add_flags!(
            format ;
            self.required_integer_digits, REQUIRED_INTEGER_DIGITS ;
            self.required_fraction_digits, REQUIRED_FRACTION_DIGITS ;
            self.required_exponent_digits, REQUIRED_EXPONENT_DIGITS ;
            self.required_mantissa_digits, REQUIRED_MANTISSA_DIGITS ;
            self.no_positive_mantissa_sign, NO_POSITIVE_MANTISSA_SIGN ;
            self.required_mantissa_sign, REQUIRED_MANTISSA_SIGN ;
            self.no_exponent_notation, NO_EXPONENT_NOTATION ;
            self.no_positive_exponent_sign, NO_POSITIVE_EXPONENT_SIGN ;
            self.required_exponent_sign, REQUIRED_EXPONENT_SIGN ;
            self.no_exponent_without_fraction, NO_EXPONENT_WITHOUT_FRACTION ;
            self.no_special, NO_SPECIAL ;
            self.case_sensitive_special, CASE_SENSITIVE_SPECIAL ;
            self.no_integer_leading_zeros, NO_INTEGER_LEADING_ZEROS ;
            self.no_float_leading_zeros, NO_FLOAT_LEADING_ZEROS ;
            self.required_exponent_notation, REQUIRED_EXPONENT_NOTATION ;
            self.case_sensitive_exponent, CASE_SENSITIVE_EXPONENT ;
            self.case_sensitive_base_prefix, CASE_SENSITIVE_BASE_PREFIX ;
            self.case_sensitive_base_suffix, CASE_SENSITIVE_BASE_SUFFIX ;
            self.integer_internal_digit_separator, INTEGER_INTERNAL_DIGIT_SEPARATOR ;
            self.fraction_internal_digit_separator, FRACTION_INTERNAL_DIGIT_SEPARATOR ;
            self.exponent_internal_digit_separator, EXPONENT_INTERNAL_DIGIT_SEPARATOR ;
            self.integer_leading_digit_separator, INTEGER_LEADING_DIGIT_SEPARATOR ;
            self.fraction_leading_digit_separator, FRACTION_LEADING_DIGIT_SEPARATOR ;
            self.exponent_leading_digit_separator, EXPONENT_LEADING_DIGIT_SEPARATOR ;
            self.integer_trailing_digit_separator, INTEGER_TRAILING_DIGIT_SEPARATOR ;
            self.fraction_trailing_digit_separator, FRACTION_TRAILING_DIGIT_SEPARATOR ;
            self.exponent_trailing_digit_separator, EXPONENT_TRAILING_DIGIT_SEPARATOR ;
            self.integer_consecutive_digit_separator, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR ;
            self.fraction_consecutive_digit_separator, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR ;
            self.exponent_consecutive_digit_separator, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR ;
            self.special_digit_separator, SPECIAL_DIGIT_SEPARATOR ;
        );
        if format & flags::DIGIT_SEPARATOR_FLAG_MASK != 0 {
            format |=
                (unwrap_or_zero(self.digit_separator) as u128) << flags::DIGIT_SEPARATOR_SHIFT;
        }
        format |= (unwrap_or_zero(self.base_prefix) as u128) << flags::BASE_PREFIX_SHIFT;
        format |= (unwrap_or_zero(self.base_suffix) as u128) << flags::BASE_SUFFIX_SHIFT;
        format |= (self.mantissa_radix as u128) << flags::MANTISSA_RADIX_SHIFT;
        format |= (unwrap_or_zero(self.exponent_base) as u128) << flags::EXPONENT_BASE_SHIFT;
        format |= (unwrap_or_zero(self.exponent_radix) as u128) << flags::EXPONENT_RADIX_SHIFT;

        format
    }

    /// Re-create builder from format.
    #[inline(always)]
    pub const fn rebuild(format: u128) -> Self {
        NumberFormatBuilder {
            digit_separator: num::NonZeroU8::new(flags::digit_separator(format)),
            base_prefix: num::NonZeroU8::new(flags::base_prefix(format)),
            base_suffix: num::NonZeroU8::new(flags::base_suffix(format)),
            mantissa_radix: flags::mantissa_radix(format) as u8,
            exponent_base: num::NonZeroU8::new(flags::exponent_base(format) as u8),
            exponent_radix: num::NonZeroU8::new(flags::exponent_radix(format) as u8),
            required_integer_digits: has_flag!(format, REQUIRED_INTEGER_DIGITS),
            required_fraction_digits: has_flag!(format, REQUIRED_FRACTION_DIGITS),
            required_exponent_digits: has_flag!(format, REQUIRED_EXPONENT_DIGITS),
            required_mantissa_digits: has_flag!(format, REQUIRED_MANTISSA_DIGITS),
            no_positive_mantissa_sign: has_flag!(format, NO_POSITIVE_MANTISSA_SIGN),
            required_mantissa_sign: has_flag!(format, REQUIRED_MANTISSA_SIGN),
            no_exponent_notation: has_flag!(format, NO_EXPONENT_NOTATION),
            no_positive_exponent_sign: has_flag!(format, NO_POSITIVE_EXPONENT_SIGN),
            required_exponent_sign: has_flag!(format, REQUIRED_EXPONENT_SIGN),
            no_exponent_without_fraction: has_flag!(format, NO_EXPONENT_WITHOUT_FRACTION),
            no_special: has_flag!(format, NO_SPECIAL),
            case_sensitive_special: has_flag!(format, CASE_SENSITIVE_SPECIAL),
            no_integer_leading_zeros: has_flag!(format, NO_INTEGER_LEADING_ZEROS),
            no_float_leading_zeros: has_flag!(format, NO_FLOAT_LEADING_ZEROS),
            required_exponent_notation: has_flag!(format, REQUIRED_EXPONENT_NOTATION),
            case_sensitive_exponent: has_flag!(format, CASE_SENSITIVE_EXPONENT),
            case_sensitive_base_prefix: has_flag!(format, CASE_SENSITIVE_BASE_PREFIX),
            case_sensitive_base_suffix: has_flag!(format, CASE_SENSITIVE_BASE_SUFFIX),
            integer_internal_digit_separator: has_flag!(format, INTEGER_INTERNAL_DIGIT_SEPARATOR),
            fraction_internal_digit_separator: has_flag!(format, FRACTION_INTERNAL_DIGIT_SEPARATOR),
            exponent_internal_digit_separator: has_flag!(format, EXPONENT_INTERNAL_DIGIT_SEPARATOR),
            integer_leading_digit_separator: has_flag!(format, INTEGER_LEADING_DIGIT_SEPARATOR),
            fraction_leading_digit_separator: has_flag!(format, FRACTION_LEADING_DIGIT_SEPARATOR),
            exponent_leading_digit_separator: has_flag!(format, EXPONENT_LEADING_DIGIT_SEPARATOR),
            integer_trailing_digit_separator: has_flag!(format, INTEGER_TRAILING_DIGIT_SEPARATOR),
            fraction_trailing_digit_separator: has_flag!(format, FRACTION_TRAILING_DIGIT_SEPARATOR),
            exponent_trailing_digit_separator: has_flag!(format, EXPONENT_TRAILING_DIGIT_SEPARATOR),
            integer_consecutive_digit_separator: has_flag!(
                format,
                INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
            ),
            fraction_consecutive_digit_separator: has_flag!(
                format,
                FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
            ),
            exponent_consecutive_digit_separator: has_flag!(
                format,
                EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
            ),
            special_digit_separator: has_flag!(format, SPECIAL_DIGIT_SEPARATOR),
        }
    }
}

impl Default for NumberFormatBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
