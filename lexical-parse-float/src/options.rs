//! Configuration options for parsing floats.

#![doc(hidden)]

use lexical_util::error::Error;
use lexical_util::options::ParseOptions;
use lexical_util::result::Result;

/// Maximum length for a special string.
const MAX_SPECIAL_STRING_LENGTH: usize = 50;

/// Builder for `Options`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionsBuilder {
    /// Disable the use of arbitrary-precision arithmetic, and always
    /// return the results from the fast or intermediate path algorithms.
    lossy: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
    /// Long string representation of `Infinity`.
    infinity_string: &'static [u8],
}

impl OptionsBuilder {
    /// Create new options builder with default options.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            lossy: false,
            nan_string: b"NaN",
            inf_string: b"inf",
            infinity_string: b"infinity",
        }
    }

    // GETTERS

    /// Get if we disable the use of arbitrary-precision arithmetic.
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

    /// Set if we disable the use of arbitrary-precision arithmetic.
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
        let length = self.inf_string.len();
        if self.inf_string.is_empty() || length > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(self.inf_string[0], b'I' | b'i') {
            false
        } else if length > self.infinity_string.len() {
            false
        } else {
            true
        }
    }

    /// Determine if `infinity_string` is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn infinity_string_is_valid(&self) -> bool {
        let length = self.infinity_string.len();
        if self.infinity_string.is_empty() || length > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(self.infinity_string[0], b'I' | b'i') {
            false
        } else if length < self.inf_string.len() {
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
        } else if !self.infinity_string_is_valid() {
            false
        } else {
            true
        }
    }

    /// Build the Options struct with bounds validation.
    ///
    /// # Safety
    ///
    /// Safe as long as `is_valid` is true. If `nan_string`, `inf_string`,
    /// or `infinity_string` are too long, writing special floats may lead
    /// to buffer overflows, and therefore severe security vulnerabilities.
    #[inline(always)]
    pub const unsafe fn build_unchecked(&self) -> Options {
        Options {
            lossy: self.lossy,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
            infinity_string: self.infinity_string,
        }
    }

    /// Build the Options struct.
    #[inline(always)]
    pub const fn build(self) -> Result<Options> {
        let nan_length = self.nan_string.len();
        let inf_length = self.inf_string.len();
        let infinity_length = self.infinity_string.len();
        if nan_length == 0 || !matches!(self.nan_string[0], b'N' | b'n') {
            Err(Error::InvalidNanString)
        } else if nan_length > MAX_SPECIAL_STRING_LENGTH {
            Err(Error::NanStringTooLong)
        } else if inf_length == 0 || !matches!(self.inf_string[0], b'I' | b'i') {
            Err(Error::InvalidInfString)
        } else if inf_length > MAX_SPECIAL_STRING_LENGTH {
            Err(Error::InfStringTooLong)
        } else if infinity_length == 0 || !matches!(self.infinity_string[0], b'I' | b'i') {
            Err(Error::InvalidInfinityString)
        } else if infinity_length > MAX_SPECIAL_STRING_LENGTH {
            Err(Error::InfinityStringTooLong)
        } else if infinity_length < inf_length {
            Err(Error::InfinityStringTooShort)
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Options {
    /// Disable the use of arbitrary-precision arithmetic, and always
    /// return the results from the fast or intermediate path algorithms.
    lossy: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
    /// Long string representation of `Infinity`.
    infinity_string: &'static [u8],
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

    /// Get if we disable the use of arbitrary-precision arithmetic.
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

    // SETTERS

    /// Set if we disable the use of arbitrary-precision arithmetic.
    ///
    /// # Safety
    ///
    /// Always safe, just marked as unsafe for API compatibility.
    #[inline(always)]
    pub unsafe fn set_lossy(&mut self, lossy: bool) {
        self.lossy = lossy
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
    /// Unsafe if `inf_string.len() > MAX_SPECIAL_STRING_LENGTH`.
    #[inline(always)]
    pub unsafe fn set_inf_string(&mut self, inf_string: &'static [u8]) {
        self.inf_string = inf_string
    }

    /// Set the long string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Unsafe if `infinity_string.len() > MAX_SPECIAL_STRING_LENGTH`.
    #[inline(always)]
    pub unsafe fn set_infinity_string(&mut self, infinity_string: &'static [u8]) {
        self.infinity_string = infinity_string
    }

    // BUILDERS

    /// Get OptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> OptionsBuilder {
        OptionsBuilder::new()
    }

    /// Create OptionsBuilder using existing values.
    #[inline(always)]
    pub const fn rebuild(&self) -> OptionsBuilder {
        OptionsBuilder {
            lossy: self.lossy,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
            infinity_string: self.infinity_string,
        }
    }
}

impl Default for Options {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl ParseOptions for Options {
    #[inline(always)]
    fn is_valid(&self) -> bool {
        Self::is_valid(self)
    }
}
