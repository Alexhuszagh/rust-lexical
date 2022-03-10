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

    /// Build the Options struct without validation.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    /// The result may be invalid if `is_valid` is not true.
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
/// use lexical_parse_float::Options;
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

    /// Create the default options for a given radix.
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn from_radix(radix: u8) -> Self {
        // Need to determine the correct exponent character ('e' or '^'),
        // since the default character is `e` normally, but this is a valid
        // digit for radix >= 15.
        let mut builder = Self::builder();
        if radix >= 15 {
            builder = builder.exponent(b'^');
        }
        // SAFETY: always safe since it uses the default arguments.
        unsafe { builder.build_unchecked() }
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
    /// Always safe, but may produce invalid output if the exponent
    /// is not a valid ASCII character.
    #[inline(always)]
    pub unsafe fn set_exponent(&mut self, exponent: u8) {
        self.exponent = exponent;
    }

    /// Set the character to separate the integer from the fraction components.
    ///
    /// # Safety
    ///
    /// Always safe, but may produce invalid output if the decimal point
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
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_nan_string(&mut self, nan_string: Option<&'static [u8]>) {
        self.nan_string = nan_string
    }

    /// Set the short string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_inf_string(&mut self, inf_string: Option<&'static [u8]>) {
        self.inf_string = inf_string
    }

    /// Set the long string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
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
// SAFETY: all of these are safe, since they are checked to be valid
// after calling `build_unchecked`. Furthermore, even though the methods
// are marked as `unsafe`, none of the produced options can cause memory
// safety issues.

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

/// Number format for a Rust literal floating-point number.
#[rustfmt::skip]
pub const RUST_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::RUST_LITERAL)
        .inf_string(options::RUST_LITERAL)
        .infinity_string(options::RUST_LITERAL)
        .build_unchecked()
};
const_assert!(RUST_LITERAL.is_valid());

/// Number format for a Python literal floating-point number.
#[rustfmt::skip]
pub const PYTHON_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::PYTHON_LITERAL)
        .inf_string(options::PYTHON_LITERAL)
        .infinity_string(options::PYTHON_LITERAL)
        .build_unchecked()
};
const_assert!(PYTHON_LITERAL.is_valid());

/// Number format for a C++ literal floating-point number.
#[rustfmt::skip]
pub const CXX_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::CXX_LITERAL_NAN)
        .inf_string(options::CXX_LITERAL_INF)
        .infinity_string(options::CXX_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(CXX_LITERAL.is_valid());

/// Number format for a C literal floating-point number.
#[rustfmt::skip]
pub const C_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::C_LITERAL_NAN)
        .inf_string(options::C_LITERAL_INF)
        .infinity_string(options::C_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(CXX_LITERAL.is_valid());

/// Number format for a Ruby literal floating-point number.
#[rustfmt::skip]
pub const RUBY_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::RUBY)
        .inf_string(options::RUBY)
        .infinity_string(options::RUBY)
        .build_unchecked()
};
const_assert!(RUBY_LITERAL.is_valid());

/// Number format to parse a Ruby float from string.
#[rustfmt::skip]
pub const RUBY_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::RUBY)
        .inf_string(options::RUBY)
        .infinity_string(options::RUBY)
        .build_unchecked()
};
const_assert!(RUBY_STRING.is_valid());

/// Number format for a Swift literal floating-point number.
#[rustfmt::skip]
pub const SWIFT_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::SWIFT_LITERAL)
        .inf_string(options::SWIFT_LITERAL)
        .infinity_string(options::SWIFT_LITERAL)
        .build_unchecked()
};
const_assert!(SWIFT_LITERAL.is_valid());

/// Number format for a Go literal floating-point number.
#[rustfmt::skip]
pub const GO_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::GO_LITERAL)
        .inf_string(options::GO_LITERAL)
        .infinity_string(options::GO_LITERAL)
        .build_unchecked()
};
const_assert!(GO_LITERAL.is_valid());

