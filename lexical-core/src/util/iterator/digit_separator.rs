//! Iterators for handling digit separators.

use super::slice::*;
#[cfg(feature = "format")]
use super::skip::*;

// ALIAS
// -----

/// Type for iteration without any digit separators.
pub(crate) type IterN<'a> = SliceIterator<'a, u8>;

/// Iterate without any skipping any digit separators.
#[inline(always)]
pub(crate) fn to_iter_n<'a>(bytes: &'a [u8], _: u8) -> IterN<'a> {
    IterN::new(bytes)
}

/// Type for iteration with a digit separator.
#[cfg(feature = "format")]
pub(crate) type IterS<'a> = SkipValueIterator<'a, u8>;

/// Iterate while skipping digit separators.
#[inline(always)]
#[cfg(feature = "format")]
pub(crate) fn to_iter_s<'a>(bytes: &'a [u8], digit_separator: u8) -> IterS<'a> {
    IterS::new(bytes, digit_separator)
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_iter_n_test() {
        assert!(to_iter_n(b"01", b'\x00').eq(b"01".iter()));
        assert!(to_iter_n(b"01_01", b'_').eq(b"01_01".iter()));
    }

    #[test]
    #[cfg(feature = "format")]
    fn to_iter_s_test() {
        assert!(to_iter_s(b"01", b'_').eq(b"01".iter()));
        assert!(to_iter_s(b"01_01", b'_').eq(b"0101".iter()));
    }
}
