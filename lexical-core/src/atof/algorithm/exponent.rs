//! Algorithm to parse an exponent from a float string.

use util::*;

/// Calculate the scientific notation exponent without overflow.
///
/// For example, 0.1 would be -1, and 10 would be 1 in base 10.
perftools_inline!{
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

/// Calculate the mantissa exponent without overflow.
///
/// Remove the number of digits that contributed to the mantissa past
/// the dot, and add the number of truncated digits from the mantissa,
/// to calculate the scaling factor for the mantissa from a raw exponent.
perftools_inline!{
pub(super) fn mantissa_exponent(raw_exponent: i32, fraction_digits: usize, truncated: usize)
    -> i32
{
    if fraction_digits > truncated {
        raw_exponent.saturating_sub((fraction_digits - truncated).try_i32_or_max())
    } else {
        raw_exponent.saturating_add((truncated - fraction_digits).try_i32_or_max())
    }
}}

/// Calculate the integral ceiling of the binary factor from a basen number.
perftools_inline!{
pub(super) fn integral_binary_factor(radix: u32)
    -> u32
{
    debug_assert_radix!(radix);

    #[cfg(not(feature = "radix"))] {
        4
    }

    #[cfg(feature = "radix")] {
        match radix.as_i32() {
            2  => 1,
            3  => 2,
            4  => 2,
            5  => 3,
            6  => 3,
            7  => 3,
            8  => 3,
            9  => 4,
            10 => 4,
            11 => 4,
            12 => 4,
            13 => 4,
            14 => 4,
            15 => 4,
            16 => 4,
            17 => 5,
            18 => 5,
            19 => 5,
            20 => 5,
            21 => 5,
            22 => 5,
            23 => 5,
            24 => 5,
            25 => 5,
            26 => 5,
            27 => 5,
            28 => 5,
            29 => 5,
            30 => 5,
            31 => 5,
            32 => 5,
            33 => 6,
            34 => 6,
            35 => 6,
            36 => 6,
            // Invalid radix
            _  => unreachable!(),
        }
    }
}}

// TESTS
// -----

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn mantissa_exponent_test() {
        assert_eq!(mantissa_exponent(10, 5, 0), 5);
        assert_eq!(mantissa_exponent(0, 5, 0), -5);
        assert_eq!(mantissa_exponent(i32::max_value(), 5, 0), i32::max_value()-5);
        assert_eq!(mantissa_exponent(i32::max_value(), 0, 5), i32::max_value());
        assert_eq!(mantissa_exponent(i32::min_value(), 5, 0), i32::min_value());
        assert_eq!(mantissa_exponent(i32::min_value(), 0, 5), i32::min_value()+5);
    }

    #[cfg(all(feature = "correct", feature = "radix"))]
    #[test]
    fn integral_binary_factor_test() {
        const TABLE: [u32; 35] = [1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6];
        for (idx, base) in (2..37).enumerate() {
            assert_eq!(integral_binary_factor(base), TABLE[idx]);
        }
    }
}
