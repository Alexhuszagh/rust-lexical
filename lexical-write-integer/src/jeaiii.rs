//! Optimized integer-to-string conversion routines for decimal values.

//! This algorihm is described in [`Faster Integer Formatting`], which uses
//! binary search trees for highly optimized digit writing. For large numbers,
//! the increased branching can destroy performance, but for 32-bit or smaller
//! integers it is always faster and can be optimized in 64-bit cases.
//!
//! This is based off of the work by James Anhalt (jeaiii) and Junekey Jeon
//! (jk-jeon). This has a few advantages, one is that indexing can be done
//! without bounds checking, without any major performance hits, which minimizes
//! the unchecked indexing and therefore potential unsoundness.
//!
//! This has some additional changes for performance enhancements, most notably,
//! it flattens out most of the comparisons and uses larger first, which
//! paradoxically seems to improve performance, potentially due to less
//! branching.
//!
//! See [Algorithm.md](/docs/Algorithm.md) for a more detailed description of
//! the algorithm choice here. See [Benchmarks.md](/docs/Benchmarks.md) for
//! recent benchmark data.
//!
//! [`Faster Integer Formatting`]: https://jk-jeon.github.io/posts/2022/02/jeaiii-algorithm/

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use lexical_util::digit::digit_to_char_const;
use lexical_util::div128::fast_u128_divrem;

use crate::table::DIGIT_TO_BASE10_SQUARED;

// Mask to extract the lower half.
const LO32: u64 = u32::MAX as u64;

/// Get the next 2 digits from the input.
#[inline(always)]
fn next2(prod: &mut u64) -> u32 {
    *prod = (*prod & LO32) * 100;
    (*prod >> 32) as u32
}

/// Quickly calculate `n / 1e10` and `n % 1e10`.
#[inline(always)]
fn u128_divrem_10_10pow10(n: u128) -> (u128, u64) {
    fast_u128_divrem(
        n,
        10000000000,
        18889465931478580854784,
        10,
        73075081866545145910184241635814150983,
        31,
    )
}

/// Quickly calculate `n / 1e10` and `n % 1e10`.
///
/// We use this for quickly breaking our integer into
/// chunks of 10 digits for fast u128 formatting.
#[inline(always)]
fn div128_rem_1e10(n: u128) -> (u128, u64) {
    u128_divrem_10_10pow10(n)
}

// Index a value from a buffer without bounds checking.
macro_rules! i {
    ($array:ident[$index:expr]) => {
        // SAFETY: Safe if `array.len() > index`.
        unsafe { *$array.get_unchecked($index) }
    };
}

// Write N digits to our buffer.
macro_rules! write_n {
    (@1 $buffer:ident, $index:expr, $n:expr) => {{
        let index = $index;
        let digit = digit_to_char_const($n as u32, 10);
        $buffer[index] = digit;
        index + 1
    }};

    (@2 $buffer:ident, $index:expr, $r:expr) => {{
        let index = $index;
        let r = $r as usize;
        // NOTE: This always should be true due to how we calculate our bounds.
        // `r` is always a single digit, so `2 * r` must be smaller than our
        // square table.
        debug_assert!(r < DIGIT_TO_BASE10_SQUARED.len());
        $buffer[index] = i!(DIGIT_TO_BASE10_SQUARED[r]);
        $buffer[index + 1] = i!(DIGIT_TO_BASE10_SQUARED[r + 1]);
        index + 2
    }};

    // Identical to `@2` except it's writing from the end, not front.
    // This is for our Alexandrescu-popularized algorithm.
    (@2sub $buffer:ident, $index:ident, $r:expr) => {{
        $index -= 2;
        _ = write_n!(@2 $buffer, $index, $r);
    }};

    // This writes 4 digits, using 2sub twice after getting the high and low.
    (@4sub $buffer:ident, $index:ident, $value:ident) => {{
        let r = $value % 10000;
        $value /= 10000;
        let r1 = 2 * (r / 100);
        let r2 = 2 * (r % 100);
        write_n!(@2sub $buffer, $index, r2);
        write_n!(@2sub $buffer, $index, r1);
    }};
}

