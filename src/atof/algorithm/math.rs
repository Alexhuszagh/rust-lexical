//! Building-blocks for arbitrary-precision math.
//!
//! These algorithms assume little-endian order for the large integer
//! buffers, so for a `vec![0, 1, 2, 3]`, `3` is the most significant `u32`,
//! and `0` is the least significant `u32`.

// SCALAR
// ------

// Scalar-to-scalar operations, for building-blocks for arbitrary-precision
// operations.

pub(in atof::algorithm) mod scalar {

use util::*;

// ABOVE HALFWAY

/// Determine if `2*rem > den`, or `2*rem == den`.
#[inline]
pub fn cmp_remainder(den: u32, rem: u32) -> (bool, bool) {
    match rem.checked_mul(2) {
        None    => (true, false),
        Some(v) => (v > den, v == den),
    }
}

// ADDITION

/// Add two small integers and return the resulting value and if overflow happens.
#[inline(always)]
pub fn add(x: u32, y: u32)
    -> (u32, bool)
{
    x.overflowing_add(y)
}

/// AddAssign two small integers and return if overflow happens.
#[inline(always)]
pub fn iadd(x: &mut u32, y: u32)
    -> bool
{
    let t = add(*x, y);
    *x = t.0;
    t.1
}

// SUBTRACTION

/// Subtract two small integers and return the resulting value and if overflow happens.
#[inline(always)]
pub fn sub(x: u32, y: u32)
    -> (u32, bool)
{
    x.overflowing_sub(y)
}

/// SubAssign two small integers and return if overflow happens.
#[inline(always)]
pub fn isub(x: &mut u32, y: u32)
    -> bool
{
    let t = sub(*x, y);
    *x = t.0;
    t.1
}

// MULTIPLICATION

/// Multiply two small integers (with carry) (and return the overflow contribution).
///
/// Returns the (low, high) components.
#[inline(always)]
pub fn mul(x: u32, y: u32, carry: u32)
    -> (u32, u32)
{
    // Cannot overflow, as long as wide is 2x as wide. This is because
    // the following is always true:
    // `Wide::max_value() - (Narrow::max_value() * Narrow::max_value()) >= Narrow::max_value()`
    let z: u64 = x.as_u64() * y.as_u64() + carry.as_u64();
    (z.as_u32(), (z >> u32::BITS).as_u32())
}

/// Multiply two small integers (with carry) (and return if overflow happens).
#[inline(always)]
pub fn imul(x: &mut u32, y: u32, carry: u32)
    -> u32
{
    let t = mul(*x, y, carry);
    *x = t.0;
    t.1
}

// DIVISION

/// Divide two small integers (with remainder) (and return the remainder contribution).
///
/// Returns the (value, remainder) components.
#[inline(always)]
pub fn div(x: u32, y: u32, rem: u32)
    -> (u32, u32)
{
    // Cannot overflow, as long as wide is 2x as wide.
    let x = x.as_u64() | (rem.as_u64() << u32::BITS);
    let y = y.as_u64();
    ((x / y).as_u32(), (x % y).as_u32())
}

/// DivAssign two small integers and return the remainder.
#[inline(always)]
pub fn idiv(x: &mut u32, y: u32, rem: u32)
    -> u32
{
    let t = div(*x, y, rem);
    *x = t.0;
    t.1
}

}   // scalar

// SMALL
// -----

// Large-to-small operations, to modify a big integer from a native scalar.

pub(in atof::algorithm) mod small {

use lib::iter;
use util::*;
use super::{large, scalar};
use super::super::small_powers::*;

// ROUNDUP

/// Round to the nearest value if there's truncation during division.
///
/// If the `2*rem > y`, or `2*rem == y` and the last, then we have a
/// division where the remainder is near the halfway case.
/// digit is odd, round-up (round-nearest, tie-even).
#[inline]
pub fn check_do_roundup<T>(x: &mut T, y: u32, rem: u32)
    where T: CloneableVecLike<u32>
{
    unsafe {
        let (is_above, is_halfway) = scalar::cmp_remainder(y, rem);
        if is_above || (is_halfway && x.front_unchecked().is_odd()) {
            *x.front_unchecked_mut() += 1;
        }
    }
}

// PROPERTIES

/// Get the number of leading zero values in the storage.
/// Assumes the value is normalized.
#[inline]
#[allow(dead_code)]
pub fn leading_zero_values(_: &[u32]) -> usize {
    0
}

/// Get the number of trailing zero values in the storage.
/// Assumes the value is normalized.
#[inline]
#[allow(dead_code)]
pub fn trailing_zero_values(x: &[u32]) -> usize {
    let mut iter = x.iter().enumerate();
    let opt = iter.find(|&tup| !tup.1.is_zero());
    let value = opt
        .map(|t| t.0)
        .unwrap_or(x.len());

    value
}

/// Get number of leading zero bits in the storage.
#[inline]
#[allow(dead_code)]
pub fn leading_zeros(x: &[u32]) -> usize {
    if x.is_empty() {
        0
    } else {
        unsafe {
            let index = x.len() - 1;
            x.get_unchecked(index).leading_zeros().as_usize()
        }
    }
}

/// Get number of trailing zero bits in the storage.
/// Assumes the value is normalized.
#[inline]
#[allow(dead_code)]
pub fn trailing_zeros(x: &[u32]) -> usize {
    // Get the index of the last non-zero value
    let index = trailing_zero_values(x);
    let mut count = index.checked_mul(u32::BITS);
    if let Some(value) = x.get(index) {
        count = count.and_then(|v| v.checked_add(value.trailing_zeros().as_usize()));
    }
    count.unwrap_or(usize::max_value())
}

// BIT LENGTH

/// Calculate the bit-length of the big-integer.
#[inline]
#[allow(dead_code)]
pub fn bit_length(x: &[u32]) -> usize {
    // Avoid overflowing, calculate via total number of bits
    // minus leading zero bits.
    let nlz = leading_zeros(x);
    u32::BITS.checked_mul(x.len())
        .map(|v| v - nlz)
        .unwrap_or(usize::max_value())
}

// SHR

/// Shift-right bits inside a buffer and returns the truncated bits.
///
/// Returns the truncated bits.
///
/// Assumes `n < 32`, IE, internally shifting bits.
#[inline]
pub fn ishr_bits<T>(x: &mut T, n: u32)
    -> u32
    where T: CloneableVecLike<u32>
{
    // Need to shift by the number of `bits % 32`.
    let bits = u32::BITS.as_u32();
    debug_assert!(n < bits && n != 0);

    // Internally, for each item, we shift left by n, and add the previous
    // right shifted 32-bits.
    // For example, we transform (for u8) shifted right 2, to:
    //      b10100100 b01000010
    //        b101001 b00010000
    let lshift = bits - n;
    let rshift = n;
    let mut prev: u32 = 0;
    for xi in x.iter_mut().rev() {
        let tmp = *xi;
        *xi >>= rshift;
        *xi |= prev << lshift;
        prev = tmp;
    }

    prev & lower_n_mask(rshift)
}

/// Shift-right `n` digits inside a buffer and returns if all the truncated digits are zero.
///
/// Assumes `n` is not 0.
#[inline]
pub fn ishr_digits<T>(x: &mut T, n: usize)
    -> bool
    where T: CloneableVecLike<u32>
{
    debug_assert!(n != 0);

    if n >= x.len() {
        unsafe {
            x.set_len(0);
        }
        false
    } else {
        let is_zero = (&x[..n]).iter().all(|v| v.is_zero());
        x.remove_many(0..n);
        is_zero
    }
}

/// Shift-left buffer by n bits and return if we should round-up.
pub fn ishr<T>(x: &mut T, n: usize)
    -> bool
    where T: CloneableVecLike<u32>
{
    let bits = u32::BITS;
    // Need to pad with zeros for the number of `bits / 32`,
    // and shift-left with carry for `bits % 32`.
    let rem = (n % bits).as_u32();
    let div = n / bits;
    let is_zero = match div.is_zero() {
        true  => true,
        false => ishr_digits(x, div),
    };
    let truncated = match rem.is_zero() {
        true  => 0,
        false => ishr_bits(x, rem),
    };

    // Calculate if we need to roundup.
    let roundup = unsafe {
        let halfway = lower_n_halfway(rem);
        if truncated > halfway {
            // Above halfway
            true
        } else if truncated == halfway {
            // Exactly halfway, if !is_zero, we have a tie-breaker,
            // otherwise, we follow round-to-nearest, tie-even rules.
            // Cannot be empty, since truncated is non-zero.
            !is_zero || x.front_unchecked().is_odd()
        } else {
            // Below halfway
            false
        }
    };

    // Normalize the data
    normalize(x);

    roundup
}

/// Shift-left buffer by n bits.
#[allow(dead_code)]
pub fn shr<T>(x: &[u32], n: usize)
    -> (T, bool)
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    let roundup = ishr(&mut z, n);
    (z, roundup)
}

// SHL

/// Shift-left bits inside a buffer.
///
/// Assumes `n < 32`, IE, internally shifting bits.
#[inline]
pub fn ishl_bits<T>(x: &mut T, n: u32)
    where T: CloneableVecLike<u32>
{
    // Need to shift by the number of `bits % 32`.
    let bits = u32::BITS.as_u32();
    debug_assert!(n != 0 && n != u32::BITS.as_u32());

    // Internally, for each item, we shift left by n, and add the previous
    // right shifted 32-bits.
    // For example, we transform (for u8) shifted left 2, to:
    //      b10100100 b01000010
    //      b10 b10010001 b00001000
    let rshift = bits - n;
    let lshift = n;
    let mut prev: u32 = 0;
    for xi in x.iter_mut() {
        let tmp = *xi;
        *xi <<= lshift;
        *xi |= prev >> rshift;
        prev = tmp;
    }

    // Always push the carry, even if it creates a non-normal result.
    let carry = prev >> rshift;
    if carry != 0 {
        x.push(carry);
    }
}

/// Shift-left bits inside a buffer.
///
/// Assumes `n < 32`, IE, internally shifting bits.
#[allow(dead_code)]
pub fn shl_bits<T>(x: &[u32], n: u32)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    ishl_bits(&mut z, n);
    z
}

/// Shift-left `n` digits inside a buffer.
///
/// Assumes `n` is not 0.
#[inline]
pub fn ishl_digits<T>(x: &mut T, n: usize)
    where T: CloneableVecLike<u32>
{
    debug_assert!(n != 0);
    if !x.is_empty() {
        x.insert_many(0, iter::repeat(0).take(n));
    }
}

/// Shift-left buffer by n bits.
pub fn ishl<T>(x: &mut T, n: usize)
    where T: CloneableVecLike<u32>
{
    let bits = u32::BITS;
    // Need to pad with zeros for the number of `bits / 32`,
    // and shift-left with carry for `bits % 32`.
    let rem = (n % bits).as_u32();
    let div = n / bits;
    if !rem.is_zero() {
        ishl_bits(x, rem);
    }
    if !div.is_zero() {
        ishl_digits(x, div);
    }
}

