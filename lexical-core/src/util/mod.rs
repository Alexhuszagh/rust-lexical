//! Helper utilities for low-level features.
// Fix a compiler bug that thinks `pow` isn't used.
#![allow(unused_imports)]

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
    pub(crate) mod test;    // TODO(ahuszagh) Remove pub visibility.
    pub(crate) use self::test::*;
}}  // cfg_if

mod algorithm;
mod consume;
mod digit;
mod format;    // TODO(ahuszagh) Move to crate::options
mod limb;
mod rounding;
mod options;   // TODO(ahuszagh) Move to crate::options
mod sign;

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::consume::*;
pub(crate) use self::digit::*;
pub(crate) use self::limb::*;

// Publicly export config globally.
pub use self::format::*;    // TODO(ahuszagh) Move to crate::options
pub use self::options::*;   // TODO(ahuszagh) Move to crate::options
pub use self::rounding::*;
pub use self::sign::*;

cfg_if! {
if #[cfg(any(feature = "atoi", feature = "itoa", all(feature = "ftoa", feature = "radix")))] {
    mod div128;
    pub(crate) use self::div128::*;
}}  // cfg_if

cfg_if! {
if #[cfg(any(feature = "atof", feature = "atoi"))] {
    mod iterator;
    pub(crate) use self::iterator::*;
}}  // cfg_if

cfg_if! {
if #[cfg(any(feature = "atof", all(feature = "ftoa", feature = "radix")))] {
    mod log2;
    pub(crate) use self::log2::*;
}}  // cfg_if

cfg_if! {
if #[cfg(all(any(feature = "atof", feature = "atoi"), feature = "format"))] {
    mod skip_value;
    pub(crate) use self::skip_value::*;
}}  // cfg_if
