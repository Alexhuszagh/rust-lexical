//! Utilities for float-to-string conversions.

use lib::ptr;
use util::*;

// TODO(ahuszagh) I can likely move this elsewhere....
// Doesn't seem to be a great module name.

// FLOAT HELPERS

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
    (d.ln() / (base as f64).ln()).floor() as i32
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

// EMIT FLOAT

/// Emit special digits for floating-point type.
pub(crate) trait EmitFloat: Float {
    /// Emit digits for special floating-point numbers.
    ///
    /// The number must be non-negative.
    #[inline]
    unsafe extern "C" fn emit_special(&self, dest: *mut u8) -> i32 {
        if self.is_zero() {
            ptr::copy_nonoverlapping(b"0.0".as_ptr(), dest, 3);
            3
        } else if self.is_special() {
            if self.is_nan() {
                ptr::copy_nonoverlapping(NAN_STRING.as_ptr(), dest, NAN_STRING.len());
                NAN_STRING.len() as i32
            } else {
                ptr::copy_nonoverlapping(INFINITY_STRING.as_ptr(), dest, INFINITY_STRING.len());
                INFINITY_STRING.len() as i32
            }
        } else {
            0
        }
    }
}

macro_rules! emit_float_impl {
    ($($t:ty)*) => ($(impl EmitFloat for $t {})*)
}

emit_float_impl! { f32 f64 }
