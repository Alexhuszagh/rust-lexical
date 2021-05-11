//! Configuration options for parsing floats.

// TODO(ahuszagh) Only need crate::util::error, misc
use crate::util::{OptionsError, OptionsErrorCode, RoundingKind};

use super::config::*;
use super::number::*;
use super::validate::*;

// PARSE FLOAT
// -----------

/// Builder for `ParseFloatOptions`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ParseFloatOptionsBuilder {
    /// Number format.
    format: NumberFormatV2,
    /// Rounding kind for float.
    rounding: RoundingKind,
    /// Use the incorrect, fast parser.
    incorrect: bool,
    /// Use the lossy, intermediate parser.
    lossy: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
    /// Long string representation of `Infinity`.
    infinity_string: &'static [u8],
}

impl ParseFloatOptionsBuilder {
    /// Create new, default builder.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            format: DEFAULT_FORMAT,
            rounding: DEFAULT_ROUNDING,
            incorrect: DEFAULT_INCORRECT,
            lossy: DEFAULT_LOSSY,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
        }
    }

    // GETTERS

    /// Get the number format.
    #[inline(always)]
    pub const fn get_format(&self) -> NumberFormatV2 {
        self.format
    }

    /// Get the rounding kind for float.
    #[inline(always)]
    pub const fn get_rounding(&self) -> RoundingKind {
        self.rounding
    }

    /// Get if we use the incorrect, fast parser.
    #[inline(always)]
    pub const fn get_incorrect(&self) -> bool {
        self.incorrect
    }

    /// Get if we use the lossy, fast parser.
    #[inline(always)]
    pub const fn get_lossy(&self) -> bool {
        self.lossy
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

    /// Get the long string representation for `Infinity`.
    #[inline(always)]
    pub const fn get_infinity_string(&self) -> &'static [u8] {
        self.infinity_string
    }

    // SETTERS

    const_fn!(
    /// Set the format specifier for ParseFloatOptionsBuilder.
    #[inline(always)]
    pub const fn format(mut self, format: Option<NumberFormatV2>) -> Self {
        self.format = match format {
            Some(format) => format,
            None => DEFAULT_FORMAT,
        };
        self
    });

    /// Set the rounding kind for ParseFloatOptionsBuilder.
    #[inline(always)]
    #[cfg(feature = "rounding")]
    pub const fn rounding(mut self, rounding: RoundingKind) -> Self {
        self.rounding = rounding;
        self
    }

    /// Set the parser to use the incorrect (fastest) algorithm.
    #[inline(always)]
    pub const fn incorrect(mut self, incorrect: bool) -> Self {
        self.incorrect = incorrect;
        self
    }

    /// Set the parser to use the lossy (intermediate) algorithm.
    #[inline(always)]
    pub const fn lossy(mut self, lossy: bool) -> Self {
        self.lossy = lossy;
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

    /// Set the long string representation for `Infinity`.
    #[inline(always)]
    pub const fn infinity_string(mut self, infinity_string: &'static [u8]) -> Self {
        self.infinity_string = infinity_string;
        self
    }

    // BUILDERS

    const_fn!(
    /// Build the ParseFloatOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Result<ParseFloatOptions, OptionsError> {
        let format = self.format;
        let rounding = self.rounding;
        let incorrect = self.incorrect;
        let lossy = self.lossy;
        let nan_string = to_nan_string!(self.nan_string);
        let inf_string = to_inf_string!(self.inf_string);
        let infinity_string = to_infinity_string!(self.infinity_string, self.inf_string);

        // Validate we can't use incorrect **and** lossy together.
        if self.incorrect && self.lossy {
            return Err(OptionsError {
                code: OptionsErrorCode::InvalidFloatParseAlgorithm,
            });
        }

        Ok(ParseFloatOptions {
            format,
            rounding,
            incorrect,
            lossy,
            nan_string,
            inf_string,
            infinity_string,
        })
    });
}

impl Default for ParseFloatOptionsBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Options to customize parsing floats.
///
/// # Examples
///
/// ```rust
/// # extern crate lexical_core;
/// use lexical_core::ParseFloatOptions;
///
/// # pub fn main() {
/// let options = ParseFloatOptions::builder()
///     .lossy(true)
///     .nan_string(b"NaN")
///     .inf_string(b"Inf")
///     .infinity_string(b"Infinity")
///     .build()
///     .unwrap();
/// # }
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ParseFloatOptions {
    /// Number format.
    format: NumberFormatV2,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
    /// Long string representation of `Infinity`.
    infinity_string: &'static [u8],
    /// Rounding kind for float.
    rounding: RoundingKind,
    /// Use the incorrect, fast parser.
    incorrect: bool,
    /// Use the lossy, intermediate parser.
    lossy: bool,
}

