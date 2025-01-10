//! Configuration options for numeric conversions.
//!
//! This enables extensive control over how numbers are parsed and written,
//! from control characters like the decimal point and the valid non-finite
//! float representations.
//!
//! <!-- TODO: Add examples -->
//!
//! # Pre-Defined Formats
//!
//! <!-- TODO: Add pre-defined formats -->
//!
//! <!-- References -->
//! <!-- TODO: Add references -->
//!
//! The following constants have the following signifiers:
//!
//! - `${X}_LITERAL`: Applies to all literal values for that language (for
//!   example, [`RUST_LITERAL`]).
//! - `${X}_STRING`: Applies to all string values for that language (for
//!   example, [`ERLANG_STRING`]).
//! - `${X}`: Applies to all values for that language (for example, [`KAWA`]).
//! - `${X}_(NAN|INF|INFINITY)`: Applies to only a single special value (for
//!   example, [`PHP_LITERAL_NAN`], [`PHP_LITERAL_INF`], and
//!   [`PHP_LITERAL_INFINITY`]).
//!
//! If it's not defined, all values are the default. The default options
//! are:
//! - NaN: (`*_NAN`): `NaN`
//! - Short infinity: (`*_INF`): `Inf` (including `+Inf` and `-Inf`)
//! - Long infinity: (`*_INFINITY`): `Infinity` (including `+Infinity` and
//!   `-Infinity`)

// FIXME: Make the literals private in 2.0 and use the options-only API.
use core::num;

use crate::ascii::{is_valid_ascii, is_valid_letter_slice};
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
use crate::constants::{FormattedSize, NumberType};
use crate::error::Error;
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
use crate::format::NumberFormat;
use crate::result::Result;

// TRAITS
// ------

#[doc(hidden)]
#[macro_export]
macro_rules! write_options_doc {
    () => {
        "
Get an upper bound on the required buffer size.

<div class=\"warning\">

This method is soft-deprecated and meant for internal use.
You should always use [`buffer_size_const`] so you can get
the required buffer size at compile time to determine the
buffer size required.

</div>

[`buffer_size_const`]: Self::buffer_size_const

This is used when custom formatting options, such as significant
digits specifiers or custom exponent breaks, are used, which
can lead to more or less significant digits being written than
expected. If using the default formatting options, then this will
always be [`FORMATTED_SIZE`][FormattedSize::FORMATTED_SIZE] or
[`FORMATTED_SIZE_DECIMAL`][FormattedSize::FORMATTED_SIZE_DECIMAL],
depending on the radix.
"
    };
}

/// Shared trait for all writer options.
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub trait WriteOptions: Default {
    /// Determine if the options are valid.
    fn is_valid(&self) -> bool;

    /// Get an upper bound on the required buffer size.
    ///
    /// <div class="warning">
    ///
    /// This method is soft-deprecated and meant for internal use.
    /// You should always use `buffer_size_const` for either [`integer`] or
    /// [`float`] writer so you can get the required buffer size at compile time
    /// to determine the buffer size required.
    ///
    /// </div>
    ///
    /// [`integer`]: https://docs.rs/lexical-write-integer/latest/lexical_write_integer/struct.Options.html#method.buffer_size_const
    /// [`float`]: https://docs.rs/lexical-write-float/latest/lexical_util/struct.Options.html#method.buffer_size_const
    ///
    /// This is used when custom formatting options, such as significant
    /// digits specifiers or custom exponent breaks, are used, which
    /// can lead to more or less significant digits being written than
    /// expected. If using the default formatting options, then this will
    /// always be [`FORMATTED_SIZE`][FormattedSize::FORMATTED_SIZE] or
    /// [`FORMATTED_SIZE_DECIMAL`][FormattedSize::FORMATTED_SIZE_DECIMAL],
    /// depending on the radix.
    ///
    /// Using `buffer_size_const` lets you create static arrays at compile time,
    /// rather than dynamically-allocate memory or know the value ahead of time.
    #[deprecated = "Use `buffer_size_const` instead. Will be removed in 2.0."]
    fn buffer_size<T: FormattedSize, const FORMAT: u128>(&self) -> usize;
}

/// Shared trait for all parser options.
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub trait ParseOptions: Default {
    /// Determine if the options are valid.
    fn is_valid(&self) -> bool;
}

/// Type with the exact same size as a `usize`.
#[doc(hidden)]
pub type OptionUsize = Option<num::NonZeroUsize>;

/// Type with the exact same size as a `i32`.
#[doc(hidden)]
pub type OptionI32 = Option<num::NonZeroI32>;

/// Const evaluation of `max` for integers.
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
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
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoundMode {
    /// Round to the nearest float string with the given number of significant
    /// digits.
    Round,

    /// Truncate the float string with the given number of significant digits.
    Truncate,
}

/// Maximum length for a special string.
pub const MAX_SPECIAL_STRING_LENGTH: usize = 50;
const MAX_SPECIAL: usize = MAX_SPECIAL_STRING_LENGTH;

