//! Configuration options for parsing integers.

// TODO(ahuszagh) Only need crate::util::error
use crate::util::OptionsError;

use super::number::*;

// PARSE INTEGER
// -------------

/// Builder for `ParseIntegerOptions`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ParseIntegerOptionsBuilder {
    /// Number format.
    format: Option<NumberFormatV2>,
}

impl ParseIntegerOptionsBuilder {
    /// Create new, default builder.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            format: None,
        }
    }

    // GETTERS

    /// Get the number format.
    #[inline(always)]
    pub const fn get_format(&self) -> Option<NumberFormatV2> {
        self.format
    }

    // SETTERS

    /// Set the format specifier for ParseIntegerOptionsBuilder.
    #[inline(always)]
    pub const fn format(mut self, format: Option<NumberFormatV2>) -> Self {
        self.format = format;
        self
    }

    // BUILDERS

    const_fn!(
    /// Build the ParseIntegerOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Result<ParseIntegerOptions, OptionsError> {
        Ok(ParseIntegerOptions {
            format: self.format,
        })
    });
}

impl Default for ParseIntegerOptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Options to customize parsing integers.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::ParseIntegerOptions;
///
/// # pub fn main() {
/// let options = ParseIntegerOptions::builder()
///     .build()
///     .unwrap();
/// # }
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ParseIntegerOptions {
    /// Number format.
    format: Option<NumberFormatV2>,
}

impl ParseIntegerOptions {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            format: None,
        }
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn binary() -> Self {
        let mut format = NumberFormatV2::STANDARD;
        format.lexer.radix = 2;
        Self {
            format: Some(format),
        }
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub const fn decimal() -> Self {
        Self {
            format: Some(NumberFormatV2::STANDARD),
        }
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn hexadecimal() -> Self {
        let mut format = NumberFormatV2::STANDARD;
        format.lexer.radix = 16;
        Self {
            format: Some(format),
        }
    }

    // GETTERS

    /// Get the number format.
    #[inline(always)]
    pub const fn format(&self) -> Option<NumberFormatV2> {
        self.format
    }

    // SETTERS

    /// Set the number format.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_format(&mut self, format: Option<NumberFormatV2>) {
        self.format = format
    }

    // BUILDERS

    /// Get ParseIntegerOptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> ParseIntegerOptionsBuilder {
        ParseIntegerOptionsBuilder::new()
    }

    /// Create ParseIntegerOptionsBuilder using existing values.
    pub const fn rebuild(self) -> ParseIntegerOptionsBuilder {
        ParseIntegerOptionsBuilder {
            format: self.format,
        }
    }
}

impl Default for ParseIntegerOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
