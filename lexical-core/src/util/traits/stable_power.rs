//! FLoating point power utilities.

use crate::util::powers::TablePower;

use super::cast::as_cast;
use super::num::Float;

// STABLE POWER

/// Calculate 2^exponent assigned straight from bits.
#[cfg(feature = "power_of_two")]
macro_rules! bitwise_pow2 {
    ($exponent:ident, $float:ty, $unsigned:ty) => {{
        debug_assert!(
            $exponent + <$float>::EXPONENT_BIAS - 1 >= 0,
            "table_pow2() have negative exponent."
        );

        // Say we have (for f32):
        //     BIAS = 127
        //     MANT_SIZE = 23
        // Then, we have denormal floats and normal floats that take
        // the following form:
        //
        // Denormal floats are [-BIAS-MANT_SIZE, -BIAS]
        //     Take form of S00000000MMMMMMMMMMMMMMMMMMMMMMM
        // Normal floats are [-BIAS+1, BIAS]
        //     Take form of SEEEEEEE100000000000000000000000
        //     Where S = Sign, E = Exponent, and M = Mantissa.

        // We adjust our exp bias here so we can find denormal floats.
        const MIN_EXP: i32 = <$float>::EXPONENT_BIAS - 1;
        const BIAS: i32 = <$float>::EXPONENT_BIAS - <$float>::MANTISSA_SIZE;
        if $exponent <= -BIAS {
            // Denormal float, can calculate it based off the shift.
            let shift = $exponent + MIN_EXP;
            <$float>::from_bits(1 as $unsigned << shift)
        } else {
            // Normal float, just shift to the bias.
            // Remember: we're not using the EXPONENT_BIAS here because
            // we assume we're having a value in the hidden bit,
            // which is `1 << MANTISSA_SIZE`. We therefore
            // need to subtract MANTISSA_SIZE from our bias to calculate
            // the float as the form `2^exponent`.
            let biased_e = ($exponent + BIAS) as $unsigned;
            <$float>::from_bits(biased_e << <$float>::MANTISSA_SIZE)
        }
    }};
}

/// Stable power implementations for increased numeric stability.
pub trait StablePower: TablePower + Float {
    // ITERATIVE

    /// Get max exponent for `iterative_pow`.
    fn iterative_max(base: u32) -> i32;

    /// Get exponent step for `iterative_pow`.
    fn iterative_step(base: u32) -> i32;

    /// Calculate base^n iteratively for better numeric stability.
    #[inline]
    fn iterative_pow(self, base: u32, exponent: i32) -> Self {
        let max = Self::iterative_max(base);
        if exponent > max {
            // Value is impossibly large, must be infinity.
            Self::INFINITY
        } else if exponent < -max {
            // Value is impossibly small, must be 0.
            Self::ZERO
        } else {
            self.iterative_pow_finite(base, exponent)
        }
    }

    /// Calculate base^n iteratively for a finite result.
    #[inline]
    fn iterative_pow_finite(mut self, base: u32, mut exponent: i32) -> Self {
        let step = Self::iterative_step(base);
        let base: Self = as_cast(base);
        if exponent < 0 {
            // negative exponent, use division for numeric stability
            while exponent <= -step {
                exponent += step;
                self /= base.powi(step)
            }
            if exponent != 0 {
                self /= base.powi(-exponent)
            }
            self
        } else {
            // positive exponent
            while exponent >= step {
                exponent -= step;
                self *= base.powi(step)
            }
            if exponent != 0 {
                self *= base.powi(exponent)
            }
            self
        }
    }

    // POW2

    /// Calculate power of 2 using precalculated table.
    #[cfg(feature = "power_of_two")]
    fn pow2(self, exponent: i32) -> Self;

    // POW

    /// Calculate power of n using precalculated table.
    #[inline]
    fn pow(self, base: u32, exponent: i32) -> Self {
        if exponent > 0 {
            self * Self::table_pow(base, exponent)
        } else {
            self / Self::table_pow(base, -exponent)
        }
    }
}

// F32

impl StablePower for f32 {
    #[inline]
    #[cfg(feature = "power_of_two")]
    fn pow2(self, exponent: i32) -> f32 {
        self * bitwise_pow2!(exponent, f32, u32)
    }

    fn iterative_max(radix: u32) -> i32 {
        // Cached max exponents.
        // Make sure the value is >= 2*log(1e45, radix), which guarantees the
        // value overflows or underflows.
        debug_assert_radix!(radix);
        match radix {
            2 => 150,
            3 => 100,
            4 => 80,
            5 => 70,
            6 => 60,
            7 => 60,
            8 => 50,
            9 => 50,
            10 => 50,
            11 => 50,
            12 => 50,
            13 => 50,
            14 => 40,
            15 => 40,
            16 => 40,
            17 => 40,
            18 => 40,
            19 => 40,
            20 => 40,
            21 => 40,
            22 => 40,
            23 => 40,
            24 => 40,
            25 => 40,
            26 => 40,
            27 => 40,
            28 => 40,
            29 => 40,
            30 => 40,
            31 => 40,
            32 => 30,
            33 => 30,
            34 => 30,
            35 => 30,
            36 => 30,
            // Invalid radix.
            _ => unreachable!(),
        }
    }