/// Builder for [`Options`].
///
/// This enables extensive control over how numbers are parsed and written,
/// from control characters like the decimal point and the valid non-finite
/// float representations.
///
/// <!-- TODO: Add examples -->
///
/// TODO:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionsBuilder {
    // INTEGER
    // -------

    // PARSE INTEGER
    /// Disable multi-digit optimizations.
    ///
    /// Using multi-digit optimizations allows parsing many digits
    /// from longer input strings at once which can dramatically
    /// improve performance (>70%) for long strings, but the
    /// increased branching can decrease performance for simple
    /// strings by 5-20%. Choose based on your inputs.
    no_multi_digit: bool,

    // WRITE INTEGER
    // N/A, none currently exist

    // FLOAT
    // -----
    /// Character to designate the exponent component of a float.
    exponent: u8,

    /// Character to separate the integer from the fraction components.
    decimal_point: u8,

    // WRITE FLOAT
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

    /// String representation of Not A Number, aka `NaN`, for float writing.
    write_nan_string: Option<&'static [u8]>,

    /// String representation of `Infinity` for float writing.
    write_inf_string: Option<&'static [u8]>,

    // PARSE FLOAT
    /// Disable the use of arbitrary-precision arithmetic, and always
    /// return the results from the fast or intermediate path algorithms.
    lossy: bool,

    /// Short string representation of Not A Number, aka `NaN`, for float
    /// parsing.
    parse_nan_string: Option<&'static [u8]>,

    /// Short string representation of `Infinity` for float parsing.
    parse_inf_string: Option<&'static [u8]>,

    /// Long string representation of `Infinity` for float parsing.
    parse_infinity_string: Option<&'static [u8]>,
}

