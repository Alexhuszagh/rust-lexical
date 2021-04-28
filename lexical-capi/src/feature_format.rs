//! C-compatible NumberFormat functions.

#![cfg(feature = "format")]

use super::option::Option;

/// Builder for `NumberFormat`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NumberFormatBuilder {
    digit_separator: u8,
    decimal_point: u8,
    exponent_default: u8,
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
}

// Simplify conversion to and from lexical_core types..
impl From<lexical_core::NumberFormatBuilder> for NumberFormatBuilder {
    #[inline(always)]
    fn from(builder: lexical_core::NumberFormatBuilder) -> NumberFormatBuilder {
        NumberFormatBuilder {
            digit_separator: builder.get_digit_separator(),
            decimal_point: builder.get_decimal_point(),
            exponent_default: builder.get_exponent_default(),
            exponent_backup: builder.get_exponent_backup(),
            required_integer_digits: builder.get_required_integer_digits(),
            required_fraction_digits: builder.get_required_fraction_digits(),
            required_exponent_digits: builder.get_required_exponent_digits(),
            no_positive_mantissa_sign: builder.get_no_positive_mantissa_sign(),
            required_mantissa_sign: builder.get_required_mantissa_sign(),
            no_exponent_notation: builder.get_no_exponent_notation(),
            no_positive_exponent_sign: builder.get_no_positive_exponent_sign(),
            required_exponent_sign: builder.get_required_exponent_sign(),
            no_exponent_without_fraction: builder.get_no_exponent_without_fraction(),
            no_special: builder.get_no_special(),
            case_sensitive_special: builder.get_case_sensitive_special(),
            no_integer_leading_zeros: builder.get_no_integer_leading_zeros(),
            no_float_leading_zeros: builder.get_no_float_leading_zeros(),
            integer_internal_digit_separator: builder.get_integer_internal_digit_separator(),
            fraction_internal_digit_separator: builder.get_fraction_internal_digit_separator(),
            exponent_internal_digit_separator: builder.get_exponent_internal_digit_separator(),
            integer_leading_digit_separator: builder.get_integer_leading_digit_separator(),
            fraction_leading_digit_separator: builder.get_fraction_leading_digit_separator(),
            exponent_leading_digit_separator: builder.get_exponent_leading_digit_separator(),
            integer_trailing_digit_separator: builder.get_integer_trailing_digit_separator(),
            fraction_trailing_digit_separator: builder.get_fraction_trailing_digit_separator(),
            exponent_trailing_digit_separator: builder.get_exponent_trailing_digit_separator(),
            integer_consecutive_digit_separator: builder.get_integer_consecutive_digit_separator(),
            fraction_consecutive_digit_separator: builder.get_fraction_consecutive_digit_separator(),
            exponent_consecutive_digit_separator: builder.get_exponent_consecutive_digit_separator(),
            special_digit_separator: builder.get_special_digit_separator(),
        }
    }
}

impl Into<lexical_core::NumberFormatBuilder> for NumberFormatBuilder {
    #[inline(always)]
    fn into(self) -> lexical_core::NumberFormatBuilder {
        lexical_core::NumberFormatBuilder::new()
            .digit_separator(self.digit_separator)
            .decimal_point(self.decimal_point)
            .exponent_default(self.exponent_default)
            .exponent_backup(self.exponent_backup)
            .required_integer_digits(self.required_integer_digits)
            .required_fraction_digits(self.required_fraction_digits)
            .required_exponent_digits(self.required_exponent_digits)
            .no_positive_mantissa_sign(self.no_positive_mantissa_sign)
            .required_mantissa_sign(self.required_mantissa_sign)
            .no_exponent_notation(self.no_exponent_notation)
            .no_positive_exponent_sign(self.no_positive_exponent_sign)
            .required_exponent_sign(self.required_exponent_sign)
            .no_exponent_without_fraction(self.no_exponent_without_fraction)
            .no_special(self.no_special)
            .case_sensitive_special(self.case_sensitive_special)
            .no_integer_leading_zeros(self.no_integer_leading_zeros)
            .no_float_leading_zeros(self.no_float_leading_zeros)
            .integer_internal_digit_separator(self.integer_internal_digit_separator)
            .fraction_internal_digit_separator(self.fraction_internal_digit_separator)
            .exponent_internal_digit_separator(self.exponent_internal_digit_separator)
            .integer_leading_digit_separator(self.integer_leading_digit_separator)
            .fraction_leading_digit_separator(self.fraction_leading_digit_separator)
            .exponent_leading_digit_separator(self.exponent_leading_digit_separator)
            .integer_trailing_digit_separator(self.integer_trailing_digit_separator)
            .fraction_trailing_digit_separator(self.fraction_trailing_digit_separator)
            .exponent_trailing_digit_separator(self.exponent_trailing_digit_separator)
            .integer_consecutive_digit_separator(self.integer_consecutive_digit_separator)
            .fraction_consecutive_digit_separator(self.fraction_consecutive_digit_separator)
            .exponent_consecutive_digit_separator(self.exponent_consecutive_digit_separator)
            .special_digit_separator(self.special_digit_separator)
    }
}

impl Default for NumberFormatBuilder {
    #[inline(always)]
    fn default() -> Self {
        lexical_core::NumberFormatBuilder::default().into()
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_number_format_rebuild(format: lexical_core::NumberFormat)
    -> NumberFormatBuilder
{
    format.rebuild().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_number_format_builder_new()
    -> NumberFormatBuilder
{
    lexical_core::NumberFormatBuilder::new().into()
}

#[no_mangle]
#[doc(hidden)]
pub extern fn lexical_number_format_builder_build(builder: NumberFormatBuilder)
    -> Option<lexical_core::NumberFormat>
{
    let builder: lexical_core::NumberFormatBuilder = builder.into();
    builder.build().map(|opts| opts.into()).into()
}