// Print the next 2 digits, using `next2`.
macro_rules! print_n {
    (@2 $buffer:ident, $index:ident, $prod:ident) => {
        $index = write_n!(@2 $buffer, $index, next2(&mut $prod) * 2);
    };

    (@n $buffer:ident, $index:ident, $n:ident, $magic:expr, $shift:expr, $remaining:expr) => {{
        let mut prod = ($n as u64) * $magic;
        prod >>= $shift;
        let two = (prod >> 32) as u32;
        if two < 10 {
            $index = write_n!(@1 $buffer, $index, two);
            for _ in 0..$remaining {
                print_n!(@2 $buffer, $index, prod);
            }
        } else {
            $index = write_n!(@2 $buffer, $index, two * 2);
            for _ in 0..$remaining {
                print_n!(@2 $buffer, $index, prod);
            }
        }
        $index
    }};
}

// Optimized digit writers for the number of digits for each.
// This avoids code duplication while keeping our flat logic.
macro_rules! write_digits {
    (@1 $buffer:ident, $n:ident) => {
        write_n!(@1 $buffer, 0, $n)
    };

    (@2 $buffer:ident, $n:ident) => {
        write_n!(@2 $buffer, 0, $n * 2)
    };

    // NOTE: This is only used for u8
    (@3 $buffer:ident, $n:ident) => {{
        // `42949673 = ceil(2^32 / 10^2)`
        let mut y = $n as u64 * 42949673u64;
        _ = write_n!(@1 $buffer, 0, y >> 32);
        write_n!(@2 $buffer, 1, next2(&mut y) * 2)
    }};

    (@3-4 $buffer:ident, $n:ident) => {{
        // `42949673 = ceil(2^32 / 10^2)`
        let mut index = 0;
        print_n!(@n $buffer, index, $n, 42949673u64, 0, 1)
    }};

    (@5 $buffer:ident, $n:ident) => {{
        // `429497 == ceil(2^32 / 10^4)`
        let mut y = $n as u64 * 429497u64;
        _ = write_n!(@1 $buffer, 0, y >> 32);
        _ = write_n!(@2 $buffer, 1, next2(&mut y) * 2);
        write_n!(@2 $buffer, 3, next2(&mut y) * 2)
    }};

    (@5-6 $buffer:ident, $n:ident) => {{
        // `429497 == ceil(2^32 / 10^4)`
        let mut index = 0;
        print_n!(@n $buffer, index, $n, 429497u64, 0, 2)
    }};

    (@7-8 $buffer:ident, $n:ident) => {{
        // `281474978 == ceil(2^48 / 10^6) + 1`
        let mut index = 0;
        print_n!(@n $buffer, index, $n, 281474978u64, 16, 3)
    }};

    (@9 $buffer:ident, $n:ident) => {{
        // 1441151882 = ceil(2^57 / 10^8) + 1
        let mut y = ($n as u64) * 1441151882u64;
        y >>= 25;
        _ = write_n!(@1 $buffer, 0, y >> 32);
        _ = write_n!(@2 $buffer, 1, next2(&mut y) * 2);
        _ = write_n!(@2 $buffer, 3, next2(&mut y) * 2);
        _ = write_n!(@2 $buffer, 5, next2(&mut y) * 2);
        write_n!(@2 $buffer, 7, next2(&mut y) * 2)
    }};

    (@10 $buffer:ident, $n:ident) => {{
        // `1441151881 = ceil(2^57 / 10^8)`
        let mut y = ($n as u64) * 1441151881u64;
        y >>= 25;
        _ = write_n!(@2 $buffer, 0, (y >> 32) * 2);
        _ = write_n!(@2 $buffer, 2, next2(&mut y) * 2);
        _ = write_n!(@2 $buffer, 4, next2(&mut y) * 2);
        _ = write_n!(@2 $buffer, 6, next2(&mut y) * 2);
        write_n!(@2 $buffer, 8, next2(&mut y) * 2)
    }};

    (@10u64 $buffer:ident, $n:ident) => {{
        // Unfortunately, there is no good way without using 128 bits,
        // since the smallest interval overflows a 64-bit integer at
        // ~>= 5.5e9. This requires the value to be in `[1e9, 1e10)`,
        // since there's no lower bound for the calculation and so it
        // will not work with smaller values.
        // D = 32, k = 8, L = 28
        // `11529215047 = ceil(2^60 / 10^8)`
        let prod = ($n as u128) * 11529215047u128;
        let mut y = (prod >> 28) as u64;
        _ = write_n!(@2 $buffer, 0, (y >> 32) * 2);
        _ = write_n!(@2 $buffer, 2, next2(&mut y) * 2);
        _ = write_n!(@2 $buffer, 4, next2(&mut y) * 2);
        _ = write_n!(@2 $buffer, 6, next2(&mut y) * 2);
        write_n!(@2 $buffer, 8, next2(&mut y) * 2)
    }};

    (@10alex $buffer:ident, $n:ident, $offset:ident) => {{
        // This always writes 10 digits for any value `[0, 1e10)`,
        // but it uses a slower algorithm to do so. Since we don't
        // have to worry about
        let mut value = $n;
        let mut index = 10 + $offset;
        write_n!(@4sub $buffer, index, value);
        write_n!(@4sub $buffer, index, value);
        write_n!(@2sub $buffer, index, value * 2);
        10 + $offset
    }};
}

