//! Precalculated small powers.

use crate::util::Limb;
use static_assertions::const_assert;

#[cfg(feature = "power_of_two")]
use super::small64_binary;
use super::small64_decimal;
#[cfg(feature = "radix")]
use super::small64_radix;

cfg_if! {
if #[cfg(limb_width_32)] {
    use super::small32_decimal::*;
    #[cfg(feature = "power_of_two")]
    use super::small32_binary::*;
    #[cfg(feature = "radix")]
    use super::small32_radix::*;
} else {
    use super::small64_decimal::*;
    #[cfg(feature = "power_of_two")]
    use super::small64_binary::*;
    #[cfg(feature = "radix")]
    use super::small64_radix::*;
}} // cfg_if

cfg_if! {
if #[cfg(feature = "f128")] {
    use super::small128_decimal;
    #[cfg(feature = "power_of_two")]
    use super::small128_binary;
    #[cfg(feature = "radix")]
    use super::small128_radix;
}} // cfg_if

// ASSERTIONS
const_assert!(POW5[1] / POW5[0] == 5);
const_assert!(POW10[1] / POW10[0] == 10);

cfg_if! {
if #[cfg(feature = "power_of_two")] {
// Ensure our small powers are valid.
const_assert!(POW2[1] / POW2[0] == 2);
const_assert!(POW4[1] / POW4[0] == 4);
const_assert!(POW8[1] / POW8[0] == 8);
const_assert!(POW16[1] / POW16[0] == 16);
const_assert!(POW32[1] / POW32[0] == 32);
}} //cfg_if

cfg_if! {
if #[cfg(feature = "radix")] {
// Ensure our small powers are valid.
const_assert!(POW3[1] / POW3[0] == 3);
const_assert!(POW4[1] / POW4[0] == 4);
const_assert!(POW6[1] / POW6[0] == 6);
const_assert!(POW7[1] / POW7[0] == 7);
const_assert!(POW9[1] / POW9[0] == 9);
const_assert!(POW11[1] / POW11[0] == 11);
const_assert!(POW12[1] / POW12[0] == 12);
const_assert!(POW13[1] / POW13[0] == 13);
const_assert!(POW14[1] / POW14[0] == 14);
const_assert!(POW15[1] / POW15[0] == 15);
const_assert!(POW17[1] / POW17[0] == 17);
const_assert!(POW18[1] / POW18[0] == 18);
const_assert!(POW19[1] / POW19[0] == 19);
const_assert!(POW20[1] / POW20[0] == 20);
const_assert!(POW21[1] / POW21[0] == 21);
const_assert!(POW22[1] / POW22[0] == 22);
const_assert!(POW23[1] / POW23[0] == 23);
const_assert!(POW24[1] / POW24[0] == 24);
const_assert!(POW25[1] / POW25[0] == 25);
const_assert!(POW26[1] / POW26[0] == 26);
const_assert!(POW27[1] / POW27[0] == 27);
const_assert!(POW28[1] / POW28[0] == 28);
const_assert!(POW29[1] / POW29[0] == 29);
const_assert!(POW30[1] / POW30[0] == 30);
const_assert!(POW31[1] / POW31[0] == 31);
const_assert!(POW33[1] / POW33[0] == 33);
const_assert!(POW34[1] / POW34[0] == 34);
const_assert!(POW35[1] / POW35[0] == 35);
const_assert!(POW36[1] / POW36[0] == 36);
}} //cfg_if

// HELPERS

/// Get the correct small power from the radix.
#[inline]
pub(crate) fn get_small_powers(radix: u32) -> &'static [Limb] {
    #[cfg(not(feature = "power_of_two"))]
    {
        match radix {
            5 => &POW5,
            10 => &POW10,
            _ => unreachable!(),
        }
    }

    #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
    {
        match radix {
            2 => &POW2,
            5 => &POW5,
            4 => &POW4,
            8 => &POW8,
            10 => &POW10,
            16 => &POW16,
            32 => &POW32,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "radix")]
    {
        match radix {
            2 => &POW2,
            3 => &POW3,
            4 => &POW4,
            5 => &POW5,
            6 => &POW6,
            7 => &POW7,
            8 => &POW8,
            9 => &POW9,
            10 => &POW10,
            11 => &POW11,
            12 => &POW12,
            13 => &POW13,
            14 => &POW14,
            15 => &POW15,
            16 => &POW16,
            17 => &POW17,
            18 => &POW18,
            19 => &POW19,
            20 => &POW20,
            21 => &POW21,
            22 => &POW22,
            23 => &POW23,
            24 => &POW24,
            25 => &POW25,
            26 => &POW26,
            27 => &POW27,
            28 => &POW28,
            29 => &POW29,
            30 => &POW30,
            31 => &POW31,
            32 => &POW32,
            33 => &POW33,
            34 => &POW34,
            35 => &POW35,
            36 => &POW36,
            _ => unreachable!(),
        }
    }
}

