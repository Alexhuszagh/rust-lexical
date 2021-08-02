use lexical_util::format::{self, NumberFormat, NumberFormatBuilder};

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

#[test]
#[cfg(feature = "format")]
fn csharp_rebuild_test() {
    const F1: u128 = NumberFormat::<{ format::CSHARP7_LITERAL }>::rebuild()
        .exponent(b'.')
        .decimal_point(b',')
        .build();

    let f0 = NumberFormat::<{ format::CSHARP7_LITERAL }> {};
    let f1 = NumberFormat::<F1> {};
    assert_eq!(f0.digit_separator(), b'_');
    assert_eq!(f1.digit_separator(), b'_');
    assert_eq!(f1.flags(), f0.flags());
    assert_eq!(f0.decimal_point(), b'.');
    assert_eq!(f1.decimal_point(), b',');
}

#[test]
fn builder_test() {
    // Test a few invalid ones.
    const F0: u128 = NumberFormatBuilder::new().exponent(b'.').build();
    assert_eq!(NumberFormat::<F0> {}.is_valid(), false);

    // Test a few valid ones.
    const F1: u128 = NumberFormatBuilder::new().decimal_point(b'.').build();
    let fmt = NumberFormat::<F1> {};
    assert_eq!(fmt.is_valid(), true);
    assert_eq!(fmt.decimal_point(), b'.');
    assert_eq!(fmt.exponent(), b'e');
    assert_eq!(fmt.required_integer_digits(), false);
    assert_eq!(fmt.required_fraction_digits(), false);
    assert_eq!(fmt.required_exponent_digits(), true);
}

#[test]
fn rebuild_test() {
    const FORMAT: u128 = NumberFormatBuilder::decimal();
    const F0: u128 = NumberFormat::<{ FORMAT }>::rebuild().decimal_point(b',').build();
    let fmt = NumberFormat::<F0> {};
    assert_eq!(fmt.is_valid(), true);
    assert_eq!(fmt.decimal_point(), b',');
    assert_eq!(fmt.exponent(), b'e');

    const F1: u128 = NumberFormat::<F0>::rebuild().exponent(b'f').build();
    let fmt = NumberFormat::<F1> {};
    assert_eq!(fmt.is_valid(), true);
    assert_eq!(fmt.decimal_point(), b',');
    assert_eq!(fmt.exponent(), b'f');

    const F2: u128 = NumberFormat::<F1>::rebuild().exponent(b'$').build();
    let fmt = NumberFormat::<F2> {};
    assert_eq!(fmt.is_valid(), true);
    assert_eq!(fmt.decimal_point(), b',');
    assert_eq!(fmt.exponent(), b'$');
}
