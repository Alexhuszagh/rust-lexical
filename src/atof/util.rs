//! Utilities for string-to-float conversions.

use sealed::mem;
use util::*;

// TRAITS

/// Compatibility trait to allow "overflowing-checked" arithmetic with atoi.
/// Doesn't really wrap,or check overflows, uses IEE754 float semantics,
/// which return Infinity.
pub(crate) trait OverflowingFloat: Sized {
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    fn overflowing_mul(self, rhs: Self) -> (Self, bool);
}

macro_rules! overflowing_float_impl {
    ($($t:ty)*) => ($(
        impl OverflowingFloat for $t {
            #[inline(always)]
            fn overflowing_add(self, rhs: $t) -> ($t, bool) { (self + rhs, false) }

            #[inline(always)]
            fn overflowing_mul(self, rhs: $t) -> ($t, bool) { (self * rhs, false) }
        }
    )*)
}

overflowing_float_impl! { f32 f64 }

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
