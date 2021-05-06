//! Cached exponents for basen values with 80-bit extended floats.
//!
//! Exact versions of base**n as an extended-precision float, with both
//! large and small powers. Use the large powers to minimize the amount
//! of compounded error.
//!
//! These values were calculated using Python, using the arbitrary-precision
//! integer to calculate exact extended-representation of each value.
//! These values are all normalized.
//!
//! These files takes ~30 KB of storage.
//!
//! Total array storage:
//!  Without radix: ~1 KB:
//!     86 u64
//!     78 i32
//!  With radix: ~29 KB:
//!     2534 u64
//!     2300 i32
//!
//! This file is mostly automatically generated, do not change values
//! manually, unless you know what you are doing. The script to generate
//! the values is as follows:
//!
//! ```text
//! import math
//! from collections import deque
//!
//! STEP_STR = "const BASE{0}_STEP: i32 = {1};"
//! SMALL_MANTISSA_STR = "const BASE{0}_SMALL_MANTISSA: [u64; {1}] = ["
//! SMALL_EXPONENT_STR = "const BASE{0}_SMALL_EXPONENT: [i32; {1}] = ["
//! LARGE_MANTISSA_STR = "const BASE{0}_LARGE_MANTISSA: [u64; {1}] = ["
//! LARGE_EXPONENT_STR = "const BASE{0}_LARGE_EXPONENT: [i32; {1}] = ["
//! SMALL_INT_STR = "const BASE{0}_SMALL_INT_POWERS: [u64; {1}] = {2};"
//! BIAS_STR = "const BASE{0}_BIAS: i32 = {1};"
//! EXP_STR = "// {}^{}"
//! POWER_STR = """pub(crate) const BASE{0}_POWERS: ModeratePathPowers<u64> = ModeratePathPowers {{
//!     small: ExtendedFloatArray {{ mant: &BASE{0}_SMALL_MANTISSA, exp: &BASE{0}_SMALL_EXPONENT }},
//!     large: ExtendedFloatArray {{ mant: &BASE{0}_LARGE_MANTISSA, exp: &BASE{0}_LARGE_EXPONENT }},
//!     small_int: &BASE{0}_SMALL_INT_POWERS,
//!     step: BASE{0}_STEP,
//!     bias: BASE{0}_BIAS,
//! }};\n"""
//!
//! def calculate_bitshift(base, exponent):
//!     '''
//!     Calculate the bitshift required for a given base. The exponent
//!     is the absolute value of the max exponent (log distance from 1.)
//!     '''
//!
//!     return 63 + math.ceil(math.log2(base**exponent))
//!
//!
//! def next_fp(fp, base, step = 1):
//!     '''Generate the next extended-floating point value.'''
//!
//!     return (fp[0] * (base**step), fp[1])
//!
//!
//! def prev_fp(fp, base, step = 1):
//!     '''Generate the previous extended-floating point value.'''
//!
//!     return (fp[0] // (base**step), fp[1])
//!
//!
//! def normalize_fp(fp):
//!     '''Normalize a extended-float so the MSB is the 64th bit'''
//!
//!     while fp[0] >> 64 != 0:
//!         fp = (fp[0] >> 1, fp[1] + 1)
//!     return fp
//!
//!
//! def generate_small(base, count):
//!     '''Generate the small powers for a given base'''
//!
//!     bitshift = calculate_bitshift(base, count)
//!     fps = []
//!     fp = (1 << bitshift, -bitshift)
//!     for exp in range(count):
//!         fps.append((normalize_fp(fp), exp))
//!         fp = next_fp(fp, base)
//!
//!     # Print the small powers as integers.
//!     ints = [base**i for _, i in fps]
//!
//!     return fps, ints
//!
//!
//! def generate_large(base, step):
//!     '''Generate the large powers for a given base.'''
//!
//!     # Get our starting parameters
//!     min_exp = math.floor(math.log(5e-324, base) - math.log(0xFFFFFFFFFFFFFFFF, base))
//!     max_exp = math.ceil(math.log(1.7976931348623157e+308, base))
//!     bitshift = calculate_bitshift(base, abs(min_exp - step))
//!     fps = deque()
//!
//!     # Add negative exponents
//!     # We need to go below the minimum exponent, since we need
//!     # all resulting exponents to be positive.
//!     fp = (1 << bitshift, -bitshift)
//!     for exp in range(-step, min_exp-step, -step):
//!         fp = prev_fp(fp, base, step)
//!         fps.appendleft((normalize_fp(fp), exp))
//!
//!     # Add positive exponents
//!     fp = (1 << bitshift, -bitshift)
//!     fps.append((normalize_fp(fp), 0))
//!     for exp in range(step, max_exp, step):
//!         fp = next_fp(fp, base, step)
//!         fps.append((normalize_fp(fp), exp))
//!
//!     # Return the smallest exp, AKA, the bias
//!     return fps, -fps[0][1]
//!
//!
//! def print_array(base, string, fps, index):
//!     '''Print an entire array'''
//!
//!     print(string.format(base, len(fps)))
//!     for fp, exp in fps:
//!         value = "    {},".format(fp[index])
//!         exp = EXP_STR.format(base, exp)
//!         print(value.ljust(30, " ") + exp)
//!     print("];")
//!
//!
//! def generate_base(base):
//!     '''Generate all powers and variables.'''
//!
//!     step = math.floor(math.log(1e10, base))
//!     small, ints = generate_small(base, step)
//!     large, bias = generate_large(base, step)
//!
//!     print_array(base, SMALL_MANTISSA_STR, small, 0)
//!     print_array(base, SMALL_EXPONENT_STR, small, 1)
//!     print_array(base, LARGE_MANTISSA_STR, large, 0)
//!     print_array(base, LARGE_EXPONENT_STR, large, 1)
//!     print(SMALL_INT_STR.format(base, len(ints), ints))
//!     print(STEP_STR.format(base, step))
//!     print(BIAS_STR.format(base, bias))
//!
//!
//! def generate():
//!     '''Generate all bases.'''
//!
//!     bases = [
//!         3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
//!         22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
//!     ]
//!
//!     for base in bases:
//!         print("// BASE{}\n".format(base))
//!         generate_base(base)
//!         print("")
//!
//!     print("// HIGH LEVEL\n// ----------\n")
//!
//!     for base in bases:
//!         print(POWER_STR.format(base))
//!
//!
//! if __name__ == '__main__':
//!     generate()
//! ```

