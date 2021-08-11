//! Configuration options for parsing floats.

use lexical_util::ascii::{is_valid_ascii, is_valid_letter_slice};
use lexical_util::error::Error;
use lexical_util::options::{self, ParseOptions};
use lexical_util::result::Result;
use static_assertions::const_assert;

/// Maximum length for a special string.
const MAX_SPECIAL_STRING_LENGTH: usize = 50;

/// Builder for `Options`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionsBuilder {
    /// Disable the use of arbitrary-precision arithmetic, and always
    /// return the results from the fast or intermediate path algorithms.
    lossy: bool,
    /// Character to designate the exponent component of a float.
    exponent: u8,
    /// Character to separate the integer from the fraction components.
    decimal_point: u8,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: Option<&'static [u8]>,
    /// Short string representation of `Infinity`.
    inf_string: Option<&'static [u8]>,
    /// Long string representation of `Infinity`.
    infinity_string: Option<&'static [u8]>,
}

impl OptionsBuilder {
    /// Create new options builder with default options.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            lossy: false,
            exponent: b'e',
            decimal_point: b'.',
            nan_string: Some(b"NaN"),
            inf_string: Some(b"inf"),
            infinity_string: Some(b"infinity"),
        }
    }

    // GETTERS

    /// Get if we disable the use of arbitrary-precision arithmetic.
    #[inline(always)]
    pub const fn get_lossy(&self) -> bool {
        self.lossy
    }

    /// Get the character to designate the exponent component of a float.
    #[inline(always)]
    pub const fn get_exponent(&self) -> u8 {
        self.exponent
    }

    /// Get the character to separate the integer from the fraction components.
    #[inline(always)]
    pub const fn get_decimal_point(&self) -> u8 {
        self.decimal_point
    }

    /// Get the string representation for `NaN`.
    #[inline(always)]
    pub const fn get_nan_string(&self) -> Option<&'static [u8]> {
        self.nan_string
    }

    /// Get the short string representation for `Infinity`.
    #[inline(always)]
    pub const fn get_inf_string(&self) -> Option<&'static [u8]> {
        self.inf_string
    }

    /// Get the long string representation for `Infinity`.
    #[inline(always)]
    pub const fn get_infinity_string(&self) -> Option<&'static [u8]> {
        self.infinity_string
    }

    // SETTERS

    /// Set if we disable the use of arbitrary-precision arithmetic.
    #[inline(always)]
    pub const fn lossy(mut self, lossy: bool) -> Self {
        self.lossy = lossy;
        self
    }

    /// Set the character to designate the exponent component of a float.
    #[inline(always)]
    pub const fn exponent(mut self, exponent: u8) -> Self {
        self.exponent = exponent;
        self
    }

    /// Set the character to separate the integer from the fraction components.
    #[inline(always)]
    pub const fn decimal_point(mut self, decimal_point: u8) -> Self {
        self.decimal_point = decimal_point;
        self
    }

    /// Set the string representation for `NaN`.
    #[inline(always)]
    pub const fn nan_string(mut self, nan_string: Option<&'static [u8]>) -> Self {
        self.nan_string = nan_string;
        self
    }

    /// Set the short string representation for `Infinity`.
    #[inline(always)]
    pub const fn inf_string(mut self, inf_string: Option<&'static [u8]>) -> Self {
        self.inf_string = inf_string;
        self
    }

    /// Set the long string representation for `Infinity`.
    #[inline(always)]
    pub const fn infinity_string(mut self, infinity_string: Option<&'static [u8]>) -> Self {
        self.infinity_string = infinity_string;
        self
    }

    // BUILDERS

    /// Determine if `nan_str` is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn nan_str_is_valid(&self) -> bool {
        if self.nan_string.is_none() {
            return true;
        }

        let nan = unwrap_str(self.nan_string);
        let length = nan.len();
        if length == 0 || length > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(nan[0], b'N' | b'n') {
            false
        } else if !is_valid_letter_slice(nan) {
            false
        } else {
            true
        }
    }

    /// Determine if `inf_str` is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn inf_str_is_valid(&self) -> bool {
        if self.infinity_string.is_none() && self.inf_string.is_some() {
            return false;
        } else if self.inf_string.is_none() {
            return true;
        }

        let inf = unwrap_str(self.inf_string);
        let length = inf.len();
        let infinity = unwrap_str(self.infinity_string);
        if length == 0 || length > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(inf[0], b'I' | b'i') {
            false
        } else if length > infinity.len() {
            false
        } else if !is_valid_letter_slice(inf) {
            false
        } else {
            true
        }
    }

    /// Determine if `infinity_string` is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn infinity_string_is_valid(&self) -> bool {
        if self.infinity_string.is_none() && self.inf_string.is_some() {
            return false;
        } else if self.infinity_string.is_none() {
            return true;
        }
        let inf = unwrap_str(self.inf_string);
        let infinity = unwrap_str(self.infinity_string);
        let length = infinity.len();
        if length == 0 || length > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(infinity[0], b'I' | b'i') {
            false
        } else if length < inf.len() {
            false
        } else if !is_valid_letter_slice(infinity) {
            false
        } else {
            true
        }
    }

    /// Check if the builder state is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn is_valid(&self) -> bool {
        if !is_valid_ascii(self.exponent) {
            false
        } else if !is_valid_ascii(self.decimal_point) {
            false
        } else if !self.nan_str_is_valid() {
            false
        } else if !self.inf_str_is_valid() {
            false
        } else if !self.infinity_string_is_valid() {
            false
        } else {
            true
        }
    }

    /// Build the Options struct with bounds validation.
    ///
    /// # Safety
    ///
    /// Safe as long as `is_valid` is true. If `nan_string`, `inf_string`,
    /// or `infinity_string` are too long, writing special floats may lead
    /// to buffer overflows, and therefore severe security vulnerabilities.
    #[inline(always)]
    pub const unsafe fn build_unchecked(&self) -> Options {
        Options {
            lossy: self.lossy,
            exponent: self.exponent,
            decimal_point: self.decimal_point,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
            infinity_string: self.infinity_string,
        }
    }

    /// Build the Options struct.
    #[inline(always)]
    #[allow(clippy::if_same_then_else)]
    pub const fn build(&self) -> Result<Options> {
        if !is_valid_ascii(self.exponent) {
            return Err(Error::InvalidExponentSymbol);
        } else if !is_valid_ascii(self.decimal_point) {
            return Err(Error::InvalidDecimalPoint);
        }

        if self.nan_string.is_some() {
            let nan = unwrap_str(self.nan_string);
            if nan.is_empty() || !matches!(nan[0], b'N' | b'n') {
                return Err(Error::InvalidNanString);
            } else if !is_valid_letter_slice(nan) {
                return Err(Error::InvalidNanString);
            } else if nan.len() > MAX_SPECIAL_STRING_LENGTH {
                return Err(Error::NanStringTooLong);
            }
        }

        if self.inf_string.is_some() && self.infinity_string.is_none() {
            return Err(Error::InfinityStringTooShort);
        }

        if self.inf_string.is_some() {
            let inf = unwrap_str(self.inf_string);
            if inf.is_empty() || !matches!(inf[0], b'I' | b'i') {
                return Err(Error::InvalidInfString);
            } else if !is_valid_letter_slice(inf) {
                return Err(Error::InvalidInfString);
            } else if inf.len() > MAX_SPECIAL_STRING_LENGTH {
                return Err(Error::InfStringTooLong);
            }
        }

        if self.infinity_string.is_some() {
            let inf = unwrap_str(self.inf_string);
            let infinity = unwrap_str(self.infinity_string);
            if infinity.is_empty() || !matches!(infinity[0], b'I' | b'i') {
                return Err(Error::InvalidInfinityString);
            } else if !is_valid_letter_slice(infinity) {
                return Err(Error::InvalidInfinityString);
            } else if infinity.len() > MAX_SPECIAL_STRING_LENGTH {
                return Err(Error::InfinityStringTooLong);
            } else if infinity.len() < inf.len() {
                return Err(Error::InfinityStringTooShort);
            }
        }

        // SAFETY: always safe, since it must be valid.
        Ok(unsafe { self.build_unchecked() })
    }
}

