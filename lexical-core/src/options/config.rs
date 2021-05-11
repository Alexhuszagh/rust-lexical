//! Default values for the options API.

use crate::util::*; // only need misc, change to this
use crate::lib::{mem, num};
use static_assertions::const_assert;

use super::number::*;

// TYPES
// -----

/// Type with the exact same size as a `u8`.
pub type OptionU8 = Option<num::NonZeroU8>;

/// Type with the exact same size as a `usize`.
pub type OptionUsize = Option<num::NonZeroUsize>;

// Ensure the sizes are identical.
const_assert!(mem::size_of::<OptionU8>() == mem::size_of::<u8>());
const_assert!(mem::size_of::<OptionUsize>() == mem::size_of::<usize>());

// CONSTANTS
// ---------

// Constants to dictate default values for options.
pub(super) const DEFAULT_RADIX: u8 = 10;
pub(super) const DEFAULT_FORMAT: NumberFormatV2 = NumberFormatV2::STANDARD;
pub(super) const DEFAULT_INF_STRING: &'static [u8] = b"inf";
pub(super) const DEFAULT_NAN_STRING: &'static [u8] = b"NaN";
pub(super) const DEFAULT_INFINITY_STRING: &'static [u8] = b"infinity";
pub(super) const DEFAULT_INCORRECT: bool = false;
pub(super) const DEFAULT_LOSSY: bool = false;
pub(super) const DEFAULT_ROUNDING: RoundingKind = RoundingKind::NearestTieEven;
pub(super) const DEFAULT_TRIM_FLOATS: bool = false;
pub(super) const DEFAULT_EXPONENT: u8 = b'e';
pub(super) const DEFAULT_DECIMAL_POINT: u8 = b'.';
pub(super) const MAX_SPECIAL_STRING: usize = 50;
