//! Helper utilities for low-level features.
//!
//! Utilities for working with pointers and compiler intrinsics that
//! may not be available  in rust, or in a `no_std` context.

#[cfg(all(test, feature = "std"))]
use std::{f32, f64};

// CONSTANTS

/// Not a Number (NaN).
#[cfg(all(test, feature = "std"))]
pub(crate) const F32_NAN: f32 = f32::NAN;

/// Not a Number (NaN).
#[cfg(all(test, not(feature = "std")))]
pub(crate) const F32_NAN: f32 = 0.0_f32 / 0.0_f32;

/// Infinity (∞).
#[cfg(all(test, feature = "std"))]
pub(crate) const F32_INFINITY: f32 = f32::INFINITY;

/// Infinity (∞).
#[cfg(all(test, not(feature = "std")))]
pub(crate) const F32_INFINITY: f32 = 1.0_f32 / 0.0_f32;

/// Not a Number (NaN).
#[cfg(all(test, feature = "std"))]
pub(crate) const F64_NAN: f64 = f64::NAN;

/// Not a Number (NaN).
#[cfg(all(test, not(feature = "std")))]
pub(crate) const F64_NAN: f64 = 0.0_f64 / 0.0_f64;

/// Infinity (∞).
#[cfg(all(test, feature = "std"))]
pub(crate) const F64_INFINITY: f64 = f64::INFINITY;

/// Infinity (∞).
#[cfg(all(test, not(feature = "std")))]
pub(crate) const F64_INFINITY: f64 = 1.0_f64 / 0.0_f64;

// INSTRINSICS

/// `f64.floor()` feature for `no_std`
#[cfg(not(feature = "std"))]
#[inline(always)]
pub(crate) fn floor(f: f64) -> f64 {
    unsafe { core::intrinsics::floorf64(f) }
}

/// `f64.ln()` feature for `no_std`
#[cfg(not(feature = "std"))]
#[inline(always)]
pub(crate) fn ln(f: f64) -> f64 {
    unsafe { core::intrinsics::logf64(f) }
}

/// `f64.floor()` feature for `std`
#[cfg(feature = "std")]
#[inline(always)]
pub(crate) fn floor(f: f64) -> f64 {
    f.floor()
}

/// `f64.ln()` feature for `std`
#[cfg(feature = "std")]
#[inline(always)]
pub(crate) fn ln(f: f64) -> f64 {
    f.ln()
}

// ALGORITHMS

/// Reverse a range of pointers.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn reverse(first: *mut u8, last: *mut u8) {
    let mut f = first;
    let mut l = last;
    let mut x: u8;
    let mut li = l.sub(1);

    while f != l && f != li {
        l = li;
        x = *f;
        *f = *l;
        *l = x;
        li = l.sub(1);
        f = f.add(1);
    }
}

/// Calculate the difference between two pointers.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn distance(first: *const u8, last: *const u8)
    -> usize
{
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_test() {
        unsafe {
            let mut x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let y: [u8; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
            let first: *mut u8 = x.as_mut_ptr();
            let last = first.add(x.len());
            reverse(first, last);
            assert_eq!(x, y);
        }
    }

    #[test]
    fn distance_test() {
        unsafe {
            let x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let first: *const u8 = x.as_ptr();
            let last = first.add(x.len());
            assert_eq!(distance(first, last), 10);
        }
    }
}
