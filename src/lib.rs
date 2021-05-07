//! Fast lexical conversion routines.
//!
//! Fast lexical conversion routines for both std and no_std environments.
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
//! extern crate lexical;
//!
//! // Number to string
//! lexical::to_string(3.0);            // "3.0", always has a fraction suffix.
//! lexical::to_string(3);              // "3"
//!
//! // String to number.
//! let i: i32 = lexical::parse("3").unwrap();      // 3, auto-type deduction.
//! let f: f32 = lexical::parse("3.5").unwrap();    // 3.5
//! let d = lexical::parse::<f64, _>("3.5");        // Ok(3.5), successful parse.
//! let d = lexical::parse::<f64, _>("3a");         // Err(Error(_)), failed to parse.
//! ```
//!
//! # Conversion API
//!
//! **To String**
//! - [`to_string`]
//! - [`to_string_with_options`]
//!
//! **From String**
//! - [`parse`]
//! - [`parse_with_options`]
//! - [`parse_partial`]
//! - [`parse_partial_with_options`]
//!
//! # Configuration API
//!
//! Lexical provides two main levels of configuration:
//! - The [`NumberFormat`] specifier.
//! - The Options API.
//!
//! ## NumberFormat
//!
//! The number format class provides numerous flags to specify
//! number parsing or writing, including:
//! - The decimal exponent character (default `b'e'`).
//! - The backup exponent character (for large radixes, default `b'^'`).
//! - The decimal point character (default `b'.'`).
//!
//! Other features, when the `format` feature is enabled, include:
//! - A digit separator character, to group digits for increased legibility.
//! - Toggling required float components, such as digits before the decimal point.
//! - Toggling whether special floats are allowed or are case-sensitive.
//! - Toggling the valid locations for digit separators, such as if consecutive digit separators are valid.
//!
//! The number format flags therefore provide extensive customizability,
//! and pre-defined constants exist when the `format` feature is enabled,
//! including:
//! - JSON, XML, TOML, YAML, SQLite, and many more.
//! - Rust, Python, C#, FORTRAN, COBOL literals and strings, and many more.
//!
//! ## Options API
//!
//! The Options API provides high-level options to specify number parsing
//! or writing, options not intrinsically tied to a number format.
//! For example, the Options API provides:
//! - Custom `NaN`, `Infinity` string representations.
//! - Different numerical bases (radixes) if the `radix` feature is enabled.
//! - Algorithm selection for parsing.
//! - Whether to trim the fraction component from integral floats.
//! - The `NumberFormat` to use.
//!
//! The available options are:
//! - [`ParseFloatOptions`]
//! - [`ParseIntegerOptions`]
//! - [`WriteFloatOptions`]
//! - [`WriteIntegerOptions`]
//!
//! ## Example
//!
//! An example of creating your own options to parse European-style
//! numbers (which use commas as decimal points, and periods as digit
//! separators) is as follows:
//!
//! ```
//! # pub fn main() {
//! #[cfg(feature = "format")] {
//!     // This creates a format to parse a European-style float number.
//!     // The decimal point is a comma, and the digit separators (optional)
//!     // are periods.
//!     let format = lexical::NumberFormat::builder()
//!         .digit_separator(b'.')
//!         .decimal_point(b',')
//!         .build()
//!         .unwrap();
//!     let options = lexical::ParseFloatOptions::builder()
//!         .format(Some(format))
//!         .build()
//!         .unwrap();
//!     assert_eq!(
//!         lexical::parse_with_options::<f32, _>("300,10", &options),
//!         Ok(300.10)
//!     );
//!
//!     // Another example, using a pre-defined constant for JSON.
//!     let format = lexical::NumberFormat::JSON;
//!     let options = lexical::ParseFloatOptions::builder()
//!         .format(Some(format))
//!         .build()
//!         .unwrap();
//!     assert_eq!(
//!         lexical::parse_with_options::<f32, _>("0e1", &options),
//!         Ok(0.0)
//!     );
//!     assert_eq!(
//!         lexical::parse_with_options::<f32, _>("1E+2", &options),
//!         Ok(100.0)
//!     );
//! }
//! # }
//! ```
//!
//! [`to_string`]: fn.to_string.html
//! [`to_string_with_options`]: fn.to_string_with_options.html
//!
//! [`parse`]: fn.parse.html
//! [`parse_with_options`]: fn.parse_with_options.html
//! [`parse_partial`]: fn.parse_partial.html
//! [`parse_partial_with_options`]: fn.parse_partial_with_options.html
//!
//! [`NumberFormat`]: struct.NumberFormat.html
//! [`ParseFloatOptions`]: struct.ParseFloatOptions.html
//! [`ParseIntegerOptions`]: struct.ParseIntegerOptions.html
//! [`WriteFloatOptions`]: struct.WriteFloatOptions.html
//! [`WriteIntegerOptions`]: struct.WriteIntegerOptions.html

