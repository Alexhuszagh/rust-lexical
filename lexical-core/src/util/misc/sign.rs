//! Enumerations for the sign of a number.

/// Enumeration for the sign of a a number.
#[doc(hidden)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sign {
    /// Negative value.
    Negative,
    /// Positive value.
    Positive,
}
