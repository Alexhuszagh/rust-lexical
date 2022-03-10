//! Configuration options for writing floats.

use core::{mem, num};
use lexical_util::ascii::{is_valid_ascii, is_valid_letter_slice};
use lexical_util::constants::FormattedSize;
use lexical_util::error::Error;
use lexical_util::format::NumberFormat;
use lexical_util::options::{self, WriteOptions};
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionsBuilder {
    /// Maximum number of significant digits to write.
    /// If not set, it defaults to the algorithm's default.
    max_significant_digits: OptionUsize,
    /// Minimum number of significant digits to write.
    /// If not set, it defaults to the algorithm's default.
    /// Note that this isn't fully respected: if you wish to format
    /// `0.1` with 25 significant digits, the correct result **should**
    /// be `0.100000000000000005551115`. However, we would output
    /// `0.100000000000000000000000`, which is still the nearest float.
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
    /// Character to designate the exponent component of a float.
    exponent: u8,
    /// Character to separate the integer from the fraction components.
    decimal_point: u8,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: Option<&'static [u8]>,
    /// String representation of `Infinity`.
    inf_string: Option<&'static [u8]>,
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
            exponent: b'e',
            decimal_point: b'.',
            nan_string: Some(b"NaN"),
            inf_string: Some(b"inf"),
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

    /// Get the character to designate the exponent component of a float.
    #[inline(always)]
    pub const fn get_exponent(&self) -> u8 {
        self.exponent
    }

    /// Get the character to separate the integer from the fraction components.
    #[inline(always)]
    pub const fn get_decimal_point(&self) -> u8 {
        self.decimal_point
    }

    /// Get the string representation for `NaN`.
    #[inline(always)]
    pub const fn get_nan_string(&self) -> Option<&'static [u8]> {
        self.nan_string
    }

    /// Get the short string representation for `Infinity`.
    #[inline(always)]
    pub const fn get_inf_string(&self) -> Option<&'static [u8]> {
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

    /// Set the character to designate the exponent component of a float.
    #[inline(always)]
    pub const fn exponent(mut self, exponent: u8) -> Self {
        self.exponent = exponent;
        self
    }

    /// Set the character to separate the integer from the fraction components.
    #[inline(always)]
    pub const fn decimal_point(mut self, decimal_point: u8) -> Self {
        self.decimal_point = decimal_point;
        self
    }

    /// Set the string representation for `NaN`.
    #[inline(always)]
    pub const fn nan_string(mut self, nan_string: Option<&'static [u8]>) -> Self {
        self.nan_string = nan_string;
        self
    }

    /// Set the string representation for `Infinity`.
    #[inline(always)]
    pub const fn inf_string(mut self, inf_string: Option<&'static [u8]>) -> Self {
        self.inf_string = inf_string;
        self
    }

    // BUILDERS

    /// Determine if `nan_str` is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn nan_str_is_valid(&self) -> bool {
        if self.nan_string.is_none() {
            return true;
        }

        let nan = unwrap_str(self.nan_string);
        let length = nan.len();
        if length == 0 || length > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(nan[0], b'N' | b'n') {
            false
        } else if !is_valid_letter_slice(nan) {
            false
        } else {
            true
        }
    }

    /// Determine if `inf_str` is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn inf_str_is_valid(&self) -> bool {
        if self.inf_string.is_none() {
            return true;
        }

        let inf = unwrap_str(self.inf_string);
        let length = inf.len();
        if length == 0 || length > MAX_SPECIAL_STRING_LENGTH {
            false
        } else if !matches!(inf[0], b'I' | b'i') {
            false
        } else if !is_valid_letter_slice(inf) {
            false
        } else {
            true
        }
    }

    /// Check if the builder state is valid.
    #[inline(always)]
    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    pub const fn is_valid(&self) -> bool {
        if !is_valid_ascii(self.exponent) {
            false
        } else if !is_valid_ascii(self.decimal_point) {
            false
        } else if !self.nan_str_is_valid() {
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
    /// Safe as long as `is_valid` is true. If `nan_string` or `inf_string`
    /// are too long, writing special floats may lead to buffer overflows,
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
            exponent: self.exponent,
            decimal_point: self.decimal_point,
            nan_string: self.nan_string,
            inf_string: self.inf_string,
        }
    }

    /// Build the Options struct.
    #[inline(always)]
    #[allow(clippy::if_same_then_else)]
    pub const fn build(&self) -> Result<Options> {
        if self.nan_string.is_some() {
            let nan = unwrap_str(self.nan_string);
            if nan.is_empty() || !matches!(nan[0], b'N' | b'n') {
                return Err(Error::InvalidNanString);
            } else if !is_valid_letter_slice(nan) {
                return Err(Error::InvalidNanString);
            } else if nan.len() > MAX_SPECIAL_STRING_LENGTH {
                return Err(Error::NanStringTooLong);
            }
        }

        if self.inf_string.is_some() {
            let inf = unwrap_str(self.inf_string);
            if inf.is_empty() || !matches!(inf[0], b'I' | b'i') {
                return Err(Error::InvalidInfString);
            } else if !is_valid_letter_slice(inf) {
                return Err(Error::InvalidInfString);
            } else if inf.len() > MAX_SPECIAL_STRING_LENGTH {
                return Err(Error::InfStringTooLong);
            }
        }

        let min_digits = unwrap_or_zero_usize(self.min_significant_digits);
        let max_digits = unwrap_or_max_usize(self.max_significant_digits);
        if max_digits < min_digits {
            Err(Error::InvalidFloatPrecision)
        } else if unwrap_or_zero_i32(self.negative_exponent_break) > 0 {
            Err(Error::InvalidNegativeExponentBreak)
        } else if unwrap_or_zero_i32(self.positive_exponent_break) < 0 {
            Err(Error::InvalidPositiveExponentBreak)
        } else if !is_valid_ascii(self.exponent) {
            Err(Error::InvalidExponentSymbol)
        } else if !is_valid_ascii(self.decimal_point) {
            Err(Error::InvalidDecimalPoint)
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
///     .nan_string(Some(b"NaN"))
///     .inf_string(Some(b"Inf"))
///     .build()
///     .unwrap();
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Character to designate the exponent component of a float.
    exponent: u8,
    /// Character to separate the integer from the fraction components.
    decimal_point: u8,
    /// String representation of Not A Number, aka `NaN`.
    nan_string: Option<&'static [u8]>,
    /// String representation of `Infinity`.
    inf_string: Option<&'static [u8]>,
}

impl Options {
    // CONSTRUCTORS

    /// Create options with default values.
    #[inline(always)]
    pub const fn new() -> Self {
        // SAFETY: always safe since it uses the default arguments.
        unsafe { Self::builder().build_unchecked() }
    }

    /// Create the default options for a given radix.
    #[inline(always)]
    #[cfg(feature = "power-of-two")]
    pub const fn from_radix(radix: u8) -> Self {
        // Need to determine the correct exponent character ('e' or '^'),
        // since the default character is `e` normally, but this is a valid
        // digit for radix >= 15.
        let mut builder = Self::builder();
        if radix >= 15 {
            builder = builder.exponent(b'^');
        }
        // SAFETY: always safe since it uses the default arguments.
        unsafe { builder.build_unchecked() }
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

    /// Get the character to designate the exponent component of a float.
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        self.exponent
    }

    /// Get the character to separate the integer from the fraction components.
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        self.decimal_point
    }

    /// Get the string representation for `NaN`.
    #[inline(always)]
    pub const fn nan_string(&self) -> Option<&'static [u8]> {
        self.nan_string
    }

    /// Get the short string representation for `Infinity`.
    #[inline(always)]
    pub const fn inf_string(&self) -> Option<&'static [u8]> {
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

    /// Set the character to designate the exponent component of a float.
    ///
    /// # Safety
    ///
    /// Always safe, but may produce invalid output if the exponent
    /// is not a valid ASCII character.
    #[inline(always)]
    pub unsafe fn set_exponent(&mut self, exponent: u8) {
        self.exponent = exponent;
    }

    /// Set the character to separate the integer from the fraction components.
    ///
    /// # Safety
    ///
    /// Always safe, but may produce invalid output if the decimal point
    /// is not a valid ASCII character.
    #[inline(always)]
    pub unsafe fn set_decimal_point(&mut self, decimal_point: u8) {
        self.decimal_point = decimal_point;
    }

    /// Set the string representation for `NaN`.
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Unsafe if `nan_string.len() > MAX_SPECIAL_STRING_LENGTH`. This might
    /// cause a special string larger than the buffer length to be written,
    /// causing a buffer overflow, potentially a severe security vulnerability.
    #[inline(always)]
    pub unsafe fn set_nan_string(&mut self, nan_string: Option<&'static [u8]>) {
        self.nan_string = nan_string
    }

    /// Set the short string representation for `Infinity`
    /// Unsafe, use the builder API for option validation.
    ///
    /// # Safety
    ///
    /// Unsafe if `nan_string.len() > MAX_SPECIAL_STRING_LENGTH`. This might
    /// cause a special string larger than the buffer length to be written,
    /// causing a buffer overflow, potentially a severe security vulnerability.
    #[inline(always)]
    pub unsafe fn set_inf_string(&mut self, inf_string: Option<&'static [u8]>) {
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
            exponent: self.exponent,
            decimal_point: self.decimal_point,
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

impl WriteOptions for Options {
    #[inline(always)]
    fn is_valid(&self) -> bool {
        Self::is_valid(self)
    }

    #[inline(always)]
    fn buffer_size<T: FormattedSize, const FORMAT: u128>(&self) -> usize {
        let format = NumberFormat::<{ FORMAT }> {};

        // At least 2 for the decimal point and sign.
        let mut count: usize = 2;

        // First need to calculate maximum number of digits from leading or
        // trailing zeros, IE, the exponent break.
        if !format.no_exponent_notation() {
            let min_exp = self.negative_exponent_break().map_or(-5, |x| x.get());
            let max_exp = self.positive_exponent_break().map_or(9, |x| x.get());
            let exp = min_exp.abs().max(max_exp) as usize;
            if cfg!(feature = "power-of-two") && exp < 13 {
                // 11 for the exponent digits in binary, 1 for the sign, 1 for the symbol
                count += 13;
            } else if exp < 5 {
                // 3 for the exponent digits in decimal, 1 for the sign, 1 for the symbol
                count += 5;
            } else {
                // More leading or trailing zeros than the exponent digits.
                count += exp;
            }
        } else if cfg!(feature = "power-of-two") {
            // Min is 2^-1075.
            count += 1075;
        } else {
            // Min is 10^-324.
            count += 324;
        }

        // Now add the number of significant digits.
        let radix = format.radix();
        let formatted_digits = if radix == 10 {
            // Really should be 18, but add some extra to be cautious.
            28
        } else {
            //  BINARY:
            //      53 significant mantissa bits for binary, add a few extra.
            //  RADIX:
            //      Our limit is `delta`. The maximum relative delta is 2.22e-16,
            //      around 1. If we have values below 1, our delta is smaller, but
            //      the max fraction is also a lot smaller. Above, and our fraction
            //      must be < 1.0, so our delta is less significant. Therefore,
            //      if our fraction is just less than 1, for a float near 2.0,
            //      we can do at **maximum** 33 digits (for base 3). Let's just
            //      assume it's a lot higher, and go with 64.
            64
        };
        let digits = if let Some(max_digits) = self.max_significant_digits() {
            formatted_digits.min(max_digits.get())
        } else {
            formatted_digits
        };
        let digits = if let Some(min_digits) = self.min_significant_digits() {
            digits.max(min_digits.get())
        } else {
            formatted_digits
        };
        count += digits;

        count
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

/// Unwrap `Option` as a const fn.
#[inline(always)]
const fn unwrap_str(option: Option<&'static [u8]>) -> &'static [u8] {
    match option {
        Some(x) => x,
        None => &[],
    }
}

// PRE-DEFINED CONSTANTS
// ---------------------

// Only constants that differ from the standard version are included.
// SAFETY: all of these are safe, since they are checked to be valid
// after calling `build_unchecked`. Furthermore, even though the methods
// are marked as `unsafe`, none of the produced options can cause memory
// safety issues since the special strings are smaller than the buffer size.

/// Standard number format.
#[rustfmt::skip]
pub const STANDARD: Options = Options::new();
const_assert!(STANDARD.is_valid());

/// Numerical format with a decimal comma.
/// This is the standard numerical format for most of the world.
#[rustfmt::skip]
pub const DECIMAL_COMMA: Options = unsafe {
    Options::builder()
        .decimal_point(b',')
        .build_unchecked()
};
const_assert!(DECIMAL_COMMA.is_valid());

/// Numerical format for hexadecimal floats, which use a `p` exponent.
#[rustfmt::skip]
pub const HEX_FLOAT: Options = unsafe {
    Options::builder()
        .exponent(b'p')
        .build_unchecked()
};
const_assert!(HEX_FLOAT.is_valid());

/// Numerical format where `^` is used as the exponent notation character.
/// This isn't very common, but is useful when `e` or `p` are valid digits.
#[rustfmt::skip]
pub const CARAT_EXPONENT: Options = unsafe {
    Options::builder()
        .exponent(b'^')
        .build_unchecked()
};
const_assert!(CARAT_EXPONENT.is_valid());

/// Number format for a Rust literal floating-point number.
#[rustfmt::skip]
pub const RUST_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::RUST_LITERAL)
        .inf_string(options::RUST_LITERAL)
        .build_unchecked()
};
const_assert!(RUST_LITERAL.is_valid());

/// Number format for a Python literal floating-point number.
#[rustfmt::skip]
pub const PYTHON_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::PYTHON_LITERAL)
        .inf_string(options::PYTHON_LITERAL)
        .build_unchecked()
};
const_assert!(PYTHON_LITERAL.is_valid());