/// Number format for a Haskell literal floating-point number.
#[rustfmt::skip]
pub const HASKELL_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::HASKELL_LITERAL)
        .inf_string(options::HASKELL_LITERAL)
        .infinity_string(options::HASKELL_LITERAL)
        .build_unchecked()
};
const_assert!(HASKELL_LITERAL.is_valid());

/// Number format to parse a Haskell float from string.
#[rustfmt::skip]
pub const HASKELL_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::HASKELL_STRING_INF)
        .infinity_string(options::HASKELL_STRING_INFINITY)
        .build_unchecked()
};
const_assert!(HASKELL_STRING.is_valid());

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

/// Number format for a Perl literal floating-point number.
#[rustfmt::skip]
pub const PERL_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::PERL_LITERAL)
        .inf_string(options::PERL_LITERAL)
        .infinity_string(options::PERL_LITERAL)
        .build_unchecked()
};
const_assert!(PERL_LITERAL.is_valid());

/// Number format for a PHP literal floating-point number.
#[rustfmt::skip]
pub const PHP_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::PHP_LITERAL_NAN)
        .inf_string(options::PHP_LITERAL_INF)
        .infinity_string(options::PHP_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(PHP_LITERAL.is_valid());

/// Number format for a Java literal floating-point number.
#[rustfmt::skip]
pub const JAVA_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::JAVA_LITERAL)
        .inf_string(options::JAVA_LITERAL)
        .infinity_string(options::JAVA_LITERAL)
        .build_unchecked()
};
const_assert!(JAVA_LITERAL.is_valid());

/// Number format to parse a Java float from string.
#[rustfmt::skip]
pub const JAVA_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::JAVA_STRING_INF)
        .infinity_string(options::JAVA_STRING_INFINITY)
        .build_unchecked()
};
const_assert!(JAVA_STRING.is_valid());

/// Number format for an R literal floating-point number.
#[rustfmt::skip]
pub const R_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::R_LITERAL_INF)
        .infinity_string(options::R_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(R_LITERAL.is_valid());

/// Number format for a Kotlin literal floating-point number.
#[rustfmt::skip]
pub const KOTLIN_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::KOTLIN_LITERAL)
        .inf_string(options::KOTLIN_LITERAL)
        .infinity_string(options::KOTLIN_LITERAL)
        .build_unchecked()
};
const_assert!(KOTLIN_LITERAL.is_valid());

/// Number format to parse a Kotlin float from string.
#[rustfmt::skip]
pub const KOTLIN_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::KOTLIN_STRING_INF)
        .infinity_string(options::KOTLIN_STRING_INFINITY)
        .build_unchecked()
};
const_assert!(KOTLIN_STRING.is_valid());

/// Number format for a Julia literal floating-point number.
#[rustfmt::skip]
pub const JULIA_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::JULIA_LITERAL_INF)
        .infinity_string(options::JULIA_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(JULIA_LITERAL.is_valid());

/// Number format for a C# literal floating-point number.
#[rustfmt::skip]
pub const CSHARP_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::CSHARP_LITERAL)
        .inf_string(options::CSHARP_LITERAL)
        .infinity_string(options::CSHARP_LITERAL)
        .build_unchecked()
};
const_assert!(CSHARP_LITERAL.is_valid());

/// Number format to parse a C# float from string.
#[rustfmt::skip]
pub const CSHARP_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::CSHARP_STRING_INF)
        .infinity_string(options::CSHARP_STRING_INFINITY)
        .build_unchecked()
};
const_assert!(CSHARP_STRING.is_valid());

/// Number format for a Kawa literal floating-point number.
#[rustfmt::skip]
pub const KAWA_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::KAWA)
        .inf_string(options::KAWA)
        .infinity_string(options::KAWA)
        .build_unchecked()
};
const_assert!(KAWA_LITERAL.is_valid());

