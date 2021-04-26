//! Fast lexical conversion routines for a no_std environment.
//!
//! lexical-core is a low-level API for number-to-string and
//! string-to-number conversions, without requiring a system
//! allocator. If you would like to use a convenient, high-level
//! API, please look at [lexical](https://crates.io/crates/lexical)
//! instead.
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
//! // TODO(ahuszagh) Add documentation here on NumberFormat and Parse/Write Options.
//!
//! [`write`]: fn.write.html
//! [`write_with_options`]: fn.write_with_options.html
//! [`parse`]: fn.parse.html
//! [`parse_with_options`]: fn.parse_with_options.html
//! [`parse_partial`]: fn.parse_partial.html
//! [`parse_partial_with_options`]: fn.parse_partial_with_options.html

// silence warnings for unused doc comments
#![allow(unused_doc_comments)]

// FEATURES

// Require intrinsics in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]

// DEPENDENCIES

#[macro_use]
extern crate cfg_if;

// Use vec if there is a system allocator, which we require only if
// we're using the correct and radix features.
#[cfg(all(not(feature = "std"), feature = "radix"))]
#[cfg_attr(test, macro_use)]
extern crate alloc;

// Use arrayvec for atof.
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
    }
}  // cfg_if

/// Facade around the core features for name mangling.
pub(crate) mod lib {
    #[cfg(feature = "std")]
    pub(crate) use std::*;

    #[cfg(not(feature = "std"))]
    pub(crate) use core::*;

    cfg_if! {
        if #[cfg(feature = "radix")] {
            #[cfg(feature = "std")]
            pub(crate) use std::vec::Vec;

            #[cfg(not(feature = "std"))]
            pub(crate) use ::alloc::vec::Vec;
        }
    }  // cfg_if
}   // lib

// API

// Hide implementation details
#[macro_use]
mod util;

#[cfg(any(feature = "atof", feature = "ftoa"))]
mod float;

#[cfg(feature = "atof")]
mod atof;

#[cfg(any(feature = "atof", feature = "atoi"))]
mod atoi;

#[cfg(feature = "ftoa")]
mod ftoa;

#[cfg(any(all(feature = "ftoa", feature = "radix"), feature = "itoa"))]
mod itoa;

// Re-export configuration and utilities globally.
pub use util::*;

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
pub fn write<'a, N: ToLexical>(n: N, bytes: &'a mut [u8])
    -> &'a mut [u8]
{
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
/// lexical_core::write_with_options(float, &mut buffer, &options);
/// ```
#[inline]
pub fn write_with_options<'a, N: ToLexicalOptions>(n: N, bytes: &'a mut [u8], options: &N::WriteOptions)
    -> &'a mut [u8]
{
    n.to_lexical_with_options(bytes, options)
}

/// Parse number from string.
///
/// This method parses the entire string, returning an error if
/// any invalid digits are found during parsing.
///
/// * `bytes`   - Byte slice containing a numeric string.
#[inline]
pub fn parse<N: FromLexical>(bytes: &[u8])
    -> Result<N>
{
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
pub fn parse_with_options<N: FromLexicalOptions>(bytes: &[u8], options: &N::ParseOptions)
    -> Result<N>
{
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
pub fn parse_partial<N: FromLexical>(bytes: &[u8])
    -> Result<(N, usize)>
{
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
pub fn parse_partial_with_options<N: FromLexicalOptions>(bytes: &[u8], options: &N::ParseOptions)
    -> Result<(N, usize)>
{
    N::from_lexical_partial_with_options(bytes, options)
}
