//! Test utilities.

use crate::config::BUFFER_SIZE;
use crate::traits::CloneableVecLike;
#[cfg(limb_width_64)]
use crate::traits::VecLike;

use super::limb::Limb;

cfg_if! {
if #[cfg(feature = "no_alloc")] {
    use arrayvec;
} else {
    use crate::lib::Vec;
}} // cfg_if

// BASES

/// Pow2 bases.
#[cfg(feature = "radix")]
pub(crate) const BASE_POW2: [u32; 5] = [2, 4, 8, 16, 32];

/// Non-pow2 bases.
#[cfg(feature = "radix")]
pub(crate) const BASE_POWN: [u32; 30] = [
    3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 33, 34, 35, 36,
];

#[cfg(not(feature = "radix"))]
pub(crate) const BASE_POWN: [u32; 1] = [10];

// BUFFER

/// Create new buffer for itoa or ftoa functionality.
#[inline]
pub(crate) fn new_buffer() -> [u8; BUFFER_SIZE] {
    [b'\0'; BUFFER_SIZE]
}

// BYTE SLICE

/// Use to help type deduction.
#[inline]
pub(crate) fn as_slice<'a, T>(x: &'a [T]) -> &'a [T] {
    x
}

// FROM U32

cfg_if! {
if #[cfg(not(feature = "no_alloc"))] {
    pub(crate) type DataType = Vec<Limb>;
} else if #[cfg(limb_width_64)] {
    pub(crate) type DataType = arrayvec::ArrayVec<[Limb; 64]>;
} else {
    pub(crate) type DataType = arrayvec::ArrayVec<[Limb; 128]>;
}} // cfg_if

#[cfg(limb_width_32)]
pub(crate) fn from_u32(x: &[u32]) -> DataType {
    x.iter().cloned().collect()
}

#[cfg(limb_width_64)]
pub(crate) fn from_u32(x: &[u32]) -> DataType {
    let mut v = DataType::default();
    v.reserve(x.len() / 2);
    for xi in x.chunks(2) {
        match xi.len() {
            1 => v.push(xi[0] as Limb),
            2 => v.push(((xi[1] as Limb) << 32) | (xi[0] as Limb)),
            _ => unreachable!(),
        }
    }

    v
}

#[cfg(limb_width_32)]
pub(crate) fn deduce_from_u32<T: CloneableVecLike<u32>>(x: &[u32]) -> T {
    from_u32(x).iter().cloned().collect()
}

#[cfg(limb_width_64)]
pub(crate) fn deduce_from_u32<T: CloneableVecLike<u64>>(x: &[u32]) -> T {
    from_u32(x).iter().cloned().collect()
}

// LITERAL BYTE SLICES

/// Create a literal byte slice.
macro_rules! b {
    ($l:expr) => {
        $l.as_bytes()
    };
}

// FLOATING-POINT EQUALITY

/// Assert two 32-bit floats are equal.
macro_rules! assert_f32_eq {
    ($l:expr, $r:expr $(, $opt:ident = $val:expr)+) => {
        assert_eq!($l, $r);
    };
    ($l:expr, $r:expr) => {
        assert_eq!($l, $r);
    };
}

/// Assert two 64-bit floats are equal.
macro_rules! assert_f64_eq {
    ($l:expr, $r:expr $(, $opt:ident = $val:expr)+) => {
        assert_eq!($l, $r);
    };
    ($l:expr, $r:expr) => {
        assert_eq!($l, $r);
    };
}

/// Assert two 32-bit floats are equal.
macro_rules! assert_f32_near_eq {
    ($l:expr, $r:expr $(, $opt:ident = $val:expr)+) => (approx::assert_relative_eq!($l, $r $(, $opt = $val)*););
    ($l:expr, $r:expr) => (approx::assert_relative_eq!($l, $r, epsilon=1e-20););
}

/// Assert two 64-bit floats are equal.
macro_rules! assert_f64_near_eq {
    ($l:expr, $r:expr $(, $opt:ident = $val:expr)+) => (approx::assert_relative_eq!($l, $r $(, $opt = $val)*););
    ($l:expr, $r:expr) => (approx::assert_relative_eq!($l, $r, epsilon=1e-20, max_relative=1e-12););
}