/// Number format to parse a Kawa float from string.
#[rustfmt::skip]
pub const KAWA_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::KAWA)
        .inf_string(options::KAWA)
        .infinity_string(options::KAWA)
        .build_unchecked()
};
const_assert!(KAWA_STRING.is_valid());

/// Number format for a Gambit-C literal floating-point number.
#[rustfmt::skip]
pub const GAMBITC_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::GAMBITC)
        .inf_string(options::GAMBITC)
        .infinity_string(options::GAMBITC)
        .build_unchecked()
};
const_assert!(GAMBITC_LITERAL.is_valid());

/// Number format to parse a Gambit-C float from string.
#[rustfmt::skip]
pub const GAMBITC_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::GAMBITC)
        .inf_string(options::GAMBITC)
        .infinity_string(options::GAMBITC)
        .build_unchecked()
};
const_assert!(GAMBITC_STRING.is_valid());

/// Number format for a Guile literal floating-point number.
#[rustfmt::skip]
pub const GUILE_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::GUILE)
        .inf_string(options::GUILE)
        .infinity_string(options::GUILE)
        .build_unchecked()
};
const_assert!(GUILE_LITERAL.is_valid());

/// Number format to parse a Guile float from string.
#[rustfmt::skip]
pub const GUILE_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::GUILE)
        .inf_string(options::GUILE)
        .infinity_string(options::GUILE)
        .build_unchecked()
};
const_assert!(GUILE_STRING.is_valid());

/// Number format for a Clojure literal floating-point number.
#[rustfmt::skip]
pub const CLOJURE_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::CLOJURE_LITERAL)
        .inf_string(options::CLOJURE_LITERAL)
        .infinity_string(options::CLOJURE_LITERAL)
        .build_unchecked()
};
const_assert!(CLOJURE_LITERAL.is_valid());

/// Number format to parse a Clojure float from string.
#[rustfmt::skip]
pub const CLOJURE_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::CLOJURE_STRING_INF)
        .infinity_string(options::CLOJURE_STRING_INFINITY)
        .build_unchecked()
};
const_assert!(CLOJURE_STRING.is_valid());

/// Number format for an Erlang literal floating-point number.
#[rustfmt::skip]
pub const ERLANG_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::ERLANG_LITERAL_NAN)
        .build_unchecked()
};
const_assert!(ERLANG_LITERAL.is_valid());

/// Number format to parse an Erlang float from string.
#[rustfmt::skip]
pub const ERLANG_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::ERLANG_STRING)
        .inf_string(options::ERLANG_STRING)
        .infinity_string(options::ERLANG_STRING)
        .build_unchecked()
};
const_assert!(ERLANG_STRING.is_valid());

/// Number format for an Elm literal floating-point number.
#[rustfmt::skip]
pub const ELM_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::ELM_LITERAL)
        .inf_string(options::ELM_LITERAL)
        .infinity_string(options::ELM_LITERAL)
        .build_unchecked()
};
const_assert!(ELM_LITERAL.is_valid());

/// Number format to parse an Elm float from string.
#[rustfmt::skip]
pub const ELM_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::ELM_STRING_NAN)
        .inf_string(options::ELM_STRING_INF)
        .infinity_string(options::ELM_STRING_INFINITY)
        .build_unchecked()
};
const_assert!(ELM_STRING.is_valid());

/// Number format for a Scala literal floating-point number.
#[rustfmt::skip]
pub const SCALA_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::SCALA_LITERAL)
        .inf_string(options::SCALA_LITERAL)
        .infinity_string(options::SCALA_LITERAL)
        .build_unchecked()
};
const_assert!(SCALA_LITERAL.is_valid());

/// Number format to parse a Scala float from string.
#[rustfmt::skip]
pub const SCALA_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::SCALA_STRING_INF)
        .infinity_string(options::SCALA_STRING_INFINITY)
        .build_unchecked()
};
const_assert!(SCALA_STRING.is_valid());

