//! Configuration options for writing floats.

use core::{mem, num};
use lexical_util::constants::FormattedSize;
use lexical_util::error::Error;
use lexical_util::result::Result;
use static_assertions::const_assert;

/// Type with the exact same size as a `usize`.
pub type OptionUsize = Option<num::NonZeroUsize>;

/// Type with the exact same size as a `i32`.
pub type OptionI32 = Option<num::NonZeroI32>;

// Ensure the sizes are identical.
const_assert!(mem::size_of::<OptionUsize>() == mem::size_of::<usize>());
const_assert!(mem::size_of::<OptionI32>() == mem::size_of::<i32>());

/// Enumeration for how to round floats with precision control.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RoundMode {
    /// Round to the nearest float string with the given number of significant digits.
    Round,
    /// Truncate the float string with the given number of significant digits.
    Truncate,
}

/// Maximum length for a special string.
const MAX_SPECIAL_STRING_LENGTH: usize = 50;
const_assert!(MAX_SPECIAL_STRING_LENGTH < f32::FORMATTED_SIZE_DECIMAL);

/// Builder for `Options`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OptionsBuilder {
    /// Maximum number of significant digits to write.
    /// If not set, it defaults to the algorithm's default.
    max_significant_digits: OptionUsize,
    /// Minimum number of significant digits to write.
    /// If not set, it defaults to the algorithm's default.
    min_significant_digits: OptionUsize,
    /// Maximum exponent prior to using scientific notation.
    /// This is ignored if the exponent base is not the same as the mantissa radix.
    /// If not provided, use the algorithm's default.
    positive_exponent_break: OptionI32,
    /// Minimum exponent prior to using scientific notation.
    /// This is ignored if the exponent base is not the same as the mantissa radix.
    /// If not provided, use the algorithm's default.
    negative_exponent_break: OptionI32,
    /// Rounding mode for writing digits with precision control.
    round_mode: RoundMode,
    /// Trim the trailing ".0" from integral float strings.
    trim_floats: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// String representation of `Infinity`.
    inf_string: &'static [u8],
}

