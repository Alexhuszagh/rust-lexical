//! Large-to-large operations.
//!
//! Perform numerical operations between big integers.

use crate::lib::{cmp, mem};
use crate::util::config::*;
use crate::util::traits::*;

use super::cast::*;
use super::scalar;
use super::small;

// RELATIVE OPERATORS
// ------------------

/// Compare `x` to `y`, in little-endian order.
#[inline]
pub fn compare(x: &[Limb], y: &[Limb]) -> cmp::Ordering {
    if x.len() > y.len() {
        return cmp::Ordering::Greater;
    } else if x.len() < y.len() {
        return cmp::Ordering::Less;
    } else {
        let iter = x.iter().rev().zip(y.iter().rev());
        for (&xi, &yi) in iter {
            if xi > yi {
                return cmp::Ordering::Greater;
            } else if xi < yi {
                return cmp::Ordering::Less;
            }
        }
        // Equal case.
        return cmp::Ordering::Equal;
    }
}

/// Check if x is greater than y.
#[inline]
pub fn greater(x: &[Limb], y: &[Limb]) -> bool {
    compare(x, y) == cmp::Ordering::Greater
}

/// Check if x is greater than or equal to y.
#[inline]
pub fn greater_equal(x: &[Limb], y: &[Limb]) -> bool {
    !less(x, y)
}

/// Check if x is less than y.
#[inline]
pub fn less(x: &[Limb], y: &[Limb]) -> bool {
    compare(x, y) == cmp::Ordering::Less
}

/// Check if x is less than or equal to y.
#[inline]
pub fn less_equal(x: &[Limb], y: &[Limb]) -> bool {
    !greater(x, y)
}

/// Check if x is equal to y.
/// Slightly optimized for equality comparisons, since it reduces the number
/// of comparisons relative to `compare`.
#[inline]
pub fn equal(x: &[Limb], y: &[Limb]) -> bool {
    let mut iter = x.iter().rev().zip(y.iter().rev());
    x.len() == y.len() && iter.all(|(&xi, &yi)| xi == yi)
}

// ADDITION
// --------

/// Implied AddAssign implementation for bigints.
///
/// Allows us to choose a start-index in x to store, so we can avoid
/// padding the buffer with zeros when not needed, optimized for vectors.
pub fn iadd_impl<T>(x: &mut T, y: &[Limb], xstart: usize)
where
    T: CloneableVecLike<Limb>,
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
        // Limb::max_value() + Limb::max_value(). Add the previous carry,
        // and store the current carry for the next.
        let mut tmp = scalar::iadd(xi, *yi);
        if carry {
            tmp |= scalar::iadd(xi, 1);
        }
        carry = tmp;
    }

    // Overflow from the previous bit.
    if carry {
        small::iadd_impl(x, 1, y.len() + xstart);
    }
}

/// AddAssign bigint to bigint.
#[inline]
pub fn iadd<T>(x: &mut T, y: &[Limb])
where
    T: CloneableVecLike<Limb>,
{
    iadd_impl(x, y, 0)
}

/// Add bigint to bigint.
#[inline]
pub fn add<T>(x: &[Limb], y: &[Limb]) -> T
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    iadd(&mut z, y);
    z
}

// SUBTRACTION
// -----------

