//! Fast lexical conversion routines.
//!
//! Fast lexical conversion routines with a C FFI for a no_std environment.

// FEATURES

// Require intrinsics in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(not(feature = "std"), feature = "algorithm_m", feature = "radix"), feature(alloc))]
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

// Use vec if there is a system allocator, which we require only if
// we're using the correct and radix features.
#[cfg(all(not(feature = "std"), feature = "algorithm_m", feature = "radix"))]
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
if #[cfg(all(feature = "algorithm_m", feature = "radix"))] {
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

mod float;

// Publicly export the low-level APIs.
pub mod atof;
pub mod atoi;
pub mod ftoa;
pub mod itoa;

// Re-export EXPONENT_DEFAULT_CHAR and EXPONENT_BACKUP_CHAR globally.
pub use util::{EXPONENT_DEFAULT_CHAR, EXPONENT_BACKUP_CHAR};

// Re-export NAN_STRING, INF_STRING and INFINITY_STRING globally.
pub use util::{INF_STRING, INFINITY_STRING, NAN_STRING};

// Re-export the error structs and enumerations.
pub use util::{Error, ErrorCode, is_success};

// Re-export the required buffer size for the low-level API.
pub use util::BUFFER_SIZE;

// Re-export the result struct and expanded-template variants (for FFI).
pub use util::Result;
pub use util::{U8Result, U16Result, U32Result, U64Result, U128Result, UsizeResult};
pub use util::{I8Result, I16Result, I32Result, I64Result, I128Result, IsizeResult};
pub use util::{F32Result, F64Result};
