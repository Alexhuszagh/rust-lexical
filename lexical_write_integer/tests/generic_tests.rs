#![cfg(feature = "power_of_two")]

use std::str::from_utf8_unchecked;

use lexical_write_integer::{generic, table};
use proptest::prelude::*;

#[test]
fn u128toa_test() {
    let mut buffer = [b'0'; 128];
    unsafe {
        let radix = 12;
        let tbl = table::get_table(radix);
        let value = 136551478823710021067381144334863695872u128;
        let index = generic::generic_u128(value, radix, tbl, &mut buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[index..]), radix);
        assert_eq!(y, Ok(value));
    }
}

fn u8toa_mockup(x: u8, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'0'; 16];
    unsafe {
        let tbl = table::get_table(radix);
        let index = generic::generic(x as u32, radix, tbl, &mut buffer);
        let y = u8::from_str_radix(from_utf8_unchecked(&buffer[index..]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u16toa_mockup(x: u16, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'0'; 16];
    unsafe {
        let tbl = table::get_table(radix);
        let index = generic::generic(x as u32, radix, tbl, &mut buffer);
        let y = u16::from_str_radix(from_utf8_unchecked(&buffer[index..]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u32toa_mockup(x: u32, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'0'; 32];
    unsafe {
        let tbl = table::get_table(radix);
        let index = generic::generic(x as u32, radix, tbl, &mut buffer);
        let y = u32::from_str_radix(from_utf8_unchecked(&buffer[index..]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u64toa_mockup(x: u64, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'0'; 64];
    unsafe {
        let tbl = table::get_table(radix);
        let index = generic::generic(x, radix, tbl, &mut buffer);
        let y = u64::from_str_radix(from_utf8_unchecked(&buffer[index..]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

fn u128toa_mockup(x: u128, radix: u32) -> Result<(), TestCaseError> {
    let mut buffer = [b'0'; 128];
    unsafe {
        let tbl = table::get_table(radix);
        let index = generic::generic_u128(x, radix, tbl, &mut buffer);
        let y = u128::from_str_radix(from_utf8_unchecked(&buffer[index..]), radix);
        prop_assert_eq!(y, Ok(x));
    }

    Ok(())
}

proptest! {
    #[test]
    #[cfg(feature = "radix")]
    fn u8toa_proptest(x: u8, radix in 2u32..=36) {
        u8toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg(not(feature = "radix"))]
    fn u8toa_proptest(x: u8, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        u8toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u16toa_proptest(x: u16, radix in 2u32..=36) {
        u16toa_mockup(x, radix)?;
    }

    #[test]
    #[cfg(not(feature = "radix"))]
    fn u16toa_proptest(x: u16, power in 1u32..=5) {
        let radix = 2u32.pow(power);
        u16toa_mockup(x, radix)?;
    }

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
