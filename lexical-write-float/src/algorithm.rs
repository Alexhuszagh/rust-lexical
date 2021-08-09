//! Implementation of the Dragonbox algorithm.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use crate::options::Options;
use lexical_util::num::Float;

// TODO(ahuszagh) Implement...

/// Optimized float-to-string algorithm for decimal strings.
/// # Safety
///
/// Safe as long as the float isn't special (NaN or Infinity), and `bytes`
/// is large enough to hold the significant digits.
#[allow(unused)] // TODO(ahuszagh) Remove...
pub unsafe fn write_float<F: Float, const FORMAT: u128>(
    float: F,
    bytes: &mut [u8],
    options: &Options,
) -> usize {
    todo!();
}
