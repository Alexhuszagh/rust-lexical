//! Fast lexical conversion routines with a C FFI for a no_std environment.
//!
//! # Getting Started
//!
//! lexical-core is a low-level, partially FFI-compatible API for
//! number-to-string and string-to-number conversions, without requiring
//! a system allocator. If you would like to use a convenient, high-level
//! API, please look at [lexical](https://crates.io/crates/lexical) instead.
//!
//! # Getting Started
//!
//! ```rust
//! extern crate lexical_core;
//!
//! // String to number using slices
//! // The argument is the byte string parsed.
//! let f = lexical_core::atof64_slice(b"3.5").unwrap();   // 3.5
//! let i = lexical_core::atoi32_slice(b"15").unwrap();    // 15
//!
//! // String to number using pointer ranges, for FFI-compatible code.
//! // The first argument is a pointer to the start of the parsed byte array,
//! // and the second argument is a pointer to 1-past-the-end. It will process
//! // bytes in the range [first, last).
//! unsafe {
//!     let bytes = b"3.5";
//!     let first = bytes.as_ptr();
//!     let last = first.add(bytes.len());
//!     let f = lexical_core::atof64_range(first, last).unwrap();
//! }
//!
//! // If and only if the `radix` feature is enabled, you may use the radix
//! // overloads to parse non-decimal floats and strings.
//! ##[cfg(feature = "radix")]
//! let f = lexical_core::atof32_radix_slice(2, b"11.1");   // 3.5
//! ##[cfg(feature = "radix")]
//! let i = lexical_core::atoi32_radix_slice(2, b"1111");   // 15
//!
//! // The ato*_slice and ato*_range parsers are not checked, they do not
//! // validate that the input data is entirely correct, and discard trailing
//! // bytes that are found. The explicit behavior is to wrap on overflow, and
//! // to discard invalid digits.
//! let i = lexical_core::atoi8_slice(b"256");    // 0, wraps from 256
//! let i = lexical_core::atoi8_slice(b"1a5");    // 1, discards "a5"
//!
//! // You should prefer the checked parsers, whenever possible. These detect
//! // numeric overflow, and no invalid trailing digits are present.
//! // The error code for success is 0, all errors are less than 0.
//!
//! // Ideally, everything works great.
//! let res = lexical_core::atoi8_slice(b"15");
//! assert!(res.is_ok());
//! assert_eq!(res.unwrap(), 15);
//!
//! // However, it detects numeric overflow, setting `res.error.code`
//! // to the appropriate value.
//! let res = lexical_core::atoi8_slice(b"256");
//! assert!(res.is_err());
//! assert_eq!(res.err().unwrap().code, lexical_core::ErrorCode::Overflow);
//!
//! // Errors occurring prematurely terminating the parser due to invalid
//! // digits return the index in the buffer where the invalid digit was
//! // seen.
//! let res = lexical_core::atoi8_slice(b"15 45");
//! assert!(res.is_err());
//! let error = res.err().unwrap();
//! assert_eq!(error.code, lexical_core::ErrorCode::InvalidDigit);
//! assert_eq!(error.index, 2);
//!
//! // Number to string using slices.
//! // The first argument is the value, the second argument is the radix,
//! // and the third argument is the buffer to write to.
//! // The function returns a subslice of the original buffer, and will
//! // always start at the same position (`buf.as_ptr() == slc.as_ptr()`).
//! let mut buf = [b'0'; lexical_core::MAX_I64_SIZE];
//! let slc = lexical_core::i64toa_slice(15, &mut buf);
//! assert_eq!(slc, b"15");
//!
//! // If an insufficiently long buffer is passed, the serializer will panic.
//! // PANICS
//! let mut buf = [b'0'; 1];
//! //let slc = lexical_core::i64toa_slice(15, &mut buf);
//!
//! // In order to guarantee the buffer is long enough, always ensure there
//! // are at least `MAX_XX_SIZE`, where XX is the type name in upperase,
//! // IE, for `isize`, `MAX_ISIZE_SIZE`.
//! let mut buf = [b'0'; lexical_core::MAX_F64_SIZE];
//! let slc = lexical_core::f64toa_slice(15.1, &mut buf);
//! assert_eq!(slc, b"15.1");
//! ```

// FEATURES

// Require intrinsics in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(not(feature = "std"), feature = "correct", feature = "radix"), feature(alloc))]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]
#![cfg_attr(all(not(test), not(feature = "std")), feature(lang_items))]

// DEPENDENCIES

#[macro_use]
extern crate cfg_if;

#[cfg(feature = "correct")]
#[allow(unused_imports)]    // Not used before 1.26.
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

// PANIC

// Need to define a panic handler when we're not testing (panic handler
// then becomes "unwind" but there is no_std). This causes us to fail
// with doctests, so ensure `--tests` is passed to `cargo test` whenever
// we are in a  `no_std` context.
cfg_if! {
if #[cfg(all(not(test), not(feature = "std")))] {
    use lib::intrinsics;
    use lib::panic::PanicInfo;

    #[panic_handler]
    fn panic(_: &PanicInfo) -> ! {
        unsafe {
            intrinsics::abort();
        }
    }

    #[lang = "eh_personality"]
    extern fn eh_personality() {}
}}  // cfg_if

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
pub use atof::*;

// Publicly re-export the low-level string-to-integer functions.
pub use atoi::*;

// Publicly re-export the low-level float-to-string functions.
pub use ftoa::*;

// Publicly re-export the low-level integer-to-string functions.
pub use itoa::*;

// Re-export configuration and utilities globally.
pub use util::*;
