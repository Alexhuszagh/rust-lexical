//! Iteration utilities for data interfaces.

use crate::lib::slice;

#[cfg(feature = "format")]
use crate::util::*;

// Type for iteration without any digit separators.
pub(super) type IteratorNoSeparator<'a> = slice::Iter<'a, u8>;

// Iterate without any skipping any digit separators.
perftools_inline!{
pub(super) fn iterate_no_separator<'a>(bytes: &'a [u8], _: u8)
    -> IteratorNoSeparator<'a>
{
    bytes.iter()
}}

// Type for iteration with a digit separator.
#[cfg(feature = "format")]
pub(super) type IteratorSeparator<'a> = SkipValueIterator<'a, u8>;

// Iterate while skipping digit separators.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn iterate_separator<'a>(bytes: &'a [u8], digit_separator: u8)
    -> IteratorSeparator<'a>
{
    IteratorSeparator::new(bytes, digit_separator)
}}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_no_separator_test() {
        assert!(iterate_no_separator(b"01", b'\x00').eq(b"01".iter()));
        assert!(iterate_no_separator(b"01_01", b'_').eq(b"01_01".iter()));
    }

    #[test]
    #[cfg(feature = "format")]
        fn iterate_separator_test() {
        assert!(iterate_separator(b"01", b'_').eq(b"01".iter()));
        assert!(iterate_separator(b"01_01", b'_').eq(b"0101".iter()));
    }
}
