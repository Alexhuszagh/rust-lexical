//! FLoating point power utilities.

use super::cast::*;
use super::num::*;

#[cfg(any(test, not(feature = "imprecise")))]
use table::*;

// STABLE POWER

#[cfg(all(any(test, not(feature = "imprecise")), feature = "table"))]
pub(crate) trait StablePowerImpl: ExactExponent + Float + TablePower {}

#[cfg(all(any(test, not(feature = "imprecise")), not(feature = "table")))]
pub(crate) trait StablePowerImpl: ExactExponent + Float {}

#[cfg(not(any(test, not(feature = "imprecise"))))]
pub(crate) trait StablePowerImpl: Float {}

impl StablePowerImpl for f32 {}
impl StablePowerImpl for f64 {}

/// Stable power implementations for increased numeric stability.
pub(crate) trait StablePower: StablePowerImpl {
//    /// Calculate pow2 with numeric exponent.
//    #[cfg(any(test, not(feature = "imprecise")))]
//    unsafe fn pow2(self, exponent: i32) -> Self;
//
//    /// Calculate base^n with numeric exponent and base.
//    #[cfg(any(test, not(feature = "imprecise")))]
//    fn pow<T: Integer>(self, base: T, exponent: i32) -> Self;

    // ITERATIVE

    /// Get max exponent for `iterative_pow`.
    fn iterative_max<T: Integer>(base: T) -> i32;

    /// Get exponent step for `iterative_pow`.
    fn iterative_step<T: Integer>(base: T) -> i32;

