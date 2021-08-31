//! Configuration options for parsing integers.

use lexical_util::options::ParseOptions;
use lexical_util::result::Result;
use static_assertions::const_assert;

/// Builder for `Options`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionsBuilder {}

impl OptionsBuilder {
    /// Create new options builder with default options.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {}
    }

    // BUILDERS

    /// Check if the builder state is valid.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        true
    }

    /// Build the Options struct with bounds validation.
    ///
    /// # Safety
    ///
    /// Safe as long as`is_valid` is true.
    #[inline(always)]
    pub const unsafe fn build_unchecked(&self) -> Options {
        Options {}
    }

    /// Build the Options struct.
    #[inline(always)]
    pub const fn build(&self) -> Result<Options> {
        // SAFETY: always safe, since it must be valid.
        Ok(unsafe { self.build_unchecked() })
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
pub struct Options {}

impl Options {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Check if the options state is valid.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        true
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
        OptionsBuilder {}
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
