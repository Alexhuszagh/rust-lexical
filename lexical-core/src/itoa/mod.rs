//! Integer-to-string formatting routines.

// Hide internal implementation details.
mod api;
mod decimal;
#[cfg(feature = "power_of_two")]
mod generic;

#[cfg(all(feature = "write_floats", feature = "power_of_two"))]
pub(crate) use self::api::{itoa_positive, Itoa};
