//! Fast lexical conversion routines with a C FFI for a no_std environment.
//!
//! # Getting Started
//!
//! lexical-core is a low-level, partially FFI-compatible API for
//! number-to-string and string-to-number conversions, without requiring
//! a system allocator. If you would like to use a convenient, high-level
//! API, please look at [lexical](https://crates.io/crates/lexical) instead.
//!
//! ```rust
//! extern crate lexical_core;
//!
//! // String to number using slices
//! // The first argument is the radix, which should be 10 for decimal strings,
//! // and the second argument is the byte string parsed.
//! let f = lexical_core::atof::atof64_slice(10, b"3.5");   // 3.5
//! let i = lexical_core::atoi::atoi32_slice(10, b"15");          // 15
//!
//! // String to number using ranges, for FFI-compatible code.
//! // The first argument is the radix, which should be 10 for decimal strings,
//! // the second argument is a pointer to the start of the parsed byte array,
//! // and the third argument is a pointer to 1-past-the-end. It will process
//! // bytes in the range [first, last).
//! unsafe {
//!     let bytes = b"3.5";
//!     let first = bytes.as_ptr();
//!     let last = first.add(bytes.len());
//!     let f = lexical_core::atof::atof64_range(10, first, last);
//! }
//!
//! // The ato*_slice and ato*_range parsers are not checked, they do not
//! // validate that the input data is entirely correct, and discard trailing
//! // bytes that are found. The explicit behavior is to wrap on overflow, and
//! // to discard invalid digits.
//! let i = lexical_core::atoi::atoi8_slice(10, b"256");    // 0, wraps from 256
//! let i = lexical_core::atoi::atoi8_slice(10, b"1a5");    // 1, discards "a5"
//!
//! // You should prefer the checked parsers, whenever possible. These detect
//! // numeric overflow, and no invalid trailing digits are present.
//! // The error code for success is 0, all errors are less than 0.
//!
//! // Ideally, everything works great.
//! let res = lexical_core::atoi::try_atoi8_slice(10, b"15");
//! assert_eq!(res.error.code, lexical_core::ErrorCode::Success);
//! assert_eq!(res.value, 15);
//!
//! // However, it detects numeric overflow, setting `res.error.code`
//! // to the appropriate value.
//! let res = lexical_core::atoi::try_atoi8_slice(10, b"256");
//! assert_eq!(res.error.code, lexical_core::ErrorCode::Overflow);
//!
//! // Errors occurring prematurely terminating the parser due to invalid
//! // digits return the index in the buffer where the invalid digit was
//! // seen. This may useful in contexts like serde, which require numerical
//! // parsers from complex data without having to extract a substring
//! // containing only numeric data ahead of time. If the error is set
//! // to a `InvalidDigit`, the value is guaranteed to be accurate up until
//! // that point. For example, if the trailing data is whitespace,
//! // the value from an invalid digit may be perfectly valid in some contexts.
//! let res = lexical_core::atoi::try_atoi8_slice(10, b"15 45");
//! assert_eq!(res.error.code, lexical_core::ErrorCode::InvalidDigit);
//! assert_eq!(res.error.index, 2);
//! assert_eq!(res.value, 15);
//!
//! // Number to string using slices.
//! // The first argument is the value, the second argument is the radix,
//! // and the third argument is the buffer to write to.
//! // The function returns a subslice of the original buffer, and will
//! // always start at the same position (`buf.as_ptr() == slc.as_ptr()`).
//! let mut buf = [b'0'; lexical_core::MAX_I64_SIZE];
//! let slc = lexical_core::itoa::i64toa_slice(15, 10, &mut buf);
//! assert_eq!(slc, b"15");
//!
//! // If an insufficiently long buffer is passed, the serializer will panic.
//! let mut buf = [b'0'; 1];
//! // PANICS
//! //let slc = lexical_core::itoa::i64toa_slice(15, 10, &mut buf);
//!
//! // In order to guarantee the buffer is long enough, always ensure there
//! // are at least `MAX_XX_SIZE`, where XX is the type name in upperase,
//! // IE, for `isize`, `MAX_ISIZE_SIZE`.
//! let mut buf = [b'0'; lexical_core::MAX_F64_SIZE];
//! let slc = lexical_core::ftoa::f64toa_slice(15.1, 10, &mut buf);
//! assert_eq!(slc, b"15.1");
//! ```

// FEATURES

// Require intrinsics in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(not(feature = "std"), feature = "correct", feature = "radix"), feature(alloc))]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]

// DEPENDENCIES

#[macro_use]
extern crate cfg_if;

#[cfg(feature = "correct")]
#[macro_use]
extern crate static_assertions;

// Testing assertions for floating-point equality.
#[cfg(test)]
#[macro_use]
extern crate approx;

// Test against randomly-generated data.
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

// Test against randomly-generated guided data.
#[cfg(test)]
#[macro_use]
extern crate proptest;

// Use vec if there is a system allocator, which we require only if
// we're using the correct and radix features.
#[cfg(all(not(feature = "std"), feature = "correct", feature = "radix"))]
#[cfg_attr(test, macro_use)]
extern crate alloc;

// Use stackvector for atof.
#[cfg(feature = "correct")]
#[macro_use]
extern crate stackvector;

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
#[cfg(feature = "std")]
pub(crate) use std::*;

#[cfg(not(feature = "std"))]
pub(crate) use core::*;

