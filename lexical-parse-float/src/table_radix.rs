//! Pre-computed tables for writing non-decimal strings.

#![cfg(feature = "radix")]
#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use crate::limits::{f32_exponent_limit, f64_exponent_limit, f64_mantissa_limit};
use crate::table_binary::*;
use crate::table_decimal::*;
use core::hint;
use lexical_util::assert::debug_assert_radix;
use static_assertions::const_assert;

// HELPERS
// -------

/// Get lookup table for 2 digit radix conversions.
///
/// # Safety
///
/// Safe as long as the radix provided is valid, and exponent is smaller
/// than the table for the radix.
#[inline]
pub unsafe fn get_small_int_power(exponent: usize, radix: u32) -> u64 {
    debug_assert_radix(radix);
    unsafe {
        match radix {
            2 => get_small_int_power2(exponent),
            3 => get_small_int_power3(exponent),
            4 => get_small_int_power4(exponent),
            5 => get_small_int_power5(exponent),
            6 => get_small_int_power6(exponent),
            7 => get_small_int_power7(exponent),
            8 => get_small_int_power8(exponent),
            9 => get_small_int_power9(exponent),
            10 => get_small_int_power10(exponent),
            11 => get_small_int_power11(exponent),
            12 => get_small_int_power12(exponent),
            13 => get_small_int_power13(exponent),
            14 => get_small_int_power14(exponent),
            15 => get_small_int_power15(exponent),
            16 => get_small_int_power16(exponent),
            17 => get_small_int_power17(exponent),
            18 => get_small_int_power18(exponent),
            19 => get_small_int_power19(exponent),
            20 => get_small_int_power20(exponent),
            21 => get_small_int_power21(exponent),
            22 => get_small_int_power22(exponent),
            23 => get_small_int_power23(exponent),
            24 => get_small_int_power24(exponent),
            25 => get_small_int_power25(exponent),
            26 => get_small_int_power26(exponent),
            27 => get_small_int_power27(exponent),
            28 => get_small_int_power28(exponent),
            29 => get_small_int_power29(exponent),
            30 => get_small_int_power30(exponent),
            31 => get_small_int_power31(exponent),
            32 => get_small_int_power32(exponent),
            33 => get_small_int_power33(exponent),
            34 => get_small_int_power34(exponent),
            35 => get_small_int_power35(exponent),
            36 => get_small_int_power36(exponent),
            _ => hint::unreachable_unchecked(),
        }
    }
}

/// Get lookup table for small f32 powers.
///
/// # Safety
///
/// Safe as long as the radix provided is valid, and exponent is smaller
/// than the table for the radix.
#[inline]
pub unsafe fn get_small_f32_power(exponent: usize, radix: u32) -> f32 {
    debug_assert_radix(radix);
    unsafe {
        match radix {
            2 => get_small_f32_power2(exponent),
            3 => get_small_f32_power3(exponent),
            4 => get_small_f32_power4(exponent),
            5 => get_small_f32_power5(exponent),
            6 => get_small_f32_power6(exponent),
            7 => get_small_f32_power7(exponent),
            8 => get_small_f32_power8(exponent),
            9 => get_small_f32_power9(exponent),
            10 => get_small_f32_power10(exponent),
            11 => get_small_f32_power11(exponent),
            12 => get_small_f32_power12(exponent),
            13 => get_small_f32_power13(exponent),
            14 => get_small_f32_power14(exponent),
            15 => get_small_f32_power15(exponent),
            16 => get_small_f32_power16(exponent),
            17 => get_small_f32_power17(exponent),
            18 => get_small_f32_power18(exponent),
            19 => get_small_f32_power19(exponent),
            20 => get_small_f32_power20(exponent),
            21 => get_small_f32_power21(exponent),
            22 => get_small_f32_power22(exponent),
            23 => get_small_f32_power23(exponent),
            24 => get_small_f32_power24(exponent),
            25 => get_small_f32_power25(exponent),
            26 => get_small_f32_power26(exponent),
            27 => get_small_f32_power27(exponent),
            28 => get_small_f32_power28(exponent),
            29 => get_small_f32_power29(exponent),
            30 => get_small_f32_power30(exponent),
            31 => get_small_f32_power31(exponent),
            32 => get_small_f32_power32(exponent),
            33 => get_small_f32_power33(exponent),
            34 => get_small_f32_power34(exponent),
            35 => get_small_f32_power35(exponent),
            36 => get_small_f32_power36(exponent),
            _ => hint::unreachable_unchecked(),
        }
    }
}

/// Get lookup table for small f64 powers.
///
/// # Safety
///
/// Safe as long as the radix provided is valid, and exponent is smaller
/// than the table for the radix.
#[inline]
pub unsafe fn get_small_f64_power(exponent: usize, radix: u32) -> f64 {
    debug_assert_radix(radix);
    unsafe {
        match radix {
            2 => get_small_f64_power2(exponent),
            3 => get_small_f64_power3(exponent),
            4 => get_small_f64_power4(exponent),
            5 => get_small_f64_power5(exponent),
            6 => get_small_f64_power6(exponent),
            7 => get_small_f64_power7(exponent),
            8 => get_small_f64_power8(exponent),
            9 => get_small_f64_power9(exponent),
            10 => get_small_f64_power10(exponent),
            11 => get_small_f64_power11(exponent),
            12 => get_small_f64_power12(exponent),
            13 => get_small_f64_power13(exponent),
            14 => get_small_f64_power14(exponent),
            15 => get_small_f64_power15(exponent),
            16 => get_small_f64_power16(exponent),
            17 => get_small_f64_power17(exponent),
            18 => get_small_f64_power18(exponent),
            19 => get_small_f64_power19(exponent),
            20 => get_small_f64_power20(exponent),
            21 => get_small_f64_power21(exponent),
            22 => get_small_f64_power22(exponent),
            23 => get_small_f64_power23(exponent),
            24 => get_small_f64_power24(exponent),
            25 => get_small_f64_power25(exponent),
            26 => get_small_f64_power26(exponent),
            27 => get_small_f64_power27(exponent),
            28 => get_small_f64_power28(exponent),
            29 => get_small_f64_power29(exponent),
            30 => get_small_f64_power30(exponent),
            31 => get_small_f64_power31(exponent),
            32 => get_small_f64_power32(exponent),
            33 => get_small_f64_power33(exponent),
            34 => get_small_f64_power34(exponent),
            35 => get_small_f64_power35(exponent),
            36 => get_small_f64_power36(exponent),
            _ => hint::unreachable_unchecked(),
        }
    }
}

/// Get pre-computed int power of 3.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW3.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power3(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW3[exponent]) }
}

/// Get pre-computed f32 power of 3.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW3.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power3(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW3[exponent]) }
}

/// Get pre-computed f64 power of 3.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW3.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power3(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW3[exponent]) }
}

/// Get pre-computed int power of 5.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW5.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power5(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW5[exponent]) }
}

/// Get pre-computed f32 power of 5.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW5.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power5(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW5[exponent]) }
}

/// Get pre-computed f64 power of 5.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW5.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power5(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW5[exponent]) }
}

/// Get pre-computed int power of 6.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW6.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power6(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW6[exponent]) }
}

/// Get pre-computed f32 power of 6.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW6.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power6(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW6[exponent]) }
}

/// Get pre-computed f64 power of 6.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW6.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power6(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW6[exponent]) }
}

/// Get pre-computed int power of 7.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW7.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power7(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW7[exponent]) }
}

