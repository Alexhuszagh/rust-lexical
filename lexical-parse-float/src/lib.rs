//! Fast lexical string-to-float conversion routines.
//!
//! The default implementations are highly optimized both for simple
//! strings, as well as input with large numbers of digits. In order to
//! keep performance optimal for simple strings, we avoid overly branching
//! to minimize the number of branches (and therefore optimization checks).
//! Most of the branches in the code are resolved at compile-time, and
//! the resulting ASM as well as comprehensive benchmarks are monitored
//! to ensure there are no regressions.
//!
//! For simple floats, we use an optimized digit parser with multiple-digit
//! optimizations (parsing 8 digits in 3 multiplication instructions),
//! and then use machine floats to create an exact representation with
//! high throughput. In more complex cases, we use the Eisel-Lemire
//! algorithm, described in "Number Parsing at a Gigabyte per Second",
//! available online [here](https://arxiv.org/abs/2101.11408). The
//! Eisel-Lemire algorithm creates an extended representation using a
//! 128-bit (or a fallback 192-bit representation) of the significant
//! digits of the float, scaled to the proper exponent using pre-computed
//! powers-of-5.
//!
//! If the Eisel-Lemire algorithm is unable to unambiguously round the float,
//! we fallback to using optimized, big-integer algorithms, which are
//! described in [Algorithm Approach](#algorithm-approach) below.
//!
//! # Features
//!
//! * `std` - Use the standard library.
//! * `power-of-two` - Add support for parsing power-of-two integer strings.
//! * `radix` - Add support for strings of any radix.
//! * `format` - Add support for parsing custom integer formats.
//! * `compact` - Reduce code size at the cost of performance.
//! * `safe` - Ensure only memory-safe indexing is used.
//! * `nightly` - Enable assembly instructions to control FPU rounding modes.
//!
//! `safe` has a fairly minimal impact, since all parsers are memory-safe
//! by default except where unsafe functionality can trivially be proven
//! correct.
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
//! # Machine Float-Only Algorithm
//!
//! We also support an algorithm that uses only machine floats for the
//! fast-path algorithm, however, this may be slower for floats with large
//! exponents since it uses an iterative algorithm. A code sample
//! using this is:
//!
//! ```rust
//! use lexical_parse_float::Options;
//! use lexical_parse_float::format::STANDARD;
//! use lexical_parse_float::parse::ParseFloat;
//!
//! let options = Options::new();
//! let result = f64::fast_path_complete::<{ STANDARD }>(b"1.34000", &options);
//! assert_eq!(result, Ok(1.34000));
//! ```
//!
//! # Version Support
//!
//! The minimum, standard, required version is 1.51.0, for const generic
//! support. Older versions of lexical support older Rust versions.
//!
//! # Design
//!
//! - [Algorithm Approach](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-float/docs/Algorithm.md)
//! - [Benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-float/docs/Benchmarks.md)
//! - [Big Integer Implementation](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-float/docs/BigInteger.md)

// Inline ASM was stabilized in 1.59.0.
// FIXME: Remove when the MSRV for Rustc >= 1.59.0.
#![allow(stable_features)]
// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(asm))]

#[macro_use]
mod index;
#[macro_use]
pub mod shared;

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
#[cfg(feature = "f16")]
pub use lexical_util::bf16::bf16;
pub use lexical_util::error::Error;
#[cfg(feature = "f16")]
pub use lexical_util::f16::f16;
pub use lexical_util::format::{self, NumberFormatBuilder};
pub use lexical_util::options::ParseOptions;
pub use lexical_util::result::Result;
