//! Specialized float parsers for different formats.

use crate::atoi;
use crate::util::*;
use crate::lib::slice;
use crate::lib::result::Result as StdResult;
use crate::atof::algorithm::state::RawFloatState;
use super::shared::*;

pub(in crate::atof::algorithm) struct Standard;

impl FormatParser for Standard {
    // Consume until a non-digit character is found.
    perftools_inline!{
    fn consume_digits<'a>(digits: &'a [u8], radix: u32, _: u8)
        -> (&'a [u8], &'a [u8])
    {
        match digits.iter().position(|&c| !is_digit(c, radix)) {
            Some(v) => (&digits[..v], &digits[v..]),
            None    => (&digits[..], &digits[digits.len()..]),
        }
    }}

    // Extract and parse the exponent substring from the float.
    //
    //  This validates that the exponent is non-empty.
    perftools_inline!{
    fn parse_exponent<'a>(state: &mut RawFloatState<'a>, bytes: &'a [u8], radix: u32, _: u8)
        -> StdResult<&'a [u8] , (ErrorCode, *const u8)>
    {
        // TODO(ahuszagh)
        //  This throws an error with an invalid result, without the
        //  exponent required digits, just always return a value.
        //  Sacrifices performance in the poor case for good performance
        //  in the common case.
        let digits = &index!(bytes[1..]);
        let (exp, ptr) = atoi::standalone_exponent(digits, radix)?;
        let first = digits.as_ptr();
        let last = index!(digits[digits.len()..]).as_ptr();
        state.exponent = unsafe { slice::from_raw_parts(first, distance(first, last)) };
        state.raw_exponent = exp;
        Ok(unsafe { slice::from_raw_parts(ptr, distance(ptr, last)) })
    }}

    // Validate the extracted float components.
    //      1. Validate all integer characters are digits.
    //      2. Validate all fraction characters are digits.
    //      3. Validate non-empty significant digits (integer or fraction).
    perftools_inline!{
    fn validate(state: &RawFloatState, bytes: &[u8], _: u8)
        -> StdResult<(), (ErrorCode, *const u8)>
    {
        // Do a simple verification of the parsed data.
        if state.integer.len().is_zero() && state.fraction.len().is_zero() {
            // Invalid floating-point number, no integer or fraction components.
            Err((ErrorCode::EmptyFraction, bytes.as_ptr()))
        } else {
            Ok(())
        }
    }}

    // Post-process float to trim leading and trailing 0s and digit separators.
    perftools_inline!{
    fn trim(state: &mut RawFloatState, _: u8) {
        state.integer = ltrim_0!(state.integer).0;
        state.fraction = rtrim_0!(state.fraction).0;
    }}
}
