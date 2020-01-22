//! Enumerations for the sign-bit of a number.

use super::format::NumberFormat;

// ENUMERATION

/// Enumeration for the sign of a a number.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sign {
    /// Negative value.
    Negative,
    /// Positive value.
    Positive,
}

// HELPERS

// Get if an option contains a digit separator.
#[inline(always)]
#[cfg(feature = "format")]
fn is_digit_separator(option: Option<&u8>, digit_separator: u8) -> bool {
    option == Some(&digit_separator)
}

// Convert option of byte to option of sign.
#[inline(always)]
#[cfg(feature = "format")]
fn to_sign(option: Option<&u8>) -> Option<Sign> {
    match option {
        Some(&b'+') => Some(Sign::Positive),
        Some(&b'-') => Some(Sign::Negative),
        _           => None
    }
}

// PARSE

/// Find and parse sign without any possible digit separators.
#[inline(always)]
pub(crate) fn parse_sign_no_separator<'a>(bytes: &'a [u8], _: u8)
    -> (Sign, &'a [u8])
{
    match bytes.get(0) {
        Some(&b'+') => (Sign::Positive, &index!(bytes[1..])),
        Some(&b'-') => (Sign::Negative, &index!(bytes[1..])),
        _           => (Sign::Positive, bytes)
    }
}

/// Find and parse sign with leading and consecutive digit separators.
///
/// We need to consider the following possibilities:
///     1). _*[+-]\d+
#[inline(always)]
#[cfg(feature = "format")]
pub(crate) fn parse_sign_lc_separator<'a>(bytes: &'a [u8], digit_separator: u8)
    -> (Sign, &'a [u8])
{
    let mut index = 0;
    while is_digit_separator(bytes.get(index), digit_separator) {
        index += 1;
    }
    if let Some(sign) = to_sign(bytes.get(index)) {
        (sign, &index!(bytes[index+1..]))
    } else {
        (Sign::Positive, bytes)
    }
}

/// Find and parse sign with leading digit separators.
///
/// We need to consider the following possibilities:
///     1). [+-]\d+
///     2). _[+-]\d+
#[inline(always)]
#[cfg(feature = "format")]
pub(crate) fn parse_sign_l_separator<'a>(bytes: &'a [u8], digit_separator: u8)
    -> (Sign, &'a [u8])
{
    let b0 = bytes.get(0);
    if let Some(sign) = to_sign(b0) {
        (sign, &index!(bytes[1..]))
    } else if is_digit_separator(b0, digit_separator) {
        if let Some(sign) = to_sign(bytes.get(1)) {
            (sign, &index!(bytes[2..]))
        } else {
            (Sign::Positive, bytes)
        }
    } else {
        (Sign::Positive, bytes)
    }
}

/// Find and parse sign with digit separators.
#[inline(always)]
#[cfg(feature = "format")]
pub(crate) fn parse_sign_separator<'a>(bytes: &'a [u8], format: NumberFormat)
    -> (Sign, &'a [u8])
{
    // If the integer cannot have leading digit separators, we know the sign
    // byte must by the first byte. Otherwise, we must consider digit separators
    // before the sign byte.
    let leading = format.integer_leading_digit_separator();
    let consecutive = format.integer_consecutive_digit_separator();
    match (leading, consecutive) {
        (true, true)    => parse_sign_lc_separator(bytes, format.digit_separator()),
        (true, false)   => parse_sign_l_separator(bytes, format.digit_separator()),
        (false, _)      => parse_sign_no_separator(bytes, format.digit_separator())
    }
}

/// Find and parse sign.
#[inline]
pub(crate) fn parse_sign<'a>(bytes: &'a [u8], format: NumberFormat)
    -> (Sign, &'a [u8])
{
    #[cfg(not(feature = "format"))]
    return parse_sign_no_separator(bytes, format.digit_separator());

    #[cfg(feature = "format")]
    return parse_sign_separator(bytes, format);
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::*;

    #[test]
    fn parse_sign_no_separator_test() {
        assert_eq!(parse_sign_no_separator(b"", b'_'), (Sign::Positive, b!("")));
        assert_eq!(parse_sign_no_separator(b"+", b'_'), (Sign::Positive, b!("")));
        assert_eq!(parse_sign_no_separator(b"-", b'_'), (Sign::Negative, b!("")));
        assert_eq!(parse_sign_no_separator(b"+5", b'_'), (Sign::Positive, b!("5")));
        assert_eq!(parse_sign_no_separator(b"-5", b'_'), (Sign::Negative, b!("5")));
        assert_eq!(parse_sign_no_separator(b"_-5", b'_'), (Sign::Positive, b!("_-5")));
        assert_eq!(parse_sign_no_separator(b"___-5", b'_'), (Sign::Positive, b!("___-5")));
    }

    #[test]
    #[cfg(feature = "format")]
    fn parse_sign_lc_separator_test() {
        assert_eq!(parse_sign_lc_separator(b"", b'_'), (Sign::Positive, b!("")));
        assert_eq!(parse_sign_lc_separator(b"+", b'_'), (Sign::Positive, b!("")));
        assert_eq!(parse_sign_lc_separator(b"-", b'_'), (Sign::Negative, b!("")));
        assert_eq!(parse_sign_lc_separator(b"+5", b'_'), (Sign::Positive, b!("5")));
        assert_eq!(parse_sign_lc_separator(b"-5", b'_'), (Sign::Negative, b!("5")));
        assert_eq!(parse_sign_lc_separator(b"_-5", b'_'), (Sign::Negative, b!("5")));
        assert_eq!(parse_sign_lc_separator(b"___-5", b'_'), (Sign::Negative, b!("5")));
    }

    #[test]
    #[cfg(feature = "format")]
    fn parse_sign_l_separator_test() {
        assert_eq!(parse_sign_l_separator(b"", b'_'), (Sign::Positive, b!("")));
        assert_eq!(parse_sign_l_separator(b"+", b'_'), (Sign::Positive, b!("")));
        assert_eq!(parse_sign_l_separator(b"-", b'_'), (Sign::Negative, b!("")));
        assert_eq!(parse_sign_l_separator(b"+5", b'_'), (Sign::Positive, b!("5")));
        assert_eq!(parse_sign_l_separator(b"-5", b'_'), (Sign::Negative, b!("5")));
        assert_eq!(parse_sign_l_separator(b"_-5", b'_'), (Sign::Negative, b!("5")));
        assert_eq!(parse_sign_l_separator(b"___-5", b'_'), (Sign::Positive, b!("___-5")));
    }

    #[test]
    #[cfg(feature = "format")]
    fn parse_sign_separator_test() {
        let format = NumberFormat::ignore(b'_').unwrap();
        assert_eq!(parse_sign_separator(b"", format), (Sign::Positive, b!("")));
        assert_eq!(parse_sign_separator(b"+", format), (Sign::Positive, b!("")));
        assert_eq!(parse_sign_separator(b"-", format), (Sign::Negative, b!("")));
        assert_eq!(parse_sign_separator(b"+5", format), (Sign::Positive, b!("5")));
        assert_eq!(parse_sign_separator(b"-5", format), (Sign::Negative, b!("5")));
        assert_eq!(parse_sign_separator(b"_-5", format), (Sign::Negative, b!("5")));
        assert_eq!(parse_sign_separator(b"___-5", format), (Sign::Negative, b!("5")));
    }

    #[test]
    fn parse_sign_test() {
        let format = NumberFormat::standard().unwrap();
        assert_eq!(parse_sign(b"", format), (Sign::Positive, b!("")));
        assert_eq!(parse_sign(b"+", format), (Sign::Positive, b!("")));
        assert_eq!(parse_sign(b"-", format), (Sign::Negative, b!("")));
        assert_eq!(parse_sign(b"+5", format), (Sign::Positive, b!("5")));
        assert_eq!(parse_sign(b"-5", format), (Sign::Negative, b!("5")));
        assert_eq!(parse_sign(b"_-5", format), (Sign::Positive, b!("_-5")));
        assert_eq!(parse_sign(b"___-5", format), (Sign::Positive, b!("___-5")));
    }
}
