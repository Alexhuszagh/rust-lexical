//! Helper utilities for low-level features.
//!
//! Utilities for working with pointers and compiler intrinsics that
//! may not be available  in rust, or in a `no_std` context.

#[cfg(feature = "std")]
use std::{f32, f64};

// GLOBALS

/// Not a Number literal
///
/// To change the expected representation of NaN as a string,
/// change this value during before using lexical.
///
/// Do not modify this value in threaded-code, as it is not thread-safe.
pub static mut NAN_STRING: &str = "NaN";

/// Infinity literal
///
/// To change the expected representation of Infinity as a string,
/// change this value during before using lexical.
pub static mut INFINITY_STRING: &str = "inf";

/// Default character for scientific notation, used when the radix < 15.
///
/// To change the expected, default character for an exponent,
/// change this value during before using lexical.
pub static mut EXPONENT_DEFAULT_CHAR: u8 = b'e';

/// Backup character for scientific notation, used when the radix >= 15.
///
/// For numerical strings of radix >= 15, 'e' or 'E' is a valid digit,
/// and therefore may no longer be used as a marker for the exponent.
///
/// To change the expected, default character for an exponent,
/// change this value during before using lexical.
pub static mut EXPONENT_BACKUP_CHAR: u8 = b'^';

// FLOAT CONSTANTS

// MASKS
// 32-bit
/// Bit-mask for the exponent, including the hidden bit.
pub const F32_EXPONENT_MASK: u32    = 0x7F800000;
/// Bit-mask for the hidden bit in exponent, which is use for the fraction.
pub const F32_HIDDEN_BIT_MASK: u32  = 0x00800000;
/// Bit-mask for the mantissa (fraction), excluding the hidden bit.
pub const F32_FRACTION_MASK: u32    = 0x007FFFFF;
// 64-bit
/// Bit-mask for the exponent, including the hidden bit.
pub const F64_EXPONENT_MASK: u64    = 0x7FF0000000000000;
/// Bit-mask for the hidden bit in exponent, which is use for the fraction.
pub const F64_HIDDEN_BIT_MASK: u64  = 0x0010000000000000;
/// Bit-mask for the mantissa (fraction), excluding the hidden bit.
pub const F64_FRACTION_MASK: u64    = 0x000FFFFFFFFFFFFF;

// PROPERTIES
// 32-bit
pub const U32_INFINITY: u32         = 0x7F800000;
/// Size of the significand (mantissa) without the hidden bit.
pub const F32_SIGNIFICAND_SIZE: i32 = 23;
/// Bias of the exponent.
pub const F32_EXPONENT_BIAS: i32 = 127 + F32_SIGNIFICAND_SIZE;
/// Exponent portion of a denormal float.
pub const F32_DENORMAL_EXPONENT: i32 = -F32_EXPONENT_BIAS + 1;
// 64-bit
/// Positive infinity as bits.
pub const U64_INFINITY: u64         = 0x7FF0000000000000;
/// Size of the significand (mantissa) without the hidden bit.
pub const F64_SIGNIFICAND_SIZE: i32 = 52;
/// Bias of the exponent.
pub const F64_EXPONENT_BIAS: i32 = 1023 + F64_SIGNIFICAND_SIZE;
/// Exponent portion of a denormal float.
pub const F64_DENORMAL_EXPONENT: i32 = -F64_EXPONENT_BIAS + 1;

// CONSTANTS

cfg_if! {
    if #[cfg(feature = "std")] {
        pub(crate) const F32_NAN: f32 = f32::NAN;
        pub(crate) const F32_INFINITY: f32 = f32::INFINITY;
        pub(crate) const F64_NAN: f64 = f64::NAN;
        pub(crate) const F64_INFINITY: f64 = f64::INFINITY;
    } else {
        pub(crate) const F32_NAN: f32 = 0.0_f32 / 0.0_f32;
        pub(crate) const F32_INFINITY: f32 = 1.0_f32 / 0.0_f32;
        pub(crate) const F64_NAN: f64 = 0.0_f64 / 0.0_f64;
        pub(crate) const F64_INFINITY: f64 = 1.0_f64 / 0.0_f64;
    }
}

// INSTRINSICS

