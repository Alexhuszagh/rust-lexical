//! Rust cast utilities.

use super::primitive::AsPrimitive;

/// Allows the high-level conversion of generic types as if `as` was used.
#[inline(always)]
pub fn as_<U: AsCast, T: AsCast>(n: T) -> U {
    AsCast::as_(n)
}

/// An interface for casting between machine scalars.
pub trait AsCast: AsPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `AsPrimitive` trait.
    fn as_<T: AsPrimitive>(n: T) -> Self;
}

macro_rules! as_cast {
    ($t:ty, $meth:ident) => {
        impl AsCast for $t {
            #[inline(always)]
            fn as_<N: AsPrimitive>(n: N) -> $t {
                n.$meth()
            }
        }
    };
}

as_cast!(u8, as_u8);
as_cast!(u16, as_u16);
as_cast!(u32, as_u32);
as_cast!(u64, as_u64);
as_cast!(usize, as_usize);
as_cast!(i8, as_i8);
as_cast!(i16, as_i16);
as_cast!(i32, as_i32);
as_cast!(i64, as_i64);
as_cast!(isize, as_isize);
as_cast!(f32, as_f32);
as_cast!(f64, as_f64);

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    fn check_as<T: AsCast>(t: T) {
        let _: i8 = as_(t);
        let _: i16 = as_(t);
        let _: i32 = as_(t);
        let _: i64 = as_(t);
        let _: isize = as_(t);
        let _: u8 = as_(t);
        let _: u16 = as_(t);
        let _: u32 = as_(t);
        let _: u64 = as_(t);
        let _: usize = as_(t);
        let _: f32 = as_(t);
        let _: f64 = as_(t);
    }

    #[test]
    fn as_test() {
        check_as(1u8);
        check_as(1u16);
        check_as(1u32);
        check_as(1u64);
        check_as(1usize);
        check_as(1i8);
        check_as(1i16);
        check_as(1i32);
        check_as(1i64);
        check_as(1isize);
        check_as(1f32);
        check_as(1f64);
    }
}
