//! Building-blocks for arbitrary-precision math.
//!
//! These algorithms assume little-endian order for the large integer
//! buffers, so for a `vec![0, 1, 2, 3]`, `3` is the most significant `u32`,
//! and `0` is the least significant `u32`.

// SCALAR
// ------

// Scalar-to-scalar operations, for building-blocks for arbitrary-precision
// operations.

mod scalar {

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

mod small {

use util::*;
use super::scalar;

// ROUNDUP

/// Round to the nearest value if there's truncation during division.
///
/// If the `2*rem > y`, or `2*rem == y` and the last, then we have a
/// division where the remainder is near the halfway case.
/// digit is odd, round-up (round-nearest, tie-even).
#[inline]
pub fn do_roundup<T: VecLike<u32>>(x: &mut T, y: u32, rem: u32) {
    unsafe {
        let (is_above, is_halfway) = scalar::cmp_remainder(y, rem);
        if is_above || (is_halfway && x.front_unchecked().is_odd()) {
            *x.front_unchecked_mut() += 1;
        }
    }
}

// NORMALIZE

/// Normalize the container by popping any leading zeros.
#[inline]
pub fn normalize<T: VecLike<u32>>(x: &mut T) {
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
pub fn iadd_impl<T: CloneableVecLike<u32>>(x: &mut T, y: u32, xstart: usize) {
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
pub fn imul_power<T>(x: &mut T, mut n: u32, small_powers: &[u32])
    where T: CloneableVecLike<u32>
{
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
pub fn mul_power<T>(x: &[u32], n: u32, small_powers: &[u32])
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    imul_power(&mut z, n, small_powers);
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
pub fn idiv_power<T>(x: &mut T, mut n: u32, small_powers: &[u32], roundup: bool)
    where T: CloneableVecLike<u32>
{
    let get_power = | i: usize | unsafe { *small_powers.get_unchecked(i) };

    // Divide by the largest small power until n < step.
    let step = small_powers.len() - 1;
    let power = get_power(step);
    let step = step as u32;
    while n >= step {
        let rem = idiv(x, power);
        if roundup {
            do_roundup(x, power, rem);
        }
        n -= step;
    }

    // Multiply by the remainder.
    let power = get_power(n as usize);
    let rem = idiv(x, power);
    if roundup {
        do_roundup(x, power, rem);
    }
}

/// Div by a power.
#[allow(dead_code)]
pub fn div_power<T>(x: &[u32], n: u32, small_powers: &[u32], roundup: bool)
    -> T
    where T: CloneableVecLike<u32>
{
    let mut z = T::default();
    z.extend_from_slice(x);
    idiv_power(&mut z, n, small_powers, roundup);
    z
}

}   // small

// LARGE
// -----

// Large-to-large operations, to modify a big integer from a native scalar.

mod large {

use util::*;
use super::{scalar, small};

/// ADDITION

/// Implied AddAssign implementation for bigints.
///
/// Allows us to choose a start-index in x to store, so we can avoid
/// padding the buffer with zeros when not needed, optimized for vectors.
pub fn iadd_impl<T: CloneableVecLike<u32>>(x: &mut T, y: &[u32], xstart: usize) {
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
pub fn iadd<T: CloneableVecLike<u32>>(x: &mut T, y: &[u32]) {
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
pub fn isub<T: CloneableVecLike<u32>>(x: &mut T, y: &[u32]) {
    // Basic underflow checks.
    debug_assert!(x.len() >= y.len());
    debug_assert!(x.len() > y.len() || x[x.len()-1] >= y[y.len()-1]);

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

/// Number of digits to bottom-out to long division.
const KARATSUBA_MIN_DIGITS: usize = 15;

/// Grade-school multiplication algorithm.
///
/// Slow, naive algorithm, using 32-bit bases and just shifting left for
/// each iteration. This could be optimized with numerous other algorithms,
/// but it's extremely simple, and works in O(n*m) time, which is fine
/// by me. Each iteration, of which there are `m` iterations, requires
/// `n` multiplications, and `n` additions, or grade-school multiplication.
fn long_mul<T: CloneableVecLike<u32>>(x: &[u32], y: &[u32]) -> T {
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
fn karatsuba_mul<T>(x: &[u32], y: &[u32]) -> T
    where T: CloneableVecLike<u32>
{
    if y.len() <= KARATSUBA_MIN_DIGITS {
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
#[allow(unused)]
fn karatsuba_uneven_mul<T>(x: &[u32], mut y: &[u32]) -> T
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
fn karatsuba_mul_fwd<T: CloneableVecLike<u32>>(x: &[u32], y: &[u32]) -> T {
    if x.len() < y.len() {
        karatsuba_mul(x, y)
    } else {
        karatsuba_mul(y, x)
    }
}

/// MulAssign bigint to bigint.
#[allow(dead_code)]
#[inline]
pub fn imul<T: CloneableVecLike<u32>>(x: &mut T, y: &[u32]) {
    unsafe {
        if y.len() == 1 {
            small::imul(x, *y.get_unchecked(0));
        } else {
            *x = karatsuba_mul_fwd(x, y);
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

// TODO(ahuszagh) Follow Knuth to implement division.
//  http://www.hackersdelight.org/hdcodetxt/divmnu64.c.txt

/// Implementation of Knuth's Algorithm D.
///
/// Based off the Hacker's Delight implementation of Knuth's Algorithm D
/// in "The Art of Computer Programming".
///     http://www.hackersdelight.org/hdcodetxt/divmnu64.c.txt
///
/// All Hacker's Delight code is public domain, so this routine shall
/// also be placed in the public domain.
// TOOO(ahuszagh) Document more...
#[allow(unused)]    // TODO(ahuszagh) Remove
fn algorithm_d_div<T: CloneableVecLike<u32>>(x: &mut T, y: &[u32], rem: &mut T) {
    unimplemented!()

// TODO(ahuszagh) Adopt this C code.
//    /* q[0], r[0], u[0], and v[0] contain the LEAST significant words.
//    (The sequence is in little-endian order).
//
//    This is a fairly precise implementation of Knuth's Algorithm D, for a
//    binary computer with base b = 2**32. The caller supplies:
//       1. Space q for the quotient, m - n + 1 words (at least one).
//       2. Space r for the remainder (optional), n words.
//       3. The dividend u, m words, m >= 1.
//       4. The divisor v, n words, n >= 2.
//    The most significant digit of the divisor, v[n-1], must be nonzero.  The
//    dividend u may have leading zeros; this just makes the algorithm take
//    longer and makes the quotient contain more leading zeros.  A value of
//    NULL may be given for the address of the remainder to signify that the
//    caller does not want the remainder.
//       The program does not alter the input parameters u and v.
//       The quotient and remainder returned may have leading zeros.  The
//    function itself returns a value of 0 for success and 1 for invalid
//    parameters (e.g., division by 0).
//       For now, we must have m >= n.  Knuth's Algorithm D also requires
//    that the dividend be at least as long as the divisor.  (In his terms,
//    m >= 0 (unstated).  Therefore m+n >= n.) */
//
//    const unsigned long long b = 4294967296LL; // Number base (2**32).
//   unsigned *un, *vn;                         // Normalized form of u, v.
//   unsigned long long qhat;                   // Estimated quotient digit.
//   unsigned long long rhat;                   // A remainder.
//   unsigned long long p;                      // Product of two digits.
//   long long t, k;
//   int s, i, j;
//
//   /* Normalize by shifting v left just enough so that its high-order
//   bit is on, and shift u left the same amount. We may have to append a
//   high-order digit on the dividend; we do that unconditionally. */
//
//   s = nlz(v[n-1]);             // 0 <= s <= 31.
//   vn = (unsigned *)alloca(4*n);
//   for (i = n - 1; i > 0; i--)
//      vn[i] = (v[i] << s) | ((unsigned long long)v[i-1] >> (32-s));
//   vn[0] = v[0] << s;
//
//   un = (unsigned *)alloca(4*(m + 1));
//   un[m] = (unsigned long long)u[m-1] >> (32-s);
//   for (i = m - 1; i > 0; i--)
//      un[i] = (u[i] << s) | ((unsigned long long)u[i-1] >> (32-s));
//   un[0] = u[0] << s;
//
//   for (j = m - n; j >= 0; j--) {       // Main loop.
//      // Compute estimate qhat of q[j].
//      qhat = (un[j+n]*b + un[j+n-1])/vn[n-1];
//      rhat = (un[j+n]*b + un[j+n-1]) - qhat*vn[n-1];
//again:
//      if (qhat >= b || qhat*vn[n-2] > b*rhat + un[j+n-2])
//      { qhat = qhat - 1;
//        rhat = rhat + vn[n-1];
//        if (rhat < b) goto again;
//      }
//
//      // Multiply and subtract.
//      k = 0;
//      for (i = 0; i < n; i++) {
//         p = qhat*vn[i];
//         t = un[i+j] - k - (p & 0xFFFFFFFFLL);
//         un[i+j] = t;
//         k = (p >> 32) - (t >> 32);
//      }
//      t = un[j+n] - k;
//      un[j+n] = t;
//
//      q[j] = qhat;              // Store quotient digit.
//      if (t < 0) {              // If we subtracted too
//         q[j] = q[j] - 1;       // much, add back.
//         k = 0;
//         for (i = 0; i < n; i++) {
//            t = (unsigned long long)un[i+j] + vn[i] + k;
//            un[i+j] = t;
//            k = t >> 32;
//         }
//         un[j+n] = un[j+n] + k;
//      }
//   } // End j.
//   // If the caller wants the remainder, unnormalize
//   // it and pass it back.
//   if (r != NULL) {
//      for (i = 0; i < n-1; i++)
//         r[i] = (un[i] >> s) | ((unsigned long long)un[i+1] << (32-s));
//      r[n-1] = un[n-1] >> s;
//   }
//   return 0;
}

/// DivAssign bigint to bigint.
#[allow(dead_code)]
pub fn idiv<T: CloneableVecLike<u32>>(x: &mut T, y: &[u32]) -> T {
    let mut rem = T::default();
    unsafe {
        if y.len() == 1 {
            // Can optimize for division by a small value.
            rem.push(small::idiv(x, *y.get_unchecked(0)));
        } else if x.len() < y.len() {
            // Can optimize easily, since the quotient is 0,
            // and the remainder is x.
            mem::swap(x, &mut rem);
        } else {
            algorithm_d_div(x, y, &mut rem);
        }
    }

    rem
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

}   // large


use lib::iter;
use float::Mantissa;
use util::*;
use super::small_powers::*;

/// Generate the imul_pown wrappers.
macro_rules! imul_power {
    ($name:ident, $pow:ident, $base:expr) => (
        /// Multiply by a power of $base.
        #[inline]
        fn $name(&mut self, n: u32) {
            self.imul_power_impl(n, &$pow)
        }
    );
}

/// Generate the idiv_pown wrappers.
macro_rules! idiv_power {
    ($name:ident, $pow:ident, $base:expr) => (
        /// Divide by a power of $base.
        #[inline]
        fn $name(&mut self, n: u32, roundup: bool) {
            self.idiv_power_impl(n, &$pow, roundup)
        }
    );
}

// TRAITS
// ------

pub(in atof::algorithm) trait SharedOps: Clone + Sized {
    /// Underlying storage type for a SmallOps.
    type StorageType: CloneableVecLike<u32>;

    // DATA

    /// Get access to the underlying data
    fn data<'a>(&'a self) -> &'a Self::StorageType;

    /// Get access to the underlying data
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType;

    // PROPERTIES

    /// Get the number of leading zero values in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn leading_zero_values(&self) -> u32 {
        0
    }

    /// Get the number of trailing zero values in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn trailing_zero_values(&self) -> u32 {
        let mut iter = self.data().iter().enumerate();
        let opt = iter.find(|&tup| !tup.1.is_zero());
        let value = opt
            .map(|t| t.0)
            .unwrap_or(self.data().len());

        value as u32
    }

    /// Get number of leading zero bits in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn leading_zeros(&self) -> u32 {
        unsafe {
            if self.data().is_empty() {
                0
            } else {
                self.data().back_unchecked().leading_zeros()
            }
        }
    }

    /// Get number of trailing zero bits in the storage.
    /// Assumes the value is normalized.
    #[inline]
    fn trailing_zeros(&self) -> u32 {
        // Get the index of the last non-zero value
        let index: usize = self.trailing_zero_values() as usize;
        let mut count = (index * u32::BITS) as u32;
        if let Some(value) = self.data().get(index) {
            count += value.trailing_zeros();
        }
        count
    }

    /// Pad the buffer with zeros to the least-significant bits.
    fn pad_zeros(&mut self, n: usize) -> usize {
        if self.data().is_empty() {
            n
        } else {
            self.data_mut().insert_many(0, iter::repeat(0).take(n));
            n
        }
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

    // SHL

    /// Shift-left bits < 32 bits with carry.
    #[inline]
    fn ishl_impl(&mut self, n: u32) {
        // Need to shift by the number of `bits % 32`.
        let bits = u32::BITS.as_u32();
        debug_assert!(n < bits && n != 0);

        // Internally, for each item, we shift left by n, and add the previous
        // right shifted 32-bits.
        // For example, we transform (for u8) shifted left 2, to:
        //      b10100100 b01000010
        //      b10 b10010001 b00001000
        let rshift = bits - n;
        let lshift = n;
        let mut prev: u32 = 0;
        for x in self.data_mut().iter_mut() {
            let tmp = *x;
            *x <<= lshift;
            *x |= prev >> rshift;
            prev = tmp;
        }

        let carry = prev >> rshift;
        if carry != 0 {
            self.data_mut().push(carry);
        }
    }

    /// Shift-left the entire buffer n bits.
    fn ishl(&mut self, n: u32) {
        let bits = u32::BITS.as_u32();
        // Need to pad with zeros for the number of `bits / 32`,
        // and shift-left with carry for `bits % 32`.
        let rem = n % bits;
        let div = (n / bits).as_usize();
        if rem != 0 {
            self.ishl_impl(rem);
        }
        if div != 0 {
            self.pad_zeros(div);
        }
    }

    /// Shift-left the entire buffer n bits.
    fn shl(&self, n: u32) -> Self {
        let mut x = self.clone();
        x.ishl(n);
        x
    }

    /// Shift-right < 32 bits with carry.
    #[inline]
    fn ishr_impl(&mut self, n: u32) {
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
        for x in self.data_mut().iter_mut().rev() {
            let tmp = *x;
            *x >>= rshift;
            *x |= prev << lshift;
            prev = tmp;
        }
    }

    /// Check if we need to round-up after shift-right.
    fn ishr_roundup_impl(&self, n: u32) -> bool {
        let bits = u32::BITS.as_u32();
        // Find the bit and index count.
        // We're shifting right that bit, and removing all indexes after it.
        let bit = n % bits;
        let index = (n / bits).as_usize();
        let mask = lower_n_mask(bit);
        let halfway = lower_n_halfway(bit);

        // We already know that the index is valid, from `shr`.
        unsafe {
            debug_assert!(index < self.data().len());
            let digit = *self.data().get_unchecked(index);
            let lower_n = digit & mask;
            if lower_n == halfway {
                // Currently at halfway, check.
                let slc = &self.data()[..index];
                if slc.iter().rev().all(|v| v.is_zero()) {
                    // Absolute halfway, roundup if the bit above is odd.
                    let oddmask = 1 << bit;
                    digit & oddmask == oddmask
                } else {
                    // Above halfway
                    true
                }
            } else if lower_n > halfway {
                // Need to round-up, above halfway
                true
            } else {
                // No need to round-up, below halfway
                false
            }
        }
    }

    /// Shift-right the entire buffer n bits.
    fn ishr(&mut self, n: u32, roundup: bool) {
        let bits = u32::BITS.as_u32();
        // Need to remove the right-most `bits / 32`,
        // and shift-right with carry for `bits % 32`.
        let bit = n % bits;
        let index = (n / bits).as_usize();

        // Clear the buffer if we go over the size of the buffer.
        if index >= self.data().len() {
            unsafe {
                self.data_mut().set_len(0);
            }
            return;
        }

        // Pre-calculate if we need to roundup, then do the operations.
        // First get rid of the previous indexes, so we do less work.
        let roundup = roundup && self.ishr_roundup_impl(n);
        if index != 0 {
            self.data_mut().remove_many(0..index.as_usize());
        }
        if bit != 0 {
            self.ishr_impl(bit);
        }

        unsafe {
            // Round-up the least significant bit.
            if roundup {
                *self.data_mut().front_unchecked_mut() += 1;
            }

            // Pop the most significant byte, as long as it is 0.
            small::normalize(self.data_mut());
        }
    }

    /// Shift-right the entire buffer n bits.
    fn shr(&self, n: u32, roundup: bool) -> Self {
        let mut x = self.clone();
        x.ishr(n, roundup);
        x
    }
}

/// Trait for small operations for arbitrary-precision numbers.
pub(in atof::algorithm) trait SmallOps: SharedOps {
    // SMALL POWERS

    /// Get the small powers from the base.
    #[inline]
    fn small_powers(base: u32) -> &'static [u32] {
        match base {
            2  => &U32_POW2,
            3  => &U32_POW3,
            4  => &U32_POW4,
            5  => &U32_POW5,
            6  => &U32_POW6,
            7  => &U32_POW7,
            8  => &U32_POW8,
            9  => &U32_POW9,
            10 => &U32_POW10,
            11 => &U32_POW11,
            12 => &U32_POW12,
            13 => &U32_POW13,
            14 => &U32_POW14,
            15 => &U32_POW15,
            16 => &U32_POW16,
            17 => &U32_POW17,
            18 => &U32_POW18,
            19 => &U32_POW19,
            20 => &U32_POW20,
            21 => &U32_POW21,
            22 => &U32_POW22,
            23 => &U32_POW23,
            24 => &U32_POW24,
            25 => &U32_POW25,
            26 => &U32_POW26,
            27 => &U32_POW27,
            28 => &U32_POW28,
            29 => &U32_POW29,
            30 => &U32_POW30,
            31 => &U32_POW31,
            32 => &U32_POW32,
            33 => &U32_POW33,
            34 => &U32_POW34,
            35 => &U32_POW35,
            36 => &U32_POW36,
            _  => unreachable!()
        }
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
    fn imul_power_impl(&mut self, n: u32, small_powers: &[u32]) {
        small::imul_power(self.data_mut(), n, small_powers);
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
        self.ishl(n)
    }

    imul_power!(imul_pow3, U32_POW3, 3);

    /// Multiply by a power of 4.
    #[inline]
    fn imul_pow4(&mut self, n: u32) {
        self.imul_pow2(2*n);
    }

    imul_power!(imul_pow5, U32_POW5, 5);

    /// Multiply by a power of 6.
    #[inline]
    fn imul_pow6(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow2(n);
    }

    imul_power!(imul_pow7, U32_POW7, 7);

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

    imul_power!(imul_pow11, U32_POW11, 11);

    /// Multiply by a power of 12.
    #[inline]
    fn imul_pow12(&mut self, n: u32) {
        self.imul_pow3(n);
        self.imul_pow4(n);
    }

    imul_power!(imul_pow13, U32_POW13, 13);

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

    imul_power!(imul_pow17, U32_POW17, 17);

    /// Multiply by a power of 18.
    #[inline]
    fn imul_pow18(&mut self, n: u32) {
        self.imul_pow9(n);
        self.imul_pow2(n);
    }

    imul_power!(imul_pow19, U32_POW19, 19);

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

    imul_power!(imul_pow23, U32_POW23, 23);

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

    imul_power!(imul_pow29, U32_POW29, 29);

    /// Multiply by a power of 30.
    #[inline]
    fn imul_pow30(&mut self, n: u32) {
        self.imul_pow15(n);
        self.imul_pow2(n);
    }

    imul_power!(imul_pow31, U32_POW31, 31);

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
    fn idiv_power_impl(&mut self, n: u32, small_powers: &[u32], roundup: bool) {
        small::idiv_power(self.data_mut(), n, small_powers, roundup);
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
        self.ishr(n, roundup)
    }

    idiv_power!(idiv_pow3, U32_POW3, 3);

    /// Divide by a power of 4.
    #[inline]
    fn idiv_pow4(&mut self, n: u32, roundup: bool) {
        self.idiv_pow2(2*n, roundup);
    }

    idiv_power!(idiv_pow5, U32_POW5, 5);

    /// Divide by a power of 6.
    #[inline]
    fn idiv_pow6(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    idiv_power!(idiv_pow7, U32_POW7, 7);

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

    idiv_power!(idiv_pow11, U32_POW11, 11);

    /// Divide by a power of 12.
    #[inline]
    fn idiv_pow12(&mut self, n: u32, roundup: bool) {
        self.idiv_pow3(n, roundup);
        self.idiv_pow4(n, roundup);
    }

    idiv_power!(idiv_pow13, U32_POW13, 13);

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

    idiv_power!(idiv_pow17, U32_POW17, 17);

    /// Divide by a power of 18.
    #[inline]
    fn idiv_pow18(&mut self, n: u32, roundup: bool) {
        self.idiv_pow9(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    idiv_power!(idiv_pow19, U32_POW19, 19);

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

    idiv_power!(idiv_pow23, U32_POW23, 23);

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

    idiv_power!(idiv_pow29, U32_POW29, 29);

    /// Divide by a power of 30.
    #[inline]
    fn idiv_pow30(&mut self, n: u32, roundup: bool) {
        self.idiv_pow15(n, roundup);
        self.idiv_pow2(n, roundup);
    }

    idiv_power!(idiv_pow31, U32_POW31, 31);

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

    /// DivAssign large integer.
    #[inline]
    fn idiv_large(&mut self, y: &Self) {
        large::idiv(self.data_mut(), y.data());
    }

    /// Div large integer to a copy of self.
    #[inline]
    fn div_small(&mut self, y: &Self) -> Self {
        let mut x = self.clone();
        x.idiv_large(y);
        x
    }
}

#[cfg(test)]
mod tests {
    use lib::Vec;
    use super::*;

    #[derive(Clone)]
    struct Bigint {
        data: Vec<u32>,
    }

    impl Bigint {
        #[inline]
        pub fn new() -> Bigint {
            Bigint { data: vec![] }
        }

        #[inline]
        pub fn from_u32(x: u32) -> Bigint {
            Bigint { data: vec![x] }
        }

        #[inline]
        pub fn from_u64(x: u64) -> Bigint {
            let (d1, d0) = Bigint::split_u64(x);
            Bigint { data: vec![d1, d0] }
        }

        #[inline]
        pub fn from_u128(x: u128) -> Bigint {
            let (d3, d2, d1, d0) = Bigint::split_u128(x);
            Bigint { data: vec![d3, d2, d1, d0] }
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

    // TODO(ahuszagh) Add idiv test
}
