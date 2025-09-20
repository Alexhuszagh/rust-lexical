//! Configuration options for writing floats.
//!
//! This enables extensive control over how the float is written, from
//! control characters like the decimal point, to the use of exponent
//! notation, and the number of significant digits.
//!
//! # Examples
//!
//! For example, to customize the writing of numbers for new exponent
//! and decimal point characters, you would use [`Options`] to create
//! a custom format and write the integer using the format.
//!
//! ```rust
//! # use core::{num, str};
//! use lexical_write_float::{FormattedSize, Options, ToLexicalWithOptions};
//! use lexical_write_float::format::STANDARD;
//!
//! let value = 1.234e45f64;
//!
//! const CUSTOM: Options = Options::builder()
//!     // write exponents as "1.2^10" and not "1.2e10"
//!     .exponent(b'^')
//!     // use the European decimal point, so "1,2" and not "1.2"
//!     .decimal_point(b',')
//!     .build_strict();
//!
//! const BUFFER_SIZE: usize = CUSTOM.buffer_size_const::<f64, STANDARD>();
//! let mut buffer = [0u8; BUFFER_SIZE];
//! let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &CUSTOM);
//! assert_eq!(str::from_utf8(digits), Ok("1,234^45"));
//! ```
//!
//! # Pre-Defined Formats
//!
//! These are the pre-defined formats for parsing numbers from various
//! programming, markup, and data languages.
//!
//! - [`STANDARD`]: Standard number format.
//! - [`DECIMAL_COMMA`]: Numerical format with a decimal comma.
//! - [`HEX_FLOAT`]: Numerical format for hexadecimal floats, which use a `p`
//!   exponent.
//! - [`CARAT_EXPONENT`]: Numerical format where `^` is used as the exponent
//!   notation character.
//! - [`RUST_LITERAL`]: Number format for a [`Rust`] literal floating-point
//!   number.
//! - [`PYTHON_LITERAL`]: Number format for a [`Python`] literal floating-point
//!   number.
//! - [`CXX_LITERAL`]: Number format for a [`C++`] literal floating-point
//!   number.
//! - [`C_LITERAL`]: Number format for a [`C`] literal floating-point number.
//! - [`RUBY_LITERAL`]: Number format for a [`Ruby`] literal floating-point
//!   number.
//! - [`RUBY_STRING`]: Number format to parse a [`Ruby`] float from string.
//! - [`SWIFT_LITERAL`]: Number format for a [`Swift`] literal floating-point
//!   number.
//! - [`GO_LITERAL`]: Number format for a [`Golang`] literal floating-point
//!   number.
//! - [`HASKELL_LITERAL`]: Number format for a [`Haskell`] literal
//!   floating-point number.
//! - [`HASKELL_STRING`]: Number format to parse a [`Haskell`] float from
//!   string.
//! - [`JAVASCRIPT_LITERAL`]: Number format for a [`Javascript`] literal
//!   floating-point number.
//! - [`JAVASCRIPT_STRING`]: Number format to parse a [`Javascript`] float from
//!   string.
//! - [`PERL_LITERAL`]: Number format for a [`Perl`] literal floating-point
//!   number.
//! - [`PHP_LITERAL`]: Number format for a [`PHP`] literal floating-point
//!   number.
//! - [`JAVA_LITERAL`]: Number format for a [`Java`] literal floating-point
//!   number.
//! - [`JAVA_STRING`]: Number format to parse a [`Java`] float from string.
//! - [`R_LITERAL`]: Number format for an [`R`] literal floating-point number.
//! - [`KOTLIN_LITERAL`]: Number format for a [`Kotlin`] literal floating-point
//!   number.
//! - [`KOTLIN_STRING`]: Number format to parse a [`Kotlin`] float from string.
//! - [`JULIA_LITERAL`]: Number format for a [`Julia`] literal floating-point
//!   number.
//! - [`CSHARP_LITERAL`]: Number format for a [`C#`] literal floating-point
//!   number.
//! - [`CSHARP_STRING`]: Number format to parse a [`C#`] float from string.
//! - [`KAWA_LITERAL`]: Number format for a [`Kawa`] literal floating-point
//!   number.
//! - [`KAWA_STRING`]: Number format to parse a [`Kawa`] float from string.
//! - [`GAMBITC_LITERAL`]: Number format for a [`Gambit-C`] literal
//!   floating-point number.
//! - [`GAMBITC_STRING`]: Number format to parse a [`Gambit-C`] float from
//!   string.
//! - [`GUILE_LITERAL`]: Number format for a [`Guile`] literal floating-point
//!   number.
//! - [`GUILE_STRING`]: Number format to parse a [`Guile`] float from string.
//! - [`CLOJURE_LITERAL`]: Number format for a [`Clojure`] literal
//!   floating-point number.
//! - [`CLOJURE_STRING`]: Number format to parse a [`Clojure`] float from
//!   string.
//! - [`ERLANG_LITERAL`]: Number format for an [`Erlang`] literal floating-point
//!   number.
//! - [`ERLANG_STRING`]: Number format to parse an [`Erlang`] float from string.
//! - [`ELM_LITERAL`]: Number format for an [`Elm`] literal floating-point
//!   number.
//! - [`ELM_STRING`]: Number format to parse an [`Elm`] float from string.
//! - [`SCALA_LITERAL`]: Number format for a [`Scala`] literal floating-point
//!   number.
//! - [`SCALA_STRING`]: Number format to parse a [`Scala`] float from string.
//! - [`ELIXIR_LITERAL`]: Number format for an [`Elixir`] literal floating-point
//!   number.
//! - [`ELIXIR_STRING`]: Number format to parse an [`Elixir`] float from string.
//! - [`FORTRAN_LITERAL`]: Number format for a [`FORTRAN`] literal
//!   floating-point number.
//! - [`D_LITERAL`]: Number format for a [`D`] literal floating-point number.
//! - [`COFFEESCRIPT_LITERAL`]: Number format for a [`Coffeescript`] literal
//!   floating-point number.
//! - [`COFFEESCRIPT_STRING`]: Number format to parse a [`Coffeescript`] float
//!   from string.
//! - [`COBOL_LITERAL`]: Number format for a [`COBOL`] literal floating-point
//!   number.
//! - [`COBOL_STRING`]: Number format to parse a [`COBOL`] float from string.
//! - [`FSHARP_LITERAL`]: Number format for an [`F#`] literal floating-point
//!   number.
//! - [`VB_LITERAL`]: Number format for a [`Visual Basic`] literal
//!   floating-point number.
//! - [`VB_STRING`]: Number format to parse a [`Visual Basic`] float from
//!   string.
//! - [`OCAML_LITERAL`]: Number format for an [`OCaml`] literal floating-point
//!   number.
//! - [`OBJECTIVEC_LITERAL`]: Number format for an [`Objective-C`] literal
//!   floating-point number.
//! - [`OBJECTIVEC_STRING`]: Number format to parse an [`Objective-C`] float
//!   from string.
//! - [`REASONML_LITERAL`]: Number format for an [`ReasonML`] literal
//!   floating-point number.
//! - [`MATLAB_LITERAL`]: Number format for a [`MATLAB`] literal floating-point
//!   number.
//! - [`ZIG_LITERAL`]: Number format for a [`Zig`] literal floating-point
//!   number.
//! - [`SAGE_LITERAL`]: Number format for a [`Sage`] literal floating-point
//!   number.
//! - [`JSON`]: Number format for a [`JSON`][`JSON-REF`] literal floating-point
//!   number.
//! - [`TOML`]: Number format for a [`TOML`][`TOML-REF`] literal floating-point
//!   number.
//! - [`YAML`]: Number format for a [`YAML`][`YAML-REF`] literal floating-point
//!   number.
//! - [`XML`]: Number format for an [`XML`][`XML-REF`] literal floating-point
//!   number.
//! - [`SQLITE`]: Number format for a [`SQLite`] literal floating-point number.
//! - [`POSTGRESQL`]: Number format for a [`PostgreSQL`] literal floating-point
//!   number.
//! - [`MYSQL`]: Number format for a [`MySQL`] literal floating-point number.
//! - [`MONGODB`]: Number format for a [`MongoDB`] literal floating-point
//!   number.
//!
//! <!-- References -->
//!
//! [`Rust`]: https://www.rust-lang.org/
//! [`Python`]: https://www.python.org/
//! [`C++`]: https://en.cppreference.com/w/
//! [`C`]: https://en.cppreference.com/w/c
//! [`Ruby`]: https://www.ruby-lang.org/en/
//! [`Swift`]: https://developer.apple.com/swift/
//! [`Golang`]: https://go.dev/
//! [`Haskell`]: https://www.haskell.org/
//! [`Javascript`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
//! [`Perl`]: https://www.perl.org/
//! [`PHP`]: https://www.php.net/
//! [`Java`]: https://www.java.com/en/
//! [`R`]: https://www.r-project.org/
//! [`Kotlin`]: https://kotlinlang.org/
//! [`Julia`]: https://julialang.org/
//! [`C#`]: https://learn.microsoft.com/en-us/dotnet/csharp/
//! [`Kawa`]: https://www.gnu.org/software/kawa/
//! [`Gambit-C`]: https://gambitscheme.org/
//! [`Guile`]: https://www.gnu.org/software/guile/
//! [`Clojure`]: https://clojure.org/
//! [`Erlang`]: https://www.erlang.org/
//! [`Elm`]: https://elm-lang.org/
//! [`Scala`]: https://www.scala-lang.org/
//! [`Elixir`]: https://elixir-lang.org/
//! [`FORTRAN`]: https://fortran-lang.org/
//! [`D`]: https://dlang.org/
//! [`Coffeescript`]: https://coffeescript.org/
//! [`Cobol`]: https://www.ibm.com/think/topics/cobol
//! [`F#`]: https://fsharp.org/
//! [`Visual Basic`]: https://learn.microsoft.com/en-us/dotnet/visual-basic/
//! [`OCaml`]: https://ocaml.org/
//! [`Objective-C`]: https://en.wikipedia.org/wiki/Objective-C
//! [`ReasonML`]: https://reasonml.github.io/
//! [`Matlab`]: https://www.mathworks.com/products/matlab.html
//! [`Zig`]: https://ziglang.org/
//! [`Sage`]: https://www.sagemath.org/
//! [`JSON-REF`]: https://www.json.org/json-en.html
//! [`TOML-REF`]: https://toml.io/en/
//! [`YAML-REF`]: https://yaml.org/
//! [`XML-REF`]: https://en.wikipedia.org/wiki/XML
//! [`SQLite`]: https://www.sqlite.org/
//! [`PostgreSQL`]: https://www.postgresql.org/
//! [`MySQL`]: https://www.mysql.com/
//! [`MongoDB`]: https://www.mongodb.com/

