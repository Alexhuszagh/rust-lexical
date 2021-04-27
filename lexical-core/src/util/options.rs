//! Configuration options for parsing and formatting numbers.

#[cfg(any(feature = "atof", feature = "ftoa"))]
use super::config::F64_FORMATTED_SIZE_DECIMAL as FLOAT_SIZE;
#[cfg(any(feature = "atof", feature = "atoi", feature = "ftoa"))]
use super::format::NumberFormat;
#[cfg(feature = "atof")]
use super::rounding::RoundingKind;

// CONSTANTS
// ---------

// Constants to dictate default values for options.
#[cfg(any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa"))]
pub(crate) const DEFAULT_RADIX: u8 = 10;
#[cfg(any(feature = "atof", feature = "ftoa"))]
pub(crate) const DEFAULT_FORMAT: NumberFormat = NumberFormat::STANDARD;
#[cfg(any(feature = "atof", feature = "ftoa"))]
pub(crate) const DEFAULT_INF_STRING: &'static [u8] = b"inf";
#[cfg(feature = "atof")]
pub(crate) const DEFAULT_INFINITY_STRING: &'static [u8] = b"infinity";
#[cfg(any(feature = "atof", feature = "ftoa"))]
pub(crate) const DEFAULT_NAN_STRING: &'static [u8] = b"NaN";
#[cfg(feature = "atof")]
pub(crate) const DEFAULT_INCORRECT: bool = false;
#[cfg(feature = "atof")]
pub(crate) const DEFAULT_LOSSY: bool = false;
#[cfg(feature = "atof")]
pub(crate) const DEFAULT_ROUNDING: RoundingKind = RoundingKind::NearestTieEven;
#[cfg(feature = "ftoa")]
pub(crate) const DEFAULT_TRIM_FLOATS: bool = false;

// VALIDATORS
// ----------

/// Return `None` if radix is invalid.
/// Short-circuits to allow use in a const fn.
#[cfg(all(feature = "radix", any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa")))]
macro_rules! to_radix {
    ($radix:expr) => {{
        if $radix < 2 || $radix > 36 {
            return None
        }
        $radix
    }};
}

/// Return `None` if radix is invalid.
/// Short-circuits to allow use in a const fn.
#[cfg(all(not(feature = "radix"), any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa")))]
macro_rules! to_radix {
    ($radix:expr) => {{
        if $radix != 10 {
            return None
        }
        $radix
    }};
}

/// Check if byte array starts with case-insensitive N.
#[inline]
#[cfg(any(feature = "atof", feature = "ftoa"))]
const fn starts_with_n(bytes: &[u8]) -> bool {
    if bytes.len() == 0 {
        false
    } else {
        match bytes[0] {
            b'N' => true,
            b'n' => true,
            _    => false,
        }
    }
}

/// Get the NaN string if the NaN string is valid.
#[cfg(any(feature = "atof", feature = "ftoa"))]
macro_rules! to_nan_string {
    ($nan:expr) => {{
        if !starts_with_n($nan) {
            return None;
        } else if $nan.len() > FLOAT_SIZE {
            return None;
        }
        $nan
    }};
}

/// Check if byte array starts with case-insensitive I.
#[inline]
#[cfg(any(feature = "atof", feature = "ftoa"))]
const fn starts_with_i(bytes: &[u8]) -> bool {
    if bytes.len() == 0 {
        false
    } else {
        match bytes[0] {
            b'I' => true,
            b'i' => true,
            _    => false,
        }
    }
}

/// Get the short infinity string if the infinity string is valid.
#[cfg(any(feature = "atof", feature = "ftoa"))]
macro_rules! to_inf_string {
    ($inf:expr) => {{
        if !starts_with_i($inf) {
            return None;
        } else if $inf.len() > FLOAT_SIZE {
            return None;
        }
        $inf
    }};
}