    fn iterative_step(radix: u32) -> i32 {
        // Cached powers to get the desired exponent.
        // Make sure all values are < 1e25.
        debug_assert_radix!(radix);
        match radix {
            2 => 90,
            3 => 60,
            4 => 50,
            5 => 40,
            6 => 40,
            7 => 30,
            8 => 30,
            9 => 30,
            10 => 30,
            11 => 30,
            12 => 30,
            13 => 30,
            14 => 30,
            15 => 30,
            16 => 30,
            17 => 30,
            18 => 20,
            19 => 20,
            20 => 20,
            21 => 20,
            22 => 20,
            23 => 20,
            24 => 20,
            25 => 20,
            26 => 20,
            27 => 20,
            28 => 20,
            29 => 20,
            30 => 20,
            31 => 20,
            32 => 20,
            33 => 20,
            34 => 20,
            35 => 20,
            36 => 20,
            // Invalid radix.
            _ => unreachable!(),
        }
    }
}

// F64

impl StablePower for f64 {
    #[inline]
    #[cfg(feature = "power_of_two")]
    fn pow2(self, exponent: i32) -> f64 {
        self * bitwise_pow2!(exponent, f64, u64)
    }

    fn iterative_max(radix: u32) -> i32 {
        // Cached max exponents.
        // Make sure the value is >= 2*log(5e324, radix), which guarantees the
        // value overflows or underflows.
        debug_assert_radix!(radix);
        match radix {
            2 => 2200,
            3 => 1400,
            4 => 1200,
            5 => 1000,
            6 => 900,
            7 => 800,
            8 => 750,
            9 => 700,
            10 => 650,
            11 => 625,
            12 => 625,
            13 => 600,
            14 => 575,
            15 => 575,
            16 => 550,
            17 => 550,
            18 => 525,
            19 => 525,
            20 => 500,
            21 => 500,
            22 => 500,
            23 => 500,
            24 => 475,
            25 => 475,
            26 => 475,
            27 => 475,
            28 => 450,
            29 => 450,
            30 => 450,
            31 => 450,
            32 => 450,
            33 => 450,
            34 => 425,
            35 => 425,
            36 => 425,
            // Invalid radix.
            _ => unreachable!(),
        }
    }

    fn iterative_step(radix: u32) -> i32 {
        // Cached powers to get the desired exponent.
        // Make sure all values are < 1e300.
        debug_assert_radix!(radix);
        match radix {
            2 => 512,
            3 => 512,
            4 => 256,
            5 => 256,
            6 => 256,
            7 => 256,
            8 => 256,
            9 => 256,
            10 => 256,
            11 => 256,
            12 => 256,
            13 => 256,
            14 => 256,
            15 => 128,
            16 => 128,
            17 => 128,
            18 => 128,
            19 => 128,
            20 => 128,
            21 => 128,
            22 => 128,
            23 => 128,
            24 => 128,
            25 => 128,
            26 => 128,
            27 => 128,
            28 => 128,
            29 => 128,
            30 => 128,
            31 => 128,
            32 => 128,
            33 => 128,
            34 => 128,
            35 => 128,
            36 => 128,
            // Invalid radix.
            _ => unreachable!(),
        }
    }
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    use approx::assert_relative_eq;

