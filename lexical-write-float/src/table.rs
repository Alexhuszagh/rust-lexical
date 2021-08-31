//! Pre-computed tables for writing float strings.

#![doc(hidden)]

// Re-export all the feature-specific files.
#[cfg(not(feature = "compact"))]
pub use crate::table_dragonbox::*;
#[cfg(feature = "compact")]
pub use crate::table_grisu::*;
