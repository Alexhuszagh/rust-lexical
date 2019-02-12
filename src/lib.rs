//! Fast lexical conversion routines.
//!
//! Fast lexical conversion routines for both std and no_std environments.
//! Lexical provides routines to convert numbers to and from decimal
//! strings. Lexical also supports non-base 10 numbers, with the `radix`
//! feature, for both integers and floats. Lexical is simple to use,
//! and exports up to 10 functions in the high-level API.
//!
//! Lexical makes heavy use of unsafe code for performance, and therefore
//! may introduce memory-safety issues. Although the code is tested with
//! wide variety of inputs to minimize the risk of memory-safety bugs,
//! no guarantees are made and you should use it at your own risk.
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
//! let i: i32 = lexical::parse("3");            // 3, auto-type deduction.
//! let f: f32 = lexical::parse("3.5");          // 3.5
//! let d = lexical::parse::<f64, _>("3.5");     // 3.5, explicit type hints.
//! let d = lexical::try_parse::<f64, _>("3.5"); // Ok(3.5), error checking parse.
//! let d = lexical::try_parse::<f64, _>("3a");  // Err(Error(_)), failed to parse.
//! ```

// FEATURES

// Require intrinsics and alloc in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

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
}}  // cfg_if

cfg_if! {
if #[cfg(feature = "std")] {
    pub(crate) use std::string::String;
    pub(crate) use std::vec::Vec;
} else {
    pub(crate) use alloc::string::String;
    pub(crate) use alloc::vec::Vec;
}
}}  // cfg_if

// API

// Hide the implementation details.
mod error;
mod traits;

// Re-export EXPONENT_DEFAULT_CHAR and EXPONENT_BACKUP_CHAR globally.
pub use lexical_core::EXPONENT_DEFAULT_CHAR;

#[cfg(feature = "radix")]
pub use lexical_core::EXPONENT_BACKUP_CHAR;

// Re-export NaN, short INF, and long INFINITY string getters and setters.
pub use lexical_core::{get_inf_string, get_infinity_string, get_nan_string};
pub use lexical_core::{set_inf_string, set_infinity_string, set_nan_string};

// Re-export the float rounding scheme used.
#[cfg(all(feature = "correct", feature = "rounding"))]
pub use lexical_core::{FLOAT_ROUNDING, RoundingKind};

// Re-export the Error and ErrorKind globally.
pub use error::{Error, ErrorKind};

// Publicly expose traits so they may be used for generic programming.
pub use traits::{FromBytes, FromBytesLossy, ToBytes};

// HIGH LEVEL