    #[test]
    fn f32_iterative_pow_finite_test() {
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 38), 1e38, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 30), 1e30, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 25), 1e25, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 20), 1e20, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 15), 1e15, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 10), 1e10, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 5), 1e5, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -5), 1e-5, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -10), 1e-10, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -15), 1e-15, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -20), 1e-20, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -25), 1e-25, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -30), 1e-30, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -38), 1e-38, max_relative = 1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -45), 1e-45, max_relative = 1e-6);

        // overflow
        assert!(f32::iterative_pow_finite(1.0, 10, 39).is_infinite());

        // underflow
        assert_eq!(f32::iterative_pow_finite(1.0, 10, -46), 0.0);
    }

    #[test]
    fn f32_iterative_pow_test() {
        assert_relative_eq!(f32::iterative_pow(1.0, 10, 10), 1e10, max_relative = 1e-15);
        assert!(f32::iterative_pow(1.0, 10, 1000).is_infinite());
        assert_eq!(f32::iterative_pow(1.0, 10, -1000), 0.0);

        // overflow
        assert!(f32::iterative_pow(1.0, 10, 39).is_infinite());

        // underflow
        assert_eq!(f32::iterative_pow(1.0, 10, -46), 0.0);
    }

    #[test]
    fn f64_iterative_pow_finite_test() {
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 308), 1e308, max_relative = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 300), 1e300, max_relative = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 200), 1e200, max_relative = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 100), 1e100, max_relative = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 50), 1e50, max_relative = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -50), 1e-50, epsilon = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -100), 1e-100, epsilon = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -200), 1e-200, epsilon = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -300), 1e-300, epsilon = 1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -308), 1e-308, epsilon = 1e-15);

        // This only affects armv6 and not armv7, but we'll skip this test
        // both, since `target_arch` does not differentiate between
        // the two.
        #[cfg(not(all(target_arch = "arm", not(target_feature = "v7"))))]
        assert_eq!(f64::iterative_pow_finite(5.0, 10, -324), 5e-324);

        // overflow
        assert!(f64::iterative_pow_finite(1.0, 10, 309).is_infinite());

        // underflow
        assert_eq!(f64::iterative_pow_finite(1.0, 10, -325), 0.0);
    }

    #[test]
    fn f64_iterative_pow_test() {
        assert_relative_eq!(f64::iterative_pow(1.0, 10, 50), 1e50, max_relative = 1e-15);
        assert!(f64::iterative_pow(1.0, 10, 1000).is_infinite());
        assert_eq!(f64::iterative_pow(1.0, 10, -1000), 0.0);

        // overflow
        assert!(f64::iterative_pow(1.0, 10, 309).is_infinite());

        // underflow
        assert_eq!(f64::iterative_pow(1.0, 10, -325), 0.0);
    }

    // These tests are ignored so we can test them on x86_64, where
    // we know powi has some guarantees. table_pow2 assigns directly
    // from bits, and therefore will always be accurate, we
    // just do a smoke test here.

    #[test]
    #[ignore]
    #[cfg(feature = "power_of_two")]
    fn test_f32_roundtrip() {
        // Check our logic is correct: by using a large type, we should
        // ensure our table_pow2 function is valid.
        for exp in -149i32..127 {
            let float = f32::pow2(1.0, exp);
            assert_eq!(float, f64::powi(2.0, exp) as f32);
        }
    }

    #[test]
    #[cfg(feature = "power_of_two")]
    fn f32_pow2_test() {
        let (min, max) = f32::exponent_limit(2);
        for i in min + 1..max + 1 {
            assert_eq!(f32::pow2(1.0, i) / f32::pow2(1.0, i - 1), 2.0);
        }
        for i in 1..max + 1 {
            let f = f32::pow2(1.0, i);
            if f < u64::max_value() as f32 {
                assert_eq!((f as u64) as f32, f);
            }
        }
    }

    #[test]
    fn f32_pow_test() {
        // Only check positive, since negative values round during division.
        for b in BASE_POWN.iter().cloned() {
            let (_, max) = f32::exponent_limit(b);
            for i in 1..max + 1 {
                let f = f32::pow(1.0, b, i);
                let p = f32::pow(1.0, b, i - 1);
                assert_eq!(f / p, b as f32);
                if f < u64::max_value() as f32 {
                    assert_eq!((f as u64) as f32, f);
                }
            }
        }
    }

    #[test]
    #[ignore]
    #[cfg(feature = "power_of_two")]
    fn test_f64_roundtrip() {
        for exp in -1074i32..1023 {
            let float = f64::pow2(1.0, exp);
            if exp > -1023 {
                // Only check for normal floats, powi isn't stable for
                // denormal floats.
                assert_eq!(float, f64::powi(2.0, exp));
            }
        }
    }

    #[test]
    #[cfg(feature = "power_of_two")]
    fn f64_pow2_test() {
        let (min, max) = f64::exponent_limit(2);
        for i in min + 1..max + 1 {
            let curr = f64::pow2(1.0, i);
            let prev = f64::pow2(1.0, i - 1);
            assert_eq!(curr / prev, 2.0);
        }
        for i in 1..max + 1 {
            let f = f64::pow2(1.0, i);
            if f < u64::max_value() as f64 {
                assert_eq!((f as u64) as f64, f);
            }
        }
    }

    #[test]
    fn f64_pow_test() {
        // Only check positive, since negative values round during division.
        for b in BASE_POWN.iter().cloned() {
            let (_, max) = f64::exponent_limit(b);
            for i in 1..max + 1 {
                let f = f64::pow(1.0, b, i);
                let p = f64::pow(1.0, b, i - 1);
                assert_eq!(f / p, b as f64);
                if f < u64::max_value() as f64 {
                    assert_eq!((f as u64) as f64, f);
                }
            }
        }
    }
}
