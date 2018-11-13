//! Decimal (big integer) implementation for exact string-to-float conversions.

// TODO(ahuszagh) Implement...

/// Large, stack-allocated integer for exact string-to-float conversions.
#[allow(dead_code)]     // TODO(ahuszagh) Remove
pub(crate) struct Decimal {
    /// Internal storage for the numbers.
    k: [u32; 40],
}

// TESTS
// -----

#[cfg(test)]
mod tests {
//    use super::*;
// TODO(ahuszagh) Implement...
}
