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
//! every float and integer format available, it only exports 4 write
//! functions and 4 parse functions.
//!
//! lexical-core is well-tested, and has been downloaded more than 5 million
//! times and currently has no known errors in correctness. lexical-core
//! prioritizes performance above all else, and aims to be competitive
//! or faster than any other float or integer parser and writer.
//!
//! In addition, despite having a large number of features, configurability,
//! and a focus on performance, we also strive for fast compile times.
//! Recent versions also add support for smaller binary sizes, as well
//! ideal for embedded or web environments, where executable bloat can
//! be much more detrimental than performance.
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
//!
//! // PANICS
//! let mut buf = [b'0'; 1];
//! //let slc = lexical_core::write::<i64>(15, &mut buf);
//!
//! // In order to guarantee the buffer is long enough, always ensure there
//! // are at least `T::FORMATTED_SIZE` bytes, which requires the
//! // `lexical_core::FormattedSize` trait to be in scope.
//! use lexical_core::FormattedSize;
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
//! # }
//! ```
//!
//! # Conversion API
//!
#![cfg_attr(feature = "write", doc = " **Write**")]
#![cfg_attr(feature = "write", doc = "")]
#![cfg_attr(feature = "write", doc = " - [`write`]")]
#![cfg_attr(feature = "write", doc = " - [`write_unchecked`]")]
#![cfg_attr(feature = "write", doc = " - [`write_with_options`]")]
#![cfg_attr(feature = "write", doc = " - [`write_with_options_unchecked`]")]
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
//! may be build with `--all-features` without issue.  The following features are enabled
//! by default:
//!
//! * `std`
//! * `write-integers`
//! * `write-floats`
//! * `parse-integers`
//! * `parse-floats`
//!
//! A complete description of supported features includes:
//!
//! ### std
//!
//! Enable use of the standard library. Currently, the standard library
//! is not used for any functionality, and may be disabled without any
//! change in functionality on stable.
//!
//! ### write-integers
//!
//! Enable support for writing integers to string.
//!
//! ### write-floats
//!
//! Enable support for writing floating-point numbers to string.
//!
//! ### parse-integers
//!
//! Enable support for parsing integers from string.
//!
//! ### parsing-floats
//!
//! Enable support for parsing floating-point numbers from string.
//!
//! ### format
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
//! ### power-of-two
//!
//! Enable doing numeric conversions to and from strings with power-of-two
//! radixes. This avoids most of the overhead and binary bloat of the radix
//! feature, while enabling support for the most commonly-used radixes.
//!
//! ### radix
//!
//! Enable doing numeric conversions to and from strings for all radixes.
//! This requires substantially more static storage than `power-of-two`,
//! and increases compile times by a fair amount, but can be quite useful
//! for esoteric programming languages which use duodecimal floats, for
//! example.
//!
//! ### compact
//!
//! Reduce the generated code size at the cost of performance. This minimizes
//! the number of static tables, inlining, and generics used, drastically
//! reducing the size of the generated binaries.
//!
//! ### safe
//!
//! All numeric parsers are memory-safe by default, since parsing complex
//! input is a major source of memory vulnerabilities. However, numeric
//! writers often opt-in for unchecked writes, for major performance
//! improvements. This may be disabled entirely by enabling the `safe`
//! feature. In addition, to simplify memory safety guarantees, extensive
//! edge-cases, property-based tests, and fuzzing is done with both the
//! safe feature enabled and disabled, with the tests verified by Miri
//! and Valgrind.
//!
//! # Configuration API
//!
//! Lexical provides two main levels of configuration:
//! - The [`NumberFormatBuilder`], creating a packed struct with custom
//!     formatting options.
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
//! - Whether leading, trailing, internal, and consecutive digit separators are allowed.
//! - Toggling required float components, such as digits before the decimal point.
//! - Toggling whether special floats are allowed or are case-sensitive.
//!
//! Many pre-defined constants therefore exist to simplify common use-cases,
//! including:
//! - JSON, XML, TOML, YAML, SQLite, and many more.
//! - Rust, Python, C#, FORTRAN, COBOL literals and strings, and many more.
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
//!
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
//! const EUROPEAN: u128 = lexical_core::NumberFormatBuilder::new()
//!     .digit_separator(b'.')
//!     .build();
//! let options = lexical_core::ParseFloatOptions::builder()
//!     .decimal_point(b',')
//!     .build()
//!     .unwrap();
//! assert_eq!(
//!     lexical_core::parse_with_options::<f32, EUROPEAN>(b"300,10", &options),
//!     Ok(300.10)
//! );
//!
//! // Another example, using a pre-defined constant for JSON.
//! const JSON: u128 = lexical_core::format::JSON;
//! let options = lexical_core::ParseFloatOptions::new();
//! assert_eq!(
//!     lexical_core::parse_with_options::<f32, JSON>(b"0e1", &options),
//!     Ok(0.0)
//! );
//! assert_eq!(
//!     lexical_core::parse_with_options::<f32, JSON>(b"1E+2", &options),
//!     Ok(100.0)
//! );
//! # }
//! # }
//! ```
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
//!
//! # Design
//!
//! - [Binary Size](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/BinarySize.md)
//! - [Build Timings](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/BuildTimings.md)
//! - [Digit Separators](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/DigitSeparators.md)
//!
//! # Version Support
//!
//! The minimum, standard, required version is 1.51.0, for const generic
//! support. Older versions of lexical support older Rust versions.
//!
//! [`write`]: crate::write
//! [`write_unchecked`]: crate::write_unchecked
//! [`write_with_options`]: crate::write_with_options
//! [`write_with_options_unchecked`]: crate::write_with_options_unchecked
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