/// Get the long infinity string if the infinity string is valid.
#[cfg(feature = "atof")]
macro_rules! to_infinity_string {
    ($infinity:expr, $inf:expr) => {{
        if $infinity.len() < $inf.len() || !starts_with_i($infinity) {
            return None;
        } else if $infinity.len() > FLOAT_SIZE {
            return None;
        }
        $infinity
    }};
}

// DUMMY OPTIONS
// -------------

// Options when no traits are available.
#[cfg(any(
    not(feature = "atof"),
    not(feature = "atoi"),
    not(feature = "ftoa"),
    not(feature = "itoa")
))]
pub struct DummyOptions {
}

// PARSE INTEGER
// -------------

/// Builder for `ParseIntegerOptions`.
#[repr(C)]
#[cfg(feature = "atoi")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseIntegerOptionsBuilder {
    /// Radix for integer string.
    radix: u8,
    /// Number format.
    format: Option<NumberFormat>
}

#[cfg(feature = "atoi")]
impl ParseIntegerOptionsBuilder {
    /// Create new, default builder.
    #[inline(always)]
    const fn new() -> Self {
        Self {
            radix: DEFAULT_RADIX,
            format: None
        }
    }

    /// Set the radix for ParseIntegerOptionsBuilder.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    /// Set the format specifier for ParseIntegerOptionsBuilder.
    #[inline(always)]
    pub const fn format(mut self, format: Option<NumberFormat>) -> Self {
        self.format = format;
        self
    }

    /// Build the ParseIntegerOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Option<ParseIntegerOptions> {
        let radix = to_radix!(self.radix) as u32;
        let format = self.format;
        Some(ParseIntegerOptions { radix, format })
    }
}

#[cfg(feature = "atoi")]
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
#[cfg(feature = "atoi")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseIntegerOptions {
    /// Radix for integer string.
    radix: u32,
    /// Number format.
    format: Option<NumberFormat>,
}

#[cfg(feature = "atoi")]
impl ParseIntegerOptions {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { radix: DEFAULT_RADIX as u32, format: None }
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to parse the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn binary() -> Self {
        Self { radix: 2, format: None }
    }

    /// Create new options to parse the default decimal format.
    #[inline(always)]
    pub const fn decimal() -> Self {
        Self { radix: 10, format: None }
    }

    /// Create new options to parse the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn hexadecimal() -> Self {
        Self { radix: 16, format: None }
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.radix
    }

    /// Get the number format.
    #[inline(always)]
    pub const fn format(&self) -> Option<NumberFormat> {
        self.format
    }

    // SETTERS

    /// Set the radix.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_radix(&mut self, radix: u32) {
        self.radix = radix
    }

    /// Set the number format.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_format(&mut self, format: Option<NumberFormat>) {
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
            radix: self.radix as u8,
            format: self.format
        }
    }
}

#[cfg(feature = "atoi")]
impl Default for ParseIntegerOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

// PARSE FLOAT
// -----------

/// Builder for `ParseFloatOptions`.
#[repr(C)]
#[cfg(feature = "atof")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseFloatOptionsBuilder {
    /// Radix for float string.
    radix: u8,
    /// Number format.
    format: NumberFormat,
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
    infinity_string: &'static [u8]
}

