//! Simple, shared algorithms for slices and iterators.

use crate::num::Integer;
#[cfg(feature = "write")]
use core::ptr;

/// Copy bytes from source to destination.
#[inline(always)]
#[cfg(feature = "write")]
pub fn copy_to_dst<Bytes: AsRef<[u8]>>(dst: &mut [u8], src: Bytes) -> usize {
    assert!(dst.len() >= src.as_ref().len());

    // SAFETY: safe, if `dst.len() <= src.len()`.
    let src = src.as_ref();
    unsafe { ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len()) };

    src.len()
}

/// Count the number of trailing characters equal to a given value.
#[inline(always)]
#[cfg(feature = "write")]
pub fn rtrim_char_count(slc: &[u8], c: u8) -> usize {
    slc.iter().rev().take_while(|&&si| si == c).count()
}

/// Count the number of leading characters equal to a given value.
#[inline(always)]
#[cfg(feature = "write")]
pub fn ltrim_char_count(slc: &[u8], c: u8) -> usize {
    slc.iter().take_while(|&&si| si == c).count()
}

/// Trim character from the end (right-side) of a slice.
#[inline(always)]
#[cfg(feature = "write")]
pub fn rtrim_char_slice(slc: &[u8], c: u8) -> (&[u8], usize) {
    let count = rtrim_char_count(slc, c);
    let index = slc.len() - count;
    // Count must be <= slc.len(), and therefore, slc.len() - count must
    // also be <= slc.len(), since this is derived from an iterator
    // in the standard library.
    debug_assert!(count <= slc.len());
    debug_assert!(index <= slc.len());
    // SAFETY: safe since `count <= slc.len()` and therefore `index <= slc.len()`.
    let slc = unsafe { slc.get_unchecked(..index) };
    (slc, count)
}

/// Check to see if parsing the float cannot possible overflow.
///
/// This allows major optimizations for those types, since we can skip checked
/// arithmetic.
///
/// Adapted from the rust corelib:
///     https://doc.rust-lang.org/1.81.0/src/core/num/mod.rs.html#1389
#[inline(always)]
pub fn cannot_overflow<T: Integer>(length: usize, radix: u32) -> bool {
    length <= T::overflow_digits(radix)
}
