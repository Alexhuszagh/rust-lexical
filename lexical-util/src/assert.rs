//! Debugging assertions to check a radix is valid.

#[cfg(feature = "write")]
use crate::constants::FormattedSize;

// RADIX

/// Check radix is in range [2, 36] in debug builds.
#[inline]
#[cfg(feature = "radix")]
pub fn debug_assert_radix(radix: u32) {
    debug_assert!((2..=36).contains(&radix), "Numerical base must be from 2-36.");
}

/// Check radix is is 10 or a power of 2.
#[inline]
#[cfg(all(feature = "power-of-two", not(feature = "radix")))]
pub fn debug_assert_radix(radix: u32) {
    debug_assert!(
        match radix {
            2 | 4 | 8 | 10 | 16 | 32 => true,
            _ => false,
        },
        "Numerical base must be from 2-36."
    );
}

/// Check radix is equal to 10.
#[inline]
#[cfg(not(feature = "power-of-two"))]
pub fn debug_assert_radix(radix: u32) {
    debug_assert!(radix == 10, "Numerical base must be 10.");
}

// BUFFER

/// Debug assertion the buffer has sufficient room for the output.
#[inline]
#[cfg(feature = "write")]
pub fn debug_assert_buffer<T: FormattedSize>(radix: u32, len: usize) {
    debug_assert!(
        match radix {
            10 => len >= T::FORMATTED_SIZE_DECIMAL,
            _ => len >= T::FORMATTED_SIZE,
        },
        "Buffer is too small: may overwrite buffer in release builds."
    );
}

/// Assertion the buffer has sufficient room for the output.
#[inline]
#[cfg(all(feature = "power-of-two", feature = "write"))]
pub fn assert_buffer<T: FormattedSize>(radix: u32, len: usize) {
    assert!(
        match radix {
            10 => len >= T::FORMATTED_SIZE_DECIMAL,
            _ => len >= T::FORMATTED_SIZE,
        },
        "Buffer is too small: may overwrite buffer in release builds."
    );
}

/// Assertion the buffer has sufficient room for the output.
#[inline]
#[cfg(all(not(feature = "power-of-two"), feature = "write"))]
pub fn assert_buffer<T: FormattedSize>(_: u32, len: usize) {
    assert!(
        len >= T::FORMATTED_SIZE_DECIMAL,
        "Buffer is too small: may overwrite buffer in release builds."
    );
}
