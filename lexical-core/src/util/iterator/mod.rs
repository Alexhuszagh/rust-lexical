//! Traits and custom iterator implementations.

mod contiguous;
mod digit_separator;
#[cfg(feature = "format")]
mod skip;
mod slice;

pub(crate) use self::contiguous::*;
pub(crate) use self::digit_separator::*;
