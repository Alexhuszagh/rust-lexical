//! Consume digits and digit separators.

// Convert radix to value.
macro_rules! to_digit {
    ($c:expr, $radix:expr) => (($c as char).to_digit($radix));
}

// Convert character to digit.
perftools_inline_always!{
#[allow(unused_variables)]
fn is_digit(c: u8, radix: u32) -> bool {
    to_digit!(c, radix).is_some()
}}

// Consume until a non-digit separator is found.
// Does not consume any digit separators.
perftools_inline!{
pub(super) fn consume_digits_no_separator<'a>(digits: &'a [u8], radix: u32)
-> (&'a [u8], &'a [u8])
{
    match digits.iter().position(|&c| !is_digit(c, radix)) {
        Some(v) => (&digits[..v], &digits[v..]),
        None    => (&digits[..], &digits[digits.len()..]),
    }
}}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_digits_no_separator() {
        assert_eq!(consume_digits_no_separator(b!("123.45"), 10), (b!("123"), b!(".45")));
        assert_eq!(consume_digits_no_separator(b!("_45"), 10), (b!(""), b!("_45")));
        assert_eq!(consume_digits_no_separator(b!("1e45"), 10), (b!("1"), b!("e45")));
        assert_eq!(consume_digits_no_separator(b!("1e"), 10), (b!("1"), b!("e")));
        assert_eq!(consume_digits_no_separator(b!("1"), 10), (b!("1"), b!("")));
    }
}

