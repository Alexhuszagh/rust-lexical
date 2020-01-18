//! Iteration utilities for data interfaces.

use crate::lib::slice;

// Type for iteration without any digit separators.
pub(super) type IteratorNoSeparator<'a> = slice::Iter<'a, u8>;

// Iterate without any skipping any digit separators.
perftools_inline!{
pub(super) fn iterate_no_separator<'a>(bytes: &'a [u8]) -> IteratorNoSeparator<'a> {
    bytes.iter()
}}

// TODO(ahuszagh) I should probably think this through.
//  No consecutive separators:
//      Easy, just check bytes[i+1] !== separator if that's the case.
//  No trailing separators:
//      Make sure i+1 is a valid digit.
//  No leading separators:
//      Make sure i != 0.

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ltrim_no_separator_test() {
        assert!(iterate_no_separator(b"01").eq(b"01".iter()));
    }
}