impl OptionsBuilder {
    /// Create new [`OptionsBuilder`] with default options.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            no_multi_digit: true,
            exponent: b'e',
            decimal_point: b'.',
            max_significant_digits: None,
            min_significant_digits: None,
            positive_exponent_break: None,
            negative_exponent_break: None,
            round_mode: RoundMode::Round,
            trim_floats: false,
            write_nan_string: Some(b"NaN"),
            write_inf_string: Some(b"inf"),
            lossy: false,
            parse_nan_string: Some(b"NaN"),
            parse_inf_string: Some(b"inf"),
            parse_infinity_string: Some(b"infinity"),
        }
    }

    // GETTERS - INTEGER

    // GETTERS - PARSE INTEGER

    /// Get if we disable the use of multi-digit optimizations.
    ///
    /// Defaults to [`true`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_no_multi_digit(), true);
    /// ```
    #[inline(always)]
    pub const fn get_no_multi_digit(&self) -> bool {
        self.no_multi_digit
    }

    // GETTERS - WRITE INTEGER
    // N/A, none currently exist

    // GETTERS - FLOAT

    /// Get the character to designate the exponent component of a float.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `e`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::builder().get_exponent(), b'e');
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
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::builder().get_decimal_point(), b'.');
    /// ```
    #[inline(always)]
    pub const fn get_decimal_point(&self) -> u8 {
        self.decimal_point
    }

    // GETTERS - PARSE FLOAT

    /// Get if we disable the use of arbitrary-precision arithmetic.
    ///
    /// Lossy algorithms never use the fallback, slow algorithm. Defaults to
    /// [`false`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::builder().get_lossy(), false);
    /// ```
    #[inline(always)]
    pub const fn get_lossy(&self) -> bool {
        self.lossy
    }

    /// Get the string representation for `NaN` for float parsing.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_parse_nan_string(), Some("NaN".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_parse_nan_string(&self) -> Option<&'static [u8]> {
        self.parse_nan_string
    }

    /// Get the short string representation for `Infinity` for float parsing.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_parse_inf_string(), Some("inf".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_parse_inf_string(&self) -> Option<&'static [u8]> {
        self.parse_inf_string
    }

    /// Get the long string representation for `Infinity` for float parsing.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). Defaults to `infinity`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_parse_infinity_string(), Some("infinity".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_parse_infinity_string(&self) -> Option<&'static [u8]> {
        self.parse_infinity_string
    }

    // GETTERS - WRITE FLOAT

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
    /// use lexical_util::options::Options;
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
    /// use lexical_util::options::Options;
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
    /// use lexical_util::options::Options;
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
    /// use lexical_util::options::Options;
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
    /// use lexical_util::options::{Options, RoundMode};
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
    /// use lexical_util::options::Options;
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

    /// Get the string representation for `NaN` for float writing.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`).  If set to `None`, then writing
    /// [`NaN`][f64::NAN] leads to an error. Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_write_nan_string(), Some("NaN".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_write_nan_string(&self) -> Option<&'static [u8]> {
        self.write_nan_string
    }

    /// Get the string representation for `Infinity` for float writing.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_write_inf_string(), Some("inf".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_write_inf_string(&self) -> Option<&'static [u8]> {
        self.write_inf_string
    }

    /// Get the string representation for `Infinity` for float writing.
    /// Alias for [`get_write_inf_string`].
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// [`get_write_inf_string`]: Self::get_write_inf_string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder();
    /// assert_eq!(builder.get_write_infinity_string(), Some("inf".as_bytes()));
    /// ```
    #[inline(always)]
    pub const fn get_write_infinity_string(&self) -> Option<&'static [u8]> {
        self.write_inf_string
    }

    // SETTERS - INTEGER

    // SETTERS - PARSE INTEGER

    /// Set if we disable the use of multi-digit optimizations.
    ///
    /// Defaults to [`true`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .no_multi_digit(false)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.get_no_multi_digit(), false);
    /// ```
    #[inline(always)]
    pub const fn no_multi_digit(mut self, no_multi_digit: bool) -> Self {
        self.no_multi_digit = no_multi_digit;
        self
    }

    // SETTERS - WRITE INTEGER
    // N/A, none currently exist

    // SETTERS - FLOAT

    /// Set the character to designate the exponent component of a float.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `e`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use core::num;
    /// use lexical_util::options::Options;
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
    /// # use core::num;
    /// use lexical_util::options::Options;
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

    // SETTERS - PARSE FLOAT

    /// Set if we disable the use of arbitrary-precision arithmetic.
    ///
    /// Lossy algorithms never use the fallback, slow algorithm. Defaults to
    /// [`false`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .lossy(true)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.lossy(), true);
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn lossy(mut self, lossy: bool) -> Self {
        self.lossy = lossy;
        self
    }

    /// Set the string representation for `NaN`.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then parsing
    /// [`NaN`][f64::NAN] returns an error. Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .parse_nan_string(Some(b"nan"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.parse_nan_string(), Some(b"nan".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements will panic at runtime. You
    /// should always build the format using [`build_strict`] or checking
    /// [`is_valid`] prior to using the format, to avoid unexpected panics.
    ///
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[must_use]
    #[inline(always)]
    pub const fn parse_nan_string(mut self, nan_string: Option<&'static [u8]>) -> Self {
        self.parse_nan_string = nan_string;
        self
    }

    /// Set the short string representation for `Infinity`.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then parsing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .parse_inf_string(Some(b"Infinity"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.parse_inf_string(), Some(b"Infinity".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements or one that is longer than
    /// [`parse_infinity_string`] will panic at runtime. You should always
    /// build the format using [`build_strict`] or checking [`is_valid`] prior
    /// to using the format, to avoid unexpected panics.
    ///
    /// [`parse_infinity_string`]: Self::parse_infinity_string
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[must_use]
    #[inline(always)]
    pub const fn parse_inf_string(mut self, inf_string: Option<&'static [u8]>) -> Self {
        self.parse_inf_string = inf_string;
        self
    }

    /// Set the long string representation for `Infinity`.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then parsing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `infinity`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .parse_infinity_string(Some(b"Infinity"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.parse_infinity_string(), Some(b"Infinity".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements or one that is shorter than
    /// [`parse_inf_string`] will panic at runtime. You should always build the
    /// format using [`build_strict`] or checking [`is_valid`] prior to
    /// using the format, to avoid unexpected panics.
    ///
    /// [`parse_inf_string`]: Self::parse_inf_string
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[must_use]
    #[inline(always)]
    pub const fn parse_infinity_string(mut self, infinity_string: Option<&'static [u8]>) -> Self {
        self.parse_infinity_string = infinity_string;
        self
    }

    // SETTERS - WRITE FLOAT

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
    /// # use core::num;
    /// use lexical_util::options::Options;
    ///
    /// let max_digits = num::NonZeroUsize::new(300);
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
    /// # use core::num;
    /// use lexical_util::options::Options;
    ///
    /// let min_digits = num::NonZeroUsize::new(10);
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
    /// # use core::num;
    /// use lexical_util::options::Options;
    ///
    /// let pos_break = num::NonZeroI32::new(3);
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
    /// # use core::num;
    /// use lexical_util::options::Options;
    ///
    /// let neg_break = num::NonZeroI32::new(-3);
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
    /// # #[cfg(feature = "write-floats")] {
    /// # use core::num;
    /// use lexical_util::format::STANDARD;
    /// use lexical_util::options::{RoundMode, Options};
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
    /// assert_eq!(TRUNCATE_SIZE, 64);
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
    /// assert_eq!(ROUND_SIZE, 64);
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
    /// use lexical_util::options::Options;
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

    /// Set the string representation for `NaN` for writing floats.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`NaN`][f64::NAN] returns an error. Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder()
    ///     .write_nan_string(Some(b"nan"));
    /// assert_eq!(builder.get_write_nan_string(), Some(b"nan".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements will panic at runtime. You
    /// should always build the format using [`build_strict`] or checking
    /// [`is_valid`] prior to using the format, to avoid unexpected panics.
    ///
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[inline(always)]
    pub const fn write_nan_string(mut self, nan_string: Option<&'static [u8]>) -> Self {
        self.write_nan_string = nan_string;
        self
    }

    /// Set the string representation for `Infinity` for writing floats.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder()
    ///     .write_inf_string(Some(b"infinity"));
    /// assert_eq!(builder.get_write_inf_string(), Some(b"infinity".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements will panic at runtime. You
    /// should always build the format using [`build_strict`] or checking
    /// [`is_valid`] prior to using the format, to avoid unexpected panics.
    ///
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[inline(always)]
    pub const fn write_inf_string(mut self, inf_string: Option<&'static [u8]>) -> Self {
        self.write_inf_string = inf_string;
        self
    }

    /// Set the string representation for `Infinity` for writing floats. Alias
    /// for [`write_inf_string`].
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// [`write_inf_string`]: Self::write_inf_string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// let builder = Options::builder()
    ///     .write_infinity_string(Some(b"infinity"));
    /// assert_eq!(builder.get_write_infinity_string(), Some(b"infinity".as_ref()));
    /// ```
    ///
    /// Panics
    ///
    /// Setting a value with more than 50 elements will panic at runtime. You
    /// should always build the format using [`build_strict`] or checking
    /// [`is_valid`] prior to using the format, to avoid unexpected panics.
    ///
    /// [`build_strict`]: Self::build_strict
    /// [`is_valid`]: Self::is_valid
    #[inline(always)]
    pub const fn write_infinity_string(self, inf_string: Option<&'static [u8]>) -> Self {
        self.write_inf_string(inf_string)
    }

    // BUILDERS

    /// Determine if the provided NaN string is valid.
    #[inline(always)]
    const fn nan_is_valid(nan: &[u8]) -> Option<Error> {
        if nan.is_empty() || !matches!(nan[0], b'N' | b'n') {
            Some(Error::InvalidNanString)
        } else if !is_valid_letter_slice(nan) {
            Some(Error::InvalidNanString)
        } else if nan.len() > MAX_SPECIAL_STRING_LENGTH {
            Some(Error::NanStringTooLong)
        } else {
            None
        }
    }

    /// Determine if the provided NaN string is valid.
    #[inline(always)]
    const fn nanopt_is_valid(nan: Option<&[u8]>) -> Option<Error> {
        match nan {
            Some(v) => Self::nan_is_valid(v),
            None => None,
        }
    }

    /// Determine if the provided short Infinity string is valid.
    #[inline(always)]
    const fn inf_is_valid(inf: &[u8], max_len: usize) -> Option<Error> {
        if inf.is_empty() || !matches!(inf[0], b'I' | b'i') {
            Some(Error::InvalidInfString)
        } else if !is_valid_letter_slice(inf) {
            Some(Error::InvalidInfString)
        } else if inf.len() > max_len {
            Some(Error::InfStringTooLong)
        } else {
            None
        }
    }

    /// Determine if the provided short Infinity string is valid.
    #[inline(always)]
    const fn infopt_is_valid(inf: Option<&[u8]>, max_len: usize) -> Option<Error> {
        match inf {
            Some(v) => Self::inf_is_valid(v, max_len),
            None => None,
        }
    }

    /// Determine if the provided long Infinity string is valid.
    #[inline(always)]
    const fn infinity_is_valid(long: &[u8], min_len: usize) -> Option<Error> {
        if long.is_empty() || !matches!(long[0], b'I' | b'i') {
            Some(Error::InvalidInfinityString)
        } else if !is_valid_letter_slice(long) {
            Some(Error::InvalidInfinityString)
        } else if long.len() < min_len {
            Some(Error::InfinityStringTooShort)
        } else if long.len() > MAX_SPECIAL {
            Some(Error::InfinityStringTooLong)
        } else {
            None
        }
    }

    /// Get the [`Error`] associated with the [`Options`].
    ///
    /// If the options are valid, it returns [`Error::Success`], otherwise it
    /// returns a diagnostic error.
    #[inline(always)]
    pub const fn error(&self) -> Error {
        // integral fields
        // NOTE: The integral components are always valid

        // shared float fields
        if !is_valid_ascii(self.exponent) {
            return Error::InvalidExponentSymbol;
        } else if !is_valid_ascii(self.decimal_point) {
            return Error::InvalidDecimalPoint;
        }

        // parse float fields: validate our special strings
        if let Some(error) = Self::nanopt_is_valid(self.parse_nan_string) {
            return error;
        }
        match (self.parse_inf_string, self.parse_infinity_string) {
            // both None, valid
            (None, None) => (),
            // no long representation but have short, cannot be valid
            (Some(_), None) => return Error::InfinityStringTooShort,
            // no lower bound (must be 1 character, make sure it's not too long)
            (None, Some(long)) => {
                if let Some(error) = Self::infinity_is_valid(long, 1) {
                    return error;
                }
            },
            // need to bound the two: `inf <= infinity` and both need to be valid
            (Some(short), Some(long)) => {
                if let Some(error) = Self::inf_is_valid(short, long.len()) {
                    return error;
                } else if let Some(error) = Self::infinity_is_valid(long, short.len()) {
                    return error;
                }
            },
        }

        // write float fields
        if let Some(error) = Self::nanopt_is_valid(self.write_nan_string) {
            return error;
        } else if let Some(error) = Self::infopt_is_valid(self.write_inf_string, MAX_SPECIAL) {
            return error;
        }
        match (self.min_significant_digits, self.max_significant_digits) {
            (Some(min), Some(max)) if max.get() < min.get() => return Error::InvalidFloatPrecision,
            _ => (),
        }
        match self.negative_exponent_break {
            Some(v) if v.get() > 0 => return Error::InvalidNegativeExponentBreak,
            _ => (),
        }
        match self.positive_exponent_break {
            Some(v) if v.get() < 0 => return Error::InvalidPositiveExponentBreak,
            _ => (),
        }

        Error::Success
    }

    /// Check if the builder state is valid (always [`true`]).
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        match self.error() {
            Error::Success => true,
            _ => false,
        }
    }

    /// Build the [`Options`] struct without validation.
    ///
    /// <div class="warning">
    ///
    /// This is safe, however, misusing this, especially the special string
    /// representations could cause panics at runtime. Always use
    /// [`is_valid`] prior to using the built options or prefer using
    /// [`build_strict`].
    ///
    /// </div>
    ///
    /// [`is_valid`]: Self::is_valid
    /// [`build_strict`]: Self::build_strict
    #[inline(always)]
    pub const fn build_unchecked(&self) -> Options {
        Options {
            no_multi_digit: self.no_multi_digit,
            exponent: self.exponent,
            decimal_point: self.decimal_point,
            max_significant_digits: self.max_significant_digits,
            min_significant_digits: self.min_significant_digits,
            positive_exponent_break: self.positive_exponent_break,
            negative_exponent_break: self.negative_exponent_break,
            round_mode: self.round_mode,
            trim_floats: self.trim_floats,
            write_nan_string: self.write_nan_string,
            write_inf_string: self.write_inf_string,
            lossy: self.lossy,
            parse_nan_string: self.parse_nan_string,
            parse_inf_string: self.parse_inf_string,
            parse_infinity_string: self.parse_infinity_string,
        }
    }

    /// Build the [`Options`] struct, panicking if the builder is invalid.
    ///
    /// # Panics
    ///
    /// If the built options are not valid.
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
    pub const fn build(&self) -> Result<Options> {
        let error = self.error();
        match error {
            Error::Success => Ok(self.build_unchecked()),
            _ => Err(error),
        }
    }
}