cfg_if! {
    if #[cfg(feature = "std")] {
        /// `f64.floor()` feature for `std`
        #[inline(always)]
        pub(crate) fn floor_f64(f: f64) -> f64 {
            f.floor()
        }

        /// `f64.ln()` feature for `std`
        #[inline(always)]
        pub(crate) fn ln_f64(f: f64) -> f64 {
            f.ln()
        }

        /// `f32.powi(i32)` feature for `std`
        #[allow(dead_code)]
        #[inline(always)]
        pub(crate) fn powi_f32(f: f32, i: i32) -> f32 {
            f.powi(i)
        }

        /// `f64.powi(i32)` feature for `std`
        #[allow(dead_code)]
        #[inline(always)]
        pub(crate) fn powi_f64(f: f64, i: i32) -> f64 {
            f.powi(i)
        }
    } else {
        /// `f64.floor()` feature for `no_std`
        #[inline(always)]
        pub(crate) fn floor_f64(f: f64) -> f64 {
            unsafe { core::intrinsics::floorf64(f) }
        }

        /// `f64.ln()` feature for `no_std`
        #[inline(always)]
        pub(crate) fn ln_f64(f: f64) -> f64 {
            unsafe { core::intrinsics::logf64(f) }
        }

        /// `f32.powi(i32)` feature for `no_std`
        #[allow(dead_code)]
        #[inline(always)]
        pub(crate) fn powi_f32(f: f32, i: i32) -> f32 {
            unsafe { core::intrinsics::powif32(f, i) }
        }

        /// `f64.powi(i32)` feature for `no_std`
        #[allow(dead_code)]
        #[inline(always)]
        pub(crate) fn powi_f64(f: f64, i: i32) -> f64 {
            unsafe { core::intrinsics::powif64(f, i) }
        }
    }
}

// MACRO

/// Fast macro absolute value calculator.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate lexical;
/// # pub main() {
/// }
/// ```
#[cfg(not(any(feature = "grisu3", feature = "ryu")))]
macro_rules! abs {
    ($n:expr) => ({
        let n = $n;
        if n < 0 { -n } else { n }
    })
}

/// Fast macro maximum value calculator.
macro_rules! max {
    ($a:expr, $b:expr) => ({
        let a = $a;
        let b = $b;
        if a > b { a } else { b }
    })
}

/// Fast macro minimum value calculator.
macro_rules! min {
    ($a:expr, $b:expr) => ({
        let a = $a;
        let b = $b;
        if a < b { a } else { b }
    })
}

/// Get a literal nullptr.
macro_rules! nullptr {
    () => ($crate::sealed::ptr::null());
}

/// Mark uninitialized memory.
macro_rules! uninitialized {
    () => ($crate::sealed::mem::uninitialized());
}

/// Copy non-overlapping (memcpy, with arguments reversed).
macro_rules! copy_nonoverlapping {
    ($src:expr, $dst:expr, $size:expr) => (
        $crate::sealed::ptr::copy_nonoverlapping($src, $dst, $size)
    );
}

/// Write byte to range (memset).
#[allow(unused_macros)]
macro_rules! write_bytes {
    ($dst:expr, $byte:expr, $size:expr) => (
        $crate::sealed::ptr::write_bytes($dst, $byte, $size)
    );
}

// STABLE POWI

/// Macro to generate stable_powi_normal for f32 and f64.
macro_rules! stable_powi_normal {
    ($value:ident, $base:ident, $exponent:ident, $step:ident, $powi:ident) => ({
        if $exponent < 0 {
            // negative exponent, use division for numeric stability
            while $exponent <= -$step {
                $exponent += $step;
                $value /= $powi($base, $step)
            }
            if $exponent != 0 {
                $value /= $powi($base, -$exponent)
            }
            $value
        } else {
            // positive exponent
            while $exponent >= $step {
                $exponent -= $step;
                $value *= $powi($base, $step)
            }
            if $exponent != 0 {
                $value *= $powi($base, $exponent)
            }
            $value
        }
    });
}

/// Macro to generate stable_powi for f32 and f64.
macro_rules! stable_powi {
    ($value:ident, $base:ident, $exponent:ident, $maxexp:ident, $inf:ident, $cb:ident) => ({
        if $exponent > $maxexp {
            // Value is impossibly large, must be infinity.
            $inf
        } else if $exponent < -$maxexp {
            // Value is impossibly small, must be 0.
            0.0
        } else {
            $cb($value, $base, $exponent)
        }
    });
}

// STABLE POWI F32

