//! Fast lexical float-to-string conversion routines.

// Hide implementation details.
mod api;
#[cfg(feature = "power_of_two")]
mod binary;
#[cfg(feature = "radix")]
mod radix;

cfg_if! {
if #[cfg(feature = "grisu3")] {
    mod grisu3;
    mod replace;
} else if #[cfg(feature = "ryu")] {
    mod ryu;
    mod replace;
} else {
    mod grisu2;
}} // cfg_if
