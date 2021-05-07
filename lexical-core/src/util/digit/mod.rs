//! Pre-computed tables and functions for conversions to and from digits.

mod decimal;
mod funcs;

pub(crate) use self::decimal::*;
pub(crate) use self::funcs::*;

cfg_if! {
if #[cfg(all(write, feature = "power_of_two"))] {
    mod binary;
    pub(crate) use self::binary::*;
}} // cfg_if

cfg_if! {
if #[cfg(all(write, feature = "radix"))] {
    mod radix;
    pub(crate) use self::radix::*;
}} // cfg_if
