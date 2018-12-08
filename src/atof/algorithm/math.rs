//! Building-blocks for arbitrary-precision math.

// SMALL
// -----

pub(in atof::algorithm) mod small {

use util::*;

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

}   // small

// LARGE
// -----

pub(in atof::algorithm) mod large {

use util::*;
use super::small;

/// ADDITION

/// AddAssign small integer to bigint.
pub fn iadd<T: VecLike<u32>>(vec: &mut T, y: u32) {
    if vec.is_empty() {
        vec.push(y);
    } else {
        unsafe {
            // Initial add
            let mut carry = small::iadd(vec.get_unchecked_mut(0), y);

            // Increment until overflow stops occurring.
            let mut size = 1;
            while carry && size < vec.len() {
                carry = small::iadd(vec.get_unchecked_mut(size), 1);
                size += 1;
            }

            // If we overflowed the buffer entirely, need to add 1 to the end
            // of the buffer.
            if carry {
                vec.push(1);
            }
        }
    }
}

// MULTIPLICATION

/// MulAssign small integer to bigint.
pub fn imul<T: VecLike<u32>>(vec: &mut T, y: u32) {
    // Multiply iteratively over all elements, adding the carry each time.
    let mut carry: u32 = 0;
    for x in vec.iter_mut() {
        carry = small::imul(x, y, carry);
    }

    // Overflow of value, add to end.
    if carry != 0 {
        vec.push(carry);
    }
}

/// MulAssign by a power.
pub fn imul_power<T: VecLike<u32>>(vec: &mut T, mut n: u32, small_powers: &[u32]) {
    let get_power = | i: usize | unsafe { *small_powers.get_unchecked(i) };

    // Multiply by the largest small power until n < step.
    let step = small_powers.len() - 1;
    let power = get_power(step);
    let step = step as u32;
    while n >= step {
        imul(vec, power);
        n -= step;
    }

    // Multiply by the remainder.
    imul(vec, get_power(n as usize));
}

/// DIVISION

/// DivAssign small integer to bigint.
pub fn idiv<T: VecLike<u32>>(vec: &mut T, y: u32) {
    // Divide iteratively over all elements, adding the carry each time.
    let mut rem: u32 = 0;
    for x in vec.iter_mut().rev() {
        rem = small::idiv(x, y, rem);
    }

    unsafe {
        // Round-up if there's truncation in least-significant bit.
        // This only occurs if rem < 0x80000000, which is the midway
        // point for when we should round.
        // The container **cannot** be empty, since rem is not 0.
        // If the vector is not padded prior to use, this rounding error
        // is **very** significant.
        if rem > 0 && rem < 0x80000000 {
            *vec.front_unchecked_mut() += 1;
        }

        // Remove leading zero if we cause underflow. Since we're dividing
        // by a small power, we have at max 1 int removed.
        if !vec.is_empty() && vec.back_unchecked().is_zero() {
            vec.pop();
        }
    }
}

/// DivAssign by a power.
pub fn idiv_power<T: VecLike<u32>>(vec: &mut T, mut n: u32, small_powers: &[u32]) {
    let get_power = | i: usize | unsafe { *small_powers.get_unchecked(i) };

    // Divide by the largest small power until n < step.
    let step = small_powers.len() - 1;
    let power = get_power(step);
    let step = step as u32;
    while n >= step {
        idiv(vec, power);
        n -= step;
    }

    // Multiply by the remainder.
    idiv(vec, get_power(n as usize));
}

}   // large

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
        fn $name(&mut self, n: u32) {
            self.idiv_power_impl(n, &$pow)
        }
    );
}

// TRAITS
// ------

pub(in atof::algorithm) trait Bignum: Clone + Sized {
    /// Underlying storage type for a Bignum.
    type StorageType: VecLike<u32>;

    /// Get access to the underlying data
    fn data<'a>(&'a self) -> &'a Self::StorageType;

    /// Get access to the underlying data
    fn data_mut<'a>(&'a mut self) -> &'a mut Self::StorageType;

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

    // PROPERTIES

    /// Get the number of leading zero values in Bigfloat.
    /// Assumes the value is normalized.
    #[inline]
    fn leading_zero_values(&self) -> u32 {
        0
    }

    /// Get the number of trailing zero values in Bigfloat.
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

    /// Get number of leading zero bits in Bigfloat.
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

    /// Get number of trailing zero bits in Bigfloat.
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

