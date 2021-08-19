//! Fast lexical string-to-float conversion routines.
//!
//! The default implementations are highly optimized both for simple
//! strings, as well as input with large numbers of digits. In order to
//! keep performance optimal for simple strings, we avoid overly branching
//! to minimize the number of branches (and therefore optimization checks).
//! Most of the branches in the code are resolved at compile-time, and
//! the resulting ASM as well as comprehensive benchmarks are monitored
//! to ensure there are no regressions. For simple floats, we use an
//! optimized digit parser with multiple-digit optimizations (parsing
//! 8 digits in 3 multiplication instructions).
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
//!
#![doc = include_str!("../docs/BigInteger.md")]
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
pub mod bigint;
pub mod binary;
pub mod float;
pub mod fpu;
pub mod lemire;
pub mod libm;
pub mod limits;
pub mod mask;
pub mod number;
pub mod options;
pub mod parse;
pub mod shared;
pub mod slow;
pub mod table;

mod api;
mod table_bellerophon_decimal;
mod table_bellerophon_radix;
mod table_binary;
mod table_decimal;
mod table_large;
mod table_lemire;
mod table_radix;
mod table_small;

// Re-exports
pub use self::api::{FromLexical, FromLexicalWithOptions};
#[doc(inline)]
pub use self::options::{Options, OptionsBuilder};
pub use lexical_util::error::Error;
pub use lexical_util::format::{self, NumberFormatBuilder};
pub use lexical_util::options::ParseOptions;
pub use lexical_util::result::Result;
