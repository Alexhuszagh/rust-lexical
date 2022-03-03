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
#![cfg_attr(feature = "format", doc = " - [RUST_LITERAL](crate::format::RUST_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [RUST_STRING](crate::format::RUST_STRING)")]
#![cfg_attr(feature = "format", doc = " - [PYTHON_LITERAL](crate::format::PYTHON_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [PYTHON_STRING](crate::format::PYTHON_STRING)")]
#![cfg_attr(feature = "format", doc = " - [PYTHON3_LITERAL](crate::format::PYTHON3_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [PYTHON3_STRING](crate::format::PYTHON3_STRING)")]
#![cfg_attr(feature = "format", doc = " - [PYTHON36_LITERAL](crate::format::PYTHON36_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [PYTHON35_LITERAL](crate::format::PYTHON35_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [PYTHON2_LITERAL](crate::format::PYTHON2_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [PYTHON2_STRING](crate::format::PYTHON2_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CXX_LITERAL](crate::format::CXX_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CXX_STRING](crate::format::CXX_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [CXX_HEX_LITERAL](crate::format::CXX_HEX_LITERAL)"
)]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [CXX_HEX_STRING](crate::format::CXX_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [CXX20_LITERAL](crate::format::CXX20_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CXX20_STRING](crate::format::CXX20_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [CXX20_HEX_LITERAL](crate::format::CXX20_HEX_LITERAL)"
)]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [CXX20_HEX_STRING](crate::format::CXX20_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [CXX17_LITERAL](crate::format::CXX17_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CXX17_STRING](crate::format::CXX17_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [CXX17_HEX_LITERAL](crate::format::CXX17_HEX_LITERAL)"
)]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [CXX17_HEX_STRING](crate::format::CXX17_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [CXX14_LITERAL](crate::format::CXX14_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CXX14_STRING](crate::format::CXX14_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [CXX14_HEX_STRING](crate::format::CXX14_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [CXX11_LITERAL](crate::format::CXX11_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CXX11_STRING](crate::format::CXX11_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [CXX11_HEX_STRING](crate::format::CXX11_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [CXX03_LITERAL](crate::format::CXX03_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CXX03_STRING](crate::format::CXX03_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CXX98_LITERAL](crate::format::CXX98_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CXX98_STRING](crate::format::CXX98_STRING)")]
#![cfg_attr(feature = "format", doc = " - [C_LITERAL](crate::format::C_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [C_STRING](crate::format::C_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C_HEX_LITERAL](crate::format::C_HEX_LITERAL)"
)]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C_HEX_STRING](crate::format::C_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [C18_LITERAL](crate::format::C18_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [C18_STRING](crate::format::C18_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C18_HEX_LITERAL](crate::format::C18_HEX_LITERAL)"
)]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C18_HEX_STRING](crate::format::C18_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [C11_LITERAL](crate::format::C11_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [C11_STRING](crate::format::C11_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C11_HEX_LITERAL](crate::format::C11_HEX_LITERAL)"
)]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C11_HEX_STRING](crate::format::C11_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [C99_LITERAL](crate::format::C99_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [C99_STRING](crate::format::C99_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C99_HEX_LITERAL](crate::format::C99_HEX_LITERAL)"
)]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C99_HEX_STRING](crate::format::C99_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [C90_LITERAL](crate::format::C90_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [C90_STRING](crate::format::C90_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C90_HEX_STRING](crate::format::C90_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [C89_LITERAL](crate::format::C89_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [C89_STRING](crate::format::C89_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [C89_HEX_STRING](crate::format::C89_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [RUBY_LITERAL](crate::format::RUBY_LITERAL)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [RUBY_OCTAL_LITERAL](crate::format::RUBY_OCTAL_LITERAL)"
)]
#![cfg_attr(feature = "format", doc = " - [RUBY_STRING](crate::format::RUBY_STRING)")]
#![cfg_attr(feature = "format", doc = " - [SWIFT_LITERAL](crate::format::SWIFT_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [SWIFT_STRING](crate::format::SWIFT_STRING)")]
#![cfg_attr(feature = "format", doc = " - [GO_LITERAL](crate::format::GO_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [GO_STRING](crate::format::GO_STRING)")]
#![cfg_attr(feature = "format", doc = " - [HASKELL_LITERAL](crate::format::HASKELL_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [HASKELL_STRING](crate::format::HASKELL_STRING)")]
#![cfg_attr(feature = "format", doc = " - [JAVASCRIPT_LITERAL](crate::format::JAVASCRIPT_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [JAVASCRIPT_STRING](crate::format::JAVASCRIPT_STRING)")]
#![cfg_attr(feature = "format", doc = " - [PERL_LITERAL](crate::format::PERL_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [PERL_STRING](crate::format::PERL_STRING)")]
#![cfg_attr(feature = "format", doc = " - [PHP_LITERAL](crate::format::PHP_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [PHP_STRING](crate::format::PHP_STRING)")]
#![cfg_attr(feature = "format", doc = " - [JAVA_LITERAL](crate::format::JAVA_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [JAVA_STRING](crate::format::JAVA_STRING)")]
#![cfg_attr(feature = "format", doc = " - [R_LITERAL](crate::format::R_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [R_STRING](crate::format::R_STRING)")]
#![cfg_attr(feature = "format", doc = " - [KOTLIN_LITERAL](crate::format::KOTLIN_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [KOTLIN_STRING](crate::format::KOTLIN_STRING)")]
#![cfg_attr(feature = "format", doc = " - [JULIA_LITERAL](crate::format::JULIA_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [JULIA_STRING](crate::format::JULIA_STRING)")]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [JULIA_HEX_LITERAL](crate::format::JULIA_HEX_LITERAL)"
)]
#![cfg_attr(
    all(feature = "format", feature = "power-of-two"),
    doc = " - [JULIA_HEX_STRING](crate::format::JULIA_HEX_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [CSHARP_LITERAL](crate::format::CSHARP_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP_STRING](crate::format::CSHARP_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP7_LITERAL](crate::format::CSHARP7_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP7_STRING](crate::format::CSHARP7_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP6_LITERAL](crate::format::CSHARP6_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP6_STRING](crate::format::CSHARP6_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP5_LITERAL](crate::format::CSHARP5_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP5_STRING](crate::format::CSHARP5_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP4_LITERAL](crate::format::CSHARP4_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP4_STRING](crate::format::CSHARP4_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP3_LITERAL](crate::format::CSHARP3_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP3_STRING](crate::format::CSHARP3_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP2_LITERAL](crate::format::CSHARP2_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP2_STRING](crate::format::CSHARP2_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP1_LITERAL](crate::format::CSHARP1_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CSHARP1_STRING](crate::format::CSHARP1_STRING)")]
