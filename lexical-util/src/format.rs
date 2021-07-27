//! Consistent API when the `format` feature is enabled or disabled.

#![cfg(feature = "parse")]

pub use crate::format_flags::*;
#[cfg(feature = "format")]
pub use crate::feature_format::*;
#[cfg(not(feature = "format"))]
pub use crate::not_feature_format::*;
