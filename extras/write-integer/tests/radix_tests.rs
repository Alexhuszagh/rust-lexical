#![cfg(not(feature = "compact"))]
#![cfg(feature = "power-of-two")]

mod util;

use core::num::ParseIntError;
use core::str::from_utf8_unchecked;

use lexical_util::{constants::BUFFER_SIZE, num::UnsignedInteger};
use lexical_write_integer::write::WriteInteger;
use proptest::prelude::*;

use crate::util::{default_proptest_config, from_radix};

pub trait FromRadix: Sized {
    fn from_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
}

macro_rules! impl_from_radix {
    ($($t:ty)*) => ($(impl FromRadix for $t {
        #[inline]
        fn from_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
            <$t>::from_str_radix(src, radix)
        }
    })*)
}

impl_from_radix! { u32 u64 u128 }

// We need to trick the algorithm into thinking we're using a const.
// NOTE: This needs to be broken down into 4 functions since otherwise
// we can overflow the stack, so we just never inline for each test.
#[inline(never)]
fn to_radix_2_9<T: UnsignedInteger + WriteInteger>(x: T, radix: u32, buffer: &mut [u8]) -> usize {
    match radix {
        2 => x.write_mantissa::<{ from_radix(2) }>(buffer),
        3 => x.write_mantissa::<{ from_radix(3) }>(buffer),
        4 => x.write_mantissa::<{ from_radix(4) }>(buffer),
        5 => x.write_mantissa::<{ from_radix(5) }>(buffer),
        6 => x.write_mantissa::<{ from_radix(6) }>(buffer),
        7 => x.write_mantissa::<{ from_radix(7) }>(buffer),
        8 => x.write_mantissa::<{ from_radix(8) }>(buffer),
        9 => x.write_mantissa::<{ from_radix(9) }>(buffer),
        _ => unimplemented!(),
    }
}

#[inline(never)]
fn to_radix_10_18<T: UnsignedInteger + WriteInteger>(x: T, radix: u32, buffer: &mut [u8]) -> usize {
    match radix {
        10 => x.write_mantissa::<{ from_radix(10) }>(buffer),
        11 => x.write_mantissa::<{ from_radix(11) }>(buffer),
        12 => x.write_mantissa::<{ from_radix(12) }>(buffer),
        13 => x.write_mantissa::<{ from_radix(13) }>(buffer),
        14 => x.write_mantissa::<{ from_radix(14) }>(buffer),
        15 => x.write_mantissa::<{ from_radix(15) }>(buffer),
        16 => x.write_mantissa::<{ from_radix(16) }>(buffer),
        17 => x.write_mantissa::<{ from_radix(17) }>(buffer),
        18 => x.write_mantissa::<{ from_radix(18) }>(buffer),
        _ => unimplemented!(),
    }
}

#[inline(never)]
fn to_radix_19_27<T: UnsignedInteger + WriteInteger>(x: T, radix: u32, buffer: &mut [u8]) -> usize {
    match radix {
        19 => x.write_mantissa::<{ from_radix(19) }>(buffer),
        20 => x.write_mantissa::<{ from_radix(20) }>(buffer),
        21 => x.write_mantissa::<{ from_radix(21) }>(buffer),
        22 => x.write_mantissa::<{ from_radix(22) }>(buffer),
        23 => x.write_mantissa::<{ from_radix(23) }>(buffer),
        24 => x.write_mantissa::<{ from_radix(24) }>(buffer),
        25 => x.write_mantissa::<{ from_radix(25) }>(buffer),
        26 => x.write_mantissa::<{ from_radix(26) }>(buffer),
        27 => x.write_mantissa::<{ from_radix(27) }>(buffer),
        _ => unimplemented!(),
    }
}

#[inline(never)]
fn to_radix_28_36<T: UnsignedInteger + WriteInteger>(x: T, radix: u32, buffer: &mut [u8]) -> usize {
    match radix {
        28 => x.write_mantissa::<{ from_radix(28) }>(buffer),
        29 => x.write_mantissa::<{ from_radix(29) }>(buffer),
        30 => x.write_mantissa::<{ from_radix(30) }>(buffer),
        31 => x.write_mantissa::<{ from_radix(31) }>(buffer),
        32 => x.write_mantissa::<{ from_radix(32) }>(buffer),
        33 => x.write_mantissa::<{ from_radix(33) }>(buffer),
        34 => x.write_mantissa::<{ from_radix(34) }>(buffer),
        35 => x.write_mantissa::<{ from_radix(35) }>(buffer),
        36 => x.write_mantissa::<{ from_radix(36) }>(buffer),
        _ => unimplemented!(),
    }
}

#[inline(never)]
fn mockup<T: UnsignedInteger + WriteInteger + FromRadix>(
    x: T,
    radix: u32,
) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let count = match radix {
        2..=9 => to_radix_2_9(x, radix, &mut buffer),
        10..=18 => to_radix_10_18(x, radix, &mut buffer),
        19..=27 => to_radix_19_27(x, radix, &mut buffer),
        28..=36 => to_radix_28_36(x, radix, &mut buffer),
        _ => unimplemented!(),
    };
    let y = unsafe { T::from_radix(from_utf8_unchecked(&buffer[..count]), radix) };
    prop_assert_eq!(y, Ok(x));

    Ok(())
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "radix")]
    fn u32toa_proptest(x: u32, radix in 2u32..=36) {
        mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(not(feature = "radix"))]
    fn u32toa_proptest(x: u32, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "radix")]
    fn u64toa_proptest(x: u64, radix in 2u32..=36) {
        mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(not(feature = "radix"))]
    fn u64toa_proptest(x: u64, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "radix")]
    fn u128toa_proptest(x: u128, radix in 2u32..=36) {
        mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(not(feature = "radix"))]
    fn u128toa_proptest(x: u128, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        mockup(x, radix)?;
    }
}
