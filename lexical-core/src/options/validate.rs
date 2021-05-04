//! Utilities to validate numerical formats.

#![cfg_attr(rustfmt, rustfmt::skip::macros(const_fn))]

use crate::util::*;

const_fn!(
    /// Determine if the digit separator is valid.
    #[inline(always)]
    pub(crate) const fn is_ascii(ch: u8) -> bool {
        ch < 0x80
    }
);

const_fn!(
    /// Determine if the control character is valid.
    ///
    /// This depends on the radix, so we can allow, say, `p` for an
    /// exponent character if the radix is 16 (C++ hexadecimal floats
    /// do this).
    #[inline(always)]
    pub(crate) const fn is_valid_control(ch: u8, radix: u32) -> bool {
        match ch {
            b'+' | b'-' => false,
            _ => is_ascii(ch) && !is_digit(ch, radix),
        }
    }
);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

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
}
