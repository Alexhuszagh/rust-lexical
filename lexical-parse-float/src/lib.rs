//! Fast and compact string-to-float conversions.
//!
//! # Note
//!
//! Only documented functionality is considered part of the public API:
//! any of the modules, internal functions, or structs may change
//! release-to-release without major or minor version changes. Use
//! internal implementation details at your own risk.
//!
//! lexical-parse-float mainly exists as an implementation detail for
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
//!
#![doc = include_str!("../docs/Algorithm.md")]
//!
#![doc = include_str!("../docs/Benchmarks.md")]
// TODO(ahuszagh) Document...

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(asm))]

#[macro_use]
mod index;

pub mod bellerophon;
pub mod binary;
pub mod compact;
pub mod float;
pub mod fpu;
pub mod lemire;
pub mod libm;
pub mod limits;
pub mod number;
pub mod options;
pub mod slow;
pub mod table;

mod api;
mod parse;
mod table_bellerophon;
mod table_binary;
mod table_decimal;
mod table_lemire;
mod table_radix;

// Re-exports
pub use self::api::{FromLexical, FromLexicalWithOptions};
#[doc(inline)]
pub use self::options::{Options, OptionsBuilder};
pub use lexical_util::error::Error;
pub use lexical_util::format::{self, NumberFormatBuilder};
pub use lexical_util::options::ParseOptions;
pub use lexical_util::result::Result;
