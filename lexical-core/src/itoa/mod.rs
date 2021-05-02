//! Integer-to-string formatting routines.

// Hide internal implementation details.
mod api;
mod decimal;

#[cfg(feature = "radix")]
mod generic;

#[cfg(feature = "radix")]
pub(crate) use self::api::{itoa_positive, Itoa};
