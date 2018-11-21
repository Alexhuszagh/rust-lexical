//! Arbitrary-precision decimal to parse a floating-point number.
// TODO(ahuszagh) Remove this arbitrary warning, we're
// in rapid development, so allow it for now.
#![allow(unused)]

use smallvec;
use float::Mantissa;
use lib::{mem, slice};
use util::*;

// ADD

/// Add two small integers (and return if overflow happens).
#[inline(always)]
fn add_small<T: Integer>(x: T, y: T)
    -> (T, bool)
{
    x.overflowing_add(y)
}

/// Add two small integers (and return if overflow happens).
#[inline(always)]
fn add_small_assign<T: Integer>(x: &mut T, y: T)
    -> bool
{
    let t = add_small(*x, y);
    *x = t.0;
    t.1
}

/// Increment on the case of overflow.
#[inline(always)]
fn add_one<T: Integer>(x: T)
    -> (T, bool)
{
    x.overflowing_add(T::ONE)
}

/// Increment on the case of overflow.
#[inline(always)]
fn add_one_assign<T: Integer>(x: &mut T)
    -> bool
{
    let t = add_one(*x);
    *x = t.0;
    t.1
}

// MUL

/// Multiply two small integers (with carry) (and return the overflow contribution).
///
/// Returns the (low, high) components.
#[inline(always)]
fn mul_small<Wide, Narrow>(x: Narrow, y: Narrow, carry: Narrow) -> (Narrow, Narrow)
    where Narrow: Integer,
          Wide: Integer
{
    // Assert that wide is 2 times as wide as narrow.
    debug_assert!(mem::size_of::<Narrow>()*2 == mem::size_of::<Wide>());

    // Cannot overflow, as long as wide is 2x as wide. This is because
    // the following is always true:
    // `Wide::max_value() - (Narrow::max_value() * Narrow::max_value()) >= Narrow::max_value()`
    let bits = mem::size_of::<Narrow>() * 8;
    let z: Wide = as_::<Wide, _>(x) * as_::<Wide, _>(y) + as_::<Wide,_>(carry);
    (as_::<Narrow, _>(z), as_::<Narrow, _>(z >> bits))
}

/// Multiply two small integers (with carry) (and return if overflow happens).
#[inline(always)]
fn mul_small_assign<Wide, Narrow>(x: &mut Narrow, y: Narrow, carry: Narrow) -> Narrow
    where Narrow: Integer,
          Wide: Integer
{
    let t = mul_small::<Wide, Narrow>(*x, y, carry);
    *x = t.0;
    t.1
}

// FROM BYTES

