//! Helper traits for iterators.

use crate::lib::slice;

/// An iterator that knows if it has been fully consumed yet.
///
/// The `ConsumedIterator` does not need to be an `ExactSizedIterator`,
/// however, that drastically simplifies the process. All it needs to
/// know is by some logic whether the next value will return a value,
/// or not. A default implementation is provided for `ExactSizeIterator`.
pub(crate) trait ConsumedIterator: Iterator {
    /// Return if the iterator has been fully consumed.
    fn consumed(&self) -> bool;
}

impl<T: ExactSizeIterator> ConsumedIterator for T {
    fn consumed(&self) -> bool {
        self.len() == 0
    }
}

/// Get access to a raw pointer from the underlying data.
///
/// A default implementation is provided for slice iterators.
/// This trait **should never** return null or outside the bounds
/// of the current slice.
pub(crate) trait AsPtrIterator<'a, T: 'a>: Iterator<Item=&'a T> {
    /// Get raw pointer from iterator state.
    fn as_ptr(&self) -> *const T;
}

impl<'a, T> AsPtrIterator<'a, T> for slice::Iter<'a, T> {
    fn as_ptr(&self) -> *const T {
        self.as_slice().as_ptr()
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumer_iterator_test() {
        let mut iter = b"12345".iter();
        assert_eq!(iter.consumed(), false);
        assert_eq!(iter.nth(4).unwrap(), &b'5');
        assert_eq!(iter.consumed(), true);
    }

    #[test]
    fn as_ptr_iterator_test() {
        let digits = b"12345";
        let mut iter = digits.iter();
        assert_eq!(iter.as_ptr(), digits.as_ptr());
        assert_eq!(iter.nth(4).unwrap(), &b'5');
        assert_eq!(iter.as_ptr(), digits[digits.len()..].as_ptr());
    }
}
