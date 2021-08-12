//! An implementation of Clinger's Bellerophon algorithm.
//!
//! This is a moderate path algorithm that uses an extended-precision
//! float, represented in 80 bits, by calculating the bits of slop
//! and determining if those bits could prevent unambiguous rounding.
//!
//! This algorithm requires less static storage than the Lemire algorithm,
//! and has decent performance, and is therefore used when non-decimal,
//! non-power-of-two strings need to be parsed. Clinger's algorithm
//! is described in depth in "How to Read Floating Point Numbers Accurately.",
//! available online [here](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.45.4152&rep=rep1&type=pdf).
//!
//! This implementation is loosely based off the Golang implementation,
//! found [here](https://github.com/golang/go/blob/b10849fbb97a2244c086991b4623ae9f32c212d0/src/strconv/extfloat.go)

#![cfg(not(feature = "compact"))]
#![cfg(feature = "radix")]
#![doc(hidden)]

use crate::float::{ExtendedFloat80, RawFloat};
use crate::number::Number;
// TODO(ahuszagh) Need the table imports...

/// Ensure truncation of digits doesn't affect our computation, by doing 2 passes.
#[inline]
pub fn bellerophon<F: RawFloat, const FORMAT: u128>(num: &Number) -> ExtendedFloat80 {
    // TODO(ahuszagh) NEed to do the old implementation, except with a biased
    // float.

    // So, what's the bias coefficient again?
    todo!();
}

// TODO(ahuszagh) Implement...