/// Shift-left buffer by n bits.
#[allow(dead_code)]
pub fn shl<T>(x: &[u32], n: usize)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    ishl(&mut z, n);
    z
}

// NORMALIZE

/// Normalize the container by popping any leading zeros.
#[inline]
pub fn normalize<T>(x: &mut T)
    where T: CloneableVecLike<u32>
{
    unsafe {
        // Remove leading zero if we cause underflow. Since we're dividing
        // by a small power, we have at max 1 int removed.
        while !x.is_empty() && x.back_unchecked().is_zero() {
            x.pop();
        }
    }
}

/// ADDITION

/// Implied AddAssign implementation for adding a small integer to bigint..
///
/// Allows us to choose a start-index in x to store, to allow incrementing
/// from a non-zero start.
pub fn iadd_impl<T>(x: &mut T, y: u32, xstart: usize)
    where T: CloneableVecLike<u32>
{
    if x.len() <= xstart {
        x.push(y);
    } else {
        unsafe {
            // Initial add
            let mut carry = scalar::iadd(x.get_unchecked_mut(xstart), y);

            // Increment until overflow stops occurring.
            let mut size = xstart + 1;
            while carry && size < x.len() {
                carry = scalar::iadd(x.get_unchecked_mut(size), 1);
                size += 1;
            }

            // If we overflowed the buffer entirely, need to add 1 to the end
            // of the buffer.
            if carry {
                x.push(1);
            }
        }
    }
}

/// AddAssign small integer to bigint.
pub fn iadd<T>(x: &mut T, y: u32)
    where T: CloneableVecLike<u32>
{
    iadd_impl(x, y, 0);
}

/// Add small integer to bigint.
#[allow(dead_code)]
pub fn add<T>(x: &[u32], y: u32)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    iadd(&mut z, y);
    z
}

// SUBTRACTION

/// SubAssign small integer to bigint.
/// Does not do overflowing subtraction.
pub fn isub_impl<T>(x: &mut T, y: u32, xstart: usize)
    where T: CloneableVecLike<u32>
{
    debug_assert!(x.len() > xstart && (x[xstart] >= y || x.len() > xstart+1));

    unsafe {
        // Initial subtraction
        let mut carry = scalar::isub(x.get_unchecked_mut(xstart), y);

        // Increment until overflow stops occurring.
        let mut size = xstart + 1;
        while carry && size < x.len() {
            carry = scalar::isub(x.get_unchecked_mut(size), 1);
            size += 1;
        }
        normalize(x);
    }
}

/// SubAssign small integer to bigint.
/// Does not do overflowing subtraction.
pub fn isub<T>(x: &mut T, y: u32)
    where T: CloneableVecLike<u32>
{
    isub_impl(x, y, 0);
}

/// Sub small integer to bigint.
#[allow(dead_code)]
pub fn sub<T>(x: &[u32], y: u32)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    isub(&mut z, y);
    z
}

// MULTIPLICATION

/// MulAssign small integer to bigint.
pub fn imul<T>(x: &mut T, y: u32)
    where T: CloneableVecLike<u32>
{
    // Multiply iteratively over all elements, adding the carry each time.
    let mut carry: u32 = 0;
    for xi in x.iter_mut() {
        carry = scalar::imul(xi, y, carry);
    }

    // Overflow of value, add to end.
    if carry != 0 {
        x.push(carry);
    }
}

/// Mul small integer to bigint.
#[allow(dead_code)]
pub fn mul<T>(x: &[u32], y: u32)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    imul(&mut z, y);
    z
}

/// MulAssign by a power.
///
/// Theoretically...

/// Use an exponentiation by squaring method, since it reduces the time
/// complexity of the multiplication to ~`O(log(n))` for the squaring,
/// and `O(n*m)` for the result. Since `m` is typically a lower-order
/// factor, this significantly reduces the number of multiplications
/// we need to do. Iteratively multiplying by small powers follows
/// the nth triangular number series, which scales as `O(p^2)`, but
/// where `p` is `n+m`. In short, it scales very poorly.
///
/// Practically....
///
/// Exponentiation by Squaring:
///     running 2 tests
///     test bigcomp_f32_lexical ... bench:       1,018 ns/iter (+/- 78)
///     test bigcomp_f64_lexical ... bench:       3,639 ns/iter (+/- 1,007)
///
/// Exponentiation by Iterative Small Powers:
///     running 2 tests
///     test bigcomp_f32_lexical ... bench:         518 ns/iter (+/- 31)
///     test bigcomp_f64_lexical ... bench:         583 ns/iter (+/- 47)
///
/// Even using worst-case scenarios, exponentiation by squaring is
/// significantly slower for our workloads. Just multiply by small powers.
pub fn imul_power<T>(x: &mut T, base: u32, mut n: u32)
    where T: CloneableVecLike<u32>
{
    let small_powers = get_small_powers(base);
    let get_power = | i: usize | unsafe { *small_powers.get_unchecked(i) };

    // Multiply by the largest small power until n < step.
    let step = small_powers.len() - 1;
    let power = get_power(step);
    let step = step as u32;
    while n >= step {
        imul(x, power);
        n -= step;
    }

    // Multiply by the remainder.
    imul(x, get_power(n as usize));
}

/// Mul by a power.
#[allow(dead_code)]
pub fn mul_power<T>(x: &[u32], base: u32, n: u32)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    imul_power(&mut z, base, n);
    z
}

/// DIVISION

/// DivAssign small integer to bigint and get the remainder.
pub fn idiv<T>(x: &mut T, y: u32)
    -> u32
    where T: CloneableVecLike<u32>
{
    // Divide iteratively over all elements, adding the carry each time.
    let mut rem: u32 = 0;
    for xi in x.iter_mut().rev() {
        rem = scalar::idiv(xi, y, rem);
    }
    normalize(x);

    rem
}

/// Div small integer to bigint and get the remainder.
#[allow(dead_code)]
pub fn div<T>(x: &[u32], y: u32)
    -> (T, u32)
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    let rem = idiv(&mut z, y);
    (z, rem)
}

/// DivAssign by a power.
///
/// It doesn't really make sense to iteratively store on to the remainders,
/// so we truncate slightly at each step. Ideally, there's enough guard digits
/// to avoid this.
///
/// See `imul_power` for why we do this iteratively, rather than calculate
/// the exponent via exponentiation by squaring and then do a
/// 1-shot division.
pub fn idiv_power<T>(x: &mut T, base: u32, mut n: u32, roundup: bool)
    where T: CloneableVecLike<u32>
{
    let small_powers = get_small_powers(base);
    let get_power = | i: usize | unsafe { *small_powers.get_unchecked(i) };

    // Divide by the largest small power until n < step.
    let step = small_powers.len() - 1;
    let power = get_power(step);
    let step = step as u32;
    while n >= step {
        let rem = idiv(x, power);
        if roundup {
            check_do_roundup(x, power, rem);
        }
        n -= step;
    }

    // Multiply by the remainder.
    let power = get_power(n as usize);
    let rem = idiv(x, power);
    if roundup {
        check_do_roundup(x, power, rem);
    }
}

/// Div by a power.
#[allow(dead_code)]
pub fn div_power<T>(x: &[u32], base: u32, n: u32, roundup: bool)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    idiv_power(&mut z, base, n, roundup);
    z
}

// POWER

/// Calculate x^n, using exponentiation by squaring.
pub fn ipow<T>(x: &mut T, mut n: u32)
    where T: CloneableVecLike<u32>
{
    // Store `x` as 1, and switch `base` to `x`.
    let mut base = T::default();
    base.push(1);
    mem::swap(x, &mut base);

    // Do main algorithm.
    loop {
        if n.is_odd() {
            large::imul(x, &base);
        }
        n /= 2;

        // We need to break as a post-condition, since the real work
        // is in the `imul` and `mul` algorithms.
        if n.is_zero() {
            break;
        } else {
            base = large::mul(&base, &base);
        }
    }
}

/// Calculate x^n, using exponentiation by squaring.
#[allow(dead_code)]
pub fn pow<T>(x: &[u32], n: u32)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    ipow(&mut z, n);
    z
}

}   // small

// LARGE
// -----

// Large-to-large operations, to modify a big integer from a native scalar.

