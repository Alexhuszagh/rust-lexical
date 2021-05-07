//! Precalculated small integral powers.

use crate::util::config::Limb;
use static_assertions::const_assert;

mod decimal64;
#[cfg(feature = "power_of_two")]
mod binary64;
#[cfg(feature = "radix")]
mod radix64;

cfg_if! {
if #[cfg(limb_width_32)] {
    mod decimal32;
    use self::decimal32::*;
    cfg_if! {
    if #[cfg(feature = "power_of_two")] {
        mod binary32;
        use self::binary32::*;
    }}  // cfg-if
    cfg_if! {
    if #[cfg(feature = "radix")] {
        mod radix32;
        use self::radix32::*;
    }}  // cfg-if
} else {
    use self::decimal64::*;
    #[cfg(feature = "power_of_two")]
    use self::binary64::*;
    #[cfg(feature = "radix")]
    use self::radix64::*;
}} // cfg_if

cfg_if! {
if #[cfg(feature = "f128")] {
    mod decimal128;
    #[cfg(feature = "power_of_two")]
    mod binary128;
    #[cfg(feature = "radix")]
    mod radix128;
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
            5 => &decimal64::POW5,
            10 => &decimal64::POW10,
            _ => unreachable!(),
        }
    }

    #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
    {
        match radix {
            2 => &binary64::POW2,
            4 => &binary64::POW4,
            5 => &decimal64::POW5,
            8 => &binary64::POW8,
            10 => &decimal64::POW10,
            16 => &binary64::POW16,
            32 => &binary64::POW32,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "radix")]
    {
        match radix {
            2 => &binary64::POW2,
            3 => &radix64::POW3,
            4 => &binary64::POW4,
            5 => &decimal64::POW5,
            6 => &radix64::POW6,
            7 => &radix64::POW7,
            8 => &binary64::POW8,
            9 => &radix64::POW9,
            10 => &decimal64::POW10,
            11 => &radix64::POW11,
            12 => &radix64::POW12,
            13 => &radix64::POW13,
            14 => &radix64::POW14,
            15 => &radix64::POW15,
            16 => &binary64::POW16,
            17 => &radix64::POW17,
            18 => &radix64::POW18,
            19 => &radix64::POW19,
            20 => &radix64::POW20,
            21 => &radix64::POW21,
            22 => &radix64::POW22,
            23 => &radix64::POW23,
            24 => &radix64::POW24,
            25 => &radix64::POW25,
            26 => &radix64::POW26,
            27 => &radix64::POW27,
            28 => &radix64::POW28,
            29 => &radix64::POW29,
            30 => &radix64::POW30,
            31 => &radix64::POW31,
            32 => &binary64::POW32,
            33 => &radix64::POW33,
            34 => &radix64::POW34,
            35 => &radix64::POW35,
            36 => &radix64::POW36,
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
            5 => &decimal128::POW5,
            10 => &decimal128::POW10,
            _ => unreachable!(),
        }
    }

    #[cfg(all(feature = "power_of_two", not(feature = "radix")))]
    {
        match radix {
            2 => &binary128::POW2,
            4 => &binary128::POW4,
            5 => &decimal128::POW5,
            8 => &binary128::POW8,
            10 => &decimal128::POW10,
            16 => &binary128::POW16,
            32 => &binary128::POW32,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "radix")]
    {
        match radix {
            2 => &binary128::POW2,
            3 => &radix128::POW3,
            4 => &binary128::POW4,
            5 => &decimal128::POW5,
            6 => &radix128::POW6,
            7 => &radix128::POW7,
            8 => &binary128::POW8,
            9 => &radix128::POW9,
            10 => &decimal128::POW10,
            11 => &radix128::POW11,
            12 => &radix128::POW12,
            13 => &radix128::POW13,
            14 => &radix128::POW14,
            15 => &radix128::POW15,
            16 => &binary128::POW16,
            17 => &radix128::POW17,
            18 => &radix128::POW18,
            19 => &radix128::POW19,
            20 => &radix128::POW20,
            21 => &radix128::POW21,
            22 => &radix128::POW22,
            23 => &radix128::POW23,
            24 => &radix128::POW24,
            25 => &radix128::POW25,
            26 => &radix128::POW26,
            27 => &radix128::POW27,
            28 => &radix128::POW28,
            29 => &radix128::POW29,
            30 => &radix128::POW30,
            31 => &radix128::POW31,
            32 => &binary128::POW32,
            33 => &radix128::POW33,
            34 => &radix128::POW34,
            35 => &radix128::POW35,
            36 => &radix128::POW36,
            _ => unreachable!(),
        }
    }
}
