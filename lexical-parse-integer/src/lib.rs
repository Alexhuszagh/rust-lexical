//! Fast lexical string-to-integer conversion routines.
//!
//! The default implementations are highly optimized both for simple
//! strings, as well as input with large numbers of digits. In order to
//! keep performance optimal for simple strings, we avoid overly branching
//! to minimize the number of branches (and therefore optimization checks).
//! Most of the branches in the code are resolved at compile-time, and
//! the resulting ASM is monitored to ensure there are no regressions. For
//! larger strings, a limited number of optimization checks are included
//! to try faster, multi-digit parsing algorithms. For 32-bit integers,
//! we try to parse 4 digits at a time, and for 64-bit and larger integers,
//! we try to parse 8 digits at a time. Attempting both checks leads to
//! significant performance penalties for simple strings, so only 1
//! optimization is used at at a time. See [Algorithm.md](/docs/Algorithm.md)
//! for documentation on the algorithms used. See
//! [Benchmarks.md](/docs/Benchmarks.md) for benchmark results compared to
//! other string-to-integer implementations.
//!
//! In addition, a compact, fallback algorithm uses a naive, simple
//! algorithm, parsing only a single digit at a time. This avoid any
//! unnecessary branching and produces smaller binaries, but comes
//! at a significant performance penalty for integers with more digits.

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod shared;

pub mod algorithm;
pub mod compact;
pub mod options;
pub mod parse;

mod api;

// Re-exports
pub use self::api::{FromLexical, FromLexicalWithOptions};
pub use self::options::Options;
