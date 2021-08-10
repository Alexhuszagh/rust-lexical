//! Shared traits for the options API.

#[cfg(feature = "write")]
use crate::constants::FormattedSize;

/// Shared trait for all writer options.
#[cfg(feature = "write")]
pub trait WriteOptions: Default {
    /// Determine if the options are valid.
    fn is_valid(&self) -> bool;

    /// Get an upper bound on the buffer size.
    fn buffer_size<T: FormattedSize, const FORMAT: u128>(&self) -> usize;
}

/// Shared trait for all parser options.
#[cfg(feature = "parse")]
pub trait ParseOptions: Default {
    /// Determine if the options are valid.
    fn is_valid(&self) -> bool;
}
