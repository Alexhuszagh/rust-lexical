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
pub(super) use self::standard::*;
pub(super) use self::traits::*;

cfg_if! {
if #[cfg(feature = "format")] {
    pub(super) use self::permissive::*;
    // TODO(ahuszagh) Add more here...
}}
