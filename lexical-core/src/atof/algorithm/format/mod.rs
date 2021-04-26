//! Module specifying float.

// Utilities.
mod exponent;
mod trim;
mod validate;

#[macro_use]
mod interface;

#[macro_use]
mod traits;

// Formats
mod standard;

cfg_if! {
if #[cfg(feature = "format")] {
    mod generic;
    mod permissive;
    mod ignore;
}}

// Re-export interface and traits.
pub(crate) use standard::*;
pub(crate) use traits::*;

cfg_if! {
if #[cfg(feature = "format")] {
    pub(crate) use generic::*;
    pub(crate) use permissive::*;
    pub(crate) use ignore::*;
}}
