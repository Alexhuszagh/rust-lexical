//! Fast lexical integer-to-string conversion routines.

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

/// Facade around the core features for name mangling.
pub(crate) mod lib {
    #[cfg(feature = "std")]
    pub(crate) use std::*;

    #[cfg(not(feature = "std"))]
    pub(crate) use core::*;
}

pub mod algorithm;
pub mod decimal;
pub mod generic;
pub mod options;
pub mod table;

mod api;
mod table_binary;
mod table_decimal;
mod table_radix;

// Re-exports
pub use self::api::{ToLexical, ToLexicalWithOptions};
pub use self::options::Options;