impl Default for OptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Options to customize parsing floats.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::ParseFloatOptions;
///
/// # pub fn main() {
/// let options = Options::builder()
///     .lossy(true)
///     .nan_string(Some(b"NaN"))
///     .inf_string(Some(b"Inf"))
///     .infinity_string(Some(b"Infinity"))
///     .build()
///     .unwrap();
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Options {
    /// Disable the use of arbitrary-precision arithmetic, and always
    /// return the results from the fast or intermediate path algorithms.
    lossy: bool,
    /// Character to designate the exponent component of a float.
    exponent: u8,
    /// Character to separate the integer from the fraction components.
    decimal_point: u8,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: Option<&'static [u8]>,
    /// Short string representation of `Infinity`.
    inf_string: Option<&'static [u8]>,
    /// Long string representation of `Infinity`.
    infinity_string: Option<&'static [u8]>,
}

impl Options {
    // CONSTRUCTORS

    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        // SAFETY: always safe since it uses the default arguments.
        unsafe { Self::builder().build_unchecked() }
    }

    // GETTERS

    /// Check if the options state is valid.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.rebuild().is_valid()
    }

    /// Get if we disable the use of arbitrary-precision arithmetic.
    #[inline(always)]
    pub const fn lossy(&self) -> bool {
        self.lossy
    }

    /// Get the character to designate the exponent component of a float.
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        self.exponent
    }

    /// Get the character to separate the integer from the fraction components.
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        self.decimal_point
    }

    /// Get the string representation for `NaN`.
    #[inline(always)]
    pub const fn nan_string(&self) -> Option<&'static [u8]> {
        self.nan_string
    }

    /// Get the short string representation for `Infinity`.
    #[inline(always)]
    pub const fn inf_string(&self) -> Option<&'static [u8]> {
        self.inf_string
    }

    /// Get the long string representation for `Infinity`.
    #[inline(always)]
    pub const fn infinity_string(&self) -> Option<&'static [u8]> {
        self.infinity_string
    }

    // SETTERS

    /// Set if we disable the use of arbitrary-precision arithmetic.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_lossy(&mut self, lossy: bool) {
        self.lossy = lossy
    }

    /// Set the character to designate the exponent component of a float.
    ///
    /// # Safety
    ///
    /// Never unsafe, but may produce invalid output if the exponent
    /// is not a valid ASCII character.
    #[inline(always)]
    pub unsafe fn set_exponent(&mut self, exponent: u8) {
        self.exponent = exponent;
    }

    /// Set the character to separate the integer from the fraction components.
    ///
    /// # Safety
    ///
    /// Never unsafe, but may produce invalid output if the decimal point
    /// is not a valid ASCII character.
    #[inline(always)]
    pub unsafe fn set_decimal_point(&mut self, decimal_point: u8) {
        self.decimal_point = decimal_point;
    }

    /// Set the string representation for `NaN`.
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Unsafe if `nan_string.len() > MAX_SPECIAL_STRING_LENGTH`.
    #[inline(always)]
    pub unsafe fn set_nan_string(&mut self, nan_string: Option<&'static [u8]>) {
        self.nan_string = nan_string
    }

    /// Set the short string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Unsafe if `inf_string.len() > MAX_SPECIAL_STRING_LENGTH`.
    #[inline(always)]
    pub unsafe fn set_inf_string(&mut self, inf_string: Option<&'static [u8]>) {
        self.inf_string = inf_string
    }

    /// Set the long string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Unsafe if `infinity_string.len() > MAX_SPECIAL_STRING_LENGTH`.
    #[inline(always)]
    pub unsafe fn set_infinity_string(&mut self, infinity_string: Option<&'static [u8]>) {
        self.infinity_string = infinity_string
    }

    // BUILDERS

    /// Get OptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> OptionsBuilder {
        OptionsBuilder::new()
    }

    /// Create OptionsBuilder using existing values.
    #[inline(always)]
    pub const fn rebuild(&self) -> OptionsBuilder {
        OptionsBuilder {
            lossy: self.lossy,
            exponent: self.exponent,
            decimal_point: self.decimal_point,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
            infinity_string: self.infinity_string,
        }
    }
}