/// Number format for a C++ literal floating-point number.
#[rustfmt::skip]
pub const CXX_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::CXX_LITERAL_NAN)
        .inf_string(options::CXX_LITERAL_INF)
        .build_unchecked()
};
const_assert!(CXX_LITERAL.is_valid());

/// Number format for a C literal floating-point number.
#[rustfmt::skip]
pub const C_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::C_LITERAL_NAN)
        .inf_string(options::C_LITERAL_INF)
        .build_unchecked()
};
const_assert!(CXX_LITERAL.is_valid());

/// Number format for a Ruby literal floating-point number.
#[rustfmt::skip]
pub const RUBY_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::RUBY)
        .inf_string(options::RUBY)
        .build_unchecked()
};
const_assert!(RUBY_LITERAL.is_valid());

/// Number format to parse a Ruby float from string.
#[rustfmt::skip]
pub const RUBY_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::RUBY)
        .inf_string(options::RUBY)
        .build_unchecked()
};
const_assert!(RUBY_STRING.is_valid());

/// Number format for a Swift literal floating-point number.
#[rustfmt::skip]
pub const SWIFT_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::SWIFT_LITERAL)
        .inf_string(options::SWIFT_LITERAL)
        .build_unchecked()
};
const_assert!(SWIFT_LITERAL.is_valid());

