//! Fast lexical conversion routines.
//!
//! Fast lexical conversion routines for both std and no_std environments.
//! Lexical provides routines to convert numbers to and from decimal
//! strings. Lexical also supports non-base 10 numbers, for both integers
//! and floats.  Lexical is simple to use, and exports only 6 functions
//! in the high-level API.
//!
//! Lexical heavily uses unsafe code for performance, and therefore may
//! introduce memory-safety issues. Although the code is tested with
//! wide variety of inputs to minimize the risk of memory-safety bugs,
//! no guarantees are made and you should use it at your own risk.

// FEATURES

// Require intrinsics and alloc in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]
#![cfg_attr(feature = "alloc", feature(alloc))]

// EXTERNAL

#[macro_use]
extern crate cfg_if;

// CONFIG

cfg_if! {
// Require alloc and use wee_alloc as the default allocator for unittesting.
if #[cfg(all(feature = "alloc", not(feature = "std")))] {
extern crate alloc;
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
}}  // cfg_if

// Testing assertions for floating-point equality.
#[cfg(test)]
#[macro_use]
extern crate approx;

// Ensure only one back-end is enabled.
#[cfg(all(feature = "grisu3", feature = "ryu"))]
compile_error!("Lexical only accepts one of the following backends: `grisu3` or `ryu`.");

// Import the back-end, if applicable.
cfg_if! {
if #[cfg(feature = "grisu3")] {
    extern crate dtoa;
} else if #[cfg(feature = "ryu")] {
    extern crate ryu;
}}  // cfg_if

/// Facade around the core features for name mangling.
pub(crate) mod lib {
cfg_if! {
if #[cfg(feature = "std")] {
    pub(crate) use std::*;
} else {
    pub(crate) use core::*;
}}  // cfg_if

cfg_if! {
if #[cfg(all(feature = "alloc", not(feature = "std")))] {
    pub(crate) use alloc::string::String;
    pub(crate) use alloc::vec::Vec;
} else if #[cfg(feature = "std")] {
    pub(crate) use std::string::String;
    pub(crate) use std::vec::Vec;
}
}}  // cfg_if

// API

// Hide the implementation details.
mod error;
mod float;
mod table;
mod traits;

#[macro_use]
mod util;

// Publicly export the low-level APIs.
pub mod atof;
pub mod atoi;
pub mod itoa;
pub mod ftoa;

// Re-export EXPONENT_DEFAULT_CHAR and EXPONENT_BACKUP_CHAR globally.
pub use util::{EXPONENT_DEFAULT_CHAR, EXPONENT_BACKUP_CHAR};

// Re-export NAN_STRING and INFINITY_STRING globally.
pub use util::{INFINITY_STRING, NAN_STRING};

// Re-export the Error and ErrorKind globally.
pub use error::{Error, ErrorKind};

// HIGH LEVEL

use lib::convert::AsRef;

use traits::{FromBytes, FromBytesLossy};

#[cfg(any(feature = "std", feature = "alloc"))]
use traits::ToBytes;

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
#[inline(always)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn to_string<N: ToBytes>(n: N) -> lib::String {
    unsafe { lib::String::from_utf8_unchecked(n.to_bytes(10)) }
}

/// High-level conversion of a number to string with a custom radix.
///
/// * `n`       - Number to convert to string.
/// * `base`    - Number of unique digits for the number (radix).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// assert_eq!(lexical::to_string_radix(5, 10), "5");
/// assert_eq!(lexical::to_string_radix(0.0, 10), "0.0");
/// # }
/// ```
#[inline(always)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn to_string_radix<N: ToBytes>(n: N, radix: u8) -> lib::String {
    assert!(2 <= radix && radix <= 36, "to_string_radix, radix must be in range `[2, 36]`, got {}", radix);
    unsafe { lib::String::from_utf8_unchecked(n.to_bytes(radix)) }
}

/// High-level conversion of decimal-encoded bytes to a number.
///
/// This function **always** returns a number, parsing until invalid
/// digits are found. For an error-checking version of this function,
/// use [`try_parse`].
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// // String overloads
/// assert_eq!(lexical::parse::<i32, _>("5"), 5);
/// assert_eq!(lexical::parse::<i32, _>("1a"), 1);
/// assert_eq!(lexical::parse::<f32, _>("0"), 0.0);
/// assert_eq!(lexical::parse::<f32, _>("1."), 1.0);
/// assert_eq!(lexical::parse::<f32, _>("1.0"), 1.0);
///
/// // Bytes overloads
/// assert_eq!(lexical::parse::<i32, _>(b"5"), 5);
/// assert_eq!(lexical::parse::<i32, _>(b"1a"), 1);
/// assert_eq!(lexical::parse::<f32, _>(b"0"), 0.0);
/// assert_eq!(lexical::parse::<f32, _>(b"1."), 1.0);
/// assert_eq!(lexical::parse::<f32, _>(b"1.0"), 1.0);
/// # }
/// ```
///
/// [`try_parse`]: fn.try_parse.html
#[inline(always)]
pub fn parse<N: FromBytes, Bytes: AsRef<[u8]>>(bytes: Bytes) -> N {
    N::from_bytes(bytes.as_ref(), 10)
}