pub(in atof::algorithm) mod large {

use util::*;
use super::{scalar, small};

// RELATIVE OPERATORS

/// Check if x is greater than y.
#[allow(dead_code)]
#[inline]
pub fn greater(x: &[u32], y: &[u32]) -> bool {
    if x.len() > y.len() {
        return true;
    } else if x.len() < y.len() {
        return false;
    } else {
        let iter = x.iter().rev().zip(y.iter().rev());
        for (&xi, &yi) in iter {
            if xi > yi {
                return true;
            } else if xi < yi {
                return false;
            }
        }
        // Equal case.
        return false;
    }
}

/// Check if x is greater than or equal to y.
#[allow(dead_code)]
#[inline]
pub fn greater_equal(x: &[u32], y: &[u32]) -> bool {
    !less(x, y)
}

/// Check if x is less than y.
#[allow(dead_code)]
#[inline]
pub fn less(x: &[u32], y: &[u32]) -> bool {
    if x.len() > y.len() {
        return false;
    } else if x.len() < y.len() {
        return true;
    } else {
        let iter = x.iter().rev().zip(y.iter().rev());
        for (&xi, &yi) in iter {
            if xi > yi {
                return false;
            } else if xi < yi {
                return true;
            }
        }
        // Equal case.
        return false;
    }
}

/// Check if x is less than or equal to y.
#[allow(dead_code)]
#[inline]
pub fn less_equal(x: &[u32], y: &[u32]) -> bool {
    !greater(x, y)
}

/// Check if x is equal to y.
#[allow(dead_code)]
#[inline]
pub fn equal(x: &[u32], y: &[u32]) -> bool {
    let mut iter = x.iter().rev().zip(y.iter().rev());
    x.len() == y.len() && iter.all(|(&xi, &yi)| xi == yi)
}

/// ADDITION

/// Implied AddAssign implementation for bigints.
///
/// Allows us to choose a start-index in x to store, so we can avoid
/// padding the buffer with zeros when not needed, optimized for vectors.
pub fn iadd_impl<T>(x: &mut T, y: &[u32], xstart: usize)
    where T: CloneableVecLike<u32>
{
    // The effective x buffer is from `xstart..x.len()`, so we need to treat
    // that as the current range. If the effective y buffer is longer, need
    // to resize to that, + the start index.
    if y.len() > x.len() - xstart {
        x.resize(y.len() + xstart, 0);
    }

    // Iteratively add elements from y to x.
    let mut carry = false;
    for (xi, yi) in (&mut x[xstart..]).iter_mut().zip(y.iter()) {
        // Only one op of the two can overflow, since we added at max
        // u32::max_value() + u32::max_value(). Add the previous carry,
        // and store the current carry for the next.
        let mut tmp = scalar::iadd(xi, *yi);
        if carry {
            tmp |= scalar::iadd(xi, 1);
        }
        carry = tmp;
    }

    // Overflow from the previous bit.
    if carry {
        small::iadd_impl(x, 1, y.len()+xstart);
    }
}

/// AddAssign bigint to bigint.
#[allow(dead_code)]
pub fn iadd<T>(x: &mut T, y: &[u32])
    where T: CloneableVecLike<u32>
{
    iadd_impl(x, y, 0)
}

/// Add bigint to bigint.
#[allow(dead_code)]
pub fn add<T>(x: &[u32], y: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    iadd(&mut z, y);
    z
}

// SUBTRACTION

/// SubAssign bigint to bigint.
#[allow(dead_code)]
pub fn isub<T>(x: &mut T, y: &[u32])
    where T: CloneableVecLike<u32>
{
    // Basic underflow checks.
    debug_assert!(greater_equal(x, y));

    // Iteratively add elements from y to x.
    let mut carry = false;
    for (xi, yi) in x.iter_mut().zip(y.iter()) {
        // Only one op of the two can overflow, since we added at max
        // u32::max_value() + u32::max_value(). Add the previous carry,
        // and store the current carry for the next.
        let mut tmp = scalar::isub(xi, *yi);
        if carry {
            tmp |= scalar::isub(xi, 1);
        }
        carry = tmp;
    }

    if carry {
        small::isub_impl(x, 1, y.len());
    } else {
        small::normalize(x);
    }
}

/// Sub bigint to bigint.
#[allow(dead_code)]
pub fn sub<T>(x: &[u32], y: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    isub(&mut z, y);
    z
}

// MULTIPLICATIION

/// Number of digits to bottom-out to asymptotically slow algorithms.
///
/// Karatsuba tends to out-perform long-multiplication at ~320-640 bits,
/// so we go halfway, while Newton division tends to out-perform
/// Algorithm D at ~1024 bits. We can toggle this for optimal performance.
const DIGITS_CUTOFF: usize = 30;

/// Grade-school multiplication algorithm.
///
/// Slow, naive algorithm, using 32-bit bases and just shifting left for
/// each iteration. This could be optimized with numerous other algorithms,
/// but it's extremely simple, and works in O(n*m) time, which is fine
/// by me. Each iteration, of which there are `m` iterations, requires
/// `n` multiplications, and `n` additions, or grade-school multiplication.
fn long_mul<T>(x: &[u32], y: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    // Make x and empty buffer, and z an immutable copy to the new data.
    let mut z = T::default();
    z.resize(x.len() + y.len(), 0);

    // Using the immutable value, multiply by all the scalars in y, using
    // the algorithm defined above.
    for (i, yi) in y.iter().enumerate() {
        let mut xi = T::default();
        xi.extend_from_slice(x);
        small::imul(&mut xi, *yi);
        iadd_impl(&mut z, &xi, i);
    }
    small::normalize(&mut z);

    z
}


/// Split two buffers into halfway, into (lo, hi).
pub fn karatsuba_split<'a>(z: &'a [u32], m: usize)
    -> (&'a [u32], &'a [u32])
{
    (&z[..m], &z[m..])
}

/// Karatsuba multiplication algorithm with roughly equal input sizes.
///
/// Assumes `y.len() >= x.len()`.
fn karatsuba_mul<T>(x: &[u32], y: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    if y.len() <= DIGITS_CUTOFF {
        // Bottom-out to long division for small cases.
        long_mul(x, y)
    } else if x.len() < y.len() / 2 {
        karatsuba_uneven_mul(x, y)
    } else {
        // Do our 3 multiplications.
        let m = y.len() / 2;
        let (xl, xh) = karatsuba_split(x, m);
        let (yl, yh) = karatsuba_split(y, m);
        let sumx: T = add(xl, xh);
        let sumy: T = add(yl, yh);
        let z0: T = karatsuba_mul(xl, yl);
        let mut z1: T = karatsuba_mul(&sumx, &sumy);
        let z2: T = karatsuba_mul(xh, yh);
        // Properly scale z1, which is `z1 - z2 - zo`.
        isub(&mut z1, &z2);
        isub(&mut z1, &z0);

        // Create our result, which is equal to, in little-endian order:
        // [z0, z1 - z2 - z0, z2]
        //  z1 must be shifted m digits (2^(32m)) over.
        //  z2 must be shifted 2*m digits (2^(64m)) over.
        let mut result = T::default();
        let len = z0.len().max(m + z1.len()).max(2*m + z2.len());
        result.reserve_exact(len);
        result.extend_from_slice(&z0);
        iadd_impl(&mut result, &z1, m);
        iadd_impl(&mut result, &z2, 2*m);

        result
    }
}

/// Karatsuba multiplication algorithm where y is substantially larger than x.
///
/// Assumes `y.len() >= x.len()`.
fn karatsuba_uneven_mul<T>(x: &[u32], mut y: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    let mut result = T::default();
    result.resize(x.len() + y.len(), 0);

    // This effectively is like grade-school multiplication between
    // two numbers, except we're using splits on `y`, and the intermediate
    // step is a Karatsuba multiplication.
    let mut start = 0;
    while y.len() != 0 {
        let m = x.len().min(y.len());
        let (yl, yh) = karatsuba_split(y, m);
        let prod: T = karatsuba_mul(x, yl);
        iadd_impl(&mut result, &prod, start);
        y = yh;
        start += m;
    }
    small::normalize(&mut result);

    result
}

/// Forwarder to the proper Karatsuba algorithm.
#[inline]
#[allow(dead_code)]
fn karatsuba_mul_fwd<T>(x: &[u32], y: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    if x.len() < y.len() {
        karatsuba_mul(x, y)
    } else {
        karatsuba_mul(y, x)
    }
}

/// MulAssign bigint to bigint.
#[allow(dead_code)]
#[inline]
pub fn imul<T>(x: &mut T, y: &[u32])
    where T: CloneableVecLike<u32>
{
    unsafe {
        if y.len() == 1 {
            small::imul(x, *y.get_unchecked(0));
        } else {
            // We're not really in a condition where using Karatsuba
            // multiplication makes sense, so we're just going to use long
            // division. ~20% speedup compared to:
            //      *x = karatsuba_mul_fwd(x, y);
            *x = long_mul(x, y);
        }
    }
}

/// Mul bigint to bigint.
#[allow(dead_code)]
pub fn mul<T>(x: &[u32], y: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    imul(&mut z, y);
    z
}

// DIVISION

/// Constants for algorithm D.
const ALGORITHM_D_B: u64 = 1 << 32;
const ALGORITHM_D_M: u64 = ALGORITHM_D_B - 1;

/// Calculate qhat (an estimate for the quotient).
///
/// This is step D3 in Algorithm D in "The Art of Computer Programming".
/// Assumes `x.len() > y.len()` and `y.len() >= 2`.
///
/// * `j`   - Current index on the iteration of the loop.
#[inline]
unsafe fn calculate_qhat(x: &[u32], y: &[u32], j: usize)
    -> u64
{
    let n = y.len();

    // Closures
    let get_u64 = | x: &[u32], i: usize | x.get_unchecked(i).as_u64();

    // Estimate qhat of q[j]
    // Original Code:
    //  qhat = (x[j+n]*B + x[j+n-1])/y[n-1];
    //  rhat = (x[j+n]*B + x[j+n-1]) - qhat*y[n-1];
    let x_jn = get_u64(&x, j+n);
    let x_jn1 = get_u64(&x, j+n-1);
    let num = (x_jn << u32::BITS) + x_jn1;
    let den = get_u64(&y, n-1);
    let mut qhat = num / den;
    let mut rhat = num - qhat * den;

    // Scale qhat and rhat
    // Original Code:
    //  again:
    //    if (qhat >= B || qhat*y[n-2] > B*rhat + x[j+n-2])
    //    { qhat = qhat - 1;
    //      rhat = rhat + y[n-1];
    //      if (rhat < B) goto again;
    //    }
    let x_jn2 = get_u64(&x, j+n-2);
    let y_n2 = get_u64(&y, n-2);
    let y_n1 = get_u64(&y, n-1);
    while qhat >= ALGORITHM_D_B || qhat * y_n2 > (rhat << u32::BITS) + x_jn2 {
        qhat -= 1;
        rhat += y_n1;
        if rhat >= ALGORITHM_D_B {
            break;
        }
    }

    qhat
}

/// Multiply and subtract.
///
/// This is step D4 in Algorithm D in "The Art of Computer Programming",
/// and returns the remainder.
#[inline]
unsafe fn multiply_and_subtract<T>(x: &mut T, y: &T, qhat: u64, j: usize)
    -> i64
    where T: CloneableVecLike<u32>
{
    let n = y.len();

    // Closures
    let set = | x: &mut T, i: usize, xi: u32 | *x.get_unchecked_mut(i) = xi;
    let get_i64 = | x: &T, i: usize | x.get_unchecked(i).as_i64();
    let get_u64 = | x: &T, i: usize | x.get_unchecked(i).as_u64();

    // Multiply and subtract
    // Original Code:
    //  k = 0;
    //  for (i = 0; i < n; i++) {
    //     p = qhat*y[i];
    //     t = x[i+j] - k - (p & 0xFFFFFFFFLL);
    //     x[i+j] = t;
    //     k = (p >> 32) - (t >> 32);
    //  }
    //  t = x[j+n] - k;
    //  x[j+n] = t;
    let mut k: i64 = 0;
    let mut t: i64;
    for i in 0..n {
        let x_ij = get_i64(&x, i+j);
        let y_i = get_u64(&y, i);
        let p = qhat * y_i;
        t = x_ij.wrapping_sub(k).wrapping_sub((p & ALGORITHM_D_M).as_i64());
        set(x, i+j, t.as_u32());
        k = (p >> 32).as_i64() - (t >> 32);
    }
    t = get_i64(&x, j+n) - k;
    set(x, j+n, t.as_u32());

    t
}

/// Calculate the quotient from the estimate and the test.
///
/// This is a mix of step D5 and D6 in Algorithm D, so the algorithm
/// may work for single passes, without a quotient buffer.
#[inline]
unsafe fn test_quotient(qhat: u64, t: i64)
    -> u64
{
    if t < 0 {
        qhat - 1
    } else {
        qhat
    }
}

/// Store the quotient from the estimate.
///
/// This is a mix of step D5 and D6 in Algorithm D, so the algorithm
/// may work for single passes, without a quotient buffer.
#[inline]
unsafe fn store_quotient<T>(q: &mut T, qhat: u64, j: usize)
    where T: CloneableVecLike<u32>
{
    *q.get_unchecked_mut(j) = qhat.as_u32();
}

