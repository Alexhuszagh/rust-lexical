//! Rust cast utilities.

use super::primitive::AsPrimitive;

// AS CAST

/// Allows the high-level conversion of generic types as if `as` was used.
#[inline(always)]
pub fn as_cast<U: AsCast, T: AsCast>(t: T) -> U {
    AsCast::as_cast(t)
}

/// An interface for casting between machine scalars.
pub trait AsCast: AsPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `AsPrimitive` trait.
    fn as_cast<N: AsPrimitive>(n: N) -> Self;
}

macro_rules! as_cast {
    ($t:ty, $meth:ident) => {
        impl AsCast for $t {
            #[inline(always)]
            fn as_cast<N: AsPrimitive>(n: N) -> $t {
                n.$meth()
            }
        }
    };
}

as_cast!(u8, as_u8);
as_cast!(u16, as_u16);
as_cast!(u32, as_u32);
as_cast!(u64, as_u64);
as_cast!(u128, as_u128);
as_cast!(usize, as_usize);
as_cast!(i8, as_i8);
as_cast!(i16, as_i16);
as_cast!(i32, as_i32);
as_cast!(i64, as_i64);
as_cast!(i128, as_i128);
as_cast!(isize, as_isize);
as_cast!(f32, as_f32);
as_cast!(f64, as_f64);

// TRY CAST
// Analogous to TryInto.

/// High-level conversion of types using TryCast.
#[inline(always)]
pub fn try_cast<U, T: TryCast<U>>(t: T) -> Option<U> {
    TryCast::try_cast(t)
}

/// An interface for casting between machine scalars.
pub trait TryCast<T>: Sized {
    /// Consume self and return the cast value (or None).
    fn try_cast(self) -> Option<T>;
}

macro_rules! try_cast {
    // Checked conversion
    (@check $v:ident, $cond:expr) => (if $cond { Some(as_cast($v)) } else { None });

    // Widen type,so no checks required, both are signed/unsigned.
    (@widen $src:tt, $($dst:tt),*) => ($(
        impl TryCast<$dst> for $src {
            #[inline]
            fn try_cast(self) -> Option<$dst> {
                try_cast!(@check self, true)
            }
        }
    )*);

    // Above zero check, for a signed to unsigned conversion of same width.
    (@positive $src:tt, $($dst:tt),*) => ($(
        impl TryCast<$dst> for $src {
            #[inline]
            fn try_cast(self) -> Option<$dst> {
                try_cast!(@check self, self >= 0)
            }
        }
    )*);

    // Check below some upper bound (for narrowing of an unsigned value).
    (@below $src:tt, $($dst:tt),*) => ($(
        impl TryCast<$dst> for $src {
            #[inline]
            fn try_cast(self) -> Option<$dst> {
                const MAX: $src = $dst::max_value() as $src;
                try_cast!(@check self, self <= MAX)
            }
        }
    )*);

    // Check within min and max bounds (for narrowing of a signed value).
    (@within $src:tt, $($dst:tt),*) => ($(
        impl TryCast<$dst> for $src {
            #[inline]
            fn try_cast(self) -> Option<$dst> {
                const MIN: $src = $dst::min_value() as $src;
                const MAX: $src = $dst::max_value() as $src;
                try_cast!(@check self, self >= MIN && self <= MAX)
            }
        }
    )*);
}

// u8
try_cast! { @widen u8, u8, u16, u32, u64, u128 }
try_cast! { @below u8, i8 }
try_cast! { @widen u8, i16, i32, i64, i128 }

// u16
try_cast! { @below u16, u8 }
try_cast! { @widen u16, u16, u32, u64, u128 }
try_cast! { @below u16, i8, i16 }
try_cast! { @widen u16, i32, i64, i128 }

// u32
try_cast! { @below u32, u8, u16 }
try_cast! { @widen u32, u32, u64, u128 }
try_cast! { @below u32, i8, i16, i32 }
try_cast! { @widen u32, i64, i128 }

// u64
try_cast! { @below u64, u8, u16, u32 }
try_cast! { @widen u64, u64, u128 }
try_cast! { @below u64, i8, i16, i32, i64 }
try_cast! { @widen u64, i128 }

// u128
try_cast! { @below u128, u8, u16, u32, u64 }
try_cast! { @widen u128, u128 }
try_cast! { @below u128, i8, i16, i32, i64, i128 }

