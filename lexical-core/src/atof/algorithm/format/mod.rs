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
pub(super) use self::standard::*;
pub(super) use self::traits::*;

cfg_if! {
if #[cfg(feature = "format")] {
    pub(super) use self::generic::*;
    pub(super) use self::permissive::*;
    pub(super) use self::ignore::*;
}}