/// High-level lossy conversion of decimal-encoded bytes to a number.
///
/// This function **always** returns a number, parsing until invalid
/// digits are found. For an error-checking version of this function,
/// use [`try_parse`].
///
/// This function uses aggressive optimizations to avoid worst-case
/// scenarios, and can return inaccurate results. For guaranteed accurate
/// floats, use [`parse`].
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// // String overloads
/// assert_eq!(lexical::parse_lossy::<f32, _>("0"), 0.0);
/// assert_eq!(lexical::parse_lossy::<f32, _>("1."), 1.0);
/// assert_eq!(lexical::parse_lossy::<f32, _>("1.0"), 1.0);
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_lossy::<f32, _>(b"0"), 0.0);
/// assert_eq!(lexical::parse_lossy::<f32, _>(b"1."), 1.0);
/// assert_eq!(lexical::parse_lossy::<f32, _>(b"1.0"), 1.0);
/// # }
/// ```
///
/// [`parse`]: fn.parse.html
/// [`try_parse`]: fn.try_parse.html
#[inline(always)]
pub fn parse_lossy<N: FromBytesLossy, Bytes: AsRef<[u8]>>(bytes: Bytes) -> N {
    N::from_bytes(bytes.as_ref(), 10)
}

/// High-level conversion of bytes to a number with a custom radix.
///
/// This function **always** returns a number, parsing until invalid
/// digits are found. For an error-checking version of this function,
/// use [`try_parse_radix`].
///
/// * `bytes`   - Byte slice to convert to number.
/// * `radix`   - Number of unique digits for the number (base).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// // String overloads
/// assert_eq!(lexical::parse_radix::<i32, _>("5", 10), 5);
/// assert_eq!(lexical::parse_radix::<i32, _>("1a", 10), 1);
/// assert_eq!(lexical::parse_radix::<f32, _>("0", 10), 0.0);
/// assert_eq!(lexical::parse_radix::<f32, _>("1.", 10), 1.0);
/// assert_eq!(lexical::parse_radix::<f32, _>("1.0", 10), 1.0);
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_radix::<i32, _>(b"5", 10), 5);
/// assert_eq!(lexical::parse_radix::<i32, _>(b"1a", 10), 1);
/// assert_eq!(lexical::parse_radix::<f32, _>(b"0", 10), 0.0);
/// assert_eq!(lexical::parse_radix::<f32, _>(b"1.", 10), 1.0);
/// assert_eq!(lexical::parse_radix::<f32, _>(b"1.0", 10), 1.0);
/// # }
/// ```
///
/// [`try_parse_radix`]: fn.try_parse_radix.html
#[inline(always)]
pub fn parse_radix<N: FromBytes, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8) -> N {
    assert!(2 <= radix && radix <= 36, "parse_radix, radix must be in range `[2, 36]`, got {}", radix);
    N::from_bytes(bytes.as_ref(), radix)
}

/// High-level lossy conversion of bytes to a number with a custom radix.
///
/// This function **always** returns a number, parsing until invalid
/// digits are found. For an error-checking version of this function,
/// use [`try_parse_radix`].
///
/// This function uses aggressive optimizations to avoid worst-case
/// scenarios, and can return inaccurate results. For guaranteed accurate
/// floats, use [`parse_radix`].
///
/// * `bytes`   - Byte slice to convert to number.
/// * `radix`   - Number of unique digits for the number (base).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # pub fn main() {
/// // String overloads
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>("0", 10), 0.0);
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>("1.", 10), 1.0);
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>("1.0", 10), 1.0);
///
/// // Bytes overloads
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>(b"0", 10), 0.0);
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>(b"1.", 10), 1.0);
/// assert_eq!(lexical::parse_lossy_radix::<f32, _>(b"1.0", 10), 1.0);
/// # }
/// ```
///
/// [`parse_radix`]: fn.parse_radix.html
/// [`try_parse_radix`]: fn.try_parse_radix.html
#[inline(always)]
pub fn parse_lossy_radix<N: FromBytesLossy, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8) -> N {
    assert!(2 <= radix && radix <= 36, "parse_radix, radix must be in range `[2, 36]`, got {}", radix);
    N::from_bytes(bytes.as_ref(), radix)
}