/// Cached powers to get the desired exponent.
/// Make sure all values are < 1e25.
const F32_POWI_EXPONENT_STEP: [i32; 35] = [
    90, 60, 50, 40, 40, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 20, 20, 20, 20, 20, 20, 20, 20,
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20
];

/// Simplify base to powi to avoid bugs.
#[inline(always)]
fn f32_powi_step(base: u64) -> i32 {
    unsafe { *F32_POWI_EXPONENT_STEP.get_unchecked(base as usize - 2) }
}

/// Cached max exponents.
/// Make sure the value is >= 2*log(1e45, base), which guarantees the
/// value overflows or underflows.
const F32_MAX_EXPONENT: [i32; 35] = [
    150, 100, 80, 70, 60, 60, 50, 50, 50, 50, 50, 50,
    40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
    40, 40, 40, 40, 40, 40, 30, 30, 30, 30, 30
];

/// Get f32 maximum exponent from base.
#[inline(always)]
pub fn f32_maxexp(base: u64) -> i32 {
    unsafe { *F32_MAX_EXPONENT.get_unchecked(base as usize - 2) }
}

/// `powi_f32` implementation that is more stable at extremely low powers.
///
/// Exponent must be non-special to get here.
///
/// Roughly equivalent to `value * powi_f32(base, exponent)`
#[inline]
pub fn stable_powi_normal_f32(mut value: f32, base: u64, mut exponent: i32) -> f32 {
    let step = f32_powi_step(base);
    let base = base as f32;
    stable_powi_normal!(value, base, exponent, step, powi_f32)
}

/// `powi_f32` implementation that is more stable at extremely low powers.
///
/// The exponent must be non-zero.
///
/// Roughly equivalent to `value * powi_f32(base, exponent)`
#[inline]
#[allow(dead_code)]
pub fn stable_powi_f32(value: f32, base: u64, exponent: i32) -> f32 {
    let maxexp = f32_maxexp(base);
    stable_powi!(value, base, exponent, maxexp, F32_INFINITY, stable_powi_normal_f32)
}

// STABLE POWI F64

/// Cached powers to get the desired exponent.
/// Make sure all values are < 1e300.
const F64_POWI_EXPONENT_STEP: [i32; 35] = [
    512, 512, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
    256, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128
];

/// Simplify base to powi to avoid bugs.
#[inline(always)]
fn f64_powi_step(base: u64) -> i32 {
    unsafe { *F64_POWI_EXPONENT_STEP.get_unchecked(base as usize - 2) }
}

/// Cached max exponents.
/// Make sure the value is >= 2*log(5e324, base), which guarantees the
/// value overflows or underflows.
const F64_MAX_EXPONENT: [i32; 35] = [
    2200, 1400, 1200, 1000, 900, 800, 750, 700, 650, 625, 625, 600,
    575, 575, 550, 550, 525, 525, 500, 500, 500, 500, 475, 475,
    475, 475, 450, 450, 450, 450, 450, 450, 425, 425, 425
];

/// Get f64 maximum exponent from base.
#[inline(always)]
pub fn f64_maxexp(base: u64) -> i32 {
    unsafe { *F64_MAX_EXPONENT.get_unchecked(base as usize - 2) }
}

/// `powi_f64` implementation that is more stable at extremely low powers.
///
/// Exponent must be non-special to get here.
///
/// Roughly equivalent to `value * powi_f64(base, exponent)`
#[inline]
pub fn stable_powi_normal_f64(mut value: f64, base: u64, mut exponent: i32) -> f64 {
    let step = f64_powi_step(base);
    let base = base as f64;
    stable_powi_normal!(value, base, exponent, step, powi_f64)
}

/// `powi_f64` implementation that is more stable at extremely low powers.
///
/// The exponent must be non-zero.
///
/// Roughly equivalent to `value * powi_f64(base, exponent)`
#[inline]
#[allow(dead_code)]
pub fn stable_powi_f64(value: f64, base: u64, exponent: i32) -> f64 {
    let maxexp = f64_maxexp(base);
    stable_powi!(value, base, exponent, maxexp, F64_INFINITY, stable_powi_normal_f64)
}

// POWN POWI

