//! Exact conversions to f32 for basen exponents.
//!
//! All these expect a valid exponent, which can be exactly represented
//! for the base in question. Overflow and underflow checks should occur in
//! `correct::atof`.

use util::*;

// BASEN TO EXACT

/// Convert base power-of-2 number to exact representation.
#[inline(always)]
pub(super) unsafe fn pow2_to_exact(float: f32, scalar:i32, exponent: i32) -> f32 {
    pow2_f32(float, scalar*exponent)
}

/// Convert basen number to exact representation.
#[inline(always)]
pub(super) unsafe fn basen_to_exact(float: f32, base:u64, exponent: i32) -> f32 {
    pown_f32(float, base, exponent)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    // TODO(ahuszagh) Implement...
    // pow2_to_exact
    // basen_to_exact
}
