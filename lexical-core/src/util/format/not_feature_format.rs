#![cfg(not(feature = "format"))]

use bitflags::bitflags;

bitflags! {
    /// Dummy bitflags for the float format.
    #[doc(hidden)]
    #[derive(Default)]
    pub struct NumberFormat: u64 {
        const __NONEXHAUSTIVE = 0;
    }
}


impl NumberFormat {
    #[inline]
    pub const fn standard() -> Option<NumberFormat> {
        Some(NumberFormat::__NONEXHAUSTIVE)
    }

    #[inline]
    pub const fn digit_separator(&self) -> u8 {
        0
    }
}
