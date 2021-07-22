//! Simple, shared algorithms for slices and iterators.

use crate::lib::ptr;

/// Copy bytes from source to destination.
///
/// # Safety
///
/// Safe as long as `dst` is larger than `src`.
#[inline]
pub unsafe fn copy_to_dst<Bytes: AsRef<[u8]>>(dst: &mut [u8], src: Bytes) -> usize {
    debug_assert!(dst.len() >= src.as_ref().len());

    let src = src.as_ref();
    let dst = unsafe { dst.get_unchecked_mut(..src.len()) };

    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
    }

    src.len()
}
