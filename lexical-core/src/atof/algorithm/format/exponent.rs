//! Utilities to parse, extract, and interpret exponent components.

use crate::atoi;
use crate::lib::slice;
use crate::util::*;
use super::iterator::*;
use super::traits::*;

// Calculate the scientific notation exponent without overflow.
//
// For example, 0.1 would be -1, and 10 would be 1 in base 10.
perftools_inline!{
#[cfg(feature = "correct")]
pub(super) fn scientific_exponent(exponent: i32, integer_digits: usize, fraction_start: usize)
    -> i32
{
    if integer_digits == 0 {
        let fraction_start = fraction_start.try_i32_or_max();
        exponent.saturating_sub(fraction_start).saturating_sub(1)
    } else {
        let integer_shift = (integer_digits - 1).try_i32_or_max();
        exponent.saturating_add(integer_shift)
    }
}}

// Calculate the mantissa exponent without overflow.
//
// Remove the number of digits that contributed to the mantissa past
// the dot, and add the number of truncated digits from the mantissa,
// to calculate the scaling factor for the mantissa from a raw exponent.
perftools_inline!{
#[cfg(feature = "correct")]
pub(super) fn mantissa_exponent(raw_exponent: i32, fraction_digits: usize, truncated: usize)
    -> i32
{
    if fraction_digits > truncated {
        raw_exponent.saturating_sub((fraction_digits - truncated).try_i32_or_max())
    } else {
        raw_exponent.saturating_add((truncated - fraction_digits).try_i32_or_max())
    }
}}

// Extract exponent substring and parse exponent.
// Does not ignore any digit separators.
// Exponent is required (cannot be empty).
perftools_inline!{
#[allow(unused_unsafe)]
pub(super) fn extract_exponent_no_separator<'a, Data>(
    data: &mut Data,
    bytes: &'a [u8],
    radix: u32
)
    -> &'a [u8]
    where Data: FastDataInterface<'a>
{
    // Remove leading exponent character and parse exponent.
    let iter = iterate_no_separator(&index!(bytes[1..]));
    let (raw_exponent, ptr) = atoi::standalone_exponent(iter, radix);
    data.set_raw_exponent(raw_exponent);

    unsafe {
        // Extract the exponent subslice.
        let first = bytes.as_ptr();
        data.set_exponent(slice::from_raw_parts(first, distance(first, ptr)));

        // Return the remaining bytes.
        let last = index!(bytes[bytes.len()..]).as_ptr();
        slice::from_raw_parts(ptr, distance(ptr, last))
    }
}}

// TESTS
// -----

#[cfg(test)]
mod test {
    use super::*;
    use super::super::standard::*;

    #[cfg(feature = "correct")]
    #[test]
    fn scientific_exponent_test() {
        // 0 digits in the integer
        assert_eq!(scientific_exponent(0, 0, 5), -6);
        assert_eq!(scientific_exponent(10, 0, 5), 4);
        assert_eq!(scientific_exponent(-10, 0, 5), -16);

        // >0 digits in the integer
        assert_eq!(scientific_exponent(0, 1, 5), 0);
        assert_eq!(scientific_exponent(0, 2, 5), 1);
        assert_eq!(scientific_exponent(0, 2, 20), 1);
        assert_eq!(scientific_exponent(10, 2, 20), 11);
        assert_eq!(scientific_exponent(-10, 2, 20), -9);

        // Underflow
        assert_eq!(scientific_exponent(i32::min_value(), 0, 0), i32::min_value());
        assert_eq!(scientific_exponent(i32::min_value(), 0, 5), i32::min_value());

        // Overflow
        assert_eq!(scientific_exponent(i32::max_value(), 0, 0), i32::max_value()-1);
        assert_eq!(scientific_exponent(i32::max_value(), 5, 0), i32::max_value());
    }

    #[cfg(feature = "correct")]
    #[test]
    fn mantissa_exponent_test() {
        assert_eq!(mantissa_exponent(10, 5, 0), 5);
        assert_eq!(mantissa_exponent(0, 5, 0), -5);
        assert_eq!(mantissa_exponent(i32::max_value(), 5, 0), i32::max_value()-5);
        assert_eq!(mantissa_exponent(i32::max_value(), 0, 5), i32::max_value());
        assert_eq!(mantissa_exponent(i32::min_value(), 5, 0), i32::min_value());
        assert_eq!(mantissa_exponent(i32::min_value(), 0, 5), i32::min_value()+5);
    }

    #[test]
    fn extract_exponent_no_separator_test() {
        // Allows required exponents.
        type Data<'a> = StandardFastDataInterface<'a>;
        let mut data = Data::new(FloatFormat::RUST_STRING);
        extract_exponent_no_separator(&mut data, b"e+23", 10);
        assert_eq!(data.exponent(), b"e+23");
        assert_eq!(data.raw_exponent(), 23);

        // Allows optional exponents.
        let mut data = Data::new(FloatFormat::RUST_STRING);
        extract_exponent_no_separator(&mut data, b"e", 10);
        assert_eq!(data.exponent(), b"e");
        assert_eq!(data.raw_exponent(), 0);
    }
}