/// Get pre-computed f32 power of 7.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW7.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power7(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW7[exponent]) }
}

/// Get pre-computed f64 power of 7.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW7.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power7(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW7[exponent]) }
}

/// Get pre-computed int power of 9.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW9.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power9(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW9[exponent]) }
}

/// Get pre-computed f32 power of 9.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW9.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power9(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW9[exponent]) }
}

/// Get pre-computed f64 power of 9.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW9.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power9(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW9[exponent]) }
}

/// Get pre-computed int power of 11.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW11.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power11(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW11[exponent]) }
}

/// Get pre-computed f32 power of 11.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW11.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power11(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW11[exponent]) }
}

/// Get pre-computed f64 power of 11.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW11.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power11(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW11[exponent]) }
}

/// Get pre-computed int power of 12.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW12.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power12(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW12[exponent]) }
}

/// Get pre-computed f32 power of 12.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW12.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power12(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW12[exponent]) }
}

/// Get pre-computed f64 power of 12.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW12.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power12(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW12[exponent]) }
}

/// Get pre-computed int power of 13.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW13.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power13(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW13[exponent]) }
}

/// Get pre-computed f32 power of 13.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW13.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power13(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW13[exponent]) }
}

/// Get pre-computed f64 power of 13.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW13.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power13(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW13[exponent]) }
}

/// Get pre-computed int power of 14.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW14.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power14(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW14[exponent]) }
}

/// Get pre-computed f32 power of 14.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW14.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power14(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW14[exponent]) }
}

/// Get pre-computed f64 power of 14.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW14.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power14(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW14[exponent]) }
}

/// Get pre-computed int power of 15.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW15.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power15(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW15[exponent]) }
}

/// Get pre-computed f32 power of 15.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW15.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power15(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW15[exponent]) }
}

/// Get pre-computed f64 power of 15.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW15.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power15(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW15[exponent]) }
}

/// Get pre-computed int power of 17.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW17.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power17(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW17[exponent]) }
}

/// Get pre-computed f32 power of 17.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW17.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power17(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW17[exponent]) }
}

/// Get pre-computed f64 power of 17.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW17.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power17(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW17[exponent]) }
}

/// Get pre-computed int power of 18.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW18.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power18(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW18[exponent]) }
}

/// Get pre-computed f32 power of 18.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW18.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power18(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW18[exponent]) }
}

/// Get pre-computed f64 power of 18.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW18.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power18(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW18[exponent]) }
}

/// Get pre-computed int power of 19.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW19.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power19(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW19[exponent]) }
}

/// Get pre-computed f32 power of 19.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW19.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power19(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW19[exponent]) }
}

/// Get pre-computed f64 power of 19.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW19.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power19(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW19[exponent]) }
}

/// Get pre-computed int power of 20.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW20.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power20(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW20[exponent]) }
}

/// Get pre-computed f32 power of 20.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW20.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power20(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW20[exponent]) }
}

/// Get pre-computed f64 power of 20.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW20.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power20(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW20[exponent]) }
}

/// Get pre-computed int power of 21.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW21.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power21(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW21[exponent]) }
}

/// Get pre-computed f32 power of 21.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW21.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power21(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW21[exponent]) }
}

/// Get pre-computed f64 power of 21.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW21.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power21(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW21[exponent]) }
}

/// Get pre-computed int power of 22.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW22.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power22(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW22[exponent]) }
}

/// Get pre-computed f32 power of 22.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW22.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power22(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW22[exponent]) }
}

/// Get pre-computed f64 power of 22.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW22.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power22(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW22[exponent]) }
}

/// Get pre-computed int power of 23.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW23.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power23(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW23[exponent]) }
}

/// Get pre-computed f32 power of 23.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW23.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power23(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW23[exponent]) }
}

/// Get pre-computed f64 power of 23.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW23.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power23(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW23[exponent]) }
}

/// Get pre-computed int power of 24.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW24.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power24(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW24[exponent]) }
}

/// Get pre-computed f32 power of 24.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW24.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power24(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW24[exponent]) }
}

/// Get pre-computed f64 power of 24.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW24.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power24(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW24[exponent]) }
}

/// Get pre-computed int power of 25.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW25.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power25(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW25[exponent]) }
}

/// Get pre-computed f32 power of 25.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW25.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power25(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW25[exponent]) }
}

/// Get pre-computed f64 power of 25.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW25.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power25(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW25[exponent]) }
}

/// Get pre-computed int power of 26.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW26.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power26(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW26[exponent]) }
}

/// Get pre-computed f32 power of 26.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW26.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power26(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW26[exponent]) }
}

/// Get pre-computed f64 power of 26.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW26.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power26(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW26[exponent]) }
}

/// Get pre-computed int power of 27.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW27.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power27(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW27[exponent]) }
}

/// Get pre-computed f32 power of 27.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW27.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power27(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW27[exponent]) }
}

/// Get pre-computed f64 power of 27.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW27.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power27(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW27[exponent]) }
}

/// Get pre-computed int power of 28.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW28.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power28(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW28[exponent]) }
}

/// Get pre-computed f32 power of 28.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW28.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power28(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW28[exponent]) }
}

/// Get pre-computed f64 power of 28.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW28.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power28(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW28[exponent]) }
}

/// Get pre-computed int power of 29.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW29.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power29(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW29[exponent]) }
}

/// Get pre-computed f32 power of 29.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW29.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power29(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW29[exponent]) }
}

/// Get pre-computed f64 power of 29.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW29.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power29(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW29[exponent]) }
}

/// Get pre-computed int power of 30.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW30.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power30(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW30[exponent]) }
}

/// Get pre-computed f32 power of 30.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW30.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power30(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW30[exponent]) }
}

/// Get pre-computed f64 power of 30.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW30.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power30(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW30[exponent]) }
}

/// Get pre-computed int power of 31.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW31.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power31(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW31[exponent]) }
}

/// Get pre-computed f32 power of 31.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW31.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power31(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW31[exponent]) }
}

/// Get pre-computed f64 power of 31.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW31.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power31(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW31[exponent]) }
}

/// Get pre-computed int power of 33.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW33.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power33(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW33[exponent]) }
}

/// Get pre-computed f32 power of 33.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW33.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power33(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW33[exponent]) }
}

/// Get pre-computed f64 power of 33.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW33.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power33(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW33[exponent]) }
}

/// Get pre-computed int power of 34.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW34.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power34(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW34[exponent]) }
}

/// Get pre-computed f32 power of 34.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW34.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power34(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW34[exponent]) }
}

/// Get pre-computed f64 power of 34.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW34.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power34(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW34[exponent]) }
}

/// Get pre-computed int power of 35.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW35.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power35(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW35[exponent]) }
}

/// Get pre-computed f32 power of 35.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW35.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power35(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW35[exponent]) }
}

/// Get pre-computed f64 power of 35.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW35.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power35(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW35[exponent]) }
}

/// Get pre-computed int power of 36.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_INT_POW36.len()`.
#[inline(always)]
pub unsafe fn get_small_int_power36(exponent: usize) -> u64 {
    unsafe { index_unchecked!(SMALL_INT_POW36[exponent]) }
}

/// Get pre-computed f32 power of 36.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F32_POW36.len()`.
#[inline(always)]
pub unsafe fn get_small_f32_power36(exponent: usize) -> f32 {
    unsafe { index_unchecked!(SMALL_F32_POW36[exponent]) }
}