impl OptionsBuilder {
    // CONSTRUCTORS

    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            max_significant_digits: None,
            min_significant_digits: None,
            positive_exponent_break: None,
            negative_exponent_break: None,
            round_mode: RoundMode::Round,
            trim_floats: false,
            nan_string: b"NaN",
            inf_string: b"inf",
        }
    }

    // GETTERS

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

    /// Get the maximum exponent prior to using scientific notation.
    #[inline(always)]
    pub const fn get_positive_exponent_break(&self) -> OptionI32 {
        self.positive_exponent_break
    }

    /// Get the minimum exponent prior to using scientific notation.
    #[inline(always)]
    pub const fn get_negative_exponent_break(&self) -> OptionI32 {
        self.negative_exponent_break
    }

    /// Get the rounding mode for writing digits with precision control.
    #[inline(always)]
    pub const fn get_round_mode(&self) -> RoundMode {
        self.round_mode
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

    // SETTERS

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

    /// Set the maximum exponent prior to using scientific notation.
    #[inline(always)]
    pub const fn positive_exponent_break(mut self, positive_exponent_break: OptionI32) -> Self {
        self.positive_exponent_break = positive_exponent_break;
        self
    }

    /// Set the minimum exponent prior to using scientific notation.
    #[inline(always)]
    pub const fn negative_exponent_break(mut self, negative_exponent_break: OptionI32) -> Self {
        self.negative_exponent_break = negative_exponent_break;
        self
    }

    /// Set the rounding mode for writing digits with precision control.
    #[inline(always)]
    pub const fn round_mode(mut self, round_mode: RoundMode) -> Self {
        self.round_mode = round_mode;
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

    /// Set the string representation for `Infinity`.
    #[inline(always)]
    pub const fn inf_string(mut self, inf_string: &'static [u8]) -> Self {
        self.inf_string = inf_string;
        self
    }

    // BUILDERS

    /// Determine if `nan_str` is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn nan_str_is_valid(&self) -> bool {
        if self.nan_string.is_empty() || self.nan_string.len() > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(self.nan_string[0], b'N' | b'n') {
            false
        } else {
            true
        }
    }

    /// Determine if `inf_str` is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn inf_str_is_valid(&self) -> bool {
        if self.inf_string.is_empty() || self.inf_string.len() > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(self.inf_string[0], b'I' | b'i') {
            false
        } else {
            true
        }
    }

    /// Check if the builder state is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn is_valid(&self) -> bool {
        if !self.nan_str_is_valid() {
            false
        } else if !self.inf_str_is_valid() {
            false
        } else {
            true
        }
    }

    /// Build the Options struct with bounds validation.
    ///
    /// # Safety
    ///
    /// Safe as long as `is_valid` is true. If `nan_string` and `inf_string`
    /// are too long, writing special floats may lead to severe buffer overflows,
    /// and therefore severe security vulnerabilities.
    #[inline(always)]
    pub const unsafe fn build_unchecked(&self) -> Options {
        Options {
            max_significant_digits: self.max_significant_digits,
            min_significant_digits: self.min_significant_digits,
            positive_exponent_break: self.positive_exponent_break,
            negative_exponent_break: self.negative_exponent_break,
            round_mode: self.round_mode,
            trim_floats: self.trim_floats,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
        }
    }

    /// Build the ParseFloatOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Result<Options> {
        let min_digits = unwrap_or_zero_usize(self.min_significant_digits);
        let max_digits = unwrap_or_max_usize(self.max_significant_digits);
        if self.nan_string.is_empty() || !matches!(self.nan_string[0], b'N' | b'n') {
            Err(Error::InvalidNanString)
        } else if self.nan_string.len() > MAX_SPECIAL_STRING_LENGTH {
            Err(Error::NanStringTooLong)
        } else if self.inf_string.is_empty() || !matches!(self.inf_string[0], b'I' | b'i') {
            Err(Error::InvalidInfString)
        } else if self.inf_string.len() > MAX_SPECIAL_STRING_LENGTH {
            Err(Error::InfStringTooLong)
        } else if max_digits < min_digits {
            Err(Error::InvalidFloatPrecision)
        } else if unwrap_or_zero_i32(self.negative_exponent_break) > 0 {
            Err(Error::InvalidNegativeExponentBreak)
        } else if unwrap_or_zero_i32(self.positive_exponent_break) < 0 {
            Err(Error::InvalidPositiveExponentBreak)
        } else {
            // SAFETY: always safe, since it must be valid.
            Ok(unsafe { self.build_unchecked() })
        }
    }
}

impl Default for OptionsBuilder {
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
/// # extern crate lexical_write_float;
/// use lexical_write_float::Options;
///
/// # pub fn main() {
/// let options = Options::builder()
///     .trim_floats(true)
///     .nan_string(b"NaN")
///     .inf_string(b"Inf")
///     .build()
///     .unwrap();
/// # }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Options {
    /// Maximum number of significant digits to write.
    /// If not set, it defaults to the algorithm's default.
    max_significant_digits: OptionUsize,
    /// Minimum number of significant digits to write.
    /// If not set, it defaults to the algorithm's default.
    min_significant_digits: OptionUsize,
    /// Maximum exponent prior to using scientific notation.
    /// This is ignored if the exponent base is not the same as the mantissa radix.
    /// If not provided, use the algorithm's default.
    positive_exponent_break: OptionI32,
    /// Minimum exponent prior to using scientific notation.
    /// This is ignored if the exponent base is not the same as the mantissa radix.
    /// If not provided, use the algorithm's default.
    negative_exponent_break: OptionI32,
    /// Rounding mode for writing digits with precision control.
    round_mode: RoundMode,
    /// Trim the trailing ".0" from integral float strings.
    trim_floats: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// String representation of `Infinity`.
    inf_string: &'static [u8],
}

