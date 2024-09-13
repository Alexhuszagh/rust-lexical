//! Specialized iterator traits.
//!
//! The traits are iterable, and provide optimizations for contiguous
//! iterators, while still working for non-contiguous data.

#![cfg(feature = "parse")]

pub use crate::buffer::Buffer;

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
///
/// # Safety
///
/// The safe methods are sound as long as the caller ensures that
/// the methods for `read_32`, `read_64`, etc. check the bounds
/// of the underlying contiguous buffer and is only called on
/// contiguous buffers.
pub unsafe trait BytesIter<'a>: Iterator<Item = &'a u8> + Buffer<'a> {
    /// Get the total number of elements in the underlying slice.
    fn length(&self) -> usize;

    /// Get the current index of the iterator in the slice.
    fn cursor(&self) -> usize;

    /// Set the current index of the iterator in the slice.
    ///
    /// This is **NOT** the current position of the iterator,
    /// since iterators may skip digits: this is the cursor
    /// in the underlying buffer. For example, if `slc[2]` is
    /// skipped, `set_cursor(3)` would be the 3rd element in
    /// the iterator, not the 4th.
    ///
    /// # Safety
    ///
    /// Safe if `index <= self.length()`. Although this won't
    /// affect safety, the caller also should be careful it
    /// does not set the cursor within skipped characters.
    unsafe fn set_cursor(&mut self, index: usize);

    /// Set the cursor to the start of the buffer.
    #[inline(always)]
    fn seek_start(&mut self) {
        // SAFETY: 0 is alwatys <= any usize value.
        unsafe { self.set_cursor(0) };
    }

    /// Get a slice to the full buffer, which may or may not be the same as `as_slice`.
    fn as_full_slice(&self) -> &'a [u8];

    /// Get the current number of values returned by the iterator.
    fn current_count(&self) -> usize;

    /// Get if the iterator cannot return any more elements.
    ///
    /// This may advance the internal iterator state, but not
    /// modify the next returned value.
    ///
    /// If this is an iterator, this is based on the number of items
    /// left to be returned. We do not necessarly know the length of
    /// the buffer. If this is a non-contiguous iterator, this **MUST**
    /// advance the state until it knows a value can be returned.
    ///
    /// Any incorrect implementations of this affect all safety invariants
    /// for the rest of the trait. For contiguous iterators, this can be
    /// as simple as checking if `self.cursor >= self.slc.len()`, but for
    /// non-contiguous iterators you **MUST** advance to the next element
    /// to be returned, then check to see if a value exists. The safest
    /// implementation is always to check if `self.peek().is_none()` and
    /// ensure [peek] is always safe.
    ///
    /// If you would like to see if the cursor is at the end of the buffer,
    /// see [is_done] or [is_empty] instead.
    ///
    /// [is_done]: BytesIter::is_done
    /// [is_empty]: Buffer::is_empty
    /// [peek]: BytesIter::peek
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    fn is_consumed(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Get if the buffer underlying the iterator is empty.
    ///
    /// This might not be the same thing as [is_consumed]: [is_consumed]
    /// checks if any more elements may be returned, which may require
    /// peeking the next value. Consumed merely checks if the
    /// iterator has an empty slice. It is effectively a cheaper,
    /// but weaker variant of [is_consumed].
    ///
    /// [is_consumed]: BytesIter::is_consumed
    fn is_done(&self) -> bool;

    /// Peek the next value of the iterator, without checking bounds.
    ///
    /// Note that this can modify the internal state, by skipping digits
    /// for iterators that find the first non-zero value, etc.
    ///
    /// # Safety
    ///
    /// Safe as long as there is at least a single valid value left in
    /// the iterator. Note that the behavior of this may lead to out-of-bounds
    /// access (for contiguous iterators) or panics (for non-contiguous
    /// iterators).
    unsafe fn peek_unchecked(&mut self) -> Self::Item;

    /// Peek the next value of the iterator, without consuming it.
    ///
    /// Note that this can modify the internal state, by skipping digits
    /// for iterators that find the first non-zero value, etc.
    #[inline(always)]
    fn peek(&mut self) -> Option<Self::Item> {
        if !self.is_empty() {
            // SAFETY: safe since the buffer cannot be empty
            unsafe { Some(self.peek_unchecked()) }
        } else {
            None
        }
    }

    /// Peek the next value of the iterator, and step only if it exists.
    #[inline(always)]
    fn try_read(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.peek() {
            // SAFETY: the slice cannot be empty because we peeked a value.
            unsafe { self.step_unchecked() };
            Some(value)
        } else {
            None
        }
    }

    /// Check if the next element is a given value.
    #[inline(always)]
    fn peek_is(&mut self, value: u8) -> bool {
        if let Some(&c) = self.peek() {
            c == value
        } else {
            false
        }
    }

    /// Peek the next value and consume it if the read value matches the expected one.
    #[inline(always)]
    fn read_if<Pred: FnOnce(&u8) -> bool>(&mut self, pred: Pred) -> Option<Self::Item> {
        if let Some(peeked) = self.peek() {
            if pred(peeked) {
                // SAFETY: the slice cannot be empty because we peeked a value.
                unsafe { self.step_unchecked() };
                Some(peeked)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline(always)]
    fn case_insensitive_peek_is(&mut self, value: u8) -> bool {
        if let Some(&c) = self.peek() {
            c.to_ascii_lowercase() == value.to_ascii_lowercase()
        } else {
            false
        }
    }

    /// Skip zeros from the start of the iterator
    #[inline(always)]
    fn skip_zeros(&mut self) -> usize {
        let start = self.cursor();
        while let Some(&b'0') = self.peek() {
            self.next();
        }
        self.cursor() - start
    }

    /// Read a value of a difference type from the iterator.
    ///
    /// This advances the internal state of the iterator. This
    /// can only be implemented for contiguous iterators: non-
    /// contiguous iterators **MUST** panic.
    ///
    /// # Safety
    ///
    /// Safe as long as the number of the buffer is contains as least as
    /// many bytes as the size of V. This must be unimplemented for
    /// non-contiguous iterators.
    #[inline(always)]
    unsafe fn read_unchecked<V>(&self) -> V {
        unimplemented!();
    }

    /// Try to read a the next four bytes as a u32.
    /// This advances the internal state of the iterator.
    fn read_u32(&self) -> Option<u32>;

    /// Try to read the next eight bytes as a u64
    /// This advances the internal state of the iterator.
    fn read_u64(&self) -> Option<u64>;

    /// Advance the internal slice by `N` elements.
    ///
    /// This does not advance the iterator by `N` elements for
    /// non-contiguous iterators: this just advances the internal,
    /// underlying buffer. This is useful for multi-digit optimizations
    /// for contiguous iterators.
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
    #[inline(always)]
    unsafe fn step_unchecked(&mut self) {
        unsafe { self.step_by_unchecked(1) };
    }
}
