//! Determine the limits of exact exponent and mantissas for floats.

// EXACT EXPONENT
// --------------

// Calculating the exponent limit requires determining the largest exponent
// we can calculate for a radix that can be **exactly** store in the
// float type. If the value is a power-of-two, then we simply
// need to scale the minimum, denormal exp and maximum exp to the type
// size. Otherwise, we need to calculate the number of digits
// that can fit into the type's precision, after removing a power-of-two
// (since these values can be represented exactly).
//
// The mantissa limit is the number of digits we can remove from
// the exponent into the mantissa, and is therefore is the
// `⌊ precision / log2(radix) ⌋`, where precision does not include
// the hidden bit.
//
// The algorithm for calculating both `exponent_limit` and `mantissa_limit`,
// in Python, can be done as follows:
//
// ```python
// import math
//
// def is_pow2(value):
//     '''Calculate if a value is a power of 2.'''
//
//     floor = int(math.log2(value))
//     return value == 2**floor
//
//
// def remove_pow2(value):
//     '''Remove a power of 2 from the value.'''
//
//     while math.floor(value / 2) == value / 2:
//         value //= 2
//     return value
//
//
// def exponent_limit(radix, mantissa_size, min_exp, max_exp):
//     '''
//     Calculate the exponent limit for a float, for a given
//     float type, where `radix` is the numerical base
//     for the float type, and mantissa size is the length
//     of the mantissa in bits. min_exp is the minimum,
//     denormal binary exponent, and max_exp is the maximum
//     binary exponent.
//     '''
//
//     if is_pow2(radix):
//         # Can always be exactly represented, calculate relative
//         # to min and max exp.
//         scaled_min_exp = int(min_exp / math.log2(radix))
//         scaled_max_exp = int(max_exp / math.log2(radix))
//         return (scaled_min_exp, scaled_max_exp)
//     else:
//         # Positive and negative should be the same,
//         # since we need to find the maximum digit
//         # representable with mantissa digits.
//         # We first need to remove the highest power-of-
//         # two from the radix, since these will be represented
//         # with exponent digits.
//         base = remove_pow2(radix)
//         precision = mantissa_size + 1
//         exp_limit = int(precision / math.log2(base))
//         return (-exp_limit, exp_limit)
//
//
// def mantissa_limit(radix, mantissa_size):
//     '''
//     Calculate mantissa limit for a float type, given
//     the radix and the length of the mantissa in bits.
//     '''
//
//     precision = mantissa_size + 1
//     return int(precision / math.log2(radix))
//
//
// def all_limits(mantissa_size, min_exp, max_exp):
//     '''Print limits for all radixes.'''
//
//     print('match radix {')
//     for radix in range(2, 37):
//         exp_limit = exponent_limit(radix, mantissa_size, min_exp, max_exp)
//         print(f'    {radix} => {exp_limit},')
//     print('}')
//
//     print('match radix {')
//     for radix in range(2, 37):
//         mant_limit = mantissa_limit(radix, mantissa_size)
//         print(f'    {radix} => {mant_limit},')
//     print('}')
// ```

/// Get exact exponent limit for radix.
#[doc(hidden)]
pub trait ExactFloat {
    /// Get min and max exponent limits (exact) from radix.
    fn exponent_limit(radix: u32) -> (i32, i32);

    /// Get the number of digits that can be shifted from exponent to mantissa.
    fn mantissa_limit(radix: u32) -> i32;
}

