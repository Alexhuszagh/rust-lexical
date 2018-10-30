// FEATURES

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]
#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(all(test, feature = "alloc", not(feature = "std")))]
extern crate wee_alloc;

#[cfg(all(feature = "alloc", not(feature = "std")))]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
pub mod ftoa;
pub mod itoa;
pub mod table;

// HIGH LEVEL

// TODO(ahuszagh) Implement...
