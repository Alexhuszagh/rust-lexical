//! Macro to check if the radix is valid, in generic code.

// RADIX

/// Check radix is in range [2, 36] in debug builds.
#[cfg(feature = "radix")]
macro_rules! debug_assert_radix {
    ($radix:expr) => {
        debug_assert!($radix >= 2 && $radix <= 36, "Numerical base must be from 2-36.");
    };
}

/// Check radix is is 10 or a power of 2.
#[cfg(all(feature = "power_of_two", not(feature = "radix")))]
macro_rules! debug_assert_radix {
    ($radix:expr) => {
        debug_assert!(
            match $radix {
                2 | 4 | 8 | 10 | 16 | 32 => true,
                _ => false,
            },
            "Numerical base must be from 2-36."
        );
    };
}

/// Check radix is equal to 10.
#[cfg(not(feature = "power_of_two"))]
macro_rules! debug_assert_radix {
    ($radix:expr) => {
        debug_assert!($radix == 10, "Numerical base must be 10.");
    };
}

// BUFFER

/// Check the buffer has sufficient room for the output.
macro_rules! assert_buffer {
    ($radix:expr, $slc:ident, $t:ty) => {{
        #[cfg(feature = "power_of_two")]
        match $radix {
            10 => assert!($slc.len() >= <$t>::FORMATTED_SIZE_DECIMAL),
            _ => assert!($slc.len() >= <$t>::FORMATTED_SIZE),
        }

        #[cfg(not(feature = "power_of_two"))]
        assert!($slc.len() >= <$t>::FORMATTED_SIZE);
    }};
}
