//! Fast lexical conversion routines for a [`no_std`] environment.
//!
//! `lexical-core` is a high-performance library for number-to-string and
//! string-to-number conversions, without requiring a system
//! allocator. If you would like to use a library that writes to [`String`],
//! look at [lexical](https://crates.io/crates/lexical) instead. In addition
//! to high performance, it's also highly configurable, supporting nearly
//! every float and integer format available.
//!
//! `lexical-core` is well-tested, and has been downloaded more than 50 million
//! times and currently has no known errors in correctness. `lexical-core`
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
//! to parse a number from bytes, validating the entire input is a number:
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "parse-integers"))] {
//! // String to number using Rust slices.
//! // The argument is the byte string parsed.
//! let f: f32 = lexical_core::parse(b"3.5").unwrap();   // 3.5
//! let i: i32 = lexical_core::parse(b"15").unwrap();    // 15
//! # }
//! ```
//!
//! All `lexical-core` parsers are validating, they check the that entire
//! input data is correct, and stop parsing when invalid data is found,
//! numerical overflow, or other errors:
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "parse-integers"))] {
//! let r = lexical_core::parse::<u8>(b"256"); // Err(ErrorCode::Overflow.into())
//! let r = lexical_core::parse::<u8>(b"1a5"); // Err(ErrorCode::InvalidDigit.into())
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
//! let r = lexical_core::parse_partial::<i8>(b"3a5"); // Ok((3, 1))
//! # }
//! ```
//!
//! #### Write API
//!
//! The main parsing API is [`write`]. For example, to write a number to an
//! existing buffer:
//!
//! ```rust
//! # #[cfg(feature = "write-floats")] {
//! use lexical_core::FormattedSize;
//!
//! let mut buf = [b'0'; f64::FORMATTED_SIZE];
//! let slc = lexical_core::write::<f64>(15.1, &mut buf);
//! assert_eq!(slc, b"15.1");
//! # }
//! ```
//!
//! If a buffer of an insufficient size is provided, the writer will panic:
//!
//! ```should_panic
//! # #[cfg(feature = "write-integers")] {
//! let mut buf = [b'0'; 1];
//! let digits = lexical_core::write::<i64>(15, &mut buf);
//! # }
//! # #[cfg(not(feature = "write-integers"))] {
//! # panic!("hidden, for the doctest to pass if the feature isn't enabled.");
//! # }
//! ```
//!
//! In order to guarantee the buffer is large enough, always ensure there
//! are at least [`T::FORMATTED_SIZE_DECIMAL`] bytes, which requires the
//! [`FormattedSize`] trait to be in scope.
//!
//! <!-- References -->
#![cfg_attr(
    any(feature = "write-floats", feature = "write-integers"),
    doc = "
[`FormattedSize`]: FormattedSize
[`T::FORMATTED_SIZE_DECIMAL`]: FormattedSize::FORMATTED_SIZE_DECIMAL
"
)]
#![cfg_attr(
    not(any(feature = "write-floats", feature = "write-integers")),
    doc = "
