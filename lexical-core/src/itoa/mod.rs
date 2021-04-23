//! Integer-to-string formatting routines.

// Hide internal implementation details.
mod decimal;

#[cfg(feature = "radix")]
mod generic;

mod api;

#[cfg(feature = "radix")]
pub(crate) use self::api::itoa_positive;
