//! Shared utilities for lexical conversion routines.

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
pub mod assert;
pub mod constants;
pub mod digit;
pub mod div128;
pub mod error;
pub mod format;
pub mod iterator;
pub mod mul;
pub mod noskip;
pub mod num;
pub mod result;
pub mod skip;
pub mod step;

mod api;
mod feature_format;
mod format_builder;
mod format_flags;
mod not_feature_format;
