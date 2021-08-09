//! Fast lexical integer-to-string conversion routines.
//!
//! The default implementations use power reduction to unroll
//! 4 loops at a time to minimize the number of required divisions,
//! leading to massive performance gains. In addition, decimal
//! strings pre-calculate the number of digits, avoiding temporary buffers.
//! See [`Algorithm.md`] for documentation on the algorithms used. See
//! [`Benchmarks.md`] for benchmark results compared to other
//! integer-to-string implementations.
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
//! Only documentation functionality os considered part of the public API:
//! any of the modules, internal functions, or structs may change
//! release-to-release without major or minor version changes. Use
//! internal implementation details at your own risk.
//!
//! [`Algorithm.md`]: https://github.com/Alexhuszagh/rust-lexical-experimental/blob/main/lexical-write-integer/docs/Algorithm.md
//! [`Benchmarks.md`]: https://github.com/Alexhuszagh/rust-lexical-experimental/blob/main/lexical-write-integer/docs/Benchmarks.md

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
pub use self::options::{Options, OptionsBuilder};
pub use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
pub use lexical_util::format::{NumberFormatBuilder, STANDARD};