use core::num;

use lexical_util::ascii::{is_valid_ascii, is_valid_letter_slice};
use lexical_util::constants::FormattedSize;
use lexical_util::error::Error;
use lexical_util::format::NumberFormat;
use lexical_util::options::{self, WriteOptions};
use lexical_util::result::Result;

// NOTE: Rust guarantees the sizes are the same:
//  https://doc.rust-lang.org/std/num/struct.NonZero.html

/// Type with the exact same size as a `usize`.
#[doc(hidden)]
pub type OptionUsize = Option<num::NonZeroUsize>;

/// Type with the exact same size as a `i32`.
#[doc(hidden)]
pub type OptionI32 = Option<num::NonZeroI32>;

/// Const evaluation of `max` for integers.
macro_rules! max {
    ($x:expr, $y:expr) => {{
        let x = $x;
        let y = $y;
        if x >= y {
            x
        } else {
            y
        }
    }};
}

/// Const evaluation of `min` for integers.
macro_rules! min {
    ($x:expr, $y:expr) => {{
        let x = $x;
        let y = $y;
        if x <= y {
            x
        } else {
            y
        }
    }};
}

/// Enumeration for how to round floats with precision control.
///
/// For example, using [`Round`][RoundMode::Round], `1.2345` rounded
/// to 4 digits would be `1.235`, while [`Truncate`][RoundMode::Truncate]
/// would be `1.234`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RoundMode {
    /// Round to the nearest float string with the given number of significant
    /// digits.
    Round,

    /// Truncate the float string with the given number of significant digits.
    Truncate,
}

/// Maximum length for a special string.
pub const MAX_SPECIAL_STRING_LENGTH: usize = 50;

/// Builder for [`Options`].
///
/// This enables extensive control over how the float is written, from
/// control characters like the decimal point, to the use of exponent
/// notation, and the number of significant digits.
///
/// # Examples
///
/// For example, to customize the writing of numbers for new exponent
/// and decimal point characters, you would use:
///
/// ```rust
/// # use core::{num, str};
/// use lexical_write_float::{FormattedSize, Options, ToLexicalWithOptions};
/// use lexical_write_float::format::STANDARD;
///
/// let value = 1.234e45f64;
///
/// const CUSTOM: Options = Options::builder()
///     // write exponents as "1.2^10" and not "1.2e10"
///     .exponent(b'^')
///     // use the European decimal point, so "1,2" and not "1.2"
///     .decimal_point(b',')
///     .build_strict();
///
/// const BUFFER_SIZE: usize = CUSTOM.buffer_size_const::<f64, STANDARD>();
/// let mut buffer = [0u8; BUFFER_SIZE];
/// let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &CUSTOM);
/// assert_eq!(str::from_utf8(digits), Ok("1,234^45"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionsBuilder {
    /// Maximum number of significant digits to write.
    ///
    /// If not set, it defaults to the algorithm's default.
    max_significant_digits: OptionUsize,

    /// Minimum number of significant digits to write.
    ///
    /// If not set, it defaults to the algorithm's default.
    /// Note that this isn't fully respected: if you wish to format
    /// `0.1` with 25 significant digits, the correct result **should**
    /// be `0.100000000000000005551115`. However, we would output
    /// `0.100000000000000000000000`, which is still the nearest float.
    min_significant_digits: OptionUsize,

    /// Maximum exponent prior to using scientific notation.
    ///
    /// This is ignored if the exponent base is not the same as the mantissa
    /// radix. If not provided, use the algorithm's default.
    positive_exponent_break: OptionI32,

    /// Minimum exponent prior to using scientific notation.
    ///
    /// This is ignored if the exponent base is not the same as the mantissa
    /// radix. If not provided, use the algorithm's default.
    negative_exponent_break: OptionI32,

    /// Rounding mode for writing digits with precision control.
    round_mode: RoundMode,

    /// Trim the trailing ".0" from integral float strings.
    ///
    /// If used in conjunction with [`min_significant_digits`],
    /// this will still trim all the significant digits if an integral
    /// value is provided.
    ///
    /// [`min_significant_digits`]: Self::min_significant_digits
    trim_floats: bool,

    /// Character to designate the exponent component of a float.
    exponent: u8,

    /// Character to separate the integer from the fraction components.
    decimal_point: u8,

    /// String representation of Not A Number, aka `NaN`.
    nan_string: Option<&'static [u8]>,

    /// String representation of `Infinity`.
    inf_string: Option<&'static [u8]>,
}

