//! Utilties for defining custom numeric types.
//!
//! This defines primarily macros intended to be used when
//! defining your own types.

#![cfg(feature = "f16")]
#![doc(hidden)]

/// Implement the reference and op assign variants of a trait.
macro_rules! op_impl {
    ($t:ty, $trait:ident, $assign:ident, $op:ident, $op_assign:ident) => {
        impl $trait<&$t> for $t {
            type Output = <Self as $trait>::Output;

            #[inline(always)]
            fn $op(self, rhs: &Self) -> Self::Output {
                self.$op(*rhs)
            }
        }

        impl $assign for $t {
            #[inline(always)]
            fn $op_assign(&mut self, other: Self) {
                *self = self.$op(other);
            }
        }

        impl $assign<&$t> for $t {
            #[inline(always)]
            fn $op_assign(&mut self, other: &Self) {
                *self = self.$op(other);
            }
        }
    };
}

pub(crate) use op_impl;

macro_rules! ref_impl {
    ($t:ty, $trait:ident, $op:ident) => {
        impl $trait for &$t {
            type Output = <$t as $trait>::Output;

            #[inline(always)]
            fn $op(self) -> Self::Output {
                $trait::$op(*self)
            }
        }
    };
}

pub(crate) use ref_impl;

macro_rules! from_impl {
    ($to:ty, $from:ty, $op:ident) => {
        impl From<$from> for $to {
            #[inline(always)]
            fn from(value: $from) -> Self {
                Self::$op(value)
            }
        }
    };
}

pub(crate) use from_impl;
