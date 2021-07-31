//! An iterator over a slice.
//!
//! This iterator has both the length of the original slice, as
//! well as the current position of the iterator in the buffer.

#![cfg(feature = "parse")]

use crate::iterator::{Byte, ByteIter};

// NOSKIP ITER
// -----------

/// Trait to simplify creation of a `NoSkip` object.
pub trait AsNoSkip<'a> {
    /// Create `NoSkip` from object.
    fn noskip(&'a self) -> NoSkip<'a>;
}

impl<'a> AsNoSkip<'a> for [u8] {
    #[inline]
    fn noskip(&'a self) -> NoSkip<'a> {
        NoSkip::new(self)
    }
}

// NOSKIP
// ------

/// Slice iterator that stores the original length of the slice.
#[derive(Clone)]
pub struct NoSkip<'a> {
    /// The raw slice for the iterator.
    slc: &'a [u8],
    /// Current index of the iterator in the slice.
    index: usize,
}

impl<'a> NoSkip<'a> {
    /// Create new byte object.
    #[inline]
    pub const fn new(slc: &'a [u8]) -> Self {
        Self {
            slc,
            index: 0,
        }
    }
}

impl<'a> Byte<'a> for NoSkip<'a> {
    const IS_CONTIGUOUS: bool = true;
    type IntegerIter = NoSkipIterator<'a>;
    type FractionIter = NoSkipIterator<'a>;
    type ExponentIter = NoSkipIterator<'a>;
    type SpecialIter = NoSkipIterator<'a>;

    #[inline]
    fn new(slc: &'a [u8]) -> Self {
        NoSkip::new(slc)
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }

    #[inline]
    fn as_slice(&self) -> &'a [u8] {
        // SAFETY: safe since index must be in range
        unsafe { self.slc.get_unchecked(self.index..) }
    }

    #[inline]
    fn length(&self) -> usize {
        self.slc.len()
    }

    #[inline]
    fn cursor(&self) -> usize {
        self.index
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.index >= self.slc.len()
    }

    #[inline]
    fn integer_iter(&'a mut self) -> Self::IntegerIter {
        Self::IntegerIter {
            byte: self,
        }
    }

    #[inline]
    fn fraction_iter(&'a mut self) -> Self::FractionIter {
        Self::FractionIter {
            byte: self,
        }
    }

    #[inline]
    fn exponent_iter(&'a mut self) -> Self::ExponentIter {
        Self::ExponentIter {
            byte: self,
        }
    }

    #[inline]
    fn special_iter(&'a mut self) -> Self::SpecialIter {
        Self::SpecialIter {
            byte: self,
        }
    }
}

// NOSKIP ITERATOR
// ---------------

/// Slice iterator that stores the original length of the slice.
pub struct NoSkipIterator<'a> {
    /// The internal byte object for the noskip iterator.
    byte: &'a mut NoSkip<'a>,
}

impl<'a> Iterator for NoSkipIterator<'a> {
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.byte.slc.get(self.byte.index)?;
        self.byte.index += 1;
        Some(value)
    }
}

impl<'a> ExactSizeIterator for NoSkipIterator<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.length() - self.cursor()
    }
}

impl<'a> ByteIter<'a> for NoSkipIterator<'a> {
    const IS_CONTIGUOUS: bool = NoSkip::IS_CONTIGUOUS;

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
    fn is_consumed(&mut self) -> bool {
        ByteIter::is_empty(self)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.byte.is_empty()
    }

    #[inline]
    unsafe fn peek_unchecked(&mut self) -> Self::Item {
        // SAFETY: safe as long as the slice is not empty.
        unsafe { self.byte.slc.get_unchecked(self.byte.index) }
    }

    #[inline]
    fn peek(&mut self) -> Option<Self::Item> {
        if self.byte.index < self.byte.slc.len() {
            // SAFETY: the slice cannot be empty, so this is safe
            Some(unsafe { self.peek_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    #[allow(clippy::assertions_on_constants)]
    unsafe fn step_by_unchecked(&mut self, count: usize) {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= count);
        self.byte.index += count;
    }

    #[inline]
    #[allow(clippy::assertions_on_constants)]
    unsafe fn step_unchecked(&mut self) {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= 1);
        self.byte.index += 1;
    }
}
