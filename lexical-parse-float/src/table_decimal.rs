//! Pre-computed tables for writing decimal strings.

#![doc(hidden)]
#![cfg(not(feature = "compact"))]

use crate::limits::{f32_exponent_limit, f64_exponent_limit, f64_mantissa_limit};
#[cfg(not(feature = "power-of-two"))]
use lexical_util::assert::debug_assert_radix;
use static_assertions::const_assert;

// HELPERS
// -------

/// Get lookup table for small int powers.
///
/// # Safety
///
/// Safe as long as the radix provided is valid, and exponent is smaller
/// than the table for the radix.
#[inline]
#[cfg(not(feature = "power-of-two"))]
pub unsafe fn get_small_int_power(exponent: usize, radix: u32) -> u64 {
    debug_assert_radix(radix);
    unsafe { get_small_int_power10(exponent) }
}

/// Get lookup table for small f32 powers.
///
/// # Safety
///
/// Safe as long as the radix provided is valid, and exponent is smaller
/// than the table for the radix.
#[inline]
#[cfg(not(feature = "power-of-two"))]
pub unsafe fn get_small_f32_power(exponent: usize, radix: u32) -> f32 {
    debug_assert_radix(radix);
    unsafe { get_small_f32_power10(exponent) }
}

/// Get lookup table for small f64 powers.
///
/// # Safety
///
/// Safe as long as the radix provided is valid, and exponent is smaller
/// than the table for the radix.
#[inline]
#[cfg(not(feature = "power-of-two"))]
pub unsafe fn get_small_f64_power(exponent: usize, radix: u32) -> f64 {
    debug_assert_radix(radix);
    unsafe { get_small_f64_power10(exponent) }
}

/// Get pre-computed int power of 10.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW10.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power10(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW10[exponent]) }
}

/// Get pre-computed f32 power of 10.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW10.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power10(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW10[exponent]) }
}

/// Get pre-computed f64 power of 10.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW10.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power10(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW10[exponent]) }
}

// TABLES
// ------

/// Pre-computed, small powers-of-10.
pub const SMALL_INT_POW10: [u64; 16] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
];
const_assert!(SMALL_INT_POW10.len() > f64_mantissa_limit(10) as usize);

/// Pre-computed, small powers-of-10.
pub const SMALL_F32_POW10: [f32; 16] =
    [1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 0., 0., 0., 0., 0.];
const_assert!(SMALL_F32_POW10.len() > f32_exponent_limit(10).1 as usize);

/// Pre-computed, small powers-of-10.
pub const SMALL_F64_POW10: [f64; 32] = [
    1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
    1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 0., 0., 0., 0., 0., 0., 0., 0., 0.,
];
const_assert!(SMALL_F64_POW10.len() > f64_exponent_limit(10).1 as usize);