#[cfg(feature = "parse-floats")]
use lexical_parse_float::{
    FromLexical as FromFloat,
    FromLexicalWithOptions as FromFloatWithOptions,
};
#[cfg(feature = "parse-integers")]
use lexical_parse_integer::{
    FromLexical as FromInteger,
    FromLexicalWithOptions as FromIntegerWithOptions,
};
#[cfg(feature = "parse")]
use lexical_util::{from_lexical, from_lexical_with_options};
#[cfg(feature = "write")]
use lexical_util::{to_lexical, to_lexical_with_options};
#[cfg(feature = "write-floats")]
use lexical_write_float::{ToLexical as ToFloat, ToLexicalWithOptions as ToFloatWithOptions};
#[cfg(feature = "write-integers")]
use lexical_write_integer::{ToLexical as ToInteger, ToLexicalWithOptions as ToIntegerWithOptions};

// Re-exports
#[cfg(feature = "parse-floats")]
pub use lexical_parse_float::{
    options as parse_float_options,
    Options as ParseFloatOptions,
    OptionsBuilder as ParseFloatOptionsBuilder,
};
#[cfg(feature = "parse-integers")]
pub use lexical_parse_integer::{
    options as parse_integer_options,
    Options as ParseIntegerOptions,
    OptionsBuilder as ParseIntegerOptionsBuilder,
};
#[cfg(feature = "f16")]
pub use lexical_util::bf16::bf16;
#[cfg(feature = "write")]
pub use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
#[cfg(feature = "parse")]
pub use lexical_util::error::Error;
#[cfg(feature = "f16")]
pub use lexical_util::f16::f16;
pub use lexical_util::format::{self, format_error, format_is_valid, NumberFormatBuilder};
#[cfg(feature = "parse")]
pub use lexical_util::options::ParseOptions;
#[cfg(feature = "write")]
pub use lexical_util::options::WriteOptions;
#[cfg(feature = "parse")]
pub use lexical_util::result::Result;
#[cfg(feature = "write-floats")]
pub use lexical_write_float::{
    options as write_float_options,
    Options as WriteFloatOptions,
    OptionsBuilder as WriteFloatOptionsBuilder,
};
#[cfg(feature = "write-integers")]
pub use lexical_write_integer::{
    options as write_integer_options,
    Options as WriteIntegerOptions,
    OptionsBuilder as WriteIntegerOptionsBuilder,
};

// API
// ---

#[cfg(feature = "parse")]
from_lexical!();
#[cfg(feature = "parse")]
from_lexical_with_options!();
#[cfg(feature = "write")]
to_lexical!();
#[cfg(feature = "write")]
to_lexical_with_options!();

/// Implement `FromLexical` and `FromLexicalWithOptions` for numeric type.
#[cfg(feature = "parse")]
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

