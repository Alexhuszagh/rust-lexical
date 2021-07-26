//! Specialized iterator traits.
//!
//! The traits are iterable, and provide optimizations for contiguous
//! iterators, while still working for non-contiguous data.

#![cfg(feature = "parse")]

use crate::lib::{mem, ptr, slice};

/// Iterator over a contiguous block of bytes.
///
/// This allows us to convert to-and-from-slices, raw pointers, and
/// peek/query the data from either end cheaply.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return `null` from `as_ptr`, or be
/// implemented for non-contiguous data.
pub trait ByteIter<'a>: Iterator<Item = &'a u8> + Clone {
    /// Determine if each yielded value is adjacent in memory.
    const IS_CONTIGUOUS: bool;

    /// Create new iterator from slice.
    fn new(slc: &'a [u8]) -> Self;

    /// Get a ptr to the current start of the iterator.
    fn as_ptr(&self) -> *const u8;

    /// Get a slice to the current start of the iterator.
    fn as_slice(&self) -> &'a [u8];

    /// Get the number of elements left in the slice.
    #[inline]
    fn slice_len(&self) -> usize {
        self.as_slice().len()
    }

    /// Get if the iterator cannot return any more elements.
    ///
    /// This may advance the internal iterator state, but not
    /// modify the next returned value.
    fn is_consumed(&mut self) -> bool;

    /// Get if the buffer underlying the iterator is empty.
    ///
    /// This might not be the same thing as `is_consumed`: `is_consumed`
    /// checks if any more elements may be returned, which may require
    /// peeking the next value. Consumed merely checks if the
    /// iterator has an empty slice. It is effectively a cheaper,
    /// but weaker variant of `is_consumed()`.
    fn is_empty(&self) -> bool;

    /// Peek the next value of the iterator, without checking bounds.
    ///
    /// # Safety
    ///
    /// Safe as long as there is at least a single valid value left in
    /// the iterator. Note that the behavior of this may lead to out-of-bounds
    /// access (for contiguous iterators) or panics (for non-contiguous
    /// iterators).
    unsafe fn peek_unchecked(&mut self) -> Self::Item;

    /// Peek the next value of the iterator, without consuming it.
    fn peek(&mut self) -> Option<Self::Item>;

    /// Read a value of a difference type from the iterator.
    /// This advances the internal state of the iterator.
    ///
    /// # Safety
    ///
    /// Safe as long as the number of the buffer is contains as least as
    /// many bytes as the size of V.
    #[inline]
    unsafe fn read_unchecked<V>(&self) -> V {
        debug_assert!(Self::IS_CONTIGUOUS);

        let slc = self.as_slice();
        // SAFETY: safe as long as the slice has at least count elements.
        unsafe { ptr::read_unaligned::<V>(slc.as_ptr() as *const _) }
    }

    /// Try to read a value of a different type from the iterator.
    /// This advances the internal state of the iterator.
    #[inline]
    fn read<V>(&self) -> Option<V> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<V>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read.
            unsafe { Some(self.read_unchecked()) }
        } else {
            None
        }
    }

    /// Advance the internal slice by `N` elements.
    ///
    /// # Safety
    ///
    /// As long as the iterator is at least `N` elements, this
    /// is safe.
    #[inline]
    unsafe fn step_by_unchecked(&mut self, count: usize) {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.slice_len() >= count);
        let rest = unsafe { self.as_slice().get_unchecked(count..) };
        *self = Self::new(rest);
    }

    /// Advance the internal slice by 1 element.
    ///
    /// # Safety
    ///
    /// Safe as long as the iterator is not empty.
    #[inline]
    unsafe fn step_unchecked(&mut self) {
        unsafe { self.step_by_unchecked(1) };
    }
}

impl<'a> ByteIter<'a> for slice::Iter<'a, u8> {
    const IS_CONTIGUOUS: bool = true;

    #[inline]
    fn new(slc: &'a [u8]) -> Self {
        slc.iter()
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }

    #[inline]
    fn as_slice(&self) -> &'a [u8] {
        slice::Iter::as_slice(self)
    }

    #[inline]
    fn is_consumed(&mut self) -> bool {
        self.as_slice().is_empty()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    #[inline]
    unsafe fn peek_unchecked(&mut self) -> Self::Item {
        let slc = self.as_slice();
        // SAFETY: safe as long as the slice is not empty.
        unsafe { slc.get_unchecked(0) }
    }

    #[inline]
    fn peek(&mut self) -> Option<Self::Item> {
        if !self.is_consumed() {
            // SAFETY: the slice cannot be empty, so this is safe
            Some(unsafe { self.peek_unchecked() })
        } else {
            None
        }
    }
}