#[cfg(feature = "atof")]
impl ParseFloatOptionsBuilder {
    /// Create new, default builder.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            radix: DEFAULT_RADIX,
            format: DEFAULT_FORMAT,
            rounding: DEFAULT_ROUNDING,
            incorrect: DEFAULT_INCORRECT,
            lossy: DEFAULT_LOSSY,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
        }
    }

    // SETTERS

    /// Set the radix for ParseFloatOptionsBuilder.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    /// Set the format specifier for ParseFloatOptionsBuilder.
    #[inline(always)]
    pub const fn format(mut self, format: Option<NumberFormat>) -> Self {
        self.format = match format {
            Some(format) => format,
            None => DEFAULT_FORMAT,
        };
        self
    }

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

    /// Build the ParseFloatOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Option<ParseFloatOptions> {
        let radix = to_radix!(self.radix) as u32;
        let kind = self.rounding.as_u32() << 8;
        let incorrect = (self.incorrect as u32) << 16;
        let lossy = (self.lossy as u32) << 17;
        let compressed = radix | kind | incorrect | lossy;
        let format = self.format;
        let nan_string = to_nan_string!(self.nan_string);
        let inf_string = to_inf_string!(self.inf_string);
        let infinity_string = to_infinity_string!(self.infinity_string, self.inf_string);

        // Validate we can't use incorrect **and** lossy together.
        if self.incorrect && self.lossy {
            return None;
        }

        Some(ParseFloatOptions {
            compressed,
            format,
            nan_string,
            inf_string,
            infinity_string,
        })
    }
}

#[cfg(feature = "atof")]
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
#[cfg(feature = "atof")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseFloatOptions {
    /// Compressed storage of the radix, rounding kind, incorrect, and lossy.
    /// Radix is the lower 8 bits, bits 8-16 are the rounding kind,
    /// bit 17 is incorrect, and bit 18 is lossy.
    compressed: u32,
    /// Number format.
    format: NumberFormat,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
    /// Long string representation of `Infinity`.
    infinity_string: &'static [u8]
}

#[cfg(feature = "atof")]
impl ParseFloatOptions {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            compressed: DEFAULT_RADIX as u32 | DEFAULT_ROUNDING.as_u32() << 8,
            format: DEFAULT_FORMAT,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
        }
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to write the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn binary() -> Self {
        Self {
            compressed: 2 | DEFAULT_ROUNDING.as_u32() << 8,
            format: DEFAULT_FORMAT,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
        }
    }

    /// Create new options to write the default decimal format.
    #[inline(always)]
    pub const fn decimal() -> Self {
        Self {
            compressed: 10 | DEFAULT_ROUNDING.as_u32() << 8,
            format: DEFAULT_FORMAT,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
        }
    }

    /// Create new options to write the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn hexadecimal() -> Self {
        Self {
            compressed: 16 | DEFAULT_ROUNDING.as_u32() << 8,
            format: DEFAULT_FORMAT,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
            infinity_string: DEFAULT_INFINITY_STRING,
        }
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.compressed & 0xFF
    }

    /// Get the rounding kind for float.
    #[inline(always)]
    pub const fn rounding(&self) -> RoundingKind {
        unsafe {
            RoundingKind::from_u32((self.compressed & 0xFF00) >> 8)
        }
    }

    /// Get if we use the incorrect, fast parser.
    #[inline(always)]
    pub const fn incorrect(&self) -> bool {
        self.compressed & 0x10000 != 0
    }

    /// Get if we use the lossy, fast parser.
    #[inline(always)]
    pub const fn lossy(&self) -> bool {
        self.compressed & 0x20000 != 0
    }

    /// Get the number format.
    #[inline(always)]
    pub const fn format(&self) -> NumberFormat {
        self.format
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

    /// Get the exponent character.
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        self.format.exponent(self.radix())
    }

    // SETTERS

    /// Set the radix.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_radix(&mut self, radix: u32) {
        // Unset the lower 8 bits, then set the radix (as an 8-bit integer).
        self.compressed &= !0xFF;
        self.compressed |= radix & 0xFF;
    }

    /// Set the rounding kind.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_rounding(&mut self, rounding: RoundingKind) {
        // Unset the lower 8 bits, then set the rounding kind (as an
        // 8-bit integer shifted 8 bits left).
        self.compressed &= !0xFF00;
        self.compressed |= (rounding.as_u32() << 8) & 0xFF00;
    }

    /// Set if we use the incorrect, fast parser.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_incorrect(&mut self, incorrect: bool) {
        // Unset the 16th bit, then set it based on the incorrect value.
        self.compressed &= !0x10000;
        self.compressed |= (incorrect as u32) << 16;
    }

    /// Set if we use the lossy, intermediate parser.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_lossy(&mut self, lossy: bool) {
        // Unset the 17th bit, then set it based on the lossy value.
        self.compressed &= !0x20000;
        self.compressed |= (lossy as u32) << 17;
    }

    /// Set the number format.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_format(&mut self, format: NumberFormat) {
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

    // BUILDERS

    /// Get ParseFloatOptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> ParseFloatOptionsBuilder {
        ParseFloatOptionsBuilder::new()
    }

    /// Create ParseFloatOptionsBuilder using existing values.
    pub const fn rebuild(self) -> ParseFloatOptionsBuilder {
        ParseFloatOptionsBuilder {
            radix: self.radix() as u8,
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

#[cfg(feature = "atof")]
impl Default for ParseFloatOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

// WRITE INTEGER
// -------------

/// Builder for `WriteIntegerOptions`.
#[repr(C)]
#[cfg(feature = "itoa")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WriteIntegerOptionsBuilder {
    radix: u8,
}

#[cfg(feature = "itoa")]
impl WriteIntegerOptionsBuilder {
    #[inline(always)]
    pub const fn new() -> WriteIntegerOptionsBuilder {
        WriteIntegerOptionsBuilder {
            radix: DEFAULT_RADIX,
        }
    }

    // SETTERS

    /// Set the radix for WriteIntegerOptionsBuilder.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    // BUILDERS

    /// Build the WriteIntegerOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Option<WriteIntegerOptions> {
        let radix = to_radix!(self.radix) as u32;
        Some(WriteIntegerOptions { radix })
    }
}

