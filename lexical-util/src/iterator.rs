//! Specialized iterator traits.
//!
//! The traits are iterable, and provide optimizations for contiguous
//! iterators, while still working for non-contiguous data.

#![cfg(feature = "parse")]

// Re-export our digit iterators.
#[cfg(not(feature = "format"))]
pub use crate::noskip::{AsBytes, Bytes};

#[cfg(feature = "format")]
pub use crate::skip::{AsBytes, Bytes};

/// Iterator over a contiguous block of bytes.
///
/// This allows us to convert to-and-from-slices, raw pointers, and
/// peek/query the data from either end cheaply.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return `null` from `as_ptr`, or be
/// implemented for non-contiguous data.
pub trait BytesIter<'a>: Iterator<Item = &'a u8> {
    /// Determine if each yielded value is adjacent in memory.
    const IS_CONTIGUOUS: bool;

    /// Get a ptr to the current start of the iterator.
    fn as_ptr(&self) -> *const u8;

    /// Get a slice to the current start of the iterator.
    fn as_slice(&self) -> &'a [u8];

    /// Get the total number of elements in the underlying slice.
    fn length(&self) -> usize;

    /// Get the current index of the iterator in the slice.
    fn cursor(&self) -> usize;

    /// Set the current index of the iterator in the slice.
    ///
    /// # Safety
    ///
    /// Safe if `index <= self.length()`.
    unsafe fn set_cursor(&mut self, index: usize);

    /// Get the current number of values returned by the iterator.
    fn current_count(&self) -> usize;

    /// Get if the iterator cannot return any more elements.
    ///
    /// This may advance the internal iterator state, but not
    /// modify the next returned value.
    #[allow(clippy::wrong_self_convention)]
    fn is_consumed(&mut self) -> bool;

    /// Get if the buffer underlying the iterator is empty.
    ///
    /// This might not be the same thing as `is_consumed`: `is_consumed`
    /// checks if any more elements may be returned, which may require
    /// peeking the next value. Consumed merely checks if the
    /// iterator has an empty slice. It is effectively a cheaper,
    /// but weaker variant of `is_consumed()`.
    fn is_done(&self) -> bool;

    /// Determine if the iterator is contiguous.
    #[inline]
    fn is_contiguous(&self) -> bool {
        Self::IS_CONTIGUOUS
    }

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

    /// Check if the next element is a given value.
    #[inline]
    fn peek_is(&mut self, value: u8) -> bool {
        if let Some(&c) = self.peek() {
            c == value
        } else {
            false
        }
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline]
    fn case_insensitive_peek_is(&mut self, value: u8) -> bool {
        if let Some(&c) = self.peek() {
            c.to_ascii_lowercase() == value.to_ascii_lowercase()
        } else {
            false
        }
    }

    /// Skip zeros from the start of the iterator
    #[inline]
    fn skip_zeros(&mut self) -> usize {
        let start = self.cursor();
        while let Some(&b'0') = self.peek() {
            self.next();
        }
        self.cursor() - start
    }

    /// Read a value of a difference type from the iterator.
    /// This advances the internal state of the iterator.
    ///
    /// # Safety
    ///
    /// Safe as long as the number of the buffer is contains as least as
    /// many bytes as the size of V.
    unsafe fn read_unchecked<V>(&self) -> V;

    /// Try to read a value of a different type from the iterator.
    /// This advances the internal state of the iterator.
    fn read<V>(&self) -> Option<V>;

    /// Advance the internal slice by `N` elements.
    ///
    /// # Safety
    ///
    /// As long as the iterator is at least `N` elements, this
    /// is safe.
    unsafe fn step_by_unchecked(&mut self, count: usize);

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
