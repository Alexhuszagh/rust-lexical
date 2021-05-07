//! Miscellaneous utilities for lexical-core.

#[macro_use]
mod assert;
#[macro_use]
mod const_fn;
#[macro_use]
mod index;
#[macro_use]
mod interface;
mod rounding;
mod sign;

pub use self::rounding::*;
pub use self::sign::*;

cfg_if! {
if #[cfg(feature = "power_of_two")] {
    mod fill;
    pub(crate) use self::fill::*;
}} // cfg_if

cfg_if! {
if #[cfg(test)] {
    #[macro_use]
    mod test;
    pub(crate) use self::test::*;
}} // cfg_if
