//! Shared utilities for lexical conversion routines.
//!
//! These are not meant to be used publicly for any numeric
//! conversion routines, but provide optimized math routines,
//! format packed struct definitions, and custom iterators
//! for all workspaces.
//!
//! # Features
//!
//! * `std` - Use the standard library.
//! * `power-of-two` - Add support for parsing power-of-two integer strings.
//! * `radix` - Add support for strings of any radix.
//! * `write-integers` - Add support for writing integers.
//! * `write-floats` - Add support for writing floats.
//! * `parse-integers` - Add support for parsing integers.
//! * `parse-floats` - Add support for parsing floats.
//! * `compact` - Reduce code size at the cost of performance.
//!
//! # Note
//!
//! None of this is considered a public API: any of the implementation
//! details may change release-to-release without major or minor version
//! changes. Use internal implementation details at your own risk.
//!
//! lexical-util mainly exists as an implementation detail for
//! lexical-core, although its API is stable. If you would like to use
//! a high-level API that writes to and parses from `String` and `&str`,
//! respectively, please look at [lexical](https://crates.io/crates/lexical)
//! instead. If you would like an API that supports multiple numeric
//! conversions, please look at [lexical-core](https://crates.io/crates/lexical-core)
//! instead.
//!
//! # Version Support
//!
//! The minimum, standard, required version is 1.51.0, for const generic
//! support. Older versions of lexical support older Rust versions.

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod algorithm;
pub mod ascii;
pub mod assert;
pub mod bf16;
pub mod constants;
pub mod digit;
pub mod div128;
pub mod error;
pub mod extended_float;
pub mod f16;
pub mod format;
pub mod iterator;
pub mod mul;
pub mod num;
pub mod options;
pub mod result;
pub mod step;

mod api;
mod feature_format;
mod format_builder;
mod format_flags;
mod noskip;
mod not_feature_format;
mod skip;
