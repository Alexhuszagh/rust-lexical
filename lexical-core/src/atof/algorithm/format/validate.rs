//! Validate buffers and other information.

use crate::util::*;
use super::traits::*;

// MANTISSA

// Validate the extracted mantissa float components.
//      1. Validate non-empty significant digits (integer or fraction).
perftools_inline!{
pub(super) fn validate_permissive_mantissa<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let integer_empty = data.integer_iter().next().is_none();
    let fraction_empty = data.fraction_iter().next().is_none();
    if integer_empty && fraction_empty {
        // Invalid floating-point number, no integer or fraction components.
        Err((ErrorCode::EmptyMantissa, data.integer().as_ptr()))
    } else {
        Ok(())
    }
}}

// Validate the extracted mantissa float components.
//      1. Validate integer component is non-empty.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_required_integer<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    if data.integer_iter().next().is_none() {
        // Invalid floating-point number, no integer component.
        Err((ErrorCode::EmptyInteger, data.integer().as_ptr()))
    } else {
        Ok(())
    }
}}

// Validate the extracted mantissa float components.
//      1. Validate fraction component is non-empty.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_required_fraction<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    if data.fraction_iter().next().is_none() {
        // Invalid floating-point number, no fraction component.
        // Get a ptr to the past-end of integer, since fraction is not guaranteed
        // to be from the same array (if no decimal point was found).
        let integer = data.integer();
        Err((ErrorCode::EmptyFraction, index!(integer[integer.len()..]).as_ptr()))
    } else {
        Ok(())
    }
}}

// Validate the extracted mantissa float components.
//      1. Validate integer component is non-empty.
//      2. Validate fraction component is non-empty.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_required_digits<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let integer = data.integer();
    if data.integer_iter().next().is_none() {
        // Invalid floating-point number, no integer component.
        Err((ErrorCode::EmptyInteger, integer.as_ptr()))
    } else if data.fraction_iter().next().is_none() {
        // Invalid floating-point number, no fraction component.
        // Get a ptr to the past-end of integer, since fraction is not guaranteed
        // to be from the same array (if no decimal point was found).
        Err((ErrorCode::EmptyFraction, index!(integer[integer.len()..]).as_ptr()))
    } else {
        Ok(())
    }
}}

// Validate mantissa depending on float format.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_mantissa<'a, Data>(data: &Data, format: NumberFormat)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let required_integer = format.required_integer_digits();
    let required_fraction = format.required_fraction_digits();
    match (required_integer, required_fraction) {
        (true, true)    => validate_required_digits(data),
        (false, true)   => validate_required_fraction(data),
        (true, false)   => validate_required_integer(data),
        (false, false)  => validate_permissive_mantissa(data)
    }
}}

// EXPONENT

// Validate the required exponent component.
//      1). If the exponent has been defined, ensure at least 1 digit follows it.
perftools_inline!{
pub(super) fn validate_required_exponent<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let mut iter = data.exponent_iter();

    // The first character is always the exponent symbol,
    // check if we have an exponent.
    if !iter.next().is_some() {
        return Ok(())
    }

    // Check if the next character is a sign symbol.
    match iter.next() {
        Some(&b'+') | Some(&b'-')   => (),
        Some(_)                     => return Ok(()),
        None                        => return Err((ErrorCode::EmptyExponent, data.exponent().as_ptr()))
    };

    // Only here if we have a sign symbol.
    match iter.next() {
        Some(_) => Ok(()),
        None    => Err((ErrorCode::EmptyExponent, data.exponent().as_ptr()))
    }
}}

// Validate optional exponent component.
//      A no-op, since the data is optional.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_optional_exponent<'a, Data>(_: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    Ok(())
}}

// Validate invalid exponent component.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_invalid_exponent<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    match data.exponent_iter().next().is_some() {
        true  => return Err((ErrorCode::InvalidExponent, data.exponent().as_ptr())),
        false => Ok(())
    }
}}

