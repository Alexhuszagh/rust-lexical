//! Large-to-scalar operations.
//!
//! Modifies a big integer from a native scalar.

use crate::lib::iter;
use crate::util::config::*;
use crate::util::traits::*;

use super::cast::*;
use super::mask::*;
use super::scalar;

// PROPERTIES
// ----------

/// Get the number of leading zero values in the storage.
/// Assumes the value is normalized.
#[inline]
pub fn leading_zero_limbs(_: &[Limb]) -> usize {
    0
}

/// Get the number of trailing zero values in the storage.
/// Assumes the value is normalized.
#[inline]
pub fn trailing_zero_limbs(x: &[Limb]) -> usize {
    let mut iter = x.iter().enumerate();
    let opt = iter.find(|&tup| !tup.1.is_zero());
    let value = opt.map(|t| t.0).unwrap_or(x.len());

    value
}

/// Get number of leading zero bits in the storage.
#[inline]
pub fn leading_zeros(x: &[Limb]) -> usize {
    if x.is_empty() {
        0
    } else {
        x.rindex(0).leading_zeros() as usize
    }
}

/// Get number of trailing zero bits in the storage.
/// Assumes the value is normalized.
#[inline]
pub fn trailing_zeros(x: &[Limb]) -> usize {
    // Get the index of the last non-zero value
    let index = trailing_zero_limbs(x);
    let mut count = index.saturating_mul(<Limb as Integer>::BITS);
    if let Some(value) = x.get(index) {
        count = count.saturating_add(value.trailing_zeros() as usize);
    }
    count
}

// BIT LENGTH
// ----------

/// Calculate the bit-length of the big-integer.
#[inline]
pub fn bit_length(x: &[Limb]) -> usize {
    // Avoid overflowing, calculate via total number of bits
    // minus leading zero bits.
    let nlz = leading_zeros(x);
    <Limb as Integer>::BITS.checked_mul(x.len()).map(|v| v - nlz).unwrap_or(usize::max_value())
}

/// Calculate the limb-length of the big-integer.
#[inline]
pub fn limb_length(x: &[Limb]) -> usize {
    x.len()
}

// SHR
// ---

/// Shift-right bits inside a buffer and returns the truncated bits.
///
/// Returns the truncated bits.
///
/// Assumes `n < <Limb as Integer>::BITS`, IE, internally shifting bits.
#[inline]
pub fn ishr_bits<T>(x: &mut T, n: usize) -> Limb
where
    T: CloneableVecLike<Limb>,
{
    // Need to shift by the number of `bits % <Limb as Integer>::BITS`.
    let bits = <Limb as Integer>::BITS;
    debug_assert!(n < bits && n != 0);

    // Internally, for each item, we shift left by n, and add the previous
    // right shifted limb-bits.
    // For example, we transform (for u8) shifted right 2, to:
    //      b10100100 b01000010
    //        b101001 b00010000
    let lshift = bits - n;
    let rshift = n;
    let mut prev: Limb = 0;
    for xi in x.iter_mut().rev() {
        let tmp = *xi;
        *xi >>= rshift;
        *xi |= prev << lshift;
        prev = tmp;
    }

    prev & lower_n_mask(as_limb(rshift))
}

/// Shift-right `n` limbs inside a buffer and returns if all the truncated limbs are zero.
///
/// Assumes `n` is not 0.
#[inline]
pub fn ishr_limbs<T>(x: &mut T, n: usize) -> bool
where
    T: CloneableVecLike<Limb>,
{
    debug_assert!(n != 0);

    if n >= x.len() {
        x.clear();
        false
    } else {
        let is_zero = (&x[..n]).iter().all(|v| v.is_zero());
        x.remove_many(0..n);
        is_zero
    }
}