/// High-level conversion of decimal-encoded bytes to a number.
///
/// This function only returns a value if the entire string is
/// successfully parsed. For an unchecked version of this function,
/// use [`parse`].
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorKind;
/// # pub fn main() {
/// // Create our error.
/// let err = |u| From::from(ErrorKind::InvalidDigit(u));
///
/// // String overloads
/// assert_eq!(lexical::try_parse::<i32, _>("5"), Ok(5));
/// assert_eq!(lexical::try_parse::<i32, _>("1a"), Err(err(1)));
/// assert_eq!(lexical::try_parse::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::try_parse::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse::<f32, _>("1."), Err(err(1)));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse::<i32, _>(b"5"), Ok(5));
/// assert_eq!(lexical::try_parse::<i32, _>(b"1a"), Err(err(1)));
/// assert_eq!(lexical::try_parse::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::try_parse::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse::<f32, _>(b"1."), Err(err(1)));
/// # }
/// ```
///
/// [`parse`]: fn.parse.html
#[inline(always)]
pub fn try_parse<N: FromBytes, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<N, Error>
{
    N::try_from_bytes(bytes.as_ref(), 10)
}

/// High-level lossy conversion of decimal-encoded bytes to a number.
/// This function uses aggressive optimizations to avoid worst-case
/// scenarios, and can return inaccurate results. For guaranteed accurate
/// floats, use [`try_parse`].
///
/// * `bytes`   - Byte slice to convert to number.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorKind;
/// # pub fn main() {
/// // Create our error.
/// let err = |u| From::from(ErrorKind::InvalidDigit(u));
///
/// // String overloads
/// assert_eq!(lexical::try_parse_lossy::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>("1."), Err(err(1)));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse_lossy::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>(b"1."), Err(err(1)));
/// # }
/// ```
///
/// [`try_parse`]: fn.try_parse.html
#[inline(always)]
pub fn try_parse_lossy<N: FromBytesLossy, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<N, Error>
{
    N::try_from_bytes_lossy(bytes.as_ref(), 10)
}

/// High-level conversion of bytes to a number with a custom radix.
///
/// This function only returns a value if the entire string is
/// successfully parsed. For an unchecked version of this function,
/// use [`parse_radix`].
///
/// * `bytes`   - Byte slice to convert to number.
/// * `radix`   - Number of unique digits for the number (base).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorKind;
/// # pub fn main() {
/// // Create our error wrapper.
/// let err = |u| From::from(ErrorKind::InvalidDigit(u));
///
/// // String overloads
/// assert_eq!(lexical::try_parse_radix::<i32, _>("5", 10), Ok(5));
/// assert_eq!(lexical::try_parse_radix::<i32, _>("1a", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<i32, _>("1.", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.0.", 10), Err(err(3)));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse_radix::<i32, _>(b"5", 10), Ok(5));
/// assert_eq!(lexical::try_parse_radix::<i32, _>(b"1a", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.0.", 10), Err(err(3)));
/// # }
/// ```
///
/// [`parse_radix`]: fn.parse_radix.html
#[inline(always)]
pub fn try_parse_radix<N: FromBytes, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<N, Error>
{
    assert!(2 <= radix && radix <= 36, "try_parse_radix, radix must be in range `[2, 36]`, got {}", radix);
    N::try_from_bytes(bytes.as_ref(), radix)
}

/// High-level lossy conversion of bytes to a float with a custom radix.
///
/// This function uses aggressive optimizations to avoid worst-case
/// scenarios, and can return inaccurate results. For guaranteed accurate
/// floats, use [`try_parse_radix`].
///
/// * `bytes`   - Byte slice to convert to number.
/// * `radix`   - Number of unique digits for the number (base).
///
/// # Examples
///
/// ```rust
/// # extern crate lexical;
/// # use lexical::ErrorKind;
/// # pub fn main() {
/// // Create our error wrapper.
/// let err = |u| From::from(ErrorKind::InvalidDigit(u));
///
/// // String overloads
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("1.", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("1.0.", 10), Err(err(3)));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"1.", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"1.0.", 10), Err(err(3)));
/// # }
/// ```
/// [`try_parse_radix`]: fn.try_parse_radix.html
#[inline(always)]
pub fn try_parse_lossy_radix<N: FromBytesLossy, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<N, Error>
{
    assert!(2 <= radix && radix <= 36, "try_parse_radix, radix must be in range `[2, 36]`, got {}", radix);
    N::try_from_bytes_lossy(bytes.as_ref(), radix)
}