// Implement ToLexical for numeric type.
#[cfg(feature = "write")]
macro_rules! to_lexical_impl {
    ($t:ident, $to:ident, $to_options:ident, $options:ident) => {
        impl ToLexical for $t {
            #[cfg_attr(not(feature = "compact"), inline)]
            unsafe fn to_lexical_unchecked<'a>(self, bytes: &'a mut [u8]) -> &'a mut [u8] {
                // SAFETY: safe as long as `bytes` is large enough to hold the significant digits.
                unsafe { <Self as $to>::to_lexical_unchecked(self, bytes) }
            }

            #[cfg_attr(not(feature = "compact"), inline)]
            fn to_lexical<'a>(self, bytes: &'a mut [u8]) -> &'a mut [u8] {
                <Self as $to>::to_lexical(self, bytes)
            }
        }

        impl ToLexicalWithOptions for $t {
            type Options = $options;

            #[cfg_attr(not(feature = "compact"), inline)]
            unsafe fn to_lexical_with_options_unchecked<'a, const FORMAT: u128>(
                self,
                bytes: &'a mut [u8],
                options: &Self::Options,
            ) -> &'a mut [u8] {
                // SAFETY: safe as long as `bytes` is large enough to hold the significant digits.
                unsafe {
                    <Self as $to_options>::to_lexical_with_options_unchecked::<FORMAT>(
                        self, bytes, options,
                    )
                }
            }

            #[cfg_attr(not(feature = "compact"), inline)]
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
/// # Example
///
/// ```
/// # pub fn main() {
/// #[cfg(feature = "write-floats")] {
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
#[cfg(feature = "write")]
pub fn write<N: ToLexical>(n: N, bytes: &mut [u8]) -> &mut [u8] {
    n.to_lexical(bytes)
}

/// Write number to string, without bounds checking the buffer.
///
/// Returns a subslice of the input buffer containing the written bytes,
/// starting from the same address in memory as the input slice.
///
/// * `value`   - Number to serialize.
/// * `bytes`   - Buffer to write number to.
///
/// # Safety
///
/// If the buffer is not be large enough to hold the serialized number,
/// it will overflow the buffer unless the `safe` feature is enabled.
/// Buffer overflows are severe security vulnerabilities, and therefore
/// to ensure the function will not overwrite the buffer, provide a
/// buffer with at least `{integer}::FORMATTED_SIZE` elements.
///
/// # Example
///
/// ```
/// # pub fn main() {
/// #[cfg(feature = "write-floats")] {
/// // import `BUFFER_SIZE` to get the maximum bytes written by the number.
/// use lexical_core::BUFFER_SIZE;
///
/// let mut buffer = [0u8; BUFFER_SIZE];
/// let float = 3.14159265359_f32;
///
/// unsafe {
///     lexical_core::write_unchecked(float, &mut buffer);
/// }
///
/// assert_eq!(&buffer[0..9], b"3.1415927");
/// # }
/// # }
/// ```
#[inline]
#[cfg(feature = "write")]
pub unsafe fn write_unchecked<N: ToLexical>(n: N, bytes: &mut [u8]) -> &mut [u8] {
    // SAFETY: safe if the provided buffer is large enough for the numerical string
    unsafe { n.to_lexical_unchecked(bytes) }
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
/// # Example
///
/// ```
/// # pub fn main() {
/// #[cfg(feature = "write-floats")] {
/// // import `BUFFER_SIZE` to get the maximum bytes written by the number.
/// use lexical_core::BUFFER_SIZE;
///
/// let mut buffer = [0u8; BUFFER_SIZE];
/// let float = 3.14159265359_f32;
///
/// const FORMAT: u128 = lexical_core::format::STANDARD;
/// let options = lexical_core::WriteFloatOptions::new();
/// lexical_core::write_with_options::<_, FORMAT>(float, &mut buffer, &options);
///
/// assert_eq!(&buffer[0..9], b"3.1415927");
/// # }
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
/// let options = lexical_core::WriteFloatOptions::new();
/// lexical_core::write_with_options::<_, FORMAT>(float, &mut buffer, &options);
/// # }
/// # #[cfg(not(feature = "write-floats"))] {
/// #     panic!("");
/// # }
/// ```
#[inline]
#[cfg(feature = "write")]
pub fn write_with_options<'a, N: ToLexicalWithOptions, const FORMAT: u128>(
    n: N,
    bytes: &'a mut [u8],
    options: &N::Options,
) -> &'a mut [u8] {
    n.to_lexical_with_options::<FORMAT>(bytes, options)
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
/// # Safety
///
/// If the buffer is not be large enough to hold the serialized number,
/// it will overflow the buffer unless the `safe` feature is enabled.
/// Buffer overflows are severe security vulnerabilities, and therefore
/// to ensure the function will not overwrite the buffer, provide a
/// buffer with at least `{integer}::FORMATTED_SIZE` elements. If you
/// are using custom digit precision control or exponent break points
/// for writing floats, these constants may be insufficient to store
/// the serialized number, and up to 1200 bytes may be required with
/// radix support.
///
/// # Panics
///
/// If the provided `FORMAT` is not valid, the function may panic. Please
/// ensure `is_valid()` is called prior to using the format, or checking
/// its validity using a static assertion.
///
/// # Example
///
/// ```
/// # pub fn main() {
/// #[cfg(feature = "write-floats")] {
/// // import `BUFFER_SIZE` to get the maximum bytes written by the number.
/// use lexical_core::BUFFER_SIZE;
///
/// let mut buffer = [0u8; BUFFER_SIZE];
/// let float = 3.14159265359_f32;
///
/// const FORMAT: u128 = lexical_core::format::STANDARD;
/// let options = lexical_core::WriteFloatOptions::new();
/// unsafe {
///     lexical_core::write_with_options_unchecked::<_, FORMAT>(float, &mut buffer, &options);
/// }
///
/// assert_eq!(&buffer[0..9], b"3.1415927");
/// # }
/// # }
/// ```
#[inline]
#[cfg(feature = "write")]
pub unsafe fn write_with_options_unchecked<'a, N: ToLexicalWithOptions, const FORMAT: u128>(
    n: N,
    bytes: &'a mut [u8],
    options: &N::Options,
) -> &'a mut [u8] {
    // SAFETY: safe if the provided buffer is large enough for the numerical string
    unsafe { n.to_lexical_with_options_unchecked::<FORMAT>(bytes, options) }
}

