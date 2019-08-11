//! Integer-to-string formatting routines.

// Hide internal implementation details.
#[cfg(all(feature = "table"))]
mod base10;

#[cfg(all(feature = "table", feature = "radix"))]
mod generic;

#[cfg(not(feature = "table"))]
mod naive;

mod api;

// Re-export everything from the API.
pub use self::api::*;
