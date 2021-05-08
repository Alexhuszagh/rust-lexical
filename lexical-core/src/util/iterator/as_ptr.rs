//! Iterator over contiguous data that can be converted to a pointer.

use crate::lib::slice;

/// Get access to a raw, const pointer from the underlying data.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return null, or be implemented
/// for non-contiguous data.
pub(crate) trait AsPtrIterator<'a, T: 'a>: Iterator<Item = &'a T> {
    /// Get raw pointer from iterator state.
    fn as_ptr(&self) -> *const T;
    /// Peek the next value
    fn peek(&mut self) -> Option<Self::Item>;
}

impl<'a, T> AsPtrIterator<'a, T> for slice::Iter<'a, T> {
    #[inline]
    fn as_ptr(&self) -> *const T {
        self.as_slice().as_ptr()
    }

    #[inline]
    fn peek(&mut self) -> Option<&'a T> {
        self.as_slice().get(0)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_ptr_iterator_test() {
        let digits = b"12345";
        let mut iter = digits.iter();
        assert_eq!(iter.as_ptr(), digits.as_ptr());
        assert_eq!(iter.nth(4).unwrap(), &b'5');
        assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());
    }
}