/// Parse complete number from string.
///
/// This method parses the entire string, returning an error if
/// any invalid digits are found during parsing.
///
/// * `bytes`   - Byte slice containing a numeric string.
///
/// # Example
///
/// ```
/// # pub fn main() {
/// #[cfg(feature = "parse-floats")] {
/// let string = "3.14159265359";
/// let result = lexical_core::parse::<f32>(string.as_bytes());
/// assert_eq!(result, Ok(3.14159265359_f32));
/// # }
/// # }
/// ```
#[inline]
#[cfg(feature = "parse")]
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
/// # Example
///
/// ```
/// # pub fn main() {
/// #[cfg(feature = "parse-floats")] {
/// let string = "3.14159265359 hello";
/// let result = lexical_core::parse_partial::<f32>(string.as_bytes());
/// assert_eq!(result, Ok((3.14159265359_f32, 13)));
/// # }
/// # }
/// ```
#[inline]
#[cfg(feature = "parse")]
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
/// # Example
///
/// ```
/// # pub fn main() {
/// #[cfg(all(feature = "parse-floats", feature = "format"))] {
/// const JSON: u128 = lexical_core::format::JSON;
/// let options = lexical_core::ParseFloatOptions::new();
/// let string = "3.14159265359";
/// let result = lexical_core::parse_with_options::<f32, JSON>(string.as_bytes(), &options);
/// assert_eq!(result, Ok(3.14159265359_f32));
/// # }
/// # }
/// ```
#[inline]
#[cfg(feature = "parse")]
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
/// # Example
///
/// ```
/// # pub fn main() {
/// #[cfg(all(feature = "parse-floats", feature = "format"))] {
/// const JSON: u128 = lexical_core::format::JSON;
/// let options = lexical_core::ParseFloatOptions::new();
/// let string = "3.14159265359 hello";
/// let result = lexical_core::parse_partial_with_options::<f32, JSON>(string.as_bytes(), &options);
/// assert_eq!(result, Ok((3.14159265359_f32, 13)));
/// # }
/// # }
/// ```
#[inline]
#[cfg(feature = "parse")]
pub fn parse_partial_with_options<N: FromLexicalWithOptions, const FORMAT: u128>(
    bytes: &[u8],
    options: &N::Options,
) -> Result<(N, usize)> {
    N::from_lexical_partial_with_options::<FORMAT>(bytes, options)
}
