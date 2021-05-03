//! Integer-to-string formatting routines.

// Hide internal implementation details.
mod api;
mod decimal;
#[cfg(feature = "binary")]
mod generic;

#[cfg(feature = "binary")]
pub(crate) use self::api::{itoa_positive, Itoa};
