//! Fast lexical conversion routines.
//!
//! `lexical-core` is a high-performance library for number-to-string and
//! string-to-number conversions. The writers require a system allocator,
//! but support a [`no_std`] environment. In addition to high performance,
//! it's also highly configurable, supporting nearly every float and integer
//! format available.
//!
//! `lexical` is well-tested, and has been downloaded more than 25 million
//! times and currently has no known errors in correctness. `lexical`
//! prioritizes performance above all else, and is competitive or faster
//! than any other float or integer parser and writer.
//!
//! In addition, despite having a large number of features, configurability,
//! and a focus on performance, it also aims to have fast compile times.
//! Recent versions also add [`support`](#compact) for smaller binary sizes, as
//! well ideal for embedded or web environments, where executable bloat can
//! be much more detrimental than performance.
//!
//! [`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
//!
//! # Getting Started
//!
//! #### Parse API
//!
//! The main parsing API is [`parse`] and [`parse_partial`]. For example,
//! to parse a number from string, validating the entire input is a number:
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "parse-integers"))] {
//! let i: i32 = lexical::parse("3").unwrap();      // 3, auto-type deduction.
//! let f: f32 = lexical::parse("3.5").unwrap();    // 3.5
//! let d = lexical::parse::<f64, _>("3.5");        // Ok(3.5), successful parse.
//! # }
//! ```
//!
//! All `lexical` parsers are validating, they check the that entire input data
//! is correct, and stop parsing when invalid data is found, numerical overflow,
//! or other errors:
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "parse-integers"))] {
//! let r = lexical::parse::<u8, _>("256"); // Err(ErrorCode::Overflow.into())
//! let r = lexical::parse::<u8, _>("1a5"); // Err(ErrorCode::InvalidDigit.into())
//! # }
//! ```
//!
//! For streaming APIs or those incrementally parsing data fed to a parser,
//! where the input data is known to be a float but where the float ends is
//! currently unknown, the partial parsers will both return the data it was
//! able to parse and the number of bytes processed:
//!
//! ```rust
//! # #[cfg(feature = "parse-integers")] {
//! let r = lexical::parse_partial::<i8, _>("3a5"); // Ok((3, 1))
//! # }
//! ```
//!
//! #### Write API
//!
//! The main parsing API is [`to_string`]. For example, to write a number to
//! string:
//!
//! ```rust
//! # #[cfg(feature = "write-floats")] {
//! let value = lexical::to_string(15.1);
//! assert_eq!(value, "15.1");
//! # }
//! ```
//!
//! # Conversion API
//!
//! This writes and parses numbers to and from a format identical to
//! Rust's [`parse`][`core-parse`] and [`write`][`core-write`].
//!
//! [`core-parse`]: core::str::FromStr::from_str
//! [`core-write`]: core::fmt::Display::fmt
//!
//! <!-- Spacer for rustfmt -->
#![cfg_attr(
    any(feature = "write-floats", feature = "write-integers"),
    doc = "- [`to_string`]:  Write a number to string."
)]
#![cfg_attr(
    any(feature = "parse-floats", feature = "parse-integers"),
    doc = "
- [`parse`]: Parse a number from string validating the complete string is a number.
- [`parse_partial`]: Parse a number from string returning the number and the number
  of digits it was able to parse.
"
)]
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "write-floats"))] {
//! // parse
//! let f: f64 = lexical::parse(b"3.5").unwrap();
//! assert_eq!(f, 3.5);
//!
//! let (f, count): (f64, usize) = lexical::parse_partial(b"3.5").unwrap();
//! assert_eq!(f, 3.5);
//! assert_eq!(count, 3);
//!
//! // write
//! let value = lexical::to_string(f);
//! assert_eq!(value, "3.5");
//! # }
//! ```
//!
//! # Options/Formatting API
//!
//! Each number parser and writer contains extensive formatting control
//! through options and [`mod@format`] specifications, including digit
//! [`separator`] support (that is, numbers such as `1_2__3.4_5`), if
//! integral, fractional, or any significant digits are required, if to
//! disable parsing or writing of non-finite values, if `+` signs are
//! invalid or required, and much more.
//!
//! [`separator`]: NumberFormat::digit_separator
//!
//! <!-- Spacer for rustfmt -->
#![cfg_attr(
    feature = "write-floats",
    doc = "[`nan_string`]: WriteFloatOptionsBuilder::nan_string"
)]
#![cfg_attr(
    all(not(feature = "write-floats"), feature = "parse-floats"),
    doc = "[`nan_string`]: ParseFloatOptionsBuilder::nan_string"
)]
#![cfg_attr(
    all(not(feature = "write-floats"), not(feature = "parse-floats")),
    doc = "[`nan_string`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.nan_string"
)]
//!
//! <!-- Spacer for rustfmt -->
#![cfg_attr(
    any(feature = "write-floats", feature = "write-integers"),
    doc = "- [`to_string_with_options`]: Write a number to string using custom formatting options."
)]
#![cfg_attr(
    any(feature = "parse-floats", feature = "parse-integers"),
    doc = "
