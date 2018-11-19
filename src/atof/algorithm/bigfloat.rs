//! Arbitrary-precision decimal to parse a floating-point number.
// TODO(ahuszagh) Remove this arbitrary warning, we're
// in rapid development, so allow it for now.
#![allow(unused)]

use float::Mantissa;
use lib::{mem, slice};
use util::*;

// ADD

/// Add two small integers (and return if overflow happens).
#[inline(always)]
fn add_small<T: Integer>(x: T, y: T) -> (T, bool) {
    x.overflowing_add(y)
}

/// Add two small integers (and return if overflow happens).
#[inline(always)]
fn add_small_assign<T: Integer>(x: &mut T, y: T) -> bool {
    let t = add_small(*x, y);
    *x = t.0;
    t.1
}

/// Increment on the case of overflow.
#[inline(always)]
fn add_one<T: Integer>(x: T) -> (T, bool) {
    x.overflowing_add(T::ONE)
}

/// Increment on the case of overflow.
#[inline(always)]
fn add_one_assign<T: Integer>(x: &mut T) -> bool {
    let t = add_one(*x);
    *x = t.0;
    t.1
}

// MUL

//macro_rules! mul {
//    ($name:ident, $assign:ident) => (
//        // Mul by N.
//        #[inline]
//        fn $name(self) -> Bigfloat {
//            let mut x = self.clone();
//            x.$assign();
//            x
//        }
//    );
//}

// FROM BYTES

/// Wrapper for basen from_bytes implementations.
// TODO(ahuszagh) Implement
macro_rules! from_bytes {
    ($name:ident) => (
        /// Initialize Bigfloat from bytes with base3.
        fn $name(first: *const u8, last: *const u8) -> (Bigfloat, *const u8) {
            let bigfloat = Bigfloat::new();
            unimplemented!()
        }
    );
}

// BIGFLOAT

/// Large, arbitrary-precision float.
///
/// This float aims to solve the half-way problem. If we have a mantissa,
/// with the following representation:
///
///     Mantissa          | Trailing | Truncated
///     101010010110101010|1000000000|0000000001
///
/// We are supposed to round this up, since the truncated bits are above
/// halfway, however, we have no way to determine this. Any lossy
/// multiplication can push the trailing bits up or below the halfway point,
/// leading to incorrect rounding and incorrect results.
#[derive(Debug, Clone)]
pub(crate) struct Bigfloat {
    /// Raw data for the underlying buffer (exactly 32**2 for the largest float).
    /// Don't store more bytes for small floats, since the denormal floats
    /// have almost no bytes of precision.
    /// These numbers are stored in little-endian format, so index 0 is
    /// the least-significant item, and index 31 is the most-significant digit.
    /// On little-endian systems, allows us to use the raw buffer left-to-right
    /// as an extended integer
    data: [u32; 32],
    /// Exponent in base32.
    exponent: i32,
    /// Number of current digits in use.
    size: usize,
}

impl Bigfloat {
    // CONSTANTS
    const BITS: usize = mem::size_of::<u32>() * 8;

    // CREATION

    /// Create new Bigfloat.
    #[inline]
    pub fn new() -> Bigfloat {
        Self::min_value()
    }