/// Get pre-computed f64 power of 36.
///
/// # Safety
///
/// Safe as long as the `exponent < SMALL_F64_POW36.len()`.
#[inline(always)]
pub unsafe fn get_small_f64_power36(exponent: usize) -> f64 {
    unsafe { index_unchecked!(SMALL_F64_POW36[exponent]) }
}

// TABLES
// ------

//  NOTE:
//      These tables were automatically generated using the following
//      script. Do not modify them unless you have a very good reason to.
//  ```text
//  def print_int(radix, max_exp):
//      print(f'/// Pre-computed, small powers-of-{radix}.')
//      print(f'pub const SMALL_INT_POW{radix}: [u64; {max_exp + 1}] = [')
//      for exponent in range(0, max_exp + 1):
//          print(f'    {radix**exponent},')
//      print('];')
//      print(f'const_assert!(SMALL_INT_POW{radix}.len() > f64_mantissa_limit({radix}) as usize);')
//      print('')
//
//  def print_f32(radix, max_exp):
//      print(f'/// Pre-computed, small powers-of-{radix}.')
//      print(f'pub const SMALL_F32_POW{radix}: [f32; {max_exp + 1}] = [')
//      for exponent in range(0, max_exp + 1):
//          print(f'    {float(radix)**exponent},')
//      print('];')
//      print(f'const_assert!(SMALL_F32_POW{radix}.len() > f32_exponent_limit({radix}).1 as usize);')
//      print('')
//
//  def print_f64(radix, max_exp):
//      print(f'/// Pre-computed, small powers-of-{radix}.')
//      print(f'pub const SMALL_F64_POW{radix}: [f64; {max_exp + 1}] = [')
//      for exponent in range(0, max_exp + 1):
//          print(f'    {float(radix)**exponent},')
//      print('];')
//      print(f'const_assert!(SMALL_F64_POW{radix}.len() > f64_exponent_limit({radix}).1 as usize);')
//      print('')
//
//  def print_tables(radix, f64_mant_limit, f32_exp_limit, f64_exp_limit):
//      print_int(radix, f64_mant_limit)
//      print_f32(radix, f32_exp_limit)
//      print_f64(radix, f64_exp_limit)
//
//  def f32_exponent_limit(radix):
//      return {
//          3 : (-15, 15),
//          5 : (-10, 10),
//          6 : (-15, 15),
//          7 : (-8, 8),
//          9 : (-7, 7),
//          11: (-6, 6),
//          12: (-15, 15),
//          13: (-6, 6),
//          14: (-8, 8),
//          15: (-6, 6),
//          17: (-5, 5),
//          18: (-7, 7),
//          19: (-5, 5),
//          20: (-10, 10),
//          21: (-5, 5),
//          22: (-6, 6),
//          23: (-5, 5),
//          24: (-15, 15),
//          25: (-5, 5),
//          26: (-6, 6),
//          27: (-5, 5),
//          28: (-8, 8),
//          29: (-4, 4),
//          30: (-6, 6),
//          31: (-4, 4),
//          33: (-4, 4),
//          34: (-5, 5),
//          35: (-4, 4),
//          36: (-7, 7),
//      }[radix]
//
//  def f64_exponent_limit(radix):
//      return {
//          3: (-33, 33),
//          5: (-22, 22),
//          6: (-33, 33),
//          7: (-18, 18),
//          9: (-16, 16),
//          11: (-15, 15),
//          12: (-33, 33),
//          13: (-14, 14),
//          14: (-18, 18),
//          15: (-13, 13),
//          17: (-12, 12),
//          18: (-16, 16),
//          19: (-12, 12),
//          20: (-22, 22),
//          21: (-12, 12),
//          22: (-15, 15),
//          23: (-11, 11),
//          24: (-33, 33),
//          25: (-11, 11),
//          26: (-14, 14),
//          27: (-11, 11),
//          28: (-18, 18),
//          29: (-10, 10),
//          30: (-13, 13),
//          31: (-10, 10),
//          33: (-10, 10),
//          34: (-12, 12),
//          35: (-10, 10),
//          36: (-16, 16),
//      }[radix]
//
//  def f64_mantissa_limit(radix):
//      return {
//          3: 33,
//          5: 22,
//          6: 20,
//          7: 18,
//          9: 16,
//          11: 15,
//          12: 14,
//          13: 14,
//          14: 13,
//          15: 13,
//          17: 12,
//          18: 12,
//          19: 12,
//          20: 12,
//          21: 12,
//          22: 11,
//          23: 11,
//          24: 11,
//          25: 11,
//          26: 11,
//          27: 11,
//          28: 11,
//          29: 10,
//          30: 10,
//          31: 10,
//          33: 10,
//          34: 10,
//          35: 10,
//          36: 10,
//      }[radix]
//
//  radixes = [3, 5, 6, 7, 9, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36]
//  for radix in radixes:
//      f64_mant_limit = f64_mantissa_limit(radix)
//      f32_exp_limit = f32_exponent_limit(radix)[1]
//      f64_exp_limit = f64_exponent_limit(radix)[1]
//      print_tables(radix, f64_mant_limit, f32_exp_limit, f64_exp_limit)
//  ```

/// Pre-computed, small powers-of-3.
pub const SMALL_INT_POW3: [u64; 34] = [
    1,
    3,
    9,
    27,
    81,
    243,
    729,
    2187,
    6561,
    19683,
    59049,
    177147,
    531441,
    1594323,
    4782969,
    14348907,
    43046721,
    129140163,
    387420489,
    1162261467,
    3486784401,
    10460353203,
    31381059609,
    94143178827,
    282429536481,
    847288609443,
    2541865828329,
    7625597484987,
    22876792454961,
    68630377364883,
    205891132094649,
    617673396283947,
    1853020188851841,
    5559060566555523,
];
const_assert!(SMALL_INT_POW3.len() > f64_mantissa_limit(3) as usize);

/// Pre-computed, small powers-of-3.
pub const SMALL_F32_POW3: [f32; 16] = [
    1.0, 3.0, 9.0, 27.0, 81.0, 243.0, 729.0, 2187.0, 6561.0, 19683.0, 59049.0, 177147.0, 531441.0,
    1594323.0, 4782969.0, 14348907.0,
];
const_assert!(SMALL_F32_POW3.len() > f32_exponent_limit(3).1 as usize);

/// Pre-computed, small powers-of-3.
pub const SMALL_F64_POW3: [f64; 34] = [
    1.0,
    3.0,
    9.0,
    27.0,
    81.0,
    243.0,
    729.0,
    2187.0,
    6561.0,
    19683.0,
    59049.0,
    177147.0,
    531441.0,
    1594323.0,
    4782969.0,
    14348907.0,
    43046721.0,
    129140163.0,
    387420489.0,
    1162261467.0,
    3486784401.0,
    10460353203.0,
    31381059609.0,
    94143178827.0,
    282429536481.0,
    847288609443.0,
    2541865828329.0,
    7625597484987.0,
    22876792454961.0,
    68630377364883.0,
    205891132094649.0,
    617673396283947.0,
    1853020188851841.0,
    5559060566555523.0,
];
const_assert!(SMALL_F64_POW3.len() > f64_exponent_limit(3).1 as usize);

/// Pre-computed, small powers-of-5.
pub const SMALL_INT_POW5: [u64; 23] = [
    1,
    5,
    25,
    125,
    625,
    3125,
    15625,
    78125,
    390625,
    1953125,
    9765625,
    48828125,
    244140625,
    1220703125,
    6103515625,
    30517578125,
    152587890625,
    762939453125,
    3814697265625,
    19073486328125,
    95367431640625,
    476837158203125,
    2384185791015625,
];
const_assert!(SMALL_INT_POW5.len() > f64_mantissa_limit(5) as usize);

