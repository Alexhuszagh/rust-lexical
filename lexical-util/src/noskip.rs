//! An iterator over a slice.
//!
//! This iterator has both the length of the original slice, as
//! well as the current position of the iterator in the buffer.

#![cfg(all(feature = "parse", not(feature = "format")))]

use core::{mem, ptr};

use crate::digit::char_is_digit_const;
use crate::iterator::{DigitsIter, Iter};
use crate::format::NumberFormat;

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
    /// Create new byte object.
    #[inline(always)]
    pub const fn new(slc: &'a [u8]) -> Self {
        Self {
            slc,
            index: 0,
        }
    }

    // TODO: Move to `Iter` as a trait along with `new` as well`
    /// Initialize the slice from raw parts.
    ///
    /// # Safety
    /// This is safe if and only if the index is <= slc.len().
    /// For this reason, since it's easy to get wrong, we only
    /// expose it to `DigitsIterator` and nothing else.
    #[inline(always)]
    #[allow(clippy::assertions_on_constants)]
    const unsafe fn from_parts(slc: &'a [u8], index: usize) -> Self {
        debug_assert!(index <= slc.len());
        debug_assert!(Self::IS_CONTIGUOUS);
        Self {
            slc,
            index,
        }
    }

    /// Get if the buffer underlying the iterator is empty.
    /// Same as `is_consumed`.
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        self.index >= self.slc.len()
    }

    // TODO: Remove the peek_is, these shouldn't be on bytes

    /// Check if the next element is a given value.
    #[inline(always)]
    pub fn peek_is_cased(&mut self, value: u8) -> bool {
        // TODO: These 2 need to be changed
        self.first_is(value)
    }

    /// Check if the next element is a given value without case sensitivity.
    #[inline(always)]
    pub fn peek_is_uncased(&mut self, value: u8) -> bool {
        // TODO: These 2 need to be changed
        self.case_insensitive_first_is(value)
    }

    /// Get iterator over integer digits.
    #[inline(always)]
    pub fn integer_iter<'b>(&'b mut self) -> DigitsIterator<'a, 'b, __> {
        DigitsIterator {
            byte: self,
        }
    }

    /// Get iterator over fraction digits.
    #[inline(always)]
    pub fn fraction_iter<'b>(&'b mut self) -> DigitsIterator<'a, 'b, __> {
        DigitsIterator {
            byte: self,
        }
    }

    /// Get iterator over exponent digits.
    #[inline(always)]
    pub fn exponent_iter<'b>(&'b mut self) -> DigitsIterator<'a, 'b, __> {
        DigitsIterator {
            byte: self,
        }
    }

    /// Get iterator over special floating point values.
    #[inline(always)]
    pub fn special_iter<'b>(&'b mut self) -> DigitsIterator<'a, 'b, __> {
        DigitsIterator {
            byte: self,
        }
    }
}

unsafe impl<'a, const __: u128> Iter<'a> for Bytes<'a, __> {
    const IS_CONTIGUOUS: bool = true;

    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }

    #[inline(always)]
    fn as_slice(&self) -> &'a [u8] {
        debug_assert!(self.index <= self.length());
        // SAFETY: safe since index must be in range.
        unsafe { self.slc.get_unchecked(self.index..) }
    }

    #[inline(always)]
    fn get_buffer(&self) -> &'a [u8] {
        self.slc
    }

    /// Get the current index of the iterator in the slice.
    #[inline(always)]
    fn cursor(&self) -> usize {
        self.index
    }

    /// Set the current index of the iterator in the slice.
    ///
    /// # Safety
    ///
    /// Safe if `index <= self.length()`.
    #[inline(always)]
    unsafe fn set_cursor(&mut self, index: usize) {
        debug_assert!(index <= self.length());
        self.index = index
    }

    /// Get the current number of values returned by the iterator.
    #[inline(always)]
    fn current_count(&self) -> usize {
        self.index
    }

    // TODO: Rename
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    #[inline(always)]
    fn first(&self) -> Option<&'a u8> {
        self.slc.get(self.index)
    }

    #[inline(always)]
    #[allow(clippy::assertions_on_constants)]
    unsafe fn step_by_unchecked(&mut self, count: usize) {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= count);
        self.index += count;
    }

    #[inline(always)]
    #[allow(clippy::assertions_on_constants)]
    unsafe fn read_unchecked<V>(&self) -> V {
        debug_assert!(Self::IS_CONTIGUOUS);
        debug_assert!(self.as_slice().len() >= mem::size_of::<V>());

        // SAFETY: safe as long as the slice has at least count elements.
        unsafe { ptr::read_unaligned::<V>(self.as_ptr() as *const _) }
    }

    #[inline(always)]
    fn read_u32(&self) -> Option<u32> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<u32>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read. u32 is valid for all bit patterns
            unsafe { Some(self.read_unchecked()) }
        } else {
            None
        }
    }

    #[inline(always)]
    fn read_u64(&self) -> Option<u64> {
        if Self::IS_CONTIGUOUS && self.as_slice().len() >= mem::size_of::<u64>() {
            // SAFETY: safe since we've guaranteed the buffer is greater than
            // the number of elements read. u64 is valid for all bit patterns
            unsafe { Some(self.read_unchecked()) }
        } else {
            None
        }
    }
}

