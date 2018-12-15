//! Slice-like and range-like types with an unbounded lifetime.

use lib::slice;
use super::algorithm::*;

/// Rust-style slice.
pub type Slice<T> = &'static [T];
pub type SliceIter<T> = slice::Iter<'static, T>;

/// Create unbounded slice from raw parts.
#[inline(always)]
pub unsafe fn from_raw_parts<T>(data: *const T, len: usize)
    -> &'static [T]
{
    slice::from_raw_parts(data, len)
}

/// Create unbounded mutable slice from raw parts.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn from_raw_parts_mut<T>(data: *mut T, len: usize)
    -> &'static mut [T]
{
    slice::from_raw_parts_mut(data, len)
}