    /// Create new Bigfloat from u32.
    #[inline]
    pub fn from_u32(x: u32) -> Bigfloat {
        Bigfloat {
            data: [x, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            exponent: 0,
            size: 1,
        }
    }

    /// Create new Bigfloat from u64.
    #[inline]
    pub fn from_u64(x: u64) -> Bigfloat {
        let hi = (x >> 32) as u32;
        let lo = (x & u64::LOMASK) as u32;
        Bigfloat {
            data: [lo, hi, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            exponent: 0,
            size: 2,
        }
    }

    /// Create new Bigfloat from u128.
    #[inline]
    pub fn from_u128(x: u128) -> Bigfloat {
        let hi64 = (x >> 64) as u64;
        let lo64 = (x & u128::LOMASK) as u64;
        let d3 = (lo64 & u64::LOMASK) as u32;
        let d2 = (lo64 >> 32) as u32;
        let d1 = (hi64 & u64::LOMASK) as u32;
        let d0 = (hi64 >> 32) as u32;
        Bigfloat {
            data: [d3, d2, d1, d0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            exponent: 0,
            size: 4,
        }
    }

    /// Create new Bigfloat with the minimal value.
    #[inline]
    pub fn min_value() -> Bigfloat {
        Bigfloat {
            data: [0; 32],
            exponent: 0,
            size: 0,
        }
    }

    /// Create new Bigfloat with the maximal value.
    #[inline]
    pub fn max_value() -> Bigfloat {
        Bigfloat {
            data: [u32::max_value(); 32],
            exponent: i32::max_value(),
            size: 32,
        }
    }

    // ADDITION

    /// AddAssign small integer to bigfloat.
    #[inline]
    fn add_small_assign(&mut self, y: u32) {
        // Initial add
        let mut carry = add_small_assign(self.get_mut(0), y);

        // Increment until overflow stops occurring.
        let mut size = 1;
        while carry && size < self.data.len() {
            carry = add_one_assign(self.get_mut(size));
            size += 1;
        }

        // Store the size, whichever is larger.
        self.size = self.size.max(size);

        // If we overflowed the buffer entirely, shift-right one and set
        // the most-significant bit to 1. We know that all the internal
        // buffers, above the last, must have been u32::max_value(), so
        // the last buffer is the only one we need to worry about.
        if carry {
            debug_assert!(self.size == self.data.len(), "Overflow must mean full-sized buffer.");
            *self.front_mut() >>= 1;
            *self.back_mut() |= 1 << (Self::BITS-1);
            self.exponent = if let Some(v) = self.exponent.checked_add(1) { v } else { i32::max_value() };
        }
    }

    /// Add small integer to bigfloat.
    #[inline]
    fn add_small(self, y: u32) -> Bigfloat {
        let mut x = self.clone();
        x.add_small_assign(y);
        x
    }

    /// AddAssign between two bigfloats.
    #[inline]
    fn add_large_assign(&mut self, y: &Bigfloat) {
        // Get the number of values to add_assign between them.
        // Only carry
        let size = self.size.max(y.size);
        let mut carry = false;
        for (l, r) in self.data.iter_mut().zip(y.data.iter()).take(size) {
            // Only one op of the two can overflow, since we added at max
            // u32::max_value() + u32::max_value(). Add the previous carry,
            // and store the current carry for the next.
            let mut tmp_carry = add_small_assign(l, *r);
            if carry {
                tmp_carry |= add_one_assign(l);
            }
            carry = tmp_carry;
        }

        // Overflow from the previous bit.
        if carry {
            if self.size == self.data.len() {
                // Overflow for the entire container, shift-right all items
                // by 1 and assign a 1-bit to the top-most element, since
                // we can overflow by at max 1.
                self.shr(1);
                *self.back_mut() |= 1 << (Self::BITS-1);
            } else {
                // Just assign 1 to the next item.
                *self.get_mut(size) += 1;
                self.size = size + 1;
            }
        }
    }

    /// Add between two bigfloats.
    #[inline]
    fn add_large(self, y: &Bigfloat) -> Bigfloat {
        let mut x = self.clone();
        x.add_large_assign(y);
        x
    }

    // FROM BYTES
    from_bytes!(from_bytes_3);
    from_bytes!(from_bytes_5);
    from_bytes!(from_bytes_6);
    from_bytes!(from_bytes_7);
    from_bytes!(from_bytes_9);
    from_bytes!(from_bytes_10);
    from_bytes!(from_bytes_11);
    from_bytes!(from_bytes_12);
    from_bytes!(from_bytes_13);
    from_bytes!(from_bytes_14);
    from_bytes!(from_bytes_15);
    from_bytes!(from_bytes_17);
    from_bytes!(from_bytes_18);
    from_bytes!(from_bytes_19);
    from_bytes!(from_bytes_20);
    from_bytes!(from_bytes_21);
    from_bytes!(from_bytes_22);
    from_bytes!(from_bytes_23);
    from_bytes!(from_bytes_24);
    from_bytes!(from_bytes_25);
    from_bytes!(from_bytes_26);
    from_bytes!(from_bytes_27);
    from_bytes!(from_bytes_28);
    from_bytes!(from_bytes_29);
    from_bytes!(from_bytes_30);
    from_bytes!(from_bytes_31);
    from_bytes!(from_bytes_33);
    from_bytes!(from_bytes_34);
    from_bytes!(from_bytes_35);
    from_bytes!(from_bytes_36);

    /// Initialize Bigfloat from bytes with custom base.
    pub fn from_bytes(base: u32, first: *const u8, last: *const u8)
        -> (Bigfloat, *const u8)
    {
        match base {
            3  => Self::from_bytes_3(first, last),
            5  => Self::from_bytes_5(first, last),
            6  => Self::from_bytes_6(first, last),
            7  => Self::from_bytes_7(first, last),
            9  => Self::from_bytes_9(first, last),
            10 => Self::from_bytes_10(first, last),
            11 => Self::from_bytes_11(first, last),
            12 => Self::from_bytes_12(first, last),
            13 => Self::from_bytes_13(first, last),
            14 => Self::from_bytes_14(first, last),
            15 => Self::from_bytes_15(first, last),
            17 => Self::from_bytes_17(first, last),
            18 => Self::from_bytes_18(first, last),
            19 => Self::from_bytes_19(first, last),
            20 => Self::from_bytes_20(first, last),
            21 => Self::from_bytes_21(first, last),
            22 => Self::from_bytes_22(first, last),
            23 => Self::from_bytes_23(first, last),
            24 => Self::from_bytes_24(first, last),
            25 => Self::from_bytes_25(first, last),
            26 => Self::from_bytes_26(first, last),
            27 => Self::from_bytes_27(first, last),
            28 => Self::from_bytes_28(first, last),
            29 => Self::from_bytes_29(first, last),
            30 => Self::from_bytes_30(first, last),
            31 => Self::from_bytes_31(first, last),
            33 => Self::from_bytes_33(first, last),
            34 => Self::from_bytes_34(first, last),
            35 => Self::from_bytes_35(first, last),
            36 => Self::from_bytes_36(first, last),
            // We shouldn't have any powers of 2 here.
            _  => unimplemented!()
        }
    }

    // TO FLOAT

    /// Export native float from bigfloat.
    pub fn as_float<F: Float>(&self) -> F {
        unimplemented!()
    }

    // INDEXING

    /// Get the front integer.
    #[inline(always)]
    fn front(&self) -> &u32 {
        self.get(0)
    }

    /// Get the front integer as mutable.
    #[inline(always)]
    fn front_mut(&mut self) -> &mut u32 {
        self.get_mut(0)
    }

    /// Get the back integer.
    #[inline(always)]
    fn back(&self) -> &u32 {
        self.get(self.data.len()-1)
    }

    /// Get the back integer as mutable.
    #[inline(always)]
    fn back_mut(&mut self) -> &mut u32 {
        let index = self.data.len()-1;
        self.get_mut(index)
    }

    /// Unchecked get that ensures the index is <= 32 during debug builds.
    #[inline(always)]
    fn get<I>(&self, index: I) -> &I::Output
        where I: slice::SliceIndex<[u32]>
    {
        unsafe { self.data.get_unchecked(index) }
    }

    /// Unchecked get_mut that ensures the index is <= 32 during debug builds.
    #[inline(always)]
    fn get_mut<I>(&mut self, index: I) -> &mut I::Output
        where I: slice::SliceIndex<[u32]>
    {
        unsafe { self.data.get_unchecked_mut(index) }
    }

    // Shifts

    /// Shift right byte and assign the bit to the most-significant bit.
    /// Used to prevent overflow for comically large numbers. We may lose
    /// exacting precision in this case, but at that point, we've already
    /// lost the game.
    ///
    /// * `shift`   - Number of bits to shift.
    fn shr(&mut self, shift: i32) {
        self.exponent = if let Some(v) = self.exponent.checked_add(shift) { v } else { i32::max_value() };
        self.shr_impl(shift);
    }

    /// Implied shift-right, where we shift all bits over by a mask.
    fn shr_impl(&mut self, shift: i32) {
        // Create a bit-mask for the lower `shift` bytes.
        let mask = (1 << shift) - 1;

        // Shift-right, carrying the bottom shift bytes and moving them over.
        let mut carry_bit = 0;
        let index = ..self.size;
        for item in self.get_mut(index).iter_mut().rev() {
            let tmp_carry = *item & mask;
            *item >>= shift;
            *item |= (carry_bit<<(Self::BITS as i32 - shift));
            carry_bit = tmp_carry;
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let bigfloat = Bigfloat::new();
        assert_eq!(bigfloat.exponent, 0);
        assert_eq!(bigfloat.size, 0);
        bigfloat.data.iter().for_each(|x| assert_eq!(*x, 0));
    }

    #[test]
    fn from_u32_test() {
        let bigfloat = Bigfloat::from_u32(255);
        assert_eq!(bigfloat.exponent, 0);
        assert_eq!(bigfloat.size, 1);
        assert_eq!(*bigfloat.front(), 255);
        bigfloat.data[1..].iter().for_each(|x| assert_eq!(*x, 0));
    }

    #[test]
    fn from_u64_test() {
        let bigfloat = Bigfloat::from_u64(1152921504606847231);
        assert_eq!(bigfloat.exponent, 0);
        assert_eq!(bigfloat.size, 2);
        assert_eq!(*bigfloat.front(), 255);
        assert_eq!(*bigfloat.get(1), 1 << 28);
        bigfloat.data[2..].iter().for_each(|x| assert_eq!(*x, 0));
    }

    #[test]
    fn from_u128_test() {
        let bigfloat = Bigfloat::from_u128(1329227997022855913342108839786316031);
        assert_eq!(bigfloat.exponent, 0);
        assert_eq!(bigfloat.size, 4);
        assert_eq!(*bigfloat.front(), 255);
        assert_eq!(*bigfloat.get(1), 1 << 28);
        assert_eq!(*bigfloat.get(2), 1 << 26);
        assert_eq!(*bigfloat.get(3), 1 << 24);
        bigfloat.data[4..].iter().for_each(|x| assert_eq!(*x, 0));
    }

    #[test]
    fn add_small_test() {
        // Overflow check
        // This should set all the internal data values to 0, the top
        // value to (1<<31), and the bottom value to (4>>1).
        // This is because the max_value + 1 leads to all 0s, we set the
        // topmost bit to 1.
        let mut bigfloat = Bigfloat::max_value();
        bigfloat.exponent = 0;
        bigfloat.add_small_assign(5);
        assert_eq!(bigfloat.exponent, 1);
        assert_eq!(*bigfloat.front(), 2);
        assert_eq!(*bigfloat.back(), 2147483648);
        bigfloat.data[1..31].iter().for_each(|x| assert_eq!(*x, 0));
    }

    #[test]
    fn add_large_test() {
        // No overflow check, add symmetric (1-int each).
        let mut x = Bigfloat::from_u32(5);
        let y = Bigfloat::from_u32(7);
        x.add_large_assign(&y);
        assert_eq!(x.exponent, 0);
        assert_eq!(*x.front(), 12);
        x.data[1..].iter().for_each(|x| assert_eq!(*x, 0));

        // No overflow, symmetric (2- and 2-ints).
        let mut x = Bigfloat::from_u64(1125899906842624);
        let mut y = Bigfloat::from_u64(35184372088832);
        x.add_large_assign(&y);
        assert_eq!(x.exponent, 0);
        assert_eq!(*x.front(), 0);
        assert_eq!(*x.get(1), 270336);
        x.data[2..].iter().for_each(|x| assert_eq!(*x, 0));

        // No overflow, asymmetric (1- and 2-ints).
        let mut x = Bigfloat::from_u32(5);
        let mut y = Bigfloat::from_u64(35184372088832);
        x.add_large_assign(&y);
        assert_eq!(x.exponent, 0);
        assert_eq!(*x.front(), 5);
        assert_eq!(*x.get(1), 8192);
        x.data[2..].iter().for_each(|x| assert_eq!(*x, 0));

        // Internal overflow check.
        let mut x = Bigfloat::from_u32(0xF1111111);
        let mut y = Bigfloat::from_u64(0x12345678);
        x.add_large_assign(&y);
        assert_eq!(x.exponent, 0);
        assert_eq!(*x.front(), 0x3456789);
        assert_eq!(*x.get(1), 1);
        x.data[2..].iter().for_each(|x| assert_eq!(*x, 0));

        // Complete overflow check
        let mut x = Bigfloat::max_value();
        x.exponent = 0;
        let mut y = Bigfloat::max_value();
        y.exponent = 0;
        x.add_large_assign(&y);
        assert_eq!(x.exponent, 1);
        x.data.iter().for_each(|x| assert_eq!(*x, 4294967295,));
    }

    #[test]
    fn shr_test() {
        // Check shifting right from the first index.
        let mut bigfloat = Bigfloat::from_u32(5);
        bigfloat.shr(1);
        assert_eq!(bigfloat.exponent, 1);
        assert_eq!(*bigfloat.front(), 2);
        bigfloat.data[1..32].iter().for_each(|x| assert_eq!(*x, 0));
    }
}
