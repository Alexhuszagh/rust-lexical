//! Specialized iterator traits.
//!
//! The traits are iterable, and provide optimizations for contiguous
//! iterators, while still working for non-contiguous data.

#![cfg(feature = "parse")]

use crate::lib::{mem, ptr};

/// Context-aware, iterator-like trait for raw bytes.
///
/// This provides methods to convert to and from slices, as well
pub trait Byte<'a>: Clone {
    /// Determine if each yielded value is adjacent in memory.
    const IS_CONTIGUOUS: bool;

    /// Type for an iterator over integer digits.
    type IntegerIter: Iterator<Item = &'a u8>;

    /// Type for an iterator over fraction digits.
    type FractionIter: Iterator<Item = &'a u8>;

    /// Type for an iterator over exponent digits.
    type ExponentIter: Iterator<Item = &'a u8>;

    /// Type for an iterator over special floating point values.
    type SpecialIter: Iterator<Item = &'a u8>;

    /// Create new object from slice.
    fn new(slc: &'a [u8]) -> Self;

    /// Get a ptr to the current start of the iterator.
    fn as_ptr(&self) -> *const u8;

    /// Get a slice to the current start of the iterator.
    fn as_slice(&self) -> &'a [u8];

    /// Get the total number of elements in the underlying slice.
    fn length(&self) -> usize;

    /// Get the current index of the iterator in the slice.
    fn cursor(&self) -> usize;

    /// Get if the buffer underlying the iterator is empty.
    ///
    /// This might not be the same thing as `is_consumed`: `is_consumed`
    /// checks if any more elements may be returned, which may require
    /// peeking the next value. Consumed merely checks if the
    /// iterator has an empty slice. It is effectively a cheaper,
    /// but weaker variant of `is_consumed()`.
    fn is_empty(&self) -> bool;

    // Determine if the abstraction is contiguous.
    #[inline]
    fn is_contiguous(&self) -> bool {
        Self::IS_CONTIGUOUS
    }

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

    /// Get iterator over integer digits.
    fn integer_iter(&'a mut self) -> Self::IntegerIter;

    /// Get iterator over fraction digits.
    fn fraction_iter(&'a mut self) -> Self::FractionIter;

    /// Get iterator over exponent digits.
    fn exponent_iter(&'a mut self) -> Self::ExponentIter;

    /// Get iterator over special floating point values.
    fn special_iter(&'a mut self) -> Self::SpecialIter;
}

/// Iterator over a contiguous block of bytes.
///
/// This allows us to convert to-and-from-slices, raw pointers, and
/// peek/query the data from either end cheaply.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return `null` from `as_ptr`, or be
/// implemented for non-contiguous data.
pub trait ByteIter<'a>: Iterator<Item = &'a u8> {
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

    // Determine if the iterator is contiguous.
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
