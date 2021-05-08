//! C-compatible error type.

use crate::lib::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error as StdError;

/// PARSE ERROR
// ------------

/// Error code during parsing, indicating failure type.
///
/// Error messages are designating by an error code of less than 0.
/// This is to be compatible with C conventions. This enumeration is
/// FFI-compatible for interfacing with C code.
///
/// # FFI
///
/// For interfacing with FFI-code, this may be approximated by:
/// ```text
/// const int32_t OVERFLOW = -1;
/// const int32_t UNDERFLOW = -2;
/// const int32_t INVALID_DIGIT = -3;
/// const int32_t EMPTY = -4;
/// const int32_t EMPTY_MANTISSA = -5;
/// const int32_t EMPTY_EXPONENT = -6;
/// const int32_t EMPTY_INTEGER = -7;
/// const int32_t EMPTY_FRACTION = -8;
/// const int32_t INVALID_POSITIVE_MANTISSA_SIGN = -9;
/// const int32_t MISSING_MANTISSA_SIGN = -10;
/// const int32_t INVALID_EXPONENT = -11;
/// const int32_t INVALID_POSITIVE_EXPONENT_SIGN = -12;
/// const int32_t MISSING_EXPONENT_SIGN = -13;
/// const int32_t EXPONENT_WITHOUT_FRACTION = -14;
/// const int32_t INVALID_LEADING_ZEROS = -15;
/// const int32_t MISSING_EXPONENT = -16;
/// ```
///
/// # Safety
///
/// Assigning any value outside the range `[-16, -1]` to value of type
/// ParseErrorCode may invoke undefined-behavior.
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParseErrorCode {
    /// Integral overflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Overflow                    = -1,
    /// Integral underflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Underflow                   = -2,
    /// Invalid digit found before string termination.
    InvalidDigit                = -3,
    /// Empty byte array found.
    Empty                       = -4,
    /// Empty mantissa found.
    EmptyMantissa               = -5,
    /// Empty exponent found.
    EmptyExponent               = -6,
    /// Empty integer found.
    EmptyInteger                = -7,
    /// Empty fraction found.
    EmptyFraction               = -8,
    /// Invalid positive mantissa sign was found.
    InvalidPositiveMantissaSign = -9,
    /// Mantissa sign was required, but not found.
    MissingMantissaSign         = -10,
    /// Exponent was present but not allowed.
    InvalidExponent             = -11,
    /// Invalid positive exponent sign was found.
    InvalidPositiveExponentSign = -12,
    /// Exponent sign was required, but not found.
    MissingExponentSign         = -13,
    /// Exponent was present without fraction component.
    ExponentWithoutFraction     = -14,
    /// Integer had invalid leading zeros.
    InvalidLeadingZeros         = -15,
    /// No exponent with required exponent notation.
    MissingExponent             = -16,

    // We may add additional variants later, so ensure that client matching
    // does not depend on exhaustive matching.
    #[doc(hidden)]
    __Nonexhaustive             = -200,
}

impl Display for ParseErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            ParseErrorCode::Overflow => "'numeric overflow occurred'",
            ParseErrorCode::Underflow => "'numeric underflow occurred'",
            ParseErrorCode::InvalidDigit => "'invalid digit found'",
            ParseErrorCode::Empty => "'the string to parse was empty'",
            ParseErrorCode::EmptyMantissa => "'no significant digits found'",
            ParseErrorCode::EmptyExponent => "'exponent notation found without an exponent'",
            ParseErrorCode::EmptyInteger => "'invalid float with no integer digits'",
            ParseErrorCode::EmptyFraction => "'invalid float with no fraction digits'",
            ParseErrorCode::InvalidPositiveMantissaSign => {
                "'invalid `+` sign before significant digits'"
            },
            ParseErrorCode::MissingMantissaSign => {
                "'missing required `+/-` sign for significant digits'"
            },
            ParseErrorCode::InvalidExponent => "'exponent found but not allowed'",
            ParseErrorCode::InvalidPositiveExponentSign => "'invalid `+` sign in exponent'",
            ParseErrorCode::MissingExponentSign => "'missing required `+/-` sign for exponent'",
            ParseErrorCode::ExponentWithoutFraction => {
                "'invalid float containing exponent without fraction'"
            },
            ParseErrorCode::InvalidLeadingZeros => {
                "'invalid number with leading zeros before digits'"
            },
            ParseErrorCode::MissingExponent => "'missing required exponent'",
            _ => unimplemented!(),
        };
        write!(f, "{}", message)
    }
}

/// Error type for lexical parsing.
///
/// This error is FFI-compatible for interfacing with C code.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ParseError {
    /// Error code designating the type of error occurred.
    pub code: ParseErrorCode,
    /// Optional position within the buffer for the error.
    pub index: usize,
}

