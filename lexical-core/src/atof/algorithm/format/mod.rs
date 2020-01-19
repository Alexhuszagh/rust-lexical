//! Module specifying float.

// Utilities.
mod consume;
mod exponent;
mod iterator;
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
    mod permissive;
    mod ignore;
    // TODO(ahuszagh) Add more here...
}}

// Re-export interface and traits.
pub(super) use standard::*;
pub(super) use traits::*;

cfg_if! {
if #[cfg(feature = "format")] {
    pub(super) use permissive::*;
    // TODO(ahuszagh) Add more here...
}}
