//! Replace characters based off a format..

use crate::util::*;

/// Wrapper for dtoa.
///
/// `f` must be non-special (NaN or infinite), non-negative,
/// and non-zero.
#[inline]
pub(crate) fn replace(bytes: &mut [u8], count: usize, format: NumberFormat)
{
    // Replace any values that differ from defaults. Track the index
    let decimal_point = format.decimal_point();
    let exponent = format.exponent(10);
    let mut index = 0;

    // Replace the decimal point
    if decimal_point != b'.' {
        match bytes[..count].iter().position(|&b| b == b'.') {
            Some(idx) => {
                bytes[idx] = decimal_point;
                index = idx;
            },
            None => (),
        };
    }

    // Replace the exponent.
    if exponent != b'e' {
        match bytes[index..count].iter().position(|&b| b == b'e') {
            Some(idx) => bytes[idx] = exponent,
            None => (),
        };
    }
}
