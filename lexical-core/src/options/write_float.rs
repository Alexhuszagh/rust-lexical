//! Configuration options for writing floats.

// TODO(ahuszagh) Only need crate::util::error
use crate::util::{OptionsError, OptionsErrorCode};

use super::config::*;
use super::number::*;
use super::validate::*;

// WRITE FLOAT
// -----------

/// Builder for `WriteFloatOptions`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WriteFloatOptionsBuilder {
    /// Number format.
    format: Option<NumberFormatV2>,
    /// Maximum number of significant digits to write.
    /// If not set, it defaults to the formatter's default.
    max_significant_digits: OptionUsize,
    /// Minimum number of significant digits to write.
    /// If not set, it defaults to the formatter's default.
    min_significant_digits: OptionUsize,
    /// Trim the trailing ".0" from integral float strings.
    trim_floats: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
}

impl WriteFloatOptionsBuilder {
    /// Create new, default builder.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            format: None,
            max_significant_digits: None,
            min_significant_digits: None,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    // GETTERS

    /// Get the number format.
    #[inline(always)]
    pub const fn get_format(&self) -> Option<NumberFormatV2> {
        self.format
    }

    /// Get the maximum number of significant digits to write.
    #[inline(always)]
    pub const fn get_max_significant_digits(&self) -> OptionUsize {
        self.max_significant_digits
    }

    /// Get the minimum number of significant digits to write.
    #[inline(always)]
    pub const fn get_min_significant_digits(&self) -> OptionUsize {
        self.min_significant_digits
    }

    /// Get if we should trim a trailing `".0"` from floats.
    #[inline(always)]
    pub const fn get_trim_floats(&self) -> bool {
        self.trim_floats
    }

    /// Get the string representation for `NaN`.
    #[inline(always)]
    pub const fn get_nan_string(&self) -> &'static [u8] {
        self.nan_string
    }

    /// Get the short string representation for `Infinity`.
    #[inline(always)]
    pub const fn get_inf_string(&self) -> &'static [u8] {
        self.inf_string
    }

    //  SETTERS

    /// Set the format specifier for WriteFloatOptionsBuilder.
    #[inline(always)]
    pub const fn format(mut self, format: Option<NumberFormatV2>) -> Self {
        self.format = format;
        self
    }

    /// Set the maximum number of significant digits to write.
    #[inline(always)]
    pub const fn max_significant_digits(mut self, max_significant_digits: OptionUsize) -> Self {
        self.max_significant_digits = max_significant_digits;
        self
    }

    /// Set the minimum number of significant digits to write.
    #[inline(always)]
    pub const fn min_significant_digits(mut self, min_significant_digits: OptionUsize) -> Self {
        self.min_significant_digits = min_significant_digits;
        self
    }

    /// Set if we should trim a trailing `".0"` from floats.
    #[inline(always)]
    pub const fn trim_floats(mut self, trim_floats: bool) -> Self {
        self.trim_floats = trim_floats;
        self
    }

    /// Set the string representation for `NaN`.
    #[inline(always)]
    pub const fn nan_string(mut self, nan_string: &'static [u8]) -> Self {
        self.nan_string = nan_string;
        self
    }

    /// Set the short string representation for `Infinity`.
    #[inline(always)]
    pub const fn inf_string(mut self, inf_string: &'static [u8]) -> Self {
        self.inf_string = inf_string;
        self
    }

    // BUILDERS

    const_fn!(
    /// Build the ParseFloatOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Result<WriteFloatOptions, OptionsError> {
        let format = self.format;
        let trim_floats = self.trim_floats;
        let max_significant_digits = self.max_significant_digits;
        let min_significant_digits = self.min_significant_digits;
        let nan_string = to_nan_string!(self.nan_string);
        let inf_string = to_inf_string!(self.inf_string);

        Ok(WriteFloatOptions {
            format,
            trim_floats,
            max_significant_digits,
            min_significant_digits,
            nan_string,
            inf_string,
        })
    });
}

