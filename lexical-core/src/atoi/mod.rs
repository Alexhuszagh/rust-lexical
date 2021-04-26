//! Fast lexical string-to-integer conversion routines.

// Hide implementation details.
#[macro_use]
#[cfg(any(feature = "atof", feature = "atoi"))]
mod shared;

cfg_if! {
if #[cfg(feature = "atof")] {
    mod exponent;
    mod mantissa;

    // Re-exports
    pub(crate) use self::exponent::*;
    pub(crate) use self::mantissa::*;
}}

cfg_if! {
if #[cfg(feature = "atoi")] {
    mod api;
    mod generic;
}}