/// Get the correct 64-bit small power from the radix.
#[inline]
pub(crate) fn get_small_powers_64(radix: u32) -> &'static [u64] {
    #[cfg(not(feature = "power_of_two"))]
    {
        match radix {
            5 => &small64_decimal::POW5,
            10 => &small64_decimal::POW10,
            _ => unreachable!(),
        }
    }

    #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
    {
        match radix {
            2 => &small64_binary::POW2,
            4 => &small64_binary::POW4,
            5 => &small64_decimal::POW5,
            8 => &small64_binary::POW8,
            10 => &small64_decimal::POW10,
            16 => &small64_binary::POW16,
            32 => &small64_binary::POW32,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "radix")]
    {
        match radix {
            2 => &small64_binary::POW2,
            3 => &small64_radix::POW3,
            4 => &small64_binary::POW4,
            5 => &small64_decimal::POW5,
            6 => &small64_radix::POW6,
            7 => &small64_radix::POW7,
            8 => &small64_binary::POW8,
            9 => &small64_radix::POW9,
            10 => &small64_decimal::POW10,
            11 => &small64_radix::POW11,
            12 => &small64_radix::POW12,
            13 => &small64_radix::POW13,
            14 => &small64_radix::POW14,
            15 => &small64_radix::POW15,
            16 => &small64_binary::POW16,
            17 => &small64_radix::POW17,
            18 => &small64_radix::POW18,
            19 => &small64_radix::POW19,
            20 => &small64_radix::POW20,
            21 => &small64_radix::POW21,
            22 => &small64_radix::POW22,
            23 => &small64_radix::POW23,
            24 => &small64_radix::POW24,
            25 => &small64_radix::POW25,
            26 => &small64_radix::POW26,
            27 => &small64_radix::POW27,
            28 => &small64_radix::POW28,
            29 => &small64_radix::POW29,
            30 => &small64_radix::POW30,
            31 => &small64_radix::POW31,
            32 => &small64_binary::POW32,
            33 => &small64_radix::POW33,
            34 => &small64_radix::POW34,
            35 => &small64_radix::POW35,
            36 => &small64_radix::POW36,
            _ => unreachable!(),
        }
    }
}

/// Get the correct 128-bit small powers from the radix.
#[inline]
#[cfg(feature = "f128")]
pub(crate) fn get_small_powers_128(radix: u32) -> &'static [u128] {
    #[cfg(not(feature = "power_of_two"))]
    {
        match radix {
            5 => &small128_decimal::POW5,
            10 => &small128_decimal::POW10,
            _ => unreachable!(),
        }
    }

    #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
    {
        match radix {
            2 => &small128_binary::POW2,
            4 => &small128_binary::POW4,
            5 => &small128_decimal::POW5,
            8 => &small128_binary::POW8,
            10 => &small128_decimal::POW10,
            16 => &small128_binary::POW16,
            32 => &small128_binary::POW32,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "radix")]
    {
        match radix {
            2 => &small128_binary::POW2,
            3 => &small128_radix::POW3,
            4 => &small128_binary::POW4,
            5 => &small128_decimal::POW5,
            6 => &small128_radix::POW6,
            7 => &small128_radix::POW7,
            8 => &small128_binary::POW8,
            9 => &small128_radix::POW9,
            10 => &small128_decimal::POW10,
            11 => &small128_radix::POW11,
            12 => &small128_radix::POW12,
            13 => &small128_radix::POW13,
            14 => &small128_radix::POW14,
            15 => &small128_radix::POW15,
            16 => &small128_binary::POW16,
            17 => &small128_radix::POW17,
            18 => &small128_radix::POW18,
            19 => &small128_radix::POW19,
            20 => &small128_radix::POW20,
            21 => &small128_radix::POW21,
            22 => &small128_radix::POW22,
            23 => &small128_radix::POW23,
            24 => &small128_radix::POW24,
            25 => &small128_radix::POW25,
            26 => &small128_radix::POW26,
            27 => &small128_radix::POW27,
            28 => &small128_radix::POW28,
            29 => &small128_radix::POW29,
            30 => &small128_radix::POW30,
            31 => &small128_radix::POW31,
            32 => &small128_binary::POW32,
            33 => &small128_radix::POW33,
            34 => &small128_radix::POW34,
            35 => &small128_radix::POW35,
            36 => &small128_radix::POW36,
            _ => unreachable!(),
        }
    }
}