impl From<ParseErrorCode> for ParseError {
    #[inline]
    fn from(code: ParseErrorCode) -> Self {
        ParseError {
            code,
            index: 0,
        }
    }
}

impl From<(ParseErrorCode, usize)> for ParseError {
    #[inline]
    fn from(error: (ParseErrorCode, usize)) -> Self {
        ParseError {
            code: error.0,
            index: error.1,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "lexical parse error: {} at index {}.", self.code, self.index)
    }
}

#[cfg(feature = "std")]
impl StdError for ParseError {
}

// AlIASES
// -------

/// Alias for ParseErrorCode for backwards compatibility.
pub type ErrorCode = ParseErrorCode;

/// Alias for ParseError for backwards compatibility.
pub type Error = ParseError;

// FORMAT ERROR
// ------------

/// Error code when building `NumberFormat`, indicating failure type.
///
/// Error messages are designating by an error code of less than 0.
/// This is to be compatible with C conventions. This enumeration is
/// FFI-compatible for interfacing with C code.
///
/// # FFI
///
/// For interfacing with FFI-code, this may be approximated by:
/// ```text
/// const int32_t INVALID_EXPONENT_SYMBOL = -1;
/// const int32_t INVALID_DECIMAL_POINT = -2;
/// const int32_t INVALID_MANTISSA_RADIX = -3;
/// const int32_t INVALID_EXPONENT_BASE = -4;
/// const int32_t INVALID_EXPONENT_RADIX = -5;
/// const int32_t INVALID_BASE_PREFIX = -6;
/// const int32_t INVALID_BASE_SUFFIX = -7;
/// const int32_t INVALID_PUNCTUATION = -8;
/// const int32_t INVALID_DIGIT_SEPARATOR = -9;
/// const int32_t INVALID_EXPONENT_FLAGS = -10;
/// const int32_t INVALID_MANTISSA_SIGN = -11;
/// const int32_t INVALID_EXPONENT_SIGN = -12;
/// const int32_t INVALID_SPECIAL = -13;
/// const int32_t INVALID_CONSECUTIVE_INTEGER_DIGIT_SEPARATOR = -14;
/// const int32_t INVALID_CONSECUTIVE_FRACTION_DIGIT_SEPARATOR = -15;
/// const int32_t INVALID_CONSECUTIVE_EXPONENT_DIGIT_SEPARATOR = -16;
/// ```
///
/// # Safety
///
/// Assigning any value outside the range `[-16, -1]` to value of type
/// NumberFormatErrorCode may invoke undefined-behavior.
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum NumberFormatErrorCode {
    // LEXER
    /// Invalid symbol to represent exponent notation.
    InvalidExponentSymbol                    = -1,
    /// Invalid decimal point character.
    InvalidDecimalPoint                      = -2,
    /// Invalid radix for the mantissa (significant) digits.
    InvalidMantissaRadix                     = -3,
    /// Invalid base for the exponent.
    InvalidExponentBase                      = -4,
    /// Invalid radix for the exponent digits.
    InvalidExponentRadix                     = -5,
    /// Invalid character for a base prefix.
    InvalidBasePrefix                        = -6,
    /// Invalid character for a base suffix.
    InvalidBaseSuffix                        = -7,
    /// Invalid punctuation characters: multiple symbols overlap.
    InvalidPunctuation                       = -8,

    // SYNTAX
    /// Invalid digit separator character.
    InvalidDigitSeparator                    = -9,
    /// Optional exponent flags were set while disabling exponent notation.
    InvalidExponentFlags                     = -10,
    /// Set no positive mantissa sign while requiring mantissa signs.
    InvalidMantissaSign                      = -11,
    /// Set no positive exponent sign while requiring exponent signs.
    InvalidExponentSign                      = -12,
    /// Set optional special float flags while disable special floats.
    InvalidSpecial                           = -13,
    /// Invalid consecutive integer digit separator.
    InvalidConsecutiveIntegerDigitSeparator  = -14,
    /// Invalid consecutive fraction digit separator.
    InvalidConsecutiveFractionDigitSeparator = -15,
    /// Invalid consecutive exponent digit separator.
    InvalidConsecutiveExponentDigitSeparator = -16,

    // We may add additional variants later, so ensure that client matching
    // does not depend on exhaustive matching.
    #[doc(hidden)]
    __Nonexhaustive                          = -200,
}

