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
    #[inline(always)]
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
    #[inline(always)]
    pub const fn new(slc: &'a [u8]) -> Self {
        Self {
            slc,
            index: 0,
        }
    }

    /// Get a ptr to the current start of the iterator.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }

    /// Get a slice to the current start of the iterator.
    #[inline(always)]
    pub fn as_slice(&self) -> &'a [u8] {
        debug_assert!(self.index <= self.length());
        // SAFETY: safe since index must be in range.
        unsafe { self.slc.get_unchecked(self.index..) }
    }

    /// Get the total number of elements in the underlying slice.
    #[inline(always)]
    pub fn length(&self) -> usize {
        self.slc.len()
    }

    /// Get the current index of the iterator in the slice.
    #[inline(always)]
    pub fn cursor(&self) -> usize {
        self.index
    }

    /// Set the current index of the iterator in the slice.
    ///
    /// # Safety
    ///
    /// Safe if `index <= self.length()`.
    #[inline(always)]
    pub unsafe fn set_cursor(&mut self, index: usize) {
        debug_assert!(index <= self.length());
        self.index = index
    }

    /// Get the current number of values returned by the iterator.
    #[inline(always)]
    pub fn current_count(&self) -> usize {
        self.index
    }

    /// Get if the buffer underlying the iterator is empty.
    /// Same as `is_consumed`.
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        self.index >= self.slc.len()
    }

    /// Read a value of a difference type from the iterator.
    /// This advances the internal state of the iterator.
    ///
    /// # Safety
    ///
    /// Safe as long as the number of the buffer is contains as least as
    /// many bytes as the size of V, and V is valid for all bit patterns.
    #[inline(always)]
    #[allow(clippy::assertions_on_constants)]
    pub unsafe fn read_unchecked<V>(&self) -> V {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= mem::size_of::<V>());

        let slc = self.as_slice();
        // SAFETY: safe as long as the slice has at least count elements.
        unsafe { ptr::read_unaligned::<V>(slc.as_ptr() as *const _) }
    }

    /// Try to read a the next four bytes as a u32.
    /// This advances the internal state of the iterator.
    #[inline(always)]
    pub fn read_u32(&self) -> Option<u32> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<u32>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read. u32 is valid for all bit patterns
            unsafe { Some(self.read_unchecked()) }
        } else {
            None
        }
    }

    /// Try to read the next eight bytes as a u64
    /// This advances the internal state of the iterator.
    #[inline(always)]
    pub fn read_u64(&self) -> Option<u64> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<u64>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read. u64 is valid for all bit patterns
            unsafe { Some(self.read_unchecked()) }
        } else {
            None
        }
    }

    /// Check if the next element is a given value.
    #[inline(always)]
    pub fn first_is(&mut self, value: u8) -> bool {
        if let Some(&c) = self.slc.get(self.index) {
            c == value
        } else {
            false
        }
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline(always)]
    pub fn case_insensitive_first_is(&mut self, value: u8) -> bool {
        if let Some(&c) = self.slc.get(self.index) {
            c.to_ascii_lowercase() == value.to_ascii_lowercase()
        } else {
            false
        }
    }

    /// Get iterator over integer digits.
    #[inline(always)]
    pub fn integer_iter<'b>(&'b mut self) -> BytesIterator<'a, 'b, __> {
        BytesIterator {
            byte: self,
        }
    }

    /// Get iterator over fraction digits.
    #[inline(always)]
    pub fn fraction_iter<'b>(&'b mut self) -> BytesIterator<'a, 'b, __> {
        BytesIterator {
            byte: self,
        }
    }

    /// Get iterator over exponent digits.
    #[inline(always)]
    pub fn exponent_iter<'b>(&'b mut self) -> BytesIterator<'a, 'b, __> {
        BytesIterator {
            byte: self,
        }
    }

    /// Get iterator over special floating point values.
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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

    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self.byte.as_ptr()
    }

    #[inline(always)]
    fn as_slice(&self) -> &'a [u8] {
        self.byte.as_slice()
    }

    #[inline(always)]
    fn length(&self) -> usize {
        self.byte.length()
    }

    #[inline(always)]
    fn cursor(&self) -> usize {
        self.byte.cursor()
    }

    #[inline(always)]
    unsafe fn set_cursor(&mut self, index: usize) {
        debug_assert!(index <= self.length());
        // SAFETY: safe if `index <= self.length()`.
        unsafe { self.byte.set_cursor(index) };
    }

    #[inline(always)]
    fn current_count(&self) -> usize {
        self.byte.current_count()
    }

    #[inline(always)]
    fn is_consumed(&mut self) -> bool {
        Self::is_done(self)
    }

    #[inline(always)]
    fn is_done(&self) -> bool {
        self.byte.is_done()
    }

    #[inline(always)]
    unsafe fn peek_unchecked(&mut self) -> <Self as Iterator>::Item {
        // SAFETY: safe if `self.cursor() < self.length()`.
        unsafe { self.byte.slc.get_unchecked(self.byte.index) }
    }

    #[inline(always)]
    fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.byte.index < self.byte.slc.len() {
            // SAFETY: the slice cannot be empty, so this is safe
            Some(unsafe { self.peek_unchecked() })
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn read_unchecked<V>(&self) -> V {
        debug_assert!(self.as_slice().len() >= mem::size_of::<V>());
        // SAFETY: safe as long as the slice has at least count elements.
        unsafe { self.byte.read_unchecked() }
    }

    #[inline(always)]
    fn read_u32(&self) -> Option<u32> {
        self.byte.read_u32()
    }

    #[inline(always)]
    fn read_u64(&self) -> Option<u64> {
        self.byte.read_u64()
    }

    #[inline(always)]
    unsafe fn step_by_unchecked(&mut self, count: usize) {
        debug_assert!(self.as_slice().len() >= count);
        // SAFETY: safe as long as `slc.len() >= count`.
        unsafe { self.byte.step_by_unchecked(count) }
    }

    #[inline(always)]
    unsafe fn step_unchecked(&mut self) {
        debug_assert!(!self.as_slice().is_empty());
        // SAFETY: safe as long as `slc.len() >= 1`.
        unsafe { self.byte.step_unchecked() }
    }
}

impl<'a: 'b, 'b, const __: u128> Iterator for BytesIterator<'a, 'b, __> {
    type Item = &'a u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.byte.slc.get(self.byte.index)?;
        self.byte.index += 1;
        Some(value)
    }
}

impl<'a: 'b, 'b, const __: u128> ExactSizeIterator for BytesIterator<'a, 'b, __> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.length() - self.cursor()
    }
}