/// Number format for an Elixir literal floating-point number.
#[rustfmt::skip]
pub const ELIXIR_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::ELIXIR)
        .inf_string(options::ELIXIR)
        .infinity_string(options::ELIXIR)
        .build_unchecked()
};
const_assert!(ELIXIR_LITERAL.is_valid());

/// Number format to parse an Elixir float from string.
#[rustfmt::skip]
pub const ELIXIR_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::ELIXIR)
        .inf_string(options::ELIXIR)
        .infinity_string(options::ELIXIR)
        .build_unchecked()
};
const_assert!(ELIXIR_STRING.is_valid());

/// Number format for a FORTRAN literal floating-point number.
#[rustfmt::skip]
pub const FORTRAN_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::FORTRAN_LITERAL)
        .inf_string(options::FORTRAN_LITERAL)
        .infinity_string(options::FORTRAN_LITERAL)
        .build_unchecked()
};
const_assert!(FORTRAN_LITERAL.is_valid());

/// Number format for a D literal floating-point number.
#[rustfmt::skip]
pub const D_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::D_LITERAL)
        .inf_string(options::D_LITERAL)
        .infinity_string(options::D_LITERAL)
        .build_unchecked()
};
const_assert!(D_LITERAL.is_valid());

/// Number format for a Coffeescript literal floating-point number.
#[rustfmt::skip]
pub const COFFEESCRIPT_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::COFFEESCRIPT_INF)
        .infinity_string(options::COFFEESCRIPT_INFINITY)
        .build_unchecked()
};
const_assert!(COFFEESCRIPT_LITERAL.is_valid());

/// Number format to parse a Coffeescript float from string.
#[rustfmt::skip]
pub const COFFEESCRIPT_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::COFFEESCRIPT_INF)
        .infinity_string(options::COFFEESCRIPT_INFINITY)
        .build_unchecked()
};
const_assert!(COFFEESCRIPT_STRING.is_valid());

/// Number format for a COBOL literal floating-point number.
#[rustfmt::skip]
pub const COBOL_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::COBOL)
        .inf_string(options::COBOL)
        .infinity_string(options::COBOL)
        .build_unchecked()
};
const_assert!(COBOL_LITERAL.is_valid());

/// Number format to parse a COBOL float from string.
#[rustfmt::skip]
pub const COBOL_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::COBOL)
        .inf_string(options::COBOL)
        .infinity_string(options::COBOL)
        .build_unchecked()
};
const_assert!(COBOL_STRING.is_valid());

/// Number format for an F# literal floating-point number.
#[rustfmt::skip]
pub const FSHARP_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::FSHARP_LITERAL_NAN)
        .inf_string(options::FSHARP_LITERAL_INF)
        .infinity_string(options::FSHARP_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(FSHARP_LITERAL.is_valid());

/// Number format for a Visual Basic literal floating-point number.
#[rustfmt::skip]
pub const VB_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::VB_LITERAL)
        .inf_string(options::VB_LITERAL)
        .infinity_string(options::VB_LITERAL)
        .build_unchecked()
};
const_assert!(VB_LITERAL.is_valid());

/// Number format to parse a Visual Basic float from string.
#[rustfmt::skip]
pub const VB_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::VB_STRING_INF)
        .infinity_string(options::VB_STRING_INFINITY)
        .build_unchecked()
};
const_assert!(VB_STRING.is_valid());

/// Number format for an OCaml literal floating-point number.
#[rustfmt::skip]
pub const OCAML_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::OCAML_LITERAL_NAN)
        .inf_string(options::OCAML_LITERAL_INF)
        .infinity_string(options::OCAML_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(OCAML_LITERAL.is_valid());

/// Number format for an Objective-C literal floating-point number.
#[rustfmt::skip]
pub const OBJECTIVEC_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::OBJECTIVEC)
        .inf_string(options::OBJECTIVEC)
        .infinity_string(options::OBJECTIVEC)
        .build_unchecked()
};
const_assert!(OBJECTIVEC_LITERAL.is_valid());

