#![cfg(not(feature = "format"))]

use lexical_util::format::{self, NumberFormat, STANDARD};

#[test]
fn format_properties_test() {
    let format = NumberFormat::<{ STANDARD }> {};
    assert_eq!(format.flags(), STANDARD & format::FLAG_MASK);
    assert_eq!(format.interface_flags(), STANDARD & format::INTERFACE_FLAG_MASK);
    assert_eq!(format.digit_separator(), b'\x00');
    assert_eq!(format.base_prefix(), b'\x00');
    assert_eq!(format.base_suffix(), b'\x00');
    assert_eq!(format.mantissa_radix(), 10);
    assert_eq!(format.radix(), 10);
    assert_eq!(format.exponent_base(), 10);
    assert_eq!(format.exponent_radix(), 10);
    assert_eq!(format.required_integer_digits(), false);
    assert_eq!(format.required_fraction_digits(), false);
    assert_eq!(format.required_exponent_digits(), true);
    assert_eq!(format.required_mantissa_digits(), true);
    assert_eq!(format.required_digits(), true);
    assert_eq!(format.no_positive_mantissa_sign(), false);
    assert_eq!(format.required_mantissa_sign(), false);
    assert_eq!(format.no_exponent_notation(), false);
    assert_eq!(format.no_positive_exponent_sign(), false);
    assert_eq!(format.required_exponent_sign(), false);
    assert_eq!(format.no_exponent_without_fraction(), false);
    assert_eq!(format.no_special(), false);
    assert_eq!(format.case_sensitive_special(), false);
    assert_eq!(format.no_integer_leading_zeros(), false);
    assert_eq!(format.no_float_leading_zeros(), false);
    assert_eq!(format.required_exponent_notation(), false);
    assert_eq!(format.case_sensitive_exponent(), false);
    assert_eq!(format.case_sensitive_base_prefix(), false);
    assert_eq!(format.case_sensitive_base_suffix(), false);
    assert_eq!(format.integer_internal_digit_separator(), false);
    assert_eq!(format.fraction_internal_digit_separator(), false);
    assert_eq!(format.exponent_internal_digit_separator(), false);
    assert_eq!(format.internal_digit_separator(), false);
    assert_eq!(format.integer_leading_digit_separator(), false);
    assert_eq!(format.fraction_leading_digit_separator(), false);
    assert_eq!(format.exponent_leading_digit_separator(), false);
    assert_eq!(format.leading_digit_separator(), false);
    assert_eq!(format.integer_trailing_digit_separator(), false);
    assert_eq!(format.fraction_trailing_digit_separator(), false);
    assert_eq!(format.exponent_trailing_digit_separator(), false);
    assert_eq!(format.trailing_digit_separator(), false);
    assert_eq!(format.integer_consecutive_digit_separator(), false);
    assert_eq!(format.fraction_consecutive_digit_separator(), false);
    assert_eq!(format.exponent_consecutive_digit_separator(), false);
    assert_eq!(format.consecutive_digit_separator(), false);
    assert_eq!(format.special_digit_separator(), false);
}