impl ParseFloatOptions {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            format: DEFAULT_FORMAT,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
            rounding: DEFAULT_ROUNDING,
            incorrect: DEFAULT_INCORRECT,
            lossy: DEFAULT_LOSSY,
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
            format,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
            rounding: DEFAULT_ROUNDING,
            incorrect: DEFAULT_INCORRECT,
            lossy: DEFAULT_LOSSY,
        }
    }

    /// Create new options to write the default decimal format.
    #[inline(always)]
    pub const fn decimal() -> Self {
        Self {
            format: DEFAULT_FORMAT,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
            rounding: DEFAULT_ROUNDING,
            incorrect: DEFAULT_INCORRECT,
            lossy: DEFAULT_LOSSY,
        }
    }

    /// Create new options to write the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "power_of_two")]
    pub const fn hexadecimal() -> Self {
        let mut format = NumberFormatV2::STANDARD;
        format.lexer.radix = 16;
        Self {
            format,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
            rounding: DEFAULT_ROUNDING,
            incorrect: DEFAULT_INCORRECT,
            lossy: DEFAULT_LOSSY,
        }
    }

    // GETTERS

    /// Get the number format.
    #[inline(always)]
    pub const fn format(&self) -> NumberFormatV2 {
        self.format
    }

    /// Get the rounding kind for float.
    #[inline(always)]
    pub const fn rounding(&self) -> RoundingKind {
        self.rounding
    }

    /// Get if we use the incorrect, fast parser.
    #[inline(always)]
    pub const fn incorrect(&self) -> bool {
        self.incorrect
    }

    /// Get if we use the lossy, fast parser.
    #[inline(always)]
    pub const fn lossy(&self) -> bool {
        self.lossy
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

    /// Get the long string representation for `Infinity`.
    #[inline(always)]
    pub const fn infinity_string(&self) -> &'static [u8] {
        self.infinity_string
    }

    // NUMBER FORMAT

    /// Get the radix for the significant digits.
    #[inline(always)]
    pub const fn mantissa_radix(&self) -> u32 {
        self.format.mantissa_radix() as u32
    }

    /// Get the exponent base.
    #[inline(always)]
    pub const fn exponent_base(&self) -> u32 {
        match self.format.exponent_base() {
            Some(radix) => radix.get() as u32,
            _ => self.mantissa_radix(),
        }
    }

    /// Get the exponent radix.
    #[inline(always)]
    pub const fn exponent_radix(&self) -> u32 {
        match self.format.exponent_radix() {
            Some(radix) => radix.get() as u32,
            _ => self.mantissa_radix(),
        }
    }

    /// Get the digit separator character.
    #[inline(always)]
    pub const fn digit_separator(&self) -> u8 {
        self.format.digit_separator()
    }

    /// Get the decimal point character.
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        self.format.decimal_point()
    }

    const_fn!(
    /// Get the exponent character.
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        self.format.exponent()
    });

    // SETTERS

    /// Set the number format.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_format(&mut self, format: NumberFormatV2) {
        self.format = format
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

    /// Set the long string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_infinity_string(&mut self, infinity_string: &'static [u8]) {
        self.infinity_string = infinity_string
    }

    /// Set the rounding kind.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_rounding(&mut self, rounding: RoundingKind) {
        self.rounding = rounding
    }

    /// Set if we use the incorrect, fast parser.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_incorrect(&mut self, incorrect: bool) {
        self.incorrect = incorrect;
    }

    /// Set if we use the lossy, intermediate parser.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_lossy(&mut self, lossy: bool) {
        self.lossy = lossy;
    }

    // BUILDERS

    /// Get ParseFloatOptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> ParseFloatOptionsBuilder {
        ParseFloatOptionsBuilder::new()
    }

    /// Create ParseFloatOptionsBuilder using existing values.
    pub const fn rebuild(self) -> ParseFloatOptionsBuilder {
        ParseFloatOptionsBuilder {
            format: self.format,
            rounding: self.rounding(),
            incorrect: self.incorrect(),
            lossy: self.lossy(),
            nan_string: self.nan_string,
            inf_string: self.inf_string,
            infinity_string: self.infinity_string,
        }
    }
}

impl Default for ParseFloatOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
