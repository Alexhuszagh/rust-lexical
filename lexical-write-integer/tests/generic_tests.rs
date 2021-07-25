#![cfg(feature = "power-of-two")]

use std::str::from_utf8_unchecked;

use lexical_util::constants::BUFFER_SIZE;
use lexical_write_integer::generic::Generic;
use proptest::prelude::*;

#[test]
fn u128toa_test() {
    let mut buffer = [b'\x00'; 128];
    unsafe {
        const RADIX: u32 = 12;
        let value = 136551478823710021067381144334863695872u128;
        let count = value.generic::<RADIX>(&mut buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[..count]), RADIX);
        assert_eq!(y, Ok(value));
    }
}

#[cfg(feature = "power-of-two")]
fn itoa<T: Generic, const RADIX: u32>(x: T, actual: &[u8]) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    unsafe {
        let count = x.generic::<RADIX>(&mut buffer);
        assert_eq!(actual.len(), count);
        assert_eq!(actual, &buffer[..count])
    }
}

#[test]
#[cfg(feature = "power-of-two")]
fn binary_test() {
    // Binary
    itoa::<_, 2>(0u128, b"0");
    itoa::<_, 2>(1u128, b"1");
    itoa::<_, 2>(5u128, b"101");
    itoa::<_, 2>(170141183460469231731687303715884105727u128, b"1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");

    // Hexadecimal
    itoa::<_, 16>(170141183460469231731687303715884105727u128, b"7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
}

#[test]
#[cfg(feature = "radix")]
fn radix_test() {
    itoa::<_, 2>(37u32, b"100101");
    itoa::<_, 3>(37u32, b"1101");
    itoa::<_, 4>(37u32, b"211");
    itoa::<_, 5>(37u32, b"122");
    itoa::<_, 6>(37u32, b"101");
    itoa::<_, 7>(37u32, b"52");
    itoa::<_, 8>(37u32, b"45");
    itoa::<_, 9>(37u32, b"41");
    itoa::<_, 10>(37u32, b"37");
    itoa::<_, 11>(37u32, b"34");
    itoa::<_, 12>(37u32, b"31");
    itoa::<_, 13>(37u32, b"2B");
    itoa::<_, 14>(37u32, b"29");
    itoa::<_, 15>(37u32, b"27");
    itoa::<_, 16>(37u32, b"25");
    itoa::<_, 17>(37u32, b"23");
    itoa::<_, 18>(37u32, b"21");
    itoa::<_, 19>(37u32, b"1I");
    itoa::<_, 20>(37u32, b"1H");
    itoa::<_, 21>(37u32, b"1G");
    itoa::<_, 22>(37u32, b"1F");
    itoa::<_, 23>(37u32, b"1E");
    itoa::<_, 24>(37u32, b"1D");
    itoa::<_, 25>(37u32, b"1C");
    itoa::<_, 26>(37u32, b"1B");
    itoa::<_, 27>(37u32, b"1A");
    itoa::<_, 28>(37u32, b"19");
    itoa::<_, 29>(37u32, b"18");
    itoa::<_, 30>(37u32, b"17");
    itoa::<_, 31>(37u32, b"16");
    itoa::<_, 32>(37u32, b"15");
    itoa::<_, 33>(37u32, b"14");
    itoa::<_, 34>(37u32, b"13");
    itoa::<_, 35>(37u32, b"12");
    itoa::<_, 36>(37u32, b"11");
}

// We need to trick the algorithm into thinking we're using a const.
// Useful for proptests, useless everywhere else.
macro_rules! radix_to_generic {
    ($x:ident, $radix:ident, $buffer:ident) => {
        match $radix {
            2 => $x.generic::<2>(&mut $buffer),
            3 => $x.generic::<3>(&mut $buffer),
            4 => $x.generic::<4>(&mut $buffer),
            5 => $x.generic::<5>(&mut $buffer),
            6 => $x.generic::<6>(&mut $buffer),
            7 => $x.generic::<7>(&mut $buffer),
            8 => $x.generic::<8>(&mut $buffer),
            9 => $x.generic::<9>(&mut $buffer),
            10 => $x.generic::<10>(&mut $buffer),
            11 => $x.generic::<11>(&mut $buffer),
            12 => $x.generic::<12>(&mut $buffer),
            13 => $x.generic::<13>(&mut $buffer),
            14 => $x.generic::<14>(&mut $buffer),
            15 => $x.generic::<15>(&mut $buffer),
            16 => $x.generic::<16>(&mut $buffer),
            17 => $x.generic::<17>(&mut $buffer),
            18 => $x.generic::<18>(&mut $buffer),
            19 => $x.generic::<19>(&mut $buffer),
            20 => $x.generic::<20>(&mut $buffer),
            21 => $x.generic::<21>(&mut $buffer),
            22 => $x.generic::<22>(&mut $buffer),
            23 => $x.generic::<23>(&mut $buffer),
            24 => $x.generic::<24>(&mut $buffer),
            25 => $x.generic::<25>(&mut $buffer),
            26 => $x.generic::<26>(&mut $buffer),
            27 => $x.generic::<27>(&mut $buffer),
            28 => $x.generic::<28>(&mut $buffer),
            29 => $x.generic::<29>(&mut $buffer),
            30 => $x.generic::<30>(&mut $buffer),
            31 => $x.generic::<31>(&mut $buffer),
            32 => $x.generic::<32>(&mut $buffer),
            33 => $x.generic::<33>(&mut $buffer),
            34 => $x.generic::<34>(&mut $buffer),
            35 => $x.generic::<35>(&mut $buffer),
            36 => $x.generic::<36>(&mut $buffer),
            _ => unimplemented!(),
        }
    };
}

fn u32toa_mockup(x: u32, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 32];
    unsafe {
        let count = radix_to_generic!(x, radix, buffer);
        let y = u32::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u64toa_mockup(x: u64, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 64];
    unsafe {
        let count = radix_to_generic!(x, radix, buffer);
        let y = u64::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u128toa_mockup(x: u128, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 128];
    unsafe {
        let count = radix_to_generic!(x, radix, buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

proptest! {
    #[test]
    #[cfg(feature = "radix")]
    fn u32toa_proptest(x: u32, radix in 2u32..=36) {
        u32toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg(not(feature = "radix"))]
    fn u32toa_proptest(x: u32, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        u32toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u64toa_proptest(x: u64, radix in 2u32..=36) {
        u64toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg(not(feature = "radix"))]
    fn u64toa_proptest(x: u64, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        u64toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u128toa_proptest(x: u128, radix in 2u32..=36) {
        u128toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg(not(feature = "radix"))]
    fn u128toa_proptest(x: u128, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        u128toa_mockup(x, radix)?;
    }
}