/// SubAssign bigint to bigint.
pub fn isub<T>(x: &mut T, y: &[Limb])
where
    T: CloneableVecLike<Limb>,
{
    // Basic underflow checks.
    debug_assert!(greater_equal(x, y));

    // Iteratively add elements from y to x.
    let mut carry = false;
    for (xi, yi) in x.iter_mut().zip(y.iter()) {
        // Only one op of the two can overflow, since we added at max
        // Limb::max_value() + Limb::max_value(). Add the previous carry,
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
#[inline]
#[allow(dead_code)]
pub fn sub<T>(x: &[Limb], y: &[Limb]) -> T
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    isub(&mut z, y);
    z
}


// MULTIPLICATIION
// ---------------

/// Number of digits to bottom-out to asymptotically slow algorithms.
///
/// Karatsuba tends to out-perform long-multiplication at ~320-640 bits,
/// so we go halfway, while Newton division tends to out-perform
/// Algorithm D at ~1024 bits. We can toggle this for optimal performance.
pub const KARATSUBA_CUTOFF: usize = 32;

/// Grade-school multiplication algorithm.
///
/// Slow, naive algorithm, using limb-bit bases and just shifting left for
/// each iteration. This could be optimized with numerous other algorithms,
/// but it's extremely simple, and works in O(n*m) time, which is fine
/// by me. Each iteration, of which there are `m` iterations, requires
/// `n` multiplications, and `n` additions, or grade-school multiplication.
fn long_mul<T>(x: &[Limb], y: &[Limb]) -> T
where
    T: CloneableVecLike<Limb>,
{
    // Using the immutable value, multiply by all the scalars in y, using
    // the algorithm defined above. Use a single buffer to avoid
    // frequent reallocations. Handle the first case to avoid a redundant
    // addition, since we know y.len() >= 1.
    let mut z: T = small::mul(x, y[0]);
    z.resize(x.len() + y.len(), 0);

    // Handle the iterative cases.
    for (i, &yi) in y[1..].iter().enumerate() {
        let zi: T = small::mul(x, yi);
        iadd_impl(&mut z, &zi, i + 1);
    }

    small::normalize(&mut z);

    z
}

/// Split two buffers into halfway, into (lo, hi).
#[inline]
pub fn karatsuba_split<'a>(z: &'a [Limb], m: usize) -> (&'a [Limb], &'a [Limb]) {
    (&z[..m], &z[m..])
}

/// Karatsuba multiplication algorithm with roughly equal input sizes.
///
/// Assumes `y.len() >= x.len()`.
fn karatsuba_mul<T>(x: &[Limb], y: &[Limb]) -> T
where
    T: CloneableVecLike<Limb>,
{
    if y.len() <= KARATSUBA_CUTOFF {
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
        let len = z0.len().max(m + z1.len()).max(2 * m + z2.len());
        result.reserve_exact(len);
        result.extend_from_slice(&z0);
        iadd_impl(&mut result, &z1, m);
        iadd_impl(&mut result, &z2, 2 * m);

        result
    }
}

/// Karatsuba multiplication algorithm where y is substantially larger than x.
///
/// Assumes `y.len() >= x.len()`.
fn karatsuba_uneven_mul<T>(x: &[Limb], mut y: &[Limb]) -> T
where
    T: CloneableVecLike<Limb>,
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
fn karatsuba_mul_fwd<T>(x: &[Limb], y: &[Limb]) -> T
where
    T: CloneableVecLike<Limb>,
{
    if x.len() < y.len() {
        karatsuba_mul(x, y)
    } else {
        karatsuba_mul(y, x)
    }
}

/// MulAssign bigint to bigint.
#[inline]
pub fn imul<T>(x: &mut T, y: &[Limb])
where
    T: CloneableVecLike<Limb>,
{
    if y.len() == 1 {
        small::imul(x, y[0]);
    } else {
        // We're not really in a condition where using Karatsuba
        // multiplication makes sense, so we're just going to use long
        // division. ~20% speedup compared to:
        //      *x = karatsuba_mul_fwd(x, y);
        *x = karatsuba_mul_fwd(x, y);
    }
}

/// Mul bigint to bigint.
#[inline]
pub fn mul<T>(x: &[Limb], y: &[Limb]) -> T
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    imul(&mut z, y);
    z
}

// DIVISION
// --------

/// Constants for algorithm D.
const ALGORITHM_D_B: Wide = 1 << <Limb as Integer>::BITS;
const ALGORITHM_D_M: Wide = ALGORITHM_D_B - 1;