/// Add back.
///
/// This is step D6 in Algorithm D in "The Art of Computer Programming",
/// and adds back the remainder on the very unlikely scenario we overestimated
/// the quotient by 1. Subtract 1 from the quotient, and add back the
/// remainder.
///
/// This step should be specifically debugged, due to its low likelihood,
/// since the probability is ~2/b, where b in this case is 2^32.
#[inline]
unsafe fn add_back<T>(x: &mut T, y: &T, mut t: i64, j: usize)
    where T: CloneableVecLike<u32>
{
    let n = y.len();

    // Closures
    let set = | x: &mut T, i: usize, xi: u32 | *x.get_unchecked_mut(i) = xi;
    let get_i64 = | x: &T, i: usize | x.get_unchecked(i).as_i64();
    let get_u64 = | x: &T, i: usize | x.get_unchecked(i).as_u64();

    // Store quotient digits
    // If we subtracted too much, add back.
    // Original Code:
    //  q[j] = qhat;              // Store quotient digit.
    //  if (t < 0) {              // If we subtracted too
    //     q[j] = q[j] - 1;       // much, add back.
    //     k = 0;
    //     for (i = 0; i < n; i++) {
    //        t = (unsigned long long)x[i+j] + y[i] + k;
    //        x[i+j] = t;
    //        k = t >> 32;
    //     }
    //     x[j+n] = x[j+n] + k;
    //  }
    if t < 0 {
        let mut k: i64 = 0;
        for i in 0..n {
            t = (get_u64(x, i+j) + get_u64(y, i)).as_i64() + k;
            set(x, i+j, t.as_u32());
            k = t >> 32;
        }
        let x_jn = get_i64(x, j+n) + k;
        set(x, j+n, x_jn.as_u32());
    }
}

/// Calculate the remainder from the quotient.
///
/// This is step D8 in Algorithm D in "The Art of Computer Programming",
/// and "unnormalizes" to calculate the remainder from the quotient.
#[inline]
unsafe fn calculate_remainder<T>(x: &[u32], y: &[u32], s: u32)
    -> T
    where T: CloneableVecLike<u32>
{
    // Closures
    let get = | x: &[u32], i: usize | *x.get_unchecked(i);
    let get_u64 = | x: &[u32], i: usize | get(x, i).as_u64();

    // Calculate the remainder.
    // Original Code:
    //  for (i = 0; i < n-1; i++)
    //     r[i] = (x[i] >> s) | ((unsigned long long)x[i+1] << (32-s));
    //  r[n-1] = x[n-1] >> s;
    let n = y.len();
    let mut r = T::default();
    r.reserve_exact(n);
    let rs = 32 - s;
    for i in 0..n-1 {
        let xi = get_u64(x, i) >> s;
        let xi1 = get_u64(x, i+1) << rs;
        let ri = xi | xi1;
        r.push(ri.as_u32());
    }
    let x_n1 = get(&x, n-1) >> s;
    r.push(x_n1.as_u32());

    r
}

/// Implementation of Knuth's Algorithm D, and return the quotient and remainder.
///
/// `x` is the dividend, and `y` is the divisor.
/// Assumes `x.len() > y.len()` and `y.len() >= 2`.
///
/// Based off the Hacker's Delight implementation of Knuth's Algorithm D
/// in "The Art of Computer Programming".
///     http://www.hackersdelight.org/hdcodetxt/divmnu64.c.txt
///
/// All Hacker's Delight code is public domain, so this routine shall
/// also be placed in the public domain. See:
///     https://www.hackersdelight.org/permissions.htm
unsafe fn algorithm_d_div<T>(x: &[u32], y: &[u32])
    -> (T, T)
    where T: CloneableVecLike<u32>
{
    // Normalize the divisor so the leading-bit is set to 1.
    // x is the dividend, y is the divisor.
    // Need a leading zero on the numerator.
    let s = y.get_unchecked(y.len()-1).leading_zeros();
    let m = x.len();
    let n = y.len();
    let mut xn: T = small::shl_bits(x, s);
    xn.push(0);
    let yn: T = small::shl_bits(y, s);

    // Store certain variables for the algorithm.
    let mut q = T::default();
    q.resize(m-n+1, 0);
    for j in (0..m-n+1).rev() {
        // Estimate the quotient
        let qhat = calculate_qhat(&xn, &yn, j);
        let t = multiply_and_subtract(&mut xn, &yn, qhat, j);
        let qhat = test_quotient(qhat, t);
        store_quotient(&mut q, qhat, j);
        add_back(&mut xn, &yn, t, j);
    }
    let mut r = calculate_remainder(&xn, &yn, s);

    // Normalize our results
    small::normalize(&mut q);
    small::normalize(&mut r);

    (q, r)
}

/// DivAssign bigint to bigint.
#[allow(dead_code)]
pub fn idiv<T>(x: &mut T, y: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    debug_assert!(y.len() != 0);

    unsafe {
        if x.len() < y.len() {
            // Can optimize easily, since the quotient is 0,
            // and the remainder is x. Put before `y.len() == 1`, since
            // it optimizes when `x.len() == 0` nicely.
            let mut r = T::default();
            mem::swap(x, &mut r);
            r
        } else if y.len() == 1 {
            // Can optimize for division by a small value.
            let mut r = T::default();
            r.push(small::idiv(x, *y.get_unchecked(0)));
            r
        } else {
            let (q, r) = algorithm_d_div(x, y);
            *x = q;
            r
        }
    }
}

/// Div bigint to bigint.
#[allow(dead_code)]
pub fn div<T>(x: &[u32], y: &[u32])
    -> (T, T)
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    let rem = idiv(&mut z, y);
    (z, rem)
}

/// Emit a single digit for the quotient and store the remainder in-place.
///
/// An extremely efficient division algorithm for small quotients, requiring
/// you to know the full range of the quotient prior to use. For example,
/// with a quotient that can range from [0, 10), you must have 4 leading
/// zeros in the divisor, so we can use a single-limb division to get
/// an accurate estimate of the quotient. Since we always underestimate
/// the quotient, we can add 1 and then emit the digit.
///
/// Requires a non-normalized denominator, with at least [1-6] leading
/// zeros, depending on the base (for example, 1 for base2, 6 for base36).
///
/// Adapted from David M. Gay's dtoa, and therefore under an MIT license:
///     www.netlib.org/fp/dtoa.c
#[allow(dead_code)]
pub unsafe fn quorem<T>(x: &mut T, y: &T)
    -> u32
    where T: CloneableVecLike<u32>
{
    debug_assert!(y.len() > 0);

    // Closures
    let set = | x: &mut T, i: usize, xi: u32 | *x.get_unchecked_mut(i) = xi;
    let get_u64 = | x: &T, i: usize | x.get_unchecked(i).as_u64();

    // Numerator is smaller the denominator, quotient always 0.
    let m = x.len();
    let n = y.len();
    if m < n {
        return 0;
    }

    // Calculate our initial estimate for q
    let xi = *x.back_unchecked();
    let yi = *y.back_unchecked();
    let mut q = xi / (yi + 1);

    // Need to calculate the remainder if we don't have a 0 quotient.
    if q != 0 {
        let mut borrow: u64 = 0;
        let mut carry: u64 = 0;
        for j in 0..m {
            let p = get_u64(y, j) * q.as_u64() + carry;
            carry = p >> 32;
            let t = get_u64(x, j).wrapping_sub(p & 0xFFFFFFFF).wrapping_sub(borrow);
            borrow = (t >> 32) & 1;
            set(x, j, t.as_u32());
        }
        small::normalize(x);
    }

    // Check if we under-estimated x.
    if greater_equal(x, y) {
        q += 1;
        let mut borrow: u64 = 0;
        let mut carry: u64 = 0;
        for j in 0..m {
            let p = get_u64(y, j)+ carry;
            carry = p >> 32;
            let t = get_u64(x, j).wrapping_sub(p & 0xFFFFFFFF).wrapping_sub(borrow);
            borrow = (t >> 32) & 1;
            set(x, j, t.as_u32());
        }
        small::normalize(x);
    }

    q
}

}   // large

use float::Mantissa;
use util::*;
use super::small_powers::*;

/// Generate the imul_pown wrappers.
macro_rules! imul_power {
    ($name:ident, $base:expr) => (
        /// Multiply by a power of $base.
        #[inline]
        fn $name(&mut self, n: u32) {
            self.imul_power_impl($base, n)
        }
    );
}

/// Generate the idiv_pown wrappers.
macro_rules! idiv_power {
    ($name:ident, $base:expr) => (
        /// Divide by a power of $base.
        #[inline]
        fn $name(&mut self, n: u32, roundup: bool) {
            self.idiv_power_impl($base, n, roundup)
        }
    );
}

// TRAITS
// ------

/// Traits for shared operations for big integers.
///
/// None of these are implemented using normal traits, since these
/// are very expensive operations, and we want to deliberately
/// and explicitly use these functions.
pub(in atof::algorithm) trait SharedOps: Clone + Sized + Default {
    /// Underlying storage type for a SmallOps.
    type StorageType: CloneableVecLike<u32>;

    // DATA

