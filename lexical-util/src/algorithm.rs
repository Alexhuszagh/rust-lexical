//! Simple, shared algorithms for slices and iterators.

#[cfg(feature = "write")]
use core::ptr;

/// Copy bytes from source to destination.
///
/// # Safety
///
/// Safe as long as `dst` is larger than `src`.
#[inline]
#[cfg(feature = "write")]
pub unsafe fn copy_to_dst<Bytes: AsRef<[u8]>>(dst: &mut [u8], src: Bytes) -> usize {
    debug_assert!(dst.len() >= src.as_ref().len());

    // SAFETY: safe, if `dst.len() <= src.len()`.
    let src = src.as_ref();
    unsafe { ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len()) };

    src.len()
}

/// Count the number of trailing characters equal to a given value.
#[inline]
#[cfg(feature = "write")]
pub fn rtrim_char_count(slc: &[u8], c: u8) -> usize {
    slc.iter().rev().take_while(|&&si| si == c).count()
}

/// Count the number of leading characters equal to a given value.
#[inline]
#[cfg(feature = "write")]
pub fn ltrim_char_count(slc: &[u8], c: u8) -> usize {
    slc.iter().take_while(|&&si| si == c).count()
}

/// Trim character from the end (right-side) of a slice.
#[inline]
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