/// Number format to parse an Objective-C float from string.
#[rustfmt::skip]
pub const OBJECTIVEC_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::OBJECTIVEC)
        .inf_string(options::OBJECTIVEC)
        .infinity_string(options::OBJECTIVEC)
        .build_unchecked()
};
const_assert!(OBJECTIVEC_STRING.is_valid());

/// Number format for an ReasonML literal floating-point number.
#[rustfmt::skip]
pub const REASONML_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::REASONML_LITERAL_NAN)
        .inf_string(options::REASONML_LITERAL_INF)
        .infinity_string(options::REASONML_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(REASONML_LITERAL.is_valid());

/// Number format for a MATLAB literal floating-point number.
#[rustfmt::skip]
pub const MATLAB_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::MATLAB_LITERAL_INF)
        .infinity_string(options::MATLAB_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(MATLAB_LITERAL.is_valid());

/// Number format for a Zig literal floating-point number.
#[rustfmt::skip]
pub const ZIG_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::ZIG_LITERAL)
        .inf_string(options::ZIG_LITERAL)
        .infinity_string(options::ZIG_LITERAL)
        .build_unchecked()
};
const_assert!(ZIG_LITERAL.is_valid());

/// Number format for a Safe literal floating-point number.
#[rustfmt::skip]
pub const SAGE_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::SAGE_LITERAL_INF)
        .infinity_string(options::SAGE_LITERAL_INFINITY)
        .build_unchecked()
};
const_assert!(SAGE_LITERAL.is_valid());

/// Number format for a JSON literal floating-point number.
#[rustfmt::skip]
pub const JSON: Options = unsafe {
    Options::builder()
        .nan_string(options::JSON)
        .inf_string(options::JSON)
        .infinity_string(options::JSON)
        .build_unchecked()
};
const_assert!(JSON.is_valid());

/// Number format for a TOML literal floating-point number.
#[rustfmt::skip]
pub const TOML: Options = unsafe {
    Options::builder()
        .nan_string(options::TOML)
        .inf_string(options::TOML)
        .infinity_string(options::TOML)
        .build_unchecked()
};
const_assert!(TOML.is_valid());

/// Number format for a YAML literal floating-point number.
#[rustfmt::skip]
pub const YAML: Options = JSON;

/// Number format for an XML literal floating-point number.
#[rustfmt::skip]
pub const XML: Options = unsafe {
    Options::builder()
        .inf_string(options::XML_INF)
        .infinity_string(options::XML_INFINITY)
        .build_unchecked()
};
const_assert!(XML.is_valid());

/// Number format for a SQLite literal floating-point number.
#[rustfmt::skip]
pub const SQLITE: Options = unsafe {
    Options::builder()
        .nan_string(options::SQLITE)
        .inf_string(options::SQLITE)
        .infinity_string(options::SQLITE)
        .build_unchecked()
};
const_assert!(SQLITE.is_valid());

/// Number format for a PostgreSQL literal floating-point number.
#[rustfmt::skip]
pub const POSTGRESQL: Options = unsafe {
    Options::builder()
        .nan_string(options::POSTGRESQL)
        .inf_string(options::POSTGRESQL)
        .infinity_string(options::POSTGRESQL)
        .build_unchecked()
};
const_assert!(POSTGRESQL.is_valid());

/// Number format for a MySQL literal floating-point number.
#[rustfmt::skip]
pub const MYSQL: Options = unsafe {
    Options::builder()
        .nan_string(options::MYSQL)
        .inf_string(options::MYSQL)
        .infinity_string(options::MYSQL)
        .build_unchecked()
};
const_assert!(MYSQL.is_valid());

/// Number format for a MongoDB literal floating-point number.
#[rustfmt::skip]
pub const MONGODB: Options = unsafe {
    Options::builder()
        .inf_string(options::MONGODB_INF)
        .infinity_string(options::MONGODB_INFINITY)
        .build_unchecked()
};
const_assert!(MONGODB.is_valid());
