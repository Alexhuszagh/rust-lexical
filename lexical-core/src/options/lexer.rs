//! Configuration of valid characters for the numerical lexer.

use super::config::*;

// LEXER FORMAT

/// Configuration of control and digit characters during parsing.
///
/// Use repr(align(8)) so we can guarantee that it pads to an 8-bit
/// boundary.
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LexerFormat {
    /// Character to designate the exponent component of a float.
    pub(super) exponent: u8,
    /// Character to separate the integer from the fraction components.
    pub(super) decimal_point: u8,
    /// Radix for the mantissa digits.
    pub(super) mantissa_radix: u8,
    /// Radix for the exponent. If not provided, defaults to `mantissa_radix`.
    /// IE, a base of 2 means we have `mantissa * 2^exponent`.
    pub(super) exponent_base: OptionU8,
    /// Radix for the exponent digits. If not provided, defaults to `mantissa_radix`.
    pub(super) exponent_radix: OptionU8,
    /// Character for the base prefix. If not provided, base prefixes are not allowed.
    ///
    /// The number will have then have the format `0$base_prefix...`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    pub(super) base_prefix: OptionU8,
    /// Character for the base suffix. If not provided, base suffixes are not allowed.
    ///
    /// The number will have then have the format `...$base_suffix`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    pub(super) base_suffix: OptionU8,
    /// Hidden for padding to ensure we are on an 8-byte boundary.
    pub(super) __padding: u8,
}

impl LexerFormat {
    // GETTERS

    /// Get the exponent character for the lexer format.
    #[inline(always)]
    pub const fn exponent(self) -> u8 {
        self.exponent
    }

    /// Get the decimal point character for the lexer format.
    #[inline(always)]
    pub const fn decimal_point(self) -> u8 {
        self.decimal_point
    }

    /// Get the radix for the mantissa digits for the lexer format.
    #[inline(always)]
    pub const fn mantissa_radix(self) -> u8 {
        self.mantissa_radix
    }

    /// Get the base for the exponent for the lexer format.
    ///
    /// IE, a base of 2 means we have `mantissa * 2^exponent`.
    /// If not provided, it defaults to `mantissa_radix`.
    #[inline(always)]
    pub const fn exponent_base(self) -> OptionU8 {
        self.exponent_base
    }

    /// Get the radix for the exponent digits.
    ///
    /// If not provided, defaults to `mantissa_radix`.
    #[inline(always)]
    pub const fn exponent_radix(self) -> OptionU8 {
        self.exponent_radix
    }

    /// Get the character for the base prefix.
    ///
    /// If not provided, base prefixes are not allowed.
    /// The number will have then have the format `0$base_prefix...`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    #[inline(always)]
    pub const fn base_prefix(self) -> OptionU8 {
        self.base_prefix
    }

    /// Character for the base suffix.
    ///
    /// If not provided, base suffixes are not allowed.
    /// The number will have then have the format `...$base_suffix`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    #[inline(always)]
    pub const fn base_suffix(self) -> OptionU8 {
        self.base_suffix
    }

    // CONSTANTS

    /// Standard lexer format.
    #[doc(hidden)]
    pub const STANDARD: Self = Self {
        exponent: DEFAULT_EXPONENT,
        decimal_point: DEFAULT_DECIMAL_POINT,
        mantissa_radix: DEFAULT_RADIX,
        exponent_base: None,
        exponent_radix: None,
        base_prefix: None,
        base_suffix: None,
        __padding: b'\x00',
    };
}

// TODO(ahuszagh) Should have aliases for common types...
