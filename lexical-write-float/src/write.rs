//! Shared trait and methods for writing floats.

#![doc(hidden)]

#[cfg(feature = "f16")]
use lexical_util::bf16::bf16;
#[cfg(feature = "f16")]
use lexical_util::f16::f16;
use lexical_util::format::NumberFormat;
use lexical_util::options::WriteOptions;
use lexical_util::{algorithm::copy_to_dst, constants::FormattedSize};
use lexical_write_integer::write::WriteInteger;

/// Select the back-end.
#[cfg(not(feature = "compact"))]
use crate::algorithm::write_float as write_float_decimal;
#[cfg(feature = "power-of-two")]
use crate::binary;
#[cfg(feature = "compact")]
use crate::compact::write_float as write_float_decimal;
use crate::float::RawFloat;
#[cfg(feature = "power-of-two")]
use crate::hex;
use crate::options::Options;
#[cfg(feature = "radix")]
use crate::radix;

/// Write an special string to the buffer.
#[inline(always)]
fn write_special(bytes: &mut [u8], special: Option<&[u8]>, error: &'static str) -> usize {
    // The NaN string must be <= 50 characters, so this should never panic.
    if let Some(special_str) = special {
        debug_assert!(special_str.len() <= 50, "special_str.len() must be <= 50");
        copy_to_dst(bytes, special_str)
    } else {
        // PANIC: the format does not support serializing that special.
        panic!("{}", error);
    }
}

/// Write an NaN string to the buffer.
fn write_nan(bytes: &mut [u8], options: &Options, count: usize) -> usize {
    count
        + write_special(
            bytes,
            options.nan_string(),
            "NaN explicitly disabled but asked to write NaN as string.",
        )
}

/// Write an Inf string to the buffer.
fn write_inf(bytes: &mut [u8], options: &Options, count: usize) -> usize {
    count
        + write_special(
            bytes,
            options.inf_string(),
            "Inf explicitly disabled but asked to write Inf as string.",
        )
}

/// Check if a buffer is sufficiently large.
#[inline(always)]
fn check_buffer<T, const FORMAT: u128>(len: usize, options: &Options) -> bool
where
    T: FormattedSize,
{
    let size = Options::buffer_size::<T, FORMAT>(options);
    len >= size
}

/// Write float trait.
pub trait WriteFloat: RawFloat + FormattedSize {
    /// Forward float writing parameters and write the float.
    ///
    /// This abstracts away handling different optimizations and radices into
    /// a single API.
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
    #[cfg_attr(not(feature = "compact"), inline(always))]
    fn write_float<const FORMAT: u128>(self, bytes: &mut [u8], options: &Options) -> usize
    where
        Self::Unsigned: FormattedSize + WriteInteger,
    {
        // Validate our format options.
        assert!(check_buffer::<Self, { FORMAT }>(bytes.len(), options));
        let format = NumberFormat::<FORMAT> {};
        assert!(format.is_valid());
        // Avoid any false assumptions for 128-bit floats.
        assert!(Self::BITS <= 64);

        #[cfg(feature = "power-of-two")]
        {
            // FIXME: I believe this incorrectly handles a few cases.
            if format.radix() != format.exponent_base() {
                assert!(matches!(
                    (format.radix(), format.exponent_base()),
                    (4, 2) | (8, 2) | (16, 2) | (32, 2) | (16, 4)
                ));
            }
        }

        let (float, count, bytes) = if self.needs_negative_sign() {
            bytes[0] = b'-';
            (-self, 1, &mut bytes[1..])
        } else if cfg!(feature = "format") && format.required_mantissa_sign() {
            bytes[0] = b'+';
            (self, 1, &mut bytes[1..])
        } else {
            (self, 0, bytes)
        };

        // Handle special values.
        if !self.is_special() {
            #[cfg(all(feature = "power-of-two", not(feature = "radix")))]
            {
                let radix = format.radix();
                let exponent_base = format.exponent_base();
                count
                    + if radix == 10 {
                        write_float_decimal::<_, FORMAT>(float, bytes, options)
                    } else if radix != exponent_base {
                        hex::write_float::<_, FORMAT>(float, bytes, options)
                    } else {
                        binary::write_float::<_, FORMAT>(float, bytes, options)
                    }
            }

            #[cfg(feature = "radix")]
            {
                let radix = format.radix();
                let exponent_base = format.exponent_base();
                count
                    + if radix == 10 {
                        write_float_decimal::<_, FORMAT>(float, bytes, options)
                    } else if radix != exponent_base {
                        hex::write_float::<_, FORMAT>(float, bytes, options)
                    } else if matches!(radix, 2 | 4 | 8 | 16 | 32) {
                        binary::write_float::<_, FORMAT>(float, bytes, options)
                    } else {
                        radix::write_float::<_, FORMAT>(float, bytes, options)
                    }
            }

            #[cfg(not(feature = "power-of-two"))]
            {
                count + write_float_decimal::<_, FORMAT>(float, bytes, options)
            }
        } else if self.is_nan() {
            write_nan(bytes, options, count)
        } else {
            write_inf(bytes, options, count)
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
            #[inline(always)]
            fn write_float<const FORMAT: u128>(self, bytes: &mut [u8], options: &Options) -> usize
            {
                self.as_f32().write_float::<FORMAT>(bytes, options)
            }
        }
    )*)
}

#[cfg(feature = "f16")]
write_float_as_f32! { bf16 f16 }
