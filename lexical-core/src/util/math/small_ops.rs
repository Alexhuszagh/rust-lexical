//! Small operations for arbitrary-precision integers.

use crate::util::config::*;
use crate::util::powers::*;

use super::power;
use super::shared_ops::*;
use super::small;

/// Generate the imul_pown wrappers.
macro_rules! imul_power {
    ($name:ident, $base:expr) => {
        /// Multiply by a power of $base.
        #[inline]
        fn $name(&mut self, n: u32) {
            self.imul_power_impl($base, n)
        }
    };
}

// SMALL OPS
// ---------

#[allow(dead_code)]
pub(crate) trait SmallOps: SharedOps {
    // SMALL POWERS

    /// Get the small powers from the radix.
    #[inline]
    fn small_powers(radix: u32) -> &'static [Limb] {
        get_small_powers(radix)
    }

    /// Get the large powers from the radix.
    #[inline]
    fn large_powers(radix: u32) -> &'static [&'static [Limb]] {
        get_large_powers(radix)
    }

    // ADDITION

    /// AddAssign small integer.
    #[inline]
    fn iadd_small(&mut self, y: Limb) {
        small::iadd(self.data_mut(), y);
    }

    /// Add small integer to a copy of self.
    #[inline]
    fn add_small(&self, y: Limb) -> Self {
        let mut x = self.clone();
        x.iadd_small(y);
        x
    }

    // SUBTRACTION

    /// SubAssign small integer.
    /// Warning: Does no overflow checking, x must be >= y.
    #[inline]
    fn isub_small(&mut self, y: Limb) {
        small::isub(self.data_mut(), y);
    }

    /// Sub small integer to a copy of self.
    /// Warning: Does no overflow checking, x must be >= y.
    #[inline]
    fn sub_small(&mut self, y: Limb) -> Self {
        let mut x = self.clone();
        x.isub_small(y);
        x
    }

    // MULTIPLICATION

    /// MulAssign small integer.
    #[inline]
    fn imul_small(&mut self, y: Limb) {
        small::imul(self.data_mut(), y);
    }

    /// Mul small integer to a copy of self.
    #[inline]
    fn mul_small(&self, y: Limb) -> Self {
        let mut x = self.clone();
        x.imul_small(y);
        x
    }

    /// MulAssign by a power.
    #[inline]
    fn imul_power_impl(&mut self, radix: u32, n: u32) {
        power::imul_power(self.data_mut(), radix, n);
    }

    #[inline]
    fn imul_power(&mut self, radix: u32, n: u32) {
        match radix {
            2 => self.imul_pow2(n),
            3 => self.imul_pow3(n),
            4 => self.imul_pow4(n),
            5 => self.imul_pow5(n),
            6 => self.imul_pow6(n),
            7 => self.imul_pow7(n),
            8 => self.imul_pow8(n),
            9 => self.imul_pow9(n),
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
            _ => unreachable!(),
        }
    }

    /// Multiply by a power of 2.
    #[inline]
    fn imul_pow2(&mut self, n: u32) {
        self.ishl(n as usize)
    }

    imul_power!(imul_pow3, 3);

    /// Multiply by a power of 4.
    #[inline]
    fn imul_pow4(&mut self, n: u32) {
        self.imul_pow2(2 * n);
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
        self.imul_pow2(3 * n);
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
        self.imul_pow2(4 * n);
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
        self.imul_pow2(5 * n);
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
    fn idiv_small(&mut self, y: Limb) -> Limb {
        small::idiv(self.data_mut(), y)
    }

    /// Div small integer to a copy of self, and return the remainder.
    #[inline]
    fn div_small(&self, y: Limb) -> (Self, Limb) {
        let mut x = self.clone();
        let rem = x.idiv_small(y);
        (x, rem)
    }

    // POWER

    /// Calculate self^n
    #[inline]
    fn ipow(&mut self, n: Limb) {
        power::ipow(self.data_mut(), n);
    }

    /// Calculate self^n
    #[inline]
    fn pow(&self, n: Limb) -> Self {
        let mut x = self.clone();
        x.ipow(n);
        x
    }
}