/// Pre-computed, small powers-of-5.
pub const SMALL_F32_POW5: [f32; 11] =
    [1.0, 5.0, 25.0, 125.0, 625.0, 3125.0, 15625.0, 78125.0, 390625.0, 1953125.0, 9765625.0];
const_assert!(SMALL_F32_POW5.len() > f32_exponent_limit(5).1 as usize);

/// Pre-computed, small powers-of-5.
pub const SMALL_F64_POW5: [f64; 23] = [
    1.0,
    5.0,
    25.0,
    125.0,
    625.0,
    3125.0,
    15625.0,
    78125.0,
    390625.0,
    1953125.0,
    9765625.0,
    48828125.0,
    244140625.0,
    1220703125.0,
    6103515625.0,
    30517578125.0,
    152587890625.0,
    762939453125.0,
    3814697265625.0,
    19073486328125.0,
    95367431640625.0,
    476837158203125.0,
    2384185791015625.0,
];
const_assert!(SMALL_F64_POW5.len() > f64_exponent_limit(5).1 as usize);

/// Pre-computed, small powers-of-6.
pub const SMALL_INT_POW6: [u64; 21] = [
    1,
    6,
    36,
    216,
    1296,
    7776,
    46656,
    279936,
    1679616,
    10077696,
    60466176,
    362797056,
    2176782336,
    13060694016,
    78364164096,
    470184984576,
    2821109907456,
    16926659444736,
    101559956668416,
    609359740010496,
    3656158440062976,
];
const_assert!(SMALL_INT_POW6.len() > f64_mantissa_limit(6) as usize);

/// Pre-computed, small powers-of-6.
pub const SMALL_F32_POW6: [f32; 16] = [
    1.0,
    6.0,
    36.0,
    216.0,
    1296.0,
    7776.0,
    46656.0,
    279936.0,
    1679616.0,
    10077696.0,
    60466176.0,
    362797056.0,
    2176782336.0,
    13060694016.0,
    78364164096.0,
    470184984576.0,
];
const_assert!(SMALL_F32_POW6.len() > f32_exponent_limit(6).1 as usize);

/// Pre-computed, small powers-of-6.
pub const SMALL_F64_POW6: [f64; 34] = [
    1.0,
    6.0,
    36.0,
    216.0,
    1296.0,
    7776.0,
    46656.0,
    279936.0,
    1679616.0,
    10077696.0,
    60466176.0,
    362797056.0,
    2176782336.0,
    13060694016.0,
    78364164096.0,
    470184984576.0,
    2821109907456.0,
    16926659444736.0,
    101559956668416.0,
    609359740010496.0,
    3656158440062976.0,
    2.1936950640377856e+16,
    1.3162170384226714e+17,
    7.897302230536028e+17,
    4.738381338321617e+18,
    2.84302880299297e+19,
    1.705817281795782e+20,
    1.0234903690774692e+21,
    6.140942214464815e+21,
    3.6845653286788893e+22,
    2.2107391972073336e+23,
    1.3264435183244001e+24,
    7.958661109946401e+24,
    4.7751966659678405e+25,
];
const_assert!(SMALL_F64_POW6.len() > f64_exponent_limit(6).1 as usize);

/// Pre-computed, small powers-of-7.
pub const SMALL_INT_POW7: [u64; 19] = [
    1,
    7,
    49,
    343,
    2401,
    16807,
    117649,
    823543,
    5764801,
    40353607,
    282475249,
    1977326743,
    13841287201,
    96889010407,
    678223072849,
    4747561509943,
    33232930569601,
    232630513987207,
    1628413597910449,
];
const_assert!(SMALL_INT_POW7.len() > f64_mantissa_limit(7) as usize);

/// Pre-computed, small powers-of-7.
pub const SMALL_F32_POW7: [f32; 9] =
    [1.0, 7.0, 49.0, 343.0, 2401.0, 16807.0, 117649.0, 823543.0, 5764801.0];
const_assert!(SMALL_F32_POW7.len() > f32_exponent_limit(7).1 as usize);

/// Pre-computed, small powers-of-7.
pub const SMALL_F64_POW7: [f64; 19] = [
    1.0,
    7.0,
    49.0,
    343.0,
    2401.0,
    16807.0,
    117649.0,
    823543.0,
    5764801.0,
    40353607.0,
    282475249.0,
    1977326743.0,
    13841287201.0,
    96889010407.0,
    678223072849.0,
    4747561509943.0,
    33232930569601.0,
    232630513987207.0,
    1628413597910449.0,
];
const_assert!(SMALL_F64_POW7.len() > f64_exponent_limit(7).1 as usize);

/// Pre-computed, small powers-of-9.
pub const SMALL_INT_POW9: [u64; 17] = [
    1,
    9,
    81,
    729,
    6561,
    59049,
    531441,
    4782969,
    43046721,
    387420489,
    3486784401,
    31381059609,
    282429536481,
    2541865828329,
    22876792454961,
    205891132094649,
    1853020188851841,
];
const_assert!(SMALL_INT_POW9.len() > f64_mantissa_limit(9) as usize);

/// Pre-computed, small powers-of-9.
pub const SMALL_F32_POW9: [f32; 8] = [1.0, 9.0, 81.0, 729.0, 6561.0, 59049.0, 531441.0, 4782969.0];
const_assert!(SMALL_F32_POW9.len() > f32_exponent_limit(9).1 as usize);

/// Pre-computed, small powers-of-9.
pub const SMALL_F64_POW9: [f64; 17] = [
    1.0,
    9.0,
    81.0,
    729.0,
    6561.0,
    59049.0,
    531441.0,
    4782969.0,
    43046721.0,
    387420489.0,
    3486784401.0,
    31381059609.0,
    282429536481.0,
    2541865828329.0,
    22876792454961.0,
    205891132094649.0,
    1853020188851841.0,
];
const_assert!(SMALL_F64_POW9.len() > f64_exponent_limit(9).1 as usize);

/// Pre-computed, small powers-of-11.
pub const SMALL_INT_POW11: [u64; 16] = [
    1,
    11,
    121,
    1331,
    14641,
    161051,
    1771561,
    19487171,
    214358881,
    2357947691,
    25937424601,
    285311670611,
    3138428376721,
    34522712143931,
    379749833583241,
    4177248169415651,
];
const_assert!(SMALL_INT_POW11.len() > f64_mantissa_limit(11) as usize);

/// Pre-computed, small powers-of-11.
pub const SMALL_F32_POW11: [f32; 7] = [1.0, 11.0, 121.0, 1331.0, 14641.0, 161051.0, 1771561.0];
const_assert!(SMALL_F32_POW11.len() > f32_exponent_limit(11).1 as usize);

/// Pre-computed, small powers-of-11.
pub const SMALL_F64_POW11: [f64; 16] = [
    1.0,
    11.0,
    121.0,
    1331.0,
    14641.0,
    161051.0,
    1771561.0,
    19487171.0,
    214358881.0,
    2357947691.0,
    25937424601.0,
    285311670611.0,
    3138428376721.0,
    34522712143931.0,
    379749833583241.0,
    4177248169415651.0,
];
const_assert!(SMALL_F64_POW11.len() > f64_exponent_limit(11).1 as usize);

