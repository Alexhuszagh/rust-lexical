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
    }
}

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
    }
}

/// Facade around the core features for name mangling.
pub(crate) mod sealed {
    cfg_if! {
        if #[cfg(feature = "std")] {
            pub use std::*;
        } else {
            pub use core::*;
        }
    }
}

// API

// Hide the implementation details.
#[macro_use]
mod util;

// Publicly export the low-level APIs.
// Macros used in atoi are required for atof, so export those.
#[macro_use]
#[doc(hidden)]
pub mod table;

#[macro_use]
pub mod atoi;

#[macro_use]
pub mod itoa;

pub mod atof;
pub mod ftoa;

#[doc(hidden)]
pub mod traits;

// Re-export EXPONENT_DEFAULT_CHAR and EXPONENT_BACKUP_CHAR globally.
pub use util::{EXPONENT_DEFAULT_CHAR, EXPONENT_BACKUP_CHAR};

// Re-export NAN_STRING and INFINITY_STRING globally.
pub use util::{INFINITY_STRING, NAN_STRING};

// HIGH LEVEL

use sealed::convert::AsRef;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::string::String;

use traits::Aton;

#[cfg(any(feature = "std", feature = "alloc"))]
use traits::Ntoa;

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
pub fn to_string<N: Ntoa>(n: N) -> String {
    n.serialize_to_string(10)
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
pub fn to_string_radix<N: Ntoa>(n: N, radix: u8) -> String {
    assert!(2 <= radix && radix <= 36, "to_string_radix, radix must be in range `[2, 36]`, got {}", radix);
    n.serialize_to_string(radix)
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
pub fn parse<N: Aton, Bytes: AsRef<[u8]>>(bytes: Bytes) -> N {
    N::deserialize_from_bytes(bytes.as_ref(), 10)
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
pub fn parse_radix<N: Aton, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8) -> N {
    assert!(2 <= radix && radix <= 36, "parse_radix, radix must be in range `[2, 36]`, got {}", radix);
    N::deserialize_from_bytes(bytes.as_ref(), radix)
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
/// # pub fn main() {
/// // String overloads
/// assert_eq!(lexical::try_parse::<i32, _>("5"), Ok(5));
/// assert_eq!(lexical::try_parse::<i32, _>("1a"), Err(1));
/// assert_eq!(lexical::try_parse::<f32, _>("0"), Ok(0.0));
/// assert_eq!(lexical::try_parse::<f32, _>("1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse::<f32, _>("1."), Err(1));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse::<i32, _>(b"5"), Ok(5));
/// assert_eq!(lexical::try_parse::<i32, _>(b"1a"), Err(1));
/// assert_eq!(lexical::try_parse::<f32, _>(b"0"), Ok(0.0));
/// assert_eq!(lexical::try_parse::<f32, _>(b"1.0"), Ok(1.0));
/// assert_eq!(lexical::try_parse::<f32, _>(b"1."), Err(1));
/// # }
/// ```
///
/// [`parse`]: fn.parse.html
#[inline(always)]
pub fn try_parse<N: Aton, Bytes: AsRef<[u8]>>(bytes: Bytes)
    -> Result<N, usize>
{
    N::try_deserialize_from_bytes(bytes.as_ref(), 10)
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
/// # pub fn main() {
/// // String overloads
/// assert_eq!(lexical::try_parse_radix::<i32, _>("5", 10), Ok(5));
/// assert_eq!(lexical::try_parse_radix::<i32, _>("1a", 10), Err(1));
/// assert_eq!(lexical::try_parse_radix::<i32, _>("1.", 10), Err(1));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.", 10), Err(1));
/// assert_eq!(lexical::try_parse_radix::<f32, _>("1.0.", 10), Err(3));
///
/// // Bytes overloads
/// assert_eq!(lexical::try_parse_radix::<i32, _>(b"5", 10), Ok(5));
/// assert_eq!(lexical::try_parse_radix::<i32, _>(b"1a", 10), Err(1));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"0", 10), Ok(0.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.0", 10), Ok(1.0));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.", 10), Err(1));
/// assert_eq!(lexical::try_parse_radix::<f32, _>(b"1.0.", 10), Err(3));
/// # }
/// ```
///
/// [`parse_radix`]: fn.parse_radix.html
#[inline(always)]
pub fn try_parse_radix<N: Aton, Bytes: AsRef<[u8]>>(bytes: Bytes, radix: u8)
    -> Result<N, usize>
{
    assert!(2 <= radix && radix <= 36, "try_parse_radix, radix must be in range `[2, 36]`, got {}", radix);
    N::try_deserialize_from_bytes(bytes.as_ref(), radix)
}
