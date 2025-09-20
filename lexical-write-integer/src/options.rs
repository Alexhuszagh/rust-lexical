//! Configuration options for writing integers.
//!
//! This currently has no functionality, since we do not
//! support any features for writing integers at this time.
//!
//! # Examples
//!
//! ```rust
//! # use core::str;
//! use lexical_write_integer::{Options, ToLexicalWithOptions};
//! use lexical_write_integer::format::STANDARD;
//!
//! const OPTIONS: Options = Options::builder()
//!     .build_strict();
//!
//! const BUFFER_SIZE: usize = OPTIONS.buffer_size_const::<u64, STANDARD>();
//! let mut buffer = [0u8; BUFFER_SIZE];
//! let value = 1234u64;
//! let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &OPTIONS);
//! assert_eq!(str::from_utf8(digits), Ok("1234"));
//! ```

use lexical_util::constants::FormattedSize;
use lexical_util::format::NumberFormat;
use lexical_util::options::WriteOptions;
use lexical_util::result::Result;

/// Builder for [`Options`].
///
/// # Examples
///
/// ```rust
/// use core::str;
///
/// use lexical_write_integer::{Options, ToLexicalWithOptions};
/// use lexical_write_integer::format::STANDARD;
///
/// const OPTIONS: Options = Options::builder()
///     .build_strict();
///
/// const BUFFER_SIZE: usize = OPTIONS.buffer_size_const::<u64, STANDARD>();
/// let mut buffer = [0u8; BUFFER_SIZE];
/// let value = 1234u64;
/// let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &OPTIONS);
/// assert_eq!(str::from_utf8(digits), Ok("1234"));
/// ```
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

    /// Build the [`Options`] struct without validation.
    ///
    /// <div class="warning">
    ///
    /// This is completely safe, however, misusing this could cause panics at
    /// runtime. Always check if [`is_valid`] prior to using the built
    /// options.
    ///
    /// </div>
    ///
    /// [`is_valid`]: Self::is_valid
    #[inline(always)]
    pub const fn build_unchecked(&self) -> Options {
        Options {}
    }

    /// Build the [`Options`] struct.
    ///
    /// This can never panic.
    ///
    /// <!-- # Panics
    ///
    /// If the built options are not valid. This should always
    /// be used within a const context to avoid panics at runtime.
    /// -->
    #[inline(always)]
    pub const fn build_strict(&self) -> Options {
        match self.build() {
            Ok(value) => value,
            Err(error) => core::panic!("{}", error.description()),
        }
    }

    /// Build the [`Options`] struct.
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
/// use core::str;
///
/// use lexical_write_integer::{Options, ToLexicalWithOptions};
/// use lexical_write_integer::format::STANDARD;
///
/// const OPTIONS: Options = Options::builder()
///     .build_strict();
///
/// const BUFFER_SIZE: usize = OPTIONS.buffer_size_const::<u64, STANDARD>();
/// let mut buffer = [0u8; BUFFER_SIZE];
/// let value = 1234u64;
/// let digits = value.to_lexical_with_options::<STANDARD>(&mut buffer, &OPTIONS);
/// assert_eq!(str::from_utf8(digits), Ok("1234"));
/// ```
// FIXME: Add phantom data for private fields.
//  This is a BREAKING change so requires a major API release.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Options {}

impl Options {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {}
    }

    /// Create the default options for a given radix.
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn from_radix(_: u8) -> Self {
        Self::new()
    }

    /// Check if the options state is valid.
    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        true
    }

    /// Get an upper bound on the required buffer size.
    ///
    /// This is always [`FORMATTED_SIZE`][FormattedSize::FORMATTED_SIZE]
    /// or [`FORMATTED_SIZE_DECIMAL`][FormattedSize::FORMATTED_SIZE_DECIMAL],
    /// depending on the radix.
    #[inline(always)]
    pub const fn buffer_size_const<T: FormattedSize, const FORMAT: u128>(&self) -> usize {
        if (NumberFormat::<FORMAT> {}.radix()) == 10 {
            T::FORMATTED_SIZE_DECIMAL
        } else {
            T::FORMATTED_SIZE
        }
    }

    // BUILDERS

    /// Get [`OptionsBuilder`] as a static function.
    #[inline(always)]
    pub const fn builder() -> OptionsBuilder {
        OptionsBuilder::new()
    }

    /// Create [`OptionsBuilder`] using existing values.
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

impl WriteOptions for Options {
    #[inline(always)]
    fn is_valid(&self) -> bool {
        Self::is_valid(self)
    }

    #[doc = lexical_util::write_options_doc!()]
    #[inline(always)]
    fn buffer_size<T: FormattedSize, const FORMAT: u128>(&self) -> usize {
        self.buffer_size_const::<T, FORMAT>()
    }
}

// PRE-DEFINED CONSTANTS
// ---------------------

/// Standard number format.
#[rustfmt::skip]
pub const STANDARD: Options = Options::new();
