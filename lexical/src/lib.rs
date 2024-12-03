//! Fast lexical conversion routines.
//!
//! Fast lexical conversion routines for both `std` and `no_std` environments.
//! lexical provides routines to convert numbers to and from decimal
//! strings. lexical also supports non-base 10 numbers, with the `radix`
//! feature, for both integers and floats. lexical is customizable
//! and yet simple to use: despite supporting nearly every float and
//! integer format available, it only exports 2 write functions
//! and 4 parse functions.
//!
//! lexical is well-tested, and has been downloaded more than 5 million
//! times and currently has no known errors in correctness. lexical
//! prioritizes performance above all else, and aims to be competitive
//! or faster than any other float or integer parser and writer.
//!
//! # Getting Started
//!
//! ```rust
//! # #[cfg(all(
//! #     feature = "parse-floats",
//! #     feature = "parse-integers",
//! #     feature = "write-floats",
//! #     feature = "write-integers",
//! # ))]
//! # {
//! // Number to string
//! lexical::to_string(3.0);            // "3.0", always has a fraction suffix.
//! lexical::to_string(3);              // "3"
//!
//! // String to number.
//! let i: i32 = lexical::parse("3").unwrap();      // 3, auto-type deduction.
//! let f: f32 = lexical::parse("3.5").unwrap();    // 3.5
//! let d = lexical::parse::<f64, _>("3.5");        // Ok(3.5), successful parse.
//! let d = lexical::parse::<f64, _>("3a");         // Err(Error(_)), failed to parse.
//! # }
//! ```
//!
//! # Conversion API
#![cfg_attr(feature = "write", doc = " **To String**")]
#![cfg_attr(feature = "write", doc = "")]
#![cfg_attr(feature = "write", doc = " - [`to_string`]")]
#![cfg_attr(feature = "write", doc = " - [`to_string_with_options`]")]
//!
#![cfg_attr(feature = "write", doc = " **From String**")]
#![cfg_attr(feature = "write", doc = "")]
#![cfg_attr(feature = "parse", doc = " - [`parse`]")]
#![cfg_attr(feature = "parse", doc = " - [`parse_partial`]")]
#![cfg_attr(feature = "parse", doc = " - [`parse_with_options`]")]
#![cfg_attr(feature = "parse", doc = " - [`parse_partial_with_options`]")]
//!
//! # Features
//!
//! In accordance with the Rust ethos, all features are additive: the crate
//! may be build with `--all-features` without issue.  The following features
//! are enabled by default:
//!
//! * `std`
//! * `write-integers`
//! * `write-floats`
//! * `parse-integers`
//! * `parse-floats`
//!
//! A complete description of supported features includes:
//!
//! #### std
//!
//! Enable use of the standard library. Currently, the standard library
//! is not used for any functionality, and may be disabled without any
//! change in functionality on stable.
//!
//! #### write-integers
//!
//! Enable support for writing integers to string.
//!
//! #### write-floats
//!
//! Enable support for writing floating-point numbers to string.
//!
//! #### parse-integers
//!
//! Enable support for parsing integers from string.
//!
//! #### parsing-floats
//!
//! Enable support for parsing floating-point numbers from string.
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
//! use lexical_core::{format, parse_with_options, ParseFloatOptions, Result};
//!
//! fn parse_json_float<Bytes: AsRef<[u8]>>(bytes: Bytes) -> Result<f64> {
//!     let options = ParseFloatOptions::new();
//!     parse_with_options::<_, { format::JSON }>(bytes.as_ref(), &options)
//! }
//! # }
//! ```
//!
//! See the [Number Format](#number-format) section below for more information.
//!
//! #### power-of-two
//!
//! Enable doing numeric conversions to and from strings with power-of-two
//! radixes. This avoids most of the overhead and binary bloat of the radix
//! feature, while enabling support for the most commonly-used radixes.
//!
//! #### radix
//!
//! Enable doing numeric conversions to and from strings for all radixes.
//! This requires substantially more static storage than `power-of-two`,
//! and increases compile times by a fair amount, but can be quite useful
//! for esoteric programming languages which use duodecimal floats, for
//! example.
//!
//! #### compact
//!
//! Reduce the generated code size at the cost of performance. This minimizes
//! the number of static tables, inlining, and generics used, drastically
//! reducing the size of the generated binaries.
//!
//! #### safe
//!
//! This replaces most unchecked indexing, required in cases where the
//! compiler cannot elide the check, with checked indexing. However,
//! it does not fully replace all unsafe behavior with safe behavior.
//! To minimize the risk of undefined behavior and out-of-bounds reads/writers,
//! extensive edge-cases, property-based tests, and fuzzing is done with both
//! the safe feature enabled and disabled, with the tests verified by Miri
//! and Valgrind.
//!
//! # Configuration API
//!
//! Lexical provides two main levels of configuration:
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
//! When the `format` feature is enabled, numerous other syntax and
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
//! - `JSON`, `XML`, `TOML`, `YAML`, `SQLite`, and many more.
//! - `Rust`, `Python`, `C#`, `FORTRAN`, `COBOL` literals and strings, and many
//!   more.
//!
//! ## Options API
//!
//! The Options API provides high-level options to specify number parsing
//! or writing, options not intrinsically tied to a number format.
//! For example, the Options API provides:
//! - The exponent character (default `b'e'`, or `b'^'`).
//! - The decimal point character (default `b'.'`).
//! - Custom `NaN`, `Infinity` string representations.
//! - Whether to trim the fraction component from integral floats.
//! - The exponent break point for scientific notation.
//! - The maximum and minimum number of significant digits to write.
//! - The rounding mode when truncating significant digits while writing.
//!
//! The available options are:
#![cfg_attr(feature = "parse-floats", doc = " - [`ParseFloatOptions`]")]
#![cfg_attr(feature = "parse-integers", doc = " - [`ParseIntegerOptions`]")]
#![cfg_attr(feature = "write-floats", doc = " - [`WriteFloatOptions`]")]
#![cfg_attr(feature = "write-integers", doc = " - [`WriteIntegerOptions`]")]
//!
//! In addition, pre-defined constants for each category of options may
//! be found in their respective modules.
//!
//! ## Example
//!
//! An example of creating your own options to parse European-style
//! numbers (which use commas as decimal points, and periods as digit
//! separators) is as follows:
//!
//! ```
//! # pub fn main() {
//! # #[cfg(all(feature = "parse_floats", feature = "format"))] {
//! // This creates a format to parse a European-style float number.
//! // The decimal point is a comma, and the digit separators (optional)
//! // are periods.
//! const EUROPEAN: u128 = lexical::NumberFormatBuilder::new()
//!     .digit_separator(b'.')
//!     .build()
//!     .unwrap();
//! let options = lexical_core::ParseFloatOptions::builder()
//!     .decimal_point(b',')
//!     .build()
//!     .unwrap();
//! assert_eq!(
//!     lexical::parse_with_options::<f32, EUROPEAN, _>("300,10", &options),
//!     Ok(300.10)
//! );
//!
//! // Another example, using a pre-defined constant for JSON.
//! const JSON: u128 = lexical::format::JSON;
//! let options = lexical::ParseFloatOptions::new();
//! assert_eq!(
//!     lexical::parse_with_options::<f32, JSON, _>("0e1", &options),
//!     Ok(0.0)
//! );
//! assert_eq!(
//!     lexical::parse_with_options::<f32, JSON, _>("1E+2", &options),
//!     Ok(100.0)
//! );
//! # }
//! # }
//! ```
//!
//! # Version Support
//!
//! The minimum, standard, required version is 1.63.0, for const generic
//! support. Older versions of lexical support older Rust versions.
//!
//! # Safety
//!
//! There is no non-trivial unsafe behavior in [lexical][crate] itself,
//! however, any incorrect safety invariants in our parsers and writers
//! (`lexical-parse-float`, `lexical-parse-integer`, `lexical-write-float`,
//! and `lexical-write-integer`) could cause those safety invariants to
//! be broken.
//!
//! [`to_string`]: fn.to_string.html
//! [`to_string_with_options`]: fn.to_string_with_options.html
//! [`write_with_options`]: crate::write_with_options
//! [`parse`]: crate::parse
//! [`parse_partial`]: crate::parse_partial
//! [`parse_with_options`]: crate::parse_with_options
//! [`parse_partial_with_options`]: crate::parse_partial_with_options
//!
//! [`NumberFormatBuilder`]: crate::NumberFormatBuilder
//! [`ParseFloatOptions`]: crate::ParseFloatOptions
//! [`ParseIntegerOptions`]: crate::ParseIntegerOptions
//! [`WriteFloatOptions`]: crate::WriteFloatOptions
//! [`WriteIntegerOptions`]: crate::WriteIntegerOptions

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    clippy::doc_markdown,
    clippy::unnecessary_safety_comment,
    clippy::semicolon_if_nothing_returned,
    clippy::unwrap_used,
    clippy::as_underscore,
    clippy::doc_markdown
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

