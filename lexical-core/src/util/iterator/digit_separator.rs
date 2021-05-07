//! Iterators for handling digit separators.

use crate::lib::slice;

#[cfg(feature = "format")]
use super::skip::*;

// Type for iteration without any digit separators.
pub(crate) type IteratorNoSeparator<'a> = slice::Iter<'a, u8>;

// Iterate without any skipping any digit separators.
#[inline(always)]
pub(crate) fn iterate_digits_no_separator<'a>(bytes: &'a [u8], _: u8) -> IteratorNoSeparator<'a> {
    bytes.iter()
}

// Type for iteration with a digit separator.
#[cfg(feature = "format")]
pub(crate) type IteratorSeparator<'a> = SkipValueIterator<'a, u8>;

// Iterate while skipping digit separators.
#[inline(always)]
#[cfg(feature = "format")]
pub(crate) fn iterate_digits_ignore_separator<'a>(
    bytes: &'a [u8],
    digit_separator: u8,
) -> IteratorSeparator<'a> {
    IteratorSeparator::new(bytes, digit_separator)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_digits_no_separator_test() {
        assert!(iterate_digits_no_separator(b"01", b'\x00').eq(b"01".iter()));
        assert!(iterate_digits_no_separator(b"01_01", b'_').eq(b"01_01".iter()));
    }

    #[test]
    #[cfg(feature = "format")]
    fn iterate_digits_ignore_separator_test() {
        assert!(iterate_digits_ignore_separator(b"01", b'_').eq(b"01".iter()));
        assert!(iterate_digits_ignore_separator(b"01_01", b'_').eq(b"0101".iter()));
    }
}
