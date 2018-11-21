//! Rust cast utilities.

use super::primitive::AsPrimitive;

/// Allows the high-level conversion of generic types as if `as` was used.
#[inline(always)]
pub fn as_cast<U: AsCast, T: AsCast>(n: T) -> U {
    AsCast::as_cast(n)
}

/// An interface for casting between machine scalars.
pub trait AsCast: AsPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `AsPrimitive` trait.
    fn as_cast<T: AsPrimitive>(n: T) -> Self;
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

// try_cast?
// TODO(ahuszagh) Need TryFrom, TryInto in so many words.
// TODO(ahuszagh) Need to check all my as_ casts, ensure they work.

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