impl Display for NumberFormatErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidExponentSymbol => "'invalid exponent symbol: must be ASCII and not a digit or a `+/-` sign'",
            Self::InvalidDecimalPoint => "'invalid decimal point: must be ASCII and not a digit or a `+/-` sign'",
            Self::InvalidMantissaRadix => "'invalid radix for mantissa digits'",
            Self::InvalidExponentBase => "'invalid exponent base'",
            Self::InvalidExponentRadix => "'invalid radix for exponent digits'",
            Self::InvalidBasePrefix => "'invalid base prefix character'",
            Self::InvalidBaseSuffix => "'invalid base suffix character'",
            Self::InvalidPunctuation => "'invalid punctuation: multiple characters overlap'",
            Self::InvalidDigitSeparator => "'invalid digit separator: must be ASCII and not a digit or a `+/-` sign'",
            Self::InvalidExponentFlags => "'exponent flags set while disabling exponent notation'",
            Self::InvalidMantissaSign => "'disabled the `+` sign while requiring a sign for significant digits'",
            Self::InvalidExponentSign => "'disabled the `+` sign while requiring a sign for exponent digits'",
            Self::InvalidSpecial => "'special flags set while disabling special floats'",
            Self::InvalidConsecutiveIntegerDigitSeparator => "'enabled consecutive digit separators in the integer without setting a valid location'",
            Self::InvalidConsecutiveFractionDigitSeparator => "'enabled consecutive digit separators in the fraction without setting a valid location'",
            Self::InvalidConsecutiveExponentDigitSeparator => "'enabled consecutive digit separators in the exponent without setting a valid location'",
            _ => unimplemented!()
        };
        write!(f, "{}", message)
    }
}

/// Error type for number format building.
///
/// This error is FFI-compatible for interfacing with C code.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct NumberFormatError {
    /// Error code designating the type of error occurred.
    pub code: NumberFormatErrorCode,
}

impl From<NumberFormatErrorCode> for NumberFormatError {
    #[inline]
    fn from(code: NumberFormatErrorCode) -> Self {
        NumberFormatError {
            code,
        }
    }
}

impl Display for NumberFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "lexical number format error: {:?}.", self.code)
    }
}

#[cfg(feature = "std")]
impl StdError for NumberFormatError {
}

// OPTIONS ERROR
// -------------

/// Error code when building the options API, indicating failure type.
///
/// Error messages are designating by an error code of less than 0.
/// This is to be compatible with C conventions. This enumeration is
/// FFI-compatible for interfacing with C code.
///
/// # FFI
///
/// For interfacing with FFI-code, this may be approximate by:
/// ```text
/// const int32_t INVALID_NAN_STRING = -1;
/// const int32_t NAN_STRING_TOO_LONG = -2;
/// const int32_t INVALID_INF_STRING = -3;
/// const int32_t INF_STRING_TOO_LONG = -4;
/// const int32_t INVALID_INFINITY_STRING = -5;
/// const int32_t INFINITY_STRING_TOO_LONG = -6;
/// const int32_t INFINITY_STRING_TOO_SHORT = -7;
/// const int32_t INVALID_FLOAT_PARSE_ALGORITHM = -8;
/// ```
///
/// # Safety
///
/// Assigning any value outside the range `[-8, -1]` to value of type
/// OptionsErrorCode may invoke undefined-behavior.
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum OptionsErrorCode {
    /// Invalid NaN string: must start with an `n` character.
    InvalidNanString           = -1,
    /// NaN string is too long.
    NanStringTooLong           = -2,
    /// Invalid short infinity string: must start with an `i` character.
    InvalidInfString           = -3,
    /// Short infinity string is too long.
    InfStringTooLong           = -4,
    /// Invalid long infinity string: must start with an `i` character.
    InvalidInfinityString      = -5,
    /// Long infinity string is too long.
    InfinityStringTooLong      = -6,
    /// Long infinity string is too short: it must be as long as short infinity.
    InfinityStringTooShort     = -7,
    /// Invalid float parsing algorithm.
    InvalidFloatParseAlgorithm = -8,

    // We may add additional variants later, so ensure that client matching
    // does not depend on exhaustive matching.
    #[doc(hidden)]
    __Nonexhaustive            = -200,
}

impl Display for OptionsErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidNanString => "'NaN string must started with `n`'",
            Self::NanStringTooLong => "'NaN string is too long'",
            Self::InvalidInfString => "'short infinity string must started with `i`'",
            Self::InfStringTooLong => "'short infinity string is too long'",
            Self::InvalidInfinityString => "'long infinity string must started with `i`'",
            Self::InfinityStringTooLong => "'long infinity string is too long'",
            Self::InfinityStringTooShort => "'long infinity string is too short'",
            Self::InvalidFloatParseAlgorithm => "'invalid combination of float parse algorithms'",
            _ => unimplemented!(),
        };
        write!(f, "{}", message)
    }
}

/// Error type for options API building.
///
/// This error is FFI-compatible for interfacing with C code.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct OptionsError {
    /// Error code designating the type of error occurred.
    pub code: OptionsErrorCode,
}

impl From<OptionsErrorCode> for OptionsError {
    #[inline]
    fn from(code: OptionsErrorCode) -> Self {
        OptionsError {
            code,
        }
    }
}

impl Display for OptionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "lexical options error: {:?}.", self.code)
    }
}

#[cfg(feature = "std")]
impl StdError for OptionsError {
}