/// Shift-left buffer by n bits and return if we should round-up.
#[inline]
pub fn ishr<T>(x: &mut T, n: usize) -> bool
where
    T: CloneableVecLike<Limb>,
{
    let bits = <Limb as Integer>::BITS;
    // Need to pad with zeros for the number of `bits / <Limb as Integer>::BITS`,
    // and shift-left with carry for `bits % <Limb as Integer>::BITS`.
    let rem = n % bits;
    let div = n / bits;
    let is_zero = match div.is_zero() {
        true => true,
        false => ishr_limbs(x, div),
    };
    let truncated = match rem.is_zero() {
        true => 0,
        false => ishr_bits(x, rem),
    };

    // Calculate if we need to roundup.
    let roundup = {
        let halfway = lower_n_halfway(as_limb(rem));
        if truncated > halfway {
            // Above halfway
            true
        } else if truncated == halfway {
            // Exactly halfway, if !is_zero, we have a tie-breaker,
            // otherwise, we follow round-to-nearest, tie-even rules.
            // Cannot be empty, since truncated is non-zero.
            !is_zero || x[0].is_odd()
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
#[inline]
#[allow(dead_code)]
pub fn shr<T>(x: &[Limb], n: usize) -> (T, bool)
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    let roundup = ishr(&mut z, n);
    (z, roundup)
}

// SHL
// ---

/// Shift-left bits inside a buffer.
///
/// Assumes `n < <Limb as Integer>::BITS`, IE, internally shifting bits.
#[inline]
pub fn ishl_bits<T>(x: &mut T, n: usize)
where
    T: CloneableVecLike<Limb>,
{
    // Need to shift by the number of `bits % <Limb as Integer>::BITS)`.
    let bits = <Limb as Integer>::BITS;
    debug_assert!(n < bits);
    if n.is_zero() {
        return;
    }

    // Internally, for each item, we shift left by n, and add the previous
    // right shifted limb-bits.
    // For example, we transform (for u8) shifted left 2, to:
    //      b10100100 b01000010
    //      b10 b10010001 b00001000
    let rshift = bits - n;
    let lshift = n;
    let mut prev: Limb = 0;
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
/// Assumes `n < <Limb as Integer>::BITS`, IE, internally shifting bits.
#[inline]
pub fn shl_bits<T>(x: &[Limb], n: usize) -> T
where
    T: CloneableVecLike<Limb>,
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
pub fn ishl_limbs<T>(x: &mut T, n: usize)
where
    T: CloneableVecLike<Limb>,
{
    debug_assert!(n != 0);
    if !x.is_empty() {
        x.insert_many(0, iter::repeat(0).take(n));
    }
}

/// Shift-left buffer by n bits.
#[inline]
pub fn ishl<T>(x: &mut T, n: usize)
where
    T: CloneableVecLike<Limb>,
{
    let bits = <Limb as Integer>::BITS;
    // Need to pad with zeros for the number of `bits / <Limb as Integer>::BITS`,
    // and shift-left with carry for `bits % <Limb as Integer>::BITS`.
    let rem = n % bits;
    let div = n / bits;
    ishl_bits(x, rem);
    if !div.is_zero() {
        ishl_limbs(x, div);
    }
}

/// Shift-left buffer by n bits.
#[inline]
#[allow(dead_code)]
pub fn shl<T>(x: &[Limb], n: usize) -> T
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    ishl(&mut z, n);
    z
}

// NORMALIZE
// ---------

/// Normalize the container by popping any leading zeros.
#[inline]
pub fn normalize<T>(x: &mut T)
where
    T: CloneableVecLike<Limb>,
{
    // Remove leading zero if we cause underflow. Since we're dividing
    // by a small power, we have at max 1 int removed.
    while !x.is_empty() && x.rindex(0).is_zero() {
        x.pop();
    }
}

// ADDITION
// --------

/// Implied AddAssign implementation for adding a small integer to bigint.
///
/// Allows us to choose a start-index in x to store, to allow incrementing
/// from a non-zero start.
#[inline]
pub fn iadd_impl<T>(x: &mut T, y: Limb, xstart: usize)
where
    T: CloneableVecLike<Limb>,
{
    if x.len() <= xstart {
        x.push(y);
    } else {
        // Initial add
        let mut carry = scalar::iadd(&mut x[xstart], y);

        // Increment until overflow stops occurring.
        let mut size = xstart + 1;
        while carry && size < x.len() {
            carry = scalar::iadd(&mut x[size], 1);
            size += 1;
        }

        // If we overflowed the buffer entirely, need to add 1 to the end
        // of the buffer.
        if carry {
            x.push(1);
        }
    }
}

/// AddAssign small integer to bigint.
#[inline]
pub fn iadd<T>(x: &mut T, y: Limb)
where
    T: CloneableVecLike<Limb>,
{
    iadd_impl(x, y, 0);
}

/// Add small integer to bigint.
#[inline]
#[allow(dead_code)]
pub fn add<T>(x: &[Limb], y: Limb) -> T
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

/// SubAssign small integer to bigint.
/// Does not do overflowing subtraction.
#[inline]
pub fn isub_impl<T>(x: &mut T, y: Limb, xstart: usize)
where
    T: CloneableVecLike<Limb>,
{
    debug_assert!(x.len() > xstart && (x[xstart] >= y || x.len() > xstart + 1));

    // Initial subtraction
    let mut carry = scalar::isub(&mut x[xstart], y);

    // Increment until overflow stops occurring.
    let mut size = xstart + 1;
    while carry && size < x.len() {
        carry = scalar::isub(&mut x[size], 1);
        size += 1;
    }
    normalize(x);
}

/// SubAssign small integer to bigint.
/// Does not do overflowing subtraction.
#[inline]
pub fn isub<T>(x: &mut T, y: Limb)
where
    T: CloneableVecLike<Limb>,
{
    isub_impl(x, y, 0);
}

/// Sub small integer to bigint.
#[inline]
#[allow(dead_code)]
pub fn sub<T>(x: &[Limb], y: Limb) -> T
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    isub(&mut z, y);
    z
}

// MULTIPLICATION
// --------------

/// MulAssign small integer to bigint.
#[inline]
pub fn imul<T>(x: &mut T, y: Limb)
where
    T: CloneableVecLike<Limb>,
{
    // Multiply iteratively over all elements, adding the carry each time.
    let mut carry: Limb = 0;
    for xi in x.iter_mut() {
        carry = scalar::imul(xi, y, carry);
    }

    // Overflow of value, add to end.
    if carry != 0 {
        x.push(carry);
    }
}

/// Mul small integer to bigint.
#[inline]
pub fn mul<T>(x: &[Limb], y: Limb) -> T
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

/// DivAssign small integer to bigint and get the remainder.
#[inline]
pub fn idiv<T>(x: &mut T, y: Limb) -> Limb
where
    T: CloneableVecLike<Limb>,
{
    // Divide iteratively over all elements, adding the carry each time.
    let mut rem: Limb = 0;
    for xi in x.iter_mut().rev() {
        rem = scalar::idiv(xi, y, rem);
    }
    normalize(x);

    rem
}

/// Div small integer to bigint and get the remainder.
#[inline]
#[allow(dead_code)]
pub fn div<T>(x: &[Limb], y: Limb) -> (T, Limb)
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    let rem = idiv(&mut z, y);
    (z, rem)
}