impl Default for OptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// TODO:
pub struct Options {
    // INTEGER
    // -------

    // PARSE INTEGER
    /// Disable multi-digit optimizations.
    ///
    /// Using multi-digit optimizations allows parsing many digits
    /// from longer input strings at once which can dramatically
    /// improve performance (>70%) for long strings, but the
    /// increased branching can decrease performance for simple
    /// strings by 5-20%. Choose based on your inputs.
    no_multi_digit: bool,

    // WRITE INTEGER
    // N/A, none currently exist

    // FLOAT
    // -----
    /// Character to designate the exponent component of a float.
    exponent: u8,

    /// Character to separate the integer from the fraction components.
    decimal_point: u8,

    // WRITE FLOAT
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

    /// String representation of Not A Number, aka `NaN`, for float writing.
    write_nan_string: Option<&'static [u8]>,

    /// String representation of `Infinity` for float writing.
    write_inf_string: Option<&'static [u8]>,

    // PARSE FLOAT
    /// Disable the use of arbitrary-precision arithmetic, and always
    /// return the results from the fast or intermediate path algorithms.
    lossy: bool,

    /// Short string representation of Not A Number, aka `NaN`, for float
    /// parsing.
    parse_nan_string: Option<&'static [u8]>,

    /// Short string representation of `Infinity` for float parsing.
    parse_inf_string: Option<&'static [u8]>,

