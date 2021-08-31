//! An iterator over a slice.
//!
//! This iterator has both the length of the original slice, as
//! well as the current position of the iterator in the buffer.

#![cfg(all(feature = "parse", not(feature = "format")))]

use crate::iterator::BytesIter;
use core::{mem, ptr};

// AS DIGITS
// ---------

/// Trait to simplify creation of a `Bytes` object.
pub trait AsBytes<'a> {
    /// Create `Bytes` from object.
    fn bytes<const __: u128>(&'a self) -> Bytes<'a, __>;
}

impl<'a> AsBytes<'a> for [u8] {
    #[inline]
    fn bytes<const __: u128>(&'a self) -> Bytes<'a, __> {
        Bytes::new(self)
    }
}

// DIGITS
// ------

/// Slice iterator that stores the original length of the slice.
#[derive(Clone)]
pub struct Bytes<'a, const __: u128> {
    /// The raw slice for the iterator.
    slc: &'a [u8],
    /// Current index of the iterator in the slice.
    index: usize,
}

impl<'a, const __: u128> Bytes<'a, __> {
    /// If each yielded value is adjacent in memory.
    pub const IS_CONTIGUOUS: bool = true;

    /// Create new byte object.
    #[inline]
    pub const fn new(slc: &'a [u8]) -> Self {
        Self {
            slc,
            index: 0,
        }
    }

    /// Get a ptr to the current start of the iterator.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }

    /// Get a slice to the current start of the iterator.
    #[inline]
    pub fn as_slice(&self) -> &'a [u8] {
        debug_assert!(self.index <= self.length());
        // SAFETY: safe since index must be in range.
        unsafe { self.slc.get_unchecked(self.index..) }
    }

    /// Get the total number of elements in the underlying slice.
    #[inline]
    pub fn length(&self) -> usize {
        self.slc.len()
    }

    /// Get the current index of the iterator in the slice.
    #[inline]
    pub fn cursor(&self) -> usize {
        self.index
    }

    /// Set the current index of the iterator in the slice.
    ///
    /// # Safety
    ///
    /// Safe if `index <= self.length()`.
    #[inline]
    pub unsafe fn set_cursor(&mut self, index: usize) {
        debug_assert!(index <= self.length());
        self.index = index
    }

    /// Get the current number of values returned by the iterator.
    #[inline]
    pub fn current_count(&self) -> usize {
        self.index
    }

    /// Get if the buffer underlying the iterator is empty.
    /// Same as `is_consumed`.
    #[inline]
    pub fn is_done(&self) -> bool {
        self.index >= self.slc.len()
    }

    /// Read a value of a difference type from the iterator.
    /// This advances the internal state of the iterator.
    ///
    /// # Safety
    ///
    /// Safe as long as the number of the buffer is contains as least as
    /// many bytes as the size of V.
    #[inline]
    #[allow(clippy::assertions_on_constants)]
    pub unsafe fn read_unchecked<V>(&self) -> V {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= mem::size_of::<V>());

        let slc = self.as_slice();
        // SAFETY: safe as long as the slice has at least count elements.
        unsafe { ptr::read_unaligned::<V>(slc.as_ptr() as *const _) }
    }

    /// Try to read a value of a different type from the iterator.
    /// This advances the internal state of the iterator.
    #[inline]
    pub fn read<V>(&self) -> Option<V> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<V>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read.
            unsafe { Some(self.read_unchecked()) }
        } else {
            None
        }
    }

    /// Check if the next element is a given value.
    #[inline]
    pub fn first_is(&mut self, value: u8) -> bool {
        if let Some(&c) = self.slc.get(self.index) {
            c == value
        } else {
            false
        }
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline]
    pub fn case_insensitive_first_is(&mut self, value: u8) -> bool {
        if let Some(&c) = self.slc.get(self.index) {
            c.to_ascii_lowercase() == value.to_ascii_lowercase()
        } else {
            false
        }
    }

    /// Get iterator over integer digits.
    #[inline]
    pub fn integer_iter<'b>(&'b mut self) -> BytesIterator<'a, 'b, __> {
        BytesIterator {
            byte: self,
        }
    }

    /// Get iterator over fraction digits.
    #[inline]
    pub fn fraction_iter<'b>(&'b mut self) -> BytesIterator<'a, 'b, __> {
        BytesIterator {
            byte: self,
        }
    }

    /// Get iterator over exponent digits.
    #[inline]
    pub fn exponent_iter<'b>(&'b mut self) -> BytesIterator<'a, 'b, __> {
        BytesIterator {
            byte: self,
        }
    }

    /// Get iterator over special floating point values.
    #[inline]
    pub fn special_iter<'b>(&'b mut self) -> BytesIterator<'a, 'b, __> {
        BytesIterator {
            byte: self,
        }
    }

    /// Advance the byte by `N` elements.
    ///
    /// # Safety
    ///
    /// As long as the iterator is at least `N` elements, this
    /// is safe.
    #[inline]
    #[allow(clippy::assertions_on_constants)]
    pub unsafe fn step_by_unchecked(&mut self, count: usize) {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= count);
        self.index += count;
    }

    /// Advance the byte by 1 element.
    ///
    /// # Safety
    ///
    /// Safe as long as the iterator is not empty.
    #[inline]
    #[allow(clippy::assertions_on_constants)]
    pub unsafe fn step_unchecked(&mut self) {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(!self.as_slice().is_empty());
        self.index += 1;
    }
}