/// Wrap operation using an assign internally.
macro_rules! wrap_assign {
    ($name:ident, $assign:ident, $(, $a:ident: $v:expr)*) => ()
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Bigfloat {
    /// Raw data for the underlying buffer (exactly 32**2 for the largest float).
    /// Don't store more bytes for small floats, since the denormal floats
    /// have almost no bytes of precision.
    /// These numbers are stored in little-endian format, so index 0 is
    /// the least-significant item, and index 31 is the most-significant digit.
    /// On little-endian systems, allows us to use the raw buffer left-to-right
    /// as an extended integer
    data: smallvec::SmallVec<[u32; 32]>,
    /// Exponent in base32.
    exponent: i32,
}

impl Bigfloat {
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
            data: smallvec![x],
            exponent: 0,
        }
    }

    /// Create new Bigfloat from u64.
    #[inline]
    pub fn from_u64(x: u64) -> Bigfloat {
        let hi = (x >> 32) as u32;
        let lo = (x & u64::LOMASK) as u32;
        Bigfloat {
            data: smallvec![lo, hi],
            exponent: 0,
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
            data: smallvec![d3, d2, d1, d0],
            exponent: 0,
        }
    }

    /// Create new Bigfloat with the minimal value.
    #[inline]
    pub fn min_value() -> Bigfloat {
        Bigfloat {
            data: smallvec![],
            exponent: 0,
        }
    }

    /// Create new Bigfloat with the maximal value on stack.
    #[inline]
    pub fn max_value() -> Bigfloat {
        Bigfloat {
            data: smallvec![u32::max_value(); 32],
            exponent: i32::max_value(),
        }
    }

    // ADDITION

    /// Implementation for AssAssign with small integer. Must be non-empty.
    #[inline]
    fn add_small_assign_impl(&mut self, y: u32) {
        // Initial add
        let mut carry = add_small_assign(self.get_mut(0), y);

        // Increment until overflow stops occurring.
        let mut size = 1;
        while carry && size < self.data.len() {
            carry = add_one_assign(self.get_mut(size));
            size += 1;
        }

        // If we overflowed the buffer entirely, need to add 1 to the end
        // of the buffer.
        if carry {
            self.data.push(1);
        }
    }

    /// AddAssign small integer to bigfloat.
    #[inline]
    fn add_small_assign(&mut self, y: u32) {
        if self.data.is_empty() {
            self.data.push(y)
        } else {
            self.add_small_assign_impl(y)
        }
    }

    /// Add small integer to bigfloat.
    #[inline]
    fn add_small(&self, y: u32) -> Bigfloat {
        let mut x = self.clone();
        x.add_small_assign(y);
        x
    }

    /// AddAssign between two bigfloats.
    #[inline]
    fn add_large_assign(&mut self, y: &Bigfloat) {
        // Logic error, ensure both numbers have the same exponent.
        debug_assert!(self.exponent == y.exponent);

        // Get the number of values to add_assign between them.
        // Resize the buffer so at least y.data elements are in x.data.
        let size = self.data.len().max(y.data.len());
        self.data.resize(size, 0);

        // Iteratively add elements from y to x.
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
            if size == self.data.len() {
                // Overflow for the entire container, push 1 to the end.
                self.data.push(1);
            } else {
                // Internal overflow, just add 1 to the next item.
                *self.get_mut(size) += 1;
            }
        }
    }

    /// Add between two bigfloats.
    #[inline]
    fn add_large(&self, y: &Bigfloat) -> Bigfloat {
        let mut x = self.clone();
        x.add_large_assign(y);
        x
    }

    // MULTIPLICATION

    /// MulAssign small integer to bigfloat.
    fn mul_small_assign(&mut self, y: u32) {
        // Multiply iteratively over all elements, adding the carry each time.
        let mut carry: u32 = 0;
        for x in self.data.iter_mut() {
            carry = mul_small_assign::<u64, u32>(x, y, carry);
        }

        // Overflow of value, add to end.
        if carry != 0 {
            self.data.push(carry);
        }
    }

    /// Mul small integer to bigfloat.
    #[inline]
    fn mul_small(&self, y: u32) -> Bigfloat {
        let mut x = self.clone();
        x.mul_small_assign(y);
        x
    }

    /// MulAssign by a power of 2.
    #[inline]
    fn mul_pow2_assign(&mut self, n: i32) {
        // Increment exponent to simulate actual addition.
        self.exponent = match self.exponent.overflowing_add(n) {
            (v, false) => v,
            (_, true) => if n < 0 { i32::min_value() } else { i32::max_value() },
        };
    }

    // MulAssign using pre-calculated small powers.
    #[inline]
    fn mul_spowers_assign(&mut self, n: i32, small_powers: &[u32]) {
        // We need to multiply by the largest small-power until we run out.
        // TODO(ahuszagh) Implement...
        unimplemented!()
    }

    /// MulAssign by a power of 3.
    #[inline]
    fn mul_pow3_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 21] = [
            1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049,
            177147,  531441, 1594323, 4782969, 14348907, 43046721,
            129140163, 387420489, 1162261467, 3486784401
        ];
        self.mul_spowers_assign(n, &SMALL_POWERS);
    }

    /// MulAssign by a power of 4.
    #[inline]
    fn mul_pow4_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow2_assign(n);
    }

    /// MulAssign by a power of 5.
    #[inline]
    fn mul_pow5_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 14] = [
            1, 5, 25, 125, 625, 3125, 15625, 78125, 390625,
            1953125, 9765625, 48828125, 244140625, 1220703125
        ];
        self.mul_spowers_assign(n, &SMALL_POWERS);
    }

    /// MulAssign by a power of 6.
    #[inline]
    fn mul_pow6_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow3_assign(n);
    }

    /// MulAssign by a power of 7.
    #[inline]
    fn mul_pow7_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 12] = [
            1, 7, 49, 343, 2401, 16807, 117649, 823543,
            5764801, 40353607, 282475249, 1977326743
        ];
        self.mul_spowers_assign(n, &SMALL_POWERS);
    }

    /// MulAssign by a power of 8.
    #[inline]
    fn mul_pow8_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow4_assign(n);
    }

    /// MulAssign by a power of 9.
    #[inline]
    fn mul_pow9_assign(&mut self, n: i32) {
        self.mul_pow3_assign(n);
        self.mul_pow3_assign(n);
    }

    /// MulAssign by a power of 10.
    #[inline]
    fn mul_pow10_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow5_assign(n);
    }

    /// MulAssign by a power of 11.
    #[inline]
    fn mul_pow11_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 10] = [1, 11, 121, 1331, 14641, 161051, 1771561, 19487171, 214358881, 2357947691];
        self.mul_spowers_assign(n, &SMALL_POWERS)
    }

    /// MulAssign by a power of 12.
    #[inline]
    fn mul_pow12_assign(&mut self, n: i32) {
        self.mul_pow3_assign(n);
        self.mul_pow4_assign(n);
    }

    /// MulAssign by a power of 13.
    #[inline]
    fn mul_pow13_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 9] = [1, 13, 169, 2197, 28561, 371293, 4826809, 62748517, 815730721];
        self.mul_spowers_assign(n, &SMALL_POWERS)
    }

    /// MulAssign by a power of 14.
    #[inline]
    fn mul_pow14_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow7_assign(n);
    }

    /// MulAssign by a power of 15.
    #[inline]
    fn mul_pow15_assign(&mut self, n: i32) {
        self.mul_pow3_assign(n);
        self.mul_pow5_assign(n);
    }

    /// MulAssign by a power of 16.
    #[inline]
    fn mul_pow16_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow8_assign(n);
    }

    /// MulAssign by a power of 17.
    #[inline]
    fn mul_pow17_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 8] = [1, 17, 289, 4913, 83521, 1419857, 24137569, 410338673];
        self.mul_spowers_assign(n, &SMALL_POWERS)
    }

    /// MulAssign by a power of 18.
    #[inline]
    fn mul_pow18_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow9_assign(n);
    }

    /// MulAssign by a power of 19.
    #[inline]
    fn mul_pow19_assign(&mut self, n: i32) {
        unimplemented!()
    }

    /// MulAssign by a power of 20.
    #[inline]
    fn mul_pow20_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow10_assign(n);
    }

    /// MulAssign by a power of 21.
    #[inline]
    fn mul_pow21_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 8] = [1, 21, 441, 9261, 194481, 4084101, 85766121, 1801088541];
        self.mul_spowers_assign(n, &SMALL_POWERS)
    }

    /// MulAssign by a power of 22.
    #[inline]
    fn mul_pow22_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow11_assign(n);
    }

    /// MulAssign by a power of 23.
    #[inline]
    fn mul_pow23_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 8] = [1, 23, 529, 12167, 279841, 6436343, 148035889, 3404825447];
        self.mul_spowers_assign(n, &SMALL_POWERS)
    }

    /// MulAssign by a power of 24.
    #[inline]
    fn mul_pow24_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow12_assign(n);
    }

    /// MulAssign by a power of 25.
    #[inline]
    fn mul_pow25_assign(&mut self, n: i32) {
        self.mul_pow5_assign(n);
        self.mul_pow5_assign(n);
    }

    /// MulAssign by a power of 26.
    #[inline]
    fn mul_pow26_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow13_assign(n);
    }

    /// MulAssign by a power of 27.
    #[inline]
    fn mul_pow27_assign(&mut self, n: i32) {
        self.mul_pow3_assign(n);
        self.mul_pow9_assign(n);
    }

    /// MulAssign by a power of 28.
    #[inline]
    fn mul_pow28_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow14_assign(n);
    }

    /// MulAssign by a power of 29.
    #[inline]
    fn mul_pow29_assign(&mut self, n: i32) {
        unimplemented!()
    }

    /// MulAssign by a power of 30.
    #[inline]
    fn mul_pow30_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow15_assign(n);
    }

    /// MulAssign by a power of 31.
    #[inline]
    fn mul_pow31_assign(&mut self, n: i32) {
        const SMALL_POWERS: [u32; 7] = [1, 31, 961, 29791, 923521, 28629151, 887503681];
        self.mul_spowers_assign(n, &SMALL_POWERS)
    }

    /// MulAssign by a power of 32.
    #[inline]
    fn mul_pow32_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow16_assign(n);
    }

    /// MulAssign by a power of 33.
    #[inline]
    fn mul_pow33_assign(&mut self, n: i32) {
        self.mul_pow3_assign(n);
        self.mul_pow11_assign(n);
    }

    /// MulAssign by a power of 34.
    #[inline]
    fn mul_pow34_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow17_assign(n);
    }

    /// MulAssign by a power of 35.
    #[inline]
    fn mul_pow35_assign(&mut self, n: i32) {
        self.mul_pow5_assign(n);
        self.mul_pow7_assign(n);
    }

    /// MulAssign by a power of 36.
    #[inline]
    fn mul_pow36_assign(&mut self, n: i32) {
        self.mul_pow2_assign(n);
        self.mul_pow18_assign(n);
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
        debug_assert!(self.data.len() > 0);
        self.get(0)
    }

    /// Get the front integer as mutable.
    #[inline(always)]
    fn front_mut(&mut self) -> &mut u32 {
        debug_assert!(self.data.len() > 0);
        self.get_mut(0)
    }

    /// Get the back integer.
    #[inline(always)]
    fn back(&self) -> &u32 {
        debug_assert!(self.data.len() > 0);
        self.get(self.data.len()-1)
    }

    /// Get the back integer as mutable.
    #[inline(always)]
    fn back_mut(&mut self) -> &mut u32 {
        debug_assert!(self.data.len() > 0);
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
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let bigfloat = Bigfloat::new();
        assert_eq!(bigfloat, Bigfloat { data: smallvec![], exponent: 0 });
    }

    #[test]
    fn from_u32_test() {
        let bigfloat = Bigfloat::from_u32(255);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255], exponent: 0 });
    }

    #[test]
    fn from_u64_test() {
        let bigfloat = Bigfloat::from_u64(1152921504606847231);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255, 1 << 28], exponent: 0 });
    }

    #[test]
    fn from_u128_test() {
        let bigfloat = Bigfloat::from_u128(1329227997022855913342108839786316031);
        assert_eq!(bigfloat, Bigfloat { data: smallvec![255, 1 << 28, 1 << 26, 1<< 24], exponent: 0 });
    }

    #[test]
    fn add_small_test() {
        // Overflow check (single)
        // This should set all the internal data values to 0, the top
        // value to (1<<31), and the bottom value to (4>>1).
        // This is because the max_value + 1 leads to all 0s, we set the
        // topmost bit to 1.
        let mut x = Bigfloat::from_u32(4294967295);
        x.add_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![4, 1], exponent: 0 });

        // No overflow, single value
        let mut x = Bigfloat::from_u32(5);
        x.add_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![12], exponent: 0 });

        // Single carry, internal overflow
        let mut x = Bigfloat::from_u64(0x80000000FFFFFFFF);
        x.add_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![6, 0x80000001], exponent: 0 });

        // Double carry, overflow
        let mut x = Bigfloat::from_u64(0xFFFFFFFFFFFFFFFF);
        x.add_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![6, 0, 1], exponent: 0 });
    }

    #[test]
    fn add_large_test() {
        // No overflow check, add symmetric (1-int each).
        let mut x = Bigfloat::from_u32(5);
        let y = Bigfloat::from_u32(7);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![12], exponent: 0 });

        // No overflow, symmetric (2- and 2-ints).
        let mut x = Bigfloat::from_u64(1125899906842624);
        let mut y = Bigfloat::from_u64(35184372088832);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![0, 270336], exponent: 0 });

        // No overflow, asymmetric (1- and 2-ints).
        let mut x = Bigfloat::from_u32(5);
        let mut y = Bigfloat::from_u64(35184372088832);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![5, 8192], exponent: 0 });

        // Internal overflow check.
        let mut x = Bigfloat::from_u32(0xF1111111);
        let mut y = Bigfloat::from_u64(0x12345678);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![0x3456789, 1], exponent: 0 });

        // Complete overflow check
        let mut x = Bigfloat::from_u32(4294967295);
        let y = Bigfloat::from_u32(4294967295);
        x.add_large_assign(&y);
        assert_eq!(x, Bigfloat { data: smallvec![4294967294, 1], exponent: 0 });
    }

    #[test]
    fn mul_small_test() {
        // No overflow check, 1-int.
        let mut x = Bigfloat::from_u32(5);
        x.mul_small_assign(7);
        assert_eq!(x, Bigfloat { data: smallvec![35], exponent: 0 });

        // No overflow check, 2-ints.
        let mut x = Bigfloat::from_u64(0x4000000040000);
        x.mul_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![0x00140000, 0x140000], exponent: 0 });

        // Overflow, 1 carry.
        let mut x = Bigfloat::from_u32(0x33333334);
        x.mul_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![4, 1], exponent: 0 });

        // Overflow, 1 carry, internal.
        let mut x = Bigfloat::from_u64(0x133333334);
        x.mul_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![4, 6], exponent: 0 });

        // Overflow, 2 carries.
        let mut x = Bigfloat::from_u64(0x3333333333333334);
        x.mul_small_assign(5);
        assert_eq!(x, Bigfloat { data: smallvec![4, 0, 1], exponent: 0 });
    }

    #[test]
    fn mul_pow2_test() {
        // TODO(ahuszagh) implement...
    }

    #[test]
    fn mul_pow3_test() {
        // TODO(ahuszagh) implement...
    }

    // TODO(ahuszagh) Add
}
