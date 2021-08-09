// TODO(ahuszagh) Document...

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

// Re-exports
pub use lexical_util::error::Error;
pub use lexical_util::format::{NumberFormatBuilder, STANDARD};
pub use lexical_util::result::Result;
