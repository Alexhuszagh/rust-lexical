//! Integer-to-string formatting routines.

// Hide internal implementation details.
mod decimal;
mod api;

#[cfg(feature = "radix")]
mod generic;

#[cfg(all(feature = "ftoa", feature = "radix"))]
pub(crate) use self::api::itoa_positive;
