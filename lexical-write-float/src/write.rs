//! Shared trait and methods for writing floats.

#![doc(hidden)]

#[cfg(not(feature = "compact"))]
use crate::algorithm::write_float as write_float_decimal;
#[cfg(feature = "power-of-two")]
use crate::binary;
/// Select the back-end.
#[cfg(feature = "compact")]
use crate::compact::write_float as write_float_decimal;
#[cfg(feature = "power-of-two")]
use crate::hex;
#[cfg(feature = "radix")]
use crate::radix;

use crate::float::RawFloat;
use crate::options::Options;
#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
use lexical_util::constants::FormattedSize;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_util::format::NumberFormat;
use lexical_write_integer::write::WriteInteger;

/// Write float trait.
pub trait WriteFloat: RawFloat {
    /// Forward write integer parameters to an unoptimized backend.
    ///
    /// # Safety
    ///
    /// Safe as long as the buffer can hold [`FORMATTED_SIZE`] elements
    /// (or [`FORMATTED_SIZE_DECIMAL`] for decimal). If using custom digit
    /// precision control (such as specifying a minimum number of significant
    /// digits), or disabling scientific notation, then more digits may be
    /// required (up to `1075` for the leading or trailing zeros, `1` for
    /// the sign and `1` for the decimal point). So,
    /// `1077 + min_significant_digits.max(52)`, so ~1200 for a reasonable
    /// threshold.
    ///
    /// # Panics
    ///
    /// Panics if the number format is invalid, or if scientific notation
    /// is used and the exponent base does not equal the mantissa radix
    /// and the format is not a hexadecimal float. It also panics
    /// if `options.nan_string` or `options.inf_string` is None and asked
    /// to serialize a NaN or Inf value.
    ///
    /// [`FORMATTED_SIZE`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE
    /// [`FORMATTED_SIZE_DECIMAL`]: lexical_util::constants::FormattedSize::FORMATTED_SIZE_DECIMAL
    #[inline]
    unsafe fn write_float<const FORMAT: u128>(self, bytes: &mut [u8], options: &Options) -> usize
    where
        Self::Unsigned: FormattedSize + WriteInteger,
    {
        // Validate our format options.
        let format = NumberFormat::<FORMAT> {};
        assert!(format.is_valid());
        // Avoid any false assumptions for 128-bit floats.
        assert!(Self::BITS <= 64);

        #[cfg(feature = "power-of-two")]
        {
            if format.radix() != format.exponent_base() {
                assert!(matches!(
                    (format.radix(), format.exponent_base()),
                    (4, 2) | (8, 2) | (16, 2) | (32, 2) | (16, 4)
                ));
            }
        }

        let (float, count, bytes) = if self < Self::ZERO {
            // SAFETY: safe if `bytes.len() > 1`.
            unsafe { index_unchecked_mut!(bytes[0]) = b'-' };
            (-self, 1, unsafe { &mut index_unchecked_mut!(bytes[1..]) })
        } else if cfg!(feature = "format") && format.required_mantissa_sign() {
            // SAFETY: safe if `bytes.len() > 1`.
            unsafe { index_unchecked_mut!(bytes[0]) = b'+' };
            (self, 1, unsafe { &mut index_unchecked_mut!(bytes[1..]) })
        } else {
            (self, 0, bytes)
        };

        // Handle special values.
        if !self.is_special() {
            #[cfg(all(feature = "power-of-two", not(feature = "radix")))]
            {
                // SAFETY: safe if the buffer can hold the significant digits
                let radix = format.radix();
                let exponent_base = format.exponent_base();
                count
                    + if radix == 10 {
                        unsafe { write_float_decimal::<_, FORMAT>(float, bytes, options) }
                    } else if radix != exponent_base {
                        unsafe { hex::write_float::<_, FORMAT>(float, bytes, options) }
                    } else {
                        unsafe { binary::write_float::<_, FORMAT>(float, bytes, options) }
                    }
            }

            #[cfg(feature = "radix")]
            {
                // SAFETY: safe if the buffer can hold the significant digits
                let radix = format.radix();
                let exponent_base = format.exponent_base();
                count
                    + if radix == 10 {
                        unsafe { write_float_decimal::<_, FORMAT>(float, bytes, options) }
                    } else if radix != exponent_base {
                        unsafe { hex::write_float::<_, FORMAT>(float, bytes, options) }
                    } else if matches!(radix, 2 | 4 | 8 | 16 | 32) {
                        unsafe { binary::write_float::<_, FORMAT>(float, bytes, options) }
                    } else {
                        unsafe { radix::write_float::<_, FORMAT>(float, bytes, options) }
                    }
            }

            #[cfg(not(feature = "power-of-two"))]
            {
                // SAFETY: safe if the buffer can hold the significant digits
                count + unsafe { write_float_decimal::<_, FORMAT>(float, bytes, options) }
            }
        } else if self.is_nan() {
            // SAFETY: safe if the buffer is longer than the NaN string.
            // The NaN string must be <= 50 characters, so safe as long as
            // the options were build using safe methods.
            if let Some(nan_string) = options.nan_string() {
                let length = nan_string.len();
                unsafe {
                    let src = nan_string.as_ptr();
                    let dst = &mut index_unchecked_mut!(bytes[..length]);
                    copy_nonoverlapping_unchecked!(dst, src, length);
                }
                count + length
            } else {
                // PANIC: cannot serialize NaN.
                panic!("NaN explicitly disabled but asked to write NaN as string.");
            }
        } else {
            // is_inf
            // SAFETY: safe if the buffer is longer than the Inf string.
            // The Inf string must be <= 50 characters, so safe as long as
            // the options were build using safe methods.
            if let Some(inf_string) = options.inf_string() {
                let length = inf_string.len();
                unsafe {
                    let src = inf_string.as_ptr();
                    let dst = &mut index_unchecked_mut!(bytes[..length]);
                    copy_nonoverlapping_unchecked!(dst, src, length);
                }
                count + length
            } else {
                // PANIC: cannot serialize inf.
                panic!("Inf explicitly disabled but asked to write Inf as string.");
            }
        }
    }
}

macro_rules! write_float_impl {
    ($($t:ty)*) => ($(
        impl WriteFloat for $t {}
    )*)
}

write_float_impl! { f32 f64 }

#[cfg(feature = "f16")]
macro_rules! write_float_as_f32 {
    ($($t:ty)*) => ($(
        impl WriteFloat for $t {
            #[inline]
            unsafe fn write_float<const FORMAT: u128>(self, bytes: &mut [u8], options: &Options) -> usize
            {
                // SAFETY: safe if `bytes` is large enough to hold the written bytes.
                unsafe { self.as_f32().write_float::<FORMAT>(bytes, options) }
            }
        }
    )*)
}

#[cfg(feature = "f16")]
write_float_as_f32! { bf16 f16 }