- [`parse_with_options`]: Parse a number from string using custom formatting options,
    validating the complete string is a number.
- [`parse_partial_with_options`]: Parse a number from string using custom formatting
    options, returning the number and the number of digits it was able to parse.
"
)]
//!
//! Some options, such as custom string representations of non-finite
//! floats (such as [`NaN`][`nan_string`]), are available without the
//! [`format`](crate#format) feature. For more comprehensive examples, see the
//! [`format`](#format) and [Comprehensive Configuration] sections
//! below.
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "write-floats", feature = "format"))] {
//! use lexical::{format, parse_float_options, write_float_options};
//!
//! // parse
//! let f: f64 = lexical::parse_with_options::<_, _, { format::JSON }>(
//!     "3.5",
//!     &parse_float_options::JSON
//! ).unwrap();
//!
//! // write
//! let value = lexical::to_string_with_options::<_, { format::JSON }>(
//!     f,
//!     &write_float_options::JSON
//! );
//! assert_eq!(value, "3.5");
//! # }
//! ```
//!
//! [Comprehensive Configuration]: #comprehensive-configuration
//!
//! # Features
//!
//! In accordance with the Rust ethos, all features are additive: the crate
//! may be build with `--all-features` without issue.  The following features
//! are enabled by default:
//!
//! * `write-integers` (Default) - Enable writing of integers.
//! * `write-floats` (Default) - Enable writing of floats.
//! * `parse-integers` (Default) - Enable parsing of integers.
//! * `parse-floats` (Default) - Enable parsing of floats.
//! * `radix` - Add support for strings of any radix.
//! * `compact` - Reduce code size at the cost of performance.
//! * `format` - Add support for custom number formatting.
//! * `f16` - Enable support for half-precision [`f16`][`ieee-f16`] and
//!   [`bf16`][`brain-float`] floats.
//! * `std` (Default) - Disable to allow use in a [`no_std`] environment.
//!
//! [`ieee-f16`]: https://en.wikipedia.org/wiki/Half-precision_floating-point_format
//! [`brain-float`]: https://en.wikipedia.org/wiki/Bfloat16_floating-point_format
//!
//! A complete description of supported features includes:
//!
//! #### write-integers
//!
//! Enable support for writing integers to string.
//!
//! ```rust
//! # #[cfg(feature = "write-integers")] {
//! let value = lexical::to_string(1234u64);
//! assert_eq!(value, "1234");
//! # }
//! ```
//!
//! #### write-floats
//!
//! Enable support for writing floating-point numbers to string.
//!
//! ```rust
//! # #[cfg(feature = "write-floats")] {
//! let value = lexical::to_string(1.234f64);
//! assert_eq!(value, "1.234");
//! # }
//! ```
//!
//! #### parse-integers
//!
//! Enable support for parsing integers from string.
//!
//! ```rust
//! # #[cfg(feature = "parse-integers")] {
//! let f: i64 = lexical::parse("1234").unwrap();
//! assert_eq!(f, 1234);
//! # }
//! ```
//!
//! #### parsing-floats
//!
//! Enable support for parsing floating-point numbers from string.
//!
//! ```rust
//! # #[cfg(feature = "parse-integers")] {
//! let f: f64 = lexical::parse("1.234").unwrap();
//! assert_eq!(f, 1.234);
//! # }
//! ```
//!
//! #### format
//!
//! Adds support for the entire format API (using [`NumberFormatBuilder`]).
//! This allows extensive configurability for parsing and writing numbers
//! in custom formats, with different valid syntax requirements.
//!
//! For example, in JSON, the following floats are valid or invalid:
//!
//! ```text
//! -1          // valid
//! +1          // invalid
//! 1           // valid
//! 1.          // invalid
//! .1          // invalid
//! 0.1         // valid
//! nan         // invalid
//! inf         // invalid
//! Infinity    // invalid
//! ```
//!
//! All of the finite numbers are valid in Rust, and Rust provides constants
//! for non-finite floats. In order to parse standard-conforming JSON floats
//! using lexical, you may use the following approach:
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "format"))] {
//! use lexical::{format, parse_with_options, ParseFloatOptions, Result};
//!
//! fn parse_json_float<Bytes: AsRef<[u8]>>(bytes: Bytes) -> Result<f64> {
//!     const OPTIONS: ParseFloatOptions = ParseFloatOptions::new();
//!     parse_with_options::<_, _, { format::JSON }>(bytes.as_ref(), &OPTIONS)
//! }
//! # }
//! ```
//!
//! Enabling the [`format`](crate#format) API significantly increases compile
//! times, however, it enables a large amount of customization in how floats are
//! written.
//!
//! #### power-of-two
//!
//! Enable doing numeric conversions to and from strings radixes that are powers
//! of two, that is, `2`, `4`, `8`, `16`, and `32`. This avoids most of the
//! overhead and binary bloat of the [`radix`](#radix) feature, while enabling
//! support for the most commonly-used radixes.
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "write-floats", feature = "power-of-two"))] {
//! use lexical::{
//!     ParseFloatOptions,
//!     WriteFloatOptions,
//!     NumberFormatBuilder
//! };
//!
//! // parse
//! const BINARY: u128 = NumberFormatBuilder::binary();
//! let value = "1.0011101111100111011011001000101101000011100101011";
//! let f: f64 = lexical::parse_with_options::<_, _, { BINARY }>(
//!     value,
//!     &ParseFloatOptions::new()
//! ).unwrap();
//!
//! // write
//! let result = lexical::to_string_with_options::<_, { BINARY }>(
//!     f,
//!     &WriteFloatOptions::new()
//! );
//! assert_eq!(result, value);
//! # }
//! ```
//!
//! #### radix
//!
//! Enable doing numeric conversions to and from strings for all radixes.
//! This requires more static storage than [`power-of-two`](#power-of-two),
//! and increases compile times, but can be quite useful
//! for esoteric programming languages which use duodecimal floats, for
//! example.
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "write-floats", feature = "radix"))] {
//! # use core::str;
//! use lexical::{
//!     ParseFloatOptions,
//!     WriteFloatOptions,
//!     NumberFormatBuilder
//! };
//!
//! // parse
//! const FORMAT: u128 = NumberFormatBuilder::from_radix(12);
//! let value = "1.29842830A44BAA2";
//! let f: f64 = lexical::parse_with_options::<_, _, { FORMAT }>(
//!     value,
//!     &ParseFloatOptions::new()
//! ).unwrap();
//!
//! // write
//! let result = lexical::to_string_with_options::<_, { FORMAT }>(
//!     f,
//!     &WriteFloatOptions::new()
//! );
//! assert_eq!(result, value);
//! # }
//! ```
//!
//! #### compact
//!
//! Reduce the generated code size at the cost of performance. This minimizes
//! the number of static tables, inlining, and generics used, drastically
//! reducing the size of the generated binaries.
//!
//! #### std
//!
//! Enable use of the standard library. Currently, the standard library
//! is not used, and may be disabled without any change in functionality
//! on stable.
//!
//! # Comprehensive Configuration
//!
//! `lexical` provides two main levels of configuration:
//! - The [`NumberFormatBuilder`], creating a packed struct with custom
//!   formatting options.
//! - The Options API.
//!
//! ## Number Format
//!
//! The number format class provides numerous flags to specify
//! number parsing or writing. When the `power-of-two` feature is
//! enabled, additional flags are added:
//! - The radix for the significant digits (default `10`).
//! - The radix for the exponent base (default `10`).
//! - The radix for the exponent digits (default `10`).
//!
//! When the [`format`](#format) feature is enabled, numerous other syntax and
//! digit separator flags are enabled, including:
//! - A digit separator character, to group digits for increased legibility.
//! - Whether leading, trailing, internal, and consecutive digit separators are
//!   allowed.
//! - Toggling required float components, such as digits before the decimal
//!   point.
//! - Toggling whether special floats are allowed or are case-sensitive.
//!
//! Many pre-defined constants therefore exist to simplify common use-cases,
//! including:
//! - [`JSON`], [`XML`], [`TOML`], [`YAML`], [`SQLite`], and many more.
//! - [`Rust`], [`Python`], [`C#`], [`FORTRAN`], [`COBOL`] literals and strings,
//!   and many more.
//!
//! For a list of all supported fields, see
//! [Fields][NumberFormatBuilder#fields-1].
//!
//! <!-- Spacer for rustfmt -->
#![cfg_attr(
    feature = "format",
    doc = "
[`JSON`]: format::JSON
[`XML`]: format::XML
[`TOML`]: format::TOML
[`YAML`]: format::YAML
[`SQLite`]: format::SQLITE
[`Rust`]: format::RUST_LITERAL
[`Python`]: format::PYTHON_LITERAL
[`C#`]: format::CSHARP_LITERAL
[`FORTRAN`]: format::FORTRAN_LITERAL
[`COBOL`]: format::COBOL_LITERAL
"
)]
#![cfg_attr(
    not(feature = "format"),
    doc = "