// Validate exponent depending on float format.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_exponent<'a, Data>(data: &Data, format: NumberFormat)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let required = format.required_exponent_digits();
    let invalid = format.no_exponent_notation();
    match (required, invalid) {
        (true, _)       => validate_required_exponent(data),
        (_, true)       => validate_invalid_exponent(data),
        (false, false)  => validate_optional_exponent(data)
    }
}}

// EXPONENT SIGN

// Validate optional exponent sign.
//      A no-op, since the data is optional.
perftools_inline!{
pub(super) fn validate_optional_exponent_sign<'a, Data>(_: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    Ok(())
}}

// Validate a required exponent sign.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_required_exponent_sign<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let mut iter = data.exponent_iter();

    // The first character is always the exponent symbol,
    // check if we have an exponent.
    if !iter.next().is_some() {
        return Ok(())
    }

    // Check if the next character is a sign symbol.
    match iter.next() {
        Some(&b'+') | Some(&b'-')   => Ok(()),
        _                           => Err((ErrorCode::MissingExponentSign, data.exponent().as_ptr()))
    }
}}

// Validate a required exponent sign.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_no_positive_exponent_sign<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let mut iter = data.exponent_iter();

    // The first character is always the exponent symbol,
    // check if we have an exponent.
    if !iter.next().is_some() {
        return Ok(())
    }

    // Check if the next character is a sign symbol.
    match iter.next() {
        Some(&b'+') => Err((ErrorCode::InvalidPositiveExponentSign, data.exponent().as_ptr())),
        _           => Ok(())
    }
}}

// Validate exponent sign depending on float format.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_exponent_sign<'a, Data>(data: &Data, format: NumberFormat)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    let required = format.required_exponent_sign();
    let no_positive = format.no_positive_exponent_sign();
    match (required, no_positive) {
        (true, _)       => validate_required_exponent_sign(data),
        (_, true)       => validate_no_positive_exponent_sign(data),
        (false, false)  => validate_optional_exponent_sign(data)
    }
}}

// EXPONENT FRACTION

// Validate an exponent may occur with or without a fraction.
perftools_inline!{
pub(super) fn validate_exponent_optional_fraction<'a, Data>(_: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    Ok(())
}}

// Validate an exponent requires a fraction component.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_exponent_required_fraction<'a, Data>(data: &Data)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    match !data.exponent().is_empty() && data.fraction().is_empty() {
        true  => Err((ErrorCode::ExponentWithoutFraction, data.exponent().as_ptr())),
        false => Ok(())
    }
}}