// DIGITS ITERATOR
// ---------------

/// Slice iterator that stores the original length of the slice.
pub struct DigitsIterator<'a: 'b, 'b, const __: u128> {
    /// The internal byte object for the noskip iterator.
    byte: &'b mut Bytes<'a, __>,
}

impl<'a: 'b, 'b, const __: u128> DigitsIterator<'a, 'b, __> {
    /// Create a new digits iterator from the bytes underlying item.
    #[inline(always)]
    pub fn new(byte: &'b mut Bytes<'a, __>) -> Self {
        Self { byte }
    }

    // TODO: Move as a trait

    /// Take the first N digits from the iterator.
    ///
    /// This only takes the digits if we have a contiguous iterator.
    /// It takes the digits, validating the bounds, and then advanced
    /// the iterators state.
    #[cfg_attr(not(feature = "compact"), inline(always))]
    #[allow(clippy::assertions_on_constants)]
    pub fn take_n(&mut self, n: usize) -> Option<Bytes<'a, __>> {
        debug_assert!(Self::IS_CONTIGUOUS);
        let end = self.byte.slc.len().min(n + self.cursor());
        // NOTE: The compiler should be able to optimize this out.
        let slc: &[u8] = &self.byte.slc[..end];

        // SAFETY: Safe since we just ensured the underlying slice has that count
        // elements, so both the underlying slice for this and this **MUST**
        // have at least count elements. We do static checking on the bounds for this.
        unsafe {
            let byte: Bytes<'_, __> = Bytes::from_parts(slc, self.cursor());
            unsafe { self.set_cursor(end) };
            Some(byte)
        }
    }
}

unsafe impl<'a: 'b, 'b, const __: u128> Iter<'a> for DigitsIterator<'a, 'b, __> {
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
    fn get_buffer(&self) -> &'a [u8] {
        self.byte.get_buffer()
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
    fn is_empty(&self) -> bool {
        self.byte.is_done()
    }

    #[inline(always)]
    fn first(&self) -> Option<&'a u8> {
        self.byte.first()
    }

    #[inline(always)]
    unsafe fn step_by_unchecked(&mut self, count: usize) {
        debug_assert!(self.as_slice().len() >= count);
        // SAFETY: safe as long as `slc.len() >= count`.
        unsafe { self.byte.step_by_unchecked(count) }
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
}

unsafe impl<'a: 'b, 'b, const FORMAT: u128> DigitsIter<'a> for DigitsIterator<'a, 'b, FORMAT> {
    #[inline(always)]
    fn is_consumed(&mut self) -> bool {
        Self::is_done(self)
    }

    #[inline(always)]
    fn is_done(&self) -> bool {
        self.byte.is_done()
    }

    #[inline(always)]
    fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
        self.byte.slc.get(self.byte.index)
    }

    /// Determine if the character is a digit.
    #[inline(always)]
    fn is_digit(&self, value: u8) -> bool {
        let format = NumberFormat::<{ FORMAT }> {};
        char_is_digit_const(value, format.mantissa_radix())
    }
}

impl<'a: 'b, 'b, const __: u128> Iterator for DigitsIterator<'a, 'b, __> {
    type Item = &'a u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.byte.slc.get(self.byte.index)?;
        self.byte.index += 1;
        Some(value)
    }
}

impl<'a: 'b, 'b, const __: u128> ExactSizeIterator for DigitsIterator<'a, 'b, __> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.length() - self.cursor()
    }
}
