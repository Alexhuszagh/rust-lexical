//! Validate buffers and other information.

use crate::util::*;
use super::result::*;
use super::traits::*;

// Checks if the byte slice is empty.
// Does not ignore any digit separators.
perftools_inline!{
fn is_empty_no_separator(bytes: &[u8])
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
pub(super) fn validate_mantissa_no_separator<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    // Do a simple verification of the parsed data.
    let integer_empty = is_empty_no_separator(data.integer());
    let fraction_empty = is_empty_no_separator(data.fraction());
    if integer_empty && fraction_empty {
        // Invalid floating-point number, no integer or fraction components.
        Err((ErrorCode::EmptyFraction, data.integer().as_ptr()))
    } else {
        Ok(())
    }
}}

// EXPONENT

// Validate the required exponent components.
//      No-op, since it's previously validated.
perftools_inline!{
pub(super) fn validate_required_exponent_no_separator<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let exponent = data.exponent();
    let length = exponent.len();
    match length {
        // No exponent character found.
        0 => Ok(()),
        // Only exponent sign, invalid.
        1 => Err((ErrorCode::EmptyExponent, exponent.as_ptr())),
        // Need to check we don't have a solitary sign bit.
        2 => {
            match index!(exponent[1]) {
                b'+' | b'-' => Err((ErrorCode::EmptyExponent, exponent.as_ptr())),
                _           => Ok(())
            }
        },
        _ => Ok(())
    }
}}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::standard::*;

    #[test]
    fn validate_no_separator_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), b!("23450"), b!(""), 0).into();
        assert!(validate_mantissa_no_separator(&data).is_ok());
        assert!(validate_required_exponent_no_separator(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_mantissa_no_separator(&data).is_ok());
        assert!(validate_required_exponent_no_separator(&data).is_err());

        let data: Data = (b!(""), b!(""), b!("e+"), 0).into();
        assert!(validate_mantissa_no_separator(&data).is_err());
        assert!(validate_required_exponent_no_separator(&data).is_err());

        let data: Data = (b!(""), b!(""), b!("e2"), 0).into();
        assert!(validate_required_exponent_no_separator(&data).is_ok());

        let data: Data = (b!(""), b!(""), b!("e+2"), 0).into();
        assert!(validate_required_exponent_no_separator(&data).is_ok());
    }
}