// FEATURES

// Require intrinsics and alloc in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]

// EXTERNAL

#[macro_use]
extern crate cfg_if;
extern crate lexical_core;

// CONFIG

// Need an allocator for String/Vec.
#[cfg(not(feature = "std"))]
extern crate alloc;

/// Facade around the core features for name mangling.
pub(crate) mod lib {
    cfg_if! {
    if #[cfg(feature = "std")] {
        pub(crate) use std::*;
    } else {
        pub(crate) use core::*;
    }} // cfg_if

    cfg_if! {
    if #[cfg(feature = "std")] {
        pub(crate) use std::string::String;
        pub(crate) use std::vec::Vec;
    } else {
        pub(crate) use ::alloc::string::String;
        pub(crate) use ::alloc::vec::Vec;
    }} // cfg_if
} // cfg_if

// API

// Re-export the float rounding scheme used.
pub use lexical_core::RoundingKind;

// Re-export the numerical format.
pub use lexical_core::{NumberFormat, NumberFormatBuilder};

// Re-export the Result, Error and ErrorCode globally.
pub use lexical_core::{Error, ErrorCode, Result};
pub use lexical_core::{ParseError, ParseErrorCode, ParseResult};

// Re-export the parsing options.
pub use lexical_core::{ParseFloatOptions, ParseFloatOptionsBuilder};
pub use lexical_core::{ParseIntegerOptions, ParseIntegerOptionsBuilder};
pub use lexical_core::{WriteFloatOptions, WriteFloatOptionsBuilder};
pub use lexical_core::{WriteIntegerOptions, WriteIntegerOptionsBuilder};

// Publicly expose traits so they may be used for generic programming.
pub use lexical_core::{FromLexical, FromLexicalOptions};
pub use lexical_core::{ToLexical, ToLexicalOptions};

// HELPERS

/// Get a vector as a slice, including the capacity.
#[inline]
unsafe fn vector_as_slice<T>(buf: &mut lib::Vec<T>) -> &mut [T] {
    let first = buf.as_mut_ptr();
    lib::slice::from_raw_parts_mut(first, buf.capacity())
}

// HIGH LEVEL

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
pub fn to_string<N: ToLexical>(n: N) -> lib::String {
    unsafe {
        let mut buf = lib::Vec::<u8>::with_capacity(N::FORMATTED_SIZE_DECIMAL);
        let len = lexical_core::write(n, vector_as_slice(&mut buf)).len();
        buf.set_len(len);
        lib::String::from_utf8_unchecked(buf)
    }
}