/// Number format for a Go literal floating-point number.
#[rustfmt::skip]
pub const GO_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::GO_LITERAL)
        .inf_string(options::GO_LITERAL)
        .build_unchecked()
};
const_assert!(GO_LITERAL.is_valid());

/// Number format for a Haskell literal floating-point number.
#[rustfmt::skip]
pub const HASKELL_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::HASKELL_LITERAL)
        .inf_string(options::HASKELL_LITERAL)
        .build_unchecked()
};
const_assert!(HASKELL_LITERAL.is_valid());

/// Number format to parse a Haskell float from string.
#[rustfmt::skip]
pub const HASKELL_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::HASKELL_STRING_INF)
        .build_unchecked()
};
const_assert!(HASKELL_STRING.is_valid());

/// Number format for a Javascript literal floating-point number.
#[rustfmt::skip]
pub const JAVASCRIPT_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::JAVASCRIPT_INF)
        .build_unchecked()
};
const_assert!(JAVASCRIPT_LITERAL.is_valid());

/// Number format to parse a Javascript float from string.
#[rustfmt::skip]
pub const JAVASCRIPT_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::JAVASCRIPT_INF)
        .build_unchecked()
};
const_assert!(JAVASCRIPT_STRING.is_valid());

/// Number format for a Perl literal floating-point number.
#[rustfmt::skip]
pub const PERL_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::PERL_LITERAL)
        .inf_string(options::PERL_LITERAL)
        .build_unchecked()
};
const_assert!(PERL_LITERAL.is_valid());

