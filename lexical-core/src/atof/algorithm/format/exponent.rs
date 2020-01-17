//! Utilities to parse and extract exponent components.

use crate::atoi;
use crate::atof::algorithm::state::FloatState1;
use crate::lib::slice;
use crate::util::*;
use super::result::*;

// Extract exponent substring and parse exponent.
// Does not ignore any digit separators.
// Exponent is required (cannot be empty).
perftools_inline!{
pub(super) fn parse_required_no_separator<'a>(
    state: &mut FloatState1<'a>,
    bytes: &'a [u8],
    radix: u32,
    _: u8
)
    -> ParseResult<&'a [u8]>
{
    // Remove leading exponent character and parse exponent.
    let digits = &index!(bytes[1..]);
    let (raw_exponent, ptr) = atoi::standalone_exponent(digits, radix)?;
    state.raw_exponent = raw_exponent;

    unsafe {
        // Extract the exponent subslice.
        let first = digits.as_ptr();
        let last = index!(digits[digits.len()..]).as_ptr();
        state.exponent = slice::from_raw_parts(first, distance(first, last));

        // Return the remaining bytes.
        Ok(slice::from_raw_parts(ptr, distance(ptr, last)))
    }
}}

// TODO(ahuszagh) Add format-dependent features here....

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_no_separator_test() {
        let mut state = FloatState1::new();
        parse_required_no_separator(&mut state, b!("e345"), 10, b'\x00').unwrap();
        assert_eq!(state.exponent, b!("345"));
        assert_eq!(state.raw_exponent, 345);
    }
}

