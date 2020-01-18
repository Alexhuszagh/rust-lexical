//! Algorithm to parse an exponent from a float string.

use util::*;

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

#[cfg(all(test, feature = "correct", feature = "radix"))]
mod test {
    use super::*;

    #[test]
    fn integral_binary_factor_test() {
        const TABLE: [u32; 35] = [1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6];
        for (idx, base) in (2..37).enumerate() {
            assert_eq!(integral_binary_factor(base), TABLE[idx]);
        }
    }
}
