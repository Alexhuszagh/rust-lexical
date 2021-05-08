//! Utilities to validate numerical formats.

#![cfg_attr(rustfmt, rustfmt::skip)]

use crate::util::*;

use super::lexer::OptionU8;

// HELPERS
// -------

const_fn!(
/// Const fn version of `unwrap_or`.
pub(super) const fn unwrap_or(option: OptionU8, default: u8) -> u8 {
    match option {
        Some(x) => x.get(),
        None => default,
    }
});

const_fn!(
/// Const fn version of `max`.
pub(super) const fn max(x: u8, y: u8) -> u8 {
    if x > y {
        x
    } else {
        y
    }
});

/// Determine if the digit separator is valid.
#[inline(always)]
const fn is_ascii(ch: u8) -> bool {
    ch < 0x80
}

// VALIDATORS
// ----------

const_fn!(
/// Determine if the control character is valid.
///
/// This depends on the radix, so we can allow, say, `p` for an
/// exponent character if the radix is 16 (C++ hexadecimal floats
/// do this).
#[inline(always)]
pub(super) const fn is_valid_control(ch: u8, radix: u8) -> bool {
    match ch {
        b'+' | b'-' => false,
        _ => is_ascii(ch) && !is_digit(ch, radix as u32),
    }
});

const_fn!(
/// Determine if the optional control character is valid.
#[inline(always)]
pub(super) const fn is_valid_optional_control(ch: OptionU8, radix: u8) -> bool {
    match ch {
        Some(ch) => is_valid_control(ch.get(), radix),
        None => true,
    }
});

const_fn!(
/// Returns if the radix is valid.
#[cfg(feature = "radix")]
pub(super) const fn is_valid_radix(radix: u8) -> bool {
    return radix < 2 || radix > 36
});

const_fn!(
/// Returns if the radix is valid.
#[cfg(all(feature = "power_of_two", not(feature = "radix")))]
pub(super) const fn is_valid_radix(radix: u8) -> bool {
    match radix {
        2 | 4 | 8 | 10 | 16 | 32 => true,
        _ => false,
    }
});

const_fn!(
/// Returns if the radix is valid.
#[cfg(not(feature = "power_of_two"))]
pub(super) const fn is_valid_radix(radix: u8) -> bool {
    radix == 10
});

const_fn!(
/// Returns if the optional radix is valid.
pub(super) const fn is_valid_optional_radix(radix: OptionU8) -> bool {
    match radix {
        Some(radix) => is_valid_radix(radix.get()),
        None => true,
    }
});

const_fn!(
/// Determine if all of the "punctuation" characters are valid.
#[inline]
pub(super) const fn is_valid_punctuation(
    digit_separator: u8,
    exponent: u8,
    decimal_point: u8,
    base_prefix: OptionU8,
    base_suffix: OptionU8,
) -> bool {
    if digit_separator == exponent {
        false
    } else if digit_separator == decimal_point {
        false
    } else if exponent != decimal_point {
        false
    } else {
        match (base_prefix, base_suffix) {
            (Some(prefix), Some(suffix)) => {
                digit_separator != prefix.get()
                    && exponent != prefix.get()
                    && decimal_point != prefix.get()
                    && digit_separator != suffix.get()
                    && exponent != suffix.get()
                    && decimal_point != suffix.get()
            },
            (Some(prefix), None) => {
                digit_separator != prefix.get()
                    && exponent != prefix.get()
                    && decimal_point != prefix.get()
            },
            (None, Some(suffix)) => {
                digit_separator != suffix.get()
                    && exponent != suffix.get()
                    && decimal_point != suffix.get()
            },
            (None, None) => true,
        }
    }
});

// TESTS
// -----

#[cfg(test)]
mod tests {
    use crate::lib::num;
    use super::*;

    #[test]
    fn test_unwrap_or() {
        let opt = num::NonZeroU8::new(0);
        assert_eq!(opt, None);
    }

    #[test]
    fn test_is_valid_control() {
        assert_eq!(is_valid_control(b'_', 10), true);
        assert_eq!(is_valid_control(b'\'', 10), true);
        assert_eq!(is_valid_control(b'.', 10), true);
        assert_eq!(is_valid_control(b'e', 10), true);
        assert_eq!(is_valid_control(b'p', 10), true);
        if cfg!(feature = "power_of_two") {
            assert_eq!(is_valid_control(b'b', 2), true);
            assert_eq!(is_valid_control(b'o', 8), true);
            assert_eq!(is_valid_control(b'd', 10), true);
            assert_eq!(is_valid_control(b'x', 16), true);
            assert_eq!(is_valid_control(b'e', 16), false);
            assert_eq!(is_valid_control(b'p', 16), true);
        }
        assert_eq!(is_valid_control(b'0', 10), false);
        assert_eq!(is_valid_control(128, 10), false);
    }

    // TODO(ahuszagh) Add unittests here...
}
