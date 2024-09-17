//! Fast and compact float-to-string conversions.
//!
//! # Features
//!
//! Each float formatter contains extensive formatting control, including
//! a maximum number of significant digits written, a minimum number of
//! significant digits remaining, the positive and negative exponent break
//! points (at what exponent, in scientific-notation, to force scientific
//! notation), whether to force or disable scientific notation, and the
//! rounding mode for truncated float strings.
//!
//! # Algorithms
//!
//! There's currently 5 algorithms used, depending on the requirements.
//!
//! 1. Compact for decimal strings uses the Grisu algorithm.
//! 2. An optimized algorithm based on the Dragonbox algorithm.
//! 3. An optimized algorithm for formatting to string with power-of-two
//!    radixes.
//! 4. An optimized algorithm for hexadecimal floats.
//! 5. A fallback algorithm for all other radixes.
//!
//! The Grisu algorithm is based on "Printing Floating-Point Numbers Quickly
//! and Accurately with Integers", by Florian Loitsch, available online
//! [here](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf).
//! The dragonbox algorithm is based on the reference C++ implementation,
//! hosted [here](https://github.com/jk-jeon/dragonbox/), and the algorithm
//! is described in depth
//! [here](https://github.com/jk-jeon/dragonbox/blob/master/other_files/Dragonbox.pdf).
//! The radix algorithm is adapted from the V8 codebase, and may be found
//! [here](https://github.com/v8/v8).
//!
//! # Features
//!
//! * `std` - Use the standard library.
//! * `power-of-two` - Add support for wring power-of-two float strings.
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
//! lexical-write-float mainly exists as an implementation detail for
//! lexical-core, although its API is stable. If you would like to use
//! a high-level API that writes to and parses from `String` and `&str`,
//! respectively, please look at [lexical](https://crates.io/crates/lexical)
//! instead. If you would like an API that supports multiple numeric
//! conversions, please look at [lexical-core](https://crates.io/crates/lexical-core)
//! instead.
//!
//! # Version Support
//!
//! The minimum, standard, required version is 1.63.0, for const generic
//! support. Older versions of lexical support older Rust versions.
//!
//! # Safety
//!
//! The `unsafe` usage in the options does not actually have any actual safety
//! concerns unless the format is not validated: the float formatters assume
//! NaN and Infinite strings are <= 50 characters. Creating custom format
//! specification with longer special strings and not validating the :
//! incorrectly using the API however can cause incorrect floating
//! point specifications due to conflicting requirements.
//!
//! # Design
//!
//! - [Algorithm Approach](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-float/docs/Algorithm.md)
//! - [Benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-float/docs/Benchmarks.md)
//! - [Comprehensive Benchmarks](https://github.com/Alexhuszagh/lexical-benchmarks)

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    clippy::doc_markdown,
    clippy::unnecessary_safety_comment,
    clippy::semicolon_if_nothing_returned,
    clippy::unwrap_used,
    clippy::as_underscore,
    clippy::doc_markdown
)]
#![allow(
    // used when concepts are logically separate
    clippy::match_same_arms,
    // loss of precision is intentional
    clippy::integer_division,
    // mathematical names use 1-character identifiers
    clippy::min_ident_chars,
    // these are not cryptographically secure contexts
    clippy::integer_division_remainder_used,
    // this can be intentional
    clippy::module_name_repetitions,
    // this is intentional: already passing a pointer and need performance
    clippy::needless_pass_by_value,
    // we use this for inline formatting for unsafe blocks
    clippy::semicolon_inside_block,
)]

#[macro_use]
mod index;
#[macro_use]
mod shared;

pub mod algorithm;
pub mod binary;
pub mod compact;
pub mod float;
pub mod hex;
pub mod options;
pub mod radix;
pub mod table;
pub mod write;

mod api;
mod table_dragonbox;
mod table_grisu;

// Re-exports
#[cfg(feature = "f16")]
pub use lexical_util::bf16::bf16;
pub use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
#[cfg(feature = "f16")]
pub use lexical_util::f16::f16;
pub use lexical_util::format::{self, NumberFormatBuilder};
pub use lexical_util::options::WriteOptions;

pub use self::api::{ToLexical, ToLexicalWithOptions};
#[doc(inline)]
pub use self::options::{Options, OptionsBuilder, RoundMode};
