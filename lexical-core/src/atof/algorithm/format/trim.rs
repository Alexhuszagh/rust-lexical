//! Trim leading and trailing 0s and digit separators.

use crate::util::*;

// Left-trim leading 0s.
// Does not consume any digit separators.
perftools_inline!{
pub(super) fn ltrim_no_separator<'a>(bytes: &'a [u8], _: u8)
    -> (&'a [u8], usize)
{
    ltrim_char_slice(bytes, b'0')
}}

// Right-trim leading 0s.
// Does not consume any digit separators.
perftools_inline!{
pub(super) fn rtrim_no_separator<'a>(bytes: &'a [u8], _: u8)
    -> (&'a [u8], usize)
{
    rtrim_char_slice(bytes, b'0')
}}

// TODO(ahuszagh) Add format-dependent features here....

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ltrim_no_separator_test() {
        assert_eq!(ltrim_no_separator(b!("01"), b'\x00'), (b!("1"), 1));
        assert_eq!(rtrim_no_separator(b!("23450"), b'\x00'), (b!("2345"), 1));
    }
}
