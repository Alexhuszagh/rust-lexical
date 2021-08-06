#![cfg(not(feature = "compact"))]
#![cfg(feature = "power-of-two")]

mod util;

use core::str::from_utf8_unchecked;
use lexical_util::constants::BUFFER_SIZE;
use lexical_write_integer::write::WriteInteger;
use proptest::prelude::*;
use util::from_radix;

#[test]
#[cfg(feature = "radix")]
fn u128toa_test() {
    const FORMAT: u128 = from_radix(12);
    let mut buffer = [b'\x00'; 128];
    unsafe {
        let value = 136551478823710021067381144334863695872u128;
        let count = value.write_mantissa::<u128, FORMAT>(&mut buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[..count]), 12);
        assert_eq!(y, Ok(value));
    }
}

#[cfg(feature = "power-of-two")]
fn write_integer<T: WriteInteger, const FORMAT: u128>(x: T, actual: &[u8]) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    unsafe {
        let count = x.write_mantissa::<T, FORMAT>(&mut buffer);
        assert_eq!(actual.len(), count);
        assert_eq!(actual, &buffer[..count])
    }
}

#[test]
#[cfg(feature = "power-of-two")]
fn binary_test() {
    // Binary
    const BINARY: u128 = from_radix(2);
    write_integer::<_, BINARY>(0u128, b"0");
    write_integer::<_, BINARY>(1u128, b"1");
    write_integer::<_, BINARY>(5u128, b"101");
    write_integer::<_, BINARY>(170141183460469231731687303715884105727u128, b"1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");

    // Hexadecimal
    const HEX: u128 = from_radix(16);
    write_integer::<_, HEX>(
        170141183460469231731687303715884105727u128,
        b"7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
    );
}

#[test]
#[cfg(feature = "radix")]
fn radix_test() {
    write_integer::<u32, { from_radix(2) }>(37u32, b"100101");
    write_integer::<u32, { from_radix(3) }>(37u32, b"1101");
    write_integer::<u32, { from_radix(4) }>(37u32, b"211");
    write_integer::<u32, { from_radix(5) }>(37u32, b"122");
    write_integer::<u32, { from_radix(6) }>(37u32, b"101");
    write_integer::<u32, { from_radix(7) }>(37u32, b"52");
    write_integer::<u32, { from_radix(8) }>(37u32, b"45");
    write_integer::<u32, { from_radix(9) }>(37u32, b"41");
    write_integer::<u32, { from_radix(10) }>(37u32, b"37");
    write_integer::<u32, { from_radix(11) }>(37u32, b"34");
    write_integer::<u32, { from_radix(12) }>(37u32, b"31");
    write_integer::<u32, { from_radix(13) }>(37u32, b"2B");
    write_integer::<u32, { from_radix(14) }>(37u32, b"29");
    write_integer::<u32, { from_radix(15) }>(37u32, b"27");
    write_integer::<u32, { from_radix(16) }>(37u32, b"25");
    write_integer::<u32, { from_radix(17) }>(37u32, b"23");
    write_integer::<u32, { from_radix(18) }>(37u32, b"21");
    write_integer::<u32, { from_radix(19) }>(37u32, b"1I");
    write_integer::<u32, { from_radix(20) }>(37u32, b"1H");
    write_integer::<u32, { from_radix(21) }>(37u32, b"1G");
    write_integer::<u32, { from_radix(22) }>(37u32, b"1F");
    write_integer::<u32, { from_radix(23) }>(37u32, b"1E");
    write_integer::<u32, { from_radix(24) }>(37u32, b"1D");
    write_integer::<u32, { from_radix(25) }>(37u32, b"1C");
    write_integer::<u32, { from_radix(26) }>(37u32, b"1B");
    write_integer::<u32, { from_radix(27) }>(37u32, b"1A");
    write_integer::<u32, { from_radix(28) }>(37u32, b"19");
    write_integer::<u32, { from_radix(29) }>(37u32, b"18");
    write_integer::<u32, { from_radix(30) }>(37u32, b"17");
    write_integer::<u32, { from_radix(31) }>(37u32, b"16");
    write_integer::<u32, { from_radix(32) }>(37u32, b"15");
    write_integer::<u32, { from_radix(33) }>(37u32, b"14");
    write_integer::<u32, { from_radix(34) }>(37u32, b"13");
    write_integer::<u32, { from_radix(35) }>(37u32, b"12");
    write_integer::<u32, { from_radix(36) }>(37u32, b"11");
}

