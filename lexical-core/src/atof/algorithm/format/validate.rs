//! Validate buffers and other information.

use crate::atof::algorithm::state::FloatState1;
use crate::util::*;
use super::result::*;

// Checks if the byte slice is empty.
// Does not ignore any digit separators.
perftools_inline!{
fn is_empty_no_separator(bytes: &[u8], _: u8)
    -> bool
{
    bytes.len().is_zero()
}}

// MANTISSA

// Validate the extracted float components.
//      1. Validate all integer characters are digits.
//      2. Validate all fraction characters are digits.
//      3. Validate non-empty significant digits (integer or fraction).
perftools_inline!{
pub(super) fn validate_required_digits_no_separator(
    state: &FloatState1,
    character_separator: u8
)
    -> ParseResult<()>
{
    // Do a simple verification of the parsed data.
    let integer_empty = is_empty_no_separator(state.integer, character_separator);
    let fraction_empty = is_empty_no_separator(state.fraction, character_separator);
    if integer_empty && fraction_empty {
        // Invalid floating-point number, no integer or fraction components.
        Err((ErrorCode::EmptyFraction, state.integer.as_ptr()))
    } else {
        Ok(())
    }
}}

// EXPONENT

// Validate the required exponent components.
//      No-op, since it's previously validated.
perftools_inline!{
pub(super) fn validate_required_exponent_no_separator(
    _: &FloatState1,
    _: u8
)
    -> ParseResult<()>
{
    Ok(())
}}

// TODO(ahuszagh) Add format-dependent features here....

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_no_separator_test() {
        let state = (b!("01"), b!("23450"), b!(""), 0).into();
        assert!(validate_required_digits_no_separator(&state, b'\x00').is_ok());
        assert!(validate_required_exponent_no_separator(&state, b'\x00').is_ok());

        let state = (b!("0"), b!(""), b!(""), 0).into();
        assert!(validate_required_digits_no_separator(&state, b'\x00').is_ok());
        assert!(validate_required_exponent_no_separator(&state, b'\x00').is_ok());

        let state = (b!(""), b!(""), b!(""), 0).into();
        assert!(validate_required_digits_no_separator(&state, b'\x00').is_err());
        assert!(validate_required_exponent_no_separator(&state, b'\x00').is_ok());
    }
}
