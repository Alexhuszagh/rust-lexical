//! Fast lexical string-to-integer conversion routines.

// Hide implementation details.
#[macro_use]
mod shared;

cfg_if! {
if #[cfg(feature = "parse_integers")] {
    mod api;
    mod generic;
}} // cfg_if

cfg_if! {
if #[cfg(feature = "parse_floats")] {
    mod exponent;
    mod mantissa;

    pub(crate) use self::exponent::*;
    pub(crate) use self::mantissa::*;
}} // cfg_if
