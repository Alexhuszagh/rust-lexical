//! Arbitrary-precision decimal to parse a floating-point number.

// TODO(ahuszgah) Implement...

/// Large, arbitrary-precision float.
pub(crate) struct Bigfloat {
    /// TODO(ahuszagh)
    // Need storage for the raw integers.
    // Need start and end indexes, or some sort of base2 exponent.

    /// Raw data for the underlying buffer.
    // TODO(ahuszagh)
    data: [u32; 32],
    /// Exponent in base32.
    exponent: u32,
}
