// FEATURES

/// Facade around the core features for name mangling.
pub(crate) mod sealed {
    #[cfg(not(feature = "std"))]
    pub use core::*;

    #[cfg(feature = "std")]
    pub use std::*;
}

// Hide the implementation details.
mod c;

// Publicly export the low-level APIs.
pub mod atof;
pub mod atoi;
// TODO(ahuszagh)   Remove
#[allow(dead_code)] pub mod ftoa;
pub mod itoa;
pub mod table;

// HIGH LEVEL

// TODO(ahuszagh) Implement...