// i8
try_cast! { @positive i8, u8, u16, u32, u64, u128 }
try_cast! { @widen i8, i8, i16, i32, i64, i128 }

// i16
try_cast! { @within i16, u8 }
try_cast! { @positive i16, u16, u32, u64, u128 }
try_cast! { @within i16, i8 }
try_cast! { @widen i16, i16, i32, i64, i128 }

// i32
try_cast! { @within i32, u8, u16 }
try_cast! { @positive i32, u32, u64, u128 }
try_cast! { @within i32, i8, i16 }
try_cast! { @widen i32, i32, i64, i128 }

// i64
try_cast! { @within i64, u8, u16, u32 }
try_cast! { @positive i64, u64, u128 }
try_cast! { @within i64, i8, i16, i32 }
try_cast! { @widen i64, i64, i128 }

// i128
try_cast! { @within i128, u8, u16, u32, u64 }
try_cast! { @positive i128, u128 }
try_cast! { @within i128, i8, i16, i32, i64 }
try_cast! { @widen i128, i128 }

cfg_if! {
if #[cfg(target_pointer_width = "16")] {
    // 16-bit usize
    try_cast! { @below usize, u8 }
    try_cast! { @widen usize, u16, u32, u64, u128, usize }
    try_cast! { @below usize, i8, i16, isize }
    try_cast! { @widen usize, i32, i64, i128 }

    // 16-bit isize
    try_cast! { @within isize, u8 }
    try_cast! { @positive isize, u16, u32, u64, u128, usize }
    try_cast! { @within isize, i8 }
    try_cast! { @widen isize, i16, i32, i64, i128, isize }
} else if #[cfg(target_pointer_width = "32")] {
    // 32-bit usize
    try_cast! { @below usize, u8, u16 }
    try_cast! { @widen usize, u32, u64, u128, usize }
    try_cast! { @below usize, i8, i16, i32, isize }
    try_cast! { @widen usize, i64, i128 }

    // 32-bit isize
    try_cast! { @within isize, u8, u16 }
    try_cast! { @positive isize, u32, u64, u128, usize }
    try_cast! { @within isize, i8, i16 }
    try_cast! { @widen isize, i32, i64, i128, isize }
} else if #[cfg(target_pointer_width = "64")] {
    // 64-bit usize
    try_cast! { @below usize, u8, u16, u32 }
    try_cast! { @widen usize, u64, u128, usize }
    try_cast! { @below usize, i8, i16, i32, i64, isize }
    try_cast! { @widen usize, i128 }

    // 64-bit isize
    try_cast! { @within isize, u8, u16, u32 }
    try_cast! { @positive isize, u64, u128, usize }
    try_cast! { @within isize, i8, i16, i32 }
    try_cast! { @widen isize, i64, i128, isize }
}}  // cfg_if


// TODO(ahuszagh) Implement the unittests
// TODO(ahuszagh) Need to check all my as_cast and as casts, ensure they work.

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    fn check_as_cast<T: AsCast>(t: T) {
        let _: i8 = as_cast(t);
        let _: i16 = as_cast(t);
        let _: i32 = as_cast(t);
        let _: i64 = as_cast(t);
        let _: i128 = as_cast(t);
        let _: isize = as_cast(t);
        let _: u8 = as_cast(t);
        let _: u16 = as_cast(t);
        let _: u32 = as_cast(t);
        let _: u64 = as_cast(t);
        let _: u128 = as_cast(t);
        let _: usize = as_cast(t);
        let _: f32 = as_cast(t);
        let _: f64 = as_cast(t);
    }

    #[test]
    fn as_cast_test() {
        check_as_cast(1u8);
        check_as_cast(1u16);
        check_as_cast(1u32);
        check_as_cast(1u64);
        check_as_cast(1u128);
        check_as_cast(1usize);
        check_as_cast(1i8);
        check_as_cast(1i16);
        check_as_cast(1i32);
        check_as_cast(1i64);
        check_as_cast(1i128);
        check_as_cast(1isize);
        check_as_cast(1f32);
        check_as_cast(1f64);
    }

    #[test]
    fn try_cast_test() {
        // TODO(ahuszagh) Implement...
        // u8
        // u16
        // u32
        // u64
    }
}
