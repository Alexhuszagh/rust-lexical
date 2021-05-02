//! Integer-to-string formatting routines.

// Hide internal implementation details.
mod decimal;
mod api;

#[cfg(feature = "radix")]
mod generic;

#[cfg(feature = "radix")]
pub(crate) use self::api::{Itoa, itoa_positive};
