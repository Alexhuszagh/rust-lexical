//! Configuration options for parsing integers.

use lexical_util::options::ParseOptions;
use lexical_util::result::Result;
use static_assertions::const_assert;

/// Builder for `Options`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionsBuilder {
    /// Disable multi-digit optimizations.
    ///
    /// Using multi-digit optimizations allows parsing many digits
    /// from longer input strings at once which can dramatically
    /// improve performance (>70%) for long strings, but the
    /// increased branching can decrease performance for simple
    /// strings by 5-20%. Choose based on your inputs.
    no_multi_digit: bool,
}

impl OptionsBuilder {
    /// Create new options builder with default options.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            no_multi_digit: true,
        }
    }

    // GETTERS

    /// Get if we disable the use of multi-digit optimizations.
    #[inline(always)]
    pub const fn get_no_multi_digit(&self) -> bool {
        self.no_multi_digit
    }

    // SETTERS

    /// Set if we disable the use of multi-digit optimizations.
    #[inline(always)]
    pub const fn no_multi_digit(mut self, no_multi_digit: bool) -> Self {
        self.no_multi_digit = no_multi_digit;
        self
    }

    // BUILDERS

    /// Check if the builder state is valid.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        true
    }

    /// Build the Options struct with bounds validation.
    #[inline(always)]
    pub const fn build_unchecked(&self) -> Options {
        Options {
            no_multi_digit: self.no_multi_digit,
        }
    }

    /// Build the Options struct.
    #[inline(always)]
    pub const fn build(&self) -> Result<Options> {
        Ok(self.build_unchecked())
    }
}

impl Default for OptionsBuilder {
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
/// use lexical_parse_integer::options::Options;
///
/// # pub fn main() {
/// let options = Options::builder()
///     .build()
///     .unwrap();
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Options {
    /// Disable multi-digit optimizations.
    ///
    /// Using multi-digit optimizations allows parsing many digits
    /// from longer input strings at once which can dramatically
    /// improve performance (>70%) for long strings, but the
    /// increased branching can decrease performance for simple
    /// strings by 5-20%. Choose based on your inputs.
    no_multi_digit: bool,
}

impl Options {
    /// Create options with default values.
    #[must_use]
    #[inline(always)]
    pub const fn new() -> Self {
        Self::builder().build_unchecked()
    }

    // GETTERS

    /// Check if the options state is valid.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        self.rebuild().is_valid()
    }

    /// Get if we disable the use of multi-digit optimizations.
    #[inline(always)]
    pub const fn get_no_multi_digit(&self) -> bool {
        self.no_multi_digit
    }

    // SETTERS

    /// Set if we disable the use of multi-digit optimizations.
    #[inline(always)]
    pub fn no_multi_digit(&mut self, no_multi_digit: bool) {
        self.no_multi_digit = no_multi_digit;
    }

    // BUILDERS

    /// Get `OptionsBuilder` as a static function.
    #[inline(always)]
    pub const fn builder() -> OptionsBuilder {
        OptionsBuilder::new()
    }

    /// Create `OptionsBuilder` using existing values.
    #[inline(always)]
    pub const fn rebuild(&self) -> OptionsBuilder {
        OptionsBuilder {
            no_multi_digit: self.no_multi_digit,
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

// PRE-DEFINED CONSTANTS
// ---------------------

/// Standard number format.
#[rustfmt::skip]
pub const STANDARD: Options = Options::new();
const_assert!(STANDARD.is_valid());

/// Options optimized for small numbers.
#[rustfmt::skip]
pub const SMALL_NUMBERS: Options = Options::builder()
        .no_multi_digit(true)
        .build_unchecked();
const_assert!(SMALL_NUMBERS.is_valid());

/// Options optimized for large numbers and long strings.
#[rustfmt::skip]
pub const LARGE_NUMBERS: Options = Options::builder()
        .no_multi_digit(false)
        .build_unchecked();
const_assert!(LARGE_NUMBERS.is_valid());
