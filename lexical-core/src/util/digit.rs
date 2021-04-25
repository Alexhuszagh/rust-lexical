//! Simple algorithms to optimize for checking if a character is a digit.

/// Convert character to digit.
/// Optimize for case when we have a radix <= 10.
#[inline(always)]
#[cfg(feature = "radix")]
pub(crate) fn is_digit(c: u8, radix: u32) -> bool {
    let digit = if radix <= 10 {
        c - b'0'
    } else {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'z' => c - b'a' + 10,
            b'A'..=b'Z' => c - b'A' + 10,
            _ => 37,
        }
    };
    (digit as u32) < radix
}

/// Convert character to digit.
/// Optimize for case when we have a radix == 10.
#[inline(always)]
#[cfg(not(feature = "radix"))]
pub(crate) fn is_digit(c: u8, _: u32) -> bool {
    let digit = c - b'0';
    (digit as u32) < 10
}
