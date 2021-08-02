//! Fast lexical integer-to-string conversion routines.
//!
//! The default implementations use power reduction to unroll
//! 4 loops at a time to minimize the number of required divisions,
//! leading to massive performance gains. In addition, decimal
//! strings pre-calculate the number of digits, avoiding temporary buffers.
//! See [Algorithm.md](/docs/Algorithm.md) for documentation on the
//! algorithms used. See [Benchmarks.md](/docs/Benchmarks.md) for
//! benchmark results compared to other integer-to-string implementations.
//!
//! A compact, fallback algorithm uses a naive, simple algorithm,
//! where each loop generates a single digit. This comes at a performance
//! penalty, but produces smaller binaries.

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

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
pub use self::options::Options;