/// Optimized jeaiii algorithm for u8.
#[inline(always)]
pub fn from_u8(n: u8, buffer: &mut [u8]) -> usize {
    // NOTE: For some reason, doing the large comparisons **FIRST**
    // seems to be faster than the inverse, for both large and small
    // values, which seems to make little sense. But, the benchmarks
    // tell us reality.
    let buffer = &mut buffer[..3];
    if n >= 100 {
        write_digits!(@3 buffer, n)
    } else if n >= 10 {
        write_digits!(@2 buffer, n)
    } else {
        write_digits!(@1 buffer, n)
    }
}

/// Optimized jeaiii algorithm for u16.
#[inline(always)]
pub fn from_u16(n: u16, buffer: &mut [u8]) -> usize {
    // NOTE: Like before, this optimizes better for large and small
    // values if there's a flat comparison with larger values first.
    let buffer = &mut buffer[..5];
    if n >= 1_0000 {
        write_digits!(@5 buffer, n)
    } else if n >= 100 {
        write_digits!(@3-4 buffer, n)
    } else if n >= 10 {
        write_digits!(@2 buffer, n)
    } else {
        write_digits!(@1 buffer, n)
    }
}

/// Optimized jeaiii algorithm for u32.
#[inline(always)]
#[allow(clippy::collapsible_else_if)] // reason = "branching is fine-tuned for performance"
pub fn from_u32(n: u32, buffer: &mut [u8]) -> usize {
    // NOTE: Like before, this optimizes better for large and small
    // values if there's a flat comparison with larger values first.
    let buffer = &mut buffer[..10];
    if n < 1_0000 {
        if n >= 100 {
            write_digits!(@3-4 buffer, n)
        } else if n >= 10 {
            write_digits!(@2 buffer, n)
        } else {
            write_digits!(@1 buffer, n)
        }
    } else if n < 1_0000_0000 {
        if n >= 100_0000 {
            write_digits!(@7-8 buffer, n)
        } else {
            write_digits!(@5-6 buffer, n)
        }
    } else {
        if n >= 10_0000_0000 {
            write_digits!(@10 buffer, n)
        } else {
            write_digits!(@9 buffer, n)
        }
    }
}

/// Optimized jeaiii algorithm for u64.
#[inline(always)]
#[allow(clippy::collapsible_else_if)] // reason = "branching is fine-tuned for performance"
fn from_u64_impl(n: u64, buffer: &mut [u8], is_signed: bool) -> usize {
    // NOTE: Like before, this optimizes better for large and small
    // values if there's a flat comparison with larger values first.
    const FACTOR: u64 = 100_0000_0000;
    // NOTE `i64` takes a max of 19 digits, while `u64` takes a max of 20.
    let buffer = if is_signed {
        &mut buffer[..19]
    } else {
        &mut buffer[..20]
    };
    if n < 1_0000 {
        // 1 to 4 digits
        if n >= 100 {
            write_digits!(@3-4 buffer, n)
        } else if n >= 10 {
            write_digits!(@2 buffer, n)
        } else {
            write_digits!(@1 buffer, n)
        }
    } else if n < FACTOR {
        // 5 to 10 digits
        if n >= 10_0000_0000 {
            // NOTE: We DO NOT know if this is >= u32::MAX,
            // and the `write_digits!(@10)` is only accurate
            // if `n <= 5.5e9`, which we cannot guarantee.
            write_digits!(@10u64 buffer, n)
        } else if n >= 1_0000_0000 {
            write_digits!(@9 buffer, n)
        } else if n >= 100_0000 {
            write_digits!(@7-8 buffer, n)
        } else {
            write_digits!(@5-6 buffer, n)
        }
    } else {
        // 11-20 digits, can do in 2 steps (11-19 if is signed).
        // NOTE: `hi` has to be in `[0, 2^31)`, while `lo` is in `[0, 10^11)`
        // So, we can use our `from_u64_small` for hi. For our `lo`, we always
        // need to write 10 digits. However, the `jeaiii` algorithm is too
        // slow, so we use a modified variant of our 2-digit unfolding for
        // exactly 10 digits to read our values. We can optimize this in
        // 2x 4 digits and 1x 2 digits.
        let hi = (n / FACTOR) as u32;
        let lo = n % FACTOR;
        let offset = from_u32(hi, buffer);
        write_digits!(@10alex buffer, lo, offset)
    }
}