[`JSON`]: https://docs.rs/lexical/latest/lexical/format/constant.JSON.html
[`XML`]: https://docs.rs/lexical/latest/lexical/format/constant.XML.html
[`TOML`]: https://docs.rs/lexical/latest/lexical/format/constant.TOML.html
[`YAML`]: https://docs.rs/lexical/latest/lexical/format/constant.YAML.html
[`SQLite`]: https://docs.rs/lexical/latest/lexical/format/constant.SQLITE.html
[`Rust`]: https://docs.rs/lexical/latest/lexical/format/constant.RUST_LITERAL.html
[`Python`]: https://docs.rs/lexical/latest/lexical/format/constant.PYTHON_LITERAL.html
[`C#`]: https://docs.rs/lexical/latest/lexical/format/constant.CSHARP_LITERAL.html
[`FORTRAN`]: https://docs.rs/lexical/latest/lexical/format/constant.FORTRAN_LITERAL.html
[`COBOL`]: https://docs.rs/lexical/latest/lexical/format/constant.COBOL_LITERAL.html
"
)]
//!
//! ## Options API
//!
//! The Options API provides high-level options to specify number parsing
//! or writing, options not intrinsically tied to a number format.
//! For example, the Options API provides:
//!
//! - The [`exponent`][`write-float-exponent`] character (defaults to `b'e'` or `b'^'`, depending on the radix).
//! - The [`decimal point`][`write-float-decimal_point`] character (defaults to `b'.'`).
//! - Custom [`NaN`][f64::NAN] and [`Infinity`][f64::INFINITY] string
//!   [`representations`][`write-float-nan_string`].
//! - Whether to [`trim`][`write-float-trim_floats`] the fraction component from integral floats.
//! - The exponent [`break-point`][`write-float-positive_exponent_break`] for scientific notation.
//! - The [`maximum`][`write-float-max_significant_digits`] and [`minimum`][`write-float-min_significant_digits`] number of significant digits to write.
//! - The rounding [`mode`][`write-float-round_mode`] when truncating significant digits while writing.
//!
//! <!-- Spacer for Rustfmt -->
#![cfg_attr(
    feature = "write-floats",
    doc = "
