//! Debugging assertions to check a radix is valid.

// RADIX

/// Check radix is in range [2, 36] in debug builds.
#[inline]
#[cfg(feature = "radix")]
pub fn debug_assert_radix(radix: u32) {
    debug_assert!(radix >= 2 && radix <= 36, "Numerical base must be from 2-36.");
}

/// Check radix is is 10 or a power of 2.
#[inline]
#[cfg(all(feature = "power_of_two", not(feature = "radix")))]
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
#[cfg(not(feature = "power_of_two"))]
pub fn debug_assert_radix(radix: u32) {
    debug_assert!(radix == 10, "Numerical base must be 10.");
}