    /// Long string representation of `Infinity` for float parsing.
    parse_infinity_string: Option<&'static [u8]>,
}

impl Options {
    /// Create [`Options`] with default values.
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self::builder().build_unchecked()
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
    /// # #[cfg(all(feature = "format", feature = "write-floats", feature = "radix"))] {
    /// # use core::num;
    /// use lexical_util::{FormattedSize, Options};
    /// use lexical_util::format::STANDARD;
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
    /// # }
    /// ```
    #[inline(always)]
    #[cfg(any(feature = "write-floats", feature = "write-integers"))]
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

        // Integers currently do not have any extra spacing requirements.
        if matches!(T::NUMBER_TYPE, NumberType::Integer) {
            return formatted_size;
        }

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

    /// Check if the builder state is valid (always [`true`]).
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.rebuild().is_valid()
    }

    // GETTERS - INTEGER

    // GETTERS - PARSE INTEGER

    /// Get if we disable the use of multi-digit optimizations.
    ///
    /// Defaults to [`true`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .no_multi_digit(true)
    ///     .build_strict();
    /// assert_eq!(OPTIONS.get_no_multi_digit(), true);
    /// ```
    #[inline(always)]
    pub const fn get_no_multi_digit(&self) -> bool {
        self.no_multi_digit
    }

    // GETTERS - WRITE INTEGER
    // N/A, none currently exist

    // GETTERS - FLOAT

    /// Get the character to designate the exponent component of a float.
    ///
    /// Any non-control character is valid, but `\t` to `\r` are also valid.
    /// The full range is `[0x09, 0x0D]` and `[0x20, 0x7F]`. Defaults to `e`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::new().exponent(), b'e');
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
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::new().decimal_point(), b'.');
    /// ```
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        self.decimal_point
    }

    // GETTERS - PARSE FLOAT

    /// Get if we disable the use of arbitrary-precision arithmetic.
    ///
    /// Lossy algorithms never use the fallback, slow algorithm. Defaults to
    /// [`false`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::new().lossy(), false);
    /// ```
    #[inline(always)]
    pub const fn lossy(&self) -> bool {
        self.lossy
    }

    /// Get the string representation for `NaN` for parsing floats.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then parsing
    /// [`NaN`][f64::NAN] returns an error. Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::new().parse_nan_string(), Some(b"NaN".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn parse_nan_string(&self) -> Option<&'static [u8]> {
        self.parse_nan_string
    }

    /// Get the short string representation for `Infinity` for parsing floats.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then parsing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::new().parse_inf_string(), Some(b"inf".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn parse_inf_string(&self) -> Option<&'static [u8]> {
        self.parse_inf_string
    }

    /// Get the long string representation for `Infinity` for parsing floats.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then parsing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `infinity`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// assert_eq!(Options::new().parse_infinity_string(), Some(b"infinity".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn parse_infinity_string(&self) -> Option<&'static [u8]> {
        self.parse_infinity_string
    }

    // GETTERS - WRITE FLOAT

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
    /// # use core::num;
    /// use lexical_util::options::Options;
    ///
    /// const MAX_DIGITS: Option<num::NonZeroUsize> = num::NonZeroUsize::new(300);
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
    /// # use core::num;
    /// use lexical_util::options::Options;
    ///
    /// const MIN_DIGITS: Option<num::NonZeroUsize> = num::NonZeroUsize::new(10);
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
    /// # use core::num;
    /// use lexical_util::options::Options;
    ///
    /// const POS_BREAK: Option<num::NonZeroI32> = num::NonZeroI32::new(3);
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
    /// # use core::num;
    /// use lexical_util::options::Options;
    ///
    /// const NEG_BREAK: Option<num::NonZeroI32> = num::NonZeroI32::new(-3);
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
    /// use lexical_util::options::{Options, RoundMode};
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
    /// use lexical_util::options::Options;
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

    /// Get the string representation for `NaN` for writing floats.
    ///
    /// The first character must start with `N` or `n` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`NaN`][f64::NAN] returns an error. Defaults to `NaN`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .write_nan_string(Some(b"nan"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.write_nan_string(), Some(b"nan".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn write_nan_string(&self) -> Option<&'static [u8]> {
        self.write_nan_string
    }

    /// Get the string representation for `Infinity` for writing floats.
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .write_inf_string(Some(b"infinity"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.write_inf_string(), Some(b"infinity".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn write_inf_string(&self) -> Option<&'static [u8]> {
        self.write_inf_string
    }

    /// Get the string representation for `Infinity` for writing floats. Alias
    /// for [`write_inf_string`].
    ///
    /// The first character must start with `I` or `i` and all characters must
    /// be valid ASCII letters (`A-Z` or `a-z`). If set to `None`, then writing
    /// [`Infinity`][f64::INFINITY] returns an error. Defaults to `inf`.
    ///
    /// [`write_inf_string`]: Self::write_inf_string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lexical_util::options::Options;
    ///
    /// const OPTIONS: Options = Options::builder()
    ///     .write_infinity_string(Some(b"infinity"))
    ///     .build_strict();
    /// assert_eq!(OPTIONS.write_infinity_string(), Some(b"infinity".as_ref()));
    /// ```
    #[inline(always)]
    pub const fn write_infinity_string(&self) -> Option<&'static [u8]> {
        self.write_inf_string
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
            no_multi_digit: self.no_multi_digit,
            exponent: self.exponent,
            decimal_point: self.decimal_point,
            max_significant_digits: self.max_significant_digits,
            min_significant_digits: self.min_significant_digits,
            positive_exponent_break: self.positive_exponent_break,
            negative_exponent_break: self.negative_exponent_break,
            round_mode: self.round_mode,
            trim_floats: self.trim_floats,
            write_nan_string: self.write_nan_string,
            write_inf_string: self.write_inf_string,
            lossy: self.lossy,
            parse_nan_string: self.parse_nan_string,
            parse_inf_string: self.parse_inf_string,
            parse_infinity_string: self.parse_infinity_string,
        }
    }
}

