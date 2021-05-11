//! Configuration options for writing integers.

// TODO(ahuszagh) Only need crate::util::error
use crate::util::{OptionsError, OptionsErrorCode};

use super::config::*;
use super::validate::*;

// WRITE INTEGER
// -------------

/// Builder for `WriteIntegerOptions`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WriteIntegerOptionsBuilder {
    radix: u8,
}

impl WriteIntegerOptionsBuilder {
    #[inline(always)]
    pub const fn new() -> WriteIntegerOptionsBuilder {
        WriteIntegerOptionsBuilder {
            radix: DEFAULT_RADIX,
        }
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn get_radix(&self) -> u8 {
        self.radix
    }

    // SETTERS

    /// Set the radix for WriteIntegerOptionsBuilder.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    // BUILDERS

    const_fn!(
    /// Build the WriteIntegerOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Result<WriteIntegerOptions, OptionsError> {
        if !is_valid_radix(self.radix) {
            return Err(OptionsError {
                code: OptionsErrorCode::InvalidRadix,
            });
        }
        let radix = self.radix as u32;
        Ok(WriteIntegerOptions {
            radix,
        })
    });
}

impl Default for WriteIntegerOptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Immutable options to customize writing integers.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::WriteIntegerOptions;
///
/// # pub fn main() {
/// let options = WriteIntegerOptions::builder()
///     .build()
///     .unwrap();
/// # }
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WriteIntegerOptions {
    /// Radix for integer string.
    radix: u32,
}

impl WriteIntegerOptions {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            radix: DEFAULT_RADIX as u32,
        }
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to write the default binary format.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn binary() -> Self {
        Self {
            radix: 2,
        }
    }

    /// Create new options to write the default decimal format.
    #[inline(always)]
    pub const fn decimal() -> Self {
        Self {
            radix: 10,
        }
    }

    /// Create new options to write the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn hexadecimal() -> Self {
        Self {
            radix: 16,
        }
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.radix
    }

    // SETTERS

    /// Set the radix.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_radix(&mut self, radix: u32) {
        self.radix = radix;
    }

    // BUILDERS

    /// Get WriteIntegerOptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> WriteIntegerOptionsBuilder {
        WriteIntegerOptionsBuilder::new()
    }

    /// Create WriteIntegerOptionsBuilder using existing values.
    pub const fn rebuild(self) -> WriteIntegerOptionsBuilder {
        WriteIntegerOptionsBuilder {
            radix: self.radix as u8,
        }
    }
}

impl Default for WriteIntegerOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