/// Number format for a PHP literal floating-point number.
#[rustfmt::skip]
pub const PHP_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::PHP_LITERAL_NAN)
        .inf_string(options::PHP_LITERAL_INF)
        .build_unchecked()
};
const_assert!(PHP_LITERAL.is_valid());

/// Number format for a Java literal floating-point number.
#[rustfmt::skip]
pub const JAVA_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::JAVA_LITERAL)
        .inf_string(options::JAVA_LITERAL)
        .build_unchecked()
};
const_assert!(JAVA_LITERAL.is_valid());

/// Number format to parse a Java float from string.
#[rustfmt::skip]
pub const JAVA_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::JAVA_STRING_INF)
        .build_unchecked()
};
const_assert!(JAVA_STRING.is_valid());

/// Number format for an R literal floating-point number.
#[rustfmt::skip]
pub const R_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::R_LITERAL_INF)
        .build_unchecked()
};
const_assert!(R_LITERAL.is_valid());

/// Number format for a Kotlin literal floating-point number.
#[rustfmt::skip]
pub const KOTLIN_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::KOTLIN_LITERAL)
        .inf_string(options::KOTLIN_LITERAL)
        .build_unchecked()
};
const_assert!(KOTLIN_LITERAL.is_valid());

/// Number format to parse a Kotlin float from string.
#[rustfmt::skip]
pub const KOTLIN_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::KOTLIN_STRING_INF)
        .build_unchecked()
};
const_assert!(KOTLIN_STRING.is_valid());

