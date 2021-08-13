//! Optimized float parser for hexadecimal floats.
//!
//! This actually works for any case where we can exactly represent
//! any power of the mantissa radix using the exponent base. For example,
//! given a mantissa radix of `16`, and an exponent base of `8`,
//! `16^2` cannot be exactly represented in octal. In short:
//! ⌊log2(r) / log2(b)⌋ == ⌈log2(r) / log2(b)⌉.
//!
//! This gives us the following mantissa radix/exponent base combinations:
//!
//! - 4, 2
//! - 8, 2
//! - 16, 2
//! - 32, 2
//! - 16, 4

#![cfg(feature = "power-of-two")]
#![doc(hidden)]
#![allow(unused)] // TODO(ahuszagh) Remove

use crate::float::{ExtendedFloat80, RawFloat};
use lexical_util::error::Error;
use lexical_util::iterator::{Bytes, BytesIter};
use lexical_util::result::Result;

/// Algorithm specialized for radixes of powers-of-two with different exponent bases.
#[inline]
pub fn hex<F: RawFloat, const FORMAT: u128>(
    mut byte: Bytes<FORMAT>,
    lossy: bool,
) -> Result<ExtendedFloat80> {
    let length = byte.length();
    let (fp, count) = hex_partial::<F, FORMAT>(byte, lossy)?;
    if count == length {
        Ok(fp)
    } else {
        Err(Error::InvalidDigit(count))
    }
}

/// Algorithm specialized for radixes of powers-of-two with different exponent bases.
#[inline]
pub fn hex_partial<F: RawFloat, const FORMAT: u128>(
    mut byte: Bytes<FORMAT>,
    lossy: bool,
) -> Result<(ExtendedFloat80, usize)> {
    // TODO(ahuszagh) Actually... this doesn't work... LOLOL
    todo!();
}

// TODO(ahuszagh) Implement...
