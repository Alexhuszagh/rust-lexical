//! Validate buffers and other information.

use crate::error::*;
use crate::result::*;

use super::traits::*;

// HELPERS

/// Determine if the integer component is empty.
#[inline(always)]
fn is_integer_empty<'a, Data>(data: &Data)
    -> bool
    where Data: FastDataInterface<'a>
{
    data.integer_iter().next().is_none()
}

/// Determine if the fraction component is empty.
#[inline(always)]
fn is_fraction_empty<'a, Data>(data: &Data)
    -> bool
    where Data: FastDataInterface<'a>
{
    data.fraction_iter().next().is_none()
}

/// Determine if the fraction component exists.
#[inline(always)]
#[cfg(feature = "format")]
fn has_fraction<'a, Data>(data: &Data)
    -> bool
    where Data: FastDataInterface<'a>
{
    data.fraction().is_some()
}

/// Determine if the exponent component exists.
#[inline(always)]
fn has_exponent<'a, Data>(data: &Data)
    -> bool
    where Data: FastDataInterface<'a>
{
    data.exponent().is_some()
}

/// Unwrap option to get the pointer.
#[inline(always)]
fn option_as_ptr(option: Option<&[u8]>) -> *const u8
{
    option.unwrap().as_ptr()
}

/// Get the last significant digit character as a ptr.
/// Useful for indexing when we have invalid, missing exponent
/// characters.
#[inline(always)]
#[cfg(feature = "format")]
fn last_mantissa_as_ptr<'a, Data>(data: &Data)
    -> *const u8
    where Data: FastDataInterface<'a>
{
    if has_fraction(data) {
        let fraction = data.fraction().unwrap();
        fraction[fraction.len()..].as_ptr()
    } else {
        let integer = data.integer();
        integer[integer.len()..].as_ptr()
    }
}

// MANTISSA

/// Validate the extracted integer has no leading zeros.
#[inline]
#[cfg(feature = "format")]
pub(super) fn validate_no_leading_zeros<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    // Check if the next character is a sign symbol.
    let mut iter = data.integer_iter();
    match iter.next() {
        Some(&b'0')     => (),
        _               => return Ok(())
    };

    // Only here if we have a leading 0 symbol.
    match iter.next() {
        Some(_) => Err((ErrorCode::InvalidLeadingZeros, data.integer().as_ptr())),
        None    => Ok(())
    }
}

/// Validate the extracted mantissa float components.
///      1. Validate non-empty significant digits (integer or fraction).
#[inline(always)]
pub(super) fn validate_permissive_mantissa<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let integer_empty = is_integer_empty(data);
    let fraction_empty = is_fraction_empty(data);
    if integer_empty && fraction_empty {
        // Invalid floating-point number, no integer or fraction components.
        Err((ErrorCode::EmptyMantissa, data.integer().as_ptr()))
    } else {
        Ok(())
    }
}

/// Validate the extracted mantissa float components.
///      1. Validate integer component is non-empty.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_required_integer<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    if is_integer_empty(data) {
        // Invalid floating-point number, no integer component.
        Err((ErrorCode::EmptyInteger, data.integer().as_ptr()))
    } else {
        Ok(())
    }
}

/// Validate the extracted mantissa float components.
///      1. Validate fraction component is non-empty if present.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_required_fraction<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    if has_fraction(data) && is_fraction_empty(data) {
        // Invalid floating-point number, no fraction component.
        Err((ErrorCode::EmptyFraction, option_as_ptr(data.fraction())))
    } else {
        Ok(())
    }
}

/// Validate the extracted mantissa float components.
///      1. Validate integer component is non-empty.
///      2. Validate fraction component is non-empty if present.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_required_digits<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    if is_integer_empty(data) {
        // Invalid floating-point number, no integer component.
        Err((ErrorCode::EmptyInteger, data.integer().as_ptr()))
    } else if has_fraction(data) && is_fraction_empty(data) {
        // Invalid floating-point number, no fraction component.
        Err((ErrorCode::EmptyFraction, option_as_ptr(data.fraction())))
    } else {
        Ok(())
    }
}

/// Validate mantissa depending on float format.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_mantissa<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    // Check no leading zeros.
    if data.format().no_float_leading_zeros() {
        validate_no_leading_zeros(data)?;
    }

    // Check required digits.
    let required_integer = data.format().required_integer_digits();
    let required_fraction = data.format().required_fraction_digits();
    match (required_integer, required_fraction) {
        (true, true)    => validate_required_digits(data),
        (false, true)   => validate_required_fraction(data),
        (true, false)   => validate_required_integer(data),
        (false, false)  => validate_permissive_mantissa(data)
    }
}

// EXPONENT

/// Validate the required exponent component.
///      1). If the exponent has been defined, ensure at least 1 digit follows it.
#[inline]
pub(super) fn validate_required_exponent_digits<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    // If we don't have an exponent stored, we're fine.
    if !has_exponent(data) {
        return Ok(())
    }

    // Check if the next character is a sign symbol.
    let mut iter = data.exponent_iter();
    match iter.next() {
        Some(&b'+') | Some(&b'-')   => (),
        Some(_)                     => return Ok(()),
        None                        => return Err((ErrorCode::EmptyExponent, option_as_ptr(data.exponent())))
    };

    // Only here if we have a sign symbol.
    match iter.next() {
        Some(_) => Ok(()),
        None    => Err((ErrorCode::EmptyExponent, option_as_ptr(data.exponent())))
    }
}

