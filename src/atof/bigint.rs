//! Bigint implementation for

pub(crate) struct BigInt {
    /// Internal storage for the numbers.
    k: [u32; 40],
}
