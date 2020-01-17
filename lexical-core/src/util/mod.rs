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

// Hide implementation details.
mod algorithm;
mod cast;
pub(crate) mod config;
mod div128;
pub(crate) mod error;
mod mask;
mod num;
mod primitive;
mod pow;
pub(crate) mod result;
mod rounding;
mod sign;
mod table;

#[cfg(feature = "format")]
mod format;

cfg_if! {
if #[cfg(feature = "correct")] {
    #[macro_use]
    mod sequence;
} else {
    mod wrapped;
}}  // cfg_if

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::cast::*;
pub(crate) use self::div128::*;
pub(crate) use self::mask::*;
pub(crate) use self::primitive::*;
pub(crate) use self::pow::*;
pub(crate) use self::rounding::*;
pub(crate) use self::sign::*;
pub(crate) use self::table::*;

cfg_if! {
if #[cfg(feature = "correct")] {
    pub(crate) use self::sequence::*;
} else {
    pub(crate) use self::wrapped::*;
}}  // cfg_if

// Publicly export config globally.
pub use self::config::*;
pub use self::error::*;
pub use self::num::*;
pub use self::result::*;
pub use self::traits::*;

#[cfg(feature = "format")]
pub use self::format::*;

#[cfg(feature = "rounding")]
pub use self::rounding::RoundingKind;
