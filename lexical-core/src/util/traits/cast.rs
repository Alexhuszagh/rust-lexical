//! Rust cast utilities.
//!
//! High-level casts to use `as`-like casts in generic code.
//! This basically makes the entire type system work.

use super::primitive::AsPrimitive;

// AS CAST
// -------

/// Allows the high-level conversion of generic types as if `as` was used.
#[inline]
pub(crate) fn as_cast<U: AsCast, T: AsCast>(t: T) -> U {
    AsCast::as_cast(t)
}

/// An interface for casting between machine scalars.
#[doc(hidden)]
pub trait AsCast: AsPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `AsPrimitive` trait.
    fn as_cast<N: AsPrimitive>(n: N) -> Self;
}

macro_rules! as_cast {
    ($($t:ty, $meth:ident ; )*) => ($(
        impl AsCast for $t {
            #[inline]
            fn as_cast<N: AsPrimitive>(n: N) -> $t {
                n.$meth()
            }
        }
    )*);
}

as_cast!(
    u8, as_u8 ;
    u16, as_u16 ;
    u32, as_u32 ;
    u64, as_u64 ;
    u128, as_u128 ;
    usize, as_usize ;
    i8, as_i8 ;
    i16, as_i16 ;
    i32, as_i32 ;
    i64, as_i64 ;
    i128, as_i128 ;
    isize, as_isize ;
    f32, as_f32 ;
    f64, as_f64 ;
);

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
}
