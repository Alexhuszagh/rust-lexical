//! Configuration options for parsing integers.

// TODO(ahuszagh) Should add a builder trait at some point.
//  Will interfere with const fn though, unfortunately.

/// Builder for `Options`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OptionsBuilder {}

impl OptionsBuilder {
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
    pub const unsafe fn build_unchecked(self) -> Options {
        Options {}
    }

    /// Build the Options struct.
    #[inline(always)]
    pub const fn build(self) -> Option<Options> {
        Some(Options {})
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
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    pub const fn rebuild(self) -> OptionsBuilder {
        OptionsBuilder {}
    }
}

impl Default for Options {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