    /// Calculate base^n iteratively for better numeric stability.
    #[inline]
    fn iterative_pow<T: Integer>(self, base: T, exponent: i32) -> Self {
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
    fn iterative_pow_finite<T: Integer>(mut self, base: T, mut exponent: i32) -> Self {
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

    /// Calculate a stable powi when the value is known to be >= -2*max && <= 2*max
    ///
    /// powi is not stable, even with exact values, at high or low exponents.
    /// However, doing it in 2 shots for exact values is exact.
    #[cfg(all(any(test, not(feature = "imprecise")), not(feature = "table")))]
    #[inline]
    unsafe fn pow2(self, exponent: i32) -> Self {
        const STEP: i32 = 75;
        if exponent > STEP {
            self * Self::TWO.powi(STEP) * Self::TWO.powi(exponent - STEP)
        } else if exponent < -STEP {
            self * Self::TWO.powi(-STEP) * Self::TWO.powi(exponent + STEP)
        } else {
            self * Self::TWO.powi(exponent)
        }
    }

    /// Calculate power of 2 using precalculated table.
    #[cfg(all(any(test, not(feature = "imprecise")), feature = "table"))]
    #[inline]
    unsafe fn pow2(self, exponent: i32) -> Self {
        self * Self::table_pow2(exponent)
    }

    // POW

    /// Calculate power of n using powi.
    #[cfg(all(any(test, not(feature = "imprecise")), not(feature = "table")))]
    #[inline]
    unsafe fn pow<T: Integer>(self, base: T, exponent: i32) -> Self {
        // Check the exponent is within bounds in debug builds.
        let (min, max) = Self::exponent_limit(base);
        debug_assert!(exponent >= min && exponent <= max);

        let base: Self = as_cast(base);
        self * base.powi(exponent)
    }

    /// Calculate power of n using precalculated table.
    #[cfg(all(any(test, not(feature = "imprecise")), feature = "table"))]
    #[inline]
    unsafe fn pow<T: Integer>(self, base: T, exponent: i32) -> Self {
        // Check the exponent is within bounds in debug builds.
        let (min, max) = Self::exponent_limit(base);
        debug_assert!(exponent >= min && exponent <= max);

        if exponent > 0 {
            self * Self::table_pow(base, exponent)
        } else {
            self / Self::table_pow(base, -exponent)
        }
    }
}

// F32

impl StablePower for f32 {
    fn iterative_max<T: Integer>(base: T) -> i32 {
        // Cached max exponents.
        // Make sure the value is >= 2*log(1e45, base), which guarantees the
        // value overflows or underflows.
        const MAX: [i32; 35] = [
            150, 100, 80, 70, 60, 60, 50, 50, 50, 50, 50, 50,
            40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
            40, 40, 40, 40, 40, 40, 30, 30, 30, 30, 30
        ];

        debug_assert!(base.as_i32() >= 2 && base.as_i32() <= 36, "Numerical base must be from 2-36");

        let idx: usize = as_cast(base.as_i32() - 2);
        unsafe { *MAX.get_unchecked(idx) }
    }

    fn iterative_step<T: Integer>(base: T) -> i32 {
        // Cached powers to get the desired exponent.
        // Make sure all values are < 1e25.
        const STEP: [i32; 35] = [
            90, 60, 50, 40, 40, 30, 30, 30, 30, 30, 30, 30,
            30, 30, 30, 30, 20, 20, 20, 20, 20, 20, 20, 20,
            20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20
        ];

        debug_assert!(base.as_i32() >= 2 && base.as_i32() <= 36, "Numerical base must be from 2-36");

        let idx: usize = as_cast(base.as_i32() - 2);
        unsafe { *STEP.get_unchecked(idx) }
    }
}

// F64

impl StablePower for f64 {
    fn iterative_max<T: Integer>(base: T) -> i32 {
        // Cached max exponents.
        // Make sure the value is >= 2*log(5e324, base), which guarantees the
        // value overflows or underflows.
        const MAX: [i32; 35] = [
            2200, 1400, 1200, 1000, 900, 800, 750, 700, 650, 625, 625, 600,
            575, 575, 550, 550, 525, 525, 500, 500, 500, 500, 475, 475,
            475, 475, 450, 450, 450, 450, 450, 450, 425, 425, 425
        ];

        debug_assert!(base.as_i32() >= 2 && base.as_i32() <= 36, "Numerical base must be from 2-36");

        let idx: usize = as_cast(base.as_i32() - 2);
        unsafe { *MAX.get_unchecked(idx) }
    }

    fn iterative_step<T: Integer>(base: T) -> i32 {
        // Cached powers to get the desired exponent.
        // Make sure all values are < 1e300.
        const STEP: [i32; 35] = [
            512, 512, 256, 256, 256, 256, 256, 256, 256, 256, 256, 256,
            256, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
            128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128
        ];

        debug_assert!(base.as_i32() >= 2 && base.as_i32() <= 36, "Numerical base must be from 2-36");

        let idx: usize = as_cast(base.as_i32() - 2);
        unsafe { *STEP.get_unchecked(idx) }
    }
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use table::*;
    use super::*;

    #[test]
    fn f32_iterative_pow_finite_test() {
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 38), 1e38, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 30), 1e30, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 25), 1e25, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 20), 1e20, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 15), 1e15, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 10), 1e10, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, 5), 1e5, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -5), 1e-5, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -10), 1e-10, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -15), 1e-15, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -20), 1e-20, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -25), 1e-25, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -30), 1e-30, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -38), 1e-38, max_relative=1e-6);
        assert_relative_eq!(f32::iterative_pow_finite(1.0, 10, -45), 1e-45, max_relative=1e-6);

        // overflow
        assert!(f32::iterative_pow_finite(1.0, 10, 39).is_infinite());

        // underflow
        assert_eq!(f32::iterative_pow_finite(1.0, 10, -46), 0.0);
    }

    #[test]
    fn f32_iterative_pow_test() {
        assert_relative_eq!(f32::iterative_pow(1.0, 10, 10), 1e10, max_relative=1e-15);
        assert!(f32::iterative_pow(1.0, 10, 1000).is_infinite());
        assert_eq!(f32::iterative_pow(1.0, 10, -1000), 0.0);

        // overflow
        assert!(f32::iterative_pow(1.0, 10, 39).is_infinite());

        // underflow
        assert_eq!(f32::iterative_pow(1.0, 10, -46), 0.0);
    }

    #[test]
    fn f64_iterative_pow_finite_test() {
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 308), 1e308, max_relative=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 300), 1e300, max_relative=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 200), 1e200, max_relative=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 100), 1e100, max_relative=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, 50), 1e50, max_relative=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -50), 1e-50, epsilon=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -100), 1e-100, epsilon=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -200), 1e-200, epsilon=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -300), 1e-300, epsilon=1e-15);
        assert_relative_eq!(f64::iterative_pow_finite(1.0, 10, -308), 1e-308, epsilon=1e-15);
        assert_eq!(f64::iterative_pow_finite(5.0, 10, -324), 5e-324);

        // overflow
        assert!(f64::iterative_pow_finite(1.0, 10, 309).is_infinite());

        // underflow
        assert_eq!(f64::iterative_pow_finite(1.0, 10, -325), 0.0);
    }

    #[test]
    fn f64_iterative_pow_test() {
        assert_relative_eq!(f64::iterative_pow(1.0, 10, 50), 1e50, max_relative=1e-15);
        assert!(f64::iterative_pow(1.0, 10, 1000).is_infinite());
        assert_eq!(f64::iterative_pow(1.0, 10, -1000), 0.0);

        // overflow
        assert!(f64::iterative_pow(1.0, 10, 309).is_infinite());

        // underflow
        assert_eq!(f64::iterative_pow(1.0, 10, -325), 0.0);
    }

    const BASEN: [u64; 30] = [
        3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21,
        22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36
    ];

    #[test]
    fn f32_pow2_test() {
        unsafe {
            let (min, max) = f32::exponent_limit(2);
            for i in min+1..max+1 {
                assert_eq!(f32::pow2(1.0, i) / f32::pow2(1.0, i-1), 2.0);
            }
            for i in 1..max+1 {
                let f = f32::pow2(1.0, i);
                if f < u64::max_value() as f32 {
                    assert_eq!((f as u64) as f32, f);
                }
            }
        }
    }

    #[test]
    fn f32_pow_test() {
        unsafe {
            // Only check positive, since negative values round during division.
            for b in BASEN.iter().cloned() {
                let (_, max) = f32::exponent_limit(b);
                for i in 1..max+1 {
                    let f = f32::pow(1.0, b, i);
                    let p = f32::pow(1.0, b, i-1);
                    assert_eq!(f / p, b as f32);
                    if f < u64::max_value() as f32 {
                        assert_eq!((f as u64) as f32, f);
                    }
                }
            }
        }
    }

    #[test]
    fn f64_pow2_test() {
        unsafe {
            let (min, max) = f64::exponent_limit(2);
            for i in min+1..max+1 {
                let curr = f64::pow2(1.0, i);
                let prev = f64::pow2(1.0, i-1);
                assert_eq!(curr / prev, 2.0);
            }
            for i in 1..max+1 {
                let f = f64::pow2(1.0, i);
                if f < u64::max_value() as f64 {
                    assert_eq!((f as u64) as f64, f);
                }
            }
        }
    }

    #[test]
    fn f64_pow_test() {
        unsafe {
            // Only check positive, since negative values round during division.
            for b in BASEN.iter().cloned() {
                let (_, max) = f64::exponent_limit(b);
                for i in 1..max+1 {
                    let f = f64::pow(1.0, b, i);
                    let p = f64::pow(1.0, b, i-1);
                    assert_eq!(f / p, b as f64);
                    if f < u64::max_value() as f64 {
                        assert_eq!((f as u64) as f64, f);
                    }
                }
            }
        }
    }
}
