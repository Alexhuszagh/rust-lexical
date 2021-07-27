//! An iterator over a slice.
//!
//! This iterator has both the length of the original slice, as
//! well as the current position of the iterator in the buffer.

#![cfg(feature = "parse")]

use crate::iterator::ByteIter;

// NOSKIP ITER
// -----------

/// Trait to simplify creation of a `SkipIterator`.
pub trait NoSkipIter<'a> {
    /// Create `NoSkipIterator` from .
    fn noskip_iter(&'a self) -> NoSkipIterator<'a>;
}

impl<'a> NoSkipIter<'a> for [u8] {
    #[inline]
    fn noskip_iter(&'a self) -> NoSkipIterator<'a> {
        NoSkipIterator::new(self)
    }
}

// NOSKIP
// ------

/// Slice iterator that stores the original length of the slice.
#[derive(Clone)]
pub struct NoSkipIterator<'a> {
    /// The raw slice for the iterator.
    slc: &'a [u8],
    /// Current index of the iterator in the slice.
    index: usize,
}

impl<'a> NoSkipIterator<'a> {
    /// Create new iterator.
    #[inline]
    pub fn new(slc: &'a [u8]) -> Self {
        Self {
            slc,
            index: 0,
        }
    }
}

impl<'a> Iterator for NoSkipIterator<'a> {
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.slc.get(self.index)?;
        self.index += 1;
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
    const IS_CONTIGUOUS: bool = true;

    #[inline]
    fn new(slc: &'a [u8]) -> Self {
        NoSkipIterator::new(slc)
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
    fn is_consumed(&mut self) -> bool {
        self.index >= self.slc.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.index >= self.slc.len()
    }

    #[inline]
    unsafe fn peek_unchecked(&mut self) -> Self::Item {
        // SAFETY: safe as long as the slice is not empty.
        unsafe { self.slc.get_unchecked(self.index) }
    }

    #[inline]
    fn peek(&mut self) -> Option<Self::Item> {
        if self.index < self.slc.len() {
            // SAFETY: the slice cannot be empty, so this is safe
            Some(unsafe { self.peek_unchecked() })
        } else {
            None
        }
    }

    #[inline]
    unsafe fn step_by_unchecked(&mut self, count: usize) {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= count);
        self.index += count;
    }
}