// TODO(ahuszagh) Need a way to insert many...
//    /// Pad the buffer with zeros to the least-significant bits.
//    fn pad_zeros(&mut self, n: usize) -> usize {
//        // Assume **no** overflow for the usize, since this would lead to
//        // other memory errors. Add `bytes` 0s to the left of the current
//        // buffer, and decrease the exponent accordingly.
//
//        // Remove the number of trailing zeros values for the padding.
//        // If we don't need to pad the resulting buffer, return early.
//        let n = n.checked_sub(self.trailing_zero_values() as usize).unwrap_or(0);
//        if n.is_zero() || self.data().is_empty() {
//            return n;
//        }
//
//        // Move data to new buffer, prepend `bytes` 0s, and then append
//        // current data.
//        let mut data = smallvec::SmallVec::with_capacity(self.data().len() + n);
//        data.resize(n, 0);
//        data.extend_from_slice(self.data().as_slice());
//
//        // Swap the buffers.
//        mem::swap(&mut data, &mut self.data);
//
//        n
//    }

    // ADDITION

    /// AddAssign small integer.
    #[inline]
    fn iadd(&mut self, y: u32) {
        large::iadd(self.data_mut(), y);
    }

    /// Add small integer to a copy of self.
    #[inline]
    fn add(&mut self, y: u32) -> Self {
        let mut x = self.clone();
        x.iadd(y);
        x
    }

    // MULTIPLICATION

    /// MulAssign small integer.
    #[inline]
    fn imul(&mut self, y: u32) {
        large::imul(self.data_mut(), y);
    }

    /// Mul small integer to a copy of self.
    #[inline]
    fn mul(&mut self, y: u32) -> Self {
        let mut x = self.clone();
        x.imul(y);
        x
    }

    /// MulAssign by a power.
    #[inline]
    fn imul_power_impl(&mut self, n: u32, small_powers: &[u32]) {
        large::imul_power(self.data_mut(), n, small_powers);
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
    #[allow(unused)]        // TODO(ahuszagh) Remove...
    fn imul_pow2(&mut self, n: u32) {
        let quotient = n / u32::BITS.as_u32();
        let remainder = n % u32::BITS.as_u32();
        // TODO(ahuszagh) Need to shift left for the remainder
        // Need to insert many on the left for the quotient...
        unimplemented!()
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

    /// DivAssign small integer.
    #[inline]
    fn idiv(&mut self, y: u32) {
        large::idiv(self.data_mut(), y);
    }

    /// Div small integer to a copy of self.
    #[inline]
    fn div(&mut self, y: u32) -> Self {
        let mut x = self.clone();
        x.idiv(y);
        x
    }

    /// Implied divAssign by a power.
    #[inline]
    fn idiv_power_impl(&mut self, n: u32, small_powers: &[u32]) {
        large::idiv_power(self.data_mut(), n, small_powers);
    }

    /// DivAssign by a power.
    fn idiv_power(&mut self, n: u32, base: u32) {
        match base {
            2  => self.idiv_pow2(n),
            3  => self.idiv_pow3(n),
            4  => self.idiv_pow4(n),
            5  => self.idiv_pow5(n),
            6  => self.idiv_pow6(n),
            7  => self.idiv_pow7(n),
            8  => self.idiv_pow8(n),
            9  => self.idiv_pow9(n),
            10 => self.idiv_pow10(n),
            11 => self.idiv_pow11(n),
            12 => self.idiv_pow12(n),
            13 => self.idiv_pow13(n),
            14 => self.idiv_pow14(n),
            15 => self.idiv_pow15(n),
            16 => self.idiv_pow16(n),
            17 => self.idiv_pow17(n),
            18 => self.idiv_pow18(n),
            19 => self.idiv_pow19(n),
            20 => self.idiv_pow20(n),
            21 => self.idiv_pow21(n),
            22 => self.idiv_pow22(n),
            23 => self.idiv_pow23(n),
            24 => self.idiv_pow24(n),
            25 => self.idiv_pow25(n),
            26 => self.idiv_pow26(n),
            27 => self.idiv_pow27(n),
            28 => self.idiv_pow28(n),
            29 => self.idiv_pow29(n),
            30 => self.idiv_pow30(n),
            31 => self.idiv_pow31(n),
            32 => self.idiv_pow32(n),
            33 => self.idiv_pow33(n),
            34 => self.idiv_pow34(n),
            35 => self.idiv_pow35(n),
            36 => self.idiv_pow36(n),
            _  => unreachable!()
        }
    }

    /// Divide by a power of 2.
    #[allow(unused)]        // TODO(ahuszagh) Remove...
    fn idiv_pow2(&mut self, n: u32) {
        let quotient = n / u32::BITS.as_u32();
        let remainder = n % u32::BITS.as_u32();
        // TODO(ahuszagh) Need to shift left for the remainder
        // Need to insert many on the left for the quotient...
        unimplemented!()
    }

    idiv_power!(idiv_pow3, U32_POW3, 3);

    /// Divide by a power of 4.
    #[inline]
    fn idiv_pow4(&mut self, n: u32) {
        self.idiv_pow2(2*n);
    }

    idiv_power!(idiv_pow5, U32_POW5, 5);

    /// Divide by a power of 6.
    #[inline]
    fn idiv_pow6(&mut self, n: u32) {
        self.idiv_pow3(n);
        self.idiv_pow2(n);
    }

    idiv_power!(idiv_pow7, U32_POW7, 7);

    /// Divide by a power of 8.
    #[inline]
    fn idiv_pow8(&mut self, n: u32) {
        self.idiv_pow2(3*n);
    }

    /// Divide by a power of 9.
    #[inline]
    fn idiv_pow9(&mut self, n: u32) {
        self.idiv_pow3(n);
        self.idiv_pow3(n);
    }

    /// Divide by a power of 10.
    #[inline]
    fn idiv_pow10(&mut self, n: u32) {
        self.idiv_pow5(n);
        self.idiv_pow2(n);
    }

    idiv_power!(idiv_pow11, U32_POW11, 11);

    /// Divide by a power of 12.
    #[inline]
    fn idiv_pow12(&mut self, n: u32) {
        self.idiv_pow3(n);
        self.idiv_pow4(n);
    }

    idiv_power!(idiv_pow13, U32_POW13, 13);

    /// Divide by a power of 14.
    #[inline]
    fn idiv_pow14(&mut self, n: u32) {
        self.idiv_pow7(n);
        self.idiv_pow2(n);
    }

    /// Divide by a power of 15.
    #[inline]
    fn idiv_pow15(&mut self, n: u32) {
        self.idiv_pow3(n);
        self.idiv_pow5(n);
    }

    /// Divide by a power of 16.
    #[inline]
    fn idiv_pow16(&mut self, n: u32) {
        self.idiv_pow2(4*n);
    }

    idiv_power!(idiv_pow17, U32_POW17, 17);

    /// Divide by a power of 18.
    #[inline]
    fn idiv_pow18(&mut self, n: u32) {
        self.idiv_pow9(n);
        self.idiv_pow2(n);
    }

    idiv_power!(idiv_pow19, U32_POW19, 19);

    /// Divide by a power of 20.
    #[inline]
    fn idiv_pow20(&mut self, n: u32) {
        self.idiv_pow5(n);
        self.idiv_pow4(n);
    }

    /// Divide by a power of 21.
    #[inline]
    fn idiv_pow21(&mut self, n: u32) {
        self.idiv_pow3(n);
        self.idiv_pow7(n);
    }

    /// Divide by a power of 22.
    #[inline]
    fn idiv_pow22(&mut self, n: u32) {
        self.idiv_pow11(n);
        self.idiv_pow2(n);
    }

    idiv_power!(idiv_pow23, U32_POW23, 23);

    /// Divide by a power of 24.
    #[inline]
    fn idiv_pow24(&mut self, n: u32) {
        self.idiv_pow3(n);
        self.idiv_pow8(n);
    }

    /// Divide by a power of 25.
    #[inline]
    fn idiv_pow25(&mut self, n: u32) {
        self.idiv_pow5(n);
        self.idiv_pow5(n);
    }

    /// Divide by a power of 26.
    #[inline]
    fn idiv_pow26(&mut self, n: u32) {
        self.idiv_pow13(n);
        self.idiv_pow2(n);
    }

    /// Divide by a power of 27.
    #[inline]
    fn idiv_pow27(&mut self, n: u32) {
        self.idiv_pow9(n);
        self.idiv_pow3(n);
    }

    /// Divide by a power of 28.
    #[inline]
    fn idiv_pow28(&mut self, n: u32) {
        self.idiv_pow7(n);
        self.idiv_pow4(n);
    }

    idiv_power!(idiv_pow29, U32_POW29, 29);

    /// Divide by a power of 30.
    #[inline]
    fn idiv_pow30(&mut self, n: u32) {
        self.idiv_pow15(n);
        self.idiv_pow2(n);
    }

    idiv_power!(idiv_pow31, U32_POW31, 31);

    /// Divide by a power of 32.
    #[inline]
    fn idiv_pow32(&mut self, n: u32) {
        self.idiv_pow2(5*n);
    }

    /// Divide by a power of 33.
    #[inline]
    fn idiv_pow33(&mut self, n: u32) {
        self.idiv_pow3(n);
        self.idiv_pow11(n);
    }

    /// Divide by a power of 34.
    #[inline]
    fn idiv_pow34(&mut self, n: u32) {
        self.idiv_pow17(n);
        self.idiv_pow2(n);
    }

    /// Divide by a power of 35.
    #[inline]
    fn idiv_pow35(&mut self, n: u32) {
        self.idiv_pow5(n);
        self.idiv_pow7(n);
    }

    /// Divide by a power of 36.
    #[inline]
    fn idiv_pow36(&mut self, n: u32) {
        self.idiv_pow9(n);
        self.idiv_pow4(n);
    }
}