// Validate exponent fraction depending on float format.
perftools_inline!{
#[cfg(feature = "format")]
pub(super) fn validate_exponent_fraction<'a, Data>(data: &Data, format: NumberFormat)
    -> ParseResult<()>
    where Data: FastDataInterface<'a>
{
    match format.no_exponent_without_fraction() {
        true  => validate_exponent_required_fraction(data),
        false => validate_exponent_optional_fraction(data)
    }
}}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::standard::*;

    #[test]
    fn validate_permissive_mantissa_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), b!("23450"), b!(""), 0).into();
        assert!(validate_permissive_mantissa(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_permissive_mantissa(&data).is_ok());

        let data: Data = (b!(""), b!(""), b!("e+"), 0).into();
        assert!(validate_permissive_mantissa(&data).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_integer_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), b!("23450"), b!(""), 0).into();
        assert!(validate_required_integer(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_required_integer(&data).is_ok());

        let data: Data = (b!(""), b!("0"), b!("e"), 0).into();
        assert!(validate_required_integer(&data).is_err());

        let data: Data = (b!(""), b!(""), b!("e"), 0).into();
        assert!(validate_required_integer(&data).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_fraction_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), b!("23450"), b!(""), 0).into();
        assert!(validate_required_fraction(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_required_fraction(&data).is_err());

        let data: Data = (b!(""), b!("0"), b!("e"), 0).into();
        assert!(validate_required_fraction(&data).is_ok());

        let data: Data = (b!(""), b!(""), b!("e"), 0).into();
        assert!(validate_required_fraction(&data).is_err());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_digits_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), b!("23450"), b!(""), 0).into();
        assert!(validate_required_digits(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_required_digits(&data).is_err());

        let data: Data = (b!(""), b!("0"), b!("e"), 0).into();
        assert!(validate_required_digits(&data).is_err());

        let data: Data = (b!(""), b!(""), b!("e"), 0).into();
        assert!(validate_required_digits(&data).is_err());
    }

    #[test]
    fn validate_required_exponent_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("01"), b!("23450"), b!(""), 0).into();
        assert!(validate_required_exponent(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_required_exponent(&data).is_err());

        let data: Data = (b!(""), b!(""), b!("e+"), 0).into();
        assert!(validate_required_exponent(&data).is_err());

        let data: Data = (b!(""), b!(""), b!("e2"), 0).into();
        assert!(validate_required_exponent(&data).is_ok());

        let data: Data = (b!(""), b!(""), b!("e+2"), 0).into();
        assert!(validate_required_exponent(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_optional_exponent_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), b!(""), b!(""), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e+"), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e2"), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e+2"), 0).into();
        assert!(validate_optional_exponent(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_invalid_exponent_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), b!(""), b!(""), 0).into();
        assert!(validate_invalid_exponent(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_invalid_exponent(&data).is_err());

        let data: Data = (b!("0"), b!(""), b!("e+"), 0).into();
        assert!(validate_invalid_exponent(&data).is_err());

        let data: Data = (b!("0"), b!(""), b!("e2"), 0).into();
        assert!(validate_invalid_exponent(&data).is_err());

        let data: Data = (b!("0"), b!(""), b!("e+2"), 0).into();
        assert!(validate_invalid_exponent(&data).is_err());
    }

    #[test]
    fn validate_optional_exponent_sign_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), b!(""), b!(""), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e+"), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e2"), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e+2"), 0).into();
        assert!(validate_optional_exponent_sign(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_required_exponent_sign_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), b!(""), b!(""), 0).into();
        assert!(validate_required_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_required_exponent_sign(&data).is_err());

        let data: Data = (b!("0"), b!(""), b!("e+"), 0).into();
        assert!(validate_required_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e2"), 0).into();
        assert!(validate_required_exponent_sign(&data).is_err());

        let data: Data = (b!("0"), b!(""), b!("e+2"), 0).into();
        assert!(validate_required_exponent_sign(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_no_positive_exponent_sign_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), b!(""), b!(""), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e+"), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_err());

        let data: Data = (b!("0"), b!(""), b!("e2"), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e+2"), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_err());

        let data: Data = (b!("0"), b!(""), b!("e-2"), 0).into();
        assert!(validate_no_positive_exponent_sign(&data).is_ok());
    }

    #[test]
    fn validate_exponent_optional_fraction_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), b!(""), b!(""), 0).into();
        assert!(validate_exponent_optional_fraction(&data).is_ok());

        let data: Data = (b!(""), b!("0"), b!(""), 0).into();
        assert!(validate_exponent_optional_fraction(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_exponent_optional_fraction(&data).is_ok());

        let data: Data = (b!(""), b!("0"), b!("e+"), 0).into();
        assert!(validate_exponent_optional_fraction(&data).is_ok());
    }

    #[test]
    #[cfg(feature = "format")]
    fn validate_exponent_required_fraction_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        let data: Data = (b!("0"), b!(""), b!(""), 0).into();
        assert!(validate_exponent_required_fraction(&data).is_ok());

        let data: Data = (b!(""), b!("0"), b!(""), 0).into();
        assert!(validate_exponent_required_fraction(&data).is_ok());

        let data: Data = (b!("0"), b!(""), b!("e"), 0).into();
        assert!(validate_exponent_required_fraction(&data).is_err());

        let data: Data = (b!(""), b!("0"), b!("e+"), 0).into();
        assert!(validate_exponent_required_fraction(&data).is_ok());
    }
}