impl OptionsBuilder {
    // CONSTRUCTORS

    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            max_significant_digits: None,
            min_significant_digits: None,
            positive_exponent_break: None,
            negative_exponent_break: None,
            round_mode: RoundMode::Round,
            trim_floats: false,
            exponent: b'e',
            decimal_point: b'.',
            nan_string: Some(b"NaN"),
            inf_string: Some(b"inf"),
        }
    }

    // GETTERS

    /// Get the maximum number of significant digits to write.
    ///
    /// This limits the total number of written digits, truncating based
    /// on the [`round_mode`] if more digits would normally be written. If
    /// no value is provided, then it writes as many digits as required to
    /// create an unambiguous representation of the float.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_max_significant_digits(), None);
    /// ```
    ///
    /// [`round_mode`]: Self::round_mode
    #[inline(always)]
    pub const fn get_max_significant_digits(&self) -> OptionUsize {
        self.max_significant_digits
    }

    /// Get the minimum number of significant digits to write.
    ///
    /// If more digits exist, such as writing "1.2" with a minimum of 5
    /// significant digits, then `0`s are appended to the end of the digits. If
    /// no value is provided, then it writes as few digits as required to
    /// create an unambiguous representation of the float.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_min_significant_digits(), None);
    /// ```
    #[inline(always)]
    pub const fn get_min_significant_digits(&self) -> OptionUsize {
        self.min_significant_digits
    }

    /// Get the maximum exponent prior to using scientific notation.
    ///
    /// If the value is set to `300`, then any value with magnitude `>= 1e300`
    /// (for base 10) will be writen in exponent notation, while any lower
    /// value will be written in decimal form. If no value is provided, for
    /// decimal floats, this defaults to `9`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_positive_exponent_break(), None);
    /// ```
    #[inline(always)]
    pub const fn get_positive_exponent_break(&self) -> OptionI32 {
        self.positive_exponent_break
    }

    /// Get the minimum exponent prior to using scientific notation.
    ///
    /// If the value is set to `-300`, then any value with magnitude `< 1e-300`
    /// (for base 10) will be writen in exponent notation, while any larger
    /// value will be written in decimal form. If no value is provided, for
    /// decimal floats, this defaults to `-5`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_negative_exponent_break(), None);
    /// ```
    #[inline(always)]
    pub const fn get_negative_exponent_break(&self) -> OptionI32 {
        self.negative_exponent_break
    }

    /// Get the rounding mode for writing digits with precision control.
    ///
    /// For example, writing `1.23456` with 5 significant digits with
    /// [`RoundMode::Round`] would produce `"1.2346"` while
    /// [`RoundMode::Truncate`] would produce `"1.2345"`. Defaults to
    /// [`RoundMode::Round`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::{Options, RoundMode};
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_round_mode(), RoundMode::Round);
    /// ```
    #[inline(always)]
    pub const fn get_round_mode(&self) -> RoundMode {
        self.round_mode
    }

    /// Get if we should trim a trailing `".0"` from integral floats.
    ///
    /// If used in conjunction with [`min_significant_digits`],
    /// this will still trim all the significant digits if an integral
    /// value is provided. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_trim_floats(), false);
    /// ```
    ///
    /// [`min_significant_digits`]: Self::min_significant_digits
    #[inline(always)]
    pub const fn get_trim_floats(&self) -> bool {
        self.trim_floats
    }

    /// Get the character to designate the exponent component of a float.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `e`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_exponent(), b'e');
    /// ```
    #[inline(always)]
    pub const fn get_exponent(&self) -> u8 {
        self.exponent
    }

    /// Get the character to separate the integer from the fraction components.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `.`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_decimal_point(), b'.');
    /// ```
    #[inline(always)]
    pub const fn get_decimal_point(&self) -> u8 {
        self.decimal_point
    }

    /// Get the string representation for `NaN`.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`).  If set to `None`, then writing
    /// [`NaN`][f64::NAN] leads to an error. Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_nan_string(), Some("NaN".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_nan_string(&self) -> Option<&'static [u8]> {
        self.nan_string
    }

    /// Get the string representation for `Infinity`.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_inf_string(), Some("inf".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_inf_string(&self) -> Option<&'static [u8]> {
        self.inf_string
    }

    /// Get the string representation for `Infinity`. Alias for
    /// [`get_inf_string`].
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// [`get_inf_string`]: Self::get_inf_string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_infinity_string(), Some("inf".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_infinity_string(&self) -> Option<&'static [u8]> {
        self.inf_string
    }

    // SETTERS

    /// Set the maximum number of significant digits to write.
    ///
    /// This limits the total number of written digits, truncating based
    /// on the [`round_mode`] if more digits would normally be written. If
    /// no value is provided, then it writes as many digits as required to
    /// create an unambiguous representation of the float.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::num::NonZeroUsize;
    ///
    /// use lexical_write_float::Options;
    ///
    /// let max_digits = NonZeroUsize::new(300);
    /// let builder = Options::builder()
    ///     .max_significant_digits(max_digits);
    /// assert_eq!(builder.get_max_significant_digits(), max_digits);
    /// ```
    ///
    /// # Panics
    ///
    /// This will panic when building the options or writing the float if the
    /// value is smaller than [`min_significant_digits`].
    ///
    /// [`round_mode`]: Self::round_mode
    /// [`min_significant_digits`]: Self::min_significant_digits
    #[inline(always)]
    pub const fn max_significant_digits(mut self, max_significant_digits: OptionUsize) -> Self {
        self.max_significant_digits = max_significant_digits;
        self
    }

    /// Set the minimum number of significant digits to write.
    ///
    /// If more digits exist, such as writing "1.2" with a minimum of 5
    /// significant digits, then `0`s are appended to the end of the digits.
    /// If no value is provided, then it writes as few digits as required to
    /// create an unambiguous representation of the float.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::num::NonZeroUsize;
    ///
    /// use lexical_write_float::Options;
    ///
    /// let min_digits = NonZeroUsize::new(10);
    /// let builder = Options::builder()
    ///     .min_significant_digits(min_digits);
    /// assert_eq!(builder.get_min_significant_digits(), min_digits);
    /// ```
    ///
    /// # Panics
    ///
    /// This will panic when building the options or writing the float if the
    /// value is larger than [`max_significant_digits`].
    ///
    /// [`max_significant_digits`]: Self::max_significant_digits
    #[inline(always)]
    pub const fn min_significant_digits(mut self, min_significant_digits: OptionUsize) -> Self {
        self.min_significant_digits = min_significant_digits;
        self
    }

    /// Set the maximum exponent prior to using scientific notation.
    ///
    /// If the value is set to `300`, then any value with magnitude `>= 1e300`
    /// (for base 10) will be writen in exponent notation, while any lower
    /// value will be written in decimal form. If no value is provided, for
    /// decimal floats, this defaults to `9`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::num::NonZeroI32;
    ///
    /// use lexical_write_float::Options;
    ///
    /// let pos_break = NonZeroI32::new(3);
    /// let builder = Options::builder()
    ///     .positive_exponent_break(pos_break);
    /// assert_eq!(builder.get_positive_exponent_break(), pos_break);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the value is `<= 0`.
    #[inline(always)]
    pub const fn positive_exponent_break(mut self, positive_exponent_break: OptionI32) -> Self {
        self.positive_exponent_break = positive_exponent_break;
        self
    }

    /// Set the minimum exponent prior to using scientific notation.
    ///
    /// If the value is set to `-300`, then any value with magnitude `< 1e-300`
    /// (for base 10) will be writen in exponent notation, while any larger
    /// value will be written in decimal form. If no value is provided, for
    /// decimal floats, this defaults to `-5`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::num::NonZeroI32;
    ///
    /// use lexical_write_float::Options;
    ///
    /// let neg_break = NonZeroI32::new(-3);
    /// let builder = Options::builder()
    ///     .negative_exponent_break(neg_break);
    /// assert_eq!(builder.get_negative_exponent_break(), neg_break);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the value is `>= 0`.
    #[inline(always)]
    pub const fn negative_exponent_break(mut self, negative_exponent_break: OptionI32) -> Self {
        self.negative_exponent_break = negative_exponent_break;
        self
    }

    /// Set the rounding mode for writing digits with precision control.
    ///
    /// For example, writing `1.23456` with 5 significant digits with
    /// [`RoundMode::Round`] would produce `"1.2346"` while
    /// [`RoundMode::Truncate`] would produce `"1.2345"`. Defaults to
    /// [`RoundMode::Round`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(all(feature = "format", feature = "radix"))] {
    /// use core::{num, str};
    ///
    /// use lexical_write_float::{RoundMode, Options, ToLexicalWithOptions};
    /// use lexical_write_float::format::STANDARD;
    ///
    ///  let value = 1.23456f64;
    ///
    /// // truncating
    /// const TRUNCATE: Options = Options::builder()
    ///     // truncate numbers when writing less digits than present, rather than round
    ///     .round_mode(RoundMode::Truncate)
    ///     // the maximum number of significant digits to write
    ///     .max_significant_digits(num::NonZeroUsize::new(5))
    ///     // build the options, panicking if they're invalid
    ///     .build_strict();
    /// const TRUNCATE_SIZE: usize = TRUNCATE.buffer_size_const::<f64, STANDARD>();
    /// let mut buffer = [0u8; TRUNCATE_SIZE];
    /// let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &TRUNCATE);
    /// assert_eq!(str::from_utf8(digits), Ok("1.2345"));
    ///
    /// // rounding
    /// const ROUND: Options = Options::builder()
    ///     // round to the nearest number when writing less digits than present
    ///     .round_mode(RoundMode::Round)
    ///     // the maximum number of significant digits to write
    ///     .max_significant_digits(num::NonZeroUsize::new(5))
    ///     // build the options, panicking if they're invalid
    ///     .build_strict();
    /// const ROUND_SIZE: usize = ROUND.buffer_size_const::<f64, STANDARD>();
    /// let mut buffer = [0u8; ROUND_SIZE];
    /// let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &ROUND);
    /// assert_eq!(str::from_utf8(digits), Ok("1.2346"));
    /// # }
    /// ```
    #[inline(always)]
    pub const fn round_mode(mut self, round_mode: RoundMode) -> Self {
        self.round_mode = round_mode;
        self
    }

    /// Set if we should trim a trailing `".0"` from integral floats.
    ///
    /// If used in conjunction with [`min_significant_digits`],
    /// this will still trim all the significant digits if an integral
    /// value is provided. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder()
    ///     .trim_floats(true);
    /// assert_eq!(builder.get_trim_floats(), true);
    /// ```
    ///
    /// [`min_significant_digits`]: Self::min_significant_digits
    #[inline(always)]
    pub const fn trim_floats(mut self, trim_floats: bool) -> Self {
        self.trim_floats = trim_floats;
        self
    }

    /// Set the character to designate the exponent component of a float.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `e`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder()
    ///     .exponent(b'^');
    /// assert_eq!(builder.get_exponent(), b'^');
    /// ```
    #[inline(always)]
    pub const fn exponent(mut self, exponent: u8) -> Self {
        self.exponent = exponent;
        self
    }

    /// Set the character to separate the integer from the fraction components.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `.`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder()
    ///     .decimal_point(b'^');
    /// assert_eq!(builder.get_decimal_point(), b'^');
    /// ```
    #[inline(always)]
    pub const fn decimal_point(mut self, decimal_point: u8) -> Self {
        self.decimal_point = decimal_point;
        self
    }

    /// Set the string representation for `NaN`.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`NaN`][f64::NAN] returns an error. Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder()
    ///     .nan_string(Some(b"nan"));
    /// assert_eq!(builder.get_nan_string(), Some(b"nan".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements will panic at runtime. You
    /// should always build the format using [`build_strict`] or checking
    /// [`is_valid`] prior to using the format, to avoid unexpected panics.
    ///
    /// [`FORMATTED_SIZE`]: `lexical_util::constants::FormattedSize::FORMATTED_SIZE`
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[inline(always)]
    pub const fn nan_string(mut self, nan_string: Option<&'static [u8]>) -> Self {
        self.nan_string = nan_string;
        self
    }

    /// Set the string representation for `Infinity`.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder()
    ///     .inf_string(Some(b"infinity"));
    /// assert_eq!(builder.get_inf_string(), Some(b"infinity".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements will panic at runtime. You
    /// should always build the format using [`build_strict`] or checking
    /// [`is_valid`] prior to using the format, to avoid unexpected panics.
    ///
    /// [`FORMATTED_SIZE`]: `lexical_util::constants::FormattedSize::FORMATTED_SIZE`
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[inline(always)]
    pub const fn inf_string(mut self, inf_string: Option<&'static [u8]>) -> Self {
        self.inf_string = inf_string;
        self
    }

    /// Set the string representation for `Infinity`. Alias for [`inf_string`].
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// [`inf_string`]: Self::inf_string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// let builder = Options::builder()
    ///     .infinity_string(Some(b"infinity"));
    /// assert_eq!(builder.get_infinity_string(), Some(b"infinity".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements will panic at runtime. You
    /// should always build the format using [`build_strict`] or checking
    /// [`is_valid`] prior to using the format, to avoid unexpected panics.
    ///
    /// [`FORMATTED_SIZE`]: `lexical_util::constants::FormattedSize::FORMATTED_SIZE`
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[inline(always)]
    pub const fn infinity_string(self, inf_string: Option<&'static [u8]>) -> Self {
        self.inf_string(inf_string)
    }

    // BUILDERS

    /// Determine if [`nan_string`][`Self::nan_string`] is valid.
    #[doc(hidden)]
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)] // reason="more logical"
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

    /// Determine if [`inf_string`][`Self::inf_string`] is valid.
    #[doc(hidden)]
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)] // reason="more logical"
    pub const fn inf_str_is_valid(&self) -> bool {
        if self.inf_string.is_none() {
            return true;
        }

        let inf = unwrap_str(self.inf_string);
        let length = inf.len();
        if length == 0 || length > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(inf[0], b'I' | b'i') {
            false
        } else if !is_valid_letter_slice(inf) {
            false
        } else {
            true
        }
    }

    /// Check if the builder state is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)] // reason="more logical"
    pub const fn is_valid(&self) -> bool {
        if !is_valid_ascii(self.exponent) {
            false
        } else if !is_valid_ascii(self.decimal_point) {
            false
        } else if !self.nan_str_is_valid() {
            false
        } else if !self.inf_str_is_valid() {
            false
        } else {
            true
        }
    }

    /// Build the [`Options`] struct without validation.
    ///
    /// <div class="warning">
    ///
    /// This is completely safe, however, misusing this, especially
    /// the [`nan_string`] and [`inf_string`] representations could cause
    /// panics at runtime. Always use [`is_valid`] prior to using the built
    /// options.
    ///
    /// </div>
    ///
    /// [`inf_string`]: Self::inf_string
    /// [`nan_string`]: Self::nan_string
    /// [`is_valid`]: Self::is_valid
    #[inline(always)]
    pub const fn build_unchecked(&self) -> Options {
        Options {
            max_significant_digits: self.max_significant_digits,
            min_significant_digits: self.min_significant_digits,
            positive_exponent_break: self.positive_exponent_break,
            negative_exponent_break: self.negative_exponent_break,
            round_mode: self.round_mode,
            trim_floats: self.trim_floats,
            exponent: self.exponent,
            decimal_point: self.decimal_point,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
        }
    }

    /// Build the [`Options`] struct, panicking if the builder is invalid.
    ///
    /// # Panics
    ///
    /// If the built options are not valid. This should always
    /// be used within a const context to avoid panics at runtime.
    #[inline(always)]
    pub const fn build_strict(&self) -> Options {
        match self.build() {
            Ok(value) => value,
            Err(error) => core::panic!("{}", error.description()),
        }
    }

    /// Build the [`Options`] struct.
    ///
    /// If the format is not valid, than an error is returned,
    /// otherwise, the successful value is returned.
    #[inline(always)]
    #[allow(clippy::if_same_then_else)] // reason="more logical"
    pub const fn build(&self) -> Result<Options> {
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

        let min_digits = unwrap_or_zero_usize(self.min_significant_digits);
        let max_digits = unwrap_or_max_usize(self.max_significant_digits);
        if max_digits < min_digits {
            Err(Error::InvalidFloatPrecision)
        } else if unwrap_or_zero_i32(self.negative_exponent_break) > 0 {
            Err(Error::InvalidNegativeExponentBreak)
        } else if unwrap_or_zero_i32(self.positive_exponent_break) < 0 {
            Err(Error::InvalidPositiveExponentBreak)
        } else if !is_valid_ascii(self.exponent) {
            Err(Error::InvalidExponentSymbol)
        } else if !is_valid_ascii(self.decimal_point) {
            Err(Error::InvalidDecimalPoint)
        } else {
            Ok(self.build_unchecked())
        }
    }
}

