//! Helper for vector-like classes.

#![allow(dead_code)]

use lib::{iter, ops, ptr, Vec};
use smallvec;
use stackvector;

// REMOVE_MANY

/// Remove many elements from a vec-like container.
///
/// Does not change the size of the vector, and may leak
/// if the destructor panics. **Must** call `set_len` after,
/// and ideally before (to 0).
fn remove_many<V, T, R>(vec: &mut V, range: R)
    where V: VecLike<T>,
          R: ops::RangeBounds<usize>
{
    // Get the bounds on the items we're removing.
    let len = vec.len();
    let start = match range.start_bound() {
        ops::Bound::Included(&n) => n,
        ops::Bound::Excluded(&n) => n + 1,
        ops::Bound::Unbounded    => 0,
    };
    let end = match range.end_bound() {
        ops::Bound::Included(&n) => n + 1,
        ops::Bound::Excluded(&n) => n,
        ops::Bound::Unbounded    => len,
    };
    assert!(start <= end);
    assert!(end <= len);

    // Drop the existing items.
    unsafe {
        // Set len temporarily to the start, in case we panic on a drop.
        // This means we leak memory, but we don't allow any double freeing,
        // or use after-free.
        vec.set_len(start);
        // Iteratively drop the range.
        let mut first = vec.as_mut_ptr().add(start);
        let last = vec.as_mut_ptr().add(end);
        while first < last {
            ptr::drop_in_place(first);
            first = first.add(1);
        }

        // Now we need to copy the end range into the buffer.
        let count = len - end;
        if count != 0 {
            let src = vec.as_ptr().add(end);
            let dst = vec.as_mut_ptr().add(start);
            ptr::copy(src, dst, count);
        }

        // Set the proper length, now that we've moved items in.
        vec.set_len(start + count);
    }
}

// VECLIKE

pub trait VecLike<T>:
    ops::Index<usize, Output=T> +
    ops::IndexMut<usize> +
    ops::Index<ops::Range<usize>, Output=[T]> +
    ops::IndexMut<ops::Range<usize>> +
    ops::Index<ops::RangeFrom<usize>, Output=[T]> +
    ops::IndexMut<ops::RangeFrom<usize>> +
    ops::Index<ops::RangeTo<usize>, Output=[T]> +
    ops::IndexMut<ops::RangeTo<usize>> +
    ops::Index<ops::RangeFull, Output=[T]> +
    ops::IndexMut<ops::RangeFull> +
    ops::DerefMut<Target = [T]> +
    Extend<T>
{

    /// Append an element to the vector.
    fn push(&mut self, value: T);

    /// Pop an element from the end of the vector.
    fn pop(&mut self) -> Option<T>;

    /// Insert many elements at index, pushing everything else to the back.
    fn insert_many<I: iter::IntoIterator<Item=T>>(&mut self, index: usize, iterable: I);

    /// Remove many elements from range.
    fn remove_many<R: ops::RangeBounds<usize>>(&mut self, range: R);

    /// Set the buffer length (unsafe).
    unsafe fn set_len(&mut self, new_len: usize);

    // FRONT

    /// Get an immutable reference to the front item.
    #[inline(always)]
    fn front(&self) -> Option<&T> {
        self.get(0)
    }

    /// Get an mutable reference to the front item.
    #[inline(always)]
    fn front_mut(&mut self) -> Option<&mut T> {
        debug_assert!(self.len() > 0);
        self.get_mut(0)
    }

    /// Get an immutable reference to the front item.
    #[inline(always)]
    unsafe fn front_unchecked(&self) -> &T {
        debug_assert!(self.len() > 0);
        self.get_unchecked(0)
    }

    /// Get an mutable reference to the front item.
    #[inline(always)]
    unsafe fn front_unchecked_mut(&mut self) -> &mut T {
        debug_assert!(self.len() > 0);
        self.get_unchecked_mut(0)
    }

    // BACK

    /// Get an immutable reference to the back item.
    #[inline(always)]
    fn back(&self) -> Option<&T> {
        let index = self.len() - 1;
        self.get(index)
    }

    /// Get an mutable reference to the back item.
    #[inline(always)]
    fn back_mut(&mut self) -> Option<&mut T> {
        debug_assert!(self.len() > 0);
        let index = self.len() - 1;
        self.get_mut(index)
    }

    /// Get an immutable reference to the back item.
    #[inline(always)]
    unsafe fn back_unchecked(&self) -> &T {
        debug_assert!(self.len() > 0);
        let index = self.len() - 1;
        self.get_unchecked(index)
    }

    /// Get an mutable reference to the back item.
    #[inline(always)]
    unsafe fn back_unchecked_mut(&mut self) -> &mut T {
        debug_assert!(self.len() > 0);
        let index = self.len() - 1;
        self.get_unchecked_mut(index)
    }
}