#[cfg(feature = "itoa")]
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
#[cfg(feature = "itoa")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WriteIntegerOptions {
    /// Radix for integer string.
    radix: u32,
}

#[cfg(feature = "itoa")]
impl WriteIntegerOptions {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { radix: DEFAULT_RADIX as u32 }
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to write the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn binary() -> Self {
        Self { radix: 2 }
    }

    /// Create new options to write the default decimal format.
    #[inline(always)]
    pub const fn decimal() -> Self {
        Self { radix: 10 }
    }

    /// Create new options to write the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn hexadecimal() -> Self {
        Self { radix: 16 }
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.radix
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

#[cfg(feature = "itoa")]
impl Default for WriteIntegerOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

// WRITE FLOAT
// -----------

/// Builder for `WriteFloatOptions`.
#[repr(C)]
#[cfg(feature = "ftoa")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WriteFloatOptionsBuilder {
    /// Radix for float string.
    radix: u8,
    /// Number format.
    format: Option<NumberFormat>,
    /// Trim the trailing ".0" from integral float strings.
    trim_floats: bool,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
}

#[cfg(feature = "ftoa")]
impl WriteFloatOptionsBuilder {
    /// Create new, default builder.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            radix: DEFAULT_RADIX,
            format: None,
            trim_floats: DEFAULT_TRIM_FLOATS,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    //  SETTERS

    /// Set the radix for WriteFloatOptionsBuilder.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn radix(mut self, radix: u8) -> Self {
        self.radix = radix;
        self
    }

    /// Set the format specifier for WriteFloatOptionsBuilder.
    #[inline(always)]
    pub const fn format(mut self, format: Option<NumberFormat>) -> Self {
        self.format = format;
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

    /// Build the ParseFloatOptions struct.
    #[inline(always)]
    pub const fn build(self) -> Option<WriteFloatOptions> {
        let radix = to_radix!(self.radix) as u32;
        let trim_floats = (self.trim_floats as u32) << 8;
        let compressed = radix | trim_floats;
        let format = self.format;
        let nan_string = to_nan_string!(self.nan_string);
        let inf_string = to_inf_string!(self.inf_string);

        Some(WriteFloatOptions {compressed, format, nan_string, inf_string })
    }
}