impl Default for OptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Options to customize writing floats.
///
/// This enables extensive control over how the float is written, from
/// control characters like the decimal point, to the use of exponent
/// notation, and the number of significant digits.
///
/// # Examples
///
/// Writing a simple float with custom special strings and
/// formatting integral floats as integers can be done as:
///
/// ```rust
/// use core::str;
///
/// use lexical_write_float::{Options, ToLexical, ToLexicalWithOptions};
/// use lexical_write_float::format::STANDARD;
///
/// const OPTS: Options = Options::builder()
///     .trim_floats(true)
///     .nan_string(Some(b"NaN"))
///     .inf_string(Some(b"Inf"))
///     .build_strict();
///
/// const SIZE: usize = OPTS.buffer_size_const::<f64, STANDARD>();
/// let mut buffer = [0u8; SIZE];
///
/// // trim floats
/// let digits = 12345.0f64.to_lexical_with_options::<STANDARD>(&mut buffer, &OPTS);
/// assert_eq!(str::from_utf8(digits), Ok("12345"));
///
/// // don't trim floats
/// let digits = 12345.0f64.to_lexical(&mut buffer);
/// assert_eq!(str::from_utf8(digits), Ok("12345.0"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Options {
    /// Maximum number of significant digits to write.
    /// If not set, it defaults to the algorithm's default.
    max_significant_digits: OptionUsize,

    /// Minimum number of significant digits to write.
    /// If not set, it defaults to the algorithm's default.
    min_significant_digits: OptionUsize,

    /// Maximum exponent prior to using scientific notation.
    /// This is ignored if the exponent base is not the same as the mantissa
    /// radix. If not provided, use the algorithm's default.
    positive_exponent_break: OptionI32,

    /// Minimum exponent prior to using scientific notation.
    /// This is ignored if the exponent base is not the same as the mantissa
    /// radix. If not provided, use the algorithm's default.
    negative_exponent_break: OptionI32,

    /// Rounding mode for writing digits with precision control.
    round_mode: RoundMode,

    /// Trim the trailing ".0" from integral float strings.
    ///
    /// If used in conjunction with [`min_significant_digits`],
    /// this will still trim all the significant digits if an integral
    /// value is provided.
    ///
    /// [`min_significant_digits`]: Self::min_significant_digits
    trim_floats: bool,

    /// Character to designate the exponent component of a float.
    exponent: u8,

    /// Character to separate the integer from the fraction components.
    decimal_point: u8,

    /// String representation of Not A Number, aka `NaN`.
    nan_string: Option<&'static [u8]>,

    /// String representation of `Infinity`.
    inf_string: Option<&'static [u8]>,
}

