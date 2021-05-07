//! Large operations for arbitrary-precision integers.

use crate::util::config::*;

use super::large;
use super::small_ops::*;

// LARGE OPS
// ---------

/// Trait for large operations for arbitrary-precision numbers.
#[allow(dead_code)]
pub(crate) trait LargeOps: SmallOps {
    // ADDITION

    /// AddAssign large integer.
    #[inline]
    fn iadd_large(&mut self, y: &Self) {
        large::iadd(self.data_mut(), y.data());
    }

    /// Add large integer to a copy of self.
    #[inline]
    fn add_large(&mut self, y: &Self) -> Self {
        let mut x = self.clone();
        x.iadd_large(y);
        x
    }

    // SUBTRACTION

    /// SubAssign large integer.
    /// Warning: Does no overflow checking, x must be >= y.
    #[inline]
    fn isub_large(&mut self, y: &Self) {
        large::isub(self.data_mut(), y.data());
    }

    /// Sub large integer to a copy of self.
    /// Warning: Does no overflow checking, x must be >= y.
    #[inline]
    fn sub_large(&mut self, y: &Self) -> Self {
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
    fn mul_large(&mut self, y: &Self) -> Self {
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
    fn div_large(&mut self, y: &Self) -> (Self, Self) {
        let mut x = self.clone();
        let rem = x.idiv_large(y);
        (x, rem)
    }

    /// Calculate the fast quotient for a single limb-bit quotient.
    ///
    /// This requires a non-normalized divisor, where there at least
    /// `integral_binary_factor` 0 bits set, to ensure at maximum a single
    /// digit will be produced for a single base.
    ///
    /// Warning: This is not a general-purpose division algorithm,
    /// it is highly specialized for peeling off singular digits.
    #[inline]
    fn quorem(&mut self, y: &Self) -> Limb {
        large::quorem(self.data_mut(), y.data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[derive(Clone, Default)]
    struct Bigint {
        data: DataType,
    }

    impl Bigint {
        #[inline]
        pub fn new() -> Bigint {
            Bigint {
                data: vector![],
            }
        }
    }

    impl SharedOps for Bigint {
        type StorageType = DataType;

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
        let x = Bigint {
            data: from_u32(&[1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(!x.greater(&y));
        assert!(!x.greater(&x));
        assert!(y.greater(&x));

        // Check asymmetric
        let x = Bigint {
            data: from_u32(&[5, 1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(x.greater(&y));
        assert!(!x.greater(&x));
        assert!(!y.greater(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint {
            data: from_u32(&[5, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[6, 2, 8]),
        };
        assert!(x.greater(&y));
        assert!(!x.greater(&x));
        assert!(!y.greater(&x));

        // Complex scenario, check it properly uses reverse ordering.
        let x = Bigint {
            data: from_u32(&[0, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[4294967295, 0, 9]),
        };
        assert!(x.greater(&y));
        assert!(!x.greater(&x));
        assert!(!y.greater(&x));
    }

    #[test]
    fn greater_equal_test() {
        // Simple
        let x = Bigint {
            data: from_u32(&[1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(!x.greater_equal(&y));
        assert!(x.greater_equal(&x));
        assert!(y.greater_equal(&x));

        // Check asymmetric
        let x = Bigint {
            data: from_u32(&[5, 1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(x.greater_equal(&y));
        assert!(x.greater_equal(&x));
        assert!(!y.greater_equal(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint {
            data: from_u32(&[5, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[6, 2, 8]),
        };
        assert!(x.greater_equal(&y));
        assert!(x.greater_equal(&x));
        assert!(!y.greater_equal(&x));

        // Complex scenario, check it properly uses reverse ordering.
        let x = Bigint {
            data: from_u32(&[0, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[4294967295, 0, 9]),
        };
        assert!(x.greater_equal(&y));
        assert!(x.greater_equal(&x));
        assert!(!y.greater_equal(&x));
    }

    #[test]
    fn equal_test() {
        // Simple
        let x = Bigint {
            data: from_u32(&[1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(!x.equal(&y));
        assert!(x.equal(&x));
        assert!(!y.equal(&x));

        // Check asymmetric
        let x = Bigint {
            data: from_u32(&[5, 1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(!x.equal(&y));
        assert!(x.equal(&x));
        assert!(!y.equal(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint {
            data: from_u32(&[5, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[6, 2, 8]),
        };
        assert!(!x.equal(&y));
        assert!(x.equal(&x));
        assert!(!y.equal(&x));

        // Complex scenario, check it properly uses reverse ordering.
        let x = Bigint {
            data: from_u32(&[0, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[4294967295, 0, 9]),
        };
        assert!(!x.equal(&y));
        assert!(x.equal(&x));
        assert!(!y.equal(&x));
    }

    #[test]
    fn less_test() {
        // Simple
        let x = Bigint {
            data: from_u32(&[1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(x.less(&y));
        assert!(!x.less(&x));
        assert!(!y.less(&x));

        // Check asymmetric
        let x = Bigint {
            data: from_u32(&[5, 1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(!x.less(&y));
        assert!(!x.less(&x));
        assert!(y.less(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint {
            data: from_u32(&[5, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[6, 2, 8]),
        };
        assert!(!x.less(&y));
        assert!(!x.less(&x));
        assert!(y.less(&x));

        // Complex scenario, check it properly uses reverse ordering.
        let x = Bigint {
            data: from_u32(&[0, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[4294967295, 0, 9]),
        };
        assert!(!x.less(&y));
        assert!(!x.less(&x));
        assert!(y.less(&x));
    }

    #[test]
    fn less_equal_test() {
        // Simple
        let x = Bigint {
            data: from_u32(&[1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(x.less_equal(&y));
        assert!(x.less_equal(&x));
        assert!(!y.less_equal(&x));

        // Check asymmetric
        let x = Bigint {
            data: from_u32(&[5, 1]),
        };
        let y = Bigint {
            data: from_u32(&[2]),
        };
        assert!(!x.less_equal(&y));
        assert!(x.less_equal(&x));
        assert!(y.less_equal(&x));

        // Check when we use reverse ordering properly.
        let x = Bigint {
            data: from_u32(&[5, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[6, 2, 8]),
        };
        assert!(!x.less_equal(&y));
        assert!(x.less_equal(&x));
        assert!(y.less_equal(&x));

        // Complex scenario, check it properly uses reverse ordering.
        let x = Bigint {
            data: from_u32(&[0, 1, 9]),
        };
        let y = Bigint {
            data: from_u32(&[4294967295, 0, 9]),
        };
        assert!(!x.less_equal(&y));
        assert!(x.less_equal(&x));
        assert!(y.less_equal(&x));
    }

    #[test]
    fn leading_zero_limbs_test() {
        assert_eq!(Bigint::new().leading_zero_limbs(), 0);

        assert_eq!(Bigint::from_u16(0xF).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u32(0xFF).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u64(0xFF00000000).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u128(0xFF000000000000000000000000).leading_zero_limbs(), 0);

        assert_eq!(Bigint::from_u16(0xF).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u32(0xF).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u64(0xF00000000).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u128(0xF000000000000000000000000).leading_zero_limbs(), 0);

        assert_eq!(Bigint::from_u16(0xF0).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u32(0xF0).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u64(0xF000000000).leading_zero_limbs(), 0);
        assert_eq!(Bigint::from_u128(0xF0000000000000000000000000).leading_zero_limbs(), 0);
    }

    #[test]
    fn trailing_zero_limbs_test() {
        assert_eq!(Bigint::new().trailing_zero_limbs(), 0);

        assert_eq!(
            Bigint {
                data: vector![0xFF]
            }
            .trailing_zero_limbs(),
            0
        );
        assert_eq!(
            Bigint {
                data: vector![0, 0xFF000]
            }
            .trailing_zero_limbs(),
            1
        );
        assert_eq!(
            Bigint {
                data: vector![0, 0, 0, 0xFF000]
            }
            .trailing_zero_limbs(),
            3
        );
    }

    #[test]
    fn leading_zeros_test() {
        assert_eq!(Bigint::new().leading_zeros(), 0);

        assert_eq!(Bigint::from_u16(0xFF).leading_zeros(), <Limb as Integer>::BITS - 8);
        assert_eq!(Bigint::from_u32(0xFF).leading_zeros(), <Limb as Integer>::BITS - 8);
        assert_eq!(Bigint::from_u64(0xFF00000000).leading_zeros(), 24);
        assert_eq!(Bigint::from_u128(0xFF000000000000000000000000).leading_zeros(), 24);

        assert_eq!(Bigint::from_u16(0xF).leading_zeros(), <Limb as Integer>::BITS - 4);
        assert_eq!(Bigint::from_u32(0xF).leading_zeros(), <Limb as Integer>::BITS - 4);
        assert_eq!(Bigint::from_u64(0xF00000000).leading_zeros(), 28);
        assert_eq!(Bigint::from_u128(0xF000000000000000000000000).leading_zeros(), 28);

        assert_eq!(Bigint::from_u16(0xF0).leading_zeros(), <Limb as Integer>::BITS - 8);
        assert_eq!(Bigint::from_u32(0xF0).leading_zeros(), <Limb as Integer>::BITS - 8);
        assert_eq!(Bigint::from_u64(0xF000000000).leading_zeros(), 24);
        assert_eq!(Bigint::from_u128(0xF0000000000000000000000000).leading_zeros(), 24);
    }

    #[test]
    fn trailing_zeros_test() {
        assert_eq!(Bigint::new().trailing_zeros(), 0);

        assert_eq!(Bigint::from_u16(0xFF).trailing_zeros(), 0);
        assert_eq!(Bigint::from_u32(0xFF).trailing_zeros(), 0);
        assert_eq!(Bigint::from_u64(0xFF00000000).trailing_zeros(), 32);
        assert_eq!(Bigint::from_u128(0xFF000000000000000000000000).trailing_zeros(), 96);

        assert_eq!(Bigint::from_u16(0xF).trailing_zeros(), 0);
        assert_eq!(Bigint::from_u32(0xF).trailing_zeros(), 0);
        assert_eq!(Bigint::from_u64(0xF00000000).trailing_zeros(), 32);
        assert_eq!(Bigint::from_u128(0xF000000000000000000000000).trailing_zeros(), 96);

        assert_eq!(Bigint::from_u16(0xF0).trailing_zeros(), 4);
        assert_eq!(Bigint::from_u32(0xF0).trailing_zeros(), 4);
        assert_eq!(Bigint::from_u64(0xF000000000).trailing_zeros(), 36);
        assert_eq!(Bigint::from_u128(0xF0000000000000000000000000).trailing_zeros(), 100);
    }

    #[test]
    fn hi32_test() {
        assert_eq!(Bigint::from_u16(0xA).hi32(), (0xA0000000, false));
        assert_eq!(Bigint::from_u32(0xAB).hi32(), (0xAB000000, false));
        assert_eq!(Bigint::from_u64(0xAB00000000).hi32(), (0xAB000000, false));
        assert_eq!(Bigint::from_u64(0xA23456789A).hi32(), (0xA2345678, true));
    }

    #[test]
    fn hi64_test() {
        assert_eq!(Bigint::from_u16(0xA).hi64(), (0xA000000000000000, false));
        assert_eq!(Bigint::from_u32(0xAB).hi64(), (0xAB00000000000000, false));
        assert_eq!(Bigint::from_u64(0xAB00000000).hi64(), (0xAB00000000000000, false));
        assert_eq!(Bigint::from_u64(0xA23456789A).hi64(), (0xA23456789A000000, false));
        assert_eq!(
            Bigint::from_u128(0xABCDEF0123456789ABCDEF0123).hi64(),
            (0xABCDEF0123456789, true)
        );
    }

    #[test]
    fn hi128_test() {
        assert_eq!(
            Bigint::from_u128(0xABCDEF0123456789ABCDEF0123).hi128(),
            (0xABCDEF0123456789ABCDEF0123000000, false)
        );
        assert_eq!(
            Bigint::from_u128(0xABCDEF0123456789ABCDEF0123456789).hi128(),
            (0xABCDEF0123456789ABCDEF0123456789, false)
        );
        assert_eq!(
            Bigint {
                data: from_u32(&[0x34567890, 0xBCDEF012, 0x3456789A, 0xBCDEF012, 0xA])
            }
            .hi128(),
            (0xABCDEF0123456789ABCDEF0123456789, false)
        );
        assert_eq!(
            Bigint {
                data: from_u32(&[0x34567891, 0xBCDEF012, 0x3456789A, 0xBCDEF012, 0xA])
            }
            .hi128(),
            (0xABCDEF0123456789ABCDEF0123456789, true)
        );
    }

    #[test]
    fn pad_zero_digits_test() {
        let mut x = Bigint {
            data: vector![0, 0, 0, 1],
        };
        x.pad_zero_digits(3);
        assert_eq!(x.data.as_slice(), &[0, 0, 0, 0, 0, 0, 1]);

        let mut x = Bigint {
            data: vector![1],
        };
        x.pad_zero_digits(1);
        assert_eq!(x.data.as_slice(), &[0, 1]);
    }

    #[test]
    fn shl_test() {
        // Pattern generated via `''.join(["1" +"0"*i for i in range(20)])`
        let mut big = Bigint {
            data: from_u32(&[0xD2210408]),
        };
        big.ishl(5);
        assert_eq!(big.data, from_u32(&[0x44208100, 0x1A]));
        big.ishl(32);
        assert_eq!(big.data, from_u32(&[0, 0x44208100, 0x1A]));
        big.ishl(27);
        assert_eq!(big.data, from_u32(&[0, 0, 0xD2210408]));

        // 96-bits of previous pattern
        let mut big = Bigint {
            data: from_u32(&[0x20020010, 0x8040100, 0xD2210408]),
        };
        big.ishl(5);
        assert_eq!(big.data, from_u32(&[0x400200, 0x802004, 0x44208101, 0x1A]));
        big.ishl(32);
        assert_eq!(big.data, from_u32(&[0, 0x400200, 0x802004, 0x44208101, 0x1A]));
        big.ishl(27);
        assert_eq!(big.data, from_u32(&[0, 0, 0x20020010, 0x8040100, 0xD2210408]));
    }

    #[test]
    fn shr_test() {
        // Simple case.
        let mut big = Bigint {
            data: from_u32(&[0xD2210408]),
        };
        big.ishr(5, false);
        assert_eq!(big.data, from_u32(&[0x6910820]));
        big.ishr(27, false);
        assert_eq!(big.data, from_u32(&[]));

        // Pattern generated via `''.join(["1" +"0"*i for i in range(20)])`
        let mut big = Bigint {
            data: from_u32(&[0x20020010, 0x8040100, 0xD2210408]),
        };
        big.ishr(5, false);
        assert_eq!(big.data, from_u32(&[0x1001000, 0x40402008, 0x6910820]));
        big.ishr(32, false);
        assert_eq!(big.data, from_u32(&[0x40402008, 0x6910820]));
        big.ishr(27, false);
        assert_eq!(big.data, from_u32(&[0xD2210408]));

        // Check no-roundup with halfway and even
        let mut big = Bigint {
            data: from_u32(&[0xD2210408]),
        };
        big.ishr(3, true);
        assert_eq!(big.data, from_u32(&[0x1A442081]));
        big.ishr(1, true);
        assert_eq!(big.data, from_u32(&[0xD221040]));

        let mut big = Bigint {
            data: from_u32(&[0xD2210408]),
        };
        big.ishr(4, true);
        assert_eq!(big.data, from_u32(&[0xD221040]));

        // Check roundup with halfway and odd
        let mut big = Bigint {
            data: from_u32(&[0xD2210438]),
        };
        big.ishr(3, true);
        assert_eq!(big.data, from_u32(&[0x1A442087]));
        big.ishr(1, true);
        assert_eq!(big.data, from_u32(&[0xD221044]));

        let mut big = Bigint {
            data: from_u32(&[0xD2210438]),
        };
        big.ishr(5, true);
        assert_eq!(big.data, from_u32(&[0x6910822]));
    }

    #[test]
    fn bit_length_test() {
        let x = Bigint {
            data: from_u32(&[0, 0, 0, 1]),
        };
        assert_eq!(x.bit_length(), 97);

        let x = Bigint {
            data: from_u32(&[0, 0, 0, 3]),
        };
        assert_eq!(x.bit_length(), 98);

        let x = Bigint {
            data: from_u32(&[1 << 31]),
        };
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
        let mut x = Bigint {
            data: from_u32(&[4294967295]),
        };
        x.iadd_small(5);
        assert_eq!(x.data, from_u32(&[4, 1]));

        // No overflow, single value
        let mut x = Bigint {
            data: from_u32(&[5]),
        };
        x.iadd_small(7);
        assert_eq!(x.data, from_u32(&[12]));

        // Single carry, internal overflow
        let mut x = Bigint::from_u64(0x80000000FFFFFFFF);
        x.iadd_small(7);
        assert_eq!(x.data, from_u32(&[6, 0x80000001]));

        // Double carry, overflow
        let mut x = Bigint::from_u64(0xFFFFFFFFFFFFFFFF);
        x.iadd_small(7);
        assert_eq!(x.data, from_u32(&[6, 0, 1]));
    }

    #[test]
    fn isub_small_test() {
        // Overflow check (single)
        let mut x = Bigint {
            data: from_u32(&[4, 1]),
        };
        x.isub_small(5);
        assert_eq!(x.data, from_u32(&[4294967295]));

        // No overflow, single value
        let mut x = Bigint {
            data: from_u32(&[12]),
        };
        x.isub_small(7);
        assert_eq!(x.data, from_u32(&[5]));

        // Single carry, internal overflow
        let mut x = Bigint {
            data: from_u32(&[6, 0x80000001]),
        };
        x.isub_small(7);
        assert_eq!(x.data, from_u32(&[0xFFFFFFFF, 0x80000000]));

        // Double carry, overflow
        let mut x = Bigint {
            data: from_u32(&[6, 0, 1]),
        };
        x.isub_small(7);
        assert_eq!(x.data, from_u32(&[0xFFFFFFFF, 0xFFFFFFFF]));
    }

    #[test]
    fn imul_small_test() {
        // No overflow check, 1-int.
        let mut x = Bigint {
            data: from_u32(&[5]),
        };
        x.imul_small(7);
        assert_eq!(x.data, from_u32(&[35]));

        // No overflow check, 2-ints.
        let mut x = Bigint::from_u64(0x4000000040000);
        x.imul_small(5);
        assert_eq!(x.data, from_u32(&[0x00140000, 0x140000]));

        // Overflow, 1 carry.
        let mut x = Bigint {
            data: from_u32(&[0x33333334]),
        };
        x.imul_small(5);
        assert_eq!(x.data, from_u32(&[4, 1]));

        // Overflow, 1 carry, internal.
        let mut x = Bigint::from_u64(0x133333334);
        x.imul_small(5);
        assert_eq!(x.data, from_u32(&[4, 6]));

        // Overflow, 2 carries.
        let mut x = Bigint::from_u64(0x3333333333333334);
        x.imul_small(5);
        assert_eq!(x.data, from_u32(&[4, 0, 1]));
    }

    #[test]
    fn idiv_small_test() {
        let mut x = Bigint {
            data: from_u32(&[4]),
        };
        assert_eq!(x.idiv_small(7), 4);
        assert_eq!(x.data, from_u32(&[]));

        let mut x = Bigint {
            data: from_u32(&[3]),
        };
        assert_eq!(x.idiv_small(7), 3);
        assert_eq!(x.data, from_u32(&[]));

        // Check roundup, odd, halfway
        let mut x = Bigint {
            data: from_u32(&[15]),
        };
        assert_eq!(x.idiv_small(10), 5);
        assert_eq!(x.data, from_u32(&[1]));

        // Check 1 carry.
        let mut x = Bigint::from_u64(0x133333334);
        assert_eq!(x.idiv_small(5), 1);
        assert_eq!(x.data, from_u32(&[0x3D70A3D7]));

        // Check 2 carries.
        let mut x = Bigint::from_u64(0x3333333333333334);
        assert_eq!(x.idiv_small(5), 4);
        assert_eq!(x.data, from_u32(&[0xD70A3D70, 0xA3D70A3]));
    }

    #[test]
    fn ipow_test() {
        let x = Bigint {
            data: from_u32(&[5]),
        };
        assert_eq!(x.pow(2).data, from_u32(&[25]));
        assert_eq!(x.pow(15).data, from_u32(&[452807053, 7]));
        assert_eq!(x.pow(16).data, from_u32(&[2264035265, 35]));
        assert_eq!(x.pow(17).data, from_u32(&[2730241733, 177]));
        assert_eq!(
            x.pow(302).data,
            from_u32(&[
                2443090281, 2149694430, 2297493928, 1584384001, 1279504719, 1930002239, 3312868939,
                3735173465, 3523274756, 2025818732, 1641675015, 2431239749, 4292780461, 3719612855,
                4174476133, 3296847770, 2677357556, 638848153, 2198928114, 3285049351, 2159526706,
                626302612
            ])
        );
    }

    // LARGE OPS

    #[test]
    fn iadd_large_test() {
        // Overflow, both single values
        let mut x = Bigint {
            data: from_u32(&[4294967295]),
        };
        let y = Bigint {
            data: from_u32(&[5]),
        };
        x.iadd_large(&y);
        assert_eq!(x.data, from_u32(&[4, 1]));

        // No overflow, single value
        let mut x = Bigint {
            data: from_u32(&[5]),
        };
        let y = Bigint {
            data: from_u32(&[7]),
        };
        x.iadd_large(&y);
        assert_eq!(x.data, from_u32(&[12]));

        // Single carry, internal overflow
        let mut x = Bigint::from_u64(0x80000000FFFFFFFF);
        let y = Bigint {
            data: from_u32(&[7]),
        };
        x.iadd_large(&y);
        assert_eq!(x.data, from_u32(&[6, 0x80000001]));

        // 1st overflows, 2nd doesn't.
        let mut x = Bigint::from_u64(0x7FFFFFFFFFFFFFFF);
        let y = Bigint::from_u64(0x7FFFFFFFFFFFFFFF);
        x.iadd_large(&y);
        assert_eq!(x.data, from_u32(&[0xFFFFFFFE, 0xFFFFFFFF]));

        // Both overflow.
        let mut x = Bigint::from_u64(0x8FFFFFFFFFFFFFFF);
        let y = Bigint::from_u64(0x7FFFFFFFFFFFFFFF);
        x.iadd_large(&y);
        assert_eq!(x.data, from_u32(&[0xFFFFFFFE, 0x0FFFFFFF, 1]));
    }

    #[test]
    fn isub_large_test() {
        // Overflow, both single values
        let mut x = Bigint {
            data: from_u32(&[4, 1]),
        };
        let y = Bigint {
            data: from_u32(&[5]),
        };
        x.isub_large(&y);
        assert_eq!(x.data, from_u32(&[4294967295]));

        // No overflow, single value
        let mut x = Bigint {
            data: from_u32(&[12]),
        };
        let y = Bigint {
            data: from_u32(&[7]),
        };
        x.isub_large(&y);
        assert_eq!(x.data, from_u32(&[5]));

        // Single carry, internal overflow
        let mut x = Bigint {
            data: from_u32(&[6, 0x80000001]),
        };
        let y = Bigint {
            data: from_u32(&[7]),
        };
        x.isub_large(&y);
        assert_eq!(x.data, from_u32(&[0xFFFFFFFF, 0x80000000]));

        // Zeros out.
        let mut x = Bigint {
            data: from_u32(&[0xFFFFFFFF, 0x7FFFFFFF]),
        };
        let y = Bigint {
            data: from_u32(&[0xFFFFFFFF, 0x7FFFFFFF]),
        };
        x.isub_large(&y);
        assert_eq!(x.data, from_u32(&[]));

        // 1st overflows, 2nd doesn't.
        let mut x = Bigint {
            data: from_u32(&[0xFFFFFFFE, 0x80000000]),
        };
        let y = Bigint {
            data: from_u32(&[0xFFFFFFFF, 0x7FFFFFFF]),
        };
        x.isub_large(&y);
        assert_eq!(x.data, from_u32(&[0xFFFFFFFF]));
    }

    #[test]
    fn imul_large_test() {
        // Test by empty
        let mut x = Bigint {
            data: from_u32(&[0xFFFFFFFF]),
        };
        let y = Bigint {
            data: from_u32(&[]),
        };
        x.imul_large(&y);
        assert_eq!(x.data, from_u32(&[]));

        // Simple case
        let mut x = Bigint {
            data: from_u32(&[0xFFFFFFFF]),
        };
        let y = Bigint {
            data: from_u32(&[5]),
        };
        x.imul_large(&y);
        assert_eq!(x.data, from_u32(&[0xFFFFFFFB, 0x4]));

        // Large u32, but still just as easy.
        let mut x = Bigint {
            data: from_u32(&[0xFFFFFFFF]),
        };
        let y = Bigint {
            data: from_u32(&[0xFFFFFFFE]),
        };
        x.imul_large(&y);
        assert_eq!(x.data, from_u32(&[0x2, 0xFFFFFFFD]));

        // Let's multiply two large values together
        let mut x = Bigint {
            data: from_u32(&[0xFFFFFFFE, 0x0FFFFFFF, 1]),
        };
        let y = Bigint {
            data: from_u32(&[0x99999999, 0x99999999, 0xCCCD9999, 0xCCCC]),
        };
        x.imul_large(&y);
        assert_eq!(
            x.data,
            from_u32(&[0xCCCCCCCE, 0x5CCCCCCC, 0x9997FFFF, 0x33319999, 0x999A7333, 0xD999])
        );
    }

    #[test]
    fn imul_karatsuba_mul_test() {
        // Test cases triggered to use `karatsuba_mul`.
        let mut x = Bigint {
            data: from_u32(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
        };
        let y = Bigint {
            data: from_u32(&[4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]),
        };
        x.imul_large(&y);
        assert_eq!(
            x.data,
            from_u32(&[
                4, 13, 28, 50, 80, 119, 168, 228, 300, 385, 484, 598, 728, 875, 1040, 1224, 1340,
                1435, 1508, 1558, 1584, 1585, 1560, 1508, 1428, 1319, 1180, 1010, 808, 573, 304
            ])
        );

        // Test cases to use karatsuba_uneven_mul
        let mut x = Bigint {
            data: from_u32(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
        };
        let y = Bigint {
            data: from_u32(&[
                4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
                26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
            ]),
        };
        x.imul_large(&y);
        assert_eq!(
            x.data,
            from_u32(&[
                4, 13, 28, 50, 80, 119, 168, 228, 300, 385, 484, 598, 728, 875, 1040, 1224, 1360,
                1496, 1632, 1768, 1904, 2040, 2176, 2312, 2448, 2584, 2720, 2856, 2992, 3128, 3264,
                3400, 3536, 3672, 3770, 3829, 3848, 3826, 3762, 3655, 3504, 3308, 3066, 2777, 2440,
                2054, 1618, 1131, 592
            ])
        );
    }

    #[test]
    fn idiv_large_test() {
        // Simple case.
        let mut x = Bigint {
            data: from_u32(&[0xFFFFFFFF]),
        };
        let y = Bigint {
            data: from_u32(&[5]),
        };
        let rem = x.idiv_large(&y);
        assert_eq!(x.data, from_u32(&[0x33333333]));
        assert_eq!(rem.data, from_u32(&[0]));

        // Two integer case
        let mut x = Bigint {
            data: from_u32(&[0x2, 0xFFFFFFFF]),
        };
        let y = Bigint {
            data: from_u32(&[0xFFFFFFFE]),
        };
        let rem = x.idiv_large(&y);
        assert_eq!(x.data, from_u32(&[1, 1]));
        assert_eq!(rem.data, from_u32(&[4]));

        // Larger large case
        let mut x = Bigint {
            data: from_u32(&[0xCCCCCCCF, 0x5CCCCCCC, 0x9997FFFF, 0x33319999, 0x999A7333, 0xD999]),
        };
        let y = Bigint {
            data: from_u32(&[0x99999999, 0x99999999, 0xCCCD9999, 0xCCCC]),
        };
        let rem = x.idiv_large(&y);
        assert_eq!(x.data, from_u32(&[0xFFFFFFFE, 0x0FFFFFFF, 1]));
        assert_eq!(rem.data, from_u32(&[1]));

        // Extremely large cases, examples from Karatsuba multiplication.
        let mut x = Bigint {
            data: from_u32(&[
                4, 13, 29, 50, 80, 119, 168, 228, 300, 385, 484, 598, 728, 875, 1040, 1224, 1340,
                1435, 1508, 1558, 1584, 1585, 1560, 1508, 1428, 1319, 1180, 1010, 808, 573, 304,
            ]),
        };
        let y = Bigint {
            data: from_u32(&[4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]),
        };
        let rem = x.idiv_large(&y);
        assert_eq!(x.data, from_u32(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]));
        assert_eq!(rem.data, from_u32(&[0, 0, 1]));
    }

    #[test]
    fn quorem_test() {
        let mut x = Bigint::from_u128(42535295865117307932921825928971026432);
        let y = Bigint::from_u128(17218479456385750618067377696052635483);
        assert_eq!(x.quorem(&y), 2);
        assert_eq!(x.data, from_u32(&[1873752394, 3049207402, 3024501058, 102215382]));
    }
}