#[cfg(feature = "ftoa")]
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
#[cfg(feature = "ftoa")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WriteFloatOptions {
    /// Compressed storage of radix and trim floats.
    /// Radix is the lower 8 bits, trim_floats is bit 9.
    compressed: u32,
    /// Number format.
    format: Option<NumberFormat>,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: &'static [u8],
    /// Short string representation of `Infinity`.
    inf_string: &'static [u8],
}

#[cfg(feature = "ftoa")]
impl WriteFloatOptions {
    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            compressed: DEFAULT_RADIX as u32,
            format: None,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    // PRE-DEFINED CONSTANTS

    /// Create new options to write the default binary format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn binary() -> Self {
        Self {
            compressed: 2,
            format: None,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    /// Create new options to write the default decimal format.
    #[inline(always)]
    pub const fn decimal() -> Self {
        Self {
            compressed: 10,
            format: None,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    /// Create new options to write the default hexadecimal format.
    #[inline(always)]
    #[cfg(feature = "radix")]
    pub const fn hexadecimal() -> Self {
        Self {
            compressed: 16,
            format: None,
            nan_string: DEFAULT_NAN_STRING,
            inf_string: DEFAULT_INF_STRING,
        }
    }

    // GETTERS

    /// Get the radix.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        self.compressed & 0xFF
    }

    /// Get if we should trim a trailing `".0"` from floats.
    #[inline(always)]
    pub const fn trim_floats(&self) -> bool {
        self.compressed & 0x100 != 0
    }

    /// Get the number format.
    #[inline(always)]
    pub const fn format(&self) -> Option<NumberFormat> {
        self.format
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

    /// Get the digit separator character.
    #[inline(always)]
    pub const fn digit_separator(&self) -> u8 {
        match self.format {
            Some(format) => format.digit_separator(),
            None => b'\x00',
        }
    }

    /// Get the decimal point character.
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        match self.format {
            Some(format) => format.decimal_point(),
            None => b'.',
        }
    }

    /// Get the exponent character.
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        // Const fn version of unwrap_or().
        let format = match self.format {
            Some(format) => format,
            None => DEFAULT_FORMAT,
        };

        format.exponent(self.radix())
    }

    // SETTERS

    /// Set the radix.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_radix(&mut self, radix: u32) {
        // Unset the lower 8 bits, then set the radix (as an 8-bit integer).
        self.compressed &= !0xFF;
        self.compressed |= radix & 0xFF;
    }

    /// Set if we should trim a trailing `".0"` from floats.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_trim_floats(&mut self, trim_floats: bool) {
        // Unset the 8th bit, then set it based on the trim floats value.
        self.compressed &= !0x100;
        self.compressed |= (trim_floats as u32) << 8;
    }

    /// Set the number format.
    /// Unsafe, use the builder API for option validation.
    #[inline(always)]
    pub unsafe fn set_format(&mut self, format: Option<NumberFormat>) {
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

    // BUILDERS

    /// Get WriteFloatOptionsBuilder as a static function.
    #[inline(always)]
    pub const fn builder() -> WriteFloatOptionsBuilder {
        WriteFloatOptionsBuilder::new()
    }

    /// Create WriteFloatOptionsBuilder using existing values.
    pub const fn rebuild(self) -> WriteFloatOptionsBuilder {
        WriteFloatOptionsBuilder {
            radix: self.radix() as u8,
            trim_floats: self.trim_floats(),
            format: self.format,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
        }
    }
}