impl Options {
    // CONSTRUCTORS

    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self::builder().build_unchecked()
    }

    /// Create the default options for a given radix.
    ///
    /// <div class="warning">
    ///
    /// This function will never fail even if the radix is invalid. It is up to
    /// the caller to ensure the format is valid using
    /// [`Options::is_valid`]. Only radixes from `2` to `36` should be used.
    ///
    /// </div>
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    // FIXME: When we release a major version, validate the radix.
    pub const fn from_radix(radix: u8) -> Self {
        // Need to determine the correct exponent character ('e' or '^'),
        // since the default character is `e` normally, but this is a valid
        // digit for radix >= 15.
        let mut builder = Self::builder();
        if radix >= 15 {
            builder = builder.exponent(b'^');
        }
        builder.build_unchecked()
    }

    /// Get an upper bound on the required buffer size.
    ///
    /// This is used when custom formatting options, such as significant
    /// digits specifiers or custom exponent breaks, are used, which
    /// can lead to more or less significant digits being written than
    /// expected. If using the default formatting options, then this will
    /// always be [`FORMATTED_SIZE`][FormattedSize::FORMATTED_SIZE] or
    /// [`FORMATTED_SIZE_DECIMAL`][FormattedSize::FORMATTED_SIZE_DECIMAL],
    /// depending on the radix.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(all(feature = "format", feature = "radix"))] {
    /// use core::{num, str};
    ///
    /// use lexical_write_float::{FormattedSize, Options, ToLexicalWithOptions};
    /// use lexical_write_float::format::STANDARD;
    ///
    /// const DEFAULT: Options = Options::builder()
    ///     // require at least 3 significant digits, so `0.01` would be `"0.0100"`.
    ///     .min_significant_digits(num::NonZeroUsize::new(3))
    ///     // allow at most 5 significant digits, so `1.23456` would be `"1.2346"`.
    ///     .max_significant_digits(num::NonZeroUsize::new(5))
    ///     // build our format, erroring if it's invalid
    ///     .build_strict();
    /// assert_eq!(DEFAULT.buffer_size_const::<f64, STANDARD>(), f64::FORMATTED_SIZE_DECIMAL);
    ///
    /// const CUSTOM: Options = Options::builder()
    ///     // require at least 300 significant digits.
    ///     .min_significant_digits(num::NonZeroUsize::new(300))
    ///     // allow at most 500 significant digits.
    ///     .max_significant_digits(num::NonZeroUsize::new(500))
    ///     // only write values with magnitude above 1e300 in exponent notation
    ///     .positive_exponent_break(num::NonZeroI32::new(300))
    ///     // only write values with magnitude below 1e-300 in exponent notation
    ///     .negative_exponent_break(num::NonZeroI32::new(-300))
    ///     .build_strict();
    /// // 300 for the significant digits (500 is never reachable), 300 extra
    /// // due to the exponent breakoff, 1 for the sign, 1 for the decimal point
    /// // in all cases, this is enough including the exponent character and sign.
    /// assert_eq!(CUSTOM.buffer_size_const::<f64, STANDARD>(), 602);
    ///
    /// // now, write out value to bytes
    /// const SIZE: usize = CUSTOM.buffer_size_const::<f64, STANDARD>();
    /// let mut buffer = [0u8; SIZE];
    /// let value = 1.23456e-299f64;
    /// let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &CUSTOM);
    ///
    /// // validate our printed digits. 600!
    /// assert_eq!(digits.len(), 600);
    /// assert!(!digits.contains(&b'e') && !digits.contains(&b'E'));
    /// assert!(digits.starts_with(b"0.000000000000000000000000"));
    ///
    /// // validate the round-trip
    /// assert_eq!(str::from_utf8(digits).unwrap().parse::<f64>(), Ok(value));
    ///
    /// // let's serialize a slightly smaller value
    /// let value = 1.23456e-301f64;
    /// let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &CUSTOM);
    /// assert_eq!(digits.len(), 306);
    /// let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &CUSTOM);
    /// # }
    /// ```
    #[inline(always)]
    pub const fn buffer_size_const<T: FormattedSize, const FORMAT: u128>(&self) -> usize {
        let format = NumberFormat::<{ FORMAT }> {};

        // NOTE: This looks like it's off by 2 but it's not. We have only 2 as a
        // baseline for the mantissa sign and decimal point, but we don't
        // hard-code in 2 for the exponent sign and character. But we consider
        // those cases already when the exponent breakof >= 5 (13 for radix).
        // Say we have `1.2345612345612346e-300` for a breakoff of `-300` with
        // 300 min significant digits, which would be:
        //  - "0." (integral + decimal point)
        // - "0" * 299 (leading zeros for e-300)
        // - "123456" * 50 (300 significant digits)
        //
        // This is exactly 601 characters, with which a sign bit is 602.
        // If we go any lower, we have `1.2e-301`, which then would become
        // `1.23456...e-301`, or would be 306 characters.

        // quick optimizations if no custom formatting options are used
        // we ensure that the mantissa and exponent radix are the same.
        let formatted_size = if format.radix() == 10 {
            T::FORMATTED_SIZE_DECIMAL
        } else {
            T::FORMATTED_SIZE
        };

        // At least 2 for the decimal point and sign.
        let mut count: usize = 2;

        // First need to calculate maximum number of digits from leading or
        // trailing zeros, IE, the exponent break.
        if !format.no_exponent_notation() {
            let min_exp = match self.negative_exponent_break() {
                Some(v) => v.get(),
                None => -5,
            };
            let max_exp = match self.positive_exponent_break() {
                Some(v) => v.get(),
                None => 9,
            };
            let exp = max!(min_exp.abs(), max_exp) as usize;
            if cfg!(feature = "power-of-two") && exp < 13 {
                // 11 for the exponent digits in binary, 1 for the sign, 1 for the symbol
                count += 13;
            } else if exp < 5 {
                // 3 for the exponent digits in decimal, 1 for the sign, 1 for the symbol
                count += 5;
            } else {
                // More leading or trailing zeros than the exponent digits.
                count += exp;
            }
        } else if cfg!(feature = "power-of-two") {
            // Min is 2^-1075.
            count += 1075;
        } else {
            // Min is 10^-324.
            count += 324;
        }

        // Now add the number of significant digits.
        let radix = format.radix();
        let formatted_digits = if radix == 10 {
            // Really should be 18, but add some extra to be cautious.
            28
        } else {
            //  BINARY:
            //      53 significant mantissa bits for binary, add a few extra.
            //  RADIX:
            //      Our limit is `delta`. The maximum relative delta is 2.22e-16,
            //      around 1. If we have values below 1, our delta is smaller, but
            //      the max fraction is also a lot smaller. Above, and our fraction
            //      must be < 1.0, so our delta is less significant. Therefore,
            //      if our fraction is just less than 1, for a float near 2.0,
            //      we can do at **maximum** 33 digits (for base 3). Let's just
            //      assume it's a lot higher, and go with 64.
            64
        };
        let digits = if let Some(max_digits) = self.max_significant_digits() {
            min!(formatted_digits, max_digits.get())
        } else {
            formatted_digits
        };
        let digits = if let Some(min_digits) = self.min_significant_digits() {
            max!(digits, min_digits.get())
        } else {
            digits
        };
        count += digits;

        // we need to make sure we have at least enough room for the
        // default formatting size, no matter what, just as a precaution.
        count = max!(count, formatted_size);

        count
    }

    // GETTERS

    /// Check if the options state is valid.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.rebuild().is_valid()
    }

    /// Get the maximum number of significant digits to write.
    ///
    /// This limits the total number of written digits, truncating based
    /// on the [`round_mode`] if more digits would normally be written. If
    /// no value is provided, then it writes as many digits as required to
    /// create an unambiguous representation of the float.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::num::NonZeroUsize;
    ///
    /// use lexical_write_float::Options;
    ///
    /// const MAX_DIGITS: Option<NonZeroUsize> = NonZeroUsize::new(300);
    /// const OPTIONS: Options = Options::builder()
    ///     .max_significant_digits(MAX_DIGITS)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.max_significant_digits(), MAX_DIGITS);
    /// ```
    ///
    /// [`round_mode`]: Self::round_mode
    #[inline(always)]
    pub const fn max_significant_digits(&self) -> OptionUsize {
        self.max_significant_digits
    }

    /// Get the minimum number of significant digits to write.
    ///
    /// If more digits exist, such as writing "1.2" with a minimum of 5
    /// significant digits, then `0`s are appended to the end of the digits.
    /// If no value is provided, then it writes as few digits as required to
    /// create an unambiguous representation of the float.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::num::NonZeroUsize;
    ///
    /// use lexical_write_float::Options;
    ///
    /// const MIN_DIGITS: Option<NonZeroUsize> = NonZeroUsize::new(10);
    /// const OPTIONS: Options = Options::builder()
    ///     .min_significant_digits(MIN_DIGITS)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.min_significant_digits(), MIN_DIGITS);
    /// ```
    #[inline(always)]
    pub const fn min_significant_digits(&self) -> OptionUsize {
        self.min_significant_digits
    }

    /// Get the maximum exponent prior to using scientific notation.
    ///
    /// If the value is set to `300`, then any value with magnitude `>= 1e300`
    /// (for base 10) will be writen in exponent notation, while any lower
    /// value will be written in decimal form. If no value is provided, for
    /// decimal floats, this defaults to `9`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::num::NonZeroI32;
    ///
    /// use lexical_write_float::Options;
    ///
    /// const POS_BREAK: Option<NonZeroI32> = NonZeroI32::new(3);
    /// const OPTIONS: Options = Options::builder()
    ///     .positive_exponent_break(POS_BREAK)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.positive_exponent_break(), POS_BREAK);
    /// ```
    #[inline(always)]
    pub const fn positive_exponent_break(&self) -> OptionI32 {
        self.positive_exponent_break
    }

    /// Get the minimum exponent prior to using scientific notation.
    ///
    /// If the value is set to `-300`, then any value with magnitude `< 1e-300`
    /// (for base 10) will be writen in exponent notation, while any larger
    /// value will be written in decimal form. If no value is provided, for
    /// decimal floats, this defaults to `-5`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::num::NonZeroI32;
    ///
    /// use lexical_write_float::Options;
    ///
    /// const NEG_BREAK: Option<NonZeroI32> = NonZeroI32::new(-3);
    /// const OPTIONS: Options = Options::builder()
    ///     .negative_exponent_break(NEG_BREAK)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.negative_exponent_break(), NEG_BREAK);
    /// ```
    #[inline(always)]
    pub const fn negative_exponent_break(&self) -> OptionI32 {
        self.negative_exponent_break
    }

    /// Get the rounding mode for writing digits with precision control.
    ///
    /// For example, writing `1.23456` with 5 significant digits with
    /// [`RoundMode::Round`] would produce `"1.2346"` while
    /// [`RoundMode::Truncate`] would produce `"1.2345"`. Defaults to
    /// [`RoundMode::Round`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::{Options, RoundMode};
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .round_mode(RoundMode::Truncate)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.round_mode(), RoundMode::Truncate);
    /// ```
    #[inline(always)]
    pub const fn round_mode(&self) -> RoundMode {
        self.round_mode
    }

    /// Get if we should trim a trailing `".0"` from integral floats.
    ///
    /// If used in conjunction with [`min_significant_digits`],
    /// this will still trim all the significant digits if an integral
    /// value is provided. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .trim_floats(true)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.trim_floats(), true);
    /// ```
    ///
    /// [`min_significant_digits`]: Self::min_significant_digits
    #[inline(always)]
    pub const fn trim_floats(&self) -> bool {
        self.trim_floats
    }

    /// Get the character to designate the exponent component of a float.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `e`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .exponent(b'^')
    ///     .build_strict();
    /// assert_eq!(OPTIONS.exponent(), b'^');
    /// ```
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        self.exponent
    }

    /// Get the character to separate the integer from the fraction components.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `.`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .exponent(b',')
    ///     .build_strict();
    /// assert_eq!(OPTIONS.exponent(), b',');
    /// ```
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        self.decimal_point
    }

    /// Get the string representation for `NaN`.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`NaN`][f64::NAN] returns an error. Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .nan_string(Some(b"nan"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.nan_string(), Some(b"nan".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn nan_string(&self) -> Option<&'static [u8]> {
        self.nan_string
    }

    /// Get the string representation for `Infinity`.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .inf_string(Some(b"infinity"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.inf_string(), Some(b"infinity".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn inf_string(&self) -> Option<&'static [u8]> {
        self.inf_string
    }

    /// Get the string representation for `Infinity`. Alias for [`inf_string`].
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// [`inf_string`]: Self::inf_string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_write_float::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .infinity_string(Some(b"infinity"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.infinity_string(), Some(b"infinity".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn infinity_string(&self) -> Option<&'static [u8]> {
        self.inf_string
    }

    // SETTERS

    /// Set the maximum number of significant digits to write.
    ///
    /// This limits the total number of written digits, truncating based
    /// on the [`round_mode`] if more digits would normally be written.
    ///
    /// # Panics
    ///
    /// This will panic when writing the float if the value is smaller than
    /// [`min_significant_digits`].
    ///
    /// [`round_mode`]: Self::round_mode
    /// [`min_significant_digits`]: Self::min_significant_digits
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    #[inline(always)]
    pub fn set_max_significant_digits(&mut self, max_significant_digits: OptionUsize) {
        self.max_significant_digits = max_significant_digits;
    }

    /// Set the minimum number of significant digits to write.
    ///
    /// If more digits exist, such as writing "1.2" with a minimum of 5
    /// significant digits, then `0`s are appended to the end of the digits.
    ///
    /// # Panics
    ///
    /// This will panic when writing the float if the value is larger than
    /// [`max_significant_digits`].
    ///
    /// [`max_significant_digits`]: Self::max_significant_digits
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_min_significant_digits(&mut self, min_significant_digits: OptionUsize) {
        self.min_significant_digits = min_significant_digits;
    }

    /// Set the maximum exponent prior to using scientific notation.
    ///
    /// If the value is set to `300`, then any value with magnitude `>= 1e300`
    /// (for base 10) will be writen in exponent notation, while any lower
    /// value will be written in decimal form.
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_positive_exponent_break(&mut self, positive_exponent_break: OptionI32) {
        self.positive_exponent_break = positive_exponent_break;
    }

    /// Set the minimum exponent prior to using scientific notation.
    ///
    /// If the value is set to `-300`, then any value with magnitude `< 1e-300`
    /// (for base 10) will be writen in exponent notation, while any larger
    /// value will be written in decimal form.
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_negative_exponent_break(&mut self, negative_exponent_break: OptionI32) {
        self.negative_exponent_break = negative_exponent_break;
    }

    /// Set the rounding mode for writing digits with precision control.
    ///
    /// For example, writing `1.23456` with 5 significant digits with
    /// [`RoundMode::Round`] would produce `"1.2346"` while
    /// [`RoundMode::Truncate`] would produce `"1.2345"`.
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_round_mode(&mut self, round_mode: RoundMode) {
        self.round_mode = round_mode;
    }

    /// Set if we should trim a trailing `".0"` from integral floats.
    ///
    /// If used in conjunction with [`min_significant_digits`],
    /// this will still trim all the significant digits if an integral
    /// value is provided.
    ///
    /// [`min_significant_digits`]: Self::min_significant_digits
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_trim_floats(&mut self, trim_floats: bool) {
        self.trim_floats = trim_floats;
    }

    /// Set the character to designate the exponent component of a float.
    ///
    /// # Safety
    ///
    /// Always safe, but may produce invalid output if the exponent
    /// is not a valid ASCII character.
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_exponent(&mut self, exponent: u8) {
        self.exponent = exponent;
    }

    /// Set the character to separate the integer from the fraction components.
    ///
    /// # Safety
    ///
    /// Always safe, but may produce invalid output if the decimal point
    /// is not a valid ASCII character.
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_decimal_point(&mut self, decimal_point: u8) {
        self.decimal_point = decimal_point;
    }

    /// Set the string representation for `NaN`.
    ///
    /// Panics
    ///
    /// Setting a value too large may cause a panic even if [`FORMATTED_SIZE`]
    /// elements are provided.
    ///
    /// [`FORMATTED_SIZE`]: `lexical_util::constants::FormattedSize::FORMATTED_SIZE`
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_nan_string(&mut self, nan_string: Option<&'static [u8]>) {
        self.nan_string = nan_string;
    }

    /// Set the short string representation for `Infinity`
    ///
    /// Panics
    ///
    /// Setting a value too large may cause a panic even if [`FORMATTED_SIZE`]
    /// elements are provided.
    ///
    /// [`FORMATTED_SIZE`]: `lexical_util::constants::FormattedSize::FORMATTED_SIZE`
    #[inline(always)]
    #[deprecated = "Options should be treated as immutable, use `OptionsBuilder` instead. Will be removed in 2.0."]
    pub fn set_inf_string(&mut self, inf_string: Option<&'static [u8]>) {
        self.inf_string = inf_string;
    }

    // BUILDERS

    /// Get [`OptionsBuilder`] as a static function.
    #[inline(always)]
    pub const fn builder() -> OptionsBuilder {
        OptionsBuilder::new()
    }

    /// Create [`OptionsBuilder`] using existing values.
    #[inline(always)]
    pub const fn rebuild(&self) -> OptionsBuilder {
        OptionsBuilder {
            max_significant_digits: self.max_significant_digits,
            min_significant_digits: self.min_significant_digits,
            positive_exponent_break: self.positive_exponent_break,
            negative_exponent_break: self.negative_exponent_break,
            round_mode: self.round_mode,
            trim_floats: self.trim_floats,
            exponent: self.exponent,
            decimal_point: self.decimal_point,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
        }
    }
}

