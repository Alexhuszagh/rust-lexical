//! Module specifying float.

// Utilities.
#[macro_use]
mod interface;
#[macro_use]
mod traits;

mod exponent;
mod standard;
mod trim;
mod validate;

// Re-export interface and traits.
pub(crate) use standard::*;
pub(crate) use traits::*;

cfg_if! {
if #[cfg(feature = "format")] {
    mod generic;
    mod permissive;
    mod ignore;

    pub(crate) use generic::*;
    pub(crate) use permissive::*;
    pub(crate) use ignore::*;
}}