/// Pre-computed, small powers-of-12.
pub const SMALL_INT_POW12: [u64; 15] = [
    1,
    12,
    144,
    1728,
    20736,
    248832,
    2985984,
    35831808,
    429981696,
    5159780352,
    61917364224,
    743008370688,
    8916100448256,
    106993205379072,
    1283918464548864,
];
const_assert!(SMALL_INT_POW12.len() > f64_mantissa_limit(12) as usize);

/// Pre-computed, small powers-of-12.
pub const SMALL_F32_POW12: [f32; 16] = [
    1.0,
    12.0,
    144.0,
    1728.0,
    20736.0,
    248832.0,
    2985984.0,
    35831808.0,
    429981696.0,
    5159780352.0,
    61917364224.0,
    743008370688.0,
    8916100448256.0,
    106993205379072.0,
    1283918464548864.0,
    1.5407021574586368e+16,
];
const_assert!(SMALL_F32_POW12.len() > f32_exponent_limit(12).1 as usize);

/// Pre-computed, small powers-of-12.
pub const SMALL_F64_POW12: [f64; 34] = [
    1.0,
    12.0,
    144.0,
    1728.0,
    20736.0,
    248832.0,
    2985984.0,
    35831808.0,
    429981696.0,
    5159780352.0,
    61917364224.0,
    743008370688.0,
    8916100448256.0,
    106993205379072.0,
    1283918464548864.0,
    1.5407021574586368e+16,
    1.848842588950364e+17,
    2.218611106740437e+18,
    2.6623333280885244e+19,
    3.194799993706229e+20,
    3.833759992447475e+21,
    4.60051199093697e+22,
    5.520614389124364e+23,
    6.624737266949237e+24,
    7.949684720339084e+25,
    9.539621664406901e+26,
    1.1447545997288282e+28,
    1.3737055196745938e+29,
    1.6484466236095125e+30,
    1.978135948331415e+31,
    2.373763137997698e+32,
    2.8485157655972377e+33,
    3.418218918716685e+34,
    4.101862702460022e+35,
];
const_assert!(SMALL_F64_POW12.len() > f64_exponent_limit(12).1 as usize);

/// Pre-computed, small powers-of-13.
pub const SMALL_INT_POW13: [u64; 15] = [
    1,
    13,
    169,
    2197,
    28561,
    371293,
    4826809,
    62748517,
    815730721,
    10604499373,
    137858491849,
    1792160394037,
    23298085122481,
    302875106592253,
    3937376385699289,
];
const_assert!(SMALL_INT_POW13.len() > f64_mantissa_limit(13) as usize);

/// Pre-computed, small powers-of-13.
pub const SMALL_F32_POW13: [f32; 7] = [1.0, 13.0, 169.0, 2197.0, 28561.0, 371293.0, 4826809.0];
const_assert!(SMALL_F32_POW13.len() > f32_exponent_limit(13).1 as usize);

/// Pre-computed, small powers-of-13.
pub const SMALL_F64_POW13: [f64; 15] = [
    1.0,
    13.0,
    169.0,
    2197.0,
    28561.0,
    371293.0,
    4826809.0,
    62748517.0,
    815730721.0,
    10604499373.0,
    137858491849.0,
    1792160394037.0,
    23298085122481.0,
    302875106592253.0,
    3937376385699289.0,
];
const_assert!(SMALL_F64_POW13.len() > f64_exponent_limit(13).1 as usize);

/// Pre-computed, small powers-of-14.
pub const SMALL_INT_POW14: [u64; 14] = [
    1,
    14,
    196,
    2744,
    38416,
    537824,
    7529536,
    105413504,
    1475789056,
    20661046784,
    289254654976,
    4049565169664,
    56693912375296,
    793714773254144,
];
const_assert!(SMALL_INT_POW14.len() > f64_mantissa_limit(14) as usize);

/// Pre-computed, small powers-of-14.
pub const SMALL_F32_POW14: [f32; 9] =
    [1.0, 14.0, 196.0, 2744.0, 38416.0, 537824.0, 7529536.0, 105413504.0, 1475789056.0];
const_assert!(SMALL_F32_POW14.len() > f32_exponent_limit(14).1 as usize);

/// Pre-computed, small powers-of-14.
pub const SMALL_F64_POW14: [f64; 19] = [
    1.0,
    14.0,
    196.0,
    2744.0,
    38416.0,
    537824.0,
    7529536.0,
    105413504.0,
    1475789056.0,
    20661046784.0,
    289254654976.0,
    4049565169664.0,
    56693912375296.0,
    793714773254144.0,
    1.1112006825558016e+16,
    1.5556809555781222e+17,
    2.1779533378093711e+18,
    3.0491346729331196e+19,
    4.2687885421063674e+20,
];
const_assert!(SMALL_F64_POW14.len() > f64_exponent_limit(14).1 as usize);

/// Pre-computed, small powers-of-15.
pub const SMALL_INT_POW15: [u64; 14] = [
    1,
    15,
    225,
    3375,
    50625,
    759375,
    11390625,
    170859375,
    2562890625,
    38443359375,
    576650390625,
    8649755859375,
    129746337890625,
    1946195068359375,
];
const_assert!(SMALL_INT_POW15.len() > f64_mantissa_limit(15) as usize);

/// Pre-computed, small powers-of-15.
pub const SMALL_F32_POW15: [f32; 7] = [1.0, 15.0, 225.0, 3375.0, 50625.0, 759375.0, 11390625.0];
const_assert!(SMALL_F32_POW15.len() > f32_exponent_limit(15).1 as usize);

/// Pre-computed, small powers-of-15.
pub const SMALL_F64_POW15: [f64; 14] = [
    1.0,
    15.0,
    225.0,
    3375.0,
    50625.0,
    759375.0,
    11390625.0,
    170859375.0,
    2562890625.0,
    38443359375.0,
    576650390625.0,
    8649755859375.0,
    129746337890625.0,
    1946195068359375.0,
];
const_assert!(SMALL_F64_POW15.len() > f64_exponent_limit(15).1 as usize);

/// Pre-computed, small powers-of-17.
pub const SMALL_INT_POW17: [u64; 13] = [
    1,
    17,
    289,
    4913,
    83521,
    1419857,
    24137569,
    410338673,
    6975757441,
    118587876497,
    2015993900449,
    34271896307633,
    582622237229761,
];
const_assert!(SMALL_INT_POW17.len() > f64_mantissa_limit(17) as usize);

/// Pre-computed, small powers-of-17.
pub const SMALL_F32_POW17: [f32; 6] = [1.0, 17.0, 289.0, 4913.0, 83521.0, 1419857.0];
const_assert!(SMALL_F32_POW17.len() > f32_exponent_limit(17).1 as usize);

/// Pre-computed, small powers-of-17.
pub const SMALL_F64_POW17: [f64; 13] = [
    1.0,
    17.0,
    289.0,
    4913.0,
    83521.0,
    1419857.0,
    24137569.0,
    410338673.0,
    6975757441.0,
    118587876497.0,
    2015993900449.0,
    34271896307633.0,
    582622237229761.0,
];
const_assert!(SMALL_F64_POW17.len() > f64_exponent_limit(17).1 as usize);

/// Pre-computed, small powers-of-18.
pub const SMALL_INT_POW18: [u64; 13] = [
    1,
    18,
    324,
    5832,
    104976,
    1889568,
    34012224,
    612220032,
    11019960576,
    198359290368,
    3570467226624,
    64268410079232,
    1156831381426176,
];
const_assert!(SMALL_INT_POW18.len() > f64_mantissa_limit(18) as usize);

