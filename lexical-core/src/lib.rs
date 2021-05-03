//! Fast lexical conversion routines for a no_std environment.
//!
//! lexical-core is a low-level API for number-to-string and
//! string-to-number conversions, without requiring a system
//! allocator. If you would like to use a high-level API that
//! writes to and parses from `String` and `&str`, respectively,
//! please look at [lexical](https://crates.io/crates/lexical)
//! instead.
//!
//! Despite the low-level API and focus on performance, lexical-core
//! strives to be simple and yet configurable: despite supporting nearly
//! every float and integer format available, it only exports 2 write
//! functions and 4 parse functions.
//!
//! lexical-core is well-tested, and has been downloaded more than 5 million
//! times and currently has no known errors in correctness. lexical-core
//! prioritizes performance above all else, and aims to be competitive
//! or faster than any other float or integer parser and writer.
//!
//! # Getting Started
//!
//! ```rust
//! extern crate lexical_core;
//!
//! // String to number using Rust slices.
//! // The argument is the byte string parsed.
//! let f: f32 = lexical_core::parse(b"3.5").unwrap();   // 3.5
//! let i: i32 = lexical_core::parse(b"15").unwrap();    // 15
//!
//! // All lexical_core parsers are checked, they validate the
//! // input data is entirely correct, and stop parsing when invalid data
//! // is found, or upon numerical overflow.
//! let r = lexical_core::parse::<u8>(b"256"); // Err(ErrorCode::Overflow.into())
//! let r = lexical_core::parse::<u8>(b"1a5"); // Err(ErrorCode::InvalidDigit.into())
//!
//! // In order to extract and parse a number from a substring of the input
//! // data, use `parse_partial`. These functions return the parsed value and
//! // the number of processed digits, allowing you to extract and parse the
//! // number in a single pass.
//! let r = lexical_core::parse_partial::<i8>(b"3a5"); // Ok((3, 1))
//!
//! // If an insufficiently long buffer is passed, the serializer will panic.
//! // PANICS
//! let mut buf = [b'0'; 1];
//! //let slc = lexical_core::write::<i64>(15, &mut buf);
//!
//! // In order to guarantee the buffer is long enough, always ensure there
//! // are at least `T::FORMATTED_SIZE` bytes, which requires the
//! // `lexical_core::Number` trait to be in scope.
//! use lexical_core::Number;
//! let mut buf = [b'0'; f64::FORMATTED_SIZE];
//! let slc = lexical_core::write::<f64>(15.1, &mut buf);
//! assert_eq!(slc, b"15.1");
//!
//! // When the `radix` feature is enabled, for decimal floats, using
//! // `T::FORMATTED_SIZE` may significantly overestimate the space
//! // required to format the number. Therefore, the
//! // `T::FORMATTED_SIZE_DECIMAL` constants allow you to get a much
//! // tighter bound on the space required.
//! let mut buf = [b'0'; f64::FORMATTED_SIZE_DECIMAL];
//! let slc = lexical_core::write::<f64>(15.1, &mut buf);
//! assert_eq!(slc, b"15.1");
//! ```
//!
//! # Conversion API
//!
//! **To String**
//! - [`write`]
//! - [`write_with_options`]
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
//! - The default exponent character (default `b'e'`).
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
//!     let format = lexical_core::NumberFormat::builder()
//!         .digit_separator(b'.')
//!         .decimal_point(b',')
//!         .build()
//!         .unwrap();
//!     let options = lexical_core::ParseFloatOptions::builder()
//!         .format(Some(format))
//!         .build()
//!         .unwrap();
//!     assert_eq!(
//!         lexical_core::parse_with_options::<f32>(b"300,10", &options),
//!         Ok(300.10)
//!     );
//!
//!     // Another example, using a pre-defined constant for JSON.
//!     let format = lexical_core::NumberFormat::JSON;
//!     let options = lexical_core::ParseFloatOptions::builder()
//!         .format(Some(format))
//!         .build()
//!         .unwrap();
//!     assert_eq!(
//!         lexical_core::parse_with_options::<f32>(b"0e1", &options),
//!         Ok(0.0)
//!     );
//!     assert_eq!(
//!         lexical_core::parse_with_options::<f32>(b"1E+2", &options),
//!         Ok(100.0)
//!     );
//! }
//! # }
//! ```
//!
//! [`write`]: fn.write.html
//! [`write_with_options`]: fn.write_with_options.html
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

// Silence warnings for unused doc comments
#![allow(unused_doc_comments)]
// FEATURES
// --------

// Require intrinsics in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]

// DEPENDENCIES
// ------------

#[macro_use]
extern crate cfg_if;

// Use vec if there is a system allocator, which we require only if
// we're using the correct and radix features.
#[cfg(all(
    not(feature = "std"),
    any(not(feature = "no_alloc"), feature = "f128", feature = "radix")
))]
#[cfg_attr(test, macro_use)]
extern crate alloc;

// Use arrayvec for atof.
#[cfg(feature = "no_alloc")]
extern crate arrayvec;

// Ensure only one back-end is enabled.
#[cfg(all(feature = "grisu3", feature = "ryu"))]
compile_error!("Lexical only accepts one of the following backends: `grisu3` or `ryu`.");

// Import the back-end, if applicable.
cfg_if! {
if #[cfg(feature = "grisu3")] {
    extern crate dtoa;
} else if #[cfg(feature = "ryu")] {
    extern crate ryu;
}} // cfg_if

/// Facade around the core features for name mangling.
pub(crate) mod lib {
    #[cfg(feature = "std")]
    pub(crate) use std::*;

