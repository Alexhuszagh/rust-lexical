//! Precalculated small powers for decimal 32-bit limbs.

// DECIMAL

pub(super) const POW5: [u32; 14] = [
    1, 5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625,
    1220703125,
];
pub(super) const POW10: [u32; 10] =
    [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