[`write-float-exponent`]: WriteFloatOptionsBuilder::exponent
[`write-float-decimal_point`]: WriteFloatOptionsBuilder::decimal_point
[`write-float-nan_string`]: WriteFloatOptionsBuilder::nan_string
[`write-float-trim_floats`]: WriteFloatOptionsBuilder::trim_floats
[`write-float-positive_exponent_break`]: WriteFloatOptionsBuilder::positive_exponent_break
[`write-float-max_significant_digits`]: WriteFloatOptionsBuilder::max_significant_digits
[`write-float-min_significant_digits`]: WriteFloatOptionsBuilder::min_significant_digits
[`write-float-round_mode`]: WriteFloatOptionsBuilder::round_mode
"
)]
#![cfg_attr(
    not(feature = "write-floats"),
    doc = "
[`write-float-exponent`]: https://docs.rs/lexical/latest/lexical/struct.WriteFloatOptionsBuilder.html#method.exponent
[`write-float-decimal_point`]: https://docs.rs/lexical/latest/lexical/struct.WriteFloatOptionsBuilder.html#method.decimal_point
[`write-float-nan_string`]: https://docs.rs/lexical/latest/lexical/struct.WriteFloatOptionsBuilder.html#method.nan_string
[`write-float-trim_floats`]: https://docs.rs/lexical/latest/lexical/struct.WriteFloatOptionsBuilder.html#method.trim_floats
[`write-float-positive_exponent_break`]: https://docs.rs/lexical/latest/lexical/struct.WriteFloatOptionsBuilder.html#method.positive_exponent_break
[`write-float-max_significant_digits`]: https://docs.rs/lexical/latest/lexical/struct.WriteFloatOptionsBuilder.html#method.max_significant_digits
[`write-float-min_significant_digits`]: https://docs.rs/lexical/latest/lexical/struct.WriteFloatOptionsBuilder.html#method.min_significant_digits
[`write-float-round_mode`]: https://docs.rs/lexical/latest/lexical/struct.WriteFloatOptionsBuilder.html#method.round_mode
"
)]
//!
//! The available options are:
#![cfg_attr(feature = "parse-floats", doc = " - [`ParseFloatOptions`]")]
#![cfg_attr(feature = "parse-integers", doc = " - [`ParseIntegerOptions`]")]
#![cfg_attr(feature = "write-floats", doc = " - [`WriteFloatOptions`]")]
#![cfg_attr(feature = "write-integers", doc = " - [`WriteIntegerOptions`]")]
//!
//! In addition, pre-defined constants for each category of options may
//! be found in their respective modules, for example, [`JSON`][`JSON-OPTS`].
//!
//! <!-- Spacer for Rustfmt -->
#![cfg_attr(feature = "parse-floats", doc = "[`JSON-OPTS`]: parse_float_options::JSON")]
#![cfg_attr(
    not(feature = "parse-floats"),
    doc = "[`JSON-OPTS`]: https://docs.rs/lexical/latest/lexical/parse_float_options/constant.JSON.html"
)]
//!
//! ## Examples
//!
//! An example of creating your own options to parse European-style
//! numbers (which use commas as decimal points, and periods as digit
//! separators) is as follows:
//!
//! ```
//! # #[cfg(all(feature = "parse-floats", feature = "format"))] {
//! # use core::num;
//! // This creates a format to parse a European-style float number.
//! // The decimal point is a comma, and the digit separators (optional)
//! // are periods.
//! const EUROPEAN: u128 = lexical::NumberFormatBuilder::new()
//!     .digit_separator(num::NonZeroU8::new(b'.'))
//!     .build_strict();
//! const COMMA_OPTIONS: lexical::ParseFloatOptions = lexical::ParseFloatOptions::builder()
//!     .decimal_point(b',')
//!     .build_strict();
//! assert_eq!(
//!     lexical::parse_with_options::<f32, _, EUROPEAN>("300,10", &COMMA_OPTIONS),
//!     Ok(300.10)
//! );
//!
//! // Another example, using a pre-defined constant for JSON.
//! const JSON: u128 = lexical::format::JSON;
//! const JSON_OPTIONS: lexical::ParseFloatOptions = lexical::ParseFloatOptions::new();
//! assert_eq!(
//!     lexical::parse_with_options::<f32, _, JSON>("0e1", &JSON_OPTIONS),
//!     Ok(0.0)
//! );
//! assert_eq!(
//!     lexical::parse_with_options::<f32, _, JSON>("1E+2", &JSON_OPTIONS),
//!     Ok(100.0)
//! );
//! # }
//! ```
//!
//! # Version Support
//!
//! The minimum, standard, required version is [`1.63.0`][`rust-1.63.0`], for
//! const generic support. Older versions of lexical support older Rust
//! versions.
//!
//! # Algorithms
//!
//! - [Parsing Floats](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-float/docs/Algorithm.md)
//! - [Parsing Integers](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-integer/docs/Algorithm.md)
//! - [Writing Floats](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-float/docs/Algorithm.md)
//! - [Writing Integers](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Algorithm.md)
//!
//! # Benchmarks
//!
//! - [Parsing Floats](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-float/docs/Benchmarks.md)
//! - [Parsing Integers](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-integer/docs/Benchmarks.md)
//! - [Writing Floats](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-float/docs/Benchmarks.md)
//! - [Writing Integers](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Benchmarks.md)
//! - [Comprehensive Benchmarks](https://github.com/Alexhuszagh/lexical-benchmarks)
//!
//! A comprehensive analysis of lexical commits and their performance can be
//! found in [benchmarks].
//!
//! # Design
//!
//! - [Binary Size](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/BinarySize.md)
//! - [Build Timings](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/BuildTimings.md)
//! - [Digit Separators](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/DigitSeparators.md)
//!
//! # Safety
//!
//! There is no non-trivial unsafe behavior in [lexical][crate] itself,
//! however, any incorrect safety invariants in our parsers and writers
//! (`lexical-parse-float`, `lexical-parse-integer`, `lexical-write-float`,
//! and `lexical-write-integer`) could cause those safety invariants to
//! be broken.
//!
//! <!-- Space for Rustfmt -->
#![cfg_attr(
    any(feature = "write-floats", feature = "write-integers"),
    doc = "
