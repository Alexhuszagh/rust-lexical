//! Specialized iterator traits.
//!
//! The traits are iterable, and provide optimizations for contiguous
//! iterators, while still working for non-contiguous data.

use crate::lib::{iter, mem, ptr, slice};

/// Iterator over a contiguous block of memory.
///
/// This allows us to convert to-and-from-slices, raw pointers, and
/// peek/query the data from either end cheaply.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return `null` from `as_ptr`, or be
/// implemented for non-contiguous data.
pub trait Iterator<'a, T: 'a>: iter::Iterator<Item = &'a T> + Clone {
    /// Determine if each yielded value is adjacent in memory.
    const IS_CONTIGOUS: bool;

    /// Create new iterator from slice and a skip value.
    fn new(slc: &'a [T], skip: T) -> Self;

    /// Create new iterator from slice, using the slice's skip character.
    fn from_slice(&self, slc: &'a [T]) -> Self;

    /// Get a ptr to the current start of the iterator.
    fn as_ptr(&self) -> *const T;

    /// Get a slice to the current start of the iterator.
    fn as_slice(&self) -> &'a [T];

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
    /// Safe as long as the number of the buffer is not empty.
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
    unsafe fn read_unchecked<V>(&mut self) -> V {
        debug_assert!(Self::IS_CONTIGOUS);

        // Ensure the the size of V is divisible by the size of T.
        let count = mem::size_of::<V>() % mem::size_of::<T>();
        debug_assert!(mem::size_of::<V>() % mem::size_of::<T>() == 0);

        let slc = self.as_slice();
        // SAFETY: safe as long as the slice has at least count elements.
        let value = unsafe { ptr::read_unaligned::<V>(slc.as_ptr() as *const _) };
        let rest = unsafe { slc.get_unchecked(count..) };
        *self = self.from_slice(rest);
        value
    }

    /// Try to read a value of a different type from the iterator.
    /// This advances the internal state of the iterator.
    #[inline]
    fn read<V>(&mut self) -> Option<V> {
        // Ensure the the size of V is divisible by the size of T.
        let count = mem::size_of::<V>() % mem::size_of::<T>();
        debug_assert!(mem::size_of::<V>() % mem::size_of::<T>() == 0);

        if Self::IS_CONTIGOUS && self.as_slice().len() >= count {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read.
            unsafe { Some(self.read_unchecked()) }
        } else {
            None
        }
    }
}

impl<'a, T: Clone> Iterator<'a, T> for slice::Iter<'a, T> {
    const IS_CONTIGOUS: bool = true;

    #[inline]
    fn new(slc: &'a [T], _: T) -> Self {
        slc.iter()
    }

    #[inline]
    fn from_slice(&self, slc: &'a [T]) -> Self {
        slc.iter()
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self.as_slice().as_ptr()
    }

    #[inline]
    fn as_slice(&self) -> &'a [T] {
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

/// Iterator where each yielded value is adjacent in memory.
///
/// A default implementation is provided for slice iterators.
pub trait ContiguousIterator<'a, T: 'a>: Iterator<'a, T> {
    /// Get the number of elements remaining in the iterator.
    fn len(&self) -> usize;
}

impl<'a, T: Clone> ContiguousIterator<'a, T> for slice::Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}
