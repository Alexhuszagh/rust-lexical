//! Test utilities.

use lib::Vec;
use util::*;

// BASES

/// Pow2 bases.
pub(crate) const BASE_POW2: [u32; 5] = [2, 4, 8, 16, 32];

/// Non-pow2 bases.
pub(crate) const BASE_POWN: [u32; 30] = [
    3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
];

#[cfg(target_pointer_width = "16")]
pub(crate) fn from_u32(x: &[u32]) -> Vec<u16> {
    let mut v = Vec::with_capacity(x.len() * 2);
    for xi in x.iter().cloned() {
        v.push(xi as u16);
        v.push((xi >> 16) as u16);
    }

    let len = v.len();
    if v.get_unchecked(len - 1) == 0 {
        v.pop();
    }

    v
}

#[cfg(target_pointer_width = "32")]
pub(crate) fn from_u32(x: &[u32]) -> Vec<u32> {
    x.iter().cloned().collect()
}

#[cfg(target_pointer_width = "64")]
pub(crate) fn from_u32(x: &[u32]) -> Vec<u64> {
    let mut v = Vec::with_capacity(x.len() / 2);
    for xi in x.chunks(2) {
        match xi.len() {
            1 => v.push(xi[0] as u64),
            2 => v.push(((xi[1] as u64) << 32) | (xi[0] as u64)),
            _ => unreachable!(),
        }
    }

    v
}

#[cfg(target_pointer_width = "16")]
pub(crate) fn deduce_from_u32<T: CloneableVecLike<u16>>(x: &[u32]) -> T
{
    from_u32(x).iter().cloned().collect()
}

#[cfg(target_pointer_width = "32")]
pub(crate) fn deduce_from_u32<T: CloneableVecLike<u32>>(x: &[u32]) -> T
{
    from_u32(x).iter().cloned().collect()
}

#[cfg(target_pointer_width = "64")]
pub(crate) fn deduce_from_u32<T: CloneableVecLike<u64>>(x: &[u32]) -> T
{
    from_u32(x).iter().cloned().collect()
}

// FLOATING-POINT EQUALITY

cfg_if! {
if #[cfg(all(feature = "table", not(feature = "imprecise")))] {
    /// Assert two 32-bit floats are equal.
    macro_rules! assert_f32_eq {
        ($l:expr, $r:expr $(, $opt:ident = $val:expr)+) => (assert_eq!($l, $r););
        ($l:expr, $r:expr) => (assert_eq!($l, $r););
    }

    /// Assert two 64-bit floats are equal.
    macro_rules! assert_f64_eq {
        ($l:expr, $r:expr $(, $opt:ident = $val:expr)+) => (assert_eq!($l, $r););
        ($l:expr, $r:expr) => (assert_eq!($l, $r););
    }
} else {
    /// Assert two 32-bit floats are equal.
    macro_rules! assert_f32_eq {
        ($l:expr, $r:expr $(, $opt:ident = $val:expr)+) => (assert_relative_eq!($l, $r $(, $opt = $val)*););
        ($l:expr, $r:expr) => (assert_relative_eq!($l, $r, epsilon=1e-20););
    }

    /// Assert two 64-bit floats are equal.
    macro_rules! assert_f64_eq {
        ($l:expr, $r:expr $(, $opt:ident = $val:expr)+) => (assert_relative_eq!($l, $r $(, $opt = $val)*););
        ($l:expr, $r:expr) => (assert_relative_eq!($l, $r, epsilon=1e-20, max_relative=1e-12););
    }
}}  // cfg_if