[`to_string`]: crate::to_string
[`to_string_with_options`]: crate::to_string_with_options
"
)]
#![cfg_attr(
    not(any(feature = "write-floats", feature = "write-integers")),
    doc = "
[`to_string`]: https://docs.rs/lexical/latest/lexical/fn.to_string.html
[`to_string_with_options`]: https://docs.rs/lexical/latest/lexical/fn.to_string_with_options.html
"
)]
#![cfg_attr(
    any(feature = "parse-floats", feature = "parse-integers"),
    doc = "
[`parse`]: crate::parse
[`parse_partial`]: crate::parse_partial
[`parse_with_options`]: crate::parse_with_options
[`parse_partial_with_options`]: crate::parse_partial_with_options
"
)]
#![cfg_attr(
    not(any(feature = "parse-floats", feature = "parse-integers")),
    doc = "
[`parse`]: https://docs.rs/lexical/latest/lexical/fn.parse.html
[`parse_partial`]: https://docs.rs/lexical/latest/lexical/fn.parse_partial.html
[`parse_with_options`]: https://docs.rs/lexical/latest/lexical/fn.parse_with_options.html
[`parse_partial_with_options`]: https://docs.rs/lexical/latest/lexical/fn.parse_partial_with_options.html
"
)]
//!
//! <!-- Space for Rustfmt -->
#![cfg_attr(feature = "parse-floats", doc = "[`ParseFloatOptions`]: crate::ParseFloatOptions")]
#![cfg_attr(feature = "parse-integers", doc = "[`ParseIntegerOptions`]: crate::ParseIntegerOptions")]
#![cfg_attr(feature = "write-floats", doc = "[`WriteFloatOptions`]: crate::WriteFloatOptions")]
#![cfg_attr(feature = "write-integers", doc = "[`WriteIntegerOptions`]: crate::WriteIntegerOptions")]
//!
//! [`NumberFormatBuilder`]: crate::NumberFormatBuilder
//! [benchmarks]: https://github.com/Alexhuszagh/lexical-benchmarks
//! [`rust-1.63.0`]: https://blog.rust-lang.org/2022/08/11/Rust-1.63.0.html

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(
    clippy::doc_markdown,
    clippy::unnecessary_safety_comment,
    clippy::semicolon_if_nothing_returned,
    clippy::unwrap_used,
    clippy::as_underscore
)]
#![allow(
    // used when concepts are logically separate
    clippy::match_same_arms,
    // loss of precision is intentional
    clippy::integer_division,
    // mathematical names use 1-character identifiers
    clippy::min_ident_chars,
    // these are not cryptographically secure contexts
    clippy::integer_division_remainder_used,
    // this can be intentional
    clippy::module_name_repetitions,
    // this is intentional: already passing a pointer and need performance
    clippy::needless_pass_by_value,
    // we use this for inline formatting for unsafe blocks
    clippy::semicolon_inside_block,
)]
#![cfg_attr(rustfmt, rustfmt_skip)]  // reason = "this simplifies our imports"