cfg_if! {
if #[cfg(all(feature = "correct", feature = "radix"))] {
    #[cfg(feature = "std")]
    pub(crate) use std::vec::Vec;

    #[cfg(not(feature = "std"))]
    pub(crate) use alloc::vec::Vec;
}}  // cfg_if

}   // lib

// API

// Hide implementation details
#[macro_use]
mod util;

mod atof;
mod atoi;
mod float;
mod ftoa;
mod itoa;

// Publicly re-export the low-level string-to-float functions.
pub use atof::{atof32_range, atof32_slice, atof64_range, atof64_slice};
pub use atof::{try_atof32_range, try_atof32_slice, try_atof64_range, try_atof64_slice};
pub use atof::{atof32_lossy_range, atof32_lossy_slice, atof64_lossy_range, atof64_lossy_slice};
pub use atof::{try_atof32_lossy_range, try_atof32_lossy_slice, try_atof64_lossy_range, try_atof64_lossy_slice};

// Publicly re-export the low-level string-to-integer functions.
pub use atoi::{atoi8_range, atoi8_slice, atou8_range, atou8_slice};
pub use atoi::{try_atoi8_range, try_atoi8_slice, try_atou8_range, try_atou8_slice};
pub use atoi::{atoi16_range, atoi16_slice, atou16_range, atou16_slice};
pub use atoi::{try_atoi16_range, try_atoi16_slice, try_atou16_range, try_atou16_slice};
pub use atoi::{atoi32_range, atoi32_slice, atou32_range, atou32_slice};
pub use atoi::{try_atoi32_range, try_atoi32_slice, try_atou32_range, try_atou32_slice};
pub use atoi::{atoi64_range, atoi64_slice, atou64_range, atou64_slice};
pub use atoi::{try_atoi64_range, try_atoi64_slice, try_atou64_range, try_atou64_slice};
pub use atoi::{atoi128_range, atoi128_slice, atou128_range, atou128_slice};
pub use atoi::{try_atoi128_range, try_atoi128_slice, try_atou128_range, try_atou128_slice};
pub use atoi::{atoisize_range, atoisize_slice, atousize_range, atousize_slice};
pub use atoi::{try_atoisize_range, try_atoisize_slice, try_atousize_range, try_atousize_slice};

// Publicly re-export the low-level float-to-string functions.
pub use ftoa::{f32toa_range, f32toa_slice, f64toa_range, f64toa_slice};

// Publicly re-export the low-level integer-to-string functions.
pub use itoa::{i8toa_range, i8toa_slice, u8toa_range, u8toa_slice};
pub use itoa::{i16toa_range, i16toa_slice, u16toa_range, u16toa_slice};
pub use itoa::{i32toa_range, i32toa_slice, u32toa_range, u32toa_slice};
pub use itoa::{i64toa_range, i64toa_slice, u64toa_range, u64toa_slice};
pub use itoa::{i128toa_range, i128toa_slice, u128toa_range, u128toa_slice};
pub use itoa::{isizetoa_range, isizetoa_slice, usizetoa_range, usizetoa_slice};

// Re-export EXPONENT_DEFAULT_CHAR and EXPONENT_BACKUP_CHAR globally.
pub use util::EXPONENT_DEFAULT_CHAR;

#[cfg(feature = "radix")]
pub use util::EXPONENT_BACKUP_CHAR;

// Re-export NAN_STRING, INF_STRING and INFINITY_STRING globally.
pub use util::{INF_STRING, INFINITY_STRING, NAN_STRING};

//Re-export the getters and setters for FFI code for the strings.
pub use util::{get_inf_string, get_infinity_string, get_nan_string};
pub use util::{set_inf_string, set_infinity_string, set_nan_string};

// Re-export the error structs and enumerations.
pub use util::{Error, ErrorCode, is_empty, is_invalid_digit, is_overflow, is_success};

// Re-export the required buffer sizes for the low-level API.
pub use util::BUFFER_SIZE;
pub use util::{MAX_I8_SIZE, MAX_I16_SIZE, MAX_I32_SIZE, MAX_I64_SIZE, MAX_I128_SIZE, MAX_ISIZE_SIZE};
pub use util::{MAX_U8_SIZE, MAX_U16_SIZE, MAX_U32_SIZE, MAX_U64_SIZE, MAX_U128_SIZE, MAX_USIZE_SIZE};
pub use util::{MAX_F32_SIZE, MAX_F64_SIZE};

// Re-export the required FFI-compatible buffer sizes for the low-level API.
pub use util::BUFFER_SIZE_FFI;
pub use util::{MAX_I8_SIZE_FFI, MAX_I16_SIZE_FFI, MAX_I32_SIZE_FFI, MAX_I64_SIZE_FFI, MAX_I128_SIZE_FFI, MAX_ISIZE_SIZE_FFI};
pub use util::{MAX_U8_SIZE_FFI, MAX_U16_SIZE_FFI, MAX_U32_SIZE_FFI, MAX_U64_SIZE_FFI, MAX_U128_SIZE_FFI, MAX_USIZE_SIZE_FFI};
pub use util::{MAX_F32_SIZE_FFI, MAX_F64_SIZE_FFI};

// Re-export the float rounding scheme used.
#[cfg(feature = "correct")]
pub use util::{FLOAT_ROUNDING, RoundingKind};

// Re-export the result struct and expanded-template variants (for FFI).
pub use util::Result;
pub use util::{U8Result, U16Result, U32Result, U64Result, U128Result, UsizeResult};
pub use util::{I8Result, I16Result, I32Result, I64Result, I128Result, IsizeResult};
pub use util::{F32Result, F64Result};