/// Pre-computed, small powers-of-18.
pub const SMALL_F32_POW18: [f32; 8] =
    [1.0, 18.0, 324.0, 5832.0, 104976.0, 1889568.0, 34012224.0, 612220032.0];
const_assert!(SMALL_F32_POW18.len() > f32_exponent_limit(18).1 as usize);

/// Pre-computed, small powers-of-18.
pub const SMALL_F64_POW18: [f64; 17] = [
    1.0,
    18.0,
    324.0,
    5832.0,
    104976.0,
    1889568.0,
    34012224.0,
    612220032.0,
    11019960576.0,
    198359290368.0,
    3570467226624.0,
    64268410079232.0,
    1156831381426176.0,
    2.082296486567117e+16,
    3.74813367582081e+17,
    6.746640616477458e+18,
    1.2143953109659425e+20,
];
const_assert!(SMALL_F64_POW18.len() > f64_exponent_limit(18).1 as usize);

/// Pre-computed, small powers-of-19.
pub const SMALL_INT_POW19: [u64; 13] = [
    1,
    19,
    361,
    6859,
    130321,
    2476099,
    47045881,
    893871739,
    16983563041,
    322687697779,
    6131066257801,
    116490258898219,
    2213314919066161,
];
const_assert!(SMALL_INT_POW19.len() > f64_mantissa_limit(19) as usize);

/// Pre-computed, small powers-of-19.
pub const SMALL_F32_POW19: [f32; 6] = [1.0, 19.0, 361.0, 6859.0, 130321.0, 2476099.0];
const_assert!(SMALL_F32_POW19.len() > f32_exponent_limit(19).1 as usize);

/// Pre-computed, small powers-of-19.
pub const SMALL_F64_POW19: [f64; 13] = [
    1.0,
    19.0,
    361.0,
    6859.0,
    130321.0,
    2476099.0,
    47045881.0,
    893871739.0,
    16983563041.0,
    322687697779.0,
    6131066257801.0,
    116490258898219.0,
    2213314919066161.0,
];
const_assert!(SMALL_F64_POW19.len() > f64_exponent_limit(19).1 as usize);

/// Pre-computed, small powers-of-20.
pub const SMALL_INT_POW20: [u64; 13] = [
    1,
    20,
    400,
    8000,
    160000,
    3200000,
    64000000,
    1280000000,
    25600000000,
    512000000000,
    10240000000000,
    204800000000000,
    4096000000000000,
];
const_assert!(SMALL_INT_POW20.len() > f64_mantissa_limit(20) as usize);

/// Pre-computed, small powers-of-20.
pub const SMALL_F32_POW20: [f32; 11] = [
    1.0,
    20.0,
    400.0,
    8000.0,
    160000.0,
    3200000.0,
    64000000.0,
    1280000000.0,
    25600000000.0,
    512000000000.0,
    10240000000000.0,
];
const_assert!(SMALL_F32_POW20.len() > f32_exponent_limit(20).1 as usize);

/// Pre-computed, small powers-of-20.
pub const SMALL_F64_POW20: [f64; 23] = [
    1.0,
    20.0,
    400.0,
    8000.0,
    160000.0,
    3200000.0,
    64000000.0,
    1280000000.0,
    25600000000.0,
    512000000000.0,
    10240000000000.0,
    204800000000000.0,
    4096000000000000.0,
    8.192e+16,
    1.6384e+18,
    3.2768e+19,
    6.5536e+20,
    1.31072e+22,
    2.62144e+23,
    5.24288e+24,
    1.048576e+26,
    2.097152e+27,
    4.194304e+28,
];
const_assert!(SMALL_F64_POW20.len() > f64_exponent_limit(20).1 as usize);

/// Pre-computed, small powers-of-21.
pub const SMALL_INT_POW21: [u64; 13] = [
    1,
    21,
    441,
    9261,
    194481,
    4084101,
    85766121,
    1801088541,
    37822859361,
    794280046581,
    16679880978201,
    350277500542221,
    7355827511386641,
];
const_assert!(SMALL_INT_POW21.len() > f64_mantissa_limit(21) as usize);

/// Pre-computed, small powers-of-21.
pub const SMALL_F32_POW21: [f32; 6] = [1.0, 21.0, 441.0, 9261.0, 194481.0, 4084101.0];
const_assert!(SMALL_F32_POW21.len() > f32_exponent_limit(21).1 as usize);

/// Pre-computed, small powers-of-21.
pub const SMALL_F64_POW21: [f64; 13] = [
    1.0,
    21.0,
    441.0,
    9261.0,
    194481.0,
    4084101.0,
    85766121.0,
    1801088541.0,
    37822859361.0,
    794280046581.0,
    16679880978201.0,
    350277500542221.0,
    7355827511386641.0,
];
const_assert!(SMALL_F64_POW21.len() > f64_exponent_limit(21).1 as usize);

/// Pre-computed, small powers-of-22.
pub const SMALL_INT_POW22: [u64; 12] = [
    1,
    22,
    484,
    10648,
    234256,
    5153632,
    113379904,
    2494357888,
    54875873536,
    1207269217792,
    26559922791424,
    584318301411328,
];
const_assert!(SMALL_INT_POW22.len() > f64_mantissa_limit(22) as usize);

/// Pre-computed, small powers-of-22.
pub const SMALL_F32_POW22: [f32; 7] = [1.0, 22.0, 484.0, 10648.0, 234256.0, 5153632.0, 113379904.0];
const_assert!(SMALL_F32_POW22.len() > f32_exponent_limit(22).1 as usize);

/// Pre-computed, small powers-of-22.
pub const SMALL_F64_POW22: [f64; 16] = [
    1.0,
    22.0,
    484.0,
    10648.0,
    234256.0,
    5153632.0,
    113379904.0,
    2494357888.0,
    54875873536.0,
    1207269217792.0,
    26559922791424.0,
    584318301411328.0,
    1.2855002631049216e+16,
    2.8281005788308275e+17,
    6.221821273427821e+18,
    1.3688006801541205e+20,
];
const_assert!(SMALL_F64_POW22.len() > f64_exponent_limit(22).1 as usize);

/// Pre-computed, small powers-of-23.
pub const SMALL_INT_POW23: [u64; 12] = [
    1,
    23,
    529,
    12167,
    279841,
    6436343,
    148035889,
    3404825447,
    78310985281,
    1801152661463,
    41426511213649,
    952809757913927,
];
const_assert!(SMALL_INT_POW23.len() > f64_mantissa_limit(23) as usize);

/// Pre-computed, small powers-of-23.
pub const SMALL_F32_POW23: [f32; 6] = [1.0, 23.0, 529.0, 12167.0, 279841.0, 6436343.0];
const_assert!(SMALL_F32_POW23.len() > f32_exponent_limit(23).1 as usize);

/// Pre-computed, small powers-of-23.
pub const SMALL_F64_POW23: [f64; 12] = [
    1.0,
    23.0,
    529.0,
    12167.0,
    279841.0,
    6436343.0,
    148035889.0,
    3404825447.0,
    78310985281.0,
    1801152661463.0,
    41426511213649.0,
    952809757913927.0,
];
const_assert!(SMALL_F64_POW23.len() > f64_exponent_limit(23).1 as usize);

