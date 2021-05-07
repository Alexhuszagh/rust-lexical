//! Iterator that can quickly detect if it can return more elements.

/// An iterator that knows if it has been fully consumed yet.
///
/// A consumed iterator will guarantee to return `None` for the next
/// value. It is effectively a weak variant of `is_empty()` on
/// `ExactSizeIterator`. When the length of an iterator is known,
/// `ConsumedIterator` will be implemented in terms of that length..
pub(crate) trait ConsumedIterator: Iterator {
    /// Return if the iterator has been consumed.
    fn consumed(&self) -> bool;
}

impl<T: ExactSizeIterator> ConsumedIterator for T {
    #[inline]
    fn consumed(&self) -> bool {
        self.len() == 0
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
}
