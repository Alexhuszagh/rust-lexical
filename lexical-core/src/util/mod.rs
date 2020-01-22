//! Helper utilities for low-level features.
// Fix a compiler bug that thinks `pow` isn't used.
#![allow(unused_imports)]

// Hide implementation details.
#[macro_use]
mod assert;

#[macro_use]
mod index;

#[macro_use]
mod perftools;

#[macro_use]
mod traits;

#[cfg(test)]
#[macro_use]
pub(crate) mod test;

#[cfg(has_i128)]
mod div128;

// Hide implementation details.
mod algorithm;
mod cast;
mod config;
mod consume;
mod error;
mod format;
mod iterator;
mod mask;
mod num;
mod pointer_methods;
mod primitive;
mod pow;
mod result;
mod rounding;
mod sign;
mod table;

#[cfg(feature = "format")]
mod skip_value;

cfg_if! {
if #[cfg(feature = "correct")] {
    mod bound;
    mod range_bounds;
    mod slice_index;

    #[macro_use]
    mod sequence;
} else {
    mod wrapped;
}}  // cfg_if

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::cast::*;
pub(crate) use self::consume::*;
pub(crate) use self::iterator::*;
pub(crate) use self::mask::*;
pub(crate) use self::pointer_methods::*;
pub(crate) use self::primitive::*;
pub(crate) use self::pow::*;
pub(crate) use self::rounding::*;
pub(crate) use self::sign::*;
pub(crate) use self::table::*;

#[cfg(has_i128)]
pub(crate) use self::div128::*;

#[cfg(feature = "format")]
pub(crate) use self::skip_value::*;

cfg_if! {
if #[cfg(feature = "correct")] {
    pub(crate) use self::sequence::*;
} else {
    pub(crate) use self::wrapped::*;
}}  // cfg_if

// Publicly export config globally.
pub use self::config::*;
pub use self::error::*;
pub use self::format::*;
pub use self::num::*;
pub use self::result::*;
pub use self::traits::*;

#[cfg(feature = "rounding")]
pub use self::rounding::RoundingKind;
