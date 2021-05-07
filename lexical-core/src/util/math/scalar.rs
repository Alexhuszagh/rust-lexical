//! Scalar-to-scalar operations.
//!
//! Building-blocks for arbitrary-precision operations.

use crate::util::config::*;
use crate::util::traits::*;

use super::cast::*;

// SCALAR
// ------

// ADDITION

/// Add two small integers and return the resulting value and if overflow happens.
#[inline]
pub fn add(x: Limb, y: Limb) -> (Limb, bool) {
    x.overflowing_add(y)
}

/// AddAssign two small integers and return if overflow happens.
#[inline]
pub fn iadd(x: &mut Limb, y: Limb) -> bool {
    let t = add(*x, y);
    *x = t.0;
    t.1
}

// SUBTRACTION

/// Subtract two small integers and return the resulting value and if overflow happens.
#[inline]
pub fn sub(x: Limb, y: Limb) -> (Limb, bool) {
    x.overflowing_sub(y)
}

/// SubAssign two small integers and return if overflow happens.
#[inline]
pub fn isub(x: &mut Limb, y: Limb) -> bool {
    let t = sub(*x, y);
    *x = t.0;
    t.1
}

// MULTIPLICATION

/// Multiply two small integers (with carry) (and return the overflow contribution).
///
/// Returns the (low, high) components.
#[inline]
pub fn mul(x: Limb, y: Limb, carry: Limb) -> (Limb, Limb) {
    // Cannot overflow, as long as wide is 2x as wide. This is because
    // the following is always true:
    // `Wide::MAX - (Narrow::MAX * Narrow::MAX) >= Narrow::MAX`
    let z: Wide = as_wide(x) * as_wide(y) + as_wide(carry);
    (as_limb(z), as_limb(z >> <Limb as Integer>::BITS))
}

/// Multiply two small integers (with carry) (and return if overflow happens).
#[inline]
pub fn imul(x: &mut Limb, y: Limb, carry: Limb) -> Limb {
    let t = mul(*x, y, carry);
    *x = t.0;
    t.1
}

// DIVISION

/// Divide two small integers (with remainder) (and return the remainder contribution).
///
/// Returns the (value, remainder) components.
#[inline]
pub fn div(x: Limb, y: Limb, rem: Limb) -> (Limb, Limb) {
    // Cannot overflow, as long as wide is 2x as wide.
    let x = as_wide(x) | (as_wide(rem) << <Limb as Integer>::BITS);
    let y = as_wide(y);
    (as_limb(x / y), as_limb(x % y))
}

/// DivAssign two small integers and return the remainder.
#[inline]
pub fn idiv(x: &mut Limb, y: Limb, rem: Limb) -> Limb {
    let t = div(*x, y, rem);
    *x = t.0;
    t.1
}
