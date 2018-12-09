//! Helper utilities for low-level features.
// Fix a compiler bug that thinks `pow` isn't used.
#![allow(unused_imports)]

// Hide implementation details.
#[macro_use]
pub(crate) mod api;

mod algorithm;
mod cast;
mod config;
mod mask;
mod num;
mod pow;
mod primitive;
mod span;
mod state;

#[cfg(any(test, not(feature = "imprecise")))]
mod veclike;

#[cfg(any(test, feature = "imprecise"))]
mod wrapped;

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::cast::*;
pub(crate) use self::mask::*;
pub(crate) use self::num::*;
pub(crate) use self::pow::*;
pub(crate) use self::primitive::*;
pub(crate) use self::span::*;
pub(crate) use self::state::*;

#[cfg(any(test, not(feature = "imprecise")))]
pub(crate) use self::veclike::*;

#[cfg(any(test, feature = "imprecise"))]
pub(crate) use self::wrapped::*;

// Publicly export config globally.
pub use self::config::*;
