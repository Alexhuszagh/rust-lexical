//! Arbitrary-precision decimal to parse a floating-point number.
// TODO(ahuszagh) Remove this arbitrary warning, we're
// in rapid development, so allow it for now.
#![allow(unused)]

use util::*;

// TODO(ahuszgah) Implement...

/// Large, arbitrary-precision float.
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
    exponent: u32,
}

impl Bigfloat {
    // ADDITION

    /// AddAssign small integer to bigfloat.
    #[inline]
    fn add_assign(&mut self, y: u32) {
        unimplemented!()
    }

    /// Add small integer to bigfloat.
    #[inline]
    fn add(self, y: u32) -> Bigfloat {
        let mut x = self.clone();
        x.add_assign(y);
        x
    }

    /// AddAssign between two bigfloats.
    #[inline]
    fn add_large_assign(&mut self, y: &Bigfloat) {
        unimplemented!()
    }

    /// Add between two bigfloats.
    #[inline]
    fn add_large(self, y: &Bigfloat) -> Bigfloat {
        let mut x = self.clone();
        x.add_large_assign(y);
        x
    }

    // MULTIPLICATION

    // MulAssign by 2.
    #[inline]
    fn mul_2_assign(&mut self) {
        unimplemented!()
    }

    // Mul by 2.
    #[inline]
    fn mul_2(&mut self) -> Bigfloat {
        let mut x = self.clone();
        x.mul_2_assign();
        x
    }

    // MulAssign by 3.
    #[inline]
    fn mul_3_assign(&mut self) {
        unimplemented!()
    }

    // Mul by 3.
    #[inline]
    fn mul_3(&mut self) -> Bigfloat {
        let mut x = self.clone();
        x.mul_3_assign();
        x
    }

    // MulAssign by 4.
    #[inline]
    fn mul_4_assign(&mut self) {
        unimplemented!()
    }

    // Mul by 4.
    #[inline]
    fn mul_4(&mut self) -> Bigfloat {
        let mut x = self.clone();
        x.mul_4_assign();
        x
    }

    // TODO(ahuszagh) Need 5-36

    // FROM STR

    /// Initialize Bigfloat from bytes with base3.
    fn from_bytes_3(first: *const u8, last: *const u8)
        -> (Bigfloat, *const u8)
    {
        unimplemented!()
    }

    // TODO(ahuszagh) Need 2-36

    /// Initialize Bigfloat from bytes with custom base.
    pub fn from_bytes(base: u32, first: *const u8, last: *const u8)
        -> (Bigfloat, *const u8)
    {
        match base {
            3  => Self::from_bytes_3(first, last),
            // We shouldn't have any powers of 2 here.
            _  => unimplemented!()
        }
    }

    // TO FLOAT

    /// Export native float from bigfloat.
    pub fn as_float<F: Float>(&self) -> F {
        unimplemented!()
    }
}
