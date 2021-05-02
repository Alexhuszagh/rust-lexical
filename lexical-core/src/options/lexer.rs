//! Configuration of valid characters for the numerical lexer.

use static_assertions::const_assert;
use crate::lib::{mem, num};

// LEXER FORMAT

/// Type with the exact same size as a `u8`.
type OptionU8 = Option<num::NonZeroU8>;

// Ensure the sizes are identical.
const_assert!(mem::size_of::<OptionU8>() == mem::size_of::<u8>());

/// Configuration of control and digit characters during parsing.
///
/// Use repr(align(8)) so we can guarantee that it pads to an 8-bit
/// boundary.
#[repr(C)]
#[repr(align(8))]
pub struct LexerFormat {
    /// Character to designate the exponent component of a float.
    exponent: u8,
    /// Character to separate the integer from the fraction components.
    decimal_point: u8,
    /// Radix for the mantissa digits.
    mantissa_radix: u8,
    /// Radix for the exponent. If not provided, defaults to `mantissa_radix`.
    /// IE, a base of 2 means we have `mantissa * 2^exponent`.
    exponent_base: OptionU8,
    /// Radix for the exponent digits. If not provided, defaults to `mantissa_radix`.
    exponent_radix: OptionU8,
    /// Character for the base prefix. If not provided, base prefixes are not allowed.
    ///
    /// The number will have then have the format `0$base_prefix...`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    base_prefix: OptionU8,
    /// Character for the base suffix. If not provided, base suffixes are not allowed.
    ///
    /// The number will have then have the format `...$base_suffix`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    base_suffix: OptionU8,
    /// Hidden for padding to ensure we are on an 8-byte boundary.
    __padding: u8,
}

impl LexerFormat {
    // TODO(ahuszagh) Add getters and setters here.
}
