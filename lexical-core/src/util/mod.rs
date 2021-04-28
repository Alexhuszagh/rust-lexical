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
mod traits;

#[macro_use]
#[cfg(feature = "atof")]
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
mod format;
mod limb;
mod num;
mod options;
mod primitive;
mod pow;
mod sign;
mod table;

#[cfg(any(feature = "atoi", feature = "itoa", all(feature = "ftoa", feature = "radix")))]
mod div128;

#[cfg(any(feature = "atof", feature = "atoi"))]
mod error;

#[cfg(any(feature = "atof", feature = "atoi"))]
mod iterator;

#[cfg(any(feature = "atof", feature = "ftoa"))]
mod mask;

#[cfg(any(feature = "atof", feature = "atoi"))]
mod result;

#[cfg(any(feature = "atof", feature = "ftoa"))]
mod rounding;

#[cfg(all(any(feature = "atof", feature = "atoi"), feature = "format"))]
mod skip_value;

#[cfg(feature = "atof")]
mod wrapped;

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::cast::*;
pub(crate) use self::consume::*;
pub(crate) use self::digit::*;
pub(crate) use self::limb::*;
pub(crate) use self::primitive::*;
pub(crate) use self::pow::*;
pub(crate) use self::table::*;

#[cfg(any(feature = "atoi", feature = "itoa", all(feature = "ftoa", feature = "radix")))]
pub(crate) use self::div128::*;

#[cfg(any(feature = "atof", feature = "atoi"))]
pub(crate) use self::iterator::*;

#[cfg(any(feature = "atof", feature = "ftoa"))]
pub(crate) use self::mask::*;

#[cfg(feature = "atof")]
pub(crate) use self::sequence::*;

#[cfg(all(any(feature = "atof", feature = "atoi"), feature = "format"))]
pub(crate) use self::skip_value::*;

#[cfg(feature = "atof")]
pub(crate) use self::wrapped::*;

// Publicly export config globally.
pub use self::config::*;
pub use self::format::*;
pub use self::num::*;
pub use self::options::*;
pub use self::sign::*;
pub use self::traits::*;

#[cfg(any(feature = "atof", feature = "atoi"))]
pub use self::error::*;

#[cfg(any(feature = "atof", feature = "atoi"))]
pub use self::result::*;

// Always export RoundingKind since it's needed for the Options API.
#[cfg(any(feature = "atof", feature = "ftoa"))]
pub use self::rounding::RoundingKind;
