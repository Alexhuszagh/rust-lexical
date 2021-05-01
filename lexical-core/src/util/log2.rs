//! Fast integral log2 implementation.

/// Check if value is power of 2 and get the power.
/// Only works for radices.
#[inline(always)]
pub(crate) fn log2(value: u32) -> i32 {
    match value {
        2  => 1,
        4  => 2,
        8  => 3,
        16 => 4,
        32 => 5,
        _  => 0,
    }
}
