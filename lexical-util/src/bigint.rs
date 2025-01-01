//! Re-exports of our big integer types for parsing support.
//!
//! This enables high-performance parsing and serialization
//! of big integers.

#![cfg(feature = "bigint")]

pub use ::i256::{i256, u256};
#[cfg(feature = "i384")]
pub use ::i256::{I384 as i384, U384 as u384};
#[cfg(feature = "i512")]
pub use ::i256::{I512 as i512, U512 as u512};
#[cfg(feature = "i1024")]
pub use ::i256::{I1024 as i1024, U1024 as u1024};
