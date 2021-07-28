//! Error type for numeric parsing functions.
//!
//! The error type is C-compatible, simplifying use external language
//! bindings.

#![cfg(feature = "parse")]

#[cfg(feature = "std")]
use crate::lib::error;
use crate::lib::fmt;

/// ERROR
// ------

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
/// ErrorCode may invoke undefined-behavior.
#[repr(i32)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ErrorCode {
    // PARSE ERRORS
    /// Integral overflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Overflow                                 = -1,
    /// Integral underflow occurred during numeric parsing.
    ///
    /// Numeric overflow takes precedence over the presence of an invalid
    /// digit.
    Underflow                                = -2,
    /// Invalid digit found before string termination.
    InvalidDigit                             = -3,
    /// Empty byte array found.
    Empty                                    = -4,
    /// Empty mantissa found.
    EmptyMantissa                            = -5,
    /// Empty exponent found.
    EmptyExponent                            = -6,
    /// Empty integer found.
    EmptyInteger                             = -7,
    /// Empty fraction found.
    EmptyFraction                            = -8,
    /// Invalid positive mantissa sign was found.
    InvalidPositiveMantissaSign              = -9,
    /// Mantissa sign was required, but not found.
    MissingMantissaSign                      = -10,
    /// Exponent was present but not allowed.
    InvalidExponent                          = -11,
    /// Invalid positive exponent sign was found.
    InvalidPositiveExponentSign              = -12,
    /// Exponent sign was required, but not found.
    MissingExponentSign                      = -13,
    /// Exponent was present without fraction component.
    ExponentWithoutFraction                  = -14,
    /// Integer had invalid leading zeros.
    InvalidLeadingZeros                      = -15,
    /// No exponent with required exponent notation.
    MissingExponent                          = -16,
    /// Integral sign was required, but not found.
    MissingSign                              = -17,
    /// Invalid positive sign for an integer was found.
    InvalidPositiveSign                      = -18,
    /// Invalid negative sign for an unsigned type was found.
    InvalidNegativeSign                      = -19,

    // NUMBER FORMAT ERRORS
    /// Invalid radix for the mantissa (significant) digits.
    InvalidMantissaRadix                     = -100,
    /// Invalid base for the exponent.
    InvalidExponentBase                      = -101,
    /// Invalid radix for the exponent digits.
    InvalidExponentRadix                     = -102,
    /// Invalid digit separator character.
    InvalidDigitSeparator                    = -103,
    /// Invalid decimal point character.
    InvalidDecimalPoint                      = -104,
    /// Invalid symbol to represent exponent notation.
    InvalidExponentSymbol                    = -105,
    /// Invalid character for a base prefix.
    InvalidBasePrefix                        = -106,
    /// Invalid character for a base suffix.
    InvalidBaseSuffix                        = -107,
    /// Invalid punctuation characters: multiple symbols overlap.
    InvalidPunctuation                       = -108,
    /// Optional exponent flags were set while disabling exponent notation.
    InvalidExponentFlags                     = -109,
    /// Set no positive mantissa sign while requiring mantissa signs.
    InvalidMantissaSign                      = -110,
    /// Set no positive exponent sign while requiring exponent signs.
    InvalidExponentSign                      = -111,
    /// Set optional special float flags while disable special floats.
    InvalidSpecial                           = -112,
    /// Invalid consecutive integer digit separator.
    InvalidConsecutiveIntegerDigitSeparator  = -113,
    /// Invalid consecutive fraction digit separator.
    InvalidConsecutiveFractionDigitSeparator = -114,
    /// Invalid consecutive exponent digit separator.
    InvalidConsecutiveExponentDigitSeparator = -115,
    /// Invalid flags were set without the format feature.
    InvalidFlags                             = -116,

    // OPTION ERRORS
    /// Invalid NaN string: must start with an `n` character.
    InvalidNanString                         = -201,
    /// NaN string is too long.
    NanStringTooLong                         = -202,
    /// Invalid short infinity string: must start with an `i` character.
    InvalidInfString                         = -203,
    /// Short infinity string is too long.
    InfStringTooLong                         = -204,
    /// Invalid long infinity string: must start with an `i` character.
    InvalidInfinityString                    = -205,
    /// Long infinity string is too long.
    InfinityStringTooLong                    = -206,
    /// Long infinity string is too short: it must be as long as short infinity.
    InfinityStringTooShort                   = -207,
    /// Invalid float parsing algorithm.
    InvalidFloatParseAlgorithm               = -208,
    /// Invalid radix for the significant digits.
    InvalidRadix                             = -209,

    // NOT AN ERROR
    /// An error did not actually occur, and the result was successful.
    Success                                  = 0,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            // PARSE ERRORS
            Self::Overflow => "'numeric overflow occurred'",
            Self::Underflow => "'numeric underflow occurred'",
            Self::InvalidDigit => "'invalid digit found'",
            Self::Empty => "'the string to parse was empty'",
            Self::EmptyMantissa => "'no significant digits found'",
            Self::EmptyExponent => "'exponent notation found without an exponent'",
            Self::EmptyInteger => "'invalid float with no integer digits'",
            Self::EmptyFraction => "'invalid float with no fraction digits'",
            Self::InvalidPositiveMantissaSign => "'invalid `+` sign before significant digits'",
            Self::MissingMantissaSign => "'missing required `+/-` sign for significant digits'",
            Self::InvalidExponent => "'exponent found but not allowed'",
            Self::InvalidPositiveExponentSign => "'invalid `+` sign in exponent'",
            Self::MissingExponentSign => "'missing required `+/-` sign for exponent'",
            Self::ExponentWithoutFraction =>  "'invalid float containing exponent without fraction'",
            Self::InvalidLeadingZeros => "'invalid number with leading zeros before digits'",
            Self::MissingExponent => "'missing required exponent'",
            Self::MissingSign => "'missing required `+/-` sign for integer'",
            Self::InvalidPositiveSign => "'invalid `+` sign for an integer was found'",
            Self::InvalidNegativeSign => "'invalid `-` sign for an unsigned type was found'",

            // NUMBER FORMAT ERRORS
            Self::InvalidMantissaRadix => "'invalid radix for mantissa digits'",
            Self::InvalidExponentBase => "'invalid exponent base'",
            Self::InvalidExponentRadix => "'invalid radix for exponent digits'",
            Self::InvalidDigitSeparator => "'invalid digit separator: must be ASCII and not a digit or a `+/-` sign'",
            Self::InvalidDecimalPoint => "'invalid decimal point: must be ASCII and not a digit or a `+/-` sign'",
            Self::InvalidExponentSymbol => "'invalid exponent symbol: must be ASCII and not a digit or a `+/-` sign'",
            Self::InvalidBasePrefix => "'invalid base prefix character'",
            Self::InvalidBaseSuffix => "'invalid base suffix character'",
            Self::InvalidPunctuation => "'invalid punctuation: multiple characters overlap'",
            Self::InvalidExponentFlags => "'exponent flags set while disabling exponent notation'",
            Self::InvalidMantissaSign => "'disabled the `+` sign while requiring a sign for significant digits'",
            Self::InvalidExponentSign => "'disabled the `+` sign while requiring a sign for exponent digits'",
            Self::InvalidSpecial => "'special flags set while disabling special floats'",
            Self::InvalidConsecutiveIntegerDigitSeparator => "'enabled consecutive digit separators in the integer without setting a valid location'",
            Self::InvalidConsecutiveFractionDigitSeparator => "'enabled consecutive digit separators in the fraction without setting a valid location'",
            Self::InvalidConsecutiveExponentDigitSeparator => "'enabled consecutive digit separators in the exponent without setting a valid location'",
            Self::InvalidFlags => "'invalid flags enabled without the format feature'",

            // OPTION ERRORS
            Self::InvalidNanString => "'NaN string must started with `n`'",
            Self::NanStringTooLong => "'NaN string is too long'",
            Self::InvalidInfString => "'short infinity string must started with `i`'",
            Self::InfStringTooLong => "'short infinity string is too long'",
            Self::InvalidInfinityString => "'long infinity string must started with `i`'",
            Self::InfinityStringTooLong => "'long infinity string is too long'",
            Self::InfinityStringTooShort => "'long infinity string is too short'",
            Self::InvalidFloatParseAlgorithm => "'invalid combination of float parse algorithms'",
            Self::InvalidRadix => "'invalid radix for significant digits'",

            // NOT AN ERROR
            Self::Success => "'not actually an error'",
        };
        write!(formatter, "{}", message)
    }
}

/// Error type for lexical parsing.
///
/// This error is FFI-compatible for interfacing with C code.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Error {
    /// Error code designating the type of error occurred.
    pub code: ErrorCode,
    /// Optional position within the buffer for the error.
    pub index: usize,
}

impl Error {
    /// Create new error from error code.
    pub const fn new(code: ErrorCode) -> Self {
        Self {
            code,
            index: 0,
        }
    }
}

impl From<ErrorCode> for Error {
    #[inline]
    fn from(code: ErrorCode) -> Self {
        Error {
            code,
            index: 0,
        }
    }
}

impl From<(ErrorCode, usize)> for Error {
    #[inline]
    fn from(error: (ErrorCode, usize)) -> Self {
        Error {
            code: error.0,
            index: error.1,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "lexical parse error: {} at index {}.", self.code, self.index)
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
}
