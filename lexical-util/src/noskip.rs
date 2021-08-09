//! An iterator over a slice.
//!
//! This iterator has both the length of the original slice, as
//! well as the current position of the iterator in the buffer.

#![cfg(all(feature = "parse", not(feature = "format")))]

use crate::iterator::{Byte, ByteIter};

// AS DIGITS
// ---------

/// Trait to simplify creation of a `Digits` object.
pub trait AsDigits<'a> {
    /// Create `Digits` from object.
    fn digits<const __: u128>(&'a self) -> Digits<'a>;
}

impl<'a> AsDigits<'a> for [u8] {
    #[inline]
    fn digits<const __: u128>(&'a self) -> Digits<'a> {
        Digits::new(self)
    }
}

// DIGITS
// ------

/// Slice iterator that stores the original length of the slice.
#[derive(Clone)]
pub struct Digits<'a> {
    /// The raw slice for the iterator.
    slc: &'a [u8],
    /// Current index of the iterator in the slice.
    index: usize,
}

impl<'a> Digits<'a> {
    /// Create new byte object.
    #[inline]
    pub const fn new(slc: &'a [u8]) -> Self {
        Self {
            slc,
            index: 0,
        }
    }
}

impl<'a> Byte<'a> for Digits<'a> {
    const IS_CONTIGUOUS: bool = true;
    type IntegerIter = DigitsIterator<'a>;
    type FractionIter = DigitsIterator<'a>;
    type ExponentIter = DigitsIterator<'a>;
    type SpecialIter = DigitsIterator<'a>;

    #[inline]
    fn new(slc: &'a [u8]) -> Self {
        Digits::new(slc)
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
    fn current_count(&self) -> usize {
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

// DIGITS ITERATOR
// ---------------

/// Slice iterator that stores the original length of the slice.
pub struct DigitsIterator<'a> {
    /// The internal byte object for the noskip iterator.
    byte: &'a mut Digits<'a>,
}

impl<'a> Iterator for DigitsIterator<'a> {
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.byte.slc.get(self.byte.index)?;
        self.byte.index += 1;
        Some(value)
    }
}

impl<'a> ExactSizeIterator for DigitsIterator<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.length() - self.cursor()
    }
}

impl<'a> ByteIter<'a> for DigitsIterator<'a> {
    const IS_CONTIGUOUS: bool = Digits::IS_CONTIGUOUS;

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
    fn current_count(&self) -> usize {
        self.byte.current_count()
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
        debug_assert!(!self.as_slice().is_empty());
        self.byte.index += 1;
    }
}
