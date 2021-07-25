//! Consistent API when the `format` feature is enabled or disabled.

#[cfg(all(feature = "format", feature = "parse"))]
pub use crate::feature_format::*;

#[cfg(all(not(feature = "format"), feature = "parse"))]
pub use crate::not_feature_format::*;
