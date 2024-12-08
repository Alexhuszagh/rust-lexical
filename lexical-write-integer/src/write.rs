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
    ($($t:tt)+) => {
        /// Internal implementation to write significant digits for float writers.
        #[doc(hidden)]
        #[inline(always)]
        fn write_mantissa<const FORMAT: u128>(self, buffer: &mut [u8]) -> usize {
            self.write_integer::<FORMAT, { format::RADIX }, { format::RADIX_SHIFT }>(buffer)
        }

        /// Internal implementation to write significant digits for float writers.
        #[doc(hidden)]
        #[inline(always)]
        fn write_mantissa_signed<const FORMAT: u128>(self, buffer: &mut [u8]) -> usize {
            self.write_integer_signed::<FORMAT, { format::RADIX }, { format::RADIX_SHIFT }>(buffer)
        }
    };
}

/// Define the implementation to write exponent digits.
macro_rules! write_exponent {
    ($($t:tt)+) => (
        // NOTE: This should always be signed, but for backwards compatibility as
        // a precaution we keep the original just in case someone uses the private API.

        /// Internal implementation to write exponent digits for float writers.
        // NOTE: This is not part of the public API.
        #[doc(hidden)]
        #[inline(always)]
        #[deprecated = "use `write_exponent_signed`, since exponents are always signed."]
        fn write_exponent<const FORMAT: u128>(self, buffer: &mut [u8]) -> usize
        {
            self.write_integer::<FORMAT, { format::EXPONENT_RADIX }, { format::EXPONENT_RADIX_SHIFT }>(buffer)
        }

        /// Internal implementation to write exponent digits for float writers.
        #[doc(hidden)]
        #[inline(always)]
        fn write_exponent_signed<const FORMAT: u128>(self, buffer: &mut [u8]) -> usize
        {
            self.write_integer_signed::<FORMAT, { format::EXPONENT_RADIX }, { format::EXPONENT_RADIX_SHIFT }>(buffer)
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
    fn write_integer<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize {
        let radix = format::radix_from_flags(FORMAT, MASK, SHIFT);
        self.compact(radix, buffer)
    }

    /// Forward write integer parameters to an optimized backend.
    ///
    /// This requires a type that was previously signed.
    ///
    /// # Preconditions
    ///
    /// `self` must be non-negative but is `>= 0` and `<= Signed::MAX`.
    ///
    /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
    #[inline(always)]
    fn write_integer_signed<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize {
        self.write_integer::<FORMAT, MASK, SHIFT>(buffer)
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
    fn write_integer<const __: u128, const ___: u128, const ____: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize {
        self.decimal(buffer)
    }

    /// Forward write integer parameters to an optimized backend.
    ///
    /// This requires a type that was previously signed.
    ///
    /// # Preconditions
    ///
    /// `self` must be non-negative but is `>= 0` and `<= Signed::MAX`.
    ///
    /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
    #[inline(always)]
    fn write_integer_signed<const __: u128, const ___: u128, const ____: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize {
        self.decimal_signed(buffer)
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
    fn write_integer<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize {
        if format::radix_from_flags(FORMAT, MASK, SHIFT) == 10 {
            self.decimal(buffer)
        } else {
            self.radix::<FORMAT, MASK, SHIFT>(buffer)
        }
    }

    /// Forward write integer parameters to an optimized backend.
    ///
    /// This requires a type that was previously signed.
    ///
    /// # Preconditions
    ///
    /// `self` must be non-negative but is `>= 0` and `<= Signed::MAX`.
    ///
    /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
    #[inline(always)]
    fn write_integer_signed<const FORMAT: u128, const MASK: u128, const SHIFT: i32>(
        self,
        buffer: &mut [u8],
    ) -> usize {
        if format::radix_from_flags(FORMAT, MASK, SHIFT) == 10 {
            self.decimal_signed(buffer)
        } else {
            self.radix::<FORMAT, MASK, SHIFT>(buffer)
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