/// Calculate qhat (an estimate for the quotient).
///
/// This is step D3 in Algorithm D in "The Art of Computer Programming".
/// Assumes `x.len() > y.len()` and `y.len() >= 2`.
///
/// * `j`   - Current index on the iteration of the loop.
fn calculate_qhat(x: &[Limb], y: &[Limb], j: usize) -> Wide {
    let n = y.len();

    // Estimate qhat of q[j]
    // Original Code:
    //  qhat = (x[j+n]*B + x[j+n-1])/y[n-1];
    //  rhat = (x[j+n]*B + x[j+n-1]) - qhat*y[n-1];
    let x_jn = as_wide(x[j + n]);
    let x_jn1 = as_wide(x[j + n - 1]);
    let num = (x_jn << <Limb as Integer>::BITS) + x_jn1;
    let den = as_wide(y[n - 1]);
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
    let x_jn2 = as_wide(x[j + n - 2]);
    let y_n2 = as_wide(y[n - 2]);
    let y_n1 = as_wide(y[n - 1]);
    // This only happens when the leading bit of qhat is set.
    while qhat >= ALGORITHM_D_B || qhat * y_n2 > (rhat << <Limb as Integer>::BITS) + x_jn2 {
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
fn multiply_and_subtract<T>(x: &mut T, y: &T, qhat: Wide, j: usize) -> SignedWide
where
    T: CloneableVecLike<Limb>,
{
    let n = y.len();

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
    let mut k: SignedWide = 0;
    let mut t: SignedWide;
    for i in 0..n {
        let x_ij = as_signed_wide(x[i + j]);
        let y_i = as_wide(y[i]);
        let p = qhat * y_i;
        t = x_ij.wrapping_sub(k).wrapping_sub(as_signed_wide(p & ALGORITHM_D_M));
        x[i + j] = as_limb(t);
        k = as_signed_wide(p >> <Limb as Integer>::BITS) - (t >> <Limb as Integer>::BITS);
    }
    t = as_signed_wide(x[j + n]) - k;
    x[j + n] = as_limb(t);

    t
}

/// Calculate the quotient from the estimate and the test.
///
/// This is a mix of step D5 and D6 in Algorithm D, so the algorithm
/// may work for single passes, without a quotient buffer.
#[inline]
fn test_quotient(qhat: Wide, t: SignedWide) -> Wide {
    if t < 0 {
        qhat - 1
    } else {
        qhat
    }
}

/// Add back.
///
/// This is step D6 in Algorithm D in "The Art of Computer Programming",
/// and adds back the remainder on the very unlikely scenario we overestimated
/// the quotient by 1. Subtract 1 from the quotient, and add back the
/// remainder.
///
/// This step should be specifically debugged, due to its low likelihood,
/// since the probability is ~2/b, where b in this case is 2^32 or 2^64.
fn add_back<T>(x: &mut T, y: &T, mut t: SignedWide, j: usize)
where
    T: CloneableVecLike<Limb>,
{
    let n = y.len();

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
        let mut k: SignedWide = 0;
        for i in 0..n {
            t = as_signed_wide(as_wide(x[i + j]) + as_wide(y[i])) + k;
            x[i + j] = as_limb(t);
            k = t >> <Limb as Integer>::BITS;
        }
        let x_jn = as_signed_wide(x[j + n]) + k;
        x[j + n] = as_limb(x_jn);
    }
}

/// Calculate the remainder from the quotient.
///
/// This is step D8 in Algorithm D in "The Art of Computer Programming",
/// and "unnormalizes" to calculate the remainder from the quotient.
fn calculate_remainder<T>(x: &[Limb], y: &[Limb], s: usize) -> T
where
    T: CloneableVecLike<Limb>,
{
    // Calculate the remainder.
    // Original Code:
    //  for (i = 0; i < n-1; i++)
    //     r[i] = (x[i] >> s) | ((unsigned long long)x[i+1] << (32-s));
    //  r[n-1] = x[n-1] >> s;
    let n = y.len();
    let mut r = T::default();
    r.reserve_exact(n);
    let rs = <Limb as Integer>::BITS - s;
    for i in 0..n - 1 {
        let xi = as_wide(x[i]) >> s;
        let xi1 = as_wide(x[i + 1]) << rs;
        let ri = xi | xi1;
        r.push(as_limb(ri));
    }
    let x_n1 = x[n - 1] >> s;
    r.push(as_limb(x_n1));

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
fn algorithm_d_div<T>(x: &[Limb], y: &[Limb]) -> (T, T)
where
    T: CloneableVecLike<Limb>,
{
    // Normalize the divisor so the leading-bit is set to 1.
    // x is the dividend, y is the divisor.
    // Need a leading zero on the numerator.
    let s = y.rindex(0).leading_zeros() as usize;
    let m = x.len();
    let n = y.len();
    let mut xn: T = small::shl_bits(x, s);
    let yn: T = small::shl_bits(y, s);
    xn.push(0);

    // Store certain variables for the algorithm.
    let mut q = T::default();
    q.resize(m - n + 1, 0);
    for j in (0..m - n + 1).rev() {
        // Estimate the quotient
        let mut qhat = calculate_qhat(&xn, &yn, j);
        if qhat != 0 {
            let t = multiply_and_subtract(&mut xn, &yn, qhat, j);
            qhat = test_quotient(qhat, t);
            add_back(&mut xn, &yn, t, j);
        }
        q[j] = as_limb(qhat);
    }
    let mut r = calculate_remainder(&xn, &yn, s);

    // Normalize our results
    small::normalize(&mut q);
    small::normalize(&mut r);

    (q, r)
}

/// DivAssign bigint to bigint.
#[inline]
pub fn idiv<T>(x: &mut T, y: &[Limb]) -> T
where
    T: CloneableVecLike<Limb>,
{
    debug_assert!(y.len() != 0);

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
        r.push(small::idiv(x, y[0]));
        r
    } else {
        let (q, r) = algorithm_d_div(x, y);
        *x = q;
        r
    }
}

/// Div bigint to bigint.
#[inline]
#[allow(dead_code)]
pub fn div<T>(x: &[Limb], y: &[Limb]) -> (T, T)
where
    T: CloneableVecLike<Limb>,
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
pub fn quorem<T>(x: &mut T, y: &T) -> Limb
where
    T: CloneableVecLike<Limb>,
{
    debug_assert!(y.len() > 0);
    let mask = as_wide(Limb::max_value());

    // Numerator is smaller the denominator, quotient always 0.
    let m = x.len();
    let n = y.len();
    if m < n {
        return 0;
    }

    // Calculate our initial estimate for q
    let mut q = x[m - 1] / (y[n - 1] + 1);

    // Need to calculate the remainder if we don't have a 0 quotient.
    if q != 0 {
        let mut borrow: Wide = 0;
        let mut carry: Wide = 0;
        for j in 0..m {
            let p = as_wide(y[j]) * as_wide(q) + carry;
            carry = p >> <Limb as Integer>::BITS;
            let t = as_wide(x[j]).wrapping_sub(p & mask).wrapping_sub(borrow);
            borrow = (t >> <Limb as Integer>::BITS) & 1;
            x[j] = as_limb(t);
        }
        small::normalize(x);
    }

    // Check if we under-estimated x.
    if greater_equal(x, y) {
        q += 1;
        let mut borrow: Wide = 0;
        let mut carry: Wide = 0;
        for j in 0..m {
            let p = as_wide(y[j]) + carry;
            carry = p >> <Limb as Integer>::BITS;
            let t = as_wide(x[j]).wrapping_sub(p & mask).wrapping_sub(borrow);
            borrow = (t >> <Limb as Integer>::BITS) & 1;
            x[j] = as_limb(t);
        }
        small::normalize(x);
    }

    q
}