// Ensure our features are properly enabled. This means no parse without
// parse support, etc.
#[cfg(all(feature = "parse", not(any(feature = "parse-integers", feature = "parse-floats"))))]
compile_error!(
    "Do not use the `parse` feature directly. Use `parse-integers` and/or `parse-floats` instead."
);

#[cfg(all(feature = "write", not(any(feature = "write-integers", feature = "write-floats"))))]
compile_error!(
    "Do not use the `write` feature directly. Use `write-integers` and/or `write-floats` instead."
);

#[cfg(all(feature = "integers", not(any(feature = "write-integers", feature = "parse-integers"))))]
compile_error!("Do not use the `integers` feature directly. Use `write-integers` and/or `parse-integers` instead.");

#[cfg(all(feature = "floats", not(any(feature = "write-floats", feature = "parse-floats"))))]
compile_error!(
    "Do not use the `floats` feature directly. Use `write-floats` and/or `parse-floats` instead."
);

// Need an allocator for String/Vec.
#[cfg(feature = "write")]
#[macro_use(vec)]
extern crate alloc;

#[cfg(feature = "write")]
use alloc::string::String;

pub use lexical_core::format::{self, format_error, format_is_valid, NumberFormatBuilder};
#[cfg(feature = "parse")]
pub use lexical_core::Error;
#[cfg(feature = "parse")]
pub use lexical_core::ParseOptions;
#[cfg(feature = "parse")]
pub use lexical_core::Result;
#[cfg(feature = "write")]
pub use lexical_core::WriteOptions;
#[cfg(feature = "f16")]
pub use lexical_core::{bf16, f16};
#[cfg(feature = "parse-floats")]
pub use lexical_core::{parse_float_options, ParseFloatOptions, ParseFloatOptionsBuilder};
#[cfg(feature = "parse-integers")]
pub use lexical_core::{parse_integer_options, ParseIntegerOptions, ParseIntegerOptionsBuilder};
#[cfg(feature = "write-floats")]
pub use lexical_core::{write_float_options, WriteFloatOptions, WriteFloatOptionsBuilder};
#[cfg(feature = "write-integers")]
pub use lexical_core::{write_integer_options, WriteIntegerOptions, WriteIntegerOptionsBuilder};
#[cfg(feature = "write")]
pub use lexical_core::{FormattedSize, BUFFER_SIZE};
#[cfg(feature = "parse")]
pub use lexical_core::{FromLexical, FromLexicalWithOptions};
#[cfg(feature = "write")]
pub use lexical_core::{ToLexical, ToLexicalWithOptions};

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
/// # extern crate lexical;
/// # pub fn main() {
/// assert_eq!(lexical::to_string(5), "5");
/// assert_eq!(lexical::to_string(0.0), "0.0");
/// # }
/// ```
#[inline]
#[cfg(feature = "write")]
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
/// # pub fn main() {
/// const FORMAT: u128 = lexical::format::STANDARD;
/// let options = lexical::WriteFloatOptions::builder()
///     .trim_floats(true)
///     .build()
///     .unwrap();
/// assert_eq!(lexical::to_string_with_options::<_, FORMAT>(0.0, &options), "0");
/// assert_eq!(lexical::to_string_with_options::<_, FORMAT>(123.456, &options), "123.456");
/// # }
/// ```
#[inline]
#[cfg(feature = "write")]
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
/// # extern crate lexical;
/// # use lexical::Error;
/// # pub fn main() {
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
/// # }
/// ```
#[inline]
#[cfg(feature = "parse")]
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
/// # extern crate lexical;
/// # pub fn main() {
///
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
/// # }
/// ```
#[inline]
#[cfg(feature = "parse")]
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
/// # pub fn main() {
/// const FORMAT: u128 = lexical::format::STANDARD;
/// let options = lexical::ParseFloatOptions::builder()
///     .exponent(b'^')
///     .decimal_point(b',')
///     .build()
///     .unwrap();
/// assert_eq!(lexical::parse_with_options::<f32, _, FORMAT>("0", &options), Ok(0.0));
/// assert_eq!(lexical::parse_with_options::<f32, _, FORMAT>("1,2345", &options), Ok(1.2345));
/// assert_eq!(lexical::parse_with_options::<f32, _, FORMAT>("1,2345^4", &options), Ok(12345.0));
/// # }
/// ```
#[inline]
#[cfg(feature = "parse")]
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
/// # pub fn main() {
/// const FORMAT: u128 = lexical::format::STANDARD;
/// let options = lexical::ParseFloatOptions::builder()
///     .exponent(b'^')
///     .decimal_point(b',')
///     .build()
///     .unwrap();
/// assert_eq!(lexical::parse_partial_with_options::<f32, _, FORMAT>("0", &options), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_with_options::<f32, _, FORMAT>("1,2345", &options), Ok((1.2345, 6)));
/// assert_eq!(lexical::parse_partial_with_options::<f32, _, FORMAT>("1,2345^4", &options), Ok((12345.0, 8)));
/// # }
/// ```
#[inline]
#[cfg(feature = "parse")]
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
