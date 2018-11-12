//! Utilities for float-to-string conversions.

use util::*;

// FLOAT HELPERS

/// Check if generic float is denormal.
macro_rules! is_denormal {
    ($f:ident, $mask:ident) => (($f.to_bits() & $mask) == 0)
}

/// Check if generic float is NaN or Infinite.
macro_rules! is_special {
    ($f:ident, $mask:ident) => (($f.to_bits() & $mask) == $mask)
}

/// Get exponent from float.
macro_rules! exponent {
    ($f:ident, $denormal:ident, $mask:ident, $size:ident, $bias:ident) => ({
        if $f.is_denormal() {
            return $denormal;
        }

        let bits = $f.to_bits();
        let biased_e = ((bits & $mask) >> $size) as i32;
        biased_e - $bias
    })
}

/// Get significand from float.
macro_rules! significand {
    ($f:ident, $fraction:ident, $hidden:ident) => ({
        let bits = $f.to_bits();
        let s = bits & $fraction;
        if !$f.is_denormal() {
          s + $hidden
        } else {
          s
        }
    })
}

/// Returns the next greater float. Returns +infinity on input +infinity.
///
/// Requires a positive number.
macro_rules! next {
    ($f:ident, $t:tt, $inf:ident) => ({
        let bits = $f.to_bits();
        if bits == $inf {
            return $t::from_bits($inf);
        }
        return $t::from_bits(bits + 1);
    })
}

/// Calculate the naive exponent from a minimal value.
///
/// Don't export this for float, since it's specialized for basen.
#[inline]
pub(crate) fn naive_exponent(d: f64, base: u64) -> i32
{
    // floor returns the minimal value, which is our
    // desired exponent
    // ln(1.1e-5) -> -4.95 -> -5
    // ln(1.1e5) -> -5.04 -> 5
    (floor_f64(ln_f64(d) / ln_f64(base as f64))) as i32
}

// BUFFER PARAMTERS

// The buffer is actually a size of 60, but use 64 since it's a power of 2.
// Simple, fast optimization.
// Since we're declaring a variable on the stack, and our power-of-two
// alignment dramatically improved atoi performance, do it.
// Use 256, actually, since we seem to have memory issues with 64-bits.
// Clearly not sufficient memory allocated for non-base10 values.
pub(crate) const MAX_FLOAT_SIZE: usize = 256;
pub(crate) const BUFFER_SIZE: usize = MAX_FLOAT_SIZE;

// NOTATION CHAR

/// Get the exponent notation character.
pub(crate) extern "C" fn exponent_notation_char(base: u64)
    -> u8
{
    unsafe {
        if base >= 15 { EXPONENT_BACKUP_CHAR } else { EXPONENT_DEFAULT_CHAR }
    }
}

// FILTER SPECIAL

/// Emit special digits from generic float.
macro_rules! emit_special {
    ($self:ident, $dest:ident, $exponent:ident, $fraction:ident) => ({
        let bits = $self.to_bits();
        let is_zero = *$self == 0.0;
        let is_special = (bits & $exponent) == $exponent;

        if is_zero {
            copy_nonoverlapping!(b"0.0".as_ptr(), $dest, 3);
            3
        } else if is_special {
            if bits & $fraction != 0 {
                copy_nonoverlapping!(NAN_STRING.as_ptr(), $dest, NAN_STRING.len());
                NAN_STRING.len() as i32
            } else {
                copy_nonoverlapping!(INFINITY_STRING.as_ptr(), $dest, INFINITY_STRING.len());
                INFINITY_STRING.len() as i32
            }
        } else {
            0
        }
    })
}

// TRAITS

/// Emit special digits for floating-point type.
pub(crate) trait Float<T: Sized>: Sized {
    /// Returns true if the float is a denormal.
    fn is_denormal(&self) -> bool;

    /// Returns true if the float is a NaN or Infinite.
    fn is_special(&self) -> bool;

    /// Get exponent from float.
    fn exponent(&self) -> i32;

    /// Get significand from float.
    fn significand(&self) -> T;

    /// Get next greater float.
    fn next(&self) -> Self;

    /// Emit digits for special floating-point numbers.
    ///
    /// The number must be non-negative.
    unsafe extern "C" fn emit_special(&self, dest: *mut u8) -> i32;
}

impl Float<u32> for f32 {
    #[inline]
    fn is_denormal(&self) -> bool {
        is_denormal!(self, F32_EXPONENT_MASK)
    }

    #[inline]
    fn is_special(&self) -> bool {
        is_special!(self, F32_EXPONENT_MASK)
    }

    #[inline]
    fn exponent(&self) -> i32 {
        exponent!(self, F32_DENORMAL_EXPONENT, F32_EXPONENT_MASK, F32_SIGNIFICAND_SIZE, F32_EXPONENT_BIAS)
    }

    #[inline]
    fn significand(&self) -> u32 {
        significand!(self, F32_FRACTION_MASK, F32_HIDDEN_BIT_MASK)
    }

    #[inline]
    fn next(&self) -> f32 {
        next!(self, f32, U32_INFINITY)
    }

    #[inline]
    unsafe extern "C" fn emit_special(&self, dest: *mut u8) -> i32 {
        emit_special!(self, dest, F32_EXPONENT_MASK, F32_FRACTION_MASK)
    }
}

impl Float<u64> for f64 {
    #[inline]
    fn is_denormal(&self) -> bool {
        is_denormal!(self, F64_EXPONENT_MASK)
    }

    #[inline]
    fn is_special(&self) -> bool {
        is_special!(self, F64_EXPONENT_MASK)
    }

    #[inline]
    fn exponent(&self) -> i32 {
        exponent!(self, F64_DENORMAL_EXPONENT, F64_EXPONENT_MASK, F64_SIGNIFICAND_SIZE, F64_EXPONENT_BIAS)
    }

    #[inline]
    fn significand(&self) -> u64 {
        significand!(self, F64_FRACTION_MASK, F64_HIDDEN_BIT_MASK)
    }

    #[inline]
    fn next(&self) -> f64 {
        next!(self, f64, U64_INFINITY)
    }

    #[inline]
    unsafe extern "C" fn emit_special(&self, dest: *mut u8) -> i32 {
        emit_special!(self, dest, F64_EXPONENT_MASK, F64_FRACTION_MASK)
    }
}