/// Calculate a stable powi when the value is known to be >= -2*max && <= 2*max
///
/// powi is not stable, even with exact values, at high or low exponents.
/// However, doing it in 2 shots for exact values is exact.
#[cfg(all(any(test, feature = "correct"), not(feature = "table")))]
macro_rules! stable_pow2 {
    ($exponent:ident, $max:expr, $powi:ident) => ({
        if $exponent > $max {
            $powi(2.0, $max) * $powi(2.0, $exponent - $max)
        } else if $exponent < -$max {
            $powi(2.0, -$max) * $powi(2.0, $exponent + $max)
        } else {
            $powi(2.0, $exponent)
        }
    })
}

/// Calculate power of 2 using powi.
#[cfg(all(any(test, feature = "correct"), not(feature = "table")))]
#[inline]
pub unsafe fn pow2_f32(value: f32, exponent: i32) -> f32 {
    value * stable_pow2!(exponent, 75, powi_f32)
}

/// Calculate power of n using powi.
#[cfg(all(any(test, feature = "correct"), not(feature = "table")))]
#[inline]
pub unsafe fn pown_f32(value: f32, base: u64, exponent: i32) -> f32 {
    // Check the exponent is within bounds in debug builds.
    let (min, max) = f64_exact_exponent_limit!(base);
    debug_assert!(exponent >= min && exponent <= max);

    value * powi_f32(base as f32, exponent)
}

/// Calculate power of 2 using powi.
#[cfg(all(any(test, feature = "correct"), not(feature = "table")))]
#[inline]
pub unsafe fn pow2_f64(value: f64, exponent: i32) -> f64 {
    value * stable_pow2!(exponent, 75, powi_f64)
}

/// Calculate power of n using powi.
#[cfg(all(any(test, feature = "correct"), not(feature = "table")))]
#[inline]
pub unsafe fn pown_f64(value: f64, base: u64, exponent: i32) -> f64 {
    // Check the exponent is within bounds in debug builds.
    let (min, max) = f64_exact_exponent_limit!(base);
    debug_assert!(exponent >= min && exponent <= max);

    value * powi_f64(base as f64, exponent)
}

// POWN TABLE

/// Calculate power of 2 using precalculated table.
#[cfg(all(any(test, feature = "correct"), feature = "table"))]
#[inline]
pub unsafe fn pow2_f32(value: f32, exponent: i32) -> f32 {
    value * f32_pow2!(exponent)
}

/// Calculate power of n using precalculated table.
#[cfg(all(any(test, feature = "correct"), feature = "table"))]
#[inline]
pub unsafe fn pown_f32(value: f32, base: u64, exponent: i32) -> f32 {
    // Check the exponent is within bounds in debug builds.
    let (min, max) = f64_exact_exponent_limit!(base);
    debug_assert!(exponent >= min && exponent <= max);

    if exponent > 0 {
        value * f32_pown!(base, exponent)
    } else {
        value / f32_pown!(base, -exponent)
    }
}

/// Calculate power of 2 using precalculated table.
#[cfg(all(any(test, feature = "correct"), feature = "table"))]
#[inline]
pub unsafe fn pow2_f64(value: f64, exponent: i32) -> f64 {
    value * f64_pow2!(exponent)
}

/// Calculate power of n using precalculated table.
#[cfg(all(any(test, feature = "correct"), feature = "table"))]
#[inline]
pub unsafe fn pown_f64(value: f64, base: u64, exponent: i32) -> f64 {
    // Check the exponent is within bounds in debug builds.
    let (min, max) = f64_exact_exponent_limit!(base);
    debug_assert!(exponent >= min && exponent <= max);

    if exponent > 0 {
        value * f64_pown!(base, exponent)
    } else {
        value / f64_pown!(base, -exponent)
    }
}

// ALGORITHMS

/// Reverse a range of pointers.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn reverse(first: *mut u8, last: *mut u8) {
    let mut f = first;
    let mut l = last;
    let mut x: u8;
    let mut li = l.sub(1);

    while f != l && f != li {
        l = li;
        x = *f;
        *f = *l;
        *l = x;
        li = l.sub(1);
        f = f.add(1);
    }
}