// DIGITS ITERATOR
// ---------------

/// Slice iterator that stores the original length of the slice.
pub struct BytesIterator<'a: 'b, 'b, const __: u128> {
    /// The internal byte object for the noskip iterator.
    byte: &'b mut Bytes<'a, __>,
}

impl<'a: 'b, 'b, const __: u128> BytesIter<'a> for BytesIterator<'a, 'b, __> {
    const IS_CONTIGUOUS: bool = Bytes::<'a, __>::IS_CONTIGUOUS;

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.byte.as_ptr()
    }

    #[inline]
    fn as_slice(&self) -> &'a [u8] {
        self.byte.as_slice()
    }

    #[inline]
    fn length(&self) -> usize {
        self.byte.length()
    }

    #[inline]
    fn cursor(&self) -> usize {
        self.byte.cursor()
    }

    #[inline]
    unsafe fn set_cursor(&mut self, index: usize) {
        debug_assert!(index <= self.length());
        // SAFETY: safe if `index <= self.length()`.
        unsafe { self.byte.set_cursor(index) };
    }

    #[inline]
    fn current_count(&self) -> usize {
        self.byte.current_count()
    }

    #[inline]
    fn is_consumed(&mut self) -> bool {
        Self::is_done(self)
    }

    #[inline]
    fn is_done(&self) -> bool {
        self.byte.is_done()
    }

    #[inline]
    unsafe fn peek_unchecked(&mut self) -> <Self as Iterator>::Item {
        // SAFETY: safe if `self.cursor() < self.length()`.
        unsafe { self.byte.slc.get_unchecked(self.byte.index) }
    }

    #[inline]
    fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.byte.index < self.byte.slc.len() {
            // SAFETY: the slice cannot be empty, so this is safe
            Some(unsafe { self.peek_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    unsafe fn read_unchecked<V>(&self) -> V {
        debug_assert!(self.as_slice().len() >= mem::size_of::<V>());
        // SAFETY: safe as long as the slice has at least count elements.
        unsafe { self.byte.read_unchecked() }
    }

    #[inline]
    fn read<V>(&self) -> Option<V> {
        self.byte.read()
    }

    #[inline]
    unsafe fn step_by_unchecked(&mut self, count: usize) {
        debug_assert!(self.as_slice().len() >= count);
        // SAFETY: safe as long as `slc.len() >= count`.
        unsafe { self.byte.step_by_unchecked(count) }
    }

    #[inline]
    unsafe fn step_unchecked(&mut self) {
        debug_assert!(!self.as_slice().is_empty());
        // SAFETY: safe as long as `slc.len() >= 1`.
        unsafe { self.byte.step_unchecked() }
    }
}

impl<'a: 'b, 'b, const __: u128> Iterator for BytesIterator<'a, 'b, __> {
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.byte.slc.get(self.byte.index)?;
        self.byte.index += 1;
        Some(value)
    }
}

impl<'a: 'b, 'b, const __: u128> ExactSizeIterator for BytesIterator<'a, 'b, __> {
    #[inline]
    fn len(&self) -> usize {
        self.length() - self.cursor()
    }
}
