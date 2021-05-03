//! Polyfill of slice::fill to versions <= 1.50.0.

#![cfg(feature = "binary")]

/// Has slice fill.
#[inline(always)]
#[cfg(has_slice_fill)]
pub(crate) fn slice_fill<T>(slice: &mut [T], value: T)
where
    T: Clone
{
    slice.fill(value)
}

/// Does not have slice fill, polyfill.
#[inline(always)]
#[cfg(not(has_slice_fill))]
pub(crate) fn slice_fill<T>(slice: &mut [T], value: T)
where
    T: Clone
{
    for elem in slice {
        elem.clone_from(&value)
    }
}