    /// Get access to the underlying data
    fn data<'a>(&'a self) -> &'a Self::StorageType;

    /// Get access to the underlying data
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType;

    // RELATIVE OPERATIONS

    /// Check if self is greater than y.
    #[inline]
    fn greater(&self, y: &Self) -> bool {
        large::greater(self.data(), y.data())
    }

    /// Check if self is greater than or equal to y.
    #[inline]
    fn greater_equal(&self, y: &Self) -> bool {
        large::greater_equal(self.data(), y.data())
    }

    /// Check if self is less than y.
    #[inline]
    fn less(&self, y: &Self) -> bool {
        large::less(self.data(), y.data())
    }

    /// Check if self is less than or equal to y.
    #[inline]
    fn less_equal(&self, y: &Self) -> bool {
        large::less_equal(self.data(), y.data())
    }

    /// Check if self is equal to y.
    #[inline]
    fn equal(&self, y: &Self) -> bool {
        large::equal(self.data(), y.data())
    }

    // PROPERTIES

    /// Get the number of leading zero values in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn leading_zero_values(&self) -> usize {
        small::leading_zero_values(self.data())
    }

    /// Get the number of trailing zero values in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn trailing_zero_values(&self) -> usize {
        small::trailing_zero_values(self.data())
    }

    /// Get number of leading zero bits in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn leading_zeros(&self) -> usize {
        small::leading_zeros(self.data())
    }

    /// Get number of trailing zero bits in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn trailing_zeros(&self) -> usize {
        small::trailing_zeros(self.data())
    }

    /// Pad the buffer with zeros to the least-significant bits.
    fn pad_zeros(&mut self, n: usize) -> usize {
        small::ishl_digits(self.data_mut(), n);
        n
    }

    // INTEGER CONVERSIONS

    /// Split u64 into two consecutive u32s, in little-endian order.
    #[inline]
    fn split_u64(x: u64) -> (u32, u32) {
        let d0 = (x >> 32) as u32;
        let d1 = (x & u64::LOMASK) as u32;
        (d1, d0)
    }

    /// Split u128 into four consecutive u32s, in little-endian order.
    #[inline]
    fn split_u128(x: u128) -> (u32, u32, u32, u32) {
        let hi64 = (x >> 64) as u64;
        let lo64 = (x & u128::LOMASK) as u64;
        let d3 = (lo64 & u64::LOMASK) as u32;
        let d2 = (lo64 >> 32) as u32;
        let d1 = (hi64 & u64::LOMASK) as u32;
        let d0 = (hi64 >> 32) as u32;
        (d3, d2, d1, d0)
    }

    // CREATION

    /// Create new big integer from u32.
    #[inline]
    fn from_u32(x: u32) -> Self {
        let mut v = Self::default();
        v.data_mut().reserve(1);
        v.data_mut().push(x);
        v.normalize();
        v
    }

    /// Create new big integer from u64.
    #[inline]
    fn from_u64(x: u64) -> Self {
        let mut v = Self::default();
        let (d1, d0) = Self::split_u64(x);
        v.data_mut().reserve(2);
        v.data_mut().push(d1);
        v.data_mut().push(d0);
        v.normalize();
        v
    }

    /// Create new big integer from u128.
    #[inline]
    fn from_u128(x: u128) -> Self {
        let mut v = Self::default();
        let (d3, d2, d1, d0) = Self::split_u128(x);
        v.data_mut().reserve(4);
        v.data_mut().push(d3);
        v.data_mut().push(d2);
        v.data_mut().push(d1);
        v.data_mut().push(d0);
        v.normalize();
        v
    }

    // NORMALIZE

    /// Normalize the integer, so any leading zero values are removed.
    #[inline]
    fn normalize(&mut self) {
        small::normalize(self.data_mut());
    }

    /// Get if the big integer is normalized.
    #[inline]
    fn is_normalized(&self) -> bool {
        unsafe {
            self.data().is_empty() || !self.data().back_unchecked().is_zero()
        }
    }

    // SHIFTS

    /// Shift-left the entire buffer n bits.
    #[inline]
    fn ishl(&mut self, n: usize) {
        small::ishl(self.data_mut(), n);
    }

    /// Shift-left the entire buffer n bits.
    fn shl(&self, n: usize) -> Self {
        let mut x = self.clone();
        x.ishl(n);
        x
    }

    /// Shift-right the entire buffer n bits.
    fn ishr(&mut self, n: usize, mut roundup: bool) {
        roundup &= small::ishr(self.data_mut(), n);

        // Round-up the least significant bit.
        if roundup {
            if self.data().is_empty() {
                self.data_mut().push(1);
            } else {
                unsafe {
                    *self.data_mut().front_unchecked_mut() += 1;
                }
            }
        }
    }

    /// Shift-right the entire buffer n bits.
    fn shr(&self, n: usize, roundup: bool) -> Self {
        let mut x = self.clone();
        x.ishr(n, roundup);
        x
    }

    // BITLENGTH

    /// Calculate the bit-length of the big-integer.
    /// Returns usize::max_value() if the value overflows,
    /// IE, if `self.data().len() > usize::max_value() / 8`.
    fn bit_length(&self) -> usize {
        small::bit_length(self.data())
    }
}

/// Trait for small operations for arbitrary-precision numbers.
pub(in atof::algorithm) trait SmallOps: SharedOps {
    // SMALL POWERS

