//! Fast lexical string-to-float conversion routines.

// Hide implementation details.
mod util;
mod basen;

cfg_if! {
    if #[cfg(feature = "correct")] {
        mod bigint;
        mod correct;
    }
}

mod api;

// Re-exports
pub use self::api::*;
