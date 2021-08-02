// TODO(ahuszagh) Document...

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod algorithm;
pub mod binary;
pub mod compact;
pub mod options;

// Re-exports
//pub use self::api::{ToLexical, ToLexicalWithOptions};
pub use self::options::Options;