    /// Get the small powers from the base.
    #[inline]
    fn small_powers(base: u32) -> &'static [u32] {
        get_small_powers(base)
    }

    // ADDITION

    /// AddAssign small integer.
    #[inline]
    fn iadd_small(&mut self, y: u32) {
        small::iadd(self.data_mut(), y);
    }

    /// Add small integer to a copy of self.
    #[inline]
    fn add_small(&self, y: u32) -> Self {
        let mut x = self.clone();
        x.iadd_small(y);
        x
    }

    // SUBTRACTION

    /// SubAssign small integer.
    /// Warning: Does no overflow checking, x must be >= y.
    #[inline]
    unsafe fn isub_small(&mut self, y: u32) {
        small::isub(self.data_mut(), y);
    }

    /// Sub small integer to a copy of self.
    /// Warning: Does no overflow checking, x must be >= y.
    #[inline]
    unsafe fn sub_small(&mut self, y: u32) -> Self {
        let mut x = self.clone();
        x.isub_small(y);
        x
    }

    // MULTIPLICATION

    /// MulAssign small integer.
    #[inline]
    fn imul_small(&mut self, y: u32) {
        small::imul(self.data_mut(), y);
    }

    /// Mul small integer to a copy of self.
    #[inline]
    fn mul_small(&self, y: u32) -> Self {
        let mut x = self.clone();
        x.imul_small(y);
        x
    }

    /// MulAssign by a power.
    #[inline]
    fn imul_power_impl(&mut self, base: u32, n: u32) {
        small::imul_power(self.data_mut(), base, n);
    }

    fn imul_power(&mut self, n: u32, base: u32) {
        match base {
            2  => self.imul_pow2(n),
            3  => self.imul_pow3(n),
            4  => self.imul_pow4(n),
            5  => self.imul_pow5(n),
            6  => self.imul_pow6(n),
            7  => self.imul_pow7(n),
            8  => self.imul_pow8(n),
            9  => self.imul_pow9(n),
            10 => self.imul_pow10(n),
            11 => self.imul_pow11(n),
            12 => self.imul_pow12(n),
            13 => self.imul_pow13(n),
            14 => self.imul_pow14(n),
            15 => self.imul_pow15(n),
            16 => self.imul_pow16(n),
            17 => self.imul_pow17(n),
            18 => self.imul_pow18(n),
            19 => self.imul_pow19(n),
            20 => self.imul_pow20(n),
            21 => self.imul_pow21(n),
            22 => self.imul_pow22(n),
            23 => self.imul_pow23(n),
            24 => self.imul_pow24(n),
            25 => self.imul_pow25(n),
            26 => self.imul_pow26(n),
            27 => self.imul_pow27(n),
            28 => self.imul_pow28(n),
            29 => self.imul_pow29(n),
            30 => self.imul_pow30(n),
            31 => self.imul_pow31(n),
            32 => self.imul_pow32(n),
            33 => self.imul_pow33(n),
            34 => self.imul_pow34(n),
            35 => self.imul_pow35(n),
            36 => self.imul_pow36(n),
            _  => unreachable!()
        }
    }

    /// Multiply by a power of 2.
    fn imul_pow2(&mut self, n: u32) {
        self.ishl(n.as_usize())
    }

    imul_power!(imul_pow3, 3);

    /// Multiply by a power of 4.
    #[inline]
    fn imul_pow4(&mut self, n: u32) {
        self.imul_pow2(2*n);
    }

    imul_power!(imul_pow5, 5);

    /// Multiply by a power of 6.
    #[inline]
    fn imul_pow6(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow2(n);
    }

    imul_power!(imul_pow7, 7);

    /// Multiply by a power of 8.
    #[inline]
    fn imul_pow8(&mut self, n: u32) {
        self.imul_pow2(3*n);
    }

    /// Multiply by a power of 9.
    #[inline]
    fn imul_pow9(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow3(n);
    }

    /// Multiply by a power of 10.
    #[inline]
    fn imul_pow10(&mut self, n: u32) {
        self.imul_pow5(n);
        self.imul_pow2(n);
    }

    imul_power!(imul_pow11, 11);

    /// Multiply by a power of 12.
    #[inline]
    fn imul_pow12(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow4(n);
    }

    imul_power!(imul_pow13, 13);

    /// Multiply by a power of 14.
    #[inline]
    fn imul_pow14(&mut self, n: u32) {
        self.imul_pow7(n);
        self.imul_pow2(n);
    }

    /// Multiply by a power of 15.
    #[inline]
    fn imul_pow15(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow5(n);
    }

    /// Multiply by a power of 16.
    #[inline]
    fn imul_pow16(&mut self, n: u32) {
        self.imul_pow2(4*n);
    }

    imul_power!(imul_pow17, 17);

    /// Multiply by a power of 18.
    #[inline]
    fn imul_pow18(&mut self, n: u32) {
        self.imul_pow9(n);
        self.imul_pow2(n);
    }

    imul_power!(imul_pow19, 19);

    /// Multiply by a power of 20.
    #[inline]
    fn imul_pow20(&mut self, n: u32) {
        self.imul_pow5(n);
        self.imul_pow4(n);
    }

    /// Multiply by a power of 21.
    #[inline]
    fn imul_pow21(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow7(n);
    }

    /// Multiply by a power of 22.
    #[inline]
    fn imul_pow22(&mut self, n: u32) {
        self.imul_pow11(n);
        self.imul_pow2(n);
    }

    imul_power!(imul_pow23, 23);

    /// Multiply by a power of 24.
    #[inline]
    fn imul_pow24(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow8(n);
    }

    /// Multiply by a power of 25.
    #[inline]
    fn imul_pow25(&mut self, n: u32) {
        self.imul_pow5(n);
        self.imul_pow5(n);
    }

    /// Multiply by a power of 26.
    #[inline]
    fn imul_pow26(&mut self, n: u32) {
        self.imul_pow13(n);
        self.imul_pow2(n);
    }

    /// Multiply by a power of 27.
    #[inline]
    fn imul_pow27(&mut self, n: u32) {
        self.imul_pow9(n);
        self.imul_pow3(n);
    }

    /// Multiply by a power of 28.
    #[inline]
    fn imul_pow28(&mut self, n: u32) {
        self.imul_pow7(n);
        self.imul_pow4(n);
    }

    imul_power!(imul_pow29, 29);

    /// Multiply by a power of 30.
    #[inline]
    fn imul_pow30(&mut self, n: u32) {
        self.imul_pow15(n);
        self.imul_pow2(n);
    }

    imul_power!(imul_pow31, 31);

    /// Multiply by a power of 32.
    #[inline]
    fn imul_pow32(&mut self, n: u32) {
        self.imul_pow2(5*n);
    }

    /// Multiply by a power of 33.
    #[inline]
    fn imul_pow33(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow11(n);
    }

    /// Multiply by a power of 34.
    #[inline]
    fn imul_pow34(&mut self, n: u32) {
        self.imul_pow17(n);
        self.imul_pow2(n);
    }

    /// Multiply by a power of 35.
    #[inline]
    fn imul_pow35(&mut self, n: u32) {
        self.imul_pow5(n);
        self.imul_pow7(n);
    }

    /// Multiply by a power of 36.
    #[inline]
    fn imul_pow36(&mut self, n: u32) {
        self.imul_pow9(n);
        self.imul_pow4(n);
    }

    // DIVISION

    /// DivAssign small integer, and return the remainder.
    #[inline]
    fn idiv_small(&mut self, y: u32) -> u32 {
        small::idiv(self.data_mut(), y)
    }

    /// Div small integer to a copy of self, and return the remainder.
    #[inline]
    fn div_small(&self, y: u32) -> (Self, u32) {
        let mut x = self.clone();
        let rem = x.idiv_small(y);
        (x, rem)
    }

    /// Implied divAssign by a power.
    #[inline]
    fn idiv_power_impl(&mut self, base: u32, n: u32, roundup: bool) {
        small::idiv_power(self.data_mut(), base, n, roundup);
    }

    /// DivAssign by a power.
    fn idiv_power(&mut self, n: u32, base: u32, roundup: bool) {
        match base {
            2  => self.idiv_pow2(n, roundup),
            3  => self.idiv_pow3(n, roundup),
            4  => self.idiv_pow4(n, roundup),
            5  => self.idiv_pow5(n, roundup),
            6  => self.idiv_pow6(n, roundup),
            7  => self.idiv_pow7(n, roundup),
            8  => self.idiv_pow8(n, roundup),
            9  => self.idiv_pow9(n, roundup),
            10 => self.idiv_pow10(n, roundup),
            11 => self.idiv_pow11(n, roundup),
            12 => self.idiv_pow12(n, roundup),
            13 => self.idiv_pow13(n, roundup),
            14 => self.idiv_pow14(n, roundup),
            15 => self.idiv_pow15(n, roundup),
            16 => self.idiv_pow16(n, roundup),
            17 => self.idiv_pow17(n, roundup),
            18 => self.idiv_pow18(n, roundup),
            19 => self.idiv_pow19(n, roundup),
            20 => self.idiv_pow20(n, roundup),
            21 => self.idiv_pow21(n, roundup),
            22 => self.idiv_pow22(n, roundup),
            23 => self.idiv_pow23(n, roundup),
            24 => self.idiv_pow24(n, roundup),
            25 => self.idiv_pow25(n, roundup),
            26 => self.idiv_pow26(n, roundup),
            27 => self.idiv_pow27(n, roundup),
            28 => self.idiv_pow28(n, roundup),
            29 => self.idiv_pow29(n, roundup),
            30 => self.idiv_pow30(n, roundup),
            31 => self.idiv_pow31(n, roundup),
            32 => self.idiv_pow32(n, roundup),
            33 => self.idiv_pow33(n, roundup),
            34 => self.idiv_pow34(n, roundup),
            35 => self.idiv_pow35(n, roundup),
            36 => self.idiv_pow36(n, roundup),
            _  => unreachable!()
        }
    }

    /// Divide by a power of 2.
    fn idiv_pow2(&mut self, n: u32, roundup: bool) {
        self.ishr(n.as_usize(), roundup)
    }

    idiv_power!(idiv_pow3, 3);

    /// Divide by a power of 4.
    #[inline]
    fn idiv_pow4(&mut self, n: u32, roundup: bool) {
        self.idiv_pow2(2*n, roundup);
    }

    idiv_power!(idiv_pow5, 5);

    /// Divide by a power of 6.
    #[inline]
    fn idiv_pow6(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    idiv_power!(idiv_pow7, 7);

    /// Divide by a power of 8.
    #[inline]
    fn idiv_pow8(&mut self, n: u32, roundup: bool) {
        self.idiv_pow2(3*n, roundup);
    }

    /// Divide by a power of 9.
    #[inline]
    fn idiv_pow9(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow3(n, roundup);
    }

    /// Divide by a power of 10.
    #[inline]
    fn idiv_pow10(&mut self, n: u32, roundup: bool) {
        self.idiv_pow5(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    idiv_power!(idiv_pow11, 11);

    /// Divide by a power of 12.
    #[inline]
    fn idiv_pow12(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow4(n, roundup);
    }

    idiv_power!(idiv_pow13, 13);

    /// Divide by a power of 14.
    #[inline]
    fn idiv_pow14(&mut self, n: u32, roundup: bool) {
        self.idiv_pow7(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    /// Divide by a power of 15.
    #[inline]
    fn idiv_pow15(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow5(n, roundup);
    }

    /// Divide by a power of 16.
    #[inline]
    fn idiv_pow16(&mut self, n: u32, roundup: bool) {
        self.idiv_pow2(4*n, roundup);
    }

    idiv_power!(idiv_pow17, 17);

    /// Divide by a power of 18.
    #[inline]
    fn idiv_pow18(&mut self, n: u32, roundup: bool) {
        self.idiv_pow9(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    idiv_power!(idiv_pow19, 19);

    /// Divide by a power of 20.
    #[inline]
    fn idiv_pow20(&mut self, n: u32, roundup: bool) {
        self.idiv_pow5(n, roundup);
        self.idiv_pow4(n, roundup);
    }

    /// Divide by a power of 21.
    #[inline]
    fn idiv_pow21(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow7(n, roundup);
    }

    /// Divide by a power of 22.
    #[inline]
    fn idiv_pow22(&mut self, n: u32, roundup: bool) {
        self.idiv_pow11(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    idiv_power!(idiv_pow23, 23);

    /// Divide by a power of 24.
    #[inline]
    fn idiv_pow24(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow8(n, roundup);
    }

    /// Divide by a power of 25.
    #[inline]
    fn idiv_pow25(&mut self, n: u32, roundup: bool) {
        self.idiv_pow5(n, roundup);
        self.idiv_pow5(n, roundup);
    }

    /// Divide by a power of 26.
    #[inline]
    fn idiv_pow26(&mut self, n: u32, roundup: bool) {
        self.idiv_pow13(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    /// Divide by a power of 27.
    #[inline]
    fn idiv_pow27(&mut self, n: u32, roundup: bool) {
        self.idiv_pow9(n, roundup);
        self.idiv_pow3(n, roundup);
    }

    /// Divide by a power of 28.
    #[inline]
    fn idiv_pow28(&mut self, n: u32, roundup: bool) {
        self.idiv_pow7(n, roundup);
        self.idiv_pow4(n, roundup);
    }

    idiv_power!(idiv_pow29, 29);

    /// Divide by a power of 30.
    #[inline]
    fn idiv_pow30(&mut self, n: u32, roundup: bool) {
        self.idiv_pow15(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    idiv_power!(idiv_pow31, 31);

    /// Divide by a power of 32.
    #[inline]
    fn idiv_pow32(&mut self, n: u32, roundup: bool) {
        self.idiv_pow2(5*n, roundup);
    }

    /// Divide by a power of 33.
    #[inline]
    fn idiv_pow33(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow11(n, roundup);
    }

    /// Divide by a power of 34.
    #[inline]
    fn idiv_pow34(&mut self, n: u32, roundup: bool) {
        self.idiv_pow17(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    /// Divide by a power of 35.
    #[inline]
    fn idiv_pow35(&mut self, n: u32, roundup: bool) {
        self.idiv_pow5(n, roundup);
        self.idiv_pow7(n, roundup);
    }

    /// Divide by a power of 36.
    #[inline]
    fn idiv_pow36(&mut self, n: u32, roundup: bool) {
        self.idiv_pow9(n, roundup);
        self.idiv_pow4(n, roundup);
    }

    // POWER

    /// Calculate self^n
    #[inline]
    fn ipow(&mut self, n: u32) {
        small::ipow(self.data_mut(), n);
    }

    /// Calculate self^n
    #[inline]
    fn pow(&self, n: u32) -> Self {
        let mut x = self.clone();
        x.ipow(n);
        x
    }
}

/// Trait for large operations for arbitrary-precision numbers.
pub(in atof::algorithm) trait LargeOps: SmallOps {
    // ADDITION

    /// AddAssign large integer.
    #[inline]
    fn iadd_large(&mut self, y: &Self) {
        large::iadd(self.data_mut(), y.data());
    }

    /// Add large integer to a copy of self.
    #[inline]
    fn add_small(&mut self, y: &Self) -> Self {
        let mut x = self.clone();
        x.iadd_large(y);
        x
    }

    // SUBTRACTION

    /// SubAssign large integer.
    /// Warning: Does no overflow checking, x must be >= y.
    #[inline]
    unsafe fn isub_large(&mut self, y: &Self) {
        large::isub(self.data_mut(), y.data());
    }

    /// Sub large integer to a copy of self.
    /// Warning: Does no overflow checking, x must be >= y.
    #[inline]
    unsafe fn sub_small(&mut self, y: &Self) -> Self {
        let mut x = self.clone();
        x.isub_large(y);
        x
    }

    // MULTIPLICATION

    /// MulAssign large integer.
    #[inline]
    fn imul_large(&mut self, y: &Self) {
        large::imul(self.data_mut(), y.data());
    }

    /// Mul large integer to a copy of self.
    #[inline]
    fn mul_small(&mut self, y: &Self) -> Self {
        let mut x = self.clone();
        x.imul_large(y);
        x
    }

    // DIVISION

    /// DivAssign large integer and get remainder.
    #[inline]
    fn idiv_large(&mut self, y: &Self) -> Self {
        let mut rem = Self::default();
        *rem.data_mut() = large::idiv(self.data_mut(), y.data());
        rem
    }

    /// Div large integer to a copy of self and get quotient and remainder.
    #[inline]
    fn div_small(&mut self, y: &Self) -> (Self, Self) {
        let mut x = self.clone();
        let rem = x.idiv_large(y);
        (x, rem)
    }

    /// Calculate the fast quotient for a single 32-bit quotient.
    ///
    /// This requires a non-normalized divisor, where there at least
    /// `integral_binary_factor` 0 bits set, to ensure at maximum a single
    /// digit will be produced for a single base.
    ///
    /// Warning: This is not a general-purpose division algorithm,
    /// it is highly specialized for peeling off singular digits.
    #[inline]
    unsafe fn quorem(&mut self, y: &Self) -> u32 {
        large::quorem(self.data_mut(), y.data())
    }
}

#[cfg(test)]
mod tests {
    use lib::Vec;
    use super::*;

    #[derive(Clone, Default)]
    struct Bigint {
        data: Vec<u32>,
    }

    impl Bigint {
        #[inline]
        pub fn new() -> Bigint {
            Bigint { data: vec![] }
        }
    }

    impl SharedOps for Bigint {
        type StorageType = Vec<u32>;

        #[inline]
        fn data<'a>(&'a self) -> &'a Self::StorageType {
            &self.data
        }

        #[inline]
        fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType {
            &mut self.data
        }
    }

    impl SmallOps for Bigint {
    }

    impl LargeOps for Bigint {
    }

    // SHARED OPS

    #[test]
    fn greater_test() {
        // Simple
        let x = Bigint { data: vec![1] };
        let y = Bigint { data: vec![2] };
        assert!(!x.greater(&y));
        assert!(!x.greater(&x));
        assert!(y.greater(&x));

        // Check asymmetric
        let x = Bigint { data: vec![5, 1] };
        let y = Bigint { data: vec![2] };
        assert!(x.greater(&y));
        assert!(!x.greater(&x));
        assert!(!y.greater(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint { data: vec![5, 1, 9] };
        let y = Bigint { data: vec![6, 2, 8] };
        assert!(x.greater(&y));
        assert!(!x.greater(&x));
        assert!(!y.greater(&x));
    }

    #[test]
    fn greater_equal_test() {
        // Simple
        let x = Bigint { data: vec![1] };
        let y = Bigint { data: vec![2] };
        assert!(!x.greater_equal(&y));
        assert!(x.greater_equal(&x));
        assert!(y.greater_equal(&x));

        // Check asymmetric
        let x = Bigint { data: vec![5, 1] };
        let y = Bigint { data: vec![2] };
        assert!(x.greater_equal(&y));
        assert!(x.greater_equal(&x));
        assert!(!y.greater_equal(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint { data: vec![5, 1, 9] };
        let y = Bigint { data: vec![6, 2, 8] };
        assert!(x.greater_equal(&y));
        assert!(x.greater_equal(&x));
        assert!(!y.greater_equal(&x));
    }

    #[test]
    fn equal_test() {
        // Simple
        let x = Bigint { data: vec![1] };
        let y = Bigint { data: vec![2] };
        assert!(!x.equal(&y));
        assert!(x.equal(&x));
        assert!(!y.equal(&x));

        // Check asymmetric
        let x = Bigint { data: vec![5, 1] };
        let y = Bigint { data: vec![2] };
        assert!(!x.equal(&y));
        assert!(x.equal(&x));
        assert!(!y.equal(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint { data: vec![5, 1, 9] };
        let y = Bigint { data: vec![6, 2, 8] };
        assert!(!x.equal(&y));
        assert!(x.equal(&x));
        assert!(!y.equal(&x));
    }

    #[test]
    fn less_test() {
        // Simple
        let x = Bigint { data: vec![1] };
        let y = Bigint { data: vec![2] };
        assert!(x.less(&y));
        assert!(!x.less(&x));
        assert!(!y.less(&x));

        // Check asymmetric
        let x = Bigint { data: vec![5, 1] };
        let y = Bigint { data: vec![2] };
        assert!(!x.less(&y));
        assert!(!x.less(&x));
        assert!(y.less(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint { data: vec![5, 1, 9] };
        let y = Bigint { data: vec![6, 2, 8] };
        assert!(!x.less(&y));
        assert!(!x.less(&x));
        assert!(y.less(&x));
    }

    #[test]
    fn less_equal_test() {
        // Simple
        let x = Bigint { data: vec![1] };
        let y = Bigint { data: vec![2] };
        assert!(x.less_equal(&y));
        assert!(x.less_equal(&x));
        assert!(!y.less_equal(&x));

        // Check asymmetric
        let x = Bigint { data: vec![5, 1] };
        let y = Bigint { data: vec![2] };
        assert!(!x.less_equal(&y));
        assert!(x.less_equal(&x));
        assert!(y.less_equal(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint { data: vec![5, 1, 9] };
        let y = Bigint { data: vec![6, 2, 8] };
        assert!(!x.less_equal(&y));
        assert!(x.less_equal(&x));
        assert!(y.less_equal(&x));
    }

    #[test]
    fn leading_zero_values_test() {
        assert_eq!(Bigint::new().leading_zero_values(), 0);

        assert_eq!(Bigint::from_u32(0xFF).leading_zero_values(), 0);
        assert_eq!(Bigint::from_u64(0xFF00000000).leading_zero_values(), 0);
        assert_eq!(Bigint::from_u128(0xFF000000000000000000000000).leading_zero_values(), 0);

        assert_eq!(Bigint::from_u32(0xF).leading_zero_values(), 0);
        assert_eq!(Bigint::from_u64(0xF00000000).leading_zero_values(), 0);
        assert_eq!(Bigint::from_u128(0xF000000000000000000000000).leading_zero_values(), 0);

        assert_eq!(Bigint::from_u32(0xF0).leading_zero_values(), 0);
        assert_eq!(Bigint::from_u64(0xF000000000).leading_zero_values(), 0);
        assert_eq!(Bigint::from_u128(0xF0000000000000000000000000).leading_zero_values(), 0);
    }

    #[test]
    fn trailing_zero_values_test() {
        assert_eq!(Bigint::new().trailing_zero_values(), 0);

        assert_eq!(Bigint::from_u32(0xFF).trailing_zero_values(), 0);
        assert_eq!(Bigint::from_u64(0xFF00000000).trailing_zero_values(), 1);
        assert_eq!(Bigint::from_u128(0xFF000000000000000000000000).trailing_zero_values(), 3);

        assert_eq!(Bigint::from_u32(0xF).trailing_zero_values(), 0);
        assert_eq!(Bigint::from_u64(0xF00000000).trailing_zero_values(), 1);
        assert_eq!(Bigint::from_u128(0xF000000000000000000000000).trailing_zero_values(), 3);

        assert_eq!(Bigint::from_u32(0xF0).trailing_zero_values(), 0);
        assert_eq!(Bigint::from_u64(0xF000000000).trailing_zero_values(), 1);
        assert_eq!(Bigint::from_u128(0xF0000000000000000000000000).trailing_zero_values(), 3);
    }

    #[test]
    fn leading_zeros_test() {
        assert_eq!(Bigint::new().leading_zeros(), 0);

        assert_eq!(Bigint::from_u32(0xFF).leading_zeros(), 24);
        assert_eq!(Bigint::from_u64(0xFF00000000).leading_zeros(), 24);
        assert_eq!(Bigint::from_u128(0xFF000000000000000000000000).leading_zeros(), 24);

        assert_eq!(Bigint::from_u32(0xF).leading_zeros(), 28);
        assert_eq!(Bigint::from_u64(0xF00000000).leading_zeros(), 28);
        assert_eq!(Bigint::from_u128(0xF000000000000000000000000).leading_zeros(), 28);

        assert_eq!(Bigint::from_u32(0xF0).leading_zeros(), 24);
        assert_eq!(Bigint::from_u64(0xF000000000).leading_zeros(), 24);
        assert_eq!(Bigint::from_u128(0xF0000000000000000000000000).leading_zeros(), 24);
    }

    #[test]
    fn trailing_zeros_test() {
        assert_eq!(Bigint::new().trailing_zeros(), 0);

        assert_eq!(Bigint::from_u32(0xFF).trailing_zeros(), 0);
        assert_eq!(Bigint::from_u64(0xFF00000000).trailing_zeros(), 32);
        assert_eq!(Bigint::from_u128(0xFF000000000000000000000000).trailing_zeros(), 96);

        assert_eq!(Bigint::from_u32(0xF).trailing_zeros(), 0);
        assert_eq!(Bigint::from_u64(0xF00000000).trailing_zeros(), 32);
        assert_eq!(Bigint::from_u128(0xF000000000000000000000000).trailing_zeros(), 96);

        assert_eq!(Bigint::from_u32(0xF0).trailing_zeros(), 4);
        assert_eq!(Bigint::from_u64(0xF000000000).trailing_zeros(), 36);
        assert_eq!(Bigint::from_u128(0xF0000000000000000000000000).trailing_zeros(), 100);
    }

    #[test]
    fn pad_zeros_test() {
        let mut x = Bigint { data: vec![0, 0, 0, 1] };
        x.pad_zeros(3);
        assert_eq!(x.data, vec![0, 0, 0, 0, 0, 0, 1]);

        let mut x = Bigint { data: vec![1] };
        x.pad_zeros(1);
        assert_eq!(x.data, vec![0, 1]);
    }

    #[test]
    fn shl_test() {
        // Pattern generated via `''.join(["1" +"0"*i for i in range(20)])`
        let mut big = Bigint { data: vec![0xD2210408] };
        big.ishl(5);
        assert_eq!(big.data, vec![0x44208100, 0x1A]);
        big.ishl(32);
        assert_eq!(big.data, vec![0, 0x44208100, 0x1A]);
        big.ishl(27);
        assert_eq!(big.data, vec![0, 0, 0xD2210408]);

        // 96-bits of previous pattern
        let mut big = Bigint { data: vec![0x20020010, 0x8040100, 0xD2210408] };
        big.ishl(5);
        assert_eq!(big.data, vec![0x400200, 0x802004, 0x44208101, 0x1A]);
        big.ishl(32);
        assert_eq!(big.data, vec![0, 0x400200, 0x802004, 0x44208101, 0x1A]);
        big.ishl(27);
        assert_eq!(big.data, vec![0, 0, 0x20020010, 0x8040100, 0xD2210408]);
    }

    #[test]
    fn shr_test() {
        // Simple case.
        let mut big = Bigint { data: vec![0xD2210408] };
        big.ishr(5, false);
        assert_eq!(big.data, vec![0x6910820]);
        big.ishr(27, false);
        assert_eq!(big.data, vec![]);

        // Pattern generated via `''.join(["1" +"0"*i for i in range(20)])`
        let mut big = Bigint { data: vec![0x20020010, 0x8040100, 0xD2210408] };
        big.ishr(5, false);
        assert_eq!(big.data, vec![0x1001000, 0x40402008, 0x6910820]);
        big.ishr(32, false);
        assert_eq!(big.data, vec![0x40402008, 0x6910820]);
        big.ishr(27, false);
        assert_eq!(big.data, vec![0xD2210408]);

        // Check no-roundup with halfway and even
        let mut big = Bigint { data: vec![0xD2210408] };
        big.ishr(3, true);
        assert_eq!(big.data, vec![0x1A442081]);
        big.ishr(1, true);
        assert_eq!(big.data, vec![0xD221040]);

        let mut big = Bigint { data: vec![0xD2210408] };
        big.ishr(4, true);
        assert_eq!(big.data, vec![0xD221040]);

        // Check roundup with halfway and odd
        let mut big = Bigint { data: vec![0xD2210438] };
        big.ishr(3, true);
        assert_eq!(big.data, vec![0x1A442087]);
        big.ishr(1, true);
        assert_eq!(big.data, vec![0xD221044]);

        let mut big = Bigint { data: vec![0xD2210438] };
        big.ishr(5, true);
        assert_eq!(big.data, vec![0x6910822]);
    }

    #[test]
    fn bit_length_test() {
        let x = Bigint { data: vec![0, 0, 0, 1] };
        assert_eq!(x.bit_length(), 97);

        let x = Bigint { data: vec![0, 0, 0, 3] };
        assert_eq!(x.bit_length(), 98);

        let x = Bigint { data: vec![1<<31] };
        assert_eq!(x.bit_length(), 32);
    }

    // SMALL OPS

    #[test]
    fn iadd_small_test() {
        // Overflow check (single)
        // This should set all the internal data values to 0, the top
        // value to (1<<31), and the bottom value to (4>>1).
        // This is because the max_value + 1 leads to all 0s, we set the
        // topmost bit to 1.
        let mut x = Bigint { data: vec![4294967295] };
        x.iadd_small(5);
        assert_eq!(x.data, vec![4, 1]);

        // No overflow, single value
        let mut x = Bigint { data: vec![5] };
        x.iadd_small(7);
        assert_eq!(x.data, vec![12]);

        // Single carry, internal overflow
        let mut x = Bigint::from_u64(0x80000000FFFFFFFF);
        x.iadd_small(7);
        assert_eq!(x.data, vec![6, 0x80000001]);

        // Double carry, overflow
        let mut x = Bigint::from_u64(0xFFFFFFFFFFFFFFFF);
        x.iadd_small(7);
        assert_eq!(x.data, vec![6, 0, 1]);
    }

    #[test]
    fn isub_small_test() {
        unsafe {
            // Overflow check (single)
            let mut x = Bigint { data: vec![4, 1] };
            x.isub_small(5);
            assert_eq!(x.data, vec![4294967295]);

            // No overflow, single value
            let mut x = Bigint { data: vec![12] };
            x.isub_small(7);
            assert_eq!(x.data, vec![5]);

            // Single carry, internal overflow
            let mut x = Bigint { data: vec![6, 0x80000001] };
            x.isub_small(7);
            assert_eq!(x.data, vec![0xFFFFFFFF, 0x80000000]);

            // Double carry, overflow
            let mut x = Bigint { data: vec![6, 0, 1] };
            x.isub_small(7);
            assert_eq!(x.data, vec![0xFFFFFFFF, 0xFFFFFFFF]);
        }
    }

    #[test]
    fn imul_small_test() {
        // No overflow check, 1-int.
        let mut x = Bigint { data: vec![5] };
        x.imul_small(7);
        assert_eq!(x.data, vec![35]);

        // No overflow check, 2-ints.
        let mut x = Bigint::from_u64(0x4000000040000);
        x.imul_small(5);
        assert_eq!(x.data, vec![0x00140000, 0x140000]);

        // Overflow, 1 carry.
        let mut x = Bigint { data: vec![0x33333334] };
        x.imul_small(5);
        assert_eq!(x.data, vec![4, 1]);

        // Overflow, 1 carry, internal.
        let mut x = Bigint::from_u64(0x133333334);
        x.imul_small(5);
        assert_eq!(x.data, vec![4, 6]);

        // Overflow, 2 carries.
        let mut x = Bigint::from_u64(0x3333333333333334);
        x.imul_small(5);
        assert_eq!(x.data, vec![4, 0, 1]);
    }

    #[test]
    fn idiv_small_test() {
        let mut x = Bigint { data: vec![4] };
        assert_eq!(x.idiv_small(7), 4);
        assert_eq!(x.data, vec![]);

        let mut x = Bigint { data: vec![3] };
        assert_eq!(x.idiv_small(7), 3);
        assert_eq!(x.data, vec![]);

        // Check roundup, odd, halfway
        let mut x = Bigint { data: vec![15] };
        assert_eq!(x.idiv_small(10), 5);
        assert_eq!(x.data, vec![1]);

        // Check 1 carry.
        let mut x = Bigint::from_u64(0x133333334);
        assert_eq!(x.idiv_small(5), 1);
        assert_eq!(x.data, vec![0x3D70A3D7]);

        // Check 2 carries.
        let mut x = Bigint::from_u64(0x3333333333333334);
        assert_eq!(x.idiv_small(5), 4);
        assert_eq!(x.data, vec![0xD70A3D70, 0xA3D70A3]);
    }

    #[test]
    fn ipow_tes() {
        let x = Bigint { data: vec![5] };
        assert_eq!(x.pow(2).data, [25]);
        assert_eq!(x.pow(15).data, [452807053, 7]);
        assert_eq!(x.pow(16).data, [2264035265, 35]);
        assert_eq!(x.pow(17).data, [2730241733, 177]);
        assert_eq!(x.pow(302).data, [2443090281, 2149694430, 2297493928, 1584384001, 1279504719, 1930002239, 3312868939, 3735173465, 3523274756, 2025818732, 1641675015, 2431239749, 4292780461, 3719612855, 4174476133, 3296847770, 2677357556, 638848153, 2198928114, 3285049351, 2159526706, 626302612]);
    }

    // LARGE OPS

    #[test]
    fn iadd_large_test() {
        // Overflow, both single values
        let mut x = Bigint { data: vec![4294967295] };
        let y = Bigint { data: vec![5] };
        x.iadd_large(&y);
        assert_eq!(x.data, vec![4, 1]);

        // No overflow, single value
        let mut x = Bigint { data: vec![5] };
        let y = Bigint { data: vec![7] };
        x.iadd_large(&y);
        assert_eq!(x.data, vec![12]);

        // Single carry, internal overflow
        let mut x = Bigint::from_u64(0x80000000FFFFFFFF);
        let y = Bigint { data: vec![7] };
        x.iadd_large(&y);
        assert_eq!(x.data, vec![6, 0x80000001]);

        // 1st overflows, 2nd doesn't.
        let mut x = Bigint::from_u64(0x7FFFFFFFFFFFFFFF);
        let y = Bigint::from_u64(0x7FFFFFFFFFFFFFFF);
        x.iadd_large(&y);
        assert_eq!(x.data, vec![0xFFFFFFFE, 0xFFFFFFFF]);

        // Both overflow.
        let mut x = Bigint::from_u64(0x8FFFFFFFFFFFFFFF);
        let y = Bigint::from_u64(0x7FFFFFFFFFFFFFFF);
        x.iadd_large(&y);
        assert_eq!(x.data, vec![0xFFFFFFFE, 0x0FFFFFFF, 1]);
    }

    #[test]
    fn isub_large_test() {
        unsafe {
            // Overflow, both single values
            let mut x = Bigint { data: vec![4, 1] };
            let y = Bigint { data: vec![5] };
            x.isub_large(&y);
            assert_eq!(x.data, vec![4294967295]);

            // No overflow, single value
            let mut x = Bigint { data: vec![12] };
            let y = Bigint { data: vec![7] };
            x.isub_large(&y);
            assert_eq!(x.data, vec![5]);

            // Single carry, internal overflow
            let mut x = Bigint { data: vec![6, 0x80000001] };
            let y = Bigint { data: vec![7] };
            x.isub_large(&y);
            assert_eq!(x.data, vec![0xFFFFFFFF, 0x80000000]);

            // Zeros out.
            let mut x = Bigint { data: vec![0xFFFFFFFF, 0x7FFFFFFF] };
            let y = Bigint { data: vec![0xFFFFFFFF, 0x7FFFFFFF] };
            x.isub_large(&y);
            assert_eq!(x.data, vec![]);

            // 1st overflows, 2nd doesn't.
            let mut x = Bigint { data: vec![0xFFFFFFFE, 0x80000000] };
            let y = Bigint { data: vec![0xFFFFFFFF, 0x7FFFFFFF] };
            x.isub_large(&y);
            assert_eq!(x.data, vec![0xFFFFFFFF]);
        }
    }

    #[test]
    fn imul_large_test() {
        // Test by empty
        let mut x = Bigint { data: vec![0xFFFFFFFF] };
        let y = Bigint { data: vec![] };
        x.imul_large(&y);
        assert_eq!(x.data, vec![]);

        // Simple case
        let mut x = Bigint { data: vec![0xFFFFFFFF] };
        let y = Bigint { data: vec![5] };
        x.imul_large(&y);
        assert_eq!(x.data, vec![0xFFFFFFFB, 0x4]);

        // Large u32, but still just as easy.
        let mut x = Bigint { data: vec![0xFFFFFFFF] };
        let y = Bigint { data: vec![0xFFFFFFFE] };
        x.imul_large(&y);
        assert_eq!(x.data, vec![0x2, 0xFFFFFFFD]);

        // Let's multiply two large values together
        let mut x = Bigint { data: vec![0xFFFFFFFE, 0x0FFFFFFF, 1] };
        let y = Bigint { data: vec![0x99999999, 0x99999999, 0xCCCD9999, 0xCCCC] };
        x.imul_large(&y);
        assert_eq!(x.data, vec![0xCCCCCCCE, 0x5CCCCCCC, 0x9997FFFF, 0x33319999, 0x999A7333, 0xD999]);
    }

    #[test]
    fn imul_karatsuba_mul_test() {
        // Test cases triggered to use `karatsuba_mul`.
        let mut x = Bigint { data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] };
        let y = Bigint { data: vec![4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19] };
        x.imul_large(&y);
        assert_eq!(x.data, vec![4, 13, 28, 50, 80, 119, 168, 228, 300, 385, 484, 598, 728, 875, 1040, 1224, 1340, 1435, 1508, 1558, 1584, 1585, 1560, 1508, 1428, 1319, 1180, 1010, 808, 573, 304]);

        // Test cases to use karatsuba_uneven_mul
        let mut x = Bigint { data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] };
        let y = Bigint { data: vec![4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37] };
        x.imul_large(&y);
        assert_eq!(x.data, vec![4, 13, 28, 50, 80, 119, 168, 228, 300, 385, 484, 598, 728, 875, 1040, 1224, 1360, 1496, 1632, 1768, 1904, 2040, 2176, 2312, 2448, 2584, 2720, 2856, 2992, 3128, 3264, 3400, 3536, 3672, 3770, 3829, 3848, 3826, 3762, 3655, 3504, 3308, 3066, 2777, 2440, 2054, 1618, 1131, 592]);
    }

    #[test]
    fn idiv_large_test() {
        // Simple case.
        let mut x = Bigint { data: vec![0xFFFFFFFF] };
        let y = Bigint { data: vec![5] };
        let rem = x.idiv_large(&y);
        assert_eq!(x.data, vec![0x33333333]);
        assert_eq!(rem.data, vec![0]);

        // Two integer case
        let mut x = Bigint { data: vec![0x2, 0xFFFFFFFF] };
        let y = Bigint { data: vec![0xFFFFFFFE] };
        let rem = x.idiv_large(&y);
        assert_eq!(x.data, vec![1, 1]);
        assert_eq!(rem.data, vec![4]);

        // Larger large case
        let mut x = Bigint { data: vec![0xCCCCCCCF, 0x5CCCCCCC, 0x9997FFFF, 0x33319999, 0x999A7333, 0xD999] };
        let y = Bigint { data: vec![0x99999999, 0x99999999, 0xCCCD9999, 0xCCCC] };
        let rem = x.idiv_large(&y);
        assert_eq!(x.data, vec![0xFFFFFFFE, 0x0FFFFFFF, 1]);
        assert_eq!(rem.data, vec![1]);

        // Extremely large cases, examples from Karatsuba multiplication.
        let mut x = Bigint { data: vec![4, 13, 29, 50, 80, 119, 168, 228, 300, 385, 484, 598, 728, 875, 1040, 1224, 1340, 1435, 1508, 1558, 1584, 1585, 1560, 1508, 1428, 1319, 1180, 1010, 808, 573, 304] };
        let y = Bigint { data: vec![4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19] };
        let rem = x.idiv_large(&y);
        assert_eq!(x.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(rem.data, vec![0, 0, 1]);
    }

    #[test]
    fn quorem_test() {
        unsafe {
            let mut x = Bigint::from_u128(42535295865117307932921825928971026432);
            let y = Bigint::from_u128(17218479456385750618067377696052635483);
            assert_eq!(x.quorem(&y), 2);
            assert_eq!(x.data, [1873752394, 3049207402, 3024501058, 102215382]);
        }
    }
}
