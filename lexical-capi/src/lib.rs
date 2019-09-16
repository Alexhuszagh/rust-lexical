//! C-FFI for lexical conversion routines.
//!
//! This crate has no public functions, type definitions, traits, or
//! variables, and exists solely to export unmangled symbols to
//! static/shared library.

// FEATURES

// Require intrinsics in a no_std context.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), feature(lang_items))]

// EXTERNAL

#[macro_use]
extern crate cfg_if;

extern crate lexical_core;

/// Facade around the core features for name mangling.
pub(crate) mod lib {
#[cfg(feature = "std")]
pub(crate) use std::*;

#[cfg(not(feature = "std"))]
pub(crate) use core::*;

}   // lib

// PANIC

// Need to define a panic handler with no_std.
cfg_if! {
if #[cfg(not(feature = "std"))] {
    use lib::intrinsics;
    use lib::panic::PanicInfo;

    #[panic_handler]
    fn panic(_: &PanicInfo) -> ! {
        unsafe {
            intrinsics::abort();
        }
    }

    #[lang = "eh_personality"]
    extern fn eh_personality() {}
}}  // cfg_if

// API

// Hide implementation details, since they will generate symbols
// but should not be used from Rust.
mod api;
mod config;
mod result;

// We need to export them to the root crate for them to generate symbols.
// Hide all documentation.
pub use self::api::*;
pub use self::config::*;
