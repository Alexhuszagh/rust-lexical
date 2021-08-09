//! Consistent API when the `format` feature is enabled or disabled.

#[cfg(feature = "format")]
pub use crate::feature_format::*;
pub use crate::format_builder::*;
pub use crate::format_flags::*;
#[cfg(not(feature = "format"))]
pub use crate::not_feature_format::*;

use static_assertions::const_assert;

/// Standard number format. This is identical to the Rust string format.
pub const STANDARD: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ STANDARD }> {}.is_valid());
