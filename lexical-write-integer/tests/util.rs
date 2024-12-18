#![allow(dead_code, unused_imports)]

#[cfg(feature = "power-of-two")]
use lexical_util::format::NumberFormatBuilder;

#[cfg(feature = "power-of-two")]
pub const fn from_radix(radix: u8) -> u128 {
    NumberFormatBuilder::from_radix(radix)
}