/// Pre-computed, small powers-of-24.
pub const SMALL_INT_POW24: [u64; 12] = [
    1,
    24,
    576,
    13824,
    331776,
    7962624,
    191102976,
    4586471424,
    110075314176,
    2641807540224,
    63403380965376,
    1521681143169024,
];
const_assert!(SMALL_INT_POW24.len() > f64_mantissa_limit(24) as usize);

/// Pre-computed, small powers-of-24.
pub const SMALL_F32_POW24: [f32; 16] = [
    1.0,
    24.0,
    576.0,
    13824.0,
    331776.0,
    7962624.0,
    191102976.0,
    4586471424.0,
    110075314176.0,
    2641807540224.0,
    63403380965376.0,
    1521681143169024.0,
    3.652034743605658e+16,
    8.764883384653578e+17,
    2.1035720123168588e+19,
    5.048572829560461e+20,
];
const_assert!(SMALL_F32_POW24.len() > f32_exponent_limit(24).1 as usize);

/// Pre-computed, small powers-of-24.
pub const SMALL_F64_POW24: [f64; 34] = [
    1.0,
    24.0,
    576.0,
    13824.0,
    331776.0,
    7962624.0,
    191102976.0,
    4586471424.0,
    110075314176.0,
    2641807540224.0,
    63403380965376.0,
    1521681143169024.0,
    3.652034743605658e+16,
    8.764883384653578e+17,
    2.1035720123168588e+19,
    5.048572829560461e+20,
    1.2116574790945107e+22,
    2.9079779498268256e+23,
    6.979147079584381e+24,
    1.6749952991002515e+26,
    4.0199887178406037e+27,
    9.647972922817449e+28,
    2.3155135014761877e+30,
    5.5572324035428505e+31,
    1.333735776850284e+33,
    3.200965864440682e+34,
    7.682318074657637e+35,
    1.8437563379178328e+37,
    4.425015211002799e+38,
    1.0620036506406717e+40,
    2.548808761537612e+41,
    6.117141027690269e+42,
    1.4681138466456645e+44,
    3.523473231949595e+45,
];
const_assert!(SMALL_F64_POW24.len() > f64_exponent_limit(24).1 as usize);

/// Pre-computed, small powers-of-25.
pub const SMALL_INT_POW25: [u64; 12] = [
    1,
    25,
    625,
    15625,
    390625,
    9765625,
    244140625,
    6103515625,
    152587890625,
    3814697265625,
    95367431640625,
    2384185791015625,
];
const_assert!(SMALL_INT_POW25.len() > f64_mantissa_limit(25) as usize);

/// Pre-computed, small powers-of-25.
pub const SMALL_F32_POW25: [f32; 6] = [1.0, 25.0, 625.0, 15625.0, 390625.0, 9765625.0];
const_assert!(SMALL_F32_POW25.len() > f32_exponent_limit(25).1 as usize);

/// Pre-computed, small powers-of-25.
pub const SMALL_F64_POW25: [f64; 12] = [
    1.0,
    25.0,
    625.0,
    15625.0,
    390625.0,
    9765625.0,
    244140625.0,
    6103515625.0,
    152587890625.0,
    3814697265625.0,
    95367431640625.0,
    2384185791015625.0,
];
const_assert!(SMALL_F64_POW25.len() > f64_exponent_limit(25).1 as usize);

/// Pre-computed, small powers-of-26.
pub const SMALL_INT_POW26: [u64; 12] = [
    1,
    26,
    676,
    17576,
    456976,
    11881376,
    308915776,
    8031810176,
    208827064576,
    5429503678976,
    141167095653376,
    3670344486987776,
];
const_assert!(SMALL_INT_POW26.len() > f64_mantissa_limit(26) as usize);

/// Pre-computed, small powers-of-26.
pub const SMALL_F32_POW26: [f32; 7] =
    [1.0, 26.0, 676.0, 17576.0, 456976.0, 11881376.0, 308915776.0];
const_assert!(SMALL_F32_POW26.len() > f32_exponent_limit(26).1 as usize);

/// Pre-computed, small powers-of-26.
pub const SMALL_F64_POW26: [f64; 15] = [
    1.0,
    26.0,
    676.0,
    17576.0,
    456976.0,
    11881376.0,
    308915776.0,
    8031810176.0,
    208827064576.0,
    5429503678976.0,
    141167095653376.0,
    3670344486987776.0,
    9.542895666168218e+16,
    2.4811528732037366e+18,
    6.450997470329715e+19,
];
const_assert!(SMALL_F64_POW26.len() > f64_exponent_limit(26).1 as usize);

/// Pre-computed, small powers-of-27.
pub const SMALL_INT_POW27: [u64; 12] = [
    1,
    27,
    729,
    19683,
    531441,
    14348907,
    387420489,
    10460353203,
    282429536481,
    7625597484987,
    205891132094649,
    5559060566555523,
];
const_assert!(SMALL_INT_POW27.len() > f64_mantissa_limit(27) as usize);

/// Pre-computed, small powers-of-27.
pub const SMALL_F32_POW27: [f32; 6] = [1.0, 27.0, 729.0, 19683.0, 531441.0, 14348907.0];
const_assert!(SMALL_F32_POW27.len() > f32_exponent_limit(27).1 as usize);

/// Pre-computed, small powers-of-27.
pub const SMALL_F64_POW27: [f64; 12] = [
    1.0,
    27.0,
    729.0,
    19683.0,
    531441.0,
    14348907.0,
    387420489.0,
    10460353203.0,
    282429536481.0,
    7625597484987.0,
    205891132094649.0,
    5559060566555523.0,
];
const_assert!(SMALL_F64_POW27.len() > f64_exponent_limit(27).1 as usize);

/// Pre-computed, small powers-of-28.
pub const SMALL_INT_POW28: [u64; 12] = [
    1,
    28,
    784,
    21952,
    614656,
    17210368,
    481890304,
    13492928512,
    377801998336,
    10578455953408,
    296196766695424,
    8293509467471872,
];
const_assert!(SMALL_INT_POW28.len() > f64_mantissa_limit(28) as usize);

/// Pre-computed, small powers-of-28.
pub const SMALL_F32_POW28: [f32; 9] =
    [1.0, 28.0, 784.0, 21952.0, 614656.0, 17210368.0, 481890304.0, 13492928512.0, 377801998336.0];
const_assert!(SMALL_F32_POW28.len() > f32_exponent_limit(28).1 as usize);

/// Pre-computed, small powers-of-28.
pub const SMALL_F64_POW28: [f64; 19] = [
    1.0,
    28.0,
    784.0,
    21952.0,
    614656.0,
    17210368.0,
    481890304.0,
    13492928512.0,
    377801998336.0,
    10578455953408.0,
    296196766695424.0,
    8293509467471872.0,
    2.322182650892124e+17,
    6.502111422497948e+18,
    1.8205911982994253e+20,
    5.097655355238391e+21,
    1.4273434994667495e+23,
    3.9965617985068985e+24,
    1.1190373035819316e+26,
];
const_assert!(SMALL_F64_POW28.len() > f64_exponent_limit(28).1 as usize);

/// Pre-computed, small powers-of-29.
pub const SMALL_INT_POW29: [u64; 11] = [
    1,
    29,
    841,
    24389,
    707281,
    20511149,
    594823321,
    17249876309,
    500246412961,
    14507145975869,
    420707233300201,
];
const_assert!(SMALL_INT_POW29.len() > f64_mantissa_limit(29) as usize);

/// Pre-computed, small powers-of-29.
pub const SMALL_F32_POW29: [f32; 5] = [1.0, 29.0, 841.0, 24389.0, 707281.0];
const_assert!(SMALL_F32_POW29.len() > f32_exponent_limit(29).1 as usize);