#[cfg(feature = "ftoa")]
impl Default for WriteFloatOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    // HELPERS

    // Wrapper for the macro for a const fn.
    #[inline]
    #[cfg(any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa"))]
    const fn to_radix(radix: u32) -> Option<u32> {
        Some(to_radix!(radix))
    }

    // Wrapper for the macro for a const fn.
    #[inline]
    #[cfg(any(feature = "atof", feature = "ftoa"))]
    const fn to_nan_string(nan: &'static [u8]) -> Option<&'static [u8]> {
        Some(to_nan_string!(nan))
    }

    // Wrapper for the macro for a const fn.
    #[inline]
    #[cfg(any(feature = "atof", feature = "ftoa"))]
    const fn to_inf_string(inf: &'static [u8]) -> Option<&'static [u8]> {
        Some(to_inf_string!(inf))
    }

    // Wrapper for the macro for a const fn.
    #[inline]
    #[cfg(feature = "atof")]
    const fn to_infinity_string(infinity: &'static [u8], inf: &'static [u8]) -> Option<&'static [u8]> {
        Some(to_infinity_string!(infinity, inf))
    }

    // TESTS

    #[test]
    #[cfg(all(feature = "radix", any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa")))]
    fn test_to_radix() {
        assert_eq!(to_radix(1), None);
        assert_eq!(to_radix(2), Some(2));
        assert_eq!(to_radix(10), Some(10));
        assert_eq!(to_radix(36), Some(36));
        assert_eq!(to_radix(37), None);
    }

    #[test]
    #[cfg(all(not(feature = "radix"), any(feature = "atof", feature = "atoi", feature = "ftoa", feature = "itoa")))]
    fn test_to_radix() {
        assert_eq!(to_radix(1), None);
        assert_eq!(to_radix(2), None);
        assert_eq!(to_radix(10), Some(10));
        assert_eq!(to_radix(36), None);
        assert_eq!(to_radix(37), None);
    }

    #[test]
    #[cfg(any(feature = "atof", feature = "ftoa"))]
    fn to_nan_string_test() {
        assert_eq!(to_nan_string(b""), None);
        assert_eq!(to_nan_string(b"i"), None);
        assert_eq!(to_nan_string(b"inf"), None);
        assert_eq!(to_nan_string(b"nan").unwrap(), b"nan");
        assert_eq!(to_nan_string(b"NAN").unwrap(), b"NAN");
        // Test long strings (>= 64 or >= 256) fail, so we can't get buffer overflows.
        assert_eq!(to_nan_string(b"NANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNANNAN"), None);
    }

    #[test]
    #[cfg(any(feature = "atof", feature = "ftoa"))]
    fn to_inf_string_test() {
        assert_eq!(to_inf_string(b""), None);
        assert_eq!(to_inf_string(b"n"), None);
        assert_eq!(to_inf_string(b"nan"), None);
        assert_eq!(to_inf_string(b"inf").unwrap(), b"inf");
        assert_eq!(to_inf_string(b"INF").unwrap(), b"INF");
        // Test long strings (>= 64 or >= 256) fail, so we can't get buffer overflows.
        assert_eq!(to_inf_string(b"INFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINF"), None);
    }

    #[test]
    #[cfg(feature = "atof")]
    fn to_infinity_string_test() {
        assert_eq!(to_infinity_string(b"", b"inf"), None);
        assert_eq!(to_infinity_string(b"n", b"inf"), None);
        assert_eq!(to_infinity_string(b"i", b"inf"), None);
        assert_eq!(to_infinity_string(b"nan", b"inf"), None);
        assert_eq!(to_infinity_string(b"in", b"inf"), None);
        assert_eq!(to_infinity_string(b"IN", b"inf"), None);
        assert_eq!(to_infinity_string(b"na", b"inf"), None);
        assert_eq!(to_infinity_string(b"infinity", b"inf").unwrap(), b"infinity");
        assert_eq!(to_infinity_string(b"INFINITY", b"inf").unwrap(), b"INFINITY");
        // Test long strings (>= 64 or >= 256) fail, so we can't get buffer overflows.
        assert_eq!(to_infinity_string(b"INFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINFINF", b"inf"), None);
    }

    #[test]
    #[cfg(all(feature = "atoi", feature = "radix"))]
    fn test_parse_integer_options() {
        let options = ParseIntegerOptions::builder()
            .radix(1)
            .build();
        assert_eq!(options, None);

        let options = ParseIntegerOptions::builder()
            .radix(32)
            .build();
        assert!(options.is_some());
        let options = options.unwrap();
        assert_eq!(options.radix(), 32);
        assert_eq!(options.format(), None);

        let options = options.rebuild()
            .radix(10)
            .build()
            .unwrap();
        assert_eq!(options.radix(), 10);
        assert_eq!(options.format(), None);
    }

    #[test]
    #[cfg(all(feature = "itoa", feature = "radix"))]
    fn test_write_integer_options() {
        let options = WriteIntegerOptions::builder()
            .radix(1)
            .build();
        assert_eq!(options, None);

        let options = WriteIntegerOptions::builder()
            .radix(32)
            .build();
        assert!(options.is_some());
        let options = options.unwrap();
        assert_eq!(options.radix(), 32);

        let options = options.rebuild()
            .radix(10)
            .build()
            .unwrap();
        assert_eq!(options.radix(), 10);
    }

    #[test]
    #[cfg(all(feature = "atof", feature = "radix"))]
    fn test_parse_float_options() {
        // Invalid radix
        let options = ParseFloatOptions::builder()
            .radix(1)
            .incorrect(true)
            .build();
        assert_eq!(options, None);

        // Lossy and incorrect
        let options = ParseFloatOptions::builder()
            .incorrect(true)
            .lossy(true)
            .build();
        assert_eq!(options, None);

        // Inf string is too long
        let options = ParseFloatOptions::builder()
            .inf_string(b"infinityinfinf")
            .build();
        assert_eq!(options, None);

        let options = ParseFloatOptions::builder()
            .radix(32)
            .incorrect(true)
            .build();
        assert!(options.is_some());
        let options = options.unwrap();
        assert_eq!(options.radix(), 32);
        assert_eq!(options.incorrect(), true);
        assert_eq!(options.lossy(), false);
        assert_eq!(options.format(), DEFAULT_FORMAT);
        assert_eq!(options.nan_string(), b"NaN");
        assert_eq!(options.inf_string(), b"inf");
        assert_eq!(options.infinity_string(), b"infinity");

        let options = options.rebuild()
            .radix(10)
            .incorrect(false)
            .lossy(true)
            .nan_string(b"nyancat")
            .inf_string(b"INF")
            .build()
            .unwrap();
        assert_eq!(options.radix(), 10);
        assert_eq!(options.incorrect(), false);
        assert_eq!(options.lossy(), true);
        assert_eq!(options.format(), DEFAULT_FORMAT);
        assert_eq!(options.nan_string(), b"nyancat");
        assert_eq!(options.inf_string(), b"INF");
        assert_eq!(options.infinity_string(), b"infinity");
    }

    #[test]
    #[cfg(all(feature = "ftoa", feature = "radix"))]
    fn test_write_float_options() {
        let options = WriteFloatOptions::builder()
            .radix(1)
            .trim_floats(true)
            .build();
        assert_eq!(options, None);

        let options = WriteFloatOptions::builder()
            .radix(32)
            .trim_floats(true)
            .build();
        assert!(options.is_some());
        let options = options.unwrap();
        assert_eq!(options.radix(), 32);
        assert_eq!(options.trim_floats(), true);
        assert_eq!(options.format(), None);
        assert_eq!(options.nan_string(), b"NaN");
        assert_eq!(options.inf_string(), b"inf");

        let options = options.rebuild()
            .radix(10)
            .trim_floats(false)
            .inf_string(b"infinity")
            .build()
            .unwrap();
        assert_eq!(options.radix(), 10);
        assert_eq!(options.trim_floats(), false);
        assert_eq!(options.format(), None);
        assert_eq!(options.nan_string(), b"NaN");
        assert_eq!(options.inf_string(), b"infinity");
    }
}
