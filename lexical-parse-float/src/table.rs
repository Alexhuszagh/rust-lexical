//! Pre-computed tables for writing float strings.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

// Re-export all the feature-specific files.
#[cfg(feature = "power-of-two")]
pub use crate::table_binary::*;
pub use crate::table_decimal::*;
pub use crate::table_lemire::*;
#[cfg(feature = "radix")]
pub use crate::table_radix::*;