/// Pre-computed, small powers-of-29.
pub const SMALL_F64_POW29: [f64; 11] = [
    1.0,
    29.0,
    841.0,
    24389.0,
    707281.0,
    20511149.0,
    594823321.0,
    17249876309.0,
    500246412961.0,
    14507145975869.0,
    420707233300201.0,
];
const_assert!(SMALL_F64_POW29.len() > f64_exponent_limit(29).1 as usize);

/// Pre-computed, small powers-of-30.
pub const SMALL_INT_POW30: [u64; 11] = [
    1,
    30,
    900,
    27000,
    810000,
    24300000,
    729000000,
    21870000000,
    656100000000,
    19683000000000,
    590490000000000,
];
const_assert!(SMALL_INT_POW30.len() > f64_mantissa_limit(30) as usize);

/// Pre-computed, small powers-of-30.
pub const SMALL_F32_POW30: [f32; 7] =
    [1.0, 30.0, 900.0, 27000.0, 810000.0, 24300000.0, 729000000.0];
const_assert!(SMALL_F32_POW30.len() > f32_exponent_limit(30).1 as usize);

/// Pre-computed, small powers-of-30.
pub const SMALL_F64_POW30: [f64; 14] = [
    1.0,
    30.0,
    900.0,
    27000.0,
    810000.0,
    24300000.0,
    729000000.0,
    21870000000.0,
    656100000000.0,
    19683000000000.0,
    590490000000000.0,
    1.77147e+16,
    5.31441e+17,
    1.594323e+19,
];
const_assert!(SMALL_F64_POW30.len() > f64_exponent_limit(30).1 as usize);

/// Pre-computed, small powers-of-31.
pub const SMALL_INT_POW31: [u64; 11] = [
    1,
    31,
    961,
    29791,
    923521,
    28629151,
    887503681,
    27512614111,
    852891037441,
    26439622160671,
    819628286980801,
];
const_assert!(SMALL_INT_POW31.len() > f64_mantissa_limit(31) as usize);

/// Pre-computed, small powers-of-31.
pub const SMALL_F32_POW31: [f32; 5] = [1.0, 31.0, 961.0, 29791.0, 923521.0];
const_assert!(SMALL_F32_POW31.len() > f32_exponent_limit(31).1 as usize);

/// Pre-computed, small powers-of-31.
pub const SMALL_F64_POW31: [f64; 11] = [
    1.0,
    31.0,
    961.0,
    29791.0,
    923521.0,
    28629151.0,
    887503681.0,
    27512614111.0,
    852891037441.0,
    26439622160671.0,
    819628286980801.0,
];
const_assert!(SMALL_F64_POW31.len() > f64_exponent_limit(31).1 as usize);

/// Pre-computed, small powers-of-33.
pub const SMALL_INT_POW33: [u64; 11] = [
    1,
    33,
    1089,
    35937,
    1185921,
    39135393,
    1291467969,
    42618442977,
    1406408618241,
    46411484401953,
    1531578985264449,
];
const_assert!(SMALL_INT_POW33.len() > f64_mantissa_limit(33) as usize);

/// Pre-computed, small powers-of-33.
pub const SMALL_F32_POW33: [f32; 5] = [1.0, 33.0, 1089.0, 35937.0, 1185921.0];
const_assert!(SMALL_F32_POW33.len() > f32_exponent_limit(33).1 as usize);

/// Pre-computed, small powers-of-33.
pub const SMALL_F64_POW33: [f64; 11] = [
    1.0,
    33.0,
    1089.0,
    35937.0,
    1185921.0,
    39135393.0,
    1291467969.0,
    42618442977.0,
    1406408618241.0,
    46411484401953.0,
    1531578985264449.0,
];
const_assert!(SMALL_F64_POW33.len() > f64_exponent_limit(33).1 as usize);

/// Pre-computed, small powers-of-34.
pub const SMALL_INT_POW34: [u64; 11] = [
    1,
    34,
    1156,
    39304,
    1336336,
    45435424,
    1544804416,
    52523350144,
    1785793904896,
    60716992766464,
    2064377754059776,
];
const_assert!(SMALL_INT_POW34.len() > f64_mantissa_limit(34) as usize);

/// Pre-computed, small powers-of-34.
pub const SMALL_F32_POW34: [f32; 6] = [1.0, 34.0, 1156.0, 39304.0, 1336336.0, 45435424.0];
const_assert!(SMALL_F32_POW34.len() > f32_exponent_limit(34).1 as usize);

/// Pre-computed, small powers-of-34.
pub const SMALL_F64_POW34: [f64; 13] = [
    1.0,
    34.0,
    1156.0,
    39304.0,
    1336336.0,
    45435424.0,
    1544804416.0,
    52523350144.0,
    1785793904896.0,
    60716992766464.0,
    2064377754059776.0,
    7.018884363803238e+16,
    2.386420683693101e+18,
];
const_assert!(SMALL_F64_POW34.len() > f64_exponent_limit(34).1 as usize);

/// Pre-computed, small powers-of-35.
pub const SMALL_INT_POW35: [u64; 11] = [
    1,
    35,
    1225,
    42875,
    1500625,
    52521875,
    1838265625,
    64339296875,
    2251875390625,
    78815638671875,
    2758547353515625,
];
const_assert!(SMALL_INT_POW35.len() > f64_mantissa_limit(35) as usize);

/// Pre-computed, small powers-of-35.
pub const SMALL_F32_POW35: [f32; 5] = [1.0, 35.0, 1225.0, 42875.0, 1500625.0];
const_assert!(SMALL_F32_POW35.len() > f32_exponent_limit(35).1 as usize);

/// Pre-computed, small powers-of-35.
pub const SMALL_F64_POW35: [f64; 11] = [
    1.0,
    35.0,
    1225.0,
    42875.0,
    1500625.0,
    52521875.0,
    1838265625.0,
    64339296875.0,
    2251875390625.0,
    78815638671875.0,
    2758547353515625.0,
];
const_assert!(SMALL_F64_POW35.len() > f64_exponent_limit(35).1 as usize);

/// Pre-computed, small powers-of-36.
pub const SMALL_INT_POW36: [u64; 11] = [
    1,
    36,
    1296,
    46656,
    1679616,
    60466176,
    2176782336,
    78364164096,
    2821109907456,
    101559956668416,
    3656158440062976,
];
const_assert!(SMALL_INT_POW36.len() > f64_mantissa_limit(36) as usize);

/// Pre-computed, small powers-of-36.
pub const SMALL_F32_POW36: [f32; 8] =
    [1.0, 36.0, 1296.0, 46656.0, 1679616.0, 60466176.0, 2176782336.0, 78364164096.0];
const_assert!(SMALL_F32_POW36.len() > f32_exponent_limit(36).1 as usize);

/// Pre-computed, small powers-of-36.
pub const SMALL_F64_POW36: [f64; 17] = [
    1.0,
    36.0,
    1296.0,
    46656.0,
    1679616.0,
    60466176.0,
    2176782336.0,
    78364164096.0,
    2821109907456.0,
    101559956668416.0,
    3656158440062976.0,
    1.3162170384226714e+17,
    4.738381338321617e+18,
    1.705817281795782e+20,
    6.140942214464815e+21,
    2.2107391972073336e+23,
    7.958661109946401e+24,
];
const_assert!(SMALL_F64_POW36.len() > f64_exponent_limit(36).1 as usize);
