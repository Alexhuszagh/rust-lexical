#![cfg(not(feature = "compact"))]
#![cfg(feature = "power-of-two")]

use std::str::from_utf8_unchecked;

use lexical_util::constants::BUFFER_SIZE;
use lexical_write_integer::radix::Radix;
#[cfg(not(miri))]
use proptest::prelude::*;

#[test]
fn u128toa_test() {
    let mut buffer = [b'\x00'; 128];
    unsafe {
        const RADIX: u32 = 12;
        let value = 136551478823710021067381144334863695872u128;
        let count = value.radix::<RADIX>(&mut buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[..count]), RADIX);
        assert_eq!(y, Ok(value));
    }
}

#[cfg(feature = "power-of-two")]
fn write_integer<T: Radix, const RADIX: u32>(x: T, actual: &[u8]) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    unsafe {
        let count = x.radix::<RADIX>(&mut buffer);
        assert_eq!(actual.len(), count);
        assert_eq!(actual, &buffer[..count])
    }
}

#[test]
#[cfg(feature = "power-of-two")]
fn binary_test() {
    // Binary
    write_integer::<_, 2>(0u128, b"0");
    write_integer::<_, 2>(1u128, b"1");
    write_integer::<_, 2>(5u128, b"101");
    write_integer::<_, 2>(170141183460469231731687303715884105727u128, b"1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");

    // Hexadecimal
    write_integer::<_, 16>(
        170141183460469231731687303715884105727u128,
        b"7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
    );
}

#[test]
#[cfg(feature = "radix")]
fn radix_test() {
    write_integer::<_, 2>(37u32, b"100101");
    write_integer::<_, 3>(37u32, b"1101");
    write_integer::<_, 4>(37u32, b"211");
    write_integer::<_, 5>(37u32, b"122");
    write_integer::<_, 6>(37u32, b"101");
    write_integer::<_, 7>(37u32, b"52");
    write_integer::<_, 8>(37u32, b"45");
    write_integer::<_, 9>(37u32, b"41");
    write_integer::<_, 10>(37u32, b"37");
    write_integer::<_, 11>(37u32, b"34");
    write_integer::<_, 12>(37u32, b"31");
    write_integer::<_, 13>(37u32, b"2B");
    write_integer::<_, 14>(37u32, b"29");
    write_integer::<_, 15>(37u32, b"27");
    write_integer::<_, 16>(37u32, b"25");
    write_integer::<_, 17>(37u32, b"23");
    write_integer::<_, 18>(37u32, b"21");
    write_integer::<_, 19>(37u32, b"1I");
    write_integer::<_, 20>(37u32, b"1H");
    write_integer::<_, 21>(37u32, b"1G");
    write_integer::<_, 22>(37u32, b"1F");
    write_integer::<_, 23>(37u32, b"1E");
    write_integer::<_, 24>(37u32, b"1D");
    write_integer::<_, 25>(37u32, b"1C");
    write_integer::<_, 26>(37u32, b"1B");
    write_integer::<_, 27>(37u32, b"1A");
    write_integer::<_, 28>(37u32, b"19");
    write_integer::<_, 29>(37u32, b"18");
    write_integer::<_, 30>(37u32, b"17");
    write_integer::<_, 31>(37u32, b"16");
    write_integer::<_, 32>(37u32, b"15");
    write_integer::<_, 33>(37u32, b"14");
    write_integer::<_, 34>(37u32, b"13");
    write_integer::<_, 35>(37u32, b"12");
    write_integer::<_, 36>(37u32, b"11");
}

// We need to trick the algorithm into thinking we're using a const.
// Useful for proptests, useless everywhere else.
macro_rules! to_radix {
    ($x:ident, $radix:ident, $buffer:ident) => {
        match $radix {
            2 => $x.radix::<2>(&mut $buffer),
            3 => $x.radix::<3>(&mut $buffer),
            4 => $x.radix::<4>(&mut $buffer),
            5 => $x.radix::<5>(&mut $buffer),
            6 => $x.radix::<6>(&mut $buffer),
            7 => $x.radix::<7>(&mut $buffer),
            8 => $x.radix::<8>(&mut $buffer),
            9 => $x.radix::<9>(&mut $buffer),
            10 => $x.radix::<10>(&mut $buffer),
            11 => $x.radix::<11>(&mut $buffer),
            12 => $x.radix::<12>(&mut $buffer),
            13 => $x.radix::<13>(&mut $buffer),
            14 => $x.radix::<14>(&mut $buffer),
            15 => $x.radix::<15>(&mut $buffer),
            16 => $x.radix::<16>(&mut $buffer),
            17 => $x.radix::<17>(&mut $buffer),
            18 => $x.radix::<18>(&mut $buffer),
            19 => $x.radix::<19>(&mut $buffer),
            20 => $x.radix::<20>(&mut $buffer),
            21 => $x.radix::<21>(&mut $buffer),
            22 => $x.radix::<22>(&mut $buffer),
            23 => $x.radix::<23>(&mut $buffer),
            24 => $x.radix::<24>(&mut $buffer),
            25 => $x.radix::<25>(&mut $buffer),
            26 => $x.radix::<26>(&mut $buffer),
            27 => $x.radix::<27>(&mut $buffer),
            28 => $x.radix::<28>(&mut $buffer),
            29 => $x.radix::<29>(&mut $buffer),
            30 => $x.radix::<30>(&mut $buffer),
            31 => $x.radix::<31>(&mut $buffer),
            32 => $x.radix::<32>(&mut $buffer),
            33 => $x.radix::<33>(&mut $buffer),
            34 => $x.radix::<34>(&mut $buffer),
            35 => $x.radix::<35>(&mut $buffer),
            36 => $x.radix::<36>(&mut $buffer),
            _ => unimplemented!(),
        }
    };
}

fn u32toa_mockup(x: u32, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 32];
    unsafe {
        let count = to_radix!(x, radix, buffer);
        let y = u32::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u64toa_mockup(x: u64, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 64];
    unsafe {
        let count = to_radix!(x, radix, buffer);
        let y = u64::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u128toa_mockup(x: u128, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 128];
    unsafe {
        let count = to_radix!(x, radix, buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

#[cfg(not(miri))]
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