impl Default for WriteFloatOptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Options to customize writing floats.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::WriteFloatOptions;
///
/// # pub fn main() {
/// let options = WriteFloatOptions::builder()
///     .trim_floats(true)
///     .nan_string(b"NaN")
///     .inf_string(b"Inf")
///     .build()
///     .unwrap();
/// # }
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WriteFloatOptions {
    /// Number format.
    format: Option<NumberFormatV2>,
    /// Maximum number of significant digits allowed.
    /// If not set, it defaults to the formatter's default.
    max_significant_digits: OptionUsize,
    /// Minimum number of significant digits allowed.
    /// If not set, it defaults to the formatter's default.
    min_significant_digits: OptionUsize,
    /// Trim the trailing ".0" from integral float strings.
    trim_floats: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
}

impl WriteFloatOptions {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            format: None,
            max_significant_digits: None,
            min_significant_digits: None,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to write the default binary format.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn binary() -> Self {
        let mut format = NumberFormatV2::STANDARD;
        format.lexer.radix = 2;
        Self {
            format: format,
            max_significant_digits: None,
            min_significant_digits: None,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    /// Create new options to write the default decimal format.
    #[inline(always)]
    pub const fn decimal() -> Self {
        Self {
            format: None,
            max_significant_digits: None,
            min_significant_digits: None,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    /// Create new options to write the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn hexadecimal() -> Self {
        let mut format = NumberFormatV2::STANDARD;
        format.lexer.radix = 16;
        Self {
            format: format,
            max_significant_digits: None,
            min_significant_digits: None,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    // GETTERS

    /// Get the number format.
    #[inline(always)]
    pub const fn format(&self) -> Option<NumberFormatV2> {
        self.format
    }

    /// Get if we should trim a trailing `".0"` from floats.
    #[inline(always)]
    pub const fn trim_floats(&self) -> bool {
        self.trim_floats
    }

    /// Get the string representation for `NaN`.
    #[inline(always)]
    pub const fn nan_string(&self) -> &'static [u8] {
        self.nan_string
    }

    /// Get the short string representation for `Infinity`.
    #[inline(always)]
    pub const fn inf_string(&self) -> &'static [u8] {
        self.inf_string
    }

    const_fn!(
    /// Get the radix for the significant digits.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        match self.format {
            Some(format) => format.mantissa_radix() as u32,
            None => DEFAULT_RADIX as u32,
        }
    });

    const_fn!(
    /// Get the digit separator character.
    #[inline(always)]
    pub const fn digit_separator(&self) -> u8 {
        match self.format {
            Some(format) => format.digit_separator(),
            None => b'\x00',
        }
    });

    const_fn!(
    /// Get the decimal point character.
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        match self.format {
            Some(format) => format.decimal_point(),
            None => DEFAULT_DECIMAL_POINT,
        }
    });

    const_fn!(
    /// Get the exponent character.
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        // Const fn version of unwrap_or().
        match self.format {
            Some(format) => format.exponent(),
            None => DEFAULT_EXPONENT,
        }
    });

    // SETTERS

    /// Set the number format.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_format(&mut self, format: Option<NumberFormatV2>) {
        self.format = format
    }

    /// Set the maximum number of significant digits to write.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_max_significant_digits(&mut self, max_significant_digits: OptionUsize) {
        self.max_significant_digits = max_significant_digits
    }

    /// Set the minimum number of significant digits to write.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_min_significant_digits(&mut self, min_significant_digits: OptionUsize) {
        self.min_significant_digits = min_significant_digits
    }

    /// Set if we should trim a trailing `".0"` from floats.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_trim_floats(&mut self, trim_floats: bool) {
        self.trim_floats = trim_floats;
    }

    /// Set the string representation for `NaN`.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_nan_string(&mut self, nan_string: &'static [u8]) {
        self.nan_string = nan_string
    }

    /// Set the short string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_inf_string(&mut self, inf_string: &'static [u8]) {
        self.inf_string = inf_string
    }

    // BUILDERS

    /// Get WriteFloatOptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> WriteFloatOptionsBuilder {
        WriteFloatOptionsBuilder::new()
    }

    /// Create WriteFloatOptionsBuilder using existing values.
    pub const fn rebuild(self) -> WriteFloatOptionsBuilder {
        WriteFloatOptionsBuilder {
            format: self.format,
            max_significant_digits: self.max_significant_digits,
            min_significant_digits: self.min_significant_digits,
            trim_floats: self.trim_floats,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
        }
    }
}

impl Default for WriteFloatOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
