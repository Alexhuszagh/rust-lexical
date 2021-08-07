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
//! 3. An optimized algorithm for formatting to string with power-of-two radixes.
//! 4. An optimized algorithm for hexadecimal floats.
//! 5. A fallback algorithm for all other radixes.
//!
//! The Grisu algorithm is based on "Printing Floating-Point Numbers Quickly
//! and Accurately with Integers", by Florian Loitsch, available online at:
//! <https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf>.
//!
//! The dragonbox algorithm is based on the reference C++ implementation,
//! hosted here: <https://github.com/jk-jeon/dragonbox/>, and the algorithm
//! is described in depth here:
//! <https://github.com/jk-jeon/dragonbox/blob/master/other_files/Dragonbox.pdf>
//!
//! The radix algorithm is adapted from the V8 codebase, and may be found here:
//! <https://github.com/v8/v8>.

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod index;

pub mod algorithm;
pub mod binary;
pub mod compact;
pub mod hex;
pub mod options;
pub mod radix;

// Re-exports
//pub use self::api::{ToLexical, ToLexicalWithOptions};
pub use self::options::Options;
