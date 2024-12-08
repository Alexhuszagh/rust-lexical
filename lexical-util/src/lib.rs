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
//! The minimum, standard, required version is 1.63.0, for const generic
//! support. Older versions of lexical support older Rust versions.
//!
//! # Safety Guarantees
//!
//! The only major sources of unsafe code are wrapped in the `iterator.rs`,
//! `skip.rs`, and `noskip.rs`. These are fully encapsulated into standalone
//! traits to clearly define safety invariants and localize any unsafety to
//! 1 or 2 lines of code.
//!
//! The core, unsafe trait is `DigitsIter` and `Iter`, both which expect
//! to be backed by a contiguous block of memory (a slice) but may skip
//! bytes internally. To guarantee safety, for non-skip iterators you
//! must implement [DigitsIter::is_consumed][is_consumed] correctly.
//!
//! This must correctly determine if there are any elements left in the
//! iterator. If the buffer is contiguous, this can just be `index ==
//! self.len()`, but for a non-contiguous iterator it must skip any digits to
//! advance to the element next to be returned or the iterator itself will be
//! unsafe. **ALL** other safety invariants depend on this being implemented
//! correctly.
//!
//! To see if the cursor is at the end of the buffer, use
//! [is_buffer_empty][is_buffer_empty].
//!
//! Any iterators must be peekable: you must be able to read and return the next
//! value without advancing the iterator past that point. For iterators that
//! skip bytes, this means advancing to the next element to be returned and
//! returning that value.
//!
//! For examples of how to safely implement skip iterators, you can do something
//! like:
//!
//! ```rust,ignore
//! impl<_> DigitsIter<_> for MyIter {
//!     fn peek(&mut self) -> Option<u8> {
//!         loop {
//!             let value = self.bytes.get(self.index)?;
//!             if value != &b'.' {
//!                 return value;
//!             }
//!             self.index += 1;
//!         }
//!     }
//! }
//! ```
//!
//! Then, [next](core::iter::Iterator::next) will be implemented in terms
//! of [peek], incrementing the position in the cursor just after the value.
//! The next iteration of peek will step to the correct byte to return.
//!
//! ```rust,ignore
//! impl<_> Iterator for MyIter {
//!     type Item = &'a u8;
//!
//!     fn next(&mut self) -> Option<Self::Item> {
//!         let value = self.peek()?;
//!         self.index += 1;
//!         Some(value)
//!     }
//! }
//! ```
//!
//! [is_buffer_empty]: <https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs#76>
//! [is_consumed]: <https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs#L276>
//! [peek]: <https://github.com/Alexhuszagh/rust-lexical/blob/8fe1d9a/lexical-util/src/iterator.rs#L284>

// FIXME: Implement clippy/allow reasons once we drop support for 1.80.0 and below
// Clippy reasons were stabilized in 1.81.0.

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

// Ensure our features are properly enabled. This means no parse without
// parse support, etc.
#[cfg(all(feature = "parse", not(any(feature = "parse-integers", feature = "parse-floats"))))]
compile_error!(
    "Do not use the `parse` feature directly. Use `parse-integers` and/or `parse-floats` instead."
);

#[cfg(all(feature = "write", not(any(feature = "write-integers", feature = "write-floats"))))]
compile_error!(
    "Do not use the `write` feature directly. Use `write-integers` and/or `write-floats` instead."
);

#[cfg(all(feature = "integers", not(any(feature = "write-integers", feature = "parse-integers"))))]
compile_error!("Do not use the `integers` feature directly. Use `write-integers` and/or `parse-integers` instead.");

#[cfg(all(feature = "floats", not(any(feature = "write-floats", feature = "parse-floats"))))]
compile_error!(
    "Do not use the `floats` feature directly. Use `write-floats` and/or `parse-floats` instead."
);

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
mod numtypes;
mod skip;