// Need an allocator for String/Vec.
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
#[macro_use(vec)]
extern crate alloc;

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
use alloc::string::String;

// Re-exports
pub use lexical_core::Error;
pub use lexical_core::Result;

pub use lexical_core::format::{
    self,
    // FIXME: Do not export in the next breaking release.
    format_error,
    // FIXME: Do not export in the next breaking release.
    format_is_valid,
    NumberFormat,
    NumberFormatBuilder,
};

#[cfg(feature = "f16")]
pub use lexical_core::{bf16, f16};

// PARSE

#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub use lexical_core::ParseOptions;

#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub use lexical_core::{FromLexical, FromLexicalWithOptions};

#[cfg(feature = "parse-floats")]
pub use lexical_core::{parse_float_options, ParseFloatOptions, ParseFloatOptionsBuilder};

#[cfg(feature = "parse-integers")]
pub use lexical_core::{parse_integer_options, ParseIntegerOptions, ParseIntegerOptionsBuilder};

// WRITE

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub use lexical_core::WriteOptions;

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub use lexical_core::{ToLexical, ToLexicalWithOptions};

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub use lexical_core::{FormattedSize, BUFFER_SIZE};

#[cfg(feature = "write-floats")]
pub use lexical_core::{write_float_options, WriteFloatOptions, WriteFloatOptionsBuilder};

#[cfg(feature = "write-integers")]
pub use lexical_core::{write_integer_options, WriteIntegerOptions, WriteIntegerOptionsBuilder};

