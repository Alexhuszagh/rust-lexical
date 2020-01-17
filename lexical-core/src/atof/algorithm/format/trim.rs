//! Trim leading and trailing 0s and digit separators.

use crate::atof::algorithm::state::FloatState1;
use crate::util::*;

// Left-trim leading 0s.
// Does not consume any digit separators.
perftools_inline!{
fn ltrim_no_separator<'a>(bytes: &'a [u8], _: u8)
    -> &'a [u8]
{
    ltrim_char_slice(bytes, b'0').0
}}

// Right-trim leading 0s.
// Does not consume any digit separators.
perftools_inline!{
fn rtrim_no_separator<'a>(bytes: &'a [u8], _: u8)
    -> &'a [u8]
{
    rtrim_char_slice(bytes, b'0').0
}}

// Trim leading and trailing 0s.
// Does not consume any digit separators.
perftools_inline!{
pub(super) fn trim_no_separator(state: &mut FloatState1, digit_separator: u8)
{
    state.integer = ltrim_no_separator(state.integer, digit_separator);
    state.fraction = rtrim_no_separator(state.fraction, digit_separator);
}}

// TODO(ahuszagh) Add format-dependent features here....

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_no_separator_test() {
        let mut state = (b!("01"), b!("23450"), b!(""), 0).into();
        trim_no_separator(&mut state, b'\x00');
        assert_eq!(state.integer, b!("1"));
        assert_eq!(state.fraction, b!("2345"));
    }
}