impl Options {
    // CONSTRUCTORS

    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        // SAFETY: always safe since it uses the default arguments.
        unsafe { Self::builder().build_unchecked() }
    }

    // GETTERS

    /// Check if the options state is valid.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.rebuild().is_valid()
    }

    /// Get the maximum number of significant digits to write.
    #[inline(always)]
    pub const fn max_significant_digits(&self) -> OptionUsize {
        self.max_significant_digits
    }

    /// Get the minimum number of significant digits to write.
    #[inline(always)]
    pub const fn min_significant_digits(&self) -> OptionUsize {
        self.min_significant_digits
    }

    /// Get the maximum exponent prior to using scientific notation.
    #[inline(always)]
    pub const fn positive_exponent_break(&self) -> OptionI32 {
        self.positive_exponent_break
    }

    /// Get the minimum exponent prior to using scientific notation.
    #[inline(always)]
    pub const fn negative_exponent_break(&self) -> OptionI32 {
        self.negative_exponent_break
    }

    /// Get the rounding mode for writing digits with precision control.
    #[inline(always)]
    pub const fn round_mode(&self) -> RoundMode {
        self.round_mode
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

    // SETTERS

    /// Set the maximum number of significant digits to write.
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_max_significant_digits(&mut self, max_significant_digits: OptionUsize) {
        self.max_significant_digits = max_significant_digits
    }

    /// Set the minimum number of significant digits to write.
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_min_significant_digits(&mut self, min_significant_digits: OptionUsize) {
        self.min_significant_digits = min_significant_digits
    }

    /// Set the maximum exponent prior to using scientific notation.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_positive_exponent_break(&mut self, positive_exponent_break: OptionI32) {
        self.positive_exponent_break = positive_exponent_break;
    }

    /// Set the minimum exponent prior to using scientific notation.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_negative_exponent_break(&mut self, negative_exponent_break: OptionI32) {
        self.negative_exponent_break = negative_exponent_break;
    }

    /// Set the rounding mode for writing digits with precision control.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_round_mode(&mut self, round_mode: RoundMode) {
        self.round_mode = round_mode;
    }

    /// Set if we should trim a trailing `".0"` from floats.
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_trim_floats(&mut self, trim_floats: bool) {
        self.trim_floats = trim_floats;
    }

    /// Set the string representation for `NaN`.
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Unsafe if `nan_string.len() > MAX_SPECIAL_STRING_LENGTH`.
    #[inline(always)]
    pub unsafe fn set_nan_string(&mut self, nan_string: &'static [u8]) {
        self.nan_string = nan_string
    }

    /// Set the short string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Unsafe if `nan_string.len() > MAX_SPECIAL_STRING_LENGTH`.
    #[inline(always)]
    pub unsafe fn set_inf_string(&mut self, inf_string: &'static [u8]) {
        self.inf_string = inf_string
    }

    // BUILDERS

    /// Get WriteFloatOptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> OptionsBuilder {
        OptionsBuilder::new()
    }

    /// Create OptionsBuilder using existing values.
    #[inline(always)]
    pub const fn rebuild(&self) -> OptionsBuilder {
        OptionsBuilder {
            max_significant_digits: self.max_significant_digits,
            min_significant_digits: self.min_significant_digits,
            positive_exponent_break: self.positive_exponent_break,
            negative_exponent_break: self.negative_exponent_break,
            round_mode: self.round_mode,
            trim_floats: self.trim_floats,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
        }
    }
}

impl Default for Options {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Define unwrap_or_zero for a custom type.
macro_rules! unwrap_or_zero {
    ($name:ident, $opt:ident, $t:ident) => {
        /// Unwrap `Option` as a const fn.
        #[inline(always)]
        const fn $name(option: $opt) -> $t {
            match option {
                Some(x) => x.get(),
                None => 0,
            }
        }
    };
}

unwrap_or_zero!(unwrap_or_zero_usize, OptionUsize, usize);
unwrap_or_zero!(unwrap_or_zero_i32, OptionI32, i32);

/// Unwrap `Option` as a const fn.
#[inline(always)]
const fn unwrap_or_max_usize(option: OptionUsize) -> usize {
    match option {
        Some(x) => x.get(),
        None => usize::MAX,
    }
}
