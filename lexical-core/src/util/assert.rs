//! Macro to check if the radix is valid, in generic code.

#[cfg(feature = "radix")]
macro_rules! debug_assert_radix {
    ($radix:expr) => (debug_assert!($radix.as_i32() >= 2 && $radix.as_i32() <= 36, "Numerical base must be from 2-36.");)
}

#[cfg(not(feature = "radix"))]
macro_rules! debug_assert_radix {
    ($radix:expr) => (debug_assert!($radix.as_i32() == 10, "Numerical base must be 10.");)
}

#[cfg(feature = "radix")]
macro_rules! assert_radix {
    ($radix:expr) => (assert!($radix.as_i32() >= 2 && $radix.as_i32() <= 36, "Numerical base must be from 2-36.");)
}

#[cfg(not(feature = "radix"))]
macro_rules! assert_radix {
    ($radix:expr) => (assert!($radix.as_i32() == 10, "Numerical base must be 10.");)
}