impl Default for Options {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
impl ParseOptions for Options {
    #[inline(always)]
    fn is_valid(&self) -> bool {
        Self::is_valid(self)
    }
}

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
impl WriteOptions for Options {
    #[inline(always)]
    fn is_valid(&self) -> bool {
        Self::is_valid(self)
    }

    #[doc = write_options_doc!()]
    #[inline(always)]
    fn buffer_size<T: FormattedSize, const FORMAT: u128>(&self) -> usize {
        self.buffer_size_const::<T, FORMAT>()
    }
}

// PRE-DEFINED CONSTANTS
// ---------------------

// The following constants have the following signifiers:
//  ${X}_LITERAL - Applies to all literal values for that language.
//  ${X}_STRING - Applies to all string values for that language.
//  ${X} - Applies to all values for that language.
//  ${X}_(NAN|INF|INFINITY) - Applies to only a single special value.
//  IF it's not defined, all values are the default.

macro_rules! literal {
    ($name:ident, $value:ident $(, $doc:literal)?) => {
        $(#[doc = $doc])?
        pub const $name: Option<&[u8]> = $value;
    };
    ($name:ident, $value:literal $(, $doc:literal)?) => {
        $(#[doc = $doc])?
        pub const $name: Option<&[u8]> = Some($value);
    };
}

literal!(RUST_LITERAL, None, "A Rust literal number (uses default options).");
// RUST_STRING
literal!(PYTHON_LITERAL, None, "A Python literal number (uses default options).");
// PYTHON_STRING
literal!(CXX_LITERAL_NAN, b"NAN", "A C++ literal NaN (`NAN`).");
literal!(CXX_LITERAL_INF, b"INFINITY", "A C++ literal short infinity (`INFINITY`).");
literal!(CXX_LITERAL_INFINITY, b"INFINITY", "A C++ literal long infinity (`INFINITY`).");
// CXX_STRING
literal!(C_LITERAL_NAN, b"NAN", "A C literal NaN (`NAN`).");
literal!(C_LITERAL_INF, b"INFINITY", "A C literal short infinity (`INFINITY`).");
literal!(C_LITERAL_INFINITY, b"INFINITY", "A C literal long infinity (`INFINITY`).");
// RUBY_LITERAL
literal!(RUBY_LITERAL_NAN, b"NaN", "A Ruby literal NaN (`NaN`).");
literal!(RUBY_LITERAL_INF, b"Infinity", "A C literal short infinity (`Infinity`).");
literal!(RUBY_STRING_NONE, None, "A Ruby string (uses default options).");
// C_STRING
literal!(SWIFT_LITERAL, None, "A Swift literal number (uses default options).");
// SWIFT_STRING
literal!(GO_LITERAL, None, "A Golang literal number (uses default options).");
// GO_STRING
literal!(HASKELL_LITERAL, None, "A Haskell literal number (uses default options).");
literal!(HASKELL_STRING_INF, b"Infinity", "A Haskell string short infinity (`Infinity`).");
literal!(HASKELL_STRING_INFINITY, b"Infinity", "A Haskell string long infinity (`Infinity`).");
literal!(JAVASCRIPT_INF, b"Infinity", "A JavaScript string short infinity (`Infinity`).");
literal!(JAVASCRIPT_INFINITY, b"Infinity", "A JavaScript string long infinity (`Infinity`).");
literal!(PERL_LITERAL, None, "A Perl literal literal (uses default options).");
// PERL_STRING
literal!(PHP_LITERAL_NAN, b"NAN", "A PHP literal NaN (`NAN`).");
literal!(PHP_LITERAL_INF, b"INF", "A PHP literal short infinity (`INF`).");
literal!(PHP_LITERAL_INFINITY, b"INF", "A PHP literal long infinity (`INF`).");
// PHP_STRING
literal!(JAVA_LITERAL, None, "A Java literal number (uses default options).");
literal!(JAVA_STRING_INF, b"Infinity", "A Java string short infinity (`Infinity`).");
literal!(JAVA_STRING_INFINITY, b"Infinity", "A Java string long infinity (`Infinity`).");
literal!(R_LITERAL_INF, b"Inf", "An R literal short infinity (`Inf`).");
literal!(R_LITERAL_INFINITY, b"Inf", "An R literal long infinity (`Inf`).");
// R_STRING
literal!(KOTLIN_LITERAL, None, "A Kotlin literal number (uses default options).");
literal!(KOTLIN_STRING_INF, b"Infinity", "A Kotlin string short infinity (`Infinity`).");
literal!(KOTLIN_STRING_INFINITY, b"Infinity", "A Kotlin string long infinity (`Infinity`).");
literal!(JULIA_LITERAL_INF, b"Inf", "A Julia string short infinity (`Inf`).");
literal!(JULIA_LITERAL_INFINITY, b"Inf", "A Julia string long infinity (`Inf`).");
// JULIA_STRING
literal!(CSHARP_LITERAL, None, "A C# literal number (uses default options).");
literal!(CSHARP_STRING_INF, b"Infinity", "A C# string short infinity (`Infinity`).");
literal!(CSHARP_STRING_INFINITY, b"Infinity", "A C# string long infinity (`Infinity`).");
literal!(KAWA, None, "A Kawa (List) literal number (uses default options).");
literal!(GAMBITC, None, "A Gambit-C (List) literal number (uses default options).");
literal!(GUILE, None, "A Guile (List) literal number (uses default options).");
literal!(CLOJURE_LITERAL, None, "A Clojure (Lisp) literal number (uses default options).");
literal!(CLOJURE_STRING_INF, b"Infinity", "A Clojure string short infinity (`Infinity`).");
literal!(CLOJURE_STRING_INFINITY, b"Infinity", "A Clojure string long infinity (`Infinity`).");
literal!(ERLANG_LITERAL_NAN, b"nan", "An Erlang literal NaN (`nan`).");
literal!(ERLANG_STRING, None, "An Erlang string number (uses default options).");
literal!(ELM_LITERAL, None, "An Elm literal number (uses default options).");
literal!(ELM_STRING_NAN, None, "An Elm stromg NaN (uses default options).");
literal!(ELM_STRING_INF, b"Infinity", "An Elm string short infinity (`Infinity`).");
literal!(ELM_STRING_INFINITY, b"Infinity", "An Elm string long infinity (`Infinity`).");
literal!(SCALA_LITERAL, None, "A Scala literal number (uses default options).");
literal!(SCALA_STRING_INF, b"Infinity", "A Scala string short infinity (`Infinity`).");
literal!(SCALA_STRING_INFINITY, b"Infinity", "A Scala string long infinity (`Infinity`).");
literal!(ELIXIR, None, "An Elixir number (uses default options).");
literal!(FORTRAN_LITERAL, None, "A FORTRAN literal number (uses default options).");
// FORTRAN_STRING
literal!(D_LITERAL, None, "A D-Lang literal number (uses default options).");
// D_STRING
literal!(COFFEESCRIPT_INF, b"Infinity", "A CoffeeScript string short infinity (`Infinity`).");
literal!(COFFEESCRIPT_INFINITY, b"Infinity", "A CoffeeScript string long infinity (`Infinity`).");
literal!(COBOL, None, "A COBOL literal number (uses default options).");
literal!(FSHARP_LITERAL_NAN, b"nan", "An F# literal NaN (`nan`).");
literal!(FSHARP_LITERAL_INF, b"infinity", "An F# literal short infinity (`infinity`).");
literal!(FSHARP_LITERAL_INFINITY, b"infinity", "An F# literal long infinity (`infinity`).");
// FSHARP_STRING
literal!(VB_LITERAL, None, "A Visual Basic literal number (uses default options)");
literal!(VB_STRING_INF, None, "A Visual Basic short string infinity (uses default options)");
literal!(VB_STRING_INFINITY, None, "A Visual Basic long string number (uses default options)");
literal!(OCAML_LITERAL_NAN, b"nan", "An OCAML literal NaN (`nan`).");
literal!(OCAML_LITERAL_INF, b"infinity", "An OCAML literal short infinity (`infinity`).");
literal!(OCAML_LITERAL_INFINITY, b"infinity", "An OCAML literal long infinity (`infinity`).");
// OCAML_STRING
literal!(OBJECTIVEC, None, "An Objective-C number (uses default options).");
literal!(REASONML_LITERAL_NAN, b"nan", "A ReasonML literal NaN (`nan`).");
literal!(REASONML_LITERAL_INF, b"infinity", "A ReasonML literal short infinity (`infinity`).");
literal!(REASONML_LITERAL_INFINITY, b"infinity", "A ReasonML literal long infinity (`infinity`).");
// REASONML_STRING
literal!(MATLAB_LITERAL_INF, b"inf", "A MATLAB literal short infinity (`inf`).");
literal!(MATLAB_LITERAL_INFINITY, b"Inf", "A MATLAB literal long infinity (`Inf`).");
// MATLAB_STRING
literal!(ZIG_LITERAL, None, "A Zig literal number (uses default options).");
// ZIG_STRING
literal!(SAGE_LITERAL_INF, b"infinity", "A SageMath literal short infinity (`infinity`).");
literal!(SAGE_LITERAL_INFINITY, b"Infinity", "A SageMath literal long infinity (`Infinity`).");
// SAGE_STRING
literal!(JSON, None, "A JSON number (uses default options).");
literal!(TOML, None, "A TOML number (uses default options).");
literal!(YAML, None, "A YAML number (uses default options).");
literal!(XML_INF, None, "An XML short infinity (uses default options).");
literal!(XML_INFINITY, None, "An XML short infinity (uses default options).");
literal!(SQLITE, None, "A SQLite number (uses default options).");
literal!(POSTGRESQL, None, "A PostgreSQL number (uses default options).");
literal!(MYSQL, None, "A MySQL number (uses default options).");
literal!(MONGODB_INF, b"Infinity", "A MongoDB short infinity (`Infinity`).");
literal!(MONGODB_INFINITY, b"Infinity", "A MongoDB long infinity (`Infinity`).");