// VEC

impl<T> VecLike<T> for Vec<T> {
    #[inline]
    fn push(&mut self, value: T) {
        Vec::push(self, value);
    }

    #[inline]
    fn pop(&mut self) -> Option<T> {
        Vec::pop(self)
    }

    #[inline]
    unsafe fn set_len(&mut self, new_len: usize) {
        Vec::set_len(self, new_len);
    }

    #[inline]
    fn insert_many<I: iter::IntoIterator<Item=T>>(&mut self, index: usize, iterable: I) {
        self.splice(index..index, iterable);
    }

    #[inline]
    fn remove_many<R: ops::RangeBounds<usize>>(&mut self, range: R) {
        remove_many(self, range)
    }
}

// SMALLVEC

impl<A: smallvec::Array> VecLike<A::Item> for smallvec::SmallVec<A> {
    #[inline]
    fn push(&mut self, value: A::Item) {
        smallvec::SmallVec::push(self, value);
    }

    #[inline]
    fn pop(&mut self) -> Option<A::Item> {
        smallvec::SmallVec::pop(self)
    }

    #[inline]
    unsafe fn set_len(&mut self, new_len: usize) {
        smallvec::SmallVec::set_len(self, new_len);
    }

    #[inline]
    fn insert_many<I: iter::IntoIterator<Item=A::Item>>(&mut self, index: usize, iterable: I) {
        smallvec::SmallVec::insert_many(self, index, iterable)
    }

    #[inline]
    fn remove_many<R: ops::RangeBounds<usize>>(&mut self, range: R) {
        remove_many(self, range)
    }
}

// STACKVEC

impl<A: stackvector::Array> VecLike<A::Item> for stackvector::StackVec<A> {
    #[inline]
    fn push(&mut self, value: A::Item) {
        stackvector::StackVec::push(self, value);
    }

    #[inline]
    fn pop(&mut self) -> Option<A::Item> {
        stackvector::StackVec::pop(self)
    }

    #[inline]
    unsafe fn set_len(&mut self, new_len: usize) {
        stackvector::StackVec::set_len(self, new_len);
    }

    #[inline]
    fn insert_many<I: iter::IntoIterator<Item=A::Item>>(&mut self, index: usize, iterable: I) {
        stackvector::StackVec::insert_many(self, index, iterable)
    }

    #[inline]
    fn remove_many<R: ops::RangeBounds<usize>>(&mut self, range: R) {
        remove_many(self, range)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_many_test() {
        let mut x = vec![0, 1, 2, 3, 4, 5];
        x.remove_many(0..3);
        assert_eq!(x, vec![3, 4, 5]);
        assert_eq!(x.len(), 3);

        let mut x = vec![0, 1, 2, 3, 4, 5];
        x.remove_many(..);
        assert_eq!(x, vec![]);

        let mut x = vec![0, 1, 2, 3, 4, 5];
        x.remove_many(3..);
        assert_eq!(x, vec![0, 1, 2]);

        let mut x = vec![0, 1, 2, 3, 4, 5];
        x.remove_many(..3);
        assert_eq!(x, vec![3, 4, 5]);
    }
}