/// Validate optional exponent component.
///      A no-op, since the data is optional.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_optional_exponent<'a, Data>(_: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    Ok(())
}

/// Validate required exponent component with required digits.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_required_exponent_and_digits<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    match has_exponent(data) {
        true  => validate_required_exponent_digits(data),
        false => return Err((ErrorCode::MissingExponent, last_mantissa_as_ptr(data)))
    }
}

/// Validate required exponent component without requiring any digits.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_required_exponent_without_digits<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    match has_exponent(data) {
        true  => Ok(()),
        false => return Err((ErrorCode::MissingExponent, last_mantissa_as_ptr(data)))
    }
}

/// Validate invalid exponent component.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_invalid_exponent<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    match has_exponent(data) {
        true  => return Err((ErrorCode::InvalidExponent, option_as_ptr(data.exponent()))),
        false => Ok(())
    }
}

/// Validate exponent depending on float format.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_exponent<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let required_exponent = data.format().required_exponent_notation();
    let required_digits = data.format().required_exponent_digits();
    let invalid = data.format().no_exponent_notation();
    match (required_exponent, required_digits, invalid) {
        (true, true, _)         => validate_required_exponent_and_digits(data),
        (true, false, _)        => validate_required_exponent_without_digits(data),
        (false, true, _)        => validate_required_exponent_digits(data),
        (false, _, true)        => validate_invalid_exponent(data),
        (false, false, false)   => validate_optional_exponent(data)
    }
}

// EXPONENT SIGN

/// Validate optional exponent sign.
///      A no-op, since the data is optional.
#[inline(always)]
pub(super) fn validate_optional_exponent_sign<'a, Data>(_: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    Ok(())
}

/// Validate a required exponent sign.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_required_exponent_sign<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    // Check if the next character is a sign symbol.
    let mut iter = data.exponent_iter();
    match iter.next() {
        Some(&b'+') | Some(&b'-')   => Ok(()),
        _ if has_exponent(data)     => Err((ErrorCode::MissingExponentSign, option_as_ptr(data.exponent()))),
        _                           => Ok(())
    }
}

/// Validate a required exponent sign.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_no_positive_exponent_sign<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    // Check if the next character is a sign symbol.
    let mut iter = data.exponent_iter();
    match iter.next() {
        Some(&b'+') => Err((ErrorCode::InvalidPositiveExponentSign, option_as_ptr(data.exponent()))),
        _           => Ok(())
    }
}

/// Validate exponent sign depending on float format.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_exponent_sign<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let required = data.format().required_exponent_sign();
    let no_positive = data.format().no_positive_exponent_sign();
    match (required, no_positive) {
        (true, _)       => validate_required_exponent_sign(data),
        (_, true)       => validate_no_positive_exponent_sign(data),
        (false, false)  => validate_optional_exponent_sign(data)
    }
}

// EXPONENT FRACTION

/// Validate an exponent may occur with or without a fraction.
#[inline(always)]
pub(super) fn validate_exponent_optional_fraction<'a, Data>(_: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    Ok(())
}

/// Validate an exponent requires a fraction component.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_exponent_required_fraction<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    match has_exponent(data) && !has_fraction(data) {
        true  => Err((ErrorCode::ExponentWithoutFraction, option_as_ptr(data.exponent()))),
        false => Ok(())
    }
}

