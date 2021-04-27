//! Fast lexical conversion routines.
//!
//! Fast lexical conversion routines for both std and no_std environments.
//! Lexical provides routines to convert numbers to and from decimal
//! strings. Lexical also supports non-base 10 numbers, with the `radix`
//! feature, for both integers and floats. Lexical is simple to use,
//! and exports up to 10 functions in the high-level API.
//!
//! # Getting Started
//!
//! ```rust
//! # #[cfg(all(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa"))] {
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
//! # }
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
//! // TODO(ahuszagh) Add documentation here on NumberFormat and Parse/Write Options.
//!
//! [`to_string`]: fn.to_string.html
//! [`to_string_with_options`]: fn.to_string_with_options.html
//!
//! [`parse`]: fn.parse.html
//! [`parse_with_options`]: fn.parse_with_options.html
//! [`parse_partial`]: fn.parse_partial.html
//! [`parse_partial_with_options`]: fn.parse_partial_with_options.html

// FEATURES

// Require intrinsics and alloc in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]

// EXTERNAL

#[macro_use]
#[cfg(any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa"))]
extern crate cfg_if;

extern crate lexical_core;

// CONFIG

// Need an allocator for String/Vec.
#[cfg(all(not(feature = "std"), any(feature = "ftoa", feature = "itoa")))]
extern crate alloc;

/// Facade around the core features for name mangling.
#[cfg(any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa"))]
pub(crate) mod lib {
    cfg_if! {
    if #[cfg(feature = "std")] {
        pub(crate) use std::*;
    } else {
        pub(crate) use core::*;
    }} // cfg_if

    cfg_if! {
    if #[cfg(feature = "std")] {
        #[cfg(any(feature = "ftoa", feature = "itoa"))]
        pub(crate) use std::string::String;
        #[cfg(any(feature = "ftoa", feature = "itoa"))]
        pub(crate) use std::vec::Vec;
    } else {
        #[cfg(any(feature = "ftoa", feature = "itoa"))]
        pub(crate) use ::alloc::string::String;
        #[cfg(any(feature = "ftoa", feature = "itoa"))]
        pub(crate) use ::alloc::vec::Vec;
    }
    }
} // cfg_if

// API

// Re-export the float rounding scheme used.
#[cfg(all(any(feature = "atof", feature = "ftoa"), feature = "rounding"))]
pub use lexical_core::RoundingKind;

// Re-export the numerical format.
pub use lexical_core::NumberFormat;

// Re-export the Result, Error and ErrorCode globally.
#[cfg(any(feature = "atof", feature = "atoi"))]
pub use lexical_core::{Error, ErrorCode, Result};

// Re-export the parsing options.
#[cfg(feature = "atof")]
pub use lexical_core::ParseFloatOptions;

#[cfg(feature = "atoi")]
pub use lexical_core::ParseIntegerOptions;

#[cfg(feature = "ftoa")]
pub use lexical_core::WriteFloatOptions;

#[cfg(feature = "itoa")]
pub use lexical_core::WriteIntegerOptions;

// Publicly expose traits so they may be used for generic programming.
#[cfg(any(feature = "atof", feature = "atoi"))]
pub use lexical_core::{FromLexical, FromLexicalOptions};

#[cfg(any(feature = "ftoa", feature = "itoa"))]
pub use lexical_core::{ToLexical, ToLexicalOptions};

// HELPERS

/// Get a vector as a slice, including the capacity.
#[inline]
#[cfg(any(feature = "ftoa", feature = "itoa"))]
unsafe fn vector_as_slice<T>(buf: &mut lib::Vec<T>) -> &mut [T] {
    let first = buf.as_mut_ptr();
    lib::slice::from_raw_parts_mut(first, buf.capacity())
}

// HIGH LEVEL

#[cfg(any(feature = "atof", feature = "atoi"))]
use lib::convert::AsRef;

/// High-level conversion of a number to a decimal-encoded string.
///
/// * `n`       - Number to convert to string.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "ftoa")] {
/// # extern crate lexical;
/// # pub fn main() {
/// assert_eq!(lexical::to_string(5), "5");
/// ##[cfg(not(feature = "trim_floats"))]
/// assert_eq!(lexical::to_string(0.0), "0.0");
/// # }
/// # }
/// ```
#[inline]
#[cfg(any(feature = "ftoa", feature = "itoa"))]
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
// TODO(ahuszagh) Add examples and doctests
#[inline]
#[cfg(any(feature = "ftoa", feature = "itoa"))]
pub fn to_string_with_options<N: ToLexicalOptions>(n: N, options: &N::WriteOptions)
    -> lib::String
{
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
/// # #[cfg(all(feature = "atof", feature = "atoi"))] {
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
/// # }
/// ```
#[inline]
#[cfg(any(feature = "atof", feature = "atoi"))]
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
// TODO(ahuszagh) Add examples and doctests
#[inline]
#[cfg(any(feature = "atof", feature = "atoi"))]
pub fn parse_with_options<N: FromLexicalOptions, Bytes: AsRef<[u8]>>(bytes: Bytes, options: &N::ParseOptions)
    -> Result<N>
{
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
/// # #[cfg(all(feature = "atof", feature = "atoi"))] {
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
/// # }
/// ```
#[inline]
#[cfg(any(feature = "atof", feature = "atoi"))]
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
// TODO(ahuszagh) Add examples and doctests
#[inline]
#[cfg(any(feature = "atof", feature = "atoi"))]
pub fn parse_partial_with_options<N: FromLexicalOptions, Bytes: AsRef<[u8]>>(bytes: Bytes, options: &N::ParseOptions)
    -> Result<(N, usize)>
{
    N::from_lexical_partial_with_options(bytes.as_ref(), options)
}