/// High-level conversion of a number to a string with custom writing options.
///
/// * `n`       - Number to convert to string.
/// * `options` - Options to specify number writing.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// let options = lexical::WriteFloatOptions::builder()
///     .trim_floats(true)
///     .build()
///     .unwrap();
/// assert_eq!(lexical::to_string_with_options(0.0, &options), "0");
/// assert_eq!(lexical::to_string_with_options(123.456, &options), "123.456");
/// # }
/// ```
#[inline]
pub fn to_string_with_options<N: ToLexicalOptions>(n: N, options: &N::WriteOptions) -> lib::String {
    #[cfg(feature = "radix")]
    let size = N::FORMATTED_SIZE;
    #[cfg(not(feature = "radix"))]
    let size = N::FORMATTED_SIZE_DECIMAL;

    unsafe {
        let mut buf = lib::Vec::<u8>::with_capacity(size);
        let len = lexical_core::write_with_options(n, vector_as_slice(&mut buf), &options).len();
        buf.set_len(len);
        lib::String::from_utf8_unchecked(buf)
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
/// # use lexical::ErrorCode;
/// # pub fn main() {
/// // Create our error.
/// fn err_code<T>(r: lexical::Result<T>) -> ErrorCode {
///     r.err().unwrap().code
/// }
///
/// // String overloads
/// assert_eq!(lexical::parse::<i32, _>("5"), Ok(5));
/// assert_eq!(err_code(lexical::parse::<i32, _>("1a")), ErrorCode::InvalidDigit);
/// assert_eq!(lexical::parse::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::parse::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::parse::<f32, _>("1."), Ok(1.0));
///
/// // Bytes overloads
/// assert_eq!(lexical::parse::<i32, _>(b"5"), Ok(5));
/// assert_eq!(err_code(lexical::parse::<i32, _>(b"1a")), ErrorCode::InvalidDigit);
/// assert_eq!(lexical::parse::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::parse::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::parse::<f32, _>(b"1."), Ok(1.0));
/// # assert_eq!(lexical::parse::<f32, _>(b"5.002868148396374"), Ok(5.002868148396374));
/// # assert_eq!(lexical::parse::<f64, _>(b"5.002868148396374"), Ok(5.002868148396374));
/// # }
/// ```
#[inline]
pub fn parse<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes) -> Result<N> {
    N::from_lexical(bytes.as_ref())
}

/// High-level conversion of bytes to a number with custom parsing options.
///
/// This function only returns a value if the entire string is
/// successfully parsed.
///
/// * `bytes`   - Byte slice to convert to number.
/// * `options` - Options to specify number parsing.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// let format = lexical::NumberFormat::builder()
///     .exponent_decimal(b'^')
///     .decimal_point(b',')
///     .build()
///     .unwrap();
///
/// let options = lexical::ParseFloatOptions::builder()
///     .format(Some(format))
///     .build()
///     .unwrap();
///
/// assert_eq!(lexical::parse_with_options::<f32, _>("0", &options), Ok(0.0));
/// assert_eq!(lexical::parse_with_options::<f32, _>("1,2345", &options), Ok(1.2345));
/// assert_eq!(lexical::parse_with_options::<f32, _>("1,2345^4", &options), Ok(12345.0));
/// # }
/// ```
#[inline]
pub fn parse_with_options<N: FromLexicalOptions, Bytes: AsRef<[u8]>>(
    bytes: Bytes,
    options: &N::ParseOptions,
) -> Result<N> {
    N::from_lexical_with_options(bytes.as_ref(), options)
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
/// # use lexical::ErrorCode;
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
pub fn parse_partial<N: FromLexical, Bytes: AsRef<[u8]>>(bytes: Bytes) -> Result<(N, usize)> {
    N::from_lexical_partial(bytes.as_ref())
}

/// High-level, partial conversion of bytes to a number with custom parsing options.
///
/// This functions parses as many digits as possible, returning the parsed
/// value and the number of digits processed if at least one character
/// is processed. If another error, such as numerical overflow or underflow
/// occurs, this function returns the error code and the index at which
/// the error occurred.
///
/// * `bytes`   - Byte slice to convert to number.
/// * `options` - Options to specify number parsing.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// let format = lexical::NumberFormat::builder()
///     .exponent_decimal(b'^')
///     .decimal_point(b',')
///     .build()
///     .unwrap();
///
/// let options = lexical::ParseFloatOptions::builder()
///     .format(Some(format))
///     .build()
///     .unwrap();
///
/// assert_eq!(lexical::parse_partial_with_options::<f32, _>("0", &options), Ok((0.0, 1)));
/// assert_eq!(lexical::parse_partial_with_options::<f32, _>("1,2345", &options), Ok((1.2345, 6)));
/// assert_eq!(lexical::parse_partial_with_options::<f32, _>("1,2345^4", &options), Ok((12345.0, 8)));
/// # }
/// ```
#[inline]
pub fn parse_partial_with_options<N: FromLexicalOptions, Bytes: AsRef<[u8]>>(
    bytes: Bytes,
    options: &N::ParseOptions,
) -> Result<(N, usize)> {
    N::from_lexical_partial_with_options(bytes.as_ref(), options)
}
