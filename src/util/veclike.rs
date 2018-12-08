//! Helper for vector-like classes.

#![allow(dead_code)]

use lib::{iter, ops, Vec};
use smallvec;
use stackvector;

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
    fn insert_many<I: iter::IntoIterator<Item=T>>(&mut self, index: usize, iterable: I) {
        self.splice(index..index, iterable);
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
    fn insert_many<I: iter::IntoIterator<Item=A::Item>>(&mut self, index: usize, iterable: I) {
        smallvec::SmallVec::insert_many(self, index, iterable)
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
    fn insert_many<I: iter::IntoIterator<Item=A::Item>>(&mut self, index: usize, iterable: I) {
        stackvector::StackVec::insert_many(self, index, iterable)
    }
}