/// Number format for a Julia literal floating-point number.
#[rustfmt::skip]
pub const JULIA_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::JULIA_LITERAL_INF)
        .build_unchecked()
};
const_assert!(JULIA_LITERAL.is_valid());

/// Number format for a C# literal floating-point number.
#[rustfmt::skip]
pub const CSHARP_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::CSHARP_LITERAL)
        .inf_string(options::CSHARP_LITERAL)
        .build_unchecked()
};
const_assert!(CSHARP_LITERAL.is_valid());

/// Number format to parse a C# float from string.
#[rustfmt::skip]
pub const CSHARP_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::CSHARP_STRING_INF)
        .build_unchecked()
};
const_assert!(CSHARP_STRING.is_valid());

/// Number format for a Kawa literal floating-point number.
#[rustfmt::skip]
pub const KAWA_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::KAWA)
        .inf_string(options::KAWA)
        .build_unchecked()
};
const_assert!(KAWA_LITERAL.is_valid());

/// Number format to parse a Kawa float from string.
#[rustfmt::skip]
pub const KAWA_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::KAWA)
        .inf_string(options::KAWA)
        .build_unchecked()
};
const_assert!(KAWA_STRING.is_valid());

/// Number format for a Gambit-C literal floating-point number.
#[rustfmt::skip]
pub const GAMBITC_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::GAMBITC)
        .inf_string(options::GAMBITC)
        .build_unchecked()
};
const_assert!(GAMBITC_LITERAL.is_valid());

/// Number format to parse a Gambit-C float from string.
#[rustfmt::skip]
pub const GAMBITC_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::GAMBITC)
        .inf_string(options::GAMBITC)
        .build_unchecked()
};
const_assert!(GAMBITC_STRING.is_valid());

/// Number format for a Guile literal floating-point number.
#[rustfmt::skip]
pub const GUILE_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::GUILE)
        .inf_string(options::GUILE)
        .build_unchecked()
};
const_assert!(GUILE_LITERAL.is_valid());

/// Number format to parse a Guile float from string.
#[rustfmt::skip]
pub const GUILE_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::GUILE)
        .inf_string(options::GUILE)
        .build_unchecked()
};
const_assert!(GUILE_STRING.is_valid());

/// Number format for a Clojure literal floating-point number.
#[rustfmt::skip]
pub const CLOJURE_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::CLOJURE_LITERAL)
        .inf_string(options::CLOJURE_LITERAL)
        .build_unchecked()
};
const_assert!(CLOJURE_LITERAL.is_valid());

/// Number format to parse a Clojure float from string.
#[rustfmt::skip]
pub const CLOJURE_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::CLOJURE_STRING_INF)
        .build_unchecked()
};
const_assert!(CLOJURE_STRING.is_valid());

/// Number format for an Erlang literal floating-point number.
#[rustfmt::skip]
pub const ERLANG_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::ERLANG_LITERAL_NAN)
        .build_unchecked()
};
const_assert!(ERLANG_LITERAL.is_valid());