    #[cfg(not(feature = "std"))]
    pub(crate) use core::*;

    cfg_if! {
    if #[cfg(any(not(feature = "no_alloc"), feature = "f128", feature = "radix"))] {
        #[cfg(feature = "std")]
        pub(crate) use std::vec::Vec;

        #[cfg(not(feature = "std"))]
        pub(crate) use ::alloc::vec::Vec;
    }} // cfg_if
} // lib

// MODULES
// -------

// Hide implementation details.
//
// To speed up compile times and simplify the internal logic,
// the following modules, in order, is as follows:
//      - config
//      - error
//      - result
//      - util
//      - options
//      - traits
//      - table
//      - float
//      - atoi/itoa
//      - atof/ftoa
//
// Modules should only import from other modules above them.
#[macro_use]
mod util;
#[macro_use]
mod options;
#[macro_use]
mod traits;

mod config;
mod error;
mod float;
mod result;
mod table;

// Re-export configuration, options, and utilities globally.
pub use config::*;
pub use error::*;
pub use options::*;
pub use result::*;
pub use table::*;
pub use traits::*;
pub use util::*;

// Submodules
mod atof;
mod atoi;
mod ftoa;
mod itoa;

// API
// ---

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
/// buffer with at least `{integer}::FORMATTED_SIZE_DECIMAL` elements.
///
/// # Example
///
/// ```
/// // import `Number` trait to get the `FORMATTED_SIZE_DECIMAL` of the number.
/// use lexical_core::Number;
///
/// let mut buffer = [0u8; f32::FORMATTED_SIZE_DECIMAL];
/// let float = 3.14159265359_f32;
///
/// lexical_core::write(float, &mut buffer);
///
/// assert_eq!(&buffer[0..9], b"3.1415927");
/// ```
///
/// This will panic, because the buffer is not large enough:
///
/// ```should_panic
/// // note: the buffer is only one byte large
/// let mut buffer = [0u8; 1];
/// let float = 3.14159265359_f32;
///
/// lexical_core::write(float, &mut buffer);
/// ```
#[inline]
pub fn write<'a, N: ToLexical>(n: N, bytes: &'a mut [u8]) -> &'a mut [u8] {
    n.to_lexical(bytes)
}

/// Write number to string with custom options.
///
/// Returns a subslice of the input buffer containing the written bytes,
/// starting from the same address in memory as the input slice.
///
/// * `value`   - Number to serialize.
/// * `bytes`   - Buffer to write number to.
/// * `options` - Options to customize number parsing.
///
/// # Panics
///
/// Panics if the buffer may not be large enough to hold the serialized
/// number. In order to ensure the function will not panic, provide a
/// buffer with at least `{integer}::FORMATTED_SIZE` elements.
///
/// # Example
///
/// ```
/// // import `Number` trait to get the `FORMATTED_SIZE` of the number.
/// use lexical_core::Number;
///
/// let mut buffer = [0u8; f32::FORMATTED_SIZE];
/// let float = 3.14159265359_f32;
///
/// let options = lexical_core::WriteFloatOptions::decimal();
/// lexical_core::write_with_options(float, &mut buffer, &options);
///
/// assert_eq!(&buffer[0..9], b"3.1415927");
/// ```
///
/// This will panic, because the buffer is not large enough:
///
/// ```should_panic
/// // note: the buffer is only one byte large
/// let mut buffer = [0u8; 1];
/// let float = 3.14159265359_f32;
///
/// let options = lexical_core::WriteFloatOptions::decimal();
/// lexical_core::write_with_options(float, &mut buffer, &options);
/// ```
#[inline]
pub fn write_with_options<'a, N: ToLexicalOptions>(
    n: N,
    bytes: &'a mut [u8],
    options: &N::WriteOptions,
) -> &'a mut [u8] {
    n.to_lexical_with_options(bytes, options)
}

/// Parse number from string.
///
/// This method parses the entire string, returning an error if
/// any invalid digits are found during parsing.
///
/// * `bytes`   - Byte slice containing a numeric string.
#[inline]
pub fn parse<N: FromLexical>(bytes: &[u8]) -> Result<N> {
    N::from_lexical(bytes)
}

/// Parse number from string with custom parsing options.
///
/// This method parses the entire string, returning an error if
/// any invalid digits are found during parsing.
///
/// * `bytes`   - Byte slice containing a numeric string.
/// * `options` - Options to customize number parsing.
#[inline]
pub fn parse_with_options<N: FromLexicalOptions>(
    bytes: &[u8],
    options: &N::ParseOptions,
) -> Result<N> {
    N::from_lexical_with_options(bytes, options)
}

/// Parse number from string.
///
/// This method parses until an invalid digit is found (or the end
/// of the string), returning the number of processed digits
/// and the parsed value until that point.
///
/// * `bytes`   - Byte slice containing a numeric string.
#[inline]
pub fn parse_partial<N: FromLexical>(bytes: &[u8]) -> Result<(N, usize)> {
    N::from_lexical_partial(bytes)
}

/// Parse number from string with custom parsing options.
///
/// This method parses until an invalid digit is found (or the end
/// of the string), returning the number of processed digits
/// and the parsed value until that point.
///
/// * `bytes`   - Byte slice containing a numeric string.
/// * `options` - Options to customize number parsing.
#[inline]
pub fn parse_partial_with_options<N: FromLexicalOptions>(
    bytes: &[u8],
    options: &N::ParseOptions,
) -> Result<(N, usize)> {
    N::from_lexical_partial_with_options(bytes, options)
}