impl Default for Options {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl ParseOptions for Options {
    #[inline(always)]
    fn is_valid(&self) -> bool {
        Self::is_valid(self)
    }
}

/// Unwrap `Option` as a const fn.
#[inline(always)]
const fn unwrap_str(option: Option<&'static [u8]>) -> &'static [u8] {
    match option {
        Some(x) => x,
        None => &[],
    }
}

// PRE-DEFINED CONSTANTS
// ---------------------

// Only constants that differ from the standard version are included.

/// Standard number format.
#[rustfmt::skip]
pub const STANDARD: Options = Options::new();
const_assert!(STANDARD.is_valid());

/// Numerical format with a decimal comma.
/// This is the standard numerical format for most of the world.
#[rustfmt::skip]
pub const DECIMAL_COMMA: Options = unsafe {
    Options::builder()
        .decimal_point(b',')
        .build_unchecked()
};
const_assert!(DECIMAL_COMMA.is_valid());

/// Numerical format for hexadecimal floats, which use a `p` exponent.
#[rustfmt::skip]
pub const HEX_FLOAT: Options = unsafe {
    Options::builder()
        .exponent(b'p')
        .build_unchecked()
};
const_assert!(HEX_FLOAT.is_valid());

/// Numerical format where `^` is used as the exponent notation character.
/// This isn't very common, but is useful when `e` or `p` are valid digits.
#[rustfmt::skip]
pub const CARAT_EXPONENT: Options = unsafe {
    Options::builder()
        .exponent(b'^')
        .build_unchecked()
};
const_assert!(CARAT_EXPONENT.is_valid());

// TODO(ahuszagh) Add a lot more...

/// Number format for a Javascript literal floating-point number.
#[rustfmt::skip]
pub const JAVASCRIPT_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::JAVASCRIPT_INF)
        .infinity_string(options::JAVASCRIPT_INFINITY)
        .build_unchecked()
};
const_assert!(JAVASCRIPT_LITERAL.is_valid());

/// Number format to parse a Javascript float from string.
#[rustfmt::skip]
pub const JAVASCRIPT_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::JAVASCRIPT_INF)
        .infinity_string(options::JAVASCRIPT_INFINITY)
        .build_unchecked()
};
const_assert!(JAVASCRIPT_STRING.is_valid());

// TODO(ahuszagh) Add a few languages...
