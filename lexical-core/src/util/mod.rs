//! Helper utilities for low-level features.

// Hide implementation details.
#[macro_use]
mod assert;
#[macro_use]
mod const_fn;
#[macro_use]
mod index;
#[macro_use]
mod interface;

cfg_if! {
if #[cfg(test)] {
    #[macro_use]
    mod test;
    pub(crate) use self::test::*;
}} // cfg_if

mod algorithm;
mod consume;
mod digit;
mod div128;
mod format; // TODO(ahuszagh) Move to crate::options
mod iterator;
mod limb;
mod log2;
mod options; // TODO(ahuszagh) Move to crate::options
mod rounding;
mod sign;

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::consume::*;
pub(crate) use self::digit::*;
pub(crate) use self::div128::*;
pub(crate) use self::iterator::*;
pub(crate) use self::limb::*;
pub(crate) use self::log2::*;

// Publicly export config globally.
pub use self::format::*; // TODO(ahuszagh) Move to crate::options
pub use self::options::*; // TODO(ahuszagh) Move to crate::options
pub use self::rounding::*;
pub use self::sign::*;

cfg_if! {
if #[cfg(feature = "format")] {
    mod skip_value;
    pub(crate) use self::skip_value::*;
}} // cfg_if
