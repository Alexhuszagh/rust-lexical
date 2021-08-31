//! Radix-generic, optimized, integer-to-string conversion routines.
//!
//! These routines are highly optimized: they unroll 4 loops at a time,
//! using pre-computed base^2 tables.
//!
//! See [Algorithm.md](/docs/Algorithm.md) for a more detailed description of
//! the algorithm choice here. See [Benchmarks.md](/docs/Benchmarks.md) for
//! recent benchmark data.

#![cfg(not(feature = "compact"))]
#![doc(hidden)]

use lexical_util::assert::debug_assert_radix;
use lexical_util::digit::digit_to_char;
use lexical_util::div128::u128_divrem;
use lexical_util::format::{radix_from_flags, NumberFormat};
use lexical_util::num::{AsCast, UnsignedInteger};
use lexical_util::step::u64_step;

/// Write 2 digits to buffer.
///
/// # Safety
///
/// Safe if `bytes` is large enough to hold 2 characters, `index >= 2`,
/// and if the 2 * remainder, or `r`, has it so `r + 1 < table.len()`.
macro_rules! write_digits {
    ($bytes:ident, $index:ident, $table:ident, $r:ident) => {{
        debug_assert!($index >= 2);
        debug_assert!($bytes.len() >= 2);
        debug_assert!($r + 1 < $table.len());
        $index -= 1;
        unsafe { index_unchecked_mut!($bytes[$index] = $table[$r + 1]) };
        $index -= 1;
        unsafe { index_unchecked_mut!($bytes[$index] = $table[$r]) };
    }};
}

/// Write 1 digit to buffer.
///
/// # Safety
///
/// Safe if `bytes` is large enough to hold 1 characters, and `r < 36`.
macro_rules! write_digit {
    ($bytes:ident, $index:ident, $r:ident) => {{
        debug_assert!($index >= 1);
        debug_assert!($bytes.len() >= 1);
        debug_assert!($r < 36);
        $index -= 1;
        unsafe { index_unchecked_mut!($bytes[$index]) = digit_to_char($r) };
    }};
}

// NOTE: Don't use too many generics:
//  We don't need generics for most of the internal algorithms,
//  and doing so kills performance. Why? I don't know, but assuming
//  it messed with the compiler's code generation.

/// Write integral digits to buffer.
///
/// This algorithm first writes 4, then 2 digits at a time, finally
/// the last 1 or 2 digits, using power reduction to speed up the
/// algorithm a lot.
///
/// # Safety
///
/// This is safe as long as the buffer is large enough to hold `T::MAX`
/// digits in radix `N`.
unsafe fn write_digits<T: UnsignedInteger>(
    mut value: T,
    radix: u32,
    table: &[u8],
    buffer: &mut [u8],
    mut index: usize,
) -> usize {
    debug_assert_radix(radix);

    // Pre-compute our powers of radix.
    let radix = T::from_u32(radix);
    let radix2 = radix * radix;
    let radix4 = radix2 * radix2;

    // SAFETY: All of these are safe for the buffer writes as long as
    // the buffer is large enough to hold `T::MAX` digits in radix `N`.

    // Decode 4 digits at a time.
    while value >= radix4 {
        let r = value % radix4;
        value /= radix4;
        let r1 = usize::as_cast(T::TWO * (r / radix2));
        let r2 = usize::as_cast(T::TWO * (r % radix2));

        // SAFETY: This is always safe, since the table is 2*radix^2, and
        // r1 and r2 must be in the range [0, 2*radix^2-1), since the maximum
        // value of r is `radix4-1`, which must have a div and r
        // in the range [0, radix^2-1).
        write_digits!(buffer, index, table, r2);
        write_digits!(buffer, index, table, r1);
    }

    // Decode 2 digits at a time.
    while value >= radix2 {
        let r = usize::as_cast(T::TWO * (value % radix2));
        value /= radix2;

        // SAFETY: this is always safe, since the table is 2*radix^2, and
        // r must be in the range [0, 2*radix^2-1).
        write_digits!(buffer, index, table, r);
    }

    // Decode last 2 digits.
    if value < radix {
        // SAFETY: this is always safe, since value < radix, so it must be < 36.
        let r = u32::as_cast(value);
        write_digit!(buffer, index, r);
    } else {
        let r = usize::as_cast(T::TWO * value);
        // SAFETY: this is always safe, since the table is 2*radix^2, and
        // the value must <= radix^2, so rem must be in the range
        // [0, 2*radix^2-1).
        write_digits!(buffer, index, table, r);
    }

    index
}

