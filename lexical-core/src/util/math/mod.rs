//! Numerical operations and bit manipulations.
//!
//! This includes operations for scalar, small, and arbitrary-precision
//! arithmetic.

mod div128;
mod log2;
mod mask;

pub(crate) use self::div128::*;
pub(crate) use self::log2::*;
pub(crate) use self::mask::*;

cfg_if! {
if #[cfg(feature = "parse_floats")] {
    mod cast;
    mod from_uint;
    mod hi;
    mod large;
    mod large_ops;
    mod power;
    mod scalar;
    mod shared_ops;
    mod small;
    mod small_ops;

    pub(crate) use self::cast::*;
    pub(crate) use self::from_uint::*;
    pub(crate) use self::hi::*;
    pub(crate) use self::large_ops::*;
    pub(crate) use self::shared_ops::*;
    pub(crate) use self::small_ops::*;
}} // cfg_if
