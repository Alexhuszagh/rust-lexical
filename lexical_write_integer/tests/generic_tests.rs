#![cfg(feature = "power_of_two")]

use std::str::from_utf8_unchecked;

use lexical_util::constants::BUFFER_SIZE;
use lexical_write_integer::generic::Generic;
use proptest::prelude::*;

#[test]
fn u128toa_test() {
    let mut buffer = [b'\x00'; 128];
    unsafe {
        let radix = 12;
        let value = 136551478823710021067381144334863695872u128;
        let count = value.generic(radix, &mut buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        assert_eq!(y, Ok(value));
    }
}

#[cfg(feature = "power_of_two")]
fn itoa<T: Generic>(x: T, radix: u32, actual: &[u8]) {
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    unsafe {
        let count = x.generic(radix, &mut buffer);
        assert_eq!(actual.len(), count);
        assert_eq!(actual, &buffer[..count])
    }
}

#[test]
#[cfg(feature = "power_of_two")]
fn binary_test() {
    // Binary
    itoa(0u128, 2, b"0");
    itoa(1u128, 2, b"1");
    itoa(5u128, 2, b"101");
    itoa(170141183460469231731687303715884105727u128, 2, b"1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");

    // Hexadecimal
    itoa(170141183460469231731687303715884105727u128, 16, b"7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
}

#[test]
#[cfg(feature = "radix")]
fn radix_test() {
    let data = [
        (2, "100101"),
        (3, "1101"),
        (4, "211"),
        (5, "122"),
        (6, "101"),
        (7, "52"),
        (8, "45"),
        (9, "41"),
        (10, "37"),
        (11, "34"),
        (12, "31"),
        (13, "2B"),
        (14, "29"),
        (15, "27"),
        (16, "25"),
        (17, "23"),
        (18, "21"),
        (19, "1I"),
        (20, "1H"),
        (21, "1G"),
        (22, "1F"),
        (23, "1E"),
        (24, "1D"),
        (25, "1C"),
        (26, "1B"),
        (27, "1A"),
        (28, "19"),
        (29, "18"),
        (30, "17"),
        (31, "16"),
        (32, "15"),
        (33, "14"),
        (34, "13"),
        (35, "12"),
        (36, "11"),
    ];

    for (radix, expected) in data.iter() {
        itoa(37u32, *radix, expected.as_bytes());
    }
}

fn u32toa_mockup(x: u32, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 32];
    unsafe {
        let count = x.generic(radix, &mut buffer);
        let y = u32::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u64toa_mockup(x: u64, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 64];
    unsafe {
        let count = x.generic(radix, &mut buffer);
        let y = u64::from_str_radix(from_utf8_unchecked(&buffer[..count]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u128toa_mockup(x: u128, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'\x00'; 128];
    unsafe {
        let count = x.generic(radix, &mut buffer);
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