impl Default for Options {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl WriteOptions for Options {
    #[inline(always)]
    fn is_valid(&self) -> bool {
        Self::is_valid(self)
    }

    #[doc = lexical_util::write_options_doc!()]
    #[inline(always)]
    fn buffer_size<T: FormattedSize, const FORMAT: u128>(&self) -> usize {
        self.buffer_size_const::<T, FORMAT>()
    }
}

/// Define `unwrap_or_zero` for a custom type.
macro_rules! unwrap_or_zero {
    ($name:ident, $opt:ident, $t:ident) => {
        /// Unwrap `Option` as a const fn.
        #[inline(always)]
        const fn $name(option: $opt) -> $t {
            match option {
                Some(x) => x.get(),
                None => 0,
            }
        }
    };
}

unwrap_or_zero!(unwrap_or_zero_usize, OptionUsize, usize);
unwrap_or_zero!(unwrap_or_zero_i32, OptionI32, i32);

/// Unwrap `Option` as a const fn.
#[inline(always)]
const fn unwrap_or_max_usize(option: OptionUsize) -> usize {
    match option {
        Some(x) => x.get(),
        None => usize::MAX,
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

/// Numerical format with a decimal comma.
///
/// This is the standard numerical format for most of the world.
#[rustfmt::skip]
pub const DECIMAL_COMMA: Options = Options::builder()
    .decimal_point(b',')
    .build_strict();

/// Numerical format for hexadecimal floats, which use a `p` exponent.
#[rustfmt::skip]
pub const HEX_FLOAT: Options = Options::builder()
    .exponent(b'p')
    .build_strict();

/// Numerical format where `^` is used as the exponent notation character.
///
/// This isn't very common, but is useful when `e` or `p` are valid digits.
#[rustfmt::skip]
pub const CARAT_EXPONENT: Options = Options::builder()
    .exponent(b'^')
    .build_strict();

/// Number format for a [`Rust`] literal floating-point number.
///
/// [`Rust`]: https://www.rust-lang.org/
#[rustfmt::skip]
pub const RUST_LITERAL: Options = Options::builder()
    .nan_string(options::RUST_LITERAL)
    .inf_string(options::RUST_LITERAL)
    .build_strict();

/// Number format for a [`Python`] literal floating-point number.
///
/// [`Python`]: https://www.python.org/
#[rustfmt::skip]
pub const PYTHON_LITERAL: Options = Options::builder()
    .nan_string(options::PYTHON_LITERAL)
    .inf_string(options::PYTHON_LITERAL)
    .build_strict();

/// Number format for a [`C++`] literal floating-point number.
///
/// [`C++`]: https://en.cppreference.com/w/
#[rustfmt::skip]
pub const CXX_LITERAL: Options = Options::builder()
    .nan_string(options::CXX_LITERAL_NAN)
    .inf_string(options::CXX_LITERAL_INF)
    .build_strict();

/// Number format for a [`C`] literal floating-point number.
///
/// [`C`]: https://en.cppreference.com/w/c
#[rustfmt::skip]
pub const C_LITERAL: Options = Options::builder()
    .nan_string(options::C_LITERAL_NAN)
    .inf_string(options::C_LITERAL_INF)
    .build_strict();

/// Number format for a [`Ruby`] literal floating-point number.
///
/// [`Ruby`]: https://www.ruby-lang.org/en/
#[rustfmt::skip]
pub const RUBY_LITERAL: Options = Options::builder()
    .positive_exponent_break(num::NonZeroI32::new(14))
    .negative_exponent_break(num::NonZeroI32::new(-4))
    .nan_string(options::RUBY_LITERAL_NAN)
    .inf_string(options::RUBY_LITERAL_INF)
    .build_strict();

/// Number format to parse a [`Ruby`] float from string.
///
/// [`Ruby`]: https://www.ruby-lang.org/en/
#[rustfmt::skip]
pub const RUBY_STRING: Options = Options::builder()
    .nan_string(options::RUBY_LITERAL_NAN)
    .inf_string(options::RUBY_LITERAL_INF)
    .build_strict();

/// Number format for a [`Swift`] literal floating-point number.
///
/// [`Swift`]: https://developer.apple.com/swift/
#[rustfmt::skip]
pub const SWIFT_LITERAL: Options = Options::builder()
    .nan_string(options::SWIFT_LITERAL)
    .inf_string(options::SWIFT_LITERAL)
    .build_strict();

/// Number format for a [`Golang`] literal floating-point number.
///
/// [`Golang`]: https://go.dev/
#[rustfmt::skip]
pub const GO_LITERAL: Options = Options::builder()
    .nan_string(options::GO_LITERAL)
    .inf_string(options::GO_LITERAL)
    .build_strict();

/// Number format for a [`Haskell`] literal floating-point number.
///
/// [`Haskell`]: https://www.haskell.org/
#[rustfmt::skip]
pub const HASKELL_LITERAL: Options = Options::builder()
    .nan_string(options::HASKELL_LITERAL)
    .inf_string(options::HASKELL_LITERAL)
    .build_strict();

/// Number format to parse a [`Haskell`] float from string.
///
/// [`Haskell`]: https://www.haskell.org/
#[rustfmt::skip]
pub const HASKELL_STRING: Options = Options::builder()
    .inf_string(options::HASKELL_STRING_INF)
    .build_strict();

/// Number format for a [`Javascript`] literal floating-point number.
///
/// [`Javascript`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
#[rustfmt::skip]
pub const JAVASCRIPT_LITERAL: Options = Options::builder()
    .inf_string(options::JAVASCRIPT_INF)
    .build_strict();

/// Number format to parse a [`Javascript`] float from string.
///
/// [`Javascript`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
#[rustfmt::skip]
pub const JAVASCRIPT_STRING: Options = Options::builder()
    .inf_string(options::JAVASCRIPT_INF)
    .build_strict();

/// Number format for a [`Perl`] literal floating-point number.
///
/// [`Perl`]: https://www.perl.org/
#[rustfmt::skip]
pub const PERL_LITERAL: Options = Options::builder()
    .nan_string(options::PERL_LITERAL)
    .inf_string(options::PERL_LITERAL)
    .build_strict();

/// Number format for a [`PHP`] literal floating-point number.
///
/// [`PHP`]: https://www.php.net/
#[rustfmt::skip]
pub const PHP_LITERAL: Options = Options::builder()
    .nan_string(options::PHP_LITERAL_NAN)
    .inf_string(options::PHP_LITERAL_INF)
    .build_strict();

/// Number format for a [`Java`] literal floating-point number.
///
/// [`Java`]: https://www.java.com/en/
#[rustfmt::skip]
pub const JAVA_LITERAL: Options = Options::builder()
    .nan_string(options::JAVA_LITERAL)
    .inf_string(options::JAVA_LITERAL)
    .build_strict();

/// Number format to parse a [`Java`] float from string.
///
/// [`Java`]: https://www.java.com/en/
#[rustfmt::skip]
pub const JAVA_STRING: Options = Options::builder()
    .inf_string(options::JAVA_STRING_INF)
    .build_strict();

/// Number format for an [`R`] literal floating-point number.
///
/// [`R`]: https://www.r-project.org/
#[rustfmt::skip]
pub const R_LITERAL: Options = Options::builder()
    .inf_string(options::R_LITERAL_INF)
    .build_strict();

/// Number format for a [`Kotlin`] literal floating-point number.
///
/// [`Kotlin`]: https://kotlinlang.org/
#[rustfmt::skip]
pub const KOTLIN_LITERAL: Options = Options::builder()
    .nan_string(options::KOTLIN_LITERAL)
    .inf_string(options::KOTLIN_LITERAL)
    .build_strict();

/// Number format to parse a [`Kotlin`] float from string.
///
/// [`Kotlin`]: https://kotlinlang.org/
#[rustfmt::skip]
pub const KOTLIN_STRING: Options = Options::builder()
    .inf_string(options::KOTLIN_STRING_INF)
    .build_strict();

/// Number format for a [`Julia`] literal floating-point number.
///
/// [`Julia`]: https://julialang.org/
#[rustfmt::skip]
pub const JULIA_LITERAL: Options = Options::builder()
    .inf_string(options::JULIA_LITERAL_INF)
    .build_strict();

/// Number format for a [`C#`] literal floating-point number.
///
/// [`C#`]: https://learn.microsoft.com/en-us/dotnet/csharp/
#[rustfmt::skip]
pub const CSHARP_LITERAL: Options = Options::builder()
    .nan_string(options::CSHARP_LITERAL)
    .inf_string(options::CSHARP_LITERAL)
    .build_strict();

/// Number format to parse a [`C#`] float from string.
///
/// [`C#`]: https://learn.microsoft.com/en-us/dotnet/csharp/
#[rustfmt::skip]
pub const CSHARP_STRING: Options = Options::builder()
    .inf_string(options::CSHARP_STRING_INF)
    .build_strict();

/// Number format for a [`Kawa`] literal floating-point number.
///
/// [`Kawa`]: https://www.gnu.org/software/kawa/
#[rustfmt::skip]
pub const KAWA_LITERAL: Options = Options::builder()
    .nan_string(options::KAWA)
    .inf_string(options::KAWA)
    .build_strict();

/// Number format to parse a [`Kawa`] float from string.
///
/// [`Kawa`]: https://www.gnu.org/software/kawa/
#[rustfmt::skip]
pub const KAWA_STRING: Options = Options::builder()
    .nan_string(options::KAWA)
    .inf_string(options::KAWA)
    .build_strict();

/// Number format for a [`Gambit-C`] literal floating-point number.
///
/// [`Gambit-C`]: https://gambitscheme.org/
#[rustfmt::skip]
pub const GAMBITC_LITERAL: Options = Options::builder()
    .nan_string(options::GAMBITC)
    .inf_string(options::GAMBITC)
    .build_strict();

/// Number format to parse a [`Gambit-C`] float from string.
///
/// [`Gambit-C`]: https://gambitscheme.org/
#[rustfmt::skip]
pub const GAMBITC_STRING: Options = Options::builder()
    .nan_string(options::GAMBITC)
    .inf_string(options::GAMBITC)
    .build_strict();

/// Number format for a [`Guile`] literal floating-point number.
///
/// [`Guile`]: https://www.gnu.org/software/guile/
#[rustfmt::skip]
pub const GUILE_LITERAL: Options = Options::builder()
    .nan_string(options::GUILE)
    .inf_string(options::GUILE)
    .build_strict();

/// Number format to parse a [`Guile`] float from string.
///
/// [`Guile`]: https://www.gnu.org/software/guile/
#[rustfmt::skip]
pub const GUILE_STRING: Options = Options::builder()
    .nan_string(options::GUILE)
    .inf_string(options::GUILE)
    .build_strict();

/// Number format for a [`Clojure`] literal floating-point number.
///
/// [`Clojure`]: https://clojure.org/
#[rustfmt::skip]
pub const CLOJURE_LITERAL: Options = Options::builder()
    .nan_string(options::CLOJURE_LITERAL)
    .inf_string(options::CLOJURE_LITERAL)
    .build_strict();

/// Number format to parse a [`Clojure`] float from string.
///
/// [`Clojure`]: https://clojure.org/
#[rustfmt::skip]
pub const CLOJURE_STRING: Options = Options::builder()
    .inf_string(options::CLOJURE_STRING_INF)
    .build_strict();

/// Number format for an [`Erlang`] literal floating-point number.
///
/// [`Erlang`]: https://www.erlang.org/
#[rustfmt::skip]
pub const ERLANG_LITERAL: Options = Options::builder()
    .nan_string(options::ERLANG_LITERAL_NAN)
    .build_strict();

/// Number format to parse an [`Erlang`] float from string.
///
/// [`Erlang`]: https://www.erlang.org/
#[rustfmt::skip]
pub const ERLANG_STRING: Options = Options::builder()
    .nan_string(options::ERLANG_STRING)
    .inf_string(options::ERLANG_STRING)
    .build_strict();

/// Number format for an [`Elm`] literal floating-point number.
///
/// [`Elm`]: https://elm-lang.org/
#[rustfmt::skip]
pub const ELM_LITERAL: Options = Options::builder()
    .nan_string(options::ELM_LITERAL)
    .inf_string(options::ELM_LITERAL)
    .build_strict();

/// Number format to parse an [`Elm`] float from string.
///
/// [`Elm`]: https://elm-lang.org/
#[rustfmt::skip]
pub const ELM_STRING: Options = Options::builder()
    .nan_string(options::ELM_STRING_NAN)
    .inf_string(options::ELM_STRING_INF)
    .build_strict();

/// Number format for a [`Scala`] literal floating-point number.
///
/// [`Scala`]: https://www.scala-lang.org/
#[rustfmt::skip]
pub const SCALA_LITERAL: Options = Options::builder()
    .nan_string(options::SCALA_LITERAL)
    .inf_string(options::SCALA_LITERAL)
    .build_strict();

/// Number format to parse a [`Scala`] float from string.
///
/// [`Scala`]: https://www.scala-lang.org/
#[rustfmt::skip]
pub const SCALA_STRING: Options = Options::builder()
    .inf_string(options::SCALA_STRING_INF)
    .build_strict();

/// Number format for an [`Elixir`] literal floating-point number.
///
/// [`Elixir`]: https://elixir-lang.org/
#[rustfmt::skip]
pub const ELIXIR_LITERAL: Options = Options::builder()
    .nan_string(options::ELIXIR)
    .inf_string(options::ELIXIR)
    .build_strict();

/// Number format to parse an [`Elixir`] float from string.
///
/// [`Elixir`]: https://elixir-lang.org/
#[rustfmt::skip]
pub const ELIXIR_STRING: Options = Options::builder()
    .nan_string(options::ELIXIR)
    .inf_string(options::ELIXIR)
    .build_strict();

/// Number format for a [`FORTRAN`] literal floating-point number.
///
/// [`FORTRAN`]: https://fortran-lang.org/
#[rustfmt::skip]
pub const FORTRAN_LITERAL: Options = Options::builder()
    .nan_string(options::FORTRAN_LITERAL)
    .inf_string(options::FORTRAN_LITERAL)
    .build_strict();

/// Number format for a [`D`] literal floating-point number.
///
/// [`D`]: https://dlang.org/
#[rustfmt::skip]
pub const D_LITERAL: Options = Options::builder()
    .nan_string(options::D_LITERAL)
    .inf_string(options::D_LITERAL)
    .build_strict();

/// Number format for a [`Coffeescript`] literal floating-point number.
///
/// [`Coffeescript`]: https://coffeescript.org/
#[rustfmt::skip]
pub const COFFEESCRIPT_LITERAL: Options = Options::builder()
    .inf_string(options::COFFEESCRIPT_INF)
    .build_strict();

/// Number format to parse a [`Coffeescript`] float from string.
///
/// [`Coffeescript`]: https://coffeescript.org/
#[rustfmt::skip]
pub const COFFEESCRIPT_STRING: Options = Options::builder()
    .inf_string(options::COFFEESCRIPT_INF)
    .build_strict();

/// Number format for a [`COBOL`] literal floating-point number.
///
/// [`Cobol`]: https://www.ibm.com/think/topics/cobol
#[rustfmt::skip]
pub const COBOL_LITERAL: Options = Options::builder()
    .nan_string(options::COBOL)
    .inf_string(options::COBOL)
    .build_strict();

/// Number format to parse a [`COBOL`] float from string.
///
/// [`Cobol`]: https://www.ibm.com/think/topics/cobol
#[rustfmt::skip]
pub const COBOL_STRING: Options = Options::builder()
    .nan_string(options::COBOL)
    .inf_string(options::COBOL)
    .build_strict();

/// Number format for an [`F#`] literal floating-point number.
///
/// [`F#`]: https://fsharp.org/
#[rustfmt::skip]
pub const FSHARP_LITERAL: Options = Options::builder()
    .nan_string(options::FSHARP_LITERAL_NAN)
    .inf_string(options::FSHARP_LITERAL_INF)
    .build_strict();

/// Number format for a [`Visual Basic`] literal floating-point number.
///
/// [`Visual Basic`]: https://learn.microsoft.com/en-us/dotnet/visual-basic/
#[rustfmt::skip]
pub const VB_LITERAL: Options = Options::builder()
    .nan_string(options::VB_LITERAL)
    .inf_string(options::VB_LITERAL)
    .build_strict();

/// Number format to parse a [`Visual Basic`] float from string.
///
/// [`Visual Basic`]: https://learn.microsoft.com/en-us/dotnet/visual-basic/
#[rustfmt::skip]
pub const VB_STRING: Options = Options::builder()
    .inf_string(options::VB_STRING_INF)
    .build_strict();

/// Number format for an [`OCaml`] literal floating-point number.
///
/// [`OCaml`]: https://ocaml.org/
#[rustfmt::skip]
pub const OCAML_LITERAL: Options = Options::builder()
    .nan_string(options::OCAML_LITERAL_NAN)
    .inf_string(options::OCAML_LITERAL_INF)
    .build_strict();

/// Number format for an [`Objective-C`] literal floating-point number.
///
/// [`Objective-C`]: https://en.wikipedia.org/wiki/Objective-C
#[rustfmt::skip]
pub const OBJECTIVEC_LITERAL: Options = Options::builder()
    .nan_string(options::OBJECTIVEC)
    .inf_string(options::OBJECTIVEC)
    .build_strict();

/// Number format to parse an [`Objective-C`] float from string.
///
/// [`Objective-C`]: https://en.wikipedia.org/wiki/Objective-C
#[rustfmt::skip]
pub const OBJECTIVEC_STRING: Options = Options::builder()
    .nan_string(options::OBJECTIVEC)
    .inf_string(options::OBJECTIVEC)
    .build_strict();

/// Number format for an [`ReasonML`] literal floating-point number.
///
/// [`ReasonML`]: https://reasonml.github.io/
#[rustfmt::skip]
pub const REASONML_LITERAL: Options = Options::builder()
    .nan_string(options::REASONML_LITERAL_NAN)
    .inf_string(options::REASONML_LITERAL_INF)
    .build_strict();

/// Number format for a [`MATLAB`] literal floating-point number.
///
/// [`Matlab`]: https://www.mathworks.com/products/matlab.html
#[rustfmt::skip]
pub const MATLAB_LITERAL: Options = Options::builder()
    .inf_string(options::MATLAB_LITERAL_INF)
    .build_strict();

/// Number format for a [`Zig`] literal floating-point number.
///
/// [`Zig`]: https://ziglang.org/
#[rustfmt::skip]
pub const ZIG_LITERAL: Options = Options::builder()
    .nan_string(options::ZIG_LITERAL)
    .inf_string(options::ZIG_LITERAL)
    .build_strict();

/// Number format for a [`Sage`] literal floating-point number.
///
/// [`Sage`]: https://www.sagemath.org/
#[rustfmt::skip]
pub const SAGE_LITERAL: Options = Options::builder()
    .inf_string(options::SAGE_LITERAL_INF)
    .build_strict();

/// Number format for a [`JSON`][`JSON-REF`] literal floating-point number.
///
/// [`JSON-REF`]: https://www.json.org/json-en.html
#[rustfmt::skip]
pub const JSON: Options = Options::builder()
    .nan_string(options::JSON)
    .inf_string(options::JSON)
    .build_strict();

/// Number format for a [`TOML`][`TOML-REF`] literal floating-point number.
///
/// [`TOML-REF`]: https://toml.io/en/
#[rustfmt::skip]
pub const TOML: Options = Options::builder()
    .nan_string(options::TOML)
    .inf_string(options::TOML)
    .build_strict();

/// Number format for a [`YAML`][`YAML-REF`] literal floating-point number.
///
/// [`YAML-REF`]: https://yaml.org/
#[rustfmt::skip]
pub const YAML: Options = JSON;

/// Number format for an [`XML`][`XML-REF`] literal floating-point number.
///
/// [`XML-REF`]: https://en.wikipedia.org/wiki/XML
#[rustfmt::skip]
pub const XML: Options = Options::builder()
    .inf_string(options::XML_INF)
    .build_strict();

/// Number format for a [`SQLite`] literal floating-point number.
///
/// [`SQLite`]: https://www.sqlite.org/
#[rustfmt::skip]
pub const SQLITE: Options = Options::builder()
    .nan_string(options::SQLITE)
    .inf_string(options::SQLITE)
    .build_strict();

/// Number format for a [`PostgreSQL`] literal floating-point number.
///
/// [`PostgreSQL`]: https://www.postgresql.org/
#[rustfmt::skip]
pub const POSTGRESQL: Options = Options::builder()
    .nan_string(options::POSTGRESQL)
    .inf_string(options::POSTGRESQL)
    .build_strict();

/// Number format for a [`MySQL`] literal floating-point number.
///
/// [`MySQL`]: https://www.mysql.com/
#[rustfmt::skip]
pub const MYSQL: Options = Options::builder()
    .nan_string(options::MYSQL)
    .inf_string(options::MYSQL)
    .build_strict();

/// Number format for a [`MongoDB`] literal floating-point number.
///
/// [`MongoDB`]: https://www.mongodb.com/
#[rustfmt::skip]
pub const MONGODB: Options = Options::builder()
    .inf_string(options::MONGODB_INF)
    .build_strict();