/// Calculate the difference between two pointers.
#[inline(always)]
pub(crate) unsafe extern "C" fn distance(first: *const u8, last: *const u8)
    -> usize
{
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

extern {
    /// Need memcmp for efficient range comparisons.
    fn memcmp(l: *const u8, r: *const u8, n: usize) -> i32;
}

/// Check if two ranges are equal to each other.
#[inline(always)]
pub(crate) unsafe extern "C" fn equal_to(l: *const u8, r: *const u8, n: usize)
    -> bool
{
    memcmp(l, r, n) == 0
}

/// Check if left range starts with right range.
#[inline(always)]
pub(crate) unsafe extern "C" fn starts_with(l: *const u8, ln: usize, r: *const u8, rn: usize)
    -> bool
{
    ln >= rn && equal_to(l, r, rn)
}

/// Check if left range ends with right range.
#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe extern "C" fn ends_with(l: *const u8, ln: usize, r: *const u8, rn: usize)
    -> bool
{
    ln >= rn && equal_to(l.add(ln - rn), r, rn)
}

/// Trim character from the left-side of a range.
///
/// Returns a pointer to the new start of the range.
#[inline(always)]
pub(crate) unsafe extern "C" fn ltrim_char(mut first: *const u8, last: *const u8, char: u8)
    -> *const u8
{
    while first < last && *first == char {
        first = first.add(1);
    }
    first
}

// LOW LEVEL WRAPPERS

/// Generate the low-level bytes API.
///
/// Wraps unsafe functions to generate the low-level, unchecked, bytes parsers.
#[doc(hidden)]
macro_rules! bytes_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level bytes to number parser.
        #[inline]
        pub fn $func(bytes: &[u8], base: u8)
            -> $t
        {
            unsafe {
                let first = bytes.as_ptr();
                let last = first.add(bytes.len());
                let (value, _, _) = $callback(first, last, base);
                value
            }
        }
    )
}

/// Error-checking version of `bytes_impl`.
///
/// Wraps unsafe functions to generate the low-level, checked, bytes parsers.
#[doc(hidden)]
macro_rules! try_bytes_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level bytes to number parser.
        /// On error, returns position of invalid char.
        #[inline]
        pub fn $func(bytes: &[u8], base: u8)
            -> Result<$t, $crate::Error>
        {
            unsafe {
                let first = bytes.as_ptr();
                let last = first.add(bytes.len());
                let (value, p, overflow) = $callback(first, last, base);
                if overflow {
                    Err(From::from($crate::ErrorKind::Overflow))
                } else if p == last {
                    Ok(value)
                } else {
                    let dist = if p == nullptr!() { 0 } else { distance(first, p) };
                    Err(From::from($crate::ErrorKind::InvalidDigit(dist)))
                }
            }
        }
    )
}

