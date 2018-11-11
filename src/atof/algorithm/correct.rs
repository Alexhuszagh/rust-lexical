//! Correct algorithms for string-to-float conversions.

// TODO(ahuszagh) Come back to later.

// FAST

// Fast path for the parse algorithm.
// In this case, the mantissa can be represented by a.

/// Parse the mantissa and exponent from a string.
///
/// The number must be non-special, non-zero, and positive.
#[inline]
#[allow(unused)]
pub(crate) unsafe extern "C" fn parse_float(first: *const u8, last: *const u8, base: u64)
    -> (u64, i32, *const u8)
{
    // TODO(ahuszagh) Implement...
    (0, 0, last)
}

// SLOW

// TODO(ahuszagh) Here...

// TESTS
// -----

#[cfg(test)]
mod tests {
    // TODO(ahuszagh) Implement
}
