//! Pre-computed tables and functions for conversions to and from digits.

// TODO(ahuszagh) Going to have to refactor a bit.
mod decimal;
mod funcs;
pub(crate) use self::decimal::*;
pub(crate) use self::funcs::*;

cfg_if! {
if #[cfg(feature = "power_of_two")] {
    mod binary;
    pub(crate) use self::binary::*;
}} // cfg_if

cfg_if! {
if #[cfg(feature = "radix")] {
    mod radix;
    pub(crate) use self::radix::*;
}} // cfg_if