// We need to trick the algorithm into thinking we're using a const.
// Useful for proptests, useless everywhere else.
macro_rules! to_radix {
    ($t:ident, $x:ident, $radix:ident, $buffer:ident) => {
        match $radix {
            2 => $x.write_mantissa::<$t, { from_radix(2) }>(&mut $buffer),
            3 => $x.write_mantissa::<$t, { from_radix(3) }>(&mut $buffer),
            4 => $x.write_mantissa::<$t, { from_radix(4) }>(&mut $buffer),
            5 => $x.write_mantissa::<$t, { from_radix(5) }>(&mut $buffer),
            6 => $x.write_mantissa::<$t, { from_radix(6) }>(&mut $buffer),
            7 => $x.write_mantissa::<$t, { from_radix(7) }>(&mut $buffer),
            8 => $x.write_mantissa::<$t, { from_radix(8) }>(&mut $buffer),
            9 => $x.write_mantissa::<$t, { from_radix(9) }>(&mut $buffer),
            10 => $x.write_mantissa::<$t, { from_radix(10) }>(&mut $buffer),
            11 => $x.write_mantissa::<$t, { from_radix(11) }>(&mut $buffer),
            12 => $x.write_mantissa::<$t, { from_radix(12) }>(&mut $buffer),
            13 => $x.write_mantissa::<$t, { from_radix(13) }>(&mut $buffer),
            14 => $x.write_mantissa::<$t, { from_radix(14) }>(&mut $buffer),
            15 => $x.write_mantissa::<$t, { from_radix(15) }>(&mut $buffer),
            16 => $x.write_mantissa::<$t, { from_radix(16) }>(&mut $buffer),
            17 => $x.write_mantissa::<$t, { from_radix(17) }>(&mut $buffer),
            18 => $x.write_mantissa::<$t, { from_radix(18) }>(&mut $buffer),
            19 => $x.write_mantissa::<$t, { from_radix(19) }>(&mut $buffer),
            20 => $x.write_mantissa::<$t, { from_radix(20) }>(&mut $buffer),
            21 => $x.write_mantissa::<$t, { from_radix(21) }>(&mut $buffer),
            22 => $x.write_mantissa::<$t, { from_radix(22) }>(&mut $buffer),
            23 => $x.write_mantissa::<$t, { from_radix(23) }>(&mut $buffer),
            24 => $x.write_mantissa::<$t, { from_radix(24) }>(&mut $buffer),
            25 => $x.write_mantissa::<$t, { from_radix(25) }>(&mut $buffer),
            26 => $x.write_mantissa::<$t, { from_radix(26) }>(&mut $buffer),
            27 => $x.write_mantissa::<$t, { from_radix(27) }>(&mut $buffer),
            28 => $x.write_mantissa::<$t, { from_radix(28) }>(&mut $buffer),
            29 => $x.write_mantissa::<$t, { from_radix(29) }>(&mut $buffer),
            30 => $x.write_mantissa::<$t, { from_radix(30) }>(&mut $buffer),
            31 => $x.write_mantissa::<$t, { from_radix(31) }>(&mut $buffer),
            32 => $x.write_mantissa::<$t, { from_radix(32) }>(&mut $buffer),
            33 => $x.write_mantissa::<$t, { from_radix(33) }>(&mut $buffer),
            34 => $x.write_mantissa::<$t, { from_radix(34) }>(&mut $buffer),
            35 => $x.write_mantissa::<$t, { from_radix(35) }>(&mut $buffer),
            36 => $x.write_mantissa::<$t, { from_radix(36) }>(&mut $buffer),
            _ => unimplemented!(),
        }
    };
}

fn u32toa_mockup(x: u32, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 32];
    unsafe {
        let count = to_radix!(u32, x, radix, buffer);
        let y = u32::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u64toa_mockup(x: u64, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 64];
    unsafe {
        let count = to_radix!(u64, x, radix, buffer);
        let y = u64::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u128toa_mockup(x: u128, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 128];
    unsafe {
        let count = to_radix!(u128, x, radix, buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

proptest! {
    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "radix")]
    fn u32toa_proptest(x: u32, radix in 2u32..=36) {
        u32toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(not(feature = "radix"))]
    fn u32toa_proptest(x: u32, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        u32toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "radix")]
    fn u64toa_proptest(x: u64, radix in 2u32..=36) {
        u64toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(not(feature = "radix"))]
    fn u64toa_proptest(x: u64, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        u64toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "radix")]
    fn u128toa_proptest(x: u128, radix in 2u32..=36) {
        u128toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(not(feature = "radix"))]
    fn u128toa_proptest(x: u128, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        u128toa_mockup(x, radix)?;
    }
}