// NOTE: We cannot just use an uninitialized vector with excess capacity and
// then use read-assign rather than `ptr::write` or `MaybeUninit.write` to
// modify the values. When LLVM was the primary code generator, this was
// **UNSPECIFIED** but not undefined behavior: reading undef primitives is safe:
//  https://llvm.org/docs/LangRef.html#undefined-values
//
// However, a different backend such as cranelift might make this undefined
// behavior. That is, from the perspective of Rust, this is undefined behavior:
//
//  ```rust
//  let x = Vec::<u8>::with_capacity(500);
//  let ptr = x.as_mut_ptr()
//  let slc = slice::from_raw_parts_mut(ptr, x.capacity())
//  // UB!!
//  slc[0] = 1;
//
//  // Fine
//  ptr.write(1);
//  ```
//
// Currently, since LLVM treats it as unspecified behavior and will not drop
// values, there is no risk of a memory leak and this is **currently** safe.
// However, this can explode at any time, just like any undefined behavior.

/// High-level conversion of a number to a decimal-encoded string.
///
/// * `n`       - Number to convert to string.
///
/// # Examples
///
/// ```rust
/// assert_eq!(lexical::to_string(5), "5");
/// assert_eq!(lexical::to_string(0.0), "0.0");
/// ```
#[inline]
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub fn to_string<N: ToLexical>(n: N) -> String {
    let mut buf = vec![0u8; N::FORMATTED_SIZE_DECIMAL];
    let len = lexical_core::write(n, buf.as_mut_slice()).len();

    // SAFETY: safe since the buffer is of sufficient size, len() must be <= the vec
    // size.
    unsafe {
        buf.set_len(len);
        String::from_utf8_unchecked(buf)
    }
}

/// High-level conversion of a number to a string with custom writing options.
///
/// * `FORMAT`  - Packed struct containing the number format.
/// * `n`       - Number to convert to string.
/// * `options` - Options to specify number writing.
///
/// # Examples
///
/// ```rust
/// const FORMAT: u128 = lexical::format::STANDARD;
/// const OPTIONS: lexical::WriteFloatOptions = lexical::WriteFloatOptions::builder()
///     .trim_floats(true)
///     .build_strict();
/// assert_eq!(lexical::to_string_with_options::<_, FORMAT>(0.0, &OPTIONS), "0");
/// assert_eq!(lexical::to_string_with_options::<_, FORMAT>(123.456, &OPTIONS), "123.456");
/// ```
#[inline]
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
#[allow(deprecated)] // reason = "allow the user of `buffer_size`"
pub fn to_string_with_options<N: ToLexicalWithOptions, const FORMAT: u128>(
    n: N,
    options: &N::Options,
) -> String {
    // Need to use the `buffer_size` hint to properly deal with float formatting
    // options.
    let size = N::Options::buffer_size::<N, FORMAT>(options);
    let mut buf = vec![0u8; size];
    let slc = buf.as_mut_slice();
    let len = lexical_core::write_with_options::<_, FORMAT>(n, slc, options).len();

    // SAFETY: safe since the buffer is of sufficient size, `len()` must be <= the
    // vec size.
    unsafe {
        buf.set_len(len);
        String::from_utf8_unchecked(buf)
    }
}

/// High-level conversion of decimal-encoded bytes to a number.
///
/// This function only returns a value if the entire string is
/// successfully parsed.
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # use lexical::Error;
/// // Create our error.
/// fn error<T>(r: lexical::Result<T>) -> Error {
///     r.err().unwrap()
/// }
///
/// // String overloads
/// assert_eq!(lexical::parse::<i32, _>("5"), Ok(5));
/// assert!(lexical::parse::<i32, _>("1a").err().unwrap().is_invalid_digit());
/// assert_eq!(lexical::parse::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::parse::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::parse::<f32, _>("1."), Ok(1.0));
///
/// // Bytes overloads
/// assert_eq!(lexical::parse::<i32, _>(b"5"), Ok(5));
/// assert!(lexical::parse::<f32, _>(b"1a").err().unwrap().is_invalid_digit());
/// assert_eq!(lexical::parse::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::parse::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::parse::<f32, _>(b"1."), Ok(1.0));
/// # assert_eq!(lexical::parse::<f32, _>(b"5.002868148396374"), Ok(5.002868148396374));
/// # assert_eq!(lexical::parse::<f64, _>(b"5.002868148396374"), Ok(5.002868148396374));
/// ```
#[inline]
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub fn parse<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes) -> Result<N> {
    N::from_lexical(bytes.as_ref())
}

