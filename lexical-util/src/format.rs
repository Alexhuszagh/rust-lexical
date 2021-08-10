//! Public API for the number format packed struct.
//!
//! This has a consistent API whether or not the `format` feature is
//! enabled, however, most functionality will be disabled if the feature
//! is not enabled.
//!
//! # Pre-Defined Formats
//!
//! These are the pre-defined formats for parsing numbers from various
//! programming, markup, and data languages.
//!
//! - [STANDARD](crate::format::STANDARD)
//!
//! # Syntax Flags
//!
//! Bitflags to get and set syntax flags for the format packed struct.
//!
//! - [REQUIRED_INTEGER_DIGITS](crate::format::REQUIRED_INTEGER_DIGITS)
//! - [REQUIRED_FRACTION_DIGITS](crate::format::REQUIRED_FRACTION_DIGITS)
//! - [REQUIRED_EXPONENT_DIGITS](crate::format::REQUIRED_EXPONENT_DIGITS)
//! - [REQUIRED_MANTISSA_DIGITS](crate::format::REQUIRED_MANTISSA_DIGITS)
//! - [REQUIRED_DIGITS](crate::format::REQUIRED_DIGITS)
//! - [NO_POSITIVE_MANTISSA_SIGN](crate::format::NO_POSITIVE_MANTISSA_SIGN)
//! - [REQUIRED_MANTISSA_SIGN](crate::format::REQUIRED_MANTISSA_SIGN)
//! - [NO_EXPONENT_NOTATION](crate::format::NO_EXPONENT_NOTATION)
//! - [NO_POSITIVE_EXPONENT_SIGN](crate::format::NO_POSITIVE_EXPONENT_SIGN)
//! - [REQUIRED_EXPONENT_SIGN](crate::format::REQUIRED_EXPONENT_SIGN)
//! - [NO_EXPONENT_WITHOUT_FRACTION](crate::format::NO_EXPONENT_WITHOUT_FRACTION)
//! - [NO_SPECIAL](crate::format::NO_SPECIAL)
//! - [CASE_SENSITIVE_SPECIAL](crate::format::CASE_SENSITIVE_SPECIAL)
//! - [NO_INTEGER_LEADING_ZEROS](crate::format::NO_INTEGER_LEADING_ZEROS)
//! - [NO_FLOAT_LEADING_ZEROS](crate::format::NO_FLOAT_LEADING_ZEROS)
//! - [REQUIRED_EXPONENT_NOTATION](crate::format::REQUIRED_EXPONENT_NOTATION)
//! - [CASE_SENSITIVE_EXPONENT](crate::format::CASE_SENSITIVE_EXPONENT)
//! - [CASE_SENSITIVE_BASE_PREFIX](crate::format::CASE_SENSITIVE_BASE_PREFIX)
//! - [CASE_SENSITIVE_BASE_SUFFIX](crate::format::CASE_SENSITIVE_BASE_SUFFIX)
//!
//! # Digit Separator Flags
//!
//! Bitflags to get and set digit separators flags for the format
//! packed struct.
//!
//! - [INTEGER_INTERNAL_DIGIT_SEPARATOR](crate::format::INTEGER_INTERNAL_DIGIT_SEPARATOR)
//! - [FRACTION_INTERNAL_DIGIT_SEPARATOR](crate::format::FRACTION_INTERNAL_DIGIT_SEPARATOR)
//! - [EXPONENT_INTERNAL_DIGIT_SEPARATOR](crate::format::EXPONENT_INTERNAL_DIGIT_SEPARATOR)
//! - [INTEGER_LEADING_DIGIT_SEPARATOR](crate::format::INTEGER_LEADING_DIGIT_SEPARATOR)
//! - [FRACTION_LEADING_DIGIT_SEPARATOR](crate::format::FRACTION_LEADING_DIGIT_SEPARATOR)
//! - [EXPONENT_LEADING_DIGIT_SEPARATOR](crate::format::EXPONENT_LEADING_DIGIT_SEPARATOR)
//! - [INTEGER_TRAILING_DIGIT_SEPARATOR](crate::format::INTEGER_TRAILING_DIGIT_SEPARATOR)
//! - [FRACTION_TRAILING_DIGIT_SEPARATOR](crate::format::FRACTION_TRAILING_DIGIT_SEPARATOR)
//! - [EXPONENT_TRAILING_DIGIT_SEPARATOR](crate::format::EXPONENT_TRAILING_DIGIT_SEPARATOR)
//! - [INTEGER_CONSECUTIVE_DIGIT_SEPARATOR](crate::format::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR)
//! - [FRACTION_CONSECUTIVE_DIGIT_SEPARATOR](crate::format::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR)
//! - [EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR](crate::format::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR)
//! - [INTERNAL_DIGIT_SEPARATOR](crate::format::INTERNAL_DIGIT_SEPARATOR)
//! - [LEADING_DIGIT_SEPARATOR](crate::format::LEADING_DIGIT_SEPARATOR)
//! - [TRAILING_DIGIT_SEPARATOR](crate::format::TRAILING_DIGIT_SEPARATOR)
//! - [CONSECUTIVE_DIGIT_SEPARATOR](crate::format::CONSECUTIVE_DIGIT_SEPARATOR)
//! - [SPECIAL_DIGIT_SEPARATOR](crate::format::SPECIAL_DIGIT_SEPARATOR)
//!
//! # Character Shifts and Masks
//!
//! Bitmasks and bitshifts to get and set control characters for the format
//! packed struct.
//!
//! - [DIGIT_SEPARATOR_SHIFT](crate::format::DIGIT_SEPARATOR_SHIFT)
//! - [DIGIT_SEPARATOR](crate::format::DIGIT_SEPARATOR)
//! - [DECIMAL_POINT_SHIFT](crate::format::DECIMAL_POINT_SHIFT)
//! - [DECIMAL_POINT](crate::format::DECIMAL_POINT)
//! - [EXPONENT_SHIFT](crate::format::EXPONENT_SHIFT)
//! - [EXPONENT](crate::format::EXPONENT)
//! - [BASE_PREFIX_SHIFT](crate::format::BASE_PREFIX_SHIFT)
//! - [BASE_PREFIX](crate::format::BASE_PREFIX)
//! - [BASE_SUFFIX_SHIFT](crate::format::BASE_SUFFIX_SHIFT)
//! - [BASE_SUFFIX](crate::format::BASE_SUFFIX)
//! - [MANTISSA_RADIX_SHIFT](crate::format::MANTISSA_RADIX_SHIFT)
//! - [MANTISSA_RADIX](crate::format::MANTISSA_RADIX)
//! - [RADIX_SHIFT](crate::format::RADIX_SHIFT)
//! - [RADIX](crate::format::RADIX)
//! - [EXPONENT_BASE_SHIFT](crate::format::EXPONENT_BASE_SHIFT)
//! - [EXPONENT_BASE](crate::format::EXPONENT_BASE)
//! - [EXPONENT_RADIX_SHIFT](crate::format::EXPONENT_RADIX_SHIFT)
//! - [EXPONENT_RADIX](crate::format::EXPONENT_RADIX)
//!
//! # Character Functions
//!
//! Functions to get control characters from the format packed struct.
//!
//! - [digit_separator](crate::format::digit_separator)
//! - [decimal_point](crate::format::decimal_point)
//! - [exponent](crate::format::exponent)
//! - [base_prefix](crate::format::base_prefix)
//! - [base_suffix](crate::format::base_suffix)
//! - [mantissa_radix](crate::format::mantissa_radix)
//! - [exponent_base](crate::format::exponent_base)
//! - [exponent_radix](crate::format::exponent_radix)
//! - [radix_from_flags](crate::format::radix_from_flags)
//!
//! # Validators
//!
//! Functions to validate control characters for the format packed struct.
//!
//! - [is_valid_digit_separator](is_valid_digit_separator)
//! - [is_valid_decimal_point](is_valid_decimal_point)
//! - [is_valid_exponent](is_valid_exponent)
//! - [is_valid_base_prefix](is_valid_base_prefix)
//! - [is_valid_base_suffix](is_valid_base_suffix)
//! - [is_valid_punctuation](is_valid_punctuation)
//! - [is_valid_radix](is_valid_radix)

#[cfg(feature = "format")]
pub use crate::feature_format::*;
pub use crate::format_builder::*;
pub use crate::format_flags::*;
#[cfg(not(feature = "format"))]
pub use crate::not_feature_format::*;

use static_assertions::const_assert;

/// Standard number format. This is identical to the Rust string format.
pub const STANDARD: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ STANDARD }> {}.is_valid());