/// Optimized jeaiii algorithm for u64.
#[inline(always)]
pub fn from_u64(n: u64, buffer: &mut [u8]) -> usize {
    from_u64_impl(n, buffer, false)
}

/// Optimized jeaiii algorithm for i64, which must be positive.
///
/// This value **MUST** have originally been from an `i64`, since it
/// uses `19` for the bounds checked, so this will panic if `>= 10^19`
/// is passed to the function.
#[inline(always)]
pub fn from_i64(n: u64, buffer: &mut [u8]) -> usize {
    debug_assert!(n <= 1000_0000_0000_0000_0000u64);
    from_u64_impl(n, buffer, true)
}

/// Optimized jeaiii algorithm for u128.
#[inline(always)]
#[allow(clippy::collapsible_else_if)] // reason = "branching is fine-tuned for performance"
pub fn from_u128(n: u128, buffer: &mut [u8]) -> usize {
    // NOTE: Like before, this optimizes better for large and small
    // values if there's a flat comparison with larger values first.
    let buffer = &mut buffer[..39];
    if n < 1_0000 {
        // 1 to 4 digits
        if n >= 100 {
            write_digits!(@3-4 buffer, n)
        } else if n >= 10 {
            write_digits!(@2 buffer, n)
        } else {
            write_digits!(@1 buffer, n)
        }
    } else if n < 100_0000_0000 {
        // 5 to 10 digits
        if n >= 10_0000_0000 {
            // NOTE: We DO NOT know if this is >= u32::MAX,
            // and the `write_digits!(@10)` is only accurate
            // if `n <= 5.5e9`, which we cannot guarantee.
            write_digits!(@10u64 buffer, n)
        } else if n >= 1_0000_0000 {
            write_digits!(@9 buffer, n)
        } else if n >= 100_0000 {
            write_digits!(@7-8 buffer, n)
        } else {
            write_digits!(@5-6 buffer, n)
        }
    } else {
        // 11-39 digits, can do in 2-4 steps

        // NOTE: We need to use fast division (`u128_divrem`) for this, which
        // we can do in 2-4 steps (`2^128 - 1 == ~3.4e38`). So, we need to
        // calculate the number of digits to avoid shifting into place, then
        // once we do, we can write 1-3 `lo` digits and the `hi` digits (which
        // must be in the range `[0, 2^29)`). Our `jeaiii` algorithm is too
        // slow, so we use a modified variant of our 2-digit unfolding for
        // exactly 10 digits to read our values. We can optimize this in
        // 2x 4 digits and 1x 2 digits.
        if n >= 100_0000_0000_0000_0000_0000_0000_0000 {
            // 4 steps
            let (mid, d) = div128_rem_1e10(n);
            let (mid, c) = div128_rem_1e10(mid);
            let (hi, b) = div128_rem_1e10(mid);
            // NOTE: `2^128 == ~3.4e38`, so `a` must be in the
            // range `[0, 2^29)`)
            let a = hi as u32;
            let mut offset = from_u32(a, buffer);
            offset = write_digits!(@10alex buffer, b, offset);
            offset = write_digits!(@10alex buffer, c, offset);
            write_digits!(@10alex buffer, d, offset)
        } else if n >= 1_0000_0000_0000_0000_0000 {
            // 3 steps
            let (mid, lo) = div128_rem_1e10(n);
            let (hi, mid) = div128_rem_1e10(mid);
            let hi = hi as u64;
            let mut offset = from_u64(hi, buffer);
            offset = write_digits!(@10alex buffer, mid, offset);
            write_digits!(@10alex buffer, lo, offset)
        } else {
            // 2 steps
            let (hi, lo) = div128_rem_1e10(n);
            let hi = hi as u64;
            let offset = from_u64(hi, buffer);
            write_digits!(@10alex buffer, lo, offset)
        }
    }
}