use lib::convert::AsRef;

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
/// ##[cfg(not(feature = "trim_floats"))]
/// assert_eq!(lexical::to_string(0.0), "0.0");
/// # }
/// ```
#[inline]
pub fn to_string<N: ToBytes>(n: N) -> lib::String {
    unsafe {
        lib::String::from_utf8_unchecked(n.to_bytes())
    }
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
/// ##[cfg(not(feature = "trim_floats"))]
/// assert_eq!(lexical::to_string_radix(0.0, 10), "0.0");
/// # }
/// ```
///
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
#[cfg(feature = "radix")]
#[inline]
pub fn to_string_radix<N: ToBytes>(n: N, radix: u8) -> lib::String {
    unsafe {
        lib::String::from_utf8_unchecked(n.to_bytes_radix(radix))
    }
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
#[inline]
pub fn parse<N: FromBytes, Bytes: AsRef<[u8]>>(bytes: Bytes) -> N {
    N::from_bytes(bytes.as_ref())
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
#[inline]
pub fn parse_lossy<N: FromBytesLossy, Bytes: AsRef<[u8]>>(bytes: Bytes) -> N {
    N::from_bytes_lossy(bytes.as_ref())
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
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
///
/// [`try_parse_radix`]: fn.try_parse_radix.html
#[cfg(feature = "radix")]
#[inline]
pub fn parse_radix<N: FromBytes, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8) -> N {
    N::from_bytes_radix(bytes.as_ref(), radix)
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
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
///
/// [`parse_radix`]: fn.parse_radix.html
/// [`try_parse_radix`]: fn.try_parse_radix.html
#[cfg(feature = "radix")]
#[inline]
pub fn parse_lossy_radix<N: FromBytesLossy, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8) -> N {
    N::from_bytes_lossy_radix(bytes.as_ref(), radix)
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
/// let err = |u| ErrorKind::InvalidDigit(u).into();
///
/// // String overloads
/// assert_eq!(lexical::try_parse::<i32, _>("5"), Ok(5));
/// assert_eq!(lexical::try_parse::<i32, _>("1a"), Err(err(1)));
/// assert_eq!(lexical::try_parse::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::try_parse::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse::<f32, _>("1."), Ok(1.0));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse::<i32, _>(b"5"), Ok(5));
/// assert_eq!(lexical::try_parse::<i32, _>(b"1a"), Err(err(1)));
/// assert_eq!(lexical::try_parse::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::try_parse::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse::<f32, _>(b"1."), Ok(1.0));
/// # assert_eq!(lexical::try_parse::<f32, _>(b"5.002868148396374"), Ok(5.002868148396374));
/// # assert_eq!(lexical::try_parse::<f64, _>(b"5.002868148396374"), Ok(5.002868148396374));
/// # }
/// ```
///
/// [`parse`]: fn.parse.html
#[inline]
pub fn try_parse<N: FromBytes, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<N, Error>
{
    N::try_from_bytes(bytes.as_ref())
}

/// High-level lossy conversion of decimal-encoded bytes to a number.
///
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
/// let err = |u| ErrorKind::InvalidDigit(u).into();
///
/// // String overloads
/// assert_eq!(lexical::try_parse_lossy::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>("1."), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>("1a"), Err(err(1)));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse_lossy::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>(b"1."), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy::<f32, _>(b"1a"), Err(err(1)));
/// # }
/// ```
///
/// [`try_parse`]: fn.try_parse.html
#[inline]
pub fn try_parse_lossy<N: FromBytesLossy, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<N, Error>
{
    N::try_from_bytes_lossy(bytes.as_ref())
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
/// let err = |u| ErrorKind::InvalidDigit(u).into();
///
/// // String overloads
/// assert_eq!(lexical::try_parse_radix::<i32, _>("5", 10), Ok(5));
/// assert_eq!(lexical::try_parse_radix::<i32, _>("1a", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<i32, _>("1.", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1a", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.0.", 10), Err(err(3)));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse_radix::<i32, _>(b"5", 10), Ok(5));
/// assert_eq!(lexical::try_parse_radix::<i32, _>(b"1a", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1a", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.0.", 10), Err(err(3)));
/// # }
/// ```
///
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
///
/// [`parse_radix`]: fn.parse_radix.html
#[cfg(feature = "radix")]
#[inline]
pub fn try_parse_radix<N: FromBytes, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<N, Error>
{
    N::try_from_bytes_radix(bytes.as_ref(), radix)
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
/// let err = |u| ErrorKind::InvalidDigit(u).into();
///
/// // String overloads
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("1.", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("1a", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>("1.0.", 10), Err(err(3)));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"1.", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"1a", 10), Err(err(1)));
/// assert_eq!(lexical::try_parse_lossy_radix::<f32, _>(b"1.0.", 10), Err(err(3)));
/// # }
/// ```
///
/// # Panics
///
/// Panics if radix is not in the range `[2, 36]`
///
/// [`try_parse_radix`]: fn.try_parse_radix.html
#[cfg(feature = "radix")]
#[inline]
pub fn try_parse_lossy_radix<N: FromBytesLossy, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<N, Error>
{
    N::try_from_bytes_lossy_radix(bytes.as_ref(), radix)
}
