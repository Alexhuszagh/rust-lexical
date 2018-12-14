//! Helper for vector-like classes.

#![allow(dead_code)]

use lib::{iter, ops, ptr, slice, Vec};
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

// AS SLICE

/// Collection that has an `as_slice()` method.
pub trait AsSlice<T>: {
    /// View the collection as a slice.
    fn as_slice(&self) -> &[T];
}

impl<T> AsSlice<T> for Vec<T> {
    #[inline]
    fn as_slice(&self) -> &[T] {
        Vec::as_slice(self)
    }
}

impl<A: stackvector::Array> AsSlice<A::Item> for stackvector::StackVec<A> {
    #[inline]
    fn as_slice(&self) -> &[A::Item] {
        stackvector::StackVec::as_slice(self)
    }
}

// EXTEND FROM SLICE

/// Collection that can be extended from a slice.
pub trait ExtendFromSlice<T: Clone>: Clone + Default {
    /// Extend collection from slice.
    fn extend_from_slice(&mut self, other: &[T]);
}

impl<T: Clone> ExtendFromSlice<T> for Vec<T> {
    #[inline]
    fn extend_from_slice(&mut self, other: &[T]) {
        Vec::extend_from_slice(self, other)
    }
}

impl<A: stackvector::Array> ExtendFromSlice<A::Item> for stackvector::StackVec<A>
    where A::Item: Copy + Clone
{
    #[inline]
    fn extend_from_slice(&mut self, other: &[A::Item]) {
        stackvector::StackVec::extend_from_slice(self, other)
    }
}

// LEN

/// Collection that has a `len()` method.
pub trait Len<T>: {
    /// Get the length of the collection.
    fn len(&self) -> usize;
}

impl<T> Len<T> for [T] {
    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T> Len<T> for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<A: stackvector::Array> Len<A::Item> for stackvector::StackVec<A> {
    #[inline]
    fn len(&self) -> usize {
        stackvector::StackVec::len(self)
    }
}

// RESERVE

/// Collection that can call reserve.
pub trait Reserve<T>: {
    /// Reserve additional capacity for the collection.
    fn reserve(&mut self, capacity: usize);
}

impl<T> Reserve<T> for Vec<T> {
    #[inline]
    fn reserve(&mut self, capacity: usize) {
        Vec::reserve(self, capacity)
    }
}

impl<A: stackvector::Array> Reserve<A::Item> for stackvector::StackVec<A> {
    #[inline]
    fn reserve(&mut self, capacity: usize) {
        assert!(capacity < self.capacity());
    }
}

// RESERVE EXACT

/// Collection that can call reserve_exact.
pub trait ReserveExact<T>: {
    /// Reserve minimal additional capacity for the collection.
    fn reserve_exact(&mut self, capacity: usize);
}

impl<T> ReserveExact<T> for Vec<T> {
    #[inline]
    fn reserve_exact(&mut self, capacity: usize) {
        Vec::reserve_exact(self, capacity)
    }
}

impl<A: stackvector::Array> ReserveExact<A::Item> for stackvector::StackVec<A>
{
    #[inline]
    fn reserve_exact(&mut self, capacity: usize) {
        assert!(capacity < self.capacity());
    }
}

// RESIZE

/// Resizable container.
///
/// Implemented for Vec, SmallVec, and StackVec.
pub trait Resize<T: Clone>: Clone + Default {
    /// Resize container to new length, with a default value if adding elements.
    fn resize(&mut self, len: usize, value: T);
}

impl<T: Clone> Resize<T> for Vec<T> {
    #[inline]
    fn resize(&mut self, len: usize, value: T) {
        Vec::resize(self, len, value)
    }
}

impl<A: stackvector::Array> Resize<A::Item> for stackvector::StackVec<A>
    where A::Item: Clone
{
    #[inline]
    fn resize(&mut self, len: usize, value: A::Item) {
        stackvector::StackVec::resize(self, len, value)
    }
}

/// A trait for reversed-indexing operations.
pub trait RSliceIndex<T: ?Sized> {
    /// Output type for the index.
    type Output: ?Sized;

    /// Get reference to element or subslice.
    fn rget(self, slc: &T) -> Option<&Self::Output>;

    /// Get mutable reference to element or subslice.
    fn rget_mut(self, slc: &mut T) -> Option<&mut Self::Output>;

    /// Get reference to element or subslice without bounds checking.
    unsafe fn rget_unchecked(self, slc: &T) -> &Self::Output;

    /// Get mutable reference to element or subslice without bounds checking.
    unsafe fn rget_unchecked_mut(self, slc: &mut T) -> &mut Self::Output;

    /// Get reference to element or subslice, panic if out-of-bounds.
    fn rindex(self, slc: &T) -> &Self::Output;

    /// Get mutable reference to element or subslice, panic if out-of-bounds.
    fn rindex_mut(self, slc: &mut T) -> &mut Self::Output;
}

impl<T> RSliceIndex<[T]> for usize {
    type Output = T;

    #[inline]
    fn rget(self, slc: &[T]) -> Option<&T> {
        let len = slc.len();
        slc.get(len - self - 1)
    }

    #[inline]
    fn rget_mut(self, slc: &mut [T]) -> Option<&mut T> {
        let len = slc.len();
        slc.get_mut(len - self - 1)
    }

