//! An iterator that skips values equal to a provided value.
//!
//! SkipValueIterator iterates over a slice, returning all values
//! except for those matching the provided skip value.
//!
//! Example
//! -------
//!
//! ```text
//! let iter = SkipValueIterator(&[1, 2, 5, 2, 6, 7], 2);
//! assert!(iter.eq([1, 5, 6, 7].iter()));
//! ```

use crate::lib::{mem, ptr};

use super::contiguous::*;
use super::slice::*;

// SKIP
// ----

/// Slice iterator that skips characters matching a given value.
///
/// This wraps an iterator over a contiguous block of memory,
/// and only returns values that are not equal to skip.
pub(crate) struct SkipValueIterator<'a, T: 'a + PartialEq> {
    /// Slice iterator to wrap.
    iter: SliceIterator<'a, T>,
    /// Value to skip.
    skip: T,
}

impl<'a, T: 'a + PartialEq> SkipValueIterator<'a, T> {
    #[inline]
    pub(crate) fn new(slc: &'a [T], skip: T) -> Self {
        SkipValueIterator {
            iter: SliceIterator::new(slc),
            skip,
        }
    }
}

impl<'a, T: 'a + PartialEq + Clone> Clone for SkipValueIterator<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        SkipValueIterator {
            iter: self.iter.clone(),
            skip: self.skip.clone(),
        }
    }
}

impl<'a, T: 'a + PartialEq> Iterator for SkipValueIterator<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let value = self.iter.next()?;
            if *value != self.skip {
                return Some(value);
            }
        }
    }
}

impl<'a, T> ContiguousIterator<'a, T> for SkipValueIterator<'a, T>
where
    T: 'a + PartialEq + Clone
{
    #[inline]
    fn new(slc: &'a [T], skip: T) -> Self {
        SkipValueIterator::new(slc, skip)
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self.iter.as_ptr()
    }

    #[inline]
    fn as_slice(&self) -> &'a [T] {
        self.iter.as_slice()
    }

    #[inline]
    fn empty(&mut self) -> bool {
        self.peek().is_some()
    }

    #[inline]
    fn consumed(&self) -> bool {
        self.iter.consumed()
    }

    #[inline]
    fn slice_length(&self) -> usize {
        self.iter.len()
    }

    #[inline]
    fn advance(&mut self) {
        self.advance_n(1);
    }

    #[inline]
    unsafe fn advance_unchecked(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn advance_n(&mut self, n: usize) {
        // TODO(ahuszagh) This is pretty inefficient with ptrs.
        // Might need to... you know, have a set_ptr thing.
        for _ in 0..n {
            self.iter.next();
        }
    }

    #[inline]
    unsafe fn advance_n_unchecked(&mut self, _: usize) {
        unimplemented!()
    }

    #[inline]
    fn peek(&mut self) -> Option<&'a T> {
        // Advance the iterator state to the next value,
        // but don't consume it.
        loop {
            let value = self.iter.peek()?;
            if *value == self.skip {
                self.iter.next();
            } else {
                return Some(value);
            }
        }
    }

    #[inline]
    unsafe fn peek_unchecked(&mut self) -> &'a T {
        // This can't be reasonably implemented, since we must loop
        // to find the next element.
        unimplemented!()
    }

    #[inline]
    unsafe fn set_ptr(&mut self, ptr: *const T) {
        self.iter.set_ptr(ptr);
    }

    #[inline]
    unsafe fn set_end(&mut self, ptr: *const T) {
        self.iter.set_end(ptr);
    }

    #[inline]
    unsafe fn read<V>(&self) -> (V, *const T) {
        // This can't be reasonably implemented, since we do not know
        // how many elements we have.
        unimplemented!()
    }

    #[inline]
    fn try_read<V>(&self) -> Option<(V, *const T)> {
        // Assert is fine here: it should be optimized out in release
        // builds, since the size it known at compile time.
        assert!(mem::size_of::<T>() % mem::size_of::<V>() == 0);
        assert!(mem::size_of::<T>() <= mem::size_of::<V>());

        // Assume a max of 8, so we don't have to reinterpret it.
        let size = mem::size_of::<V>() / mem::size_of::<T>();
        assert!(size <= 8);

        // Clone self, and read into a static array and reinterpret it.
        let mut iter = self.clone();
        let mut array: mem::MaybeUninit<[T; 8]> = mem::MaybeUninit::uninit();
        let array = array.as_mut_ptr() as *mut T;
        unsafe {
            for i in 0..size {
                // Have a memory leak, in theory, if we return None.
                // In practice, since everything is trivially droppable,
                // should be fine.
                ptr::write(array.add(i), iter.next()?.clone());
            }

            // Read our value in, which we've guaranteed by here
            // we have enough values for.
            let value = ptr::read_unaligned::<V>(array as *const _);

            // Drop our values (this should be a no-op).
            // Don't need to forget our array, since we haven't ever
            // assumed it's init.
            for i in 0..size {
                ptr::drop_in_place(array.add(i));
            }

            Some((value, iter.as_ptr()))
        }
    }

    #[inline]
    fn trim(&mut self) {
        // Will auto-advance to the next iterator value or the end.
        self.peek();
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_value_test() {
        let slc = &[1, 2, 5, 2, 6, 7];
        let iter = SkipValueIterator::new(slc, 2);
        assert!(iter.eq([1, 5, 6, 7].iter()));

        let iter = SkipValueIterator::new(slc, 5);
        assert!(iter.eq([1, 2, 2, 6, 7].iter()));

        let iter = SkipValueIterator::new(slc, 1);
        assert!(iter.eq([2, 5, 2, 6, 7].iter()));
    }
}
