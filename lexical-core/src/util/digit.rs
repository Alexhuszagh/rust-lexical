use crate::lib::result::Result;

// DIGITS

cfg_if! {
if #[cfg(any(feature = "atof", all(feature = "atoi", feature = "format")))] {
    /// Get if the character is a digit.
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
                _ => return false,
            }
        };
        (digit as u32) < radix
    }

    /// Get if the character is a digit.
    /// Optimize for case when we have a radix == 10.
    #[inline(always)]
    #[cfg(not(feature = "radix"))]
    pub(crate) fn is_digit(c: u8, _: u32) -> bool {
        let digit = c - b'0';
        (digit as u32) < 10
    }
}}   // cfg_if

/// Get if the character is not a digit.
#[inline(always)]
#[cfg(feature = "atof")]
pub(crate) fn is_not_digit_char(c: u8, radix: u32) -> bool {
    !is_digit(c, radix)
}

// Convert character to digit.
#[inline(always)]
#[cfg(any(
    feature = "atof",
    feature = "atoi",
    all(feature = "ftoa", feature = "radix")
))]
pub(crate) fn to_digit(c: u8, radix: u32) -> Option<u32> {
    (c as char).to_digit(radix)
}

// Convert character to digit.
#[inline(always)]
#[cfg(feature = "atof")]
pub(crate) fn to_digit_err<'a>(c: &'a u8, radix: u32) -> Result<u32, &'a u8> {
    match to_digit(*c, radix) {
        Some(v) => Ok(v),
        None    => Err(c),
    }
}
