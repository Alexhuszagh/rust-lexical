// SliceIndex support for older Rust versions.

use lib::ops;
use lib::slice::{from_raw_parts, from_raw_parts_mut};
use super::pointer_methods::PointerMethods;

//  The compiler does not have SliceIndex defined, this is code
//  adopted from Rust:
//      https://doc.rust-lang.org/beta/src/core/slice/mod.rs.html
//
//  All copyright for the remainder of this file remains with the
//  Rust contributors:
//      https://github.com/rust-lang/rust/blob/master/COPYRIGHT

#[cfg(has_slice_index)]
pub use lib::slice::SliceIndex;

#[cfg(not(has_slice_index))]
mod private_slice_index {
    use lib::ops;
    pub trait Sealed {}

    impl Sealed for usize {}
    impl Sealed for ops::Range<usize> {}
    impl Sealed for ops::RangeTo<usize> {}
    impl Sealed for ops::RangeFrom<usize> {}
    impl Sealed for ops::RangeFull {}

    #[cfg(has_range_inclusive)]
    impl Sealed for ops::RangeInclusive<usize> {}

    #[cfg(has_range_inclusive)]
    impl Sealed for ops::RangeToInclusive<usize> {}
}

#[cfg(not(has_slice_index))]
#[inline(never)]
#[cold]
fn slice_index_len_fail(index: usize, len: usize) -> ! {
    panic!("index {} out of range for slice of length {}", index, len);
}

#[cfg(not(has_slice_index))]
#[inline(never)]
#[cold]
fn slice_index_order_fail(index: usize, end: usize) -> ! {
    panic!("slice index starts at {} but ends at {}", index, end);
}

#[cfg(all(not(has_slice_index), has_full_range_inclusive))]
#[inline(never)]
#[cold]
fn slice_index_overflow_fail() -> ! {
    panic!("attempted to index slice up to maximum usize");
}

#[cfg(not(has_slice_index))]
pub trait SliceIndex<T: ?Sized>: private_slice_index::Sealed {
    /// Output type.
    type Output: ?Sized;

    /// Get immutable reference to value(s).
    fn get(self, slice: &T) -> Option<&Self::Output>;

    /// Get mutable reference to value(s).
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;

    /// Get immutable reference to value(s) without bounds checking.
    unsafe fn get_unchecked(self, slice: &T) -> &Self::Output;

    /// Get mutable reference to value(s) without bounds checking.
    unsafe fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;

    /// Get immutable reference to value(s), panicking if out-of-bounds.
    fn index(self, slice: &T) -> &Self::Output;

    /// Get mutable reference to value(s), panicking if out-of-bounds.
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}

#[cfg(not(has_slice_index))]
impl<T> SliceIndex<[T]> for usize {
    type Output = T;

    #[inline]
    fn get(self, slice: &[T]) -> Option<&T> {
        if self < slice.len() {
            unsafe {
                Some(self.get_unchecked(slice))
            }
        } else {
            None
        }
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut T> {
        if self < slice.len() {
            unsafe {
                Some(self.get_unchecked_mut(slice))
            }
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &T {
        &*slice.as_ptr().padd(self)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut T {
        &mut *slice.as_mut_ptr().padd(self)
    }

    #[inline]
    fn index(self, slice: &[T]) -> &T {
        &(*slice)[self]
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut T {
        &mut (*slice)[self]
    }
}

#[cfg(not(has_slice_index))]
impl<T> SliceIndex<[T]> for  ops::Range<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        if self.start > self.end || self.end > slice.len() {
            None
        } else {
            unsafe {
                Some(self.get_unchecked(slice))
            }
        }
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        if self.start > self.end || self.end > slice.len() {
            None
        } else {
            unsafe {
                Some(self.get_unchecked_mut(slice))
            }
        }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] {
        from_raw_parts(slice.as_ptr().padd(self.start), self.end - self.start)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
        from_raw_parts_mut(slice.as_mut_ptr().padd(self.start), self.end - self.start)
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        if self.start > self.end {
            slice_index_order_fail(self.start, self.end);
        } else if self.end > slice.len() {
            slice_index_len_fail(self.end, slice.len());
        }
        unsafe {
            self.get_unchecked(slice)
        }
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        if self.start > self.end {
            slice_index_order_fail(self.start, self.end);
        } else if self.end > slice.len() {
            slice_index_len_fail(self.end, slice.len());
        }
        unsafe {
            self.get_unchecked_mut(slice)
        }
    }
}

#[cfg(not(has_slice_index))]
impl<T> SliceIndex<[T]> for ops::RangeTo<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        (0..self.end).get(slice)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        (0..self.end).get_mut(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] {
        (0..self.end).get_unchecked(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
        (0..self.end).get_unchecked_mut(slice)
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        (0..self.end).index(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        (0..self.end).index_mut(slice)
    }
}

#[cfg(not(has_slice_index))]
impl<T> SliceIndex<[T]> for ops::RangeFrom<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        (self.start..slice.len()).get(slice)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        (self.start..slice.len()).get_mut(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] {
        (self.start..slice.len()).get_unchecked(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
        (self.start..slice.len()).get_unchecked_mut(slice)
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        (self.start..slice.len()).index(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        (self.start..slice.len()).index_mut(slice)
    }
}

#[cfg(not(has_slice_index))]
impl<T> SliceIndex<[T]> for ops::RangeFull {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        Some(slice)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        Some(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] {
        slice
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
        slice
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        slice
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        slice
    }
}


#[cfg(all(not(has_slice_index), has_full_range_inclusive))]
impl<T> SliceIndex<[T]> for ops::RangeInclusive<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        if *self.end() == usize::max_value() { None }
        else { (*self.start()..self.end() + 1).get(slice) }
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        if *self.end() == usize::max_value() { None }
        else { (*self.start()..self.end() + 1).get_mut(slice) }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] {
        (*self.start()..self.end() + 1).get_unchecked(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
        (*self.start()..self.end() + 1).get_unchecked_mut(slice)
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        if *self.end() == usize::max_value() { slice_index_overflow_fail(); }
        (*self.start()..self.end() + 1).index(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        if *self.end() == usize::max_value() { slice_index_overflow_fail(); }
        (*self.start()..self.end() + 1).index_mut(slice)
    }
}

#[cfg(all(not(has_slice_index), has_full_range_inclusive))]
impl<T> SliceIndex<[T]> for ops::RangeToInclusive<usize> {
    type Output = [T];

    #[inline]
    fn get(self, slice: &[T]) -> Option<&[T]> {
        (0..=self.end).get(slice)
    }

    #[inline]
    fn get_mut(self, slice: &mut [T]) -> Option<&mut [T]> {
        (0..=self.end).get_mut(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &[T]) -> &[T] {
        (0..=self.end).get_unchecked(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut [T] {
        (0..=self.end).get_unchecked_mut(slice)
    }

    #[inline]
    fn index(self, slice: &[T]) -> &[T] {
        (0..=self.end).index(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut [T]) -> &mut [T] {
        (0..=self.end).index_mut(slice)
    }
}