/// Number format to parse an Erlang float from string.
#[rustfmt::skip]
pub const ERLANG_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::ERLANG_STRING)
        .inf_string(options::ERLANG_STRING)
        .build_unchecked()
};
const_assert!(ERLANG_STRING.is_valid());

/// Number format for an Elm literal floating-point number.
#[rustfmt::skip]
pub const ELM_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::ELM_LITERAL)
        .inf_string(options::ELM_LITERAL)
        .build_unchecked()
};
const_assert!(ELM_LITERAL.is_valid());

/// Number format to parse an Elm float from string.
#[rustfmt::skip]
pub const ELM_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::ELM_STRING_NAN)
        .inf_string(options::ELM_STRING_INF)
        .build_unchecked()
};
const_assert!(ELM_STRING.is_valid());

/// Number format for a Scala literal floating-point number.
#[rustfmt::skip]
pub const SCALA_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::SCALA_LITERAL)
        .inf_string(options::SCALA_LITERAL)
        .build_unchecked()
};
const_assert!(SCALA_LITERAL.is_valid());

/// Number format to parse a Scala float from string.
#[rustfmt::skip]
pub const SCALA_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::SCALA_STRING_INF)
        .build_unchecked()
};
const_assert!(SCALA_STRING.is_valid());

/// Number format for an Elixir literal floating-point number.
#[rustfmt::skip]
pub const ELIXIR_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::ELIXIR)
        .inf_string(options::ELIXIR)
        .build_unchecked()
};
const_assert!(ELIXIR_LITERAL.is_valid());

/// Number format to parse an Elixir float from string.
#[rustfmt::skip]
pub const ELIXIR_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::ELIXIR)
        .inf_string(options::ELIXIR)
        .build_unchecked()
};
const_assert!(ELIXIR_STRING.is_valid());

/// Number format for a FORTRAN literal floating-point number.
#[rustfmt::skip]
pub const FORTRAN_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::FORTRAN_LITERAL)
        .inf_string(options::FORTRAN_LITERAL)
        .build_unchecked()
};
const_assert!(FORTRAN_LITERAL.is_valid());

/// Number format for a D literal floating-point number.
#[rustfmt::skip]
pub const D_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::D_LITERAL)
        .inf_string(options::D_LITERAL)
        .build_unchecked()
};
const_assert!(D_LITERAL.is_valid());

/// Number format for a Coffeescript literal floating-point number.
#[rustfmt::skip]
pub const COFFEESCRIPT_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::COFFEESCRIPT_INF)
        .build_unchecked()
};
const_assert!(COFFEESCRIPT_LITERAL.is_valid());

/// Number format to parse a Coffeescript float from string.
#[rustfmt::skip]
pub const COFFEESCRIPT_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::COFFEESCRIPT_INF)
        .build_unchecked()
};
const_assert!(COFFEESCRIPT_STRING.is_valid());

/// Number format for a COBOL literal floating-point number.
#[rustfmt::skip]
pub const COBOL_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::COBOL)
        .inf_string(options::COBOL)
        .build_unchecked()
};
const_assert!(COBOL_LITERAL.is_valid());

/// Number format to parse a COBOL float from string.
#[rustfmt::skip]
pub const COBOL_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::COBOL)
        .inf_string(options::COBOL)
        .build_unchecked()
};
const_assert!(COBOL_STRING.is_valid());

/// Number format for an F# literal floating-point number.
#[rustfmt::skip]
pub const FSHARP_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::FSHARP_LITERAL_NAN)
        .inf_string(options::FSHARP_LITERAL_INF)
        .build_unchecked()
};
const_assert!(FSHARP_LITERAL.is_valid());

/// Number format for a Visual Basic literal floating-point number.
#[rustfmt::skip]
pub const VB_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::VB_LITERAL)
        .inf_string(options::VB_LITERAL)
        .build_unchecked()
};
const_assert!(VB_LITERAL.is_valid());

/// Number format to parse a Visual Basic float from string.
#[rustfmt::skip]
pub const VB_STRING: Options = unsafe {
    Options::builder()
        .inf_string(options::VB_STRING_INF)
        .build_unchecked()
};
const_assert!(VB_STRING.is_valid());

