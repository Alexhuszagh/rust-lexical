//! Helper utilities for low-level features.
// Fix a compiler bug that thinks `pow` isn't used.
#![allow(unused_imports)]

// Hide implementation details.
#[macro_use]
pub(crate) mod api;

mod algorithm;
mod cast;
mod config;
mod num;
mod pow;
mod primitive;

#[cfg(any(test, not(feature = "correct")))]
mod wrapped;

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::cast::*;
pub(crate) use self::num::*;
pub(crate) use self::pow::*;

#[cfg(any(test, not(feature = "correct")))]
pub(crate) use self::wrapped::*;

// Publicly export config globally.
pub use self::config::*;
