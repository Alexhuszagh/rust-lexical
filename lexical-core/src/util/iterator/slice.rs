//! Iterator over slices.
//!
//! Very similar to `slice::Iter`, except it contains a few
//! methods for optimizations and raw ptr implementations.

use crate::lib::{mem, marker, slice, ptr};

use super::contiguous::*;

// SLICE ITERATOR
// --------------

/// Iterator over slices, with raw pointer comparisons.
///
/// Unlike `slice::Iter`, it does not handle zero-sized types.
pub(crate) struct SliceIterator<'a, T> {
    first: *const T,
    last: *const T,
    __marker: marker::PhantomData<&'a T>,
}

impl<'a, T> SliceIterator<'a, T> {
    /// Create new iterator from slice.
    #[inline]
    pub fn new(slc: &'a [T]) -> Self {
        debug_assert!(mem::size_of::<T>() != 0, "SliceIterator used with ZST.");
        unsafe {
            let first = slc.as_ptr();
            let last = first.add(slc.len());
            Self {
                first,
                last,
                __marker: marker::PhantomData
            }
        }
    }

    /// Get a slice to the current start of the iterator.
    #[inline]
    fn as_slice(&self) -> &'a [T] {
        unsafe {
            slice::from_raw_parts(self.first, self.len())
        }
    }
}

impl<'a, T: 'a + Clone> Clone for SliceIterator<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        SliceIterator {
            first: self.first,
            last: self.last,
            __marker: marker::PhantomData
        }
    }
}

impl<T> AsRef<[T]> for SliceIterator<'_, T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<'a, T> Iterator for SliceIterator<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.first != self.last {
                let ptr = self.first;
                self.first = self.first.add(1);
                Some(&*ptr)
            } else {
                None
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl<'a, T> DoubleEndedIterator for SliceIterator<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        unsafe {
            if self.first != self.last {
                self.last = self.last.sub(1);
                Some(&*self.last)
            } else {
                None
            }
        }
    }
}

impl<T> ExactSizeIterator for SliceIterator<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.last as usize - self.first as usize
    }
}

impl<'a, T: Clone> ContiguousIterator<'a, T> for SliceIterator<'a, T> {
    #[inline]
    fn new(slc: &'a [T], _: T) -> Self {
        SliceIterator::new(slc)
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self.first
    }

    #[inline]
    fn as_slice(&self) -> &'a [T] {
        SliceIterator::as_slice(self)
    }

    #[inline]
    fn empty(&mut self) -> bool {
        self.first == self.last
    }

    #[inline]
    fn consumed(&self) -> bool {
        self.first == self.last
    }

    #[inline]
    fn slice_length(&self) -> usize {
        self.len()
    }

    #[inline]
    fn advance(&mut self) {
        self.advance_n(1);
    }

    #[inline]
    unsafe fn advance_unchecked(&mut self) {
        self.advance_n_unchecked(1);
    }

    #[inline]
    fn advance_n(&mut self, n: usize) {
        unsafe {
            // Can't just take `self.first.add(n).min(self.last),
            // since if we have a wrapping add, this will fail
            // spectacularly.
            debug_assert!(self.first <= self.first.add(n));
            if self.len() <= n {
                self.advance_n_unchecked(n);
            } else {
                self.first = self.last;
            }
        }
    }

    #[inline]
    unsafe fn advance_n_unchecked(&mut self, n: usize) {
        self.first = self.first.add(n);
    }

    #[inline]
    fn peek(&mut self) -> Option<&'a T> {
        if !self.empty() {
            unsafe {
                Some(&*self.first)
            }
        } else {
            None
        }
    }

    #[inline]
    unsafe fn peek_unchecked(&mut self) -> &'a T {
        &*self.first
    }

    #[inline]
    unsafe fn set_ptr(&mut self, ptr: *const T) {
        self.first = ptr;
    }

    #[inline]
    unsafe fn set_end(&mut self, ptr: *const T) {
        self.last = ptr;
    }

    #[inline]
    unsafe fn read<V>(&self) -> (V, *const T) {
        let value = ptr::read_unaligned::<V>(self.first as *const _);
        let ptr = self.first.add(mem::size_of::<V>());
        (value, ptr)
    }

    #[inline]
    fn try_read<V>(&self) -> Option<(V, *const T)> {
        if self.len() >= mem::size_of::<V>() {
            unsafe {
                Some(self.read())
            }
        } else {
            None
        }
    }

    #[inline]
    fn trim(&mut self) {
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    type ByteIterator<'a> = SliceIterator<'a, u8>;

    #[test]
    fn slice_iterator_test() {
        let digits = b"12345";
        let mut iter = ByteIterator::new(digits);
        assert_eq!(iter.as_slice(), &digits[..]);
        assert_eq!(iter.as_ptr(), digits.as_ptr());
        assert_eq!(iter.try_read::<u32>().unwrap().0, 0x34333231);
        assert_eq!(iter.try_read::<u64>(), None);
        assert_eq!(iter.nth(4).unwrap(), &b'5');
        assert_eq!(iter.as_slice(), &digits[digits.len()..]);
        assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());
    }
}