/// Number format for an OCaml literal floating-point number.
#[rustfmt::skip]
pub const OCAML_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::OCAML_LITERAL_NAN)
        .inf_string(options::OCAML_LITERAL_INF)
        .build_unchecked()
};
const_assert!(OCAML_LITERAL.is_valid());

/// Number format for an Objective-C literal floating-point number.
#[rustfmt::skip]
pub const OBJECTIVEC_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::OBJECTIVEC)
        .inf_string(options::OBJECTIVEC)
        .build_unchecked()
};
const_assert!(OBJECTIVEC_LITERAL.is_valid());

/// Number format to parse an Objective-C float from string.
#[rustfmt::skip]
pub const OBJECTIVEC_STRING: Options = unsafe {
    Options::builder()
        .nan_string(options::OBJECTIVEC)
        .inf_string(options::OBJECTIVEC)
        .build_unchecked()
};
const_assert!(OBJECTIVEC_STRING.is_valid());

/// Number format for an ReasonML literal floating-point number.
#[rustfmt::skip]
pub const REASONML_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::REASONML_LITERAL_NAN)
        .inf_string(options::REASONML_LITERAL_INF)
        .build_unchecked()
};
const_assert!(REASONML_LITERAL.is_valid());

/// Number format for a MATLAB literal floating-point number.
#[rustfmt::skip]
pub const MATLAB_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::MATLAB_LITERAL_INF)
        .build_unchecked()
};
const_assert!(MATLAB_LITERAL.is_valid());

/// Number format for a Zig literal floating-point number.
#[rustfmt::skip]
pub const ZIG_LITERAL: Options = unsafe {
    Options::builder()
        .nan_string(options::ZIG_LITERAL)
        .inf_string(options::ZIG_LITERAL)
        .build_unchecked()
};
const_assert!(ZIG_LITERAL.is_valid());

/// Number format for a Safe literal floating-point number.
#[rustfmt::skip]
pub const SAGE_LITERAL: Options = unsafe {
    Options::builder()
        .inf_string(options::SAGE_LITERAL_INF)
        .build_unchecked()
};
const_assert!(SAGE_LITERAL.is_valid());

/// Number format for a JSON literal floating-point number.
#[rustfmt::skip]
pub const JSON: Options = unsafe {
    Options::builder()
        .nan_string(options::JSON)
        .inf_string(options::JSON)
        .build_unchecked()
};
const_assert!(JSON.is_valid());

/// Number format for a TOML literal floating-point number.
#[rustfmt::skip]
pub const TOML: Options = unsafe {
    Options::builder()
        .nan_string(options::TOML)
        .inf_string(options::TOML)
        .build_unchecked()
};
const_assert!(TOML.is_valid());

/// Number format for a YAML literal floating-point number.
#[rustfmt::skip]
pub const YAML: Options = JSON;

/// Number format for an XML literal floating-point number.
#[rustfmt::skip]
pub const XML: Options = unsafe {
    Options::builder()
        .inf_string(options::XML_INF)
        .build_unchecked()
};
const_assert!(XML.is_valid());

/// Number format for a SQLite literal floating-point number.
#[rustfmt::skip]
pub const SQLITE: Options = unsafe {
    Options::builder()
        .nan_string(options::SQLITE)
        .inf_string(options::SQLITE)
        .build_unchecked()
};
const_assert!(SQLITE.is_valid());

/// Number format for a PostgreSQL literal floating-point number.
#[rustfmt::skip]
pub const POSTGRESQL: Options = unsafe {
    Options::builder()
        .nan_string(options::POSTGRESQL)
        .inf_string(options::POSTGRESQL)
        .build_unchecked()
};
const_assert!(POSTGRESQL.is_valid());

/// Number format for a MySQL literal floating-point number.
#[rustfmt::skip]
pub const MYSQL: Options = unsafe {
    Options::builder()
        .nan_string(options::MYSQL)
        .inf_string(options::MYSQL)
        .build_unchecked()
};
const_assert!(MYSQL.is_valid());

/// Number format for a MongoDB literal floating-point number.
#[rustfmt::skip]
pub const MONGODB: Options = unsafe {
    Options::builder()
        .inf_string(options::MONGODB_INF)
        .build_unchecked()
};
const_assert!(MONGODB.is_valid());