[`FormattedSize`]: https://docs.rs/lexical-core/latest/lexical_core/trait.FormattedSize.html
[`T::FORMATTED_SIZE_DECIMAL`]: https://docs.rs/lexical-core/latest/lexical_core/trait.FormattedSize.html#associatedconstant.FORMATTED_SIZE_DECIMAL
"
)]
//!
//! ```rust
//! # #[cfg(feature = "write-integers")] {
//! use lexical_core::FormattedSize;
//!
//! let mut buf = [b'0'; f64::FORMATTED_SIZE];
//! let slc = lexical_core::write::<f64>(15.1, &mut buf);
//! assert_eq!(slc, b"15.1");
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
    doc = "- [`write`]: Write a number to string."
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
//! # use core::str;
//! use lexical_core::FormattedSize;
//!
//! // parse
//! let f: f64 = lexical_core::parse(b"3.5").unwrap();
//! assert_eq!(f, 3.5);
//!
//! let (f, count): (f64, usize) = lexical_core::parse_partial(b"3.5").unwrap();
//! assert_eq!(f, 3.5);
//! assert_eq!(count, 3);
//!
//! // write
//! let mut buffer = [0u8; f64::FORMATTED_SIZE_DECIMAL];
//! let digits = lexical_core::write(f, &mut buffer);
//! assert_eq!(str::from_utf8(digits), Ok("3.5"));
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
    doc = "- [`write_with_options`]: Write a number to string using custom formatting options."
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
//! # use core::str;
//! use lexical_core::{format, parse_float_options, write_float_options, FormattedSize};
//!
//! // parse
//! let f: f64 = lexical_core::parse_with_options::<_, { format::JSON }>(
//!     b"3.5",
//!     &parse_float_options::JSON
//! ).unwrap();
//!
//! // write
//! const BUFFER_SIZE: usize = write_float_options::
//!     JSON.buffer_size_const::<f64, { format::JSON }>();
//! let mut buffer = [0u8; BUFFER_SIZE];
//! let digits = lexical_core::write_with_options::<_, { format::JSON }>(
//!     f,
//!     &mut buffer,
//!     &write_float_options::JSON
//! );
//! assert_eq!(str::from_utf8(digits), Ok("3.5"));
//! # }
//! ```
//!
//! [Comprehensive Configuration]: #comprehensive-configuration
//!
//! # Features
//!
//! In accordance with the Rust ethos, all features are additive: the crate
//! may be build with `--all-features` without issue. The following features
//! are enabled by default:
//!
//! * `write-integers` (Default) - Enable writing of integers.
//! * `write-floats` (Default) - Enable writing of floats.
//! * `parse-integers` (Default) - Enable parsing of integers.
//! * `parse-floats` (Default) - Enable parsing of floats.
//! * `power-of-two` - Add support for writing power-of-two number strings.
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
//! # use core::str;
//! use lexical_core::FormattedSize;
//!
//! let mut buffer = [0u8; i64::FORMATTED_SIZE_DECIMAL];
//! let digits = lexical_core::write(1234, &mut buffer);
//! assert_eq!(str::from_utf8(digits), Ok("1234"));
//! # }
//! ```
//!
//! #### write-floats
//!
//! Enable support for writing floating-point numbers to string.
//!
//! ```rust
//! # #[cfg(feature = "write-floats")] {
//! # use core::str;
//! use lexical_core::FormattedSize;
//!
//! let mut buffer = [0u8; f64::FORMATTED_SIZE_DECIMAL];
//! let digits = lexical_core::write(1.234, &mut buffer);
//! assert_eq!(str::from_utf8(digits), Ok("1.234"));
//! # }
//! ```
//!
//! #### parse-integers
//!
//! Enable support for parsing integers from string.
//!
//! ```rust
//! # #[cfg(feature = "parse-integers")] {
//! let f: i64 = lexical_core::parse(b"1234").unwrap();
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
//! let f: f64 = lexical_core::parse(b"1.234").unwrap();
//! assert_eq!(f, 1.234);
//! # }
//! ```
//!
//! #### format
//!
//! Adds support for the entire format [API][NumberFormatBuilder]. This
//! allows extensive configurability for parsing and writing numbers
//! in custom formats, with different valid syntax requirements.
//!
//! ##### JSON
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
//! All of the finite numbers are valid in Rust, and Rust supports non-finite
//! floats. In order to parse standard-conforming JSON floats using
//! `lexical-core`, you may use the following approach:
//!
//! ```rust
//! # #[cfg(all(feature = "parse-floats", feature = "format"))] {
//! use lexical_core::{format, parse_float_options, parse_with_options, Result};
//!
//! fn parse_json_float<Bytes: AsRef<[u8]>>(bytes: Bytes) -> Result<f64> {
//!     parse_with_options::<_, { format::JSON }>(bytes.as_ref(), &parse_float_options::JSON)
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
//! # use core::str;
//! use lexical_core::{
//!     ParseFloatOptions,
//!     WriteFloatOptions,
//!     FormattedSize,
//!     NumberFormatBuilder
//! };
//!
//! // parse
//! const BINARY: u128 = NumberFormatBuilder::binary();
//! let value = "1.0011101111100111011011001000101101000011100101011";
//! let f: f64 = lexical_core::parse_with_options::<_, { BINARY }>(
//!     value.as_bytes(),
//!     &ParseFloatOptions::new()
//! ).unwrap();
//!
//! // write
//! let mut buffer = [0u8; f64::FORMATTED_SIZE];
//! let digits = lexical_core::write_with_options::<_, { BINARY }>(
//!     f,
//!     &mut buffer,
//!     &WriteFloatOptions::new()
//! );
//! assert_eq!(str::from_utf8(digits), Ok(value));
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
//! use lexical_core::{
//!     ParseFloatOptions,
//!     WriteFloatOptions,
//!     FormattedSize,
//!     NumberFormatBuilder
//! };
//!
//! // parse
//! const FORMAT: u128 = NumberFormatBuilder::from_radix(12);
//! let value = "1.29842830A44BAA2";
//! let f: f64 = lexical_core::parse_with_options::<_, { FORMAT }>(
//!     value.as_bytes(),
//!     &ParseFloatOptions::new()
//! ).unwrap();
//!
//! // write
//! let mut buffer = [0u8; f64::FORMATTED_SIZE];
//! let digits = lexical_core::write_with_options::<_, { FORMAT }>(
//!     f,
//!     &mut buffer,
//!     &WriteFloatOptions::new()
//! );
//! assert_eq!(str::from_utf8(digits), Ok(value));
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
//! `lexical-core` provides two main levels of configuration:
//! - The [`NumberFormatBuilder`], creating a packed struct with custom
//!   formatting options.
//! - The Options API.
//!
//! ## Number Format
//!
//! The number format class provides numerous flags to specify number parsing or
//! writing. When the [`power-of-two`](#power-of-two) feature is
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
[`JSON`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.JSON.html
[`XML`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.XML.html
[`TOML`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.TOML.html
[`YAML`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.YAML.html
[`SQLite`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.SQLITE.html
[`Rust`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.RUST_LITERAL.html
[`Python`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.PYTHON_LITERAL.html
[`C#`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.CSHARP_LITERAL.html
[`FORTRAN`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.FORTRAN_LITERAL.html
[`COBOL`]: https://docs.rs/lexical-core/latest/lexical_core/format/constant.COBOL_LITERAL.html
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
[`write-float-exponent`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.exponent
[`write-float-decimal_point`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.decimal_point
[`write-float-nan_string`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.nan_string
[`write-float-trim_floats`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.trim_floats
[`write-float-positive_exponent_break`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.positive_exponent_break
[`write-float-max_significant_digits`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.max_significant_digits
[`write-float-min_significant_digits`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.min_significant_digits
[`write-float-round_mode`]: https://docs.rs/lexical-core/latest/lexical_core/struct.WriteFloatOptionsBuilder.html#method.round_mode
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
    doc = "[`JSON-OPTS`]: https://docs.rs/lexical-core/latest/lexical_core/parse_float_options/constant.JSON.html"
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
//! const EUROPEAN: u128 = lexical_core::NumberFormatBuilder::new()
//!     .digit_separator(num::NonZeroU8::new(b'.'))
//!     .build_strict();
//! const COMMA_OPTIONS: lexical_core::ParseFloatOptions = lexical_core::ParseFloatOptions::builder()
//!     .decimal_point(b',')
//!     .build_strict();
//! assert_eq!(
//!     lexical_core::parse_with_options::<f32, EUROPEAN>(b"300,10", &COMMA_OPTIONS),
//!     Ok(300.10)
//! );
//!
//! // Another example, using a pre-defined constant for JSON.
//! const JSON: u128 = lexical_core::format::JSON;
//! const JSON_OPTIONS: lexical_core::ParseFloatOptions = lexical_core::ParseFloatOptions::new();
//! assert_eq!(
//!     lexical_core::parse_with_options::<f32, JSON>(b"0e1", &JSON_OPTIONS),
//!     Ok(0.0)
//! );
//! assert_eq!(
//!     lexical_core::parse_with_options::<f32, JSON>(b"1E+2", &JSON_OPTIONS),
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
//! # Safety Guarantees
//!
//! There is no non-trivial unsafe behavior in `lexical-core` itself,
//! however, any incorrect safety invariants in our parsers and writers
//! ([`lexical-parse-float`], [`lexical-parse-integer`],
//! [`lexical-write-float`], and [`lexical-write-integer`]) could cause those
//! safety invariants to be broken.
//!
//! [`lexical-parse-float`]: https://crates.io/crates/lexical-parse-float
//! [`lexical-parse-integer`]: https://crates.io/crates/lexical-parse-integer
//! [`lexical-write-float`]: https://crates.io/crates/lexical-write-float
//! [`lexical-write-integer`]: https://crates.io/crates/lexical-write-integer
//!
//! <!-- Spacer for Rustfmt -->
#![cfg_attr(
    any(feature = "write-floats", feature = "write-integers"),
    doc = "
[`write`]: crate::write
[`write_with_options`]: crate::write_with_options
"
)]
#![cfg_attr(
    not(any(feature = "write-floats", feature = "write-integers")),
    doc = "
[`write`]: https://docs.rs/lexical-core/latest/lexical_core/fn.write.html
[`write_with_options`]: https://docs.rs/lexical-core/latest/lexical_core/fn.write_with_options.html
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
[`parse`]: https://docs.rs/lexical-core/latest/lexical_core/fn.parse.html
[`parse_partial`]: https://docs.rs/lexical-core/latest/lexical_core/fn.parse_partial.html
[`parse_with_options`]: https://docs.rs/lexical-core/latest/lexical_core/fn.parse_with_options.html
[`parse_partial_with_options`]: https://docs.rs/lexical-core/latest/lexical_core/fn.parse_partial_with_options.html
"
)]
//!
//! <!-- Space for Rustfmt -->
#![cfg_attr(feature = "parse-floats", doc = "[`ParseFloatOptions`]: crate::ParseFloatOptions")]
#![cfg_attr(feature = "parse-integers", doc = "[`ParseIntegerOptions`]: crate::ParseIntegerOptions")]
#![cfg_attr(feature = "write-floats", doc = "[`WriteFloatOptions`]: crate::WriteFloatOptions")]
#![cfg_attr(feature = "write-integers", doc = "[`WriteIntegerOptions`]: crate::WriteIntegerOptions")]
//!
//! [`String`]: https://doc.rust-lang.org/alloc/string/struct.String.html
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

// Re-exports
pub use lexical_util::Error;
pub use lexical_util::result::Result;

pub use lexical_util::format::{
    self,
    // FIXME: Do not export in the next breaking release.
    format_error,
    // FIXME: Do not export in the next breaking release.
    format_is_valid,
    NumberFormat,
    NumberFormatBuilder,
};

#[cfg(feature = "f16")]
pub use lexical_util::bf16::bf16;

#[cfg(feature = "f16")]
pub use lexical_util::f16::f16;

// PARSE

#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub use lexical_util::options::ParseOptions;

#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
use lexical_util::{from_lexical, from_lexical_with_options};

#[cfg(feature = "parse-floats")]
pub use lexical_parse_float::{
    options as parse_float_options,
    Options as ParseFloatOptions,
    OptionsBuilder as ParseFloatOptionsBuilder,
};

#[cfg(feature = "parse-floats")]
use lexical_parse_float::{
    FromLexical as FromFloat,
    FromLexicalWithOptions as FromFloatWithOptions,
};
#[cfg(feature = "parse-integers")]
pub use lexical_parse_integer::{
    options as parse_integer_options,
    Options as ParseIntegerOptions,
    OptionsBuilder as ParseIntegerOptionsBuilder,
};
#[cfg(feature = "parse-integers")]
use lexical_parse_integer::{
    FromLexical as FromInteger,
    FromLexicalWithOptions as FromIntegerWithOptions,
};

// WRITE

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub use lexical_util::options::WriteOptions;

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
use lexical_util::{to_lexical, to_lexical_with_options};

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub use lexical_util::constants::{FormattedSize, BUFFER_SIZE};

#[cfg(feature = "write-floats")]
pub use lexical_write_float::{
    options as write_float_options,
    Options as WriteFloatOptions,
    OptionsBuilder as WriteFloatOptionsBuilder,
};
#[cfg(feature = "write-floats")]
use lexical_write_float::{ToLexical as ToFloat, ToLexicalWithOptions as ToFloatWithOptions};

#[cfg(feature = "write-integers")]
pub use lexical_write_integer::{
    options as write_integer_options,
    Options as WriteIntegerOptions,
    OptionsBuilder as WriteIntegerOptionsBuilder,
};

#[cfg(feature = "write-integers")]
use lexical_write_integer::{ToLexical as ToInteger, ToLexicalWithOptions as ToIntegerWithOptions};

// API
// ---

#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
from_lexical!(
    "lexical_core",
    1234,
    u64,
    4,
    #[cfg_attr(docsrs, doc(cfg(any(feature = "parse-floats", feature = "parse-integers"))))]
);

#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
from_lexical_with_options!(
    "lexical_core",
    1234,
    u64,
    4,
    ParseIntegerOptions,
    #[cfg_attr(docsrs, doc(cfg(any(feature = "parse-floats", feature = "parse-integers"))))]
);

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
to_lexical!(
    "lexical_core",
    1234,
    u64,
    #[cfg_attr(docsrs, doc(cfg(any(feature = "write-floats", feature = "write-integers"))))]
);

#[cfg(any(feature = "write-floats", feature = "write-integers"))]
to_lexical_with_options!(
    "lexical_core",
    1234,
    u64,
    WriteIntegerOptions,
    #[cfg_attr(docsrs, doc(cfg(any(feature = "write-floats", feature = "write-integers"))))]
);

/// Implement `FromLexical` and `FromLexicalWithOptions` for numeric types.
///
/// * `t`                           - The numerical type.
/// * `from`                        - The internal trait that implements
///   `from_lexical`.
/// * `from_lexical_with_options`   - The internal trait that implements
///   `from_lexical`.
/// * `options`                     - The options type to configure settings.
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
macro_rules! from_lexical_impl {
    ($t:ident, $from:ident, $from_options:ident, $options:ident) => {
        impl FromLexical for $t {
            #[cfg_attr(not(feature = "compact"), inline)]
            fn from_lexical(bytes: &[u8]) -> Result<Self> {
                <Self as $from>::from_lexical(bytes)
            }

            #[cfg_attr(not(feature = "compact"), inline)]
            fn from_lexical_partial(bytes: &[u8]) -> Result<(Self, usize)> {
                <Self as $from>::from_lexical_partial(bytes)
            }
        }

        impl FromLexicalWithOptions for $t {
            type Options = $options;

            #[cfg_attr(not(feature = "compact"), inline)]
            fn from_lexical_with_options<const FORMAT: u128>(
                bytes: &[u8],
                options: &Self::Options,
            ) -> Result<Self> {
                <Self as $from_options>::from_lexical_with_options::<FORMAT>(bytes, options)
            }

            #[cfg_attr(not(feature = "compact"), inline)]
            fn from_lexical_partial_with_options<const FORMAT: u128>(
                bytes: &[u8],
                options: &Self::Options,
            ) -> Result<(Self, usize)> {
                <Self as $from_options>::from_lexical_partial_with_options::<FORMAT>(bytes, options)
            }
        }
    };
}

/// Implement `FromLexical` and `FromLexicalWithOptions` for integers.
#[cfg(feature = "parse-integers")]
macro_rules! integer_from_lexical {
    ($($t:ident)*) => ($(
        from_lexical_impl!($t, FromInteger, FromIntegerWithOptions, ParseIntegerOptions);
    )*);
}

#[cfg(feature = "parse-integers")]
integer_from_lexical! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

/// Implement `FromLexical` and `FromLexicalWithOptions` for floats.
#[cfg(feature = "parse-floats")]
macro_rules! float_from_lexical {
    ($($t:ident)*) => ($(
        from_lexical_impl!($t, FromFloat, FromFloatWithOptions, ParseFloatOptions);
    )*);
}

#[cfg(feature = "parse-floats")]
float_from_lexical! { f32 f64 }

/// Implement `ToLexical` and `ToLexicalWithOptions` for numeric types.
///
/// * `t`                           - The numerical type.
/// * `to`                          - The internal trait that implements
///   `to_lexical`.
/// * `to_lexical_with_options`     - The internal trait that implements
///   `to_lexical`.
/// * `options`                     - The options type to configure settings.
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
macro_rules! to_lexical_impl {
    ($t:ident, $to:ident, $to_options:ident, $options:ident) => {
        impl ToLexical for $t {
            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical(self, bytes: &mut [u8]) -> &mut [u8] {
                <Self as $to>::to_lexical(self, bytes)
            }
        }

        impl ToLexicalWithOptions for $t {
            type Options = $options;
            #[cfg_attr(not(feature = "compact"), inline(always))]
            fn to_lexical_with_options<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8] {
                <Self as $to_options>::to_lexical_with_options::<FORMAT>(self, bytes, options)
            }
        }
    };
}

/// Implement `ToLexical` and `ToLexicalWithOptions` for integers.
#[cfg(feature = "write-integers")]
macro_rules! integer_to_lexical {
    ($($t:ident)*) => ($(
        to_lexical_impl!($t, ToInteger, ToIntegerWithOptions, WriteIntegerOptions);
    )*);
}

#[cfg(feature = "write-integers")]
integer_to_lexical! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

/// Implement `ToLexical` and `ToLexicalWithOptions` for floats.
#[cfg(feature = "write-floats")]
macro_rules! float_to_lexical {
    ($($t:ident)*) => ($(
        to_lexical_impl!($t, ToFloat, ToFloatWithOptions, WriteFloatOptions);
    )*);
}

#[cfg(feature = "write-floats")]
float_to_lexical! { f32 f64 }

/// Write number to string.
///
/// Returns a subslice of the input buffer containing the written bytes,
/// starting from the same address in memory as the input slice.
///
/// * `value`   - Number to serialize.
/// * `bytes`   - Buffer to write number to.
///
/// # Panics
///
/// Panics if the buffer may not be large enough to hold the serialized
/// number. In order to ensure the function will not panic, provide a
/// buffer with at least `{integer}::FORMATTED_SIZE` elements.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "write-floats")] {
/// // import `BUFFER_SIZE` to get the maximum bytes written by the number.
/// use lexical_core::BUFFER_SIZE;
///
/// let mut buffer = [0u8; BUFFER_SIZE];
/// let float = 3.14159265359_f32;
///
/// lexical_core::write(float, &mut buffer);
///
/// assert_eq!(&buffer[0..9], b"3.1415927");
/// # }
/// ```
///
/// This will panic, because the buffer is not large enough:
///
/// ```should_panic
/// # #[cfg(feature = "write-floats")] {
/// // note: the buffer is only one byte large
/// let mut buffer = [0u8; 1];
/// let float = 3.14159265359_f32;
///
/// lexical_core::write(float, &mut buffer);
/// # }
/// # #[cfg(not(feature = "write-floats"))] {
/// #     panic!("");
/// # }
/// ```
#[inline]
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub fn write<N: ToLexical>(n: N, bytes: &mut [u8]) -> &mut [u8] {
    n.to_lexical(bytes)
}

/// Write number to string with custom options.
///
/// Returns a subslice of the input buffer containing the written bytes,
/// starting from the same address in memory as the input slice.
///
/// * `FORMAT`  - Packed struct containing the number format.
/// * `value`   - Number to serialize.
/// * `bytes`   - Buffer to write number to.
/// * `options` - Options to customize number parsing.
///
/// # Panics
///
/// Panics if the buffer may not be large enough to hold the serialized
/// number. In order to ensure the function will not panic, provide a
/// buffer with at least `{integer}::FORMATTED_SIZE` elements. If you
/// are using custom digit precision control or exponent break points
/// for writing floats, these constants may be insufficient to store
/// the serialized number, and up to 1200 bytes may be required with
/// radix support.
///
/// If the provided `FORMAT` is not valid, the function may panic. Please
/// ensure `is_valid()` is called prior to using the format, or checking
/// its validity using a static assertion.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "write-floats")] {
/// // import `BUFFER_SIZE` to get the maximum bytes written by the number.
/// use lexical_core::BUFFER_SIZE;
///
/// let mut buffer = [0u8; BUFFER_SIZE];
/// let float = 3.14159265359_f32;
///
/// const FORMAT: u128 = lexical_core::format::STANDARD;
/// const OPTIONS: lexical_core::WriteFloatOptions = lexical_core::WriteFloatOptions::new();
/// lexical_core::write_with_options::<_, FORMAT>(float, &mut buffer, &OPTIONS);
///
/// assert_eq!(&buffer[0..9], b"3.1415927");
/// # }
/// ```
///
/// This will panic, because the buffer is not large enough:
///
/// ```should_panic
/// # #[cfg(feature = "write-floats")] {
/// // note: the buffer is only one byte large
/// let mut buffer = [0u8; 1];
/// let float = 3.14159265359_f32;
///
/// const FORMAT: u128 = lexical_core::format::STANDARD;
/// const OPTIONS: lexical_core::WriteFloatOptions = lexical_core::WriteFloatOptions::new();
/// lexical_core::write_with_options::<_, FORMAT>(float, &mut buffer, &OPTIONS);
/// # }
/// # #[cfg(not(feature = "write-floats"))] {
/// #     panic!("");
/// # }
/// ```
#[inline]
#[cfg(any(feature = "write-floats", feature = "write-integers"))]
pub fn write_with_options<'a, N: ToLexicalWithOptions, const FORMAT: u128>(
    n: N,
    bytes: &'a mut [u8],
    options: &N::Options,
) -> &'a mut [u8] {
    n.to_lexical_with_options::<FORMAT>(bytes, options)
}

/// Parse complete number from string.
///
/// This method parses the entire string, returning an error if
/// any invalid digits are found during parsing.
///
/// * `bytes`   - Byte slice containing a numeric string.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "parse-floats")] {
/// let string = "3.14159265359";
/// let result = lexical_core::parse::<f32>(string.as_bytes());
/// assert_eq!(result, Ok(3.14159265359_f32));
/// # }
/// ```
#[inline]
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub fn parse<N: FromLexical>(bytes: &[u8]) -> Result<N> {
    N::from_lexical(bytes)
}

/// Parse partial number from string.
///
/// This method parses until an invalid digit is found (or the end
/// of the string), returning the number of processed digits
/// and the parsed value until that point.
///
/// * `bytes`   - Byte slice containing a numeric string.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "parse-floats")] {
/// let string = "3.14159265359 hello";
/// let result = lexical_core::parse_partial::<f32>(string.as_bytes());
/// assert_eq!(result, Ok((3.14159265359_f32, 13)));
/// # }
/// ```
#[inline]
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub fn parse_partial<N: FromLexical>(bytes: &[u8]) -> Result<(N, usize)> {
    N::from_lexical_partial(bytes)
}

/// Parse complete number from string with custom parsing options.
///
/// This method parses the entire string, returning an error if
/// any invalid digits are found during parsing.
///
/// * `FORMAT`  - Packed struct containing the number format.
/// * `bytes`   - Byte slice containing a numeric string.
/// * `options` - Options to customize number parsing.
///
/// # Examples
///
/// ```
/// # #[cfg(all(feature = "parse-floats", feature = "format"))] {
/// const JSON: u128 = lexical_core::format::JSON;
/// const OPTIONS: lexical_core::ParseFloatOptions = lexical_core::ParseFloatOptions::new();
/// let string = "3.14159265359";
/// let result = lexical_core::parse_with_options::<f32, JSON>(string.as_bytes(), &OPTIONS);
/// assert_eq!(result, Ok(3.14159265359_f32));
/// # }
/// ```
#[inline]
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub fn parse_with_options<N: FromLexicalWithOptions, const FORMAT: u128>(
    bytes: &[u8],
    options: &N::Options,
) -> Result<N> {
    N::from_lexical_with_options::<FORMAT>(bytes, options)
}

/// Parse partial number from string with custom parsing options.
///
/// This method parses until an invalid digit is found (or the end
/// of the string), returning the number of processed digits
/// and the parsed value until that point.
///
/// * `FORMAT`  - Packed struct containing the number format.
/// * `bytes`   - Byte slice containing a numeric string.
/// * `options` - Options to customize number parsing.
///
/// # Examples
///
/// ```
/// # #[cfg(all(feature = "parse-floats", feature = "format"))] {
/// const JSON: u128 = lexical_core::format::JSON;
/// const OPTIONS: lexical_core::ParseFloatOptions = lexical_core::ParseFloatOptions::new();
/// let string = "3.14159265359 hello";
/// let result = lexical_core::parse_partial_with_options::<f32, JSON>(string.as_bytes(), &OPTIONS);
/// assert_eq!(result, Ok((3.14159265359_f32, 13)));
/// # }
/// ```
#[inline]
#[cfg(any(feature = "parse-floats", feature = "parse-integers"))]
pub fn parse_partial_with_options<N: FromLexicalWithOptions, const FORMAT: u128>(
    bytes: &[u8],
    options: &N::Options,
) -> Result<(N, usize)> {
    N::from_lexical_partial_with_options::<FORMAT>(bytes, options)
}
