//! Overflowing add and mul for floats.

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