#[cfg(feature = "f16")]
impl ExactFloat for f16 {
    #[inline]
    fn exponent_limit(radix: u32) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-4, 4)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => (-24, 15),
                4 => (-12, 7),
                8 => (-8, 5),
                10 => (-4, 4),
                16 => (-6, 3),
                32 => (-4, 3),
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => (-24, 15),
                3 => (-6, 6),
                4 => (-12, 7),
                5 => (-4, 4),
                6 => (-6, 6),
                7 => (-3, 3),
                8 => (-8, 5),
                9 => (-3, 3),
                10 => (-4, 4),
                11 => (-3, 3),
                12 => (-6, 6),
                13 => (-2, 2),
                14 => (-3, 3),
                15 => (-2, 2),
                16 => (-6, 3),
                17 => (-2, 2),
                18 => (-3, 3),
                19 => (-2, 2),
                20 => (-4, 4),
                21 => (-2, 2),
                22 => (-3, 3),
                23 => (-2, 2),
                24 => (-6, 6),
                25 => (-2, 2),
                26 => (-2, 2),
                27 => (-2, 2),
                28 => (-3, 3),
                29 => (-2, 2),
                30 => (-2, 2),
                31 => (-2, 2),
                32 => (-4, 3),
                33 => (-2, 2),
                34 => (-2, 2),
                35 => (-2, 2),
                36 => (-3, 3),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit(radix: u32) -> i32 {
        debug_assert_radix!(radix.as_I32());
        #[cfg(not(feature = "power_of_two"))]
        {
            3
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => 11,
                4 => 5,
                8 => 3,
                10 => 3,
                16 => 2,
                32 => 2,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => 11,
                3 => 6,
                4 => 5,
                5 => 4,
                6 => 4,
                7 => 3,
                8 => 3,
                9 => 3,
                10 => 3,
                11 => 3,
                12 => 3,
                13 => 2,
                14 => 2,
                15 => 2,
                16 => 2,
                17 => 2,
                18 => 2,
                19 => 2,
                20 => 2,
                21 => 2,
                22 => 2,
                23 => 2,
                24 => 2,
                25 => 2,
                26 => 2,
                27 => 2,
                28 => 2,
                29 => 2,
                30 => 2,
                31 => 2,
                32 => 2,
                33 => 2,
                34 => 2,
                35 => 2,
                36 => 2,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(feature = "f16")]
impl ExactFloat for bf16 {
    #[inline]
    fn exponent_limit(radix: u32) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-3, 3)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => (-133, 127),
                4 => (-66, 63),
                8 => (-44, 42),
                10 => (-3, 3),
                16 => (-33, 31),
                32 => (-26, 25),
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => (-133, 127),
                3 => (-5, 5),
                4 => (-66, 63),
                5 => (-3, 3),
                6 => (-5, 5),
                7 => (-2, 2),
                8 => (-44, 42),
                9 => (-2, 2),
                10 => (-3, 3),
                11 => (-2, 2),
                12 => (-5, 5),
                13 => (-2, 2),
                14 => (-2, 2),
                15 => (-2, 2),
                16 => (-33, 31),
                17 => (-1, 1),
                18 => (-2, 2),
                19 => (-1, 1),
                20 => (-3, 3),
                21 => (-1, 1),
                22 => (-2, 2),
                23 => (-1, 1),
                24 => (-5, 5),
                25 => (-1, 1),
                26 => (-2, 2),
                27 => (-1, 1),
                28 => (-2, 2),
                29 => (-1, 1),
                30 => (-2, 2),
                31 => (-1, 1),
                32 => (-26, 25),
                33 => (-1, 1),
                34 => (-1, 1),
                35 => (-1, 1),
                36 => (-2, 2),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit(radix: u32) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            2
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => 8,
                4 => 4,
                8 => 2,
                10 => 2,
                16 => 2,
                32 => 1,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => 8,
                3 => 5,
                4 => 4,
                5 => 3,
                6 => 3,
                7 => 2,
                8 => 2,
                9 => 2,
                10 => 2,
                11 => 2,
                12 => 2,
                13 => 2,
                14 => 2,
                15 => 2,
                16 => 2,
                17 => 1,
                18 => 1,
                19 => 1,
                20 => 1,
                21 => 1,
                22 => 1,
                23 => 1,
                24 => 1,
                25 => 1,
                26 => 1,
                27 => 1,
                28 => 1,
                29 => 1,
                30 => 1,
                31 => 1,
                32 => 1,
                33 => 1,
                34 => 1,
                35 => 1,
                36 => 1,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

impl ExactFloat for f32 {
    #[inline]
    fn exponent_limit(radix: u32) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-10, 10)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => (-149, 127),
                4 => (-74, 63),
                8 => (-49, 42),
                10 => (-10, 10),
                16 => (-37, 31),
                32 => (-29, 25),
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => (-149, 127),
                3 => (-15, 15),
                4 => (-74, 63),
                5 => (-10, 10),
                6 => (-15, 15),
                7 => (-8, 8),
                8 => (-49, 42),
                9 => (-7, 7),
                10 => (-10, 10),
                11 => (-6, 6),
                12 => (-15, 15),
                13 => (-6, 6),
                14 => (-8, 8),
                15 => (-6, 6),
                16 => (-37, 31),
                17 => (-5, 5),
                18 => (-7, 7),
                19 => (-5, 5),
                20 => (-10, 10),
                21 => (-5, 5),
                22 => (-6, 6),
                23 => (-5, 5),
                24 => (-15, 15),
                25 => (-5, 5),
                26 => (-6, 6),
                27 => (-5, 5),
                28 => (-8, 8),
                29 => (-4, 4),
                30 => (-6, 6),
                31 => (-4, 4),
                32 => (-29, 25),
                33 => (-4, 4),
                34 => (-5, 5),
                35 => (-4, 4),
                36 => (-7, 7),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit(radix: u32) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            7
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => 24,
                4 => 12,
                8 => 8,
                10 => 7,
                16 => 6,
                32 => 4,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => 24,
                3 => 15,
                4 => 12,
                5 => 10,
                6 => 9,
                7 => 8,
                8 => 8,
                9 => 7,
                10 => 7,
                11 => 6,
                12 => 6,
                13 => 6,
                14 => 6,
                15 => 6,
                16 => 6,
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
                28 => 4,
                29 => 4,
                30 => 4,
                31 => 4,
                32 => 4,
                33 => 4,
                34 => 4,
                35 => 4,
                36 => 4,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

impl ExactFloat for f64 {
    #[inline]
    fn exponent_limit(radix: u32) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-22, 22)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => (-1074, 1023),
                4 => (-537, 511),
                8 => (-358, 341),
                10 => (-22, 22),
                16 => (-268, 255),
                32 => (-214, 204),
                // Invalid radix
                _ => unreachable!(),
            }
        }
        #[cfg(feature = "radix")]
        {
            match radix {
                2 => (-1074, 1023),
                3 => (-33, 33),
                4 => (-537, 511),
                5 => (-22, 22),
                6 => (-33, 33),
                7 => (-18, 18),
                8 => (-358, 341),
                9 => (-16, 16),
                10 => (-22, 22),
                11 => (-15, 15),
                12 => (-33, 33),
                13 => (-14, 14),
                14 => (-18, 18),
                15 => (-13, 13),
                16 => (-268, 255),
                17 => (-12, 12),
                18 => (-16, 16),
                19 => (-12, 12),
                20 => (-22, 22),
                21 => (-12, 12),
                22 => (-15, 15),
                23 => (-11, 11),
                24 => (-33, 33),
                25 => (-11, 11),
                26 => (-14, 14),
                27 => (-11, 11),
                28 => (-18, 18),
                29 => (-10, 10),
                30 => (-13, 13),
                31 => (-10, 10),
                32 => (-214, 204),
                33 => (-10, 10),
                34 => (-12, 12),
                35 => (-10, 10),
                36 => (-16, 16),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit(radix: u32) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            15
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => 53,
                4 => 26,
                8 => 17,
                10 => 15,
                16 => 13,
                32 => 10,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => 53,
                3 => 33,
                4 => 26,
                5 => 22,
                6 => 20,
                7 => 18,
                8 => 17,
                9 => 16,
                10 => 15,
                11 => 15,
                12 => 14,
                13 => 14,
                14 => 13,
                15 => 13,
                16 => 13,
                17 => 12,
                18 => 12,
                19 => 12,
                20 => 12,
                21 => 12,
                22 => 11,
                23 => 11,
                24 => 11,
                25 => 11,
                26 => 11,
                27 => 11,
                28 => 11,
                29 => 10,
                30 => 10,
                31 => 10,
                32 => 10,
                33 => 10,
                34 => 10,
                35 => 10,
                36 => 10,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(feature = "f128")]
impl ExactFloat for f128 {
    #[inline]
    fn exponent_limit(radix: u32) -> (i32, i32) {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            (-48, 48)
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => (-16494, 16383),
                4 => (-8247, 8191),
                8 => (-5498, 5461),
                10 => (-48, 48),
                16 => (-4123, 4095),
                32 => (-3298, 3276),
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => (-16494, 16383),
                3 => (-71, 71),
                4 => (-8247, 8191),
                5 => (-48, 48),
                6 => (-71, 71),
                7 => (-40, 40),
                8 => (-5498, 5461),
                9 => (-35, 35),
                10 => (-48, 48),
                11 => (-32, 32),
                12 => (-71, 71),
                13 => (-30, 30),
                14 => (-40, 40),
                15 => (-28, 28),
                16 => (-4123, 4095),
                17 => (-27, 27),
                18 => (-35, 35),
                19 => (-26, 26),
                20 => (-48, 48),
                21 => (-25, 25),
                22 => (-32, 32),
                23 => (-24, 24),
                24 => (-71, 71),
                25 => (-24, 24),
                26 => (-30, 30),
                27 => (-23, 23),
                28 => (-40, 40),
                29 => (-23, 23),
                30 => (-28, 28),
                31 => (-22, 22),
                32 => (-3298, 3276),
                33 => (-22, 22),
                34 => (-27, 27),
                35 => (-22, 22),
                36 => (-35, 35),
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn mantissa_limit(radix: u32) -> i32 {
        debug_assert_radix!(radix);
        #[cfg(not(feature = "power_of_two"))]
        {
            34
        }

        #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
        {
            match radix {
                2 => 113,
                4 => 56,
                8 => 37,
                10 => 34,
                16 => 28,
                32 => 22,
                // Invalid radix
                _ => unreachable!(),
            }
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                2 => 113,
                3 => 71,
                4 => 56,
                5 => 48,
                6 => 43,
                7 => 40,
                8 => 37,
                9 => 35,
                10 => 34,
                11 => 32,
                12 => 31,
                13 => 30,
                14 => 29,
                15 => 28,
                16 => 28,
                17 => 27,
                18 => 27,
                19 => 26,
                20 => 26,
                21 => 25,
                22 => 25,
                23 => 24,
                24 => 24,
                25 => 24,
                26 => 24,
                27 => 23,
                28 => 23,
                29 => 23,
                30 => 23,
                31 => 22,
                32 => 22,
                33 => 22,
                34 => 22,
                35 => 22,
                36 => 21,
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}
