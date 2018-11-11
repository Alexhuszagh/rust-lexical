//! Utilities for string-to-float conversions.

use sealed::mem;
use util::*;

// TRAITS

/// Compatibility trait to allow wrapping arithmetic with atoi.
/// Doesn't really wrap, uses IEE754 float semantics.
pub(crate) trait WrappingFloat: Sized {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
}

macro_rules! wrapping_float_impl {
    ($($t:ty)*) => ($(
        impl WrappingFloat for $t {
            #[inline(always)]
            fn wrapping_add(self, rhs: $t) -> $t { self + rhs }

            #[inline(always)]
            fn wrapping_mul(self, rhs: $t) -> $t { self * rhs }
        }
    )*)
}

wrapping_float_impl! { f32 f64 }

// STATE

/// Stores temporary state over atof
#[repr(C)]
pub(crate) struct State {
    /// Absolute start position.
    pub first: *const u8,
    /// Absolute last position.
    pub last: *const u8,
    /// Current first position.
    pub curr_first: *const u8,
    /// Current last position.
    pub curr_last: *const u8,
}

impl State {
    #[inline(always)]
    pub fn new(first: *const u8, last: *const u8) -> State {
        State {
            first: first,
            last: last,
            curr_first: unsafe { mem::uninitialized() },
            curr_last: unsafe { mem::uninitialized() }
        }
    }
}

// SPECIAL

#[inline(always)]
pub(crate) unsafe extern "C" fn is_nan(first: *const u8, length: usize)
    -> bool
{
    starts_with(first, length, NAN_STRING.as_ptr(), NAN_STRING.len())
}

#[inline(always)]
pub(crate) unsafe extern "C" fn is_infinity(first: *const u8, length: usize)
    -> bool
{
    starts_with(first, length, INFINITY_STRING.as_ptr(), INFINITY_STRING.len())
}

#[inline(always)]
pub(crate) unsafe extern "C" fn is_zero(first: *const u8, length: usize)
    -> bool
{
    length == 3 && equal_to(first, "0.0".as_ptr(), 3)
}