use crate::traits::*;

use super::float80_decimal::*;
#[cfg(feature = "radix")]
use super::float80_radix::*;
use super::cache::ModeratePathPowers;

/// Get powers from base.
pub(crate) fn get_powers(radix: u32) -> &'static ModeratePathPowers<u64> {
    debug_assert_radix!(radix);

    #[cfg(not(feature = "radix"))]
    {
        &BASE10_POWERS
    }

    #[cfg(feature = "radix")]
    {
        match radix {
            3 => &BASE3_POWERS,
            5 => &BASE5_POWERS,
            6 => &BASE6_POWERS,
            7 => &BASE7_POWERS,
            9 => &BASE9_POWERS,
            10 => &BASE10_POWERS,
            11 => &BASE11_POWERS,
            12 => &BASE12_POWERS,
            13 => &BASE13_POWERS,
            14 => &BASE14_POWERS,
            15 => &BASE15_POWERS,
            17 => &BASE17_POWERS,
            18 => &BASE18_POWERS,
            19 => &BASE19_POWERS,
            20 => &BASE20_POWERS,
            21 => &BASE21_POWERS,
            22 => &BASE22_POWERS,
            23 => &BASE23_POWERS,
            24 => &BASE24_POWERS,
            25 => &BASE25_POWERS,
            26 => &BASE26_POWERS,
            27 => &BASE27_POWERS,
            28 => &BASE28_POWERS,
            29 => &BASE29_POWERS,
            30 => &BASE30_POWERS,
            31 => &BASE31_POWERS,
            33 => &BASE33_POWERS,
            34 => &BASE34_POWERS,
            35 => &BASE35_POWERS,
            36 => &BASE36_POWERS,
            // Powers of 2, and others, should already be handled by now.
            _ => unreachable!(),
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn normalization_test() {
        // Ensure each valid is normalized.
        for base in BASE_POWN.iter().cloned() {
            let powers = get_powers(base);
            for idx in 0..powers.small.len() {
                let fp = powers.get_small(idx);
                assert_eq!(fp.mant.leading_zeros(), 0);
            }
            for idx in 0..powers.large.len() {
                let fp = powers.get_large(idx);
                assert_eq!(fp.mant.leading_zeros(), 0);
            }
        }
    }

    #[cfg(feature = "radix")]
    #[test]
    #[should_panic]
    fn pow2_test() {
        for base in BASE_POW2.iter().cloned() {
            let _ = get_powers(base);
        }
    }
}
