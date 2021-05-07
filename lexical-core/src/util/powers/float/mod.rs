//! Pre-computed tables for exponent powers for floats.
//!
//! Conditionally compile the radix POWI tables.
//! These tables contain all the values that can be exactly represented
//! by a given float of a certain size.
//!
//! Total array storage: 2.1 KB (f32) + 21.5 KB (f64).
//! The total performance enhancements save ~350+ clock cycles (x86) or
//! ~100 clock cycles (x87) for the FYL2X and F2XM1 instructions, require
//! to compute a power. This should be a significant performance win.

// Hide modules.
mod decimal;
#[cfg(feature = "radix")]
mod radix;

// TABLE POW
// ---------

/// Calculate powers using pre-calculated lookup tables.
/// No error-checking occurs, these methods are not safe.
pub trait TablePower {
    /// Get power of 2 from exponent.
    fn table_pow(radix: u32, exponent: i32) -> Self;
}

impl TablePower for f32 {
    #[inline]
    fn table_pow(radix: u32, exponent: i32) -> f32 {
        debug_assert!(exponent >= 0, "table_pow() have negative exponent.");
        debug_assert_radix!(radix);
        let exponent = exponent as usize;

        #[cfg(not(feature = "radix"))]
        {
            debug_assert!(radix == 10, "radix must be 10");
            decimal::F32_POW10[exponent]
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                3 => radix::F32_POW3[exponent],
                5 => radix::F32_POW5[exponent],
                6 => radix::F32_POW6[exponent],
                7 => radix::F32_POW7[exponent],
                9 => radix::F32_POW9[exponent],
                10 => decimal::F32_POW10[exponent],
                11 => radix::F32_POW11[exponent],
                12 => radix::F32_POW12[exponent],
                13 => radix::F32_POW13[exponent],
                14 => radix::F32_POW14[exponent],
                15 => radix::F32_POW15[exponent],
                17 => radix::F32_POW17[exponent],
                18 => radix::F32_POW18[exponent],
                19 => radix::F32_POW19[exponent],
                20 => radix::F32_POW20[exponent],
                21 => radix::F32_POW21[exponent],
                22 => radix::F32_POW22[exponent],
                23 => radix::F32_POW23[exponent],
                24 => radix::F32_POW24[exponent],
                25 => radix::F32_POW25[exponent],
                26 => radix::F32_POW26[exponent],
                27 => radix::F32_POW27[exponent],
                28 => radix::F32_POW28[exponent],
                29 => radix::F32_POW29[exponent],
                30 => radix::F32_POW30[exponent],
                31 => radix::F32_POW31[exponent],
                33 => radix::F32_POW33[exponent],
                34 => radix::F32_POW34[exponent],
                35 => radix::F32_POW35[exponent],
                36 => radix::F32_POW36[exponent],
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}

impl TablePower for f64 {
    #[inline]
    fn table_pow(radix: u32, exponent: i32) -> f64 {
        debug_assert!(exponent >= 0, "table_pow() have negative exponent.");
        debug_assert_radix!(radix);
        let exponent = exponent as usize;

        #[cfg(not(feature = "radix"))]
        {
            debug_assert!(radix == 10, "radix must be 10");
            decimal::F64_POW10[exponent]
        }

        #[cfg(feature = "radix")]
        {
            match radix {
                3 => radix::F64_POW3[exponent],
                5 => radix::F64_POW5[exponent],
                6 => radix::F64_POW6[exponent],
                7 => radix::F64_POW7[exponent],
                9 => radix::F64_POW9[exponent],
                10 => decimal::F64_POW10[exponent],
                11 => radix::F64_POW11[exponent],
                12 => radix::F64_POW12[exponent],
                13 => radix::F64_POW13[exponent],
                14 => radix::F64_POW14[exponent],
                15 => radix::F64_POW15[exponent],
                17 => radix::F64_POW17[exponent],
                18 => radix::F64_POW18[exponent],
                19 => radix::F64_POW19[exponent],
                20 => radix::F64_POW20[exponent],
                21 => radix::F64_POW21[exponent],
                22 => radix::F64_POW22[exponent],
                23 => radix::F64_POW23[exponent],
                24 => radix::F64_POW24[exponent],
                25 => radix::F64_POW25[exponent],
                26 => radix::F64_POW26[exponent],
                27 => radix::F64_POW27[exponent],
                28 => radix::F64_POW28[exponent],
                29 => radix::F64_POW29[exponent],
                30 => radix::F64_POW30[exponent],
                31 => radix::F64_POW31[exponent],
                33 => radix::F64_POW33[exponent],
                34 => radix::F64_POW34[exponent],
                35 => radix::F64_POW35[exponent],
                36 => radix::F64_POW36[exponent],
                // Invalid radix
                _ => unreachable!(),
            }
        }
    }
}
