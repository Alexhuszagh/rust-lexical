//! Trim leading and trailing 0s and digit separators.

use crate::util::*;

// Trim leading 0s.
// Does not consume any digit separators.
perftools_inline!{
pub(super) fn ltrim_zero_no_separator<'a>(bytes: &'a [u8])
    -> (&'a [u8], usize)
{
    ltrim_char_slice(bytes, b'0')
}}

// Trim leading digit separators (so, nothing).
// Does not consume any digit separators.
perftools_inline!{
pub(super) fn ltrim_separator_no_separator<'a>(bytes: &'a [u8])
    -> (&'a [u8], usize)
{
    (bytes, 0)
}}

// Trim leading 0s.
// Does not consume any digit separators.
perftools_inline!{
pub(super) fn rtrim_zero_no_separator<'a>(bytes: &'a [u8])
    -> (&'a [u8], usize)
{
    rtrim_char_slice(bytes, b'0')
}}

// Trim trailing digit separators (so, nothing).
// Does not consume any digit separators.
perftools_inline!{
pub(super) fn rtrim_separator_no_separator<'a>(bytes: &'a [u8])
    -> (&'a [u8], usize)
{
    (bytes, 0)
}}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_zero_no_separator_test() {
        assert_eq!(ltrim_zero_no_separator(b!("01")), (b!("1"), 1));
        assert_eq!(rtrim_zero_no_separator(b!("23450")), (b!("2345"), 1));
    }

    #[test]
    fn trim_separator_no_separator_test() {
        assert_eq!(ltrim_separator_no_separator(b!("01")), (b!("01"), 0));
        assert_eq!(rtrim_separator_no_separator(b!("23450")), (b!("23450"), 0));
    }
}
