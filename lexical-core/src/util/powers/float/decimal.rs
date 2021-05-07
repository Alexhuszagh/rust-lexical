//! Cached tables for precalculated values for decimal strings.

use static_assertions::const_assert;

// F32

/// Precalculated values of radix**i for i in range [0, arr.len()-1].
/// Each value can be **exactly** represented as that type.
pub(super) const F32_POW10: [f32; 11] = [
    1.0,
    10.0,
    100.0,
    1000.0,
    10000.0,
    100000.0,
    1000000.0,
    10000000.0,
    100000000.0,
    1000000000.0,
    10000000000.0,
];

// Compile-time guarantees for our tables.
const_assert!(F32_POW10[1] / F32_POW10[0] == 10.0);

/// Precalculated values of radix**i for i in range [0, arr.len()-1].
/// Each value can be **exactly** represented as that type.
pub(super) const F64_POW10: [f64; 23] = [
    1.0,
    10.0,
    100.0,
    1000.0,
    10000.0,
    100000.0,
    1000000.0,
    10000000.0,
    100000000.0,
    1000000000.0,
    10000000000.0,
    100000000000.0,
    1000000000000.0,
    10000000000000.0,
    100000000000000.0,
    1000000000000000.0,
    10000000000000000.0,
    100000000000000000.0,
    1000000000000000000.0,
    10000000000000000000.0,
    100000000000000000000.0,
    1000000000000000000000.0,
    10000000000000000000000.0,
];

// Compile-time guarantees for our tables.
const_assert!(F64_POW10[1] / F64_POW10[0] == 10.0);
