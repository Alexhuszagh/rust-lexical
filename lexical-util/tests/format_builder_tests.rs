use lexical_util::format::{NumberFormat, NumberFormatBuilder};

#[test]
fn decimal_test() {
    const FORMAT: u128 = NumberFormatBuilder::decimal();
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 10);
    assert_eq!(format.mantissa_radix(), 10);
    assert_eq!(format.exponent_base(), 10);
    assert_eq!(format.exponent_radix(), 10);
}

#[test]
#[cfg(feature = "power-of-two")]
fn binary_test() {
    const FORMAT: u128 = NumberFormatBuilder::binary();
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 2);
    assert_eq!(format.mantissa_radix(), 2);
    assert_eq!(format.exponent_base(), 2);
    assert_eq!(format.exponent_radix(), 2);
}

#[test]
#[cfg(feature = "power-of-two")]
fn octal_test() {
    const FORMAT: u128 = NumberFormatBuilder::octal();
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 8);
    assert_eq!(format.mantissa_radix(), 8);
    assert_eq!(format.exponent_base(), 8);
    assert_eq!(format.exponent_radix(), 8);
}

#[test]
#[cfg(feature = "power-of-two")]
fn hexadecimal_test() {
    const FORMAT: u128 = NumberFormatBuilder::hexadecimal();
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 16);
    assert_eq!(format.mantissa_radix(), 16);
    assert_eq!(format.exponent_base(), 16);
    assert_eq!(format.exponent_radix(), 16);
}

#[test]
#[cfg(feature = "power-of-two")]
fn from_radix_test() {
    const FORMAT: u128 = NumberFormatBuilder::from_radix(32);
    let format = NumberFormat::<FORMAT> {};
    assert!(format.is_valid());
    assert_eq!(format.radix(), 32);
    assert_eq!(format.mantissa_radix(), 32);
    assert_eq!(format.exponent_base(), 32);
    assert_eq!(format.exponent_radix(), 32);
}
