//! Fast and compact float-to-string conversions.
//!
//! This contains high-performance methods to write floating-point numbers
//! directly to bytes, can be converted to [`str`] using
//! [`str::from_utf8`]. Using [`to_lexical`] is analogous to [`to_string`],
//! just writing to an existing buffer.
//!
//! It also contains extensively formatting control, including the use of
//! exponent notation, if to round or truncate floats, the number of significant
//! digits, and more.
//!
//! [`str::from_utf8`]: core::str::from_utf8
//! [`to_lexical`]: ToLexical::to_lexical
//!
//! # Getting Started
//!
//! To write a number to bytes, use [`to_lexical`]:
//!
//! [`to_lexical`]: ToLexical::to_lexical
//!
//! ```rust
//! # #[no_std]
//! # use core::str;
//! use lexical_write_float::{FormattedSize, ToLexical};
//!
//! let mut buffer = [0u8; f64::FORMATTED_SIZE_DECIMAL];
//! let digits = 1.234f64.to_lexical(&mut buffer);
//! assert_eq!(str::from_utf8(digits), Ok("1.234"));
//! ```
//!
//! With the default options, using [`FORMATTED_SIZE_DECIMAL`]
//! guarantees the buffer will be large enough to write the digits for all
//! numbers of that type.
//!
//! [`FORMATTED_SIZE_DECIMAL`]: FormattedSize::FORMATTED_SIZE_DECIMAL
//!
//! # Options/Formatting API
//!
//! Each float formatter contains extensive formatting control, including
//! a maximum number of significant digits written, a minimum number of
//! significant digits remaining, the positive and negative exponent break
//! points (at what exponent, in scientific-notation, to force scientific
//! notation), whether to force or disable scientific notation, the rounding
//! mode for truncated float strings, and how to display non-finite floats.
//! While using custom float options, you must use
//! [`Options::buffer_size_const`] to determine the correct buffer size:
//!
//! ```rust
//! # #[cfg(feature = "format")] {
//! # use core::str;
//! use lexical_write_float::{format, options, ToLexicalWithOptions};
//!
//! const BUFFER_SIZE: usize = options::RUST_LITERAL
//!     .buffer_size_const::<f64, { format::RUST_LITERAL }>();
//!
//! fn write_rust_float(f: f64) -> ([u8; BUFFER_SIZE], usize) {
//!     let mut buffer = [0u8; BUFFER_SIZE];
//!     let digits = f.to_lexical_with_options::<{ format::RUST_LITERAL }>(
//!         &mut buffer,
//!         &options::RUST_LITERAL
//!     );
//!     let count = digits.len();
//!     (buffer, count)
//! }
//!
//! let (digits, count) = write_rust_float(3.5);
//! assert_eq!(str::from_utf8(&digits[..count]), Ok("3.5"));
//! # }
//! ```
//!
//! For additional supported options for customizing how to write floats, see
//! the [`OptionsBuilder`]. If you're looking to parse floats with a grammar
//! for a programming language, many pre-defined options such as for [`JSON`]
//! exist in [`mod@options`]. For even more customization, see the
//! [`format`](#format) and [Comprehensive Configuration] sections
//! below.
//!
//! [`JSON`]: crate::options::JSON
//! [Comprehensive Configuration]: #comprehensive-configuration
//!
//! # Features
//!
//! * `format` - Add support for custom float formatting.
//! * `power-of-two` - Add support for writing power-of-two float strings.
//! * `radix` - Add support for strings of any radix.
//! * `compact` - Reduce code size at the cost of performance.
//! * `f16` - Enable support for half-precision [`f16`][`ieee-f16`] and
//!   [`bf16`][`brain-float`] floats.
//! * `std` (Default) - Disable to allow use in a [`no_std`] environment.
//!
//! [`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
//! [`ieee-f16`]: https://en.wikipedia.org/wiki/Half-precision_floating-point_format
//! [`brain-float`]: https://en.wikipedia.org/wiki/Bfloat16_floating-point_format
//!
//! A complete description of supported features includes:
//!
//! #### format
//!
//! Add support custom float formatting specifications. This should be used in
//! conjunction with [`Options`] for extensible float writing. You must use
//! [`Options::buffer_size_const`] to determine the number of bytes requires in
//! the buffer. This allows changing the use of exponent notation, requiring or
//! not allowing signs, and more.
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
//! floats. In order to write standard-conforming JSON floats using
//! `lexical-core`, you may use the following approach:
//!
//! ```rust
//! # #[cfg(feature = "format")] {
//! # use core::str;
//! use lexical_write_float::{format, options, ToLexicalWithOptions};
//!
//! const BUFFER_SIZE: usize = options::JSON.buffer_size_const::<f64, { format::JSON }>();
//!
//! fn write_json_float(f: f64) -> ([u8; BUFFER_SIZE], usize) {
//!     let mut buffer = [0u8; BUFFER_SIZE];
//!     let digits = f.to_lexical_with_options::<{ format::JSON }>(
//!         &mut buffer,
//!         &options::JSON
//!     );
//!     let count = digits.len();
//!     (buffer, count)
//! }
//!
//! let (digits, count) = write_json_float(3.5);
//! assert_eq!(str::from_utf8(&digits[..count]), Ok("3.5"));
//! # }
//! ```
//!
//! ##### Custom Signs
//!
//! An example of building a custom format to ensure positive signs are always
//! written is as follows:
//!
//! ```rust
//! # #[cfg(feature = "radix")] {
//! # use core::str;
//! use lexical_write_float::{FormattedSize, NumberFormatBuilder, Options, ToLexicalWithOptions};
//!
//! const FORMAT: u128 = NumberFormatBuilder::new()
//!     // require a `+` or `-` sign before the number
//!     .required_mantissa_sign(true)
//!     // require a `+` or `-` sign before the exponent digits
//!     .required_exponent_sign(true)
//!     // build the format, panicking if the format is invalid
//!     .build_strict();
//! const OPTIONS: Options = Options::new();
//!
//! const BUFFER_SIZE: usize = OPTIONS.buffer_size_const::<f64, FORMAT>();
//! let mut buffer = [0u8; BUFFER_SIZE];
//!
//! let digits = 1.234e300f64.to_lexical_with_options::<FORMAT>(&mut buffer, &OPTIONS);
//! assert_eq!(str::from_utf8(digits), Ok("+1.234e+300"));
//! # }
//! ```
//!
//! Enabling the [`format`](crate#format) API significantly increases compile
//! times, however, it enables a large amount of customization in how floats are
//! written.
//!
//! #### power-of-two
//!
//! Enable writing numbers that are powers of two, that is, `2`, `4`, `8`, `16`,
//! and `32`. In these cases, you should use [`FORMATTED_SIZE`] to create a
//! sufficiently large buffer.
//!
//! [`FORMATTED_SIZE`]: FormattedSize::FORMATTED_SIZE
//!
//! ```rust
//! # #[cfg(feature = "power-of-two")] {
//! # use core::str;
//! use lexical_write_float::{FormattedSize, NumberFormatBuilder, Options, ToLexicalWithOptions};
//!
//! let mut buffer = [0u8; f64::FORMATTED_SIZE];
//! const BINARY: u128 = NumberFormatBuilder::binary();
//! const OPTIONS: Options = Options::new();
//! let digits = 1.234f64.to_lexical_with_options::<BINARY>(&mut buffer, &OPTIONS);
//! assert_eq!(str::from_utf8(digits), Ok("1.0011101111100111011011001000101101000011100101011"));
//! # }
//! ```
//!
//! #### radix
//!
//! Enable writing numbers using all radixes from `2` to `36`. This requires
//! more static storage than [`power-of-two`][crate#power-of-two], and increases
//! compile times, but can be quite useful for esoteric programming languages
//! which use duodecimal floats, for example.
//!
//! ```rust
//! # #[cfg(feature = "radix")] {
//! # use core::str;
//! use lexical_write_float::{FormattedSize, NumberFormatBuilder, Options, ToLexicalWithOptions};
//!
//! const FORMAT: u128 = NumberFormatBuilder::from_radix(12);
//! const OPTIONS: Options = Options::new();
//!
//! let mut buffer = [0u8; f64::FORMATTED_SIZE];
//! let digits = 1.234f64.to_lexical_with_options::<FORMAT>(&mut buffer, &OPTIONS);
//! assert_eq!(str::from_utf8(digits), Ok("1.29842830A44BAA2"));
//! # }
//! ```
//!
//! #### compact
//!
//! Reduce the generated code size at the cost of performance. This minimizes
//! the number of static tables, inlining, and generics used, drastically
//! reducing the size of the generated binaries. However, this resulting
//! performance of the generated code is much lower.
//!
//! #### f16
//!
//! This enables the use of the half-precision floats [`f16`][`ieee-f16`] and
//! [`bf16`][`brain-float`]. However, since these have limited hardware support
//! and are primarily used for vectorized operations, they are formatted as if
//! they were an [`f32`]. Due to the low precision of 16-bit floats, the results
//! may appear to have significant rounding error.
//!
//! ```rust
//! # #[cfg(feature = "f16")] {
//! # use core::str;
//! use lexical_write_float::{f16, FormattedSize, ToLexical};
//!
//! let mut buffer = [0u8; f16::FORMATTED_SIZE];
//! let value = f16::from_f64_const(1.234f64);
//! let digits = value.to_lexical(&mut buffer);
//! assert_eq!(str::from_utf8(digits), Ok("1.234375"));
//! # }
//! ```
//!
//! #### std
//!
//! Enable use of the standard library. Currently, the standard library
//! is not used, and may be disabled without any change in functionality
//! on stable.
//!
//! # Comprehensive Configuration
//!
//! `lexical-write-float` provides two main levels of configuration:
//! - The [`NumberFormatBuilder`], creating a packed struct with custom
//!   formatting options.
//! - The [`Options`] API.
//!
//! ## Number Format
//!
//! The number format class provides numerous flags to specify number writing.
//! When the [`power-of-two`](#power-of-two) feature is enabled, additional
//! flags are added:
//! - The radix for the significant digits (default `10`).
//! - The radix for the exponent base (default `10`).
//! - The radix for the exponent digits (default `10`).
//!
//! When the [`format`](#format) feature is enabled, numerous other syntax and
//! digit separator flags are enabled, including:
//! - Requiring or ommitting `+` signs.
//! - If to use exponent notation.
//!
//! Many pre-defined constants therefore exist to simplify common use-cases,
//! including:
//! - [`JSON`], [`XML`], [`TOML`], [`YAML`], [`SQLite`], and many more.
//! - [`Rust`], [`Python`], [`C#`], [`FORTRAN`], [`COBOL`] literals and strings,
//!   and many more.
//!
//! For a list of all supported fields, see [Write
//! Float Fields][NumberFormatBuilder#write-float-fields].
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
[`JSON`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.JSON.html
[`XML`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.XML.html
[`TOML`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.TOML.html
[`YAML`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.YAML.html
[`SQLite`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.SQLITE.html
[`Rust`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.RUST_LITERAL.html
[`Python`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.PYTHON_LITERAL.html
[`C#`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.CSHARP_LITERAL.html
[`FORTRAN`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.FORTRAN_LITERAL.html
[`COBOL`]: https://docs.rs/lexical-write-float/latest/lexical_write_float/format/constant.COBOL_LITERAL.html
"
)]
//!
//! ## Options API
//!
//! The Options API provides high-level options to specify number parsing
//! or writing, options not intrinsically tied to a number format.
//! For example, the Options API provides:
//! - The [`exponent`][Options::exponent] character (defaults to `b'e'` or
//!   `b'^'`, depending on the radix).
//! - The [`decimal point`][Options::decimal_point] character (defaults to
//!   `b'.'`).
//! - Custom [`NaN`][f64::NAN] and [`Infinity`][f64::INFINITY] string
//!   [`representations`][Options::nan_string].
//! - Whether to [`trim`][Options::trim_floats] the fraction component from
//!   integral floats.
//! - The exponent [`break-point`][Options::positive_exponent_break] for
//!   scientific notation.
//! - The [`maximum`][Options::max_significant_digits] and
//!   [`minimum`][Options::min_significant_digits] number of significant digits
//!   to write.
//! - The rounding [`mode`][Options::round_mode] when truncating significant
//!   digits while writing.
//!
//! In addition, pre-defined constants for each category of options may
//! be found in their respective modules, for example, [`JSON`][`JSON-OPTS`].
//!
//! [`JSON-OPTS`]: options::JSON
//!
//! ## Examples
//!
//! An example of creating your own options to parse European-style
//! numbers (which use commas as decimal points, controlling the number
//! of significant digits, special number representations, and more, is as
//! follows:
//!
//! ```rust
//! # use core::{num, str};
//! use lexical_write_float::{FormattedSize, Options, ToLexicalWithOptions};
//!
//! const FORMAT: u128 = lexical_write_float::format::STANDARD;
//! const CUSTOM: Options = Options::builder()
//!     // write exponents as "1.2^10" and not "1.2e10"
//!     .exponent(b'^')
//!     // use the European decimal point, so "1,2" and not "1.2"
//!     .decimal_point(b',')
//!     // write NaN and Infinity using the following formats
//!     .nan_string(Some(b"nan"))
//!     .inf_string(Some(b"inf"))
//!     // set the minimum and maximum number of significant digits to write;
//!     .min_significant_digits(num::NonZeroUsize::new(3))
//!     .max_significant_digits(num::NonZeroUsize::new(5))
//!     .build_strict();
//!
//! const BUFFER_SIZE: usize = CUSTOM.buffer_size_const::<f64, FORMAT>();
//! let mut buffer = [0u8; BUFFER_SIZE];
//!
//! // write 4 digits, no exponent notation
//! let digits = 1.234f64.to_lexical_with_options::<FORMAT>(&mut buffer, &CUSTOM);
//! assert_eq!(str::from_utf8(digits), Ok("1,234"));
//!
//! // write 6 digits, rounding to 5
//! let digits = 1.23456f64.to_lexical_with_options::<FORMAT>(&mut buffer, &CUSTOM);
//! assert_eq!(str::from_utf8(digits), Ok("1,2346"));
//!
//! // write 6 digits, rounding to 5, with exponent notation
//! let digits = 1.23456e300f64.to_lexical_with_options::<FORMAT>(&mut buffer, &CUSTOM);
//! assert_eq!(str::from_utf8(digits), Ok("1,2346^300"));
//!
//! // write 4 digits, no exponent notation
//! let digits = 1.2f64.to_lexical_with_options::<FORMAT>(&mut buffer, &CUSTOM);
//! assert_eq!(str::from_utf8(digits), Ok("1,20"));
//!
//! // write a literal NaN string
//! let digits = f64::NAN.to_lexical_with_options::<FORMAT>(&mut buffer, &CUSTOM);
//! assert_eq!(str::from_utf8(digits), Ok("nan"));
//!
//! // write a literal +Infinity string
//! let digits = f64::INFINITY.to_lexical_with_options::<FORMAT>(&mut buffer, &CUSTOM);
//! assert_eq!(str::from_utf8(digits), Ok("inf"));
//! ```
//!
//! # Higher-Level APIs
//!
//! If you would like support for writing to [`String`] directly, use
//! [`lexical`] instead. If you would like an API that supports multiple numeric
//! conversions rather than just writing integers, use [`lexical-core`] instead.
//!
//! [`lexical`]: https://crates.io/crates/lexical
//! [`lexical-core`]: https://crates.io/crates/lexical-core
//!
//! # Version Support
//!
//! The minimum, standard, required version is [`1.63.0`][`rust-1.63.0`], for
//! const generic support. Older versions of lexical support older Rust
//! versions.
//!
//! # Algorithms
//!
//! There's currently 5 algorithms used, depending on the requirements.
//!
//! 1. Compact for decimal strings uses the Grisu algorithm.
//! 2. An optimized algorithm based on the Dragonbox algorithm.
//! 3. An optimized algorithm for formatting to string with power-of-two
//!    radixes.
//! 4. An optimized algorithm for hexadecimal floats.
//! 5. A fallback algorithm for all other radixes.
//!
//! The Grisu algorithm is based on "Printing Floating-Point Numbers Quickly
//! and Accurately with Integers", by Florian Loitsch, available online
//! [here](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf).
//! The dragonbox algorithm is based on the reference C++ implementation,
//! hosted [here](https://github.com/jk-jeon/dragonbox/), and the algorithm
//! is described in depth
//! [here](https://github.com/jk-jeon/dragonbox/blob/master/other_files/Dragonbox.pdf).
//! The radix algorithm is adapted from the V8 codebase, and may be found
//! [here](https://github.com/v8/v8).
//!
//! # Design
//!
//! - [Algorithm Approach](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-float/docs/Algorithm.md)
//! - [Benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-float/docs/Benchmarks.md)
//! - [Comprehensive Benchmarks](https://github.com/Alexhuszagh/lexical-benchmarks)
//!
//! [`rust-1.63.0`]: https://blog.rust-lang.org/2022/08/11/Rust-1.63.0.html
//! [`String`]: https://doc.rust-lang.org/alloc/string/struct.String.html
//! [`to_string`]: https://doc.rust-lang.org/alloc/string/trait.ToString.html#tymethod.to_string

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

#[macro_use]
mod index;
#[macro_use]
mod shared;

pub mod algorithm;
pub mod binary;
pub mod compact;
pub mod float;
pub mod hex;
pub mod options;
pub mod radix;
pub mod table;
pub mod write;

mod api;
mod table_dragonbox;
mod table_grisu;

// Re-exports
#[cfg(feature = "f16")]
pub use lexical_util::bf16::bf16;
pub use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
pub use lexical_util::error::Error;
#[cfg(feature = "f16")]
pub use lexical_util::f16::f16;
pub use lexical_util::format::{self, NumberFormat, NumberFormatBuilder};
pub use lexical_util::options::WriteOptions;
pub use lexical_util::result::Result;

pub use self::api::{ToLexical, ToLexicalWithOptions};
#[doc(inline)]
pub use self::options::{Options, OptionsBuilder, RoundMode};