/// Specialized digits writer for u128, since it writes at least step digits.
///
/// # Safety
///
/// This is safe as long as the buffer is large enough to hold `T::MAX`
/// digits in radix `N`.
unsafe fn write_step_digits<T: UnsignedInteger>(
    value: T,
    radix: u32,
    table: &[u8],
    buffer: &mut [u8],
    index: usize,
    step: usize,
) -> usize {
    debug_assert_radix(radix);

    let start = index;
    // SAFETY: safe as long as the call to write_step_digits is safe.
    let index = unsafe { write_digits(value, radix, table, buffer, index) };
    // Write the remaining 0 bytes.
    // SAFETY: this is always safe since `end < index && index < start`.
    let end = start.saturating_sub(step);
    unsafe {
        let zeros = &mut index_unchecked_mut!(buffer[end..index]);
        slice_fill_unchecked!(zeros, b'0');
    }

    end
}

/// Optimized implementation for radix-N numbers.
///
/// # Safety
///
/// Safe as long as the buffer is large enough to hold as many digits
/// that can be in the largest value of `T`, in radix `N`.
#[inline]
pub unsafe fn algorithm<T>(value: T, radix: u32, table: &[u8], buffer: &mut [u8]) -> usize
where
    T: UnsignedInteger,
{
    // This is so that radix^4 does not overflow, since 36^4 overflows a u16.
    debug_assert!(T::BITS >= 32, "Must have at least 32 bits in the input.");
    debug_assert_radix(radix);

    // SAFETY: Both forms of unchecked indexing cannot overflow.
    // The table always has 2*radix^2 elements, so it must be a legal index.
    // The buffer is ensured to have at least `FORMATTED_SIZE` or
    // `FORMATTED_SIZE_DECIMAL` characters, which is the maximum number of
    // digits an integer of that size may write.
    unsafe { write_digits(value, radix, table, buffer, buffer.len()) }
}

/// Optimized implementation for radix-N 128-bit numbers.
///
/// # Safety
///
/// Safe as long as the buffer is large enough to hold as many digits
/// that can be in the largest value of `T`, in radix `N`.
#[inline]
pub unsafe fn algorithm_u128<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
    value: u128,
    table: &[u8],
    buffer: &mut [u8],
) -> usize {
    //  NOTE:
    //      Use the const version of radix for u64_step and u128_divrem
    //      to ensure they're evaluated at compile time.
    assert!(NumberFormat::<{ FORMAT }> {}.is_valid());

    // Quick approximations to make the algorithm **a lot** faster.
    // If the value can be represented in a 64-bit integer, we can
    // do this as a native integer.
    let radix = radix_from_flags(FORMAT, MASK, SHIFT);
    if value <= u64::MAX as _ {
        // SAFETY: safe if the buffer is large enough to hold the significant digits.
        return unsafe { algorithm(value as u64, radix, table, buffer) };
    }

    // SAFETY: Both forms of unchecked indexing cannot overflow.
    // The table always has 2*radix^2 elements, so it must be a legal index.
    // The buffer is ensured to have at least `FORMATTED_SIZE` or
    // `FORMATTED_SIZE_DECIMAL` characters, which is the maximum number of
    // digits an integer of that size may write.

    // We use a fast 128-bit division algorithm, described in depth
    // in lexical_util/div128.

    // Decode 4-digits at a time.
    // To deal with internal 0 values or values with internal 0 digits set,
    // we store the starting index, and if not all digits are written,
    // we just skip down `digits` digits for the next value.
    let step = u64_step(radix_from_flags(FORMAT, MASK, SHIFT));
    let (value, low) = u128_divrem(value, radix_from_flags(FORMAT, MASK, SHIFT));
    let mut index = buffer.len();
    index = unsafe { write_step_digits(low, radix, table, buffer, index, step) };
    if value <= u64::MAX as _ {
        return unsafe { write_digits(value as u64, radix, table, buffer, index) };
    }

    // Value has to be greater than 1.8e38
    let (value, mid) = u128_divrem(value, radix_from_flags(FORMAT, MASK, SHIFT));
    index = unsafe { write_step_digits(mid, radix, table, buffer, index, step) };
    if index != 0 {
        index = unsafe { write_digits(value as u64, radix, table, buffer, index) };
    }

    index
}