#![cfg_attr(feature = "format", doc = " - [KAWA_LITERAL](crate::format::KAWA_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [KAWA_STRING](crate::format::KAWA_STRING)")]
#![cfg_attr(feature = "format", doc = " - [GAMBITC_LITERAL](crate::format::GAMBITC_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [GAMBITC_STRING](crate::format::GAMBITC_STRING)")]
#![cfg_attr(feature = "format", doc = " - [GUILE_LITERAL](crate::format::GUILE_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [GUILE_STRING](crate::format::GUILE_STRING)")]
#![cfg_attr(feature = "format", doc = " - [CLOJURE_LITERAL](crate::format::CLOJURE_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [CLOJURE_STRING](crate::format::CLOJURE_STRING)")]
#![cfg_attr(feature = "format", doc = " - [ERLANG_LITERAL](crate::format::ERLANG_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [ERLANG_STRING](crate::format::ERLANG_STRING)")]
#![cfg_attr(feature = "format", doc = " - [ELM_LITERAL](crate::format::ELM_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [ELM_STRING](crate::format::ELM_STRING)")]
#![cfg_attr(feature = "format", doc = " - [SCALA_LITERAL](crate::format::SCALA_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [SCALA_STRING](crate::format::SCALA_STRING)")]
#![cfg_attr(feature = "format", doc = " - [ELIXIR_LITERAL](crate::format::ELIXIR_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [ELIXIR_STRING](crate::format::ELIXIR_STRING)")]
#![cfg_attr(feature = "format", doc = " - [FORTRAN_LITERAL](crate::format::FORTRAN_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [FORTRAN_STRING](crate::format::FORTRAN_STRING)")]
#![cfg_attr(feature = "format", doc = " - [D_LITERAL](crate::format::D_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [D_STRING](crate::format::D_STRING)")]
#![cfg_attr(
    feature = "format",
    doc = " - [COFFEESCRIPT_LITERAL](crate::format::COFFEESCRIPT_LITERAL)"
)]
#![cfg_attr(
    feature = "format",
    doc = " - [COFFEESCRIPT_STRING](crate::format::COFFEESCRIPT_STRING)"
)]
#![cfg_attr(feature = "format", doc = " - [COBOL_LITERAL](crate::format::COBOL_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [COBOL_STRING](crate::format::COBOL_STRING)")]
#![cfg_attr(feature = "format", doc = " - [FSHARP_LITERAL](crate::format::FSHARP_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [FSHARP_STRING](crate::format::FSHARP_STRING)")]
#![cfg_attr(feature = "format", doc = " - [VB_LITERAL](crate::format::VB_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [VB_STRING](crate::format::VB_STRING)")]
#![cfg_attr(feature = "format", doc = " - [OCAML_LITERAL](crate::format::OCAML_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [OCAML_STRING](crate::format::OCAML_STRING)")]
#![cfg_attr(feature = "format", doc = " - [OBJECTIVEC_LITERAL](crate::format::OBJECTIVEC_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [OBJECTIVEC_STRING](crate::format::OBJECTIVEC_STRING)")]
#![cfg_attr(feature = "format", doc = " - [REASONML_LITERAL](crate::format::REASONML_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [REASONML_STRING](crate::format::REASONML_STRING)")]
#![cfg_attr(feature = "format", doc = " - [OCTAVE_LITERAL](crate::format::OCTAVE_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [OCTAVE_STRING](crate::format::OCTAVE_STRING)")]
#![cfg_attr(feature = "format", doc = " - [MATLAB_LITERAL](crate::format::MATLAB_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [MATLAB_STRING](crate::format::MATLAB_STRING)")]
#![cfg_attr(feature = "format", doc = " - [ZIG_LITERAL](crate::format::ZIG_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [ZIG_STRING](crate::format::ZIG_STRING)")]
#![cfg_attr(feature = "format", doc = " - [SAGE_LITERAL](crate::format::SAGE_LITERAL)")]
#![cfg_attr(feature = "format", doc = " - [SAGE_STRING](crate::format::SAGE_STRING)")]
#![cfg_attr(feature = "format", doc = " - [JSON](crate::format::JSON)")]
#![cfg_attr(feature = "format", doc = " - [TOML](crate::format::TOML)")]
#![cfg_attr(feature = "format", doc = " - [YAML](crate::format::YAML)")]
#![cfg_attr(feature = "format", doc = " - [XML](crate::format::XML)")]
#![cfg_attr(feature = "format", doc = " - [SQLITE](crate::format::SQLITE)")]
#![cfg_attr(feature = "format", doc = " - [POSTGRESQL](crate::format::POSTGRESQL)")]
#![cfg_attr(feature = "format", doc = " - [MYSQL](crate::format::MYSQL)")]
#![cfg_attr(feature = "format", doc = " - [MONGODB](crate::format::MONGODB)")]
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

use crate::error::Error;
use static_assertions::const_assert;

/// Determine if the format packed struct is valid.
#[inline]
pub const fn format_is_valid<const FORMAT: u128>() -> bool {
    NumberFormat::<FORMAT> {}.is_valid()
}

/// Get the error type from the format packed struct.
///
/// An error type of `Error::Success` means the format is valid, any
/// other error signifies an invalid format.
#[inline]
pub const fn format_error<const FORMAT: u128>() -> Error {
    NumberFormat::<FORMAT> {}.error()
}

/// Standard number format. This is identical to the Rust string format.
pub const STANDARD: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ STANDARD }> {}.is_valid());
