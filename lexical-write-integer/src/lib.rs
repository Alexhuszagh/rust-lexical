//! Fast lexical integer-to-string conversion routines.
//!
//! The default implementations use power reduction to unroll
//! 4 loops at a time to minimize the number of required divisions,
//! leading to massive performance gains. In addition, decimal
//! strings pre-calculate the number of digits, avoiding temporary buffers.
//!
//! A compact, fallback algorithm uses a naive, simple algorithm,
//! where each loop generates a single digit. This comes at a performance
//! penalty, but produces smaller binaries.
//!
//! # Features
//!
//! * `std` - Use the standard library.
//! * `power-of-two` - Add support for writing power-of-two integer strings.
//! * `radix` - Add support for strings of any radix.
//! * `compact` - Reduce code size at the cost of performance.
//! * `safe` - Ensure only memory-safe indexing is used.
//!
//! # Note
//!
//! Only documented functionality is considered part of the public API:
//! any of the modules, internal functions, or structs may change
//! release-to-release without major or minor version changes. Use
//! internal implementation details at your own risk.
//!
//! lexical-write-integer mainly exists as an implementation detail for
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
//! # Design
//!
//! - [Algorithm Approach](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Algorithm.md)
//! - [Benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Benchmarks.md)

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod index;

pub mod algorithm;
pub mod compact;
pub mod decimal;
pub mod options;
pub mod radix;
pub mod table;
pub mod write;

mod api;
mod table_binary;
mod table_decimal;
mod table_radix;

// Re-exports
pub use self::api::{ToLexical, ToLexicalWithOptions};
#[doc(inline)]
pub use self::options::{Options, OptionsBuilder};
pub use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
pub use lexical_util::format::{self, NumberFormatBuilder};
pub use lexical_util::options::WriteOptions;
