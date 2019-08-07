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
//! // String to number using Rust slices.
//! // The argument is the byte string parsed.
//! let f = lexical_core::atof64(b"3.5").unwrap();   // 3.5
//! let i = lexical_core::atoi32(b"15").unwrap();    // 15
//!
//! // String to number using pointer ranges, for FFI-compatible code.
//! // The first argument is a pointer to the start of the parsed byte array,
//! // and the second argument is a pointer to 1-past-the-end. It will process
//! // bytes in the range [first, last).
//! unsafe {
//!     // Get an FFI-compatible range.
//!     let bytes = b"3.5";
//!     let first = bytes.as_ptr();
//!     let last = first.add(bytes.len());
//!     // Get our result and extract our value using C-compatible functions.
//!     let res = lexical_core::ffi::atof64(first, last);
//!     let f = lexical_core::ffi::f64_result_ok(res); // Aborts if res is not ok.
//! }
//!
//! // The ato* and ffi::ato* parsers are checked, they validate the
//! // input data is entirely correct, and stop parsing when invalid data
//! // is found, or upon numerical overflow.
//! let r = lexical_core::atoi8(b"256"); // Err(ErrorCode::Overflow.into())
//! let r = lexical_core::atoi8(b"1a5"); // Err(ErrorCode::InvalidDigit.into())
//!
//! // In order to extract and parse a number from a substring of the input
//! // data, use the ato*_partial and ffi::ato*_partial parsers.
//! // These functions return the parsed value and the number of processed
//! // digits, allowing you to extract and parse the number in a single pass.
//! let r = lexical_core::atoi8(b"3a5"); // Ok((3, 1))
//!
//! // Lexical-core includes FFI functions to properly extract data and handle
//! // errors during routines. All the following functions may be used in
//! // external libraries, include from C.
//!
//! unsafe {
//!     unsafe fn to_range(bytes: &'static [u8]) -> (*const u8, *const u8) {
//!         let first = bytes.as_ptr();
//!         let last = first.add(bytes.len());
//!         (first, last)
//!     }
//!
//!     // Ideally, everything works great.
//!     let (first, last) = to_range(b"15");
//!     let res = lexical_core::ffi::atoi8(first, last);
//!     if lexical_core::ffi::i8_result_is_ok(res) {
//!         let i = lexical_core::ffi::i8_result_ok(res);
//!         assert_eq!(i, 15);
//!     }
//!
//!     // However, it detects numeric overflow, returning an error with
//!     // an error code equal to `ErrorCode::Overflow`.
//!     let (first, last) = to_range(b"256");
//!     let res = lexical_core::ffi::atoi8(first, last);
//!     if lexical_core::ffi::i8_result_is_err(res) {
//!         let err = lexical_core::ffi::i8_result_err(res);
//!         assert_eq!(err.code, lexical_core::ffi::ErrorCode::Overflow);
//!     }
//!
//!     // Errors occurring prematurely terminating the parser due to invalid
//!     // digits return the index in the buffer where the invalid digit was
//!     // seen. This may useful in contexts like serde, which require numerical
//!     // parsers from complex data without having to extract a substring
//!     // containing only numeric data ahead of time.
//!     let (first, last) = to_range(b"15 45");
//!     let res = lexical_core::ffi::atoi8(first, last);
//!     if lexical_core::ffi::i8_result_is_err(res) {
//!         let err = lexical_core::ffi::i8_result_err(res);
//!         assert_eq!(err.code, lexical_core::ffi::ErrorCode::InvalidDigit);
//!         assert_eq!(err.index, 2);
//!     }
//!
//!     // Number to string using slices.
//!     // The first argument is the value, the second argument is the radix,
//!     // and the third argument is the buffer to write to.
//!     // The function returns a subslice of the original buffer, and will
//!     // always start at the same position (`buf.as_ptr() == slc.as_ptr()`).
//!     let mut buf = [b'0'; lexical_core::MAX_I64_SIZE];
//!     let slc = lexical_core::i64toa(15, &mut buf);
//!     assert_eq!(slc, b"15");
//! }
//!
//! // If an insufficiently long buffer is passed, the serializer will panic.
//! // PANICS
//! let mut buf = [b'0'; 1];
//! //let slc = lexical_core::i64toa(15, &mut buf);
//!
//! // In order to guarantee the buffer is long enough, always ensure there
//! // are at least `MAX_*_SIZE`, where * is the type name in upperase,
//! // IE, for `isize`, `MAX_ISIZE_SIZE`.
//! let mut buf = [b'0'; lexical_core::MAX_F64_SIZE];
//! let slc = lexical_core::f64toa(15.1, &mut buf);
//! assert_eq!(slc, b"15.1");
//!
//! // When the `radix` feature is enabled, for base10 floats, using `MAX_*_SIZE`
//! // may significantly overestimate the space required to format the number.
//! // Therefore, the `MAX_*_SIZE_BASE10` constants allow you to get a much
//! // tighter bound on the space required.
//! let mut buf = [b'0'; lexical_core::MAX_F64_SIZE_BASE10];
//! let slc = lexical_core::f64toa(15.1, &mut buf);
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
#[cfg(all(test, feature = "std"))]
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

// Publicly expose the FFI module for documentation purposes.
pub mod ffi;

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