/// Generate the low-level string API using wrappers around the unsafe function.
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! string_impl {
    ($func:ident, $t:ty, $callback:ident, $capacity:expr) => (
        /// Low-level string exporter for numbers.
        #[inline]
        pub fn $func(value: $t, base: u8)
            -> ::sealed::String
        {
            let mut string = ::sealed::String::with_capacity($capacity);
            unsafe {
                let buf = string.as_mut_vec();
                let first: *mut u8 = buf.as_mut_ptr();
                let last = first.add(buf.capacity());
                let end = $callback(value, first, last, base);
                let size = distance(first, end);
                buf.set_len(size);

            }
            string
        }
    )
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_powi_normal_f32_test() {
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, 38), 1e38, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, 30), 1e30, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, 25), 1e25, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, 20), 1e20, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, 15), 1e15, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, 10), 1e10, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, 5), 1e5, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, -5), 1e-5, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, -10), 1e-10, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, -15), 1e-15, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, -20), 1e-20, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, -25), 1e-25, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, -30), 1e-30, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, -38), 1e-38, max_relative=1e-6);
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, -45), 1e-45, max_relative=1e-6);

        // overflow
        assert!(stable_powi_normal_f32(1.0, 10, 39).is_infinite());

        // underflow
        assert_eq!(stable_powi_normal_f32(1.0, 10, -46), 0.0);
    }

    #[test]
    fn stable_powi_f32_test() {
        assert_relative_eq!(stable_powi_normal_f32(1.0, 10, 10), 1e10, max_relative=1e-15);
        assert!(stable_powi_normal_f32(1.0, 10, 1000).is_infinite());
        assert_eq!(stable_powi_normal_f32(1.0, 10, -1000), 0.0);

        // overflow
        assert!(stable_powi_f32(1.0, 10, 39).is_infinite());

        // underflow
        assert_eq!(stable_powi_f32(1.0, 10, -46), 0.0);
    }

    #[test]
    fn stable_powi_normal_f64_test() {
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, 308), 1e308, max_relative=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, 300), 1e300, max_relative=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, 200), 1e200, max_relative=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, 100), 1e100, max_relative=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, 50), 1e50, max_relative=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, -50), 1e-50, epsilon=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, -100), 1e-100, epsilon=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, -200), 1e-200, epsilon=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, -300), 1e-300, epsilon=1e-15);
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, -308), 1e-308, epsilon=1e-15);
        assert_eq!(stable_powi_normal_f64(5.0, 10, -324), 5e-324);

        // overflow
        assert!(stable_powi_normal_f64(1.0, 10, 309).is_infinite());

        // underflow
        assert_eq!(stable_powi_normal_f64(1.0, 10, -325), 0.0);
    }

    #[test]
    fn stable_powi_f64_test() {
        assert_relative_eq!(stable_powi_normal_f64(1.0, 10, 50), 1e50, max_relative=1e-15);
        assert!(stable_powi_normal_f64(1.0, 10, 1000).is_infinite());
        assert_eq!(stable_powi_normal_f64(1.0, 10, -1000), 0.0);

        // overflow
        assert!(stable_powi_f64(1.0, 10, 309).is_infinite());

        // underflow
        assert_eq!(stable_powi_f64(1.0, 10, -325), 0.0);
    }

    const BASEN: [u64; 30] = [
        3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
        22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
    ];

    #[test]
    fn pow2_f32_test() {
        unsafe {
            let (min, max) = f32_exact_exponent_limit!(2);
            for i in min+1..max+1 {
                assert_eq!(pow2_f32(1.0, i)/pow2_f32(1.0, i-1), 2.0);
            }
            for i in 1..max+1 {
                let f = pow2_f32(1.0, i);
                if f < u64::max_value() as f32 {
                    assert_eq!((f as u64) as f32, f);
                }
            }
        }
    }

    #[test]
    fn pown_f32_test() {
        unsafe {
            // Only check positive, since negative values round during division.
            for b in BASEN.iter().cloned() {
                let (_, max) = f32_exact_exponent_limit!(b);
                for i in 1..max+1 {
                    let f = pown_f32(1.0, b, i);
                    let p = pown_f32(1.0, b, i-1);
                    assert_eq!(f / p, b as f32);
                    if f < u64::max_value() as f32 {
                        assert_eq!((f as u64) as f32, f);
                    }
                }
            }
        }
    }

    #[test]
    fn pow2_f64_test() {
        unsafe {
            let (min, max) = f64_exact_exponent_limit!(2);
            for i in min+1..max+1 {
                let curr = pow2_f64(1.0, i);
                let prev = pow2_f64(1.0, i-1);
                assert_eq!(curr / prev, 2.0);
            }
            for i in 1..max+1 {
                let f = pow2_f64(1.0, i);
                if f < u64::max_value() as f64 {
                    assert_eq!((f as u64) as f64, f);
                }
            }
        }
    }

    #[test]
    fn pown_f64_test() {
        unsafe {
            // Only check positive, since negative values round during division.
            for b in BASEN.iter().cloned() {
                let (_, max) = f64_exact_exponent_limit!(b);
                for i in 1..max+1 {
                    let f = pown_f64(1.0, b, i);
                    let p = pown_f64(1.0, b, i-1);
                    assert_eq!(f / p, b as f64);
                    if f < u64::max_value() as f64 {
                        assert_eq!((f as u64) as f64, f);
                    }
                }
            }
        }
    }

    #[test]
    fn reverse_test() {
        unsafe {
            let mut x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let y: [u8; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
            let first: *mut u8 = x.as_mut_ptr();
            let last = first.add(x.len());
            reverse(first, last);
            assert_eq!(x, y);
        }
    }

    #[test]
    fn distance_test() {
        unsafe {
            let x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            let first: *const u8 = x.as_ptr();
            let last = first.add(x.len());
            assert_eq!(distance(first, last), 10);
        }
    }

    #[test]
    fn equal_to_test() {
        unsafe {
            let x = "Hello";
            let y = "Hello";
            let z = "hello";
            assert!(equal_to(x.as_ptr(), y.as_ptr(), x.len()));
            assert!(!equal_to(x.as_ptr(), z.as_ptr(), x.len()));
            assert!(!equal_to(y.as_ptr(), z.as_ptr(), x.len()));
        }
    }

    #[test]
    fn starts_with_test() {
        unsafe {
            let x = "Hello";
            let y = "H";
            let z = "h";

            // forward
            assert!(starts_with(x.as_ptr(), x.len(), y.as_ptr(), y.len()));
            assert!(!starts_with(x.as_ptr(), x.len(), z.as_ptr(), z.len()));
            assert!(!starts_with(y.as_ptr(), y.len(), z.as_ptr(), z.len()));

            // back
            assert!(!starts_with(y.as_ptr(), y.len(), x.as_ptr(), x.len()));
            assert!(!starts_with(z.as_ptr(), z.len(), x.as_ptr(), x.len()));
        }
    }

    #[test]
    fn ends_with_test() {
        unsafe {
            let w = "Hello";
            let x = "lO";
            let y = "lo";
            let z = "o";

            // forward
            assert!(!ends_with(w.as_ptr(), w.len(), x.as_ptr(), x.len()));
            assert!(ends_with(w.as_ptr(), w.len(), y.as_ptr(), y.len()));
            assert!(ends_with(w.as_ptr(), w.len(), z.as_ptr(), z.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), y.as_ptr(), y.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), z.as_ptr(), z.len()));
            assert!(ends_with(y.as_ptr(), y.len(), z.as_ptr(), z.len()));

            // back
            assert!(!ends_with(z.as_ptr(), z.len(), y.as_ptr(), y.len()));
            assert!(!ends_with(z.as_ptr(), z.len(), x.as_ptr(), x.len()));
            assert!(!ends_with(z.as_ptr(), z.len(), w.as_ptr(), w.len()));
            assert!(!ends_with(y.as_ptr(), y.len(), x.as_ptr(), x.len()));
            assert!(!ends_with(y.as_ptr(), y.len(), w.as_ptr(), w.len()));
            assert!(!ends_with(x.as_ptr(), x.len(), w.as_ptr(), w.len()));
        }
    }

    #[test]
    fn ltrim_char_test() {
        unsafe {
            let w = "0001";
            let x = "1010";
            let y = "1.00";
            let z = "1e05";

            let ltrim_char_wrapper = |w: &str, c: u8| {
                let first = w.as_ptr();
                let last = first.add(w.len());
                distance(first, ltrim_char(first, last, c))
            };

            assert_eq!(ltrim_char_wrapper(w, b'0'), 3);
            assert_eq!(ltrim_char_wrapper(x, b'0'), 0);
            assert_eq!(ltrim_char_wrapper(x, b'1'), 1);
            assert_eq!(ltrim_char_wrapper(y, b'0'), 0);
            assert_eq!(ltrim_char_wrapper(y, b'1'), 1);
            assert_eq!(ltrim_char_wrapper(z, b'0'), 0);
            assert_eq!(ltrim_char_wrapper(z, b'1'), 1);
        }
    }

    cfg_if! {
        if #[cfg(feature = "std")] {
            use super::super::atof::*;
            use super::super::ftoa::*;

            // Only enable when no other threads touch NAN_STRING or INFINITY_STRING.
            #[test]
            #[ignore]
            fn special_string_test() {
                // Test serializing and deserializing special strings.
                assert!(atof32_bytes(b"NaN", 10).is_nan());
                assert!(atof32_bytes(b"inf", 10).is_infinite());
                assert!(!atof32_bytes(b"nan", 10).is_nan());
                assert!(!atof32_bytes(b"Infinity", 10).is_infinite());
                assert_eq!(&f64toa_string(F64_NAN, 10), "NaN");
                assert_eq!(&f64toa_string(F64_INFINITY, 10), "inf");

                unsafe {
                    NAN_STRING = "nan";
                    INFINITY_STRING = "Infinity";
                }

                assert!(!atof32_bytes(b"NaN", 10).is_nan());
                assert!(!atof32_bytes(b"inf", 10).is_infinite());
                assert!(atof32_bytes(b"nan", 10).is_nan());
                assert!(atof32_bytes(b"Infinity", 10).is_infinite());
                assert_eq!(&f64toa_string(F64_NAN, 10), "nan");
                assert_eq!(&f64toa_string(F64_INFINITY, 10), "Infinity");

                unsafe {
                    NAN_STRING = "NaN";
                    INFINITY_STRING = "inf";
                }
            }
        }
    }
}
