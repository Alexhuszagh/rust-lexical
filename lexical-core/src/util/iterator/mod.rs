//! Traits and custom iterator implementations.

mod as_ptr;
mod consumed;
mod digit_separator;
#[cfg(feature = "format")]
mod skip;

pub(crate) use self::as_ptr::*;
pub(crate) use self::consumed::*;
pub(crate) use self::digit_separator::*;
