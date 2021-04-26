//! Helper utilities for low-level features.
// Fix a compiler bug that thinks `pow` isn't used.
#![allow(unused_imports)]

// Hide implementation details.
#[macro_use]
mod assert;

#[macro_use]
mod index;

#[macro_use]
mod traits;

#[macro_use]
mod sequence;

#[cfg(test)]
#[macro_use]
pub(crate) mod test;

// Hide implementation details.
mod algorithm;
mod cast;
mod config;
mod consume;
mod digit;
mod div128;
mod error;
mod format;
mod iterator;
mod limb;
mod mask;
mod num;
mod options;
mod primitive;
mod pow;
mod result;
mod rounding;
mod sign;
mod table;
mod wrapped;

#[cfg(feature = "format")]
mod skip_value;

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::cast::*;
pub(crate) use self::consume::*;
pub(crate) use self::digit::*;
pub(crate) use self::div128::*;
pub(crate) use self::iterator::*;
pub(crate) use self::limb::*;
pub(crate) use self::mask::*;
pub(crate) use self::primitive::*;
pub(crate) use self::pow::*;
pub(crate) use self::rounding::*;
pub(crate) use self::sequence::*;
pub(crate) use self::sign::*;
pub(crate) use self::table::*;
pub(crate) use self::wrapped::*;

#[cfg(feature = "format")]
pub(crate) use self::skip_value::*;

// Publicly export config globally.
pub use self::config::*;    // TODO(ahuszagh) Remove the config options.
pub use self::error::*;
pub use self::format::*;
pub use self::num::*;
pub use self::options::*;
pub use self::result::*;
pub use self::traits::*;

#[cfg(feature = "rounding")]
pub use self::rounding::RoundingKind;
