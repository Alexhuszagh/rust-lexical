//! Shared trait and methods for writing integers.

#![doc(hidden)]

use lexical_util::format;

/// Select the back-end.
#[cfg(feature = "compact")]
use crate::compact::Compact;
#[cfg(not(feature = "compact"))]
use crate::decimal::Decimal;
#[cfg(all(not(feature = "compact"), feature = "power-of-two"))]
use crate::radix::Radix;

/// Define the implementation to write significant digits.
macro_rules! write_mantissa {
    ($($t:tt)+) => (
        /// Internal implementation to write significant digits for float writers.
        #[doc(hidden)]
        #[inline(always)]
        fn write_mantissa<U, const FORMAT: u128>(self, buffer: &mut [u8]) -> usize
        where
            U: $($t)+,
        {
            self.write_integer::<U, FORMAT, { format::RADIX }, { format::RADIX_SHIFT }>(buffer)
        }
    )
}

/// Define the implementation to write exponent digits.
macro_rules! write_exponent {
    ($($t:tt)+) => (
        /// Internal implementation to write exponent digits for float writers.
        #[doc(hidden)]
        #[inline(always)]
        fn write_exponent<U, const FORMAT: u128>(self, buffer: &mut [u8]) -> usize
        where
            U: $($t)+,
        {
            self.write_integer::<U, FORMAT, { format::EXPONENT_RADIX }, { format::EXPONENT_RADIX_SHIFT }>(buffer)
        }
    )
}

/// Write integer trait, implemented in terms of the compact back-end.
#[cfg(feature = "compact")]
pub trait WriteInteger: Compact {
    /// Forward write integer parameters to an unoptimized backend.
    ///
    /// # Preconditions
    ///
    /// `self` must be non-negative and unsigned.
    ///
    /// [`FORMATTED_SIZE`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE
    /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
    fn write_integer<U, const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize
    where
        U: Compact,
    {
        let value = U::as_cast(self);
        let radix = format::radix_from_flags(FORMAT, MASK, SHIFT);
        value.compact(radix, buffer)
    }

    write_mantissa!(Compact);
    write_exponent!(Compact);
}

/// Write integer trait, implemented in terms of the optimized, decimal
/// back-end.
#[cfg(all(not(feature = "compact"), not(feature = "power-of-two")))]
pub trait WriteInteger: Decimal {
    /// Forward write integer parameters to an optimized backend.
    ///
    /// # Preconditions
    ///
    /// `self` must be non-negative and unsigned.
    ///
    /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
    #[inline(always)]
    fn write_integer<U, const __: u128, const ___: u128, const ____: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize
    where
        U: Decimal,
    {
        let value = U::as_cast(self);
        value.decimal(buffer)
    }

    write_mantissa!(Decimal);
    write_exponent!(Decimal);
}

/// Write integer trait, implemented in terms of the optimized, decimal or radix
/// back-end.
#[cfg(all(not(feature = "compact"), feature = "power-of-two"))]
pub trait WriteInteger: Decimal + Radix {
    /// Forward write integer parameters to an optimized backend.
    ///
    /// # Preconditions
    ///
    /// `self` must be non-negative and unsigned.
    ///
    /// [`FORMATTED_SIZE`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE
    /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
    #[inline(always)]
    fn write_integer<U, const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize
    where
        U: Decimal + Radix,
    {
        let value = U::as_cast(self);
        if format::radix_from_flags(FORMAT, MASK, SHIFT) == 10 {
            value.decimal(buffer)
        } else {
            value.radix::<FORMAT, MASK, SHIFT>(buffer)
        }
    }

    write_mantissa!(Decimal + Radix);
    write_exponent!(Decimal + Radix);
}

macro_rules! write_integer_impl {
    ($($t:ty)*) => ($(
        impl WriteInteger for $t {}
    )*)
}

write_integer_impl! { u8 u16 u32 u64 u128 usize }
