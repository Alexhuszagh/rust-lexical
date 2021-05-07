//! Shared utilities for implementing parsing and writing algorithms.
//!
//! These utilities include those re-exported as part of the public
//! API, but mostly include pre-calculated tables, arbitrary-precision
//! arithmetic, extended-precision floats, and an assortment of
//! algorithms.

// Hide implementation details.
//
// To speed up compile times and simplify the internal logic,
// the following modules, in order, is as follows:
//      - misc
//      - config
//      - error
//      - result
//      - digit
//      - powers
//      - options
//      - traits
//      - algorithm
//      - math
//      - float
//      - cached

// Hide implementation details.
#[macro_use]
mod misc;
#[macro_use]
mod traits;
mod algorithm;
mod cached;
mod config;
mod digit;
mod error;
mod float;
mod iterator;
mod math;
mod result;
mod powers;

// TODO(ahuszagh) Need to rework these two.
mod format;
mod options;

// Publicly export everything with crate-visibility.
pub(crate) use self::algorithm::*;
pub(crate) use self::cached::*;
pub(crate) use self::config::*;
pub(crate) use self::digit::*;
pub(crate) use self::float::*;
pub(crate) use self::iterator::*;
pub(crate) use self::math::*;
pub(crate) use self::powers::*;

// Publicly export config globally.
pub use self::error::*;
pub use self::format::*; // TODO(ahuszagh) Move to crate::options
pub use self::options::*; // TODO(ahuszagh) Move to crate::options
pub use self::misc::*;
pub use self::result::*;
pub use self::traits::*;