/// High-level, partial conversion of decimal-encoded bytes to a number.
///
/// This functions parses as many digits as possible, returning the parsed
/// value and the number of digits processed if at least one character
/// is processed. If another error, such as numerical overflow or underflow
/// occurs, this function returns the error code and the index at which
/// the error occurred.
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// // String overloads
/// assert_eq!(lexical::parse_partial::<i32, _>("5"), Ok((5, 1)));
/// assert_eq!(lexical::parse_partial::<i32, _>("1a"), Ok((1, 1)));
/// assert_eq!(lexical::parse_partial::<f32, _>("0"), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial::<f32, _>("1.0"), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial::<f32, _>("1."), Ok((1.0, 2)));
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_partial::<i32, _>(b"5"), Ok((5, 1)));
/// assert_eq!(lexical::parse_partial::<i32, _>(b"1a"), Ok((1, 1)));
/// assert_eq!(lexical::parse_partial::<f32, _>(b"0"), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial::<f32, _>(b"1.0"), Ok((1.0, 3)));
/// assert_eq!(lexical::parse_partial::<f32, _>(b"1."), Ok((1.0, 2)));
/// # assert_eq!(lexical::parse_partial::<f32, _>(b"5.002868148396374"), Ok((5.002868148396374, 17)));
/// # assert_eq!(lexical::parse_partial::<f64, _>(b"5.002868148396374"), Ok((5.002868148396374, 17)));
/// ```
#[inline]
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub fn parse_partial<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes) -> Result<(N, usize)> {
    N::from_lexical_partial(bytes.as_ref())
}

/// High-level conversion of bytes to a number with custom parsing options.
///
/// This function only returns a value if the entire string is
/// successfully parsed.
///
/// * `FORMAT`  - Packed struct containing the number format.
/// * `bytes`   - Byte slice to convert to number.
/// * `options` - Options to specify number parsing.
///
/// # Panics
///
/// If the provided `FORMAT` is not valid, the function may panic. Please
/// ensure `is_valid()` is called prior to using the format, or checking
/// its validity using a static assertion.
///
/// # Examples
///
/// ```rust
/// const FORMAT: u128 = lexical::format::STANDARD;
/// const OPTIONS: lexical::ParseFloatOptions = lexical::ParseFloatOptions::builder()
///     .exponent(b'^')
///     .decimal_point(b',')
///     .build_strict();
/// assert_eq!(lexical::parse_with_options::<f32, _, FORMAT>("0", &OPTIONS), Ok(0.0));
/// assert_eq!(lexical::parse_with_options::<f32, _, FORMAT>("1,2345", &OPTIONS), Ok(1.2345));
/// assert_eq!(lexical::parse_with_options::<f32, _, FORMAT>("1,2345^4", &OPTIONS), Ok(12345.0));
/// ```
#[inline]
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub fn parse_with_options<N: FromLexicalWithOptions, Bytes: AsRef<[u8]>, const FORMAT: u128>(
    bytes: Bytes,
    options: &N::Options,
) -> Result<N> {
    N::from_lexical_with_options::<FORMAT>(bytes.as_ref(), options)
}

/// High-level, partial conversion of bytes to a number with custom parsing
/// options.
///
/// This functions parses as many digits as possible, returning the parsed
/// value and the number of digits processed if at least one character
/// is processed. If another error, such as numerical overflow or underflow
/// occurs, this function returns the error code and the index at which
/// the error occurred.
///
/// * `FORMAT`  - Packed struct containing the number format.
/// * `bytes`   - Byte slice to convert to number.
/// * `options` - Options to specify number parsing.
///
/// # Panics
///
/// If the provided `FORMAT` is not valid, the function may panic. Please
/// ensure `is_valid()` is called prior to using the format, or checking
/// its validity using a static assertion.
///
/// # Examples
///
/// ```rust
/// const FORMAT: u128 = lexical::format::STANDARD;
/// const OPTIONS: lexical::ParseFloatOptions = lexical::ParseFloatOptions::builder()
///     .exponent(b'^')
///     .decimal_point(b',')
///     .build_strict();
/// assert_eq!(lexical::parse_partial_with_options::<f32, _, FORMAT>("0", &OPTIONS), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_with_options::<f32, _, FORMAT>("1,2345", &OPTIONS), Ok((1.2345, 6)));
/// assert_eq!(lexical::parse_partial_with_options::<f32, _, FORMAT>("1,2345^4", &OPTIONS), Ok((12345.0, 8)));
/// ```
#[inline]
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub fn parse_partial_with_options<
    N: FromLexicalWithOptions,
    Bytes: AsRef<[u8]>,
    const FORMAT: u128,
>(
    bytes: Bytes,
    options: &N::Options,
) -> Result<(N, usize)> {
    N::from_lexical_partial_with_options::<FORMAT>(bytes.as_ref(), options)
}
