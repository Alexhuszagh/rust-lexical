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
//! - [STANDARD]
#![cfg_attr(feature = "format", doc = " - [`RUST_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`RUST_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`PYTHON_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`PYTHON_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`PYTHON3_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`PYTHON3_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`PYTHON36_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`PYTHON35_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`PYTHON2_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`PYTHON2_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`CXX_HEX_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`CXX_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX20_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX20_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`CXX20_HEX_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`CXX20_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX17_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX17_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`CXX17_HEX_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`CXX17_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX14_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX14_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`CXX14_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX11_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX11_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`CXX11_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX03_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX03_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX98_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CXX98_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`C_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`C_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C_HEX_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`C18_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`C18_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C18_HEX_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C18_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`C11_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`C11_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C11_HEX_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C11_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`C99_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`C99_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C99_HEX_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C99_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`C90_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`C90_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C90_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`C89_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`C89_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`C89_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`RUBY_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`RUBY_OCTAL_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`RUBY_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`SWIFT_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`SWIFT_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`GO_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`GO_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`HASKELL_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`HASKELL_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`JAVASCRIPT_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`JAVASCRIPT_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`PERL_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`PERL_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`PHP_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`PHP_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`JAVA_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`JAVA_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`R_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`R_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`KOTLIN_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`KOTLIN_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`JULIA_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`JULIA_STRING`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`JULIA_HEX_LITERAL`]")]
#![cfg_attr(all(feature = "format", feature = "power-of-two"), doc = " - [`JULIA_HEX_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP7_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP7_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP6_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP6_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP5_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP5_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP4_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP4_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP3_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP3_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP2_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP2_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP1_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CSHARP1_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`KAWA_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`KAWA_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`GAMBITC_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`GAMBITC_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`GUILE_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`GUILE_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`CLOJURE_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`CLOJURE_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`ERLANG_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`ERLANG_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`ELM_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`ELM_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`SCALA_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`SCALA_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`ELIXIR_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`ELIXIR_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`FORTRAN_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`FORTRAN_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`D_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`D_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`COFFEESCRIPT_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`COFFEESCRIPT_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`COBOL_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`COBOL_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`FSHARP_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`FSHARP_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`VB_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`VB_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`OCAML_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`OCAML_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`OBJECTIVEC_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`OBJECTIVEC_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`REASONML_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`REASONML_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`OCTAVE_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`OCTAVE_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`MATLAB_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`MATLAB_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`ZIG_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`ZIG_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`SAGE_LITERAL`]")]
#![cfg_attr(feature = "format", doc = " - [`SAGE_STRING`]")]
#![cfg_attr(feature = "format", doc = " - [`JSON`]")]
#![cfg_attr(feature = "format", doc = " - [`TOML`]")]
#![cfg_attr(feature = "format", doc = " - [`YAML`]")]
#![cfg_attr(feature = "format", doc = " - [`XML`]")]
#![cfg_attr(feature = "format", doc = " - [`SQLITE`]")]
#![cfg_attr(feature = "format", doc = " - [`POSTGRESQL`]")]
#![cfg_attr(feature = "format", doc = " - [`MYSQL`]")]
#![cfg_attr(feature = "format", doc = " - [`MONGODB`]")]
//!
//! # Syntax Flags
//!
//! Bitflags to get and set syntax flags for the format packed struct.
//!
//! - [`REQUIRED_INTEGER_DIGITS`]
//! - [`REQUIRED_FRACTION_DIGITS`]
//! - [`REQUIRED_EXPONENT_DIGITS`]
//! - [`REQUIRED_MANTISSA_DIGITS`]
//! - [`REQUIRED_DIGITS`]
//! - [`NO_POSITIVE_MANTISSA_SIGN`]
//! - [`REQUIRED_MANTISSA_SIGN`]
//! - [`NO_EXPONENT_NOTATION`]
//! - [`NO_POSITIVE_EXPONENT_SIGN`]
//! - [`REQUIRED_EXPONENT_SIGN`]
//! - [`NO_EXPONENT_WITHOUT_FRACTION`]
//! - [`NO_SPECIAL`]
//! - [`CASE_SENSITIVE_SPECIAL`]
//! - [`NO_INTEGER_LEADING_ZEROS`]
//! - [`NO_FLOAT_LEADING_ZEROS`]
//! - [`REQUIRED_EXPONENT_NOTATION`]
//! - [`CASE_SENSITIVE_EXPONENT`]
//! - [`CASE_SENSITIVE_BASE_PREFIX`]
//! - [`CASE_SENSITIVE_BASE_SUFFIX`]
//!
//! # Digit Separator Flags
//!
//! Bitflags to get and set digit separators flags for the format
//! packed struct.
//!
//! - [`INTEGER_INTERNAL_DIGIT_SEPARATOR`]
//! - [`FRACTION_INTERNAL_DIGIT_SEPARATOR`]
//! - [`EXPONENT_INTERNAL_DIGIT_SEPARATOR`]
//! - [`INTEGER_LEADING_DIGIT_SEPARATOR`]
//! - [`FRACTION_LEADING_DIGIT_SEPARATOR`]
//! - [`EXPONENT_LEADING_DIGIT_SEPARATOR`]
//! - [`INTEGER_TRAILING_DIGIT_SEPARATOR`]
//! - [`FRACTION_TRAILING_DIGIT_SEPARATOR`]
//! - [`EXPONENT_TRAILING_DIGIT_SEPARATOR`]
//! - [`INTEGER_CONSECUTIVE_DIGIT_SEPARATOR`]
//! - [`FRACTION_CONSECUTIVE_DIGIT_SEPARATOR`]
//! - [`EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR`]
//! - [`INTERNAL_DIGIT_SEPARATOR`]
//! - [`LEADING_DIGIT_SEPARATOR`]
//! - [`TRAILING_DIGIT_SEPARATOR`]
//! - [`CONSECUTIVE_DIGIT_SEPARATOR`]
//! - [`SPECIAL_DIGIT_SEPARATOR`]
//!
//! # Character Shifts and Masks
//!
//! Bitmasks and bit shifts to get and set control characters for the format
//! packed struct.
//!
//! - [`DIGIT_SEPARATOR_SHIFT`]
//! - [`DIGIT_SEPARATOR`]
//! - [`BASE_PREFIX_SHIFT`]
//! - [`BASE_PREFIX`]
//! - [`BASE_SUFFIX_SHIFT`]
//! - [`BASE_SUFFIX`]
//! - [`MANTISSA_RADIX_SHIFT`]
//! - [`MANTISSA_RADIX`]
//! - [`RADIX_SHIFT`]
//! - [`RADIX`]
//! - [`EXPONENT_BASE_SHIFT`]
//! - [`EXPONENT_BASE`]
//! - [`EXPONENT_RADIX_SHIFT`]
//! - [`EXPONENT_RADIX`]
//!
//! # Character Functions
//!
//! Functions to get control characters from the format packed struct.
//!
//! - [`digit_separator`]
//! - [`base_prefix`]
//! - [`base_suffix`]
//! - [`mantissa_radix`]
//! - [`exponent_base`]
//! - [`exponent_radix`]
//! - [`radix_from_flags`]
//!
//! # Validators
//!
//! Functions to validate control characters for the format packed struct.
//!
//! - [`is_valid_digit_separator`]
//! - [`is_valid_base_prefix`]
//! - [`is_valid_base_suffix`]
//! - [`is_valid_punctuation`]
//! - [`is_valid_radix`]

use static_assertions::const_assert;

use crate::error::Error;
#[cfg(feature = "format")]
pub use crate::feature_format::*;
pub use crate::format_builder::*;
pub use crate::format_flags::*;
#[cfg(not(feature = "format"))]
pub use crate::not_feature_format::*;

/// Determine if the format packed struct is valid.
#[inline(always)]
pub const fn format_is_valid<const FORMAT: u128>() -> bool {
    NumberFormat::<FORMAT> {}.is_valid()
}

/// Get the error type from the format packed struct.
///
/// An error type of `Error::Success` means the format is valid, any
/// other error signifies an invalid format.
#[inline(always)]
pub const fn format_error<const FORMAT: u128>() -> Error {
    NumberFormat::<FORMAT> {}.error()
}

/// Standard number format. This is identical to the Rust string format.
pub const STANDARD: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ STANDARD }> {}.is_valid());