/// Validate exponent fraction depending on float format.
#[inline(always)]
#[cfg(feature = "format")]
pub(super) fn validate_exponent_fraction<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    match data.format().no_exponent_without_fraction() {
        true  => validate_exponent_required_fraction(data),
        false => validate_exponent_optional_fraction(data)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::standard::*;

    #[test]
    #[cfg(feature = "format")]
    fn last_mantissa_as_ptr_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        // Create a very simple, empty buffer.
        let buffer = b!(".");
        let integer = &buffer[0..0];
        let fraction = &buffer[1..1];

        // Test with a fraction (and empty).
        let data: Data = (integer, Some(fraction), None, 0).into();
        assert_eq!(last_mantissa_as_ptr(&data), fraction.as_ptr());

        // Test without a fraction (and empty).
        let data: Data = (integer, None, None, 0).into();
        assert_eq!(last_mantissa_as_ptr(&data), integer.as_ptr());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_no_leading_zeros_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), Some(b!("23450")), None, 0).into();
        assert!(validate_no_leading_zeros(&data).is_err());

        let data: Data = (b!("1"), Some(b!("23450")), None, 0).into();
        assert!(validate_no_leading_zeros(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("23450")), None, 0).into();
        assert!(validate_no_leading_zeros(&data).is_ok());

        let data: Data = (b!(""), Some(b!("23450")), None, 0).into();
        assert!(validate_no_leading_zeros(&data).is_ok());
    }

    #[test]
    fn validate_permissive_mantissa_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), Some(b!("23450")), None, 0).into();
        assert!(validate_permissive_mantissa(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_permissive_mantissa(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_permissive_mantissa(&data).is_ok());

        let data: Data = (b!(""), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_permissive_mantissa(&data).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_integer_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), Some(b!("23450")), None, 0).into();
        assert!(validate_required_integer(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_integer(&data).is_ok());

        let data: Data = (b!(""), Some(b!("0")), Some(b!("")), 0).into();
        assert!(validate_required_integer(&data).is_err());

        let data: Data = (b!(""), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_integer(&data).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_fraction_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), Some(b!("23450")), None, 0).into();
        assert!(validate_required_fraction(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_fraction(&data).is_err());

        let data: Data = (b!(""), Some(b!("0")), Some(b!("")), 0).into();
        assert!(validate_required_fraction(&data).is_ok());

        let data: Data = (b!(""), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_fraction(&data).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_digits_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), Some(b!("23450")), None, 0).into();
        assert!(validate_required_digits(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_digits(&data).is_err());

        let data: Data = (b!(""), Some(b!("0")), Some(b!("")), 0).into();
        assert!(validate_required_digits(&data).is_err());

        let data: Data = (b!(""), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_digits(&data).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_exponent_and_digits_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), Some(b!("23450")), None, 0).into();
        assert!(validate_required_exponent_and_digits(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_exponent_and_digits(&data).is_err());

        let data: Data = (b!(""), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_required_exponent_and_digits(&data).is_err());

        let data: Data = (b!(""), Some(b!("")), Some(b!("2")), 0).into();
        assert!(validate_required_exponent_and_digits(&data).is_ok());

        let data: Data = (b!(""), Some(b!("")), Some(b!("+2")), 0).into();
        assert!(validate_required_exponent_and_digits(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_exponent_without_digits_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), Some(b!("23450")), None, 0).into();
        assert!(validate_required_exponent_without_digits(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_exponent_without_digits(&data).is_ok());

        let data: Data = (b!(""), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_required_exponent_without_digits(&data).is_ok());

        let data: Data = (b!(""), Some(b!("")), Some(b!("2")), 0).into();
        assert!(validate_required_exponent_without_digits(&data).is_ok());

        let data: Data = (b!(""), Some(b!("")), Some(b!("+2")), 0).into();
        assert!(validate_required_exponent_without_digits(&data).is_ok());
    }

    #[test]
    fn validate_required_exponent_digits_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), Some(b!("23450")), None, 0).into();
        assert!(validate_required_exponent_digits(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_exponent_digits(&data).is_err());

        let data: Data = (b!(""), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_required_exponent_digits(&data).is_err());

        let data: Data = (b!(""), Some(b!("")), Some(b!("2")), 0).into();
        assert!(validate_required_exponent_digits(&data).is_ok());

        let data: Data = (b!(""), Some(b!("")), Some(b!("+2")), 0).into();
        assert!(validate_required_exponent_digits(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_optional_exponent_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), Some(b!("")), None, 0).into();
        assert!(validate_optional_exponent(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("2")), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+2")), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_invalid_exponent_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), Some(b!("")), None, 0).into();
        assert!(validate_invalid_exponent(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_invalid_exponent(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_invalid_exponent(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("2")), 0).into();
        assert!(validate_invalid_exponent(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+2")), 0).into();
        assert!(validate_invalid_exponent(&data).is_err());
    }

    #[test]
    fn validate_optional_exponent_sign_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), Some(b!("")), None, 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("2")), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+2")), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_exponent_sign_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), Some(b!("")), None, 0).into();
        assert!(validate_required_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_required_exponent_sign(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_required_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("2")), 0).into();
        assert!(validate_required_exponent_sign(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+2")), 0).into();
        assert!(validate_required_exponent_sign(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_no_positive_exponent_sign_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), Some(b!("")), None, 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+")), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("2")), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("+2")), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_err());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("-2")), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_ok());
    }

    #[test]
    fn validate_exponent_optional_fraction_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), Some(b!("")), None, 0).into();
        assert!(validate_exponent_optional_fraction(&data).is_ok());

        let data: Data = (b!(""), Some(b!("0")), None, 0).into();
        assert!(validate_exponent_optional_fraction(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_exponent_optional_fraction(&data).is_ok());

        let data: Data = (b!(""), Some(b!("0")), Some(b!("+")), 0).into();
        assert!(validate_exponent_optional_fraction(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_exponent_required_fraction_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), Some(b!("")), None, 0).into();
        assert!(validate_exponent_required_fraction(&data).is_ok());

        let data: Data = (b!("0"), Some(b!("")), Some(b!("")), 0).into();
        assert!(validate_exponent_required_fraction(&data).is_ok());

        let data: Data = (b!("0"), None, Some(b!("")), 0).into();
        assert!(validate_exponent_required_fraction(&data).is_err());

        let data: Data = (b!(""), Some(b!("0")), Some(b!("+")), 0).into();
        assert!(validate_exponent_required_fraction(&data).is_ok());
    }
}