    #[inline]
    unsafe fn rget_unchecked(self, slc: &[T]) -> &T {
        let len = slc.len();
        slc.get_unchecked(len - self - 1)
    }

    #[inline]
    unsafe fn rget_unchecked_mut(self, slc: &mut [T]) -> &mut T {
        let len = slc.len();
        slc.get_unchecked_mut(len - self - 1)
    }

    #[inline]
    fn rindex(self, slc: &[T]) -> &T {
        let len = slc.len();
        &(*slc)[len - self - 1]
    }

    #[inline]
    fn rindex_mut(self, slc: &mut [T]) -> &mut T {
        let len = slc.len();
        &mut (*slc)[len - self - 1]
    }
}

// RGET

/// Get items using reverse-indexing.
pub trait Rget<T> {
    /// Get reference to element or subslice.
    fn rget<I: RSliceIndex<[T]>>(&self, index: I) -> Option<&I::Output>;

    /// Get mutable reference to element or subslice.
    fn rget_mut<I: RSliceIndex<[T]>>(&mut self, index: I) -> Option<&mut I::Output>;

    /// Get reference to element or subslice without bounds checking.
    unsafe fn rget_unchecked<I: RSliceIndex<[T]>>(&self, index: I) -> &I::Output;

    /// Get mutable reference to element or subslice without bounds checking.
    unsafe fn rget_unchecked_mut<I: RSliceIndex<[T]>>(&mut self, index: I) -> &mut I::Output;
}

impl<T> Rget<T> for [T] {
    fn rget<I: RSliceIndex<[T]>>(&self, index: I)
        -> Option<&I::Output>
    {
        index.rget(self)
    }

    fn rget_mut<I: RSliceIndex<[T]>>(&mut self, index: I)
        -> Option<&mut I::Output>
    {
        index.rget_mut(self)
    }

    unsafe fn rget_unchecked<I: RSliceIndex<[T]>>(&self, index: I)
        -> &I::Output
    {
        index.rget_unchecked(self)
    }

    unsafe fn rget_unchecked_mut<I: RSliceIndex<[T]>>(&mut self, index: I)
        -> &mut I::Output
    {
        index.rget_unchecked_mut(self)
    }
}

impl<T> Rget<T> for Vec<T> {
    fn rget<I: RSliceIndex<[T]>>(&self, index: I)
        -> Option<&I::Output>
    {
        index.rget(self.as_slice())
    }

    fn rget_mut<I: RSliceIndex<[T]>>(&mut self, index: I)
        -> Option<&mut I::Output>
    {
        index.rget_mut(self.as_mut_slice())
    }

    unsafe fn rget_unchecked<I: RSliceIndex<[T]>>(&self, index: I)
        -> &I::Output
    {
        index.rget_unchecked(self.as_slice())
    }

    unsafe fn rget_unchecked_mut<I: RSliceIndex<[T]>>(&mut self, index: I)
        -> &mut I::Output
    {
        index.rget_unchecked_mut(self.as_mut_slice())
    }
}

impl<A: stackvector::Array> Rget<A::Item> for stackvector::StackVec<A> {
    fn rget<I: RSliceIndex<[A::Item]>>(&self, index: I)
        -> Option<&I::Output>
    {
        index.rget(self.as_slice())
    }

    fn rget_mut<I: RSliceIndex<[A::Item]>>(&mut self, index: I)
        -> Option<&mut I::Output>
    {
        index.rget_mut(self.as_mut_slice())
    }

    unsafe fn rget_unchecked<I: RSliceIndex<[A::Item]>>(&self, index: I)
        -> &I::Output
    {
        index.rget_unchecked(self.as_slice())
    }

    unsafe fn rget_unchecked_mut<I: RSliceIndex<[A::Item]>>(&mut self, index: I)
        -> &mut I::Output
    {
        index.rget_unchecked_mut(self.as_mut_slice())
    }
}

// VECLIKE

/// Vector-like container.
///
/// Implemented for Vec, SmallVec, and StackVec.
pub trait VecLike<T>:
    iter::FromIterator<T> +
    iter::IntoIterator +
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
    AsSlice<T> +
    Extend<T> +
    Len<T> +
    Rget<T> +
    Default
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
        self.rget(0)
    }

    /// Get an mutable reference to the back item.
    #[inline(always)]
    fn back_mut(&mut self) -> Option<&mut T> {
        self.rget_mut(0)
    }

    /// Get an immutable reference to the back item.
    #[inline(always)]
    unsafe fn back_unchecked(&self) -> &T {
        self.rget_unchecked(0)
    }

    /// Get an mutable reference to the back item.
    #[inline(always)]
    unsafe fn back_unchecked_mut(&mut self) -> &mut T {
        self.rget_unchecked_mut(0)
    }
}

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

// CLONEABLE VECLIKE

/// Vector-like container with cloneable values.
///
/// Implemented for Vec, SmallVec, and StackVec.
pub trait CloneableVecLike<T: Clone + Copy + Send>:
    Send +
    ExtendFromSlice<T> +
    Resize<T> +
    Reserve<T> +
    ReserveExact<T> +
    VecLike<T>
{}

impl<T: Clone + Copy + Send> CloneableVecLike<T> for Vec<T> {
}

impl<A: stackvector::Array> CloneableVecLike<A::Item> for stackvector::StackVec<A>
    where A::Item: Clone + Copy + Send
{}

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
