#![cfg(feature = "power-of-two")]

mod parse_radix;

use core::num;
use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
use lexical_util::format::NumberFormatBuilder;
use lexical_util::num::{Float, Integer};
use lexical_write_float::options::RoundMode;
use lexical_write_float::{binary, Options};
use lexical_write_integer::write::WriteInteger;
use parse_radix::{parse_f32, parse_f64};
use proptest::prelude::*;
use quickcheck::quickcheck;

const BINARY: u128 = NumberFormatBuilder::binary();
const BASE4: u128 = NumberFormatBuilder::from_radix(4);
const OCTAL: u128 = NumberFormatBuilder::octal();
const HEX: u128 = NumberFormatBuilder::hexadecimal();
const BASE32: u128 = NumberFormatBuilder::from_radix(32);
const BASE2_2_4: u128 = NumberFormatBuilder::new()
    .mantissa_radix(2)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(4))
    .build();
const BASE4_4_8: u128 = NumberFormatBuilder::new()
    .mantissa_radix(4)
    .exponent_base(num::NonZeroU8::new(4))
    .exponent_radix(num::NonZeroU8::new(8))
    .build();
const BASE4_2_32: u128 = NumberFormatBuilder::new()
    .mantissa_radix(4)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(32))
    .build();
const BASE4_8_4: u128 = NumberFormatBuilder::new()
    .mantissa_radix(4)
    .exponent_base(num::NonZeroU8::new(8))
    .exponent_radix(num::NonZeroU8::new(4))
    .build();
const BASE32_2_32: u128 = NumberFormatBuilder::new()
    .mantissa_radix(32)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(32))
    .build();
const HEX_OPTIONS: Options = unsafe { Options::builder().exponent(b'^').build_unchecked() };

#[test]
fn fast_log2_test() {
    assert_eq!(binary::fast_log2(2), 1);
    assert_eq!(binary::fast_log2(4), 2);
    assert_eq!(binary::fast_log2(8), 3);
    assert_eq!(binary::fast_log2(16), 4);
    assert_eq!(binary::fast_log2(32), 5);
}

#[test]
fn significant_bits_test() {
    assert_eq!(binary::significant_bits(0u32), 0);
    assert_eq!(binary::significant_bits(1u32), 1);
    assert_eq!(binary::significant_bits(2u32), 2);
    assert_eq!(binary::significant_bits(3u32), 2);
    assert_eq!(binary::significant_bits(4u32), 3);
    assert_eq!(binary::significant_bits(5u32), 3);
    assert_eq!(binary::significant_bits(8u32), 4);
    assert_eq!(binary::significant_bits(9u32), 4);
    assert_eq!(binary::significant_bits(15u32), 4);
    assert_eq!(binary::significant_bits(16u32), 5);
    assert_eq!(binary::significant_bits(17u32), 5);
}

#[test]
fn fast_ceildiv_test() {
    assert_eq!(binary::fast_ceildiv(10, 4), 3);
    assert_eq!(binary::fast_ceildiv(10, 5), 2);
    assert_eq!(binary::fast_ceildiv(10, 6), 2);
    assert_eq!(binary::fast_ceildiv(0, 5), 0);
    assert_eq!(binary::fast_ceildiv(4, 5), 1);
    assert_eq!(binary::fast_ceildiv(5, 5), 1);
    assert_eq!(binary::fast_ceildiv(6, 5), 2);
    assert_eq!(binary::fast_ceildiv(9, 5), 2);
    assert_eq!(binary::fast_ceildiv(11, 5), 3);
}

#[test]
fn inverse_remainder_test() {
    assert_eq!(binary::inverse_remainder(0, 8), 0);
    assert_eq!(binary::inverse_remainder(1, 8), 7);
    assert_eq!(binary::inverse_remainder(2, 8), 6);
    assert_eq!(binary::inverse_remainder(3, 8), 5);
}

#[test]
fn calculate_shl_test() {
    // Binary will always be a 0 shift.
    assert_eq!(binary::calculate_shl(2, 1), 0);
    assert_eq!(binary::calculate_shl(3, 1), 0);
    assert_eq!(binary::calculate_shl(5, 1), 0);
    assert_eq!(binary::calculate_shl(-5, 1), 0);
    assert_eq!(binary::calculate_shl(-3, 1), 0);
    assert_eq!(binary::calculate_shl(-2, 1), 0);

    // Can have a 0 or 1 shift for base 4.
    assert_eq!(binary::calculate_shl(2, 2), 0);
    assert_eq!(binary::calculate_shl(3, 2), 1);
    assert_eq!(binary::calculate_shl(4, 2), 0);
    assert_eq!(binary::calculate_shl(-4, 2), 0);
    assert_eq!(binary::calculate_shl(-3, 2), 1);
    assert_eq!(binary::calculate_shl(-2, 2), 0);

    // Octal can have a `[0, 2]` shift.
    assert_eq!(binary::calculate_shl(2, 3), 2);
    assert_eq!(binary::calculate_shl(3, 3), 0);
    assert_eq!(binary::calculate_shl(4, 3), 1);
    assert_eq!(binary::calculate_shl(-3, 3), 0);
    assert_eq!(binary::calculate_shl(-2, 3), 1);
    assert_eq!(binary::calculate_shl(-1, 3), 2);
}

#[test]
fn scale_sci_exp_test() {
    // Binary is always the same.
    assert_eq!(binary::scale_sci_exp(2, 1), 2);
    assert_eq!(binary::scale_sci_exp(1, 1), 1);
    assert_eq!(binary::scale_sci_exp(0, 1), 0);
    assert_eq!(binary::scale_sci_exp(-1, 1), -1);
    assert_eq!(binary::scale_sci_exp(-2, 1), -2);

    // Base 4 will always be the round-to-negative-infinity div.
    assert_eq!(binary::scale_sci_exp(2, 2), 1);
    assert_eq!(binary::scale_sci_exp(1, 2), 0);
    assert_eq!(binary::scale_sci_exp(0, 2), 0);
    assert_eq!(binary::scale_sci_exp(-1, 2), -1);
    assert_eq!(binary::scale_sci_exp(-2, 2), -1);
    assert_eq!(binary::scale_sci_exp(-3, 2), -2);
}

#[test]
fn truncate_and_round_test() {
    let truncate = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Truncate)
        .build()
        .unwrap();
    let round = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Round)
        .build()
        .unwrap();

    // Above halfway
    assert_eq!(binary::truncate_and_round(6602499140956772u64, 2, &round), (12, 53));
    assert_eq!(binary::truncate_and_round(6602499140956772u64, 2, &truncate), (11, 53));

    // At halfway
    assert_eq!(binary::truncate_and_round(6473924464345088u64, 2, &round), (12, 53));
    assert_eq!(binary::truncate_and_round(6473924464345088u64, 2, &truncate), (11, 53));

    // Below halfway.
    assert_eq!(binary::truncate_and_round(6473924464345087u64, 2, &round), (11, 53));
    assert_eq!(binary::truncate_and_round(6473924464345087u64, 2, &truncate), (11, 53));
}

// NOTE: This doesn't handle float rounding or truncation.
// It assumes this has already been done.
fn write_float_scientific<T: Float, const FORMAT: u128>(f: T, options: &Options, expected: &str)
where
    <T as Float>::Unsigned: WriteInteger + FormattedSize,
{
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let mantissa = f.mantissa();
    let mantissa_bits = binary::significant_bits(mantissa) as i32;
    let exp = f.exponent();
    let mut sci_exp = exp + mantissa_bits - 1;
    if mantissa == <T as Float>::Unsigned::ZERO {
        sci_exp = 0;
    }

    let count = unsafe {
        binary::write_float_scientific::<_, FORMAT>(mantissa, exp, sci_exp, &mut buffer, options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_scientific_test() {
    // Positive exponent

    // Check no formatting, binary.
    let options = Options::builder().build().unwrap();
    write_float_scientific::<_, BINARY>(0.0f64, &options, "0.0e0");
    write_float_scientific::<_, BINARY>(1.0f64, &options, "1.0e0");
    write_float_scientific::<_, BINARY>(2.0f64, &options, "1.0e1");
    write_float_scientific::<_, BINARY>(0.5f64, &options, "1.0e-1");
    write_float_scientific::<_, BINARY>(
        0.2345678901234567890e2f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001e100",
    );
    write_float_scientific::<_, BINARY>(
        0.1172839450617284e2f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001e11",
    );
    write_float_scientific::<_, BINARY>(
        0.0586419725308642e2f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001e10",
    );
    write_float_scientific::<_, BINARY>(
        0.0293209862654321e2f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001e1",
    );
    write_float_scientific::<_, BINARY>(
        0.01466049313271605e2f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001e0",
    );

    // Check no formatting, base 4.
    write_float_scientific::<_, BASE4>(0.0f64, &options, "0.0e0");
    write_float_scientific::<_, BASE4>(1.0f64, &options, "1.0e0");
    write_float_scientific::<_, BASE4>(2.0f64, &options, "2.0e0");
    write_float_scientific::<_, BASE4>(0.5f64, &options, "2.0e-1");
    write_float_scientific::<_, BASE4>(
        0.2345678901234567890e2f64,
        &options,
        "1.1313103300013332310302121e2",
    );
    write_float_scientific::<_, BASE4>(
        0.1172839450617284e2f64,
        &options,
        "2.3232213200033331221210302e1",
    );
    write_float_scientific::<_, BASE4>(
        0.0586419725308642e2f64,
        &options,
        "1.1313103300013332310302121e1",
    );
    write_float_scientific::<_, BASE4>(
        0.0293209862654321e2f64,
        &options,
        "2.3232213200033331221210302e0",
    );
    write_float_scientific::<_, BASE4>(
        0.01466049313271605e2f64,
        &options,
        "1.1313103300013332310302121e0",
    );

    // Check no formatting, octal.
    write_float_scientific::<_, OCTAL>(0.0f64, &options, "0.0e0");
    write_float_scientific::<_, OCTAL>(1.0f64, &options, "1.0e0");
    write_float_scientific::<_, OCTAL>(2.0f64, &options, "2.0e0");
    write_float_scientific::<_, OCTAL>(0.5f64, &options, "4.0e-1");
    write_float_scientific::<_, OCTAL>(
        0.2345678901234567890e2f64,
        &options,
        "2.73517003773231144e1",
    );
    write_float_scientific::<_, OCTAL>(0.1172839450617284e2f64, &options, "1.35647401775514462e1");
    write_float_scientific::<_, OCTAL>(0.0586419725308642e2f64, &options, "5.6723600776646231e0");
    write_float_scientific::<_, OCTAL>(0.0293209862654321e2f64, &options, "2.73517003773231144e0");
    write_float_scientific::<_, OCTAL>(0.01466049313271605e2f64, &options, "1.35647401775514462e0");

    // Check no formatting, hexadecimal.
    write_float_scientific::<_, HEX>(0.0f64, &HEX_OPTIONS, "0.0^0");
    write_float_scientific::<_, HEX>(1.0f64, &HEX_OPTIONS, "1.0^0");
    write_float_scientific::<_, HEX>(2.0f64, &HEX_OPTIONS, "2.0^0");
    write_float_scientific::<_, HEX>(0.5f64, &HEX_OPTIONS, "8.0^-1");
    write_float_scientific::<_, HEX>(0.2345678901234567890e2f64, &HEX_OPTIONS, "1.774F01FED3264^1");
    write_float_scientific::<_, HEX>(0.1172839450617284e2f64, &HEX_OPTIONS, "B.BA780FF69932^0");
    write_float_scientific::<_, HEX>(0.0586419725308642e2f64, &HEX_OPTIONS, "5.DD3C07FB4C99^0");
    write_float_scientific::<_, HEX>(0.0293209862654321e2f64, &HEX_OPTIONS, "2.EE9E03FDA64C8^0");
    write_float_scientific::<_, HEX>(0.01466049313271605e2f64, &HEX_OPTIONS, "1.774F01FED3264^0");

    // Check no formatting, base 32.
    write_float_scientific::<_, BASE32>(0.0f64, &HEX_OPTIONS, "0.0^0");
    write_float_scientific::<_, BASE32>(1.0f64, &HEX_OPTIONS, "1.0^0");
    write_float_scientific::<_, BASE32>(2.0f64, &HEX_OPTIONS, "2.0^0");
    write_float_scientific::<_, BASE32>(0.5f64, &HEX_OPTIONS, "G.0^-1");
    write_float_scientific::<_, BASE32>(0.2345678901234567890e2f64, &HEX_OPTIONS, "N.EJO1VR9ICG^0");
    write_float_scientific::<_, BASE32>(0.1172839450617284e2f64, &HEX_OPTIONS, "B.N9S0VTKP68^0");
    write_float_scientific::<_, BASE32>(0.0586419725308642e2f64, &HEX_OPTIONS, "5.RKU0FUQCJ4^0");
    write_float_scientific::<_, BASE32>(0.0293209862654321e2f64, &HEX_OPTIONS, "2.TQF07VD69I^0");
    write_float_scientific::<_, BASE32>(0.01466049313271605e2f64, &HEX_OPTIONS, "1.ET7G3VMJ4P^0");

    // Negative exponent

    // Check no formatting, binary.
    write_float_scientific::<_, BINARY>(
        0.2345678901234567890f64,
        &options,
        "1.11100000011001010010000101000110001011001111110111e-11",
    );
    write_float_scientific::<_, BINARY>(
        0.1172839450617284f64,
        &options,
        "1.11100000011001010010000101000110001011001111110111e-100",
    );
    write_float_scientific::<_, BINARY>(
        0.0586419725308642f64,
        &options,
        "1.11100000011001010010000101000110001011001111110111e-101",
    );
    write_float_scientific::<_, BINARY>(
        0.0293209862654321f64,
        &options,
        "1.11100000011001010010000101000110001011001111110111e-110",
    );
    write_float_scientific::<_, BINARY>(
        0.01466049313271605f64,
        &options,
        "1.11100000011001010010000101000110001011001111110111e-111",
    );

    // Check no formatting, base 4.
    write_float_scientific::<_, BASE4>(
        0.2345678901234567890f64,
        &options,
        "3.3000302210022030112133232e-2",
    );
    write_float_scientific::<_, BASE4>(
        0.1172839450617284f64,
        &options,
        "1.3200121102011012023033313e-2",
    );
    write_float_scientific::<_, BASE4>(
        0.0586419725308642f64,
        &options,
        "3.3000302210022030112133232e-3",
    );
    write_float_scientific::<_, BASE4>(
        0.0293209862654321f64,
        &options,
        "1.3200121102011012023033313e-3",
    );
    write_float_scientific::<_, BASE4>(
        0.01466049313271605f64,
        &options,
        "3.3000302210022030112133232e-10",
    );

    // Check no formatting, octal.
    write_float_scientific::<_, OCTAL>(
        0.2345678901234567890f64,
        &options,
        "1.70062441214263756e-1",
    );
    write_float_scientific::<_, OCTAL>(0.1172839450617284f64, &options, "7.4031220506131767e-2");
    write_float_scientific::<_, OCTAL>(0.0586419725308642f64, &options, "3.60145102430547734e-2");
    write_float_scientific::<_, OCTAL>(0.0293209862654321f64, &options, "1.70062441214263756e-2");
    write_float_scientific::<_, OCTAL>(0.01466049313271605f64, &options, "7.4031220506131767e-3");

    // Check no formatting, hexadecimal.
    write_float_scientific::<_, HEX>(0.2345678901234567890f64, &HEX_OPTIONS, "3.C0CA428C59FB8^-1");
    write_float_scientific::<_, HEX>(0.1172839450617284f64, &HEX_OPTIONS, "1.E06521462CFDC^-1");
    write_float_scientific::<_, HEX>(0.0586419725308642f64, &HEX_OPTIONS, "F.03290A3167EE^-2");
    write_float_scientific::<_, HEX>(0.0293209862654321f64, &HEX_OPTIONS, "7.81948518B3F7^-2");
    write_float_scientific::<_, HEX>(0.01466049313271605f64, &HEX_OPTIONS, "3.C0CA428C59FB8^-2");

    // Check no formatting, base 32.
    write_float_scientific::<_, BASE32>(0.2345678901234567890f64, &HEX_OPTIONS, "7.G6A8A65JUS^-1");
    write_float_scientific::<_, BASE32>(0.1172839450617284f64, &HEX_OPTIONS, "3.O354532PVE^-1");
    write_float_scientific::<_, BASE32>(0.0586419725308642f64, &HEX_OPTIONS, "1.S1II2HHCVN^-1");
    write_float_scientific::<_, BASE32>(0.0293209862654321f64, &HEX_OPTIONS, "U.0P918OMFRG^-2");
    write_float_scientific::<_, BASE32>(0.01466049313271605f64, &HEX_OPTIONS, "F.0CKGKCB7TO^-2");

    // Different exponent radix.
    write_float_scientific::<_, BASE2_2_4>(
        0.2345678901234567890e2f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001e10",
    );
    write_float_scientific::<_, BASE4_4_8>(
        0.2345678901234567890e2f64,
        &options,
        "1.1313103300013332310302121e2",
    );

    // Check no formatting, f32, binary.
    write_float_scientific::<_, BINARY>(
        1.2345678901234567890f32,
        &options,
        "1.0011110000001100101001e0",
    );
    write_float_scientific::<_, BINARY>(
        3.2345678901234567890f32,
        &options,
        "1.10011110000001100101001e1",
    );
    write_float_scientific::<_, BINARY>(1f32, &options, "1.0e0");
    write_float_scientific::<_, BINARY>(
        0.2345678901234567890f32,
        &options,
        "1.11100000011001010010001e-11",
    );
    write_float_scientific::<_, BINARY>(
        0.7345678901234567890f32,
        &options,
        "1.011110000001100101001e-1",
    );
    write_float_scientific::<_, BINARY>(1.4e-45f32, &options, "1.0e-10010101");
    write_float_scientific::<_, BINARY>(
        3.4028234664e38f32,
        &options,
        "1.11111111111111111111111e1111111",
    );

    // Check with a minimum number of digits.
    let options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(5)).build().unwrap();
    write_float_scientific::<_, BINARY>(0.0f64, &options, "0.0000e0");
    write_float_scientific::<_, BINARY>(1.0f64, &options, "1.0000e0");
    write_float_scientific::<_, BINARY>(2.0f64, &options, "1.0000e1");
    write_float_scientific::<_, BINARY>(0.5f64, &options, "1.0000e-1");
    write_float_scientific::<_, BASE4>(
        0.2345678901234567890e2f64,
        &options,
        "1.1313103300013332310302121e2",
    );

    let options = Options::builder()
        .min_significant_digits(num::NonZeroUsize::new(5))
        .trim_floats(true)
        .build()
        .unwrap();
    write_float_scientific::<_, BINARY>(0.0f64, &options, "0e0");
    write_float_scientific::<_, BINARY>(1.0f64, &options, "1e0");
    write_float_scientific::<_, BINARY>(2.0f64, &options, "1e1");
    write_float_scientific::<_, BINARY>(0.5f64, &options, "1e-1");
    write_float_scientific::<_, BASE4>(
        0.2345678901234567890e2f64,
        &options,
        "1.1313103300013332310302121e2",
    );

    // Check trimming floats
    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float_scientific::<_, BINARY>(1f32, &options, "1e0");
    write_float_scientific::<_, BINARY>(1.4e-45f32, &options, "1e-10010101");
    write_float_scientific::<_, BINARY>(
        1.2345678901234567890f32,
        &options,
        "1.0011110000001100101001e0",
    );
}

// NOTE: This doesn't handle float rounding or truncation.
// It assumes this has already been done.
fn write_float_negative_exponent<T: Float, const FORMAT: u128>(
    f: T,
    options: &Options,
    expected: &str,
) where
    <T as Float>::Unsigned: WriteInteger + FormattedSize,
{
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let mantissa = f.mantissa();
    let mantissa_bits = binary::significant_bits(mantissa) as i32;
    let exp = f.exponent();
    let sci_exp = exp + mantissa_bits - 1;

    let count = unsafe {
        binary::write_float_negative_exponent::<_, FORMAT>(
            mantissa,
            exp,
            sci_exp,
            &mut buffer,
            options,
        )
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_negative_exponent_test() {
    // Negative exponent

    // Check no formatting, binary.
    let options = Options::builder().build().unwrap();
    write_float_negative_exponent::<_, BINARY>(
        0.2345678901234567890f64,
        &options,
        "0.00111100000011001010010000101000110001011001111110111",
    );
    write_float_negative_exponent::<_, BINARY>(
        0.1172839450617284f64,
        &options,
        "0.000111100000011001010010000101000110001011001111110111",
    );
    write_float_negative_exponent::<_, BINARY>(
        0.0586419725308642f64,
        &options,
        "0.0000111100000011001010010000101000110001011001111110111",
    );
    write_float_negative_exponent::<_, BINARY>(
        0.0293209862654321f64,
        &options,
        "0.00000111100000011001010010000101000110001011001111110111",
    );
    write_float_negative_exponent::<_, BINARY>(
        0.01466049313271605f64,
        &options,
        "0.000000111100000011001010010000101000110001011001111110111",
    );

    // Check no formatting, base 4.
    write_float_negative_exponent::<_, BASE4>(
        0.2345678901234567890f64,
        &options,
        "0.033000302210022030112133232",
    );
    write_float_negative_exponent::<_, BASE4>(
        0.1172839450617284f64,
        &options,
        "0.013200121102011012023033313",
    );
    write_float_negative_exponent::<_, BASE4>(
        0.0586419725308642f64,
        &options,
        "0.0033000302210022030112133232",
    );
    write_float_negative_exponent::<_, BASE4>(
        0.0293209862654321f64,
        &options,
        "0.0013200121102011012023033313",
    );
    write_float_negative_exponent::<_, BASE4>(
        0.01466049313271605f64,
        &options,
        "0.00033000302210022030112133232",
    );

    // Check no formatting, octal.
    write_float_negative_exponent::<_, OCTAL>(
        0.2345678901234567890f64,
        &options,
        "0.170062441214263756",
    );
    write_float_negative_exponent::<_, OCTAL>(
        0.1172839450617284f64,
        &options,
        "0.074031220506131767",
    );
    write_float_negative_exponent::<_, OCTAL>(
        0.0586419725308642f64,
        &options,
        "0.0360145102430547734",
    );
    write_float_negative_exponent::<_, OCTAL>(
        0.0293209862654321f64,
        &options,
        "0.0170062441214263756",
    );
    write_float_negative_exponent::<_, OCTAL>(
        0.01466049313271605f64,
        &options,
        "0.0074031220506131767",
    );

    // Check no formatting, hexadecimal.
    write_float_negative_exponent::<_, HEX>(
        0.2345678901234567890f64,
        &HEX_OPTIONS,
        "0.3C0CA428C59FB8",
    );
    write_float_negative_exponent::<_, HEX>(
        0.1172839450617284f64,
        &HEX_OPTIONS,
        "0.1E06521462CFDC",
    );
    write_float_negative_exponent::<_, HEX>(
        0.0586419725308642f64,
        &HEX_OPTIONS,
        "0.0F03290A3167EE",
    );
    write_float_negative_exponent::<_, HEX>(
        0.0293209862654321f64,
        &HEX_OPTIONS,
        "0.0781948518B3F7",
    );
    write_float_negative_exponent::<_, HEX>(
        0.01466049313271605f64,
        &HEX_OPTIONS,
        "0.03C0CA428C59FB8",
    );

    // Check no formatting, base 32.
    write_float_negative_exponent::<_, BASE32>(
        0.2345678901234567890f64,
        &HEX_OPTIONS,
        "0.7G6A8A65JUS",
    );
    write_float_negative_exponent::<_, BASE32>(
        0.1172839450617284f64,
        &HEX_OPTIONS,
        "0.3O354532PVE",
    );
    write_float_negative_exponent::<_, BASE32>(
        0.0586419725308642f64,
        &HEX_OPTIONS,
        "0.1S1II2HHCVN",
    );
    write_float_negative_exponent::<_, BASE32>(
        0.0293209862654321f64,
        &HEX_OPTIONS,
        "0.0U0P918OMFRG",
    );
    write_float_negative_exponent::<_, BASE32>(
        0.01466049313271605f64,
        &HEX_OPTIONS,
        "0.0F0CKGKCB7TO",
    );

    // Different exponent radix.
    write_float_negative_exponent::<_, BASE2_2_4>(
        0.2345678901234567890f64,
        &options,
        "0.00111100000011001010010000101000110001011001111110111",
    );
    write_float_negative_exponent::<_, BASE4_2_32>(
        0.2345678901234567890f64,
        &options,
        "0.033000302210022030112133232",
    );
    write_float_negative_exponent::<_, BASE4_4_8>(
        0.2345678901234567890f64,
        &options,
        "0.033000302210022030112133232",
    );
    write_float_negative_exponent::<_, BASE4_8_4>(
        0.2345678901234567890f64,
        &options,
        "0.033000302210022030112133232",
    );
    write_float_negative_exponent::<_, BASE32_2_32>(
        0.2345678901234567890f64,
        &options,
        "0.7G6A8A65JUS",
    );

    // Check no formatting, f32, binary.
    write_float_negative_exponent::<_, BINARY>(
        0.2345678901234567890f32,
        &options,
        "0.00111100000011001010010001",
    );
    write_float_negative_exponent::<_, BINARY>(
        0.7345678901234567890f32,
        &options,
        "0.1011110000001100101001",
    );
    write_float_negative_exponent::<_, BINARY>(1.4e-45f32, &options, "0.00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001");

    // Check with a minimum number of digits.
    let options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(5)).build().unwrap();
    write_float_negative_exponent::<_, BINARY>(0.5f64, &options, "0.10000");
    write_float_negative_exponent::<_, BASE4>(
        0.2345678901234567890f64,
        &options,
        "0.033000302210022030112133232",
    );

    let options = Options::builder()
        .min_significant_digits(num::NonZeroUsize::new(5))
        .trim_floats(true)
        .build()
        .unwrap();
    write_float_negative_exponent::<_, BINARY>(0.5f64, &options, "0.10000");
    write_float_negative_exponent::<_, BASE4>(
        0.2345678901234567890f64,
        &options,
        "0.033000302210022030112133232",
    );

    // Check trimming floats does nothing.
    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float_negative_exponent::<_, BINARY>(0.5f64, &options, "0.1");
    write_float_negative_exponent::<_, BASE4>(
        0.2345678901234567890f64,
        &options,
        "0.033000302210022030112133232",
    );
}

// NOTE: This doesn't handle float rounding or truncation.
// It assumes this has already been done.
fn write_float_positive_exponent<T: Float, const FORMAT: u128>(
    f: T,
    options: &Options,
    expected: &str,
) where
    <T as Float>::Unsigned: WriteInteger + FormattedSize,
{
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let mantissa = f.mantissa();
    let mantissa_bits = binary::significant_bits(mantissa) as i32;
    let exp = f.exponent();
    let mut sci_exp = exp + mantissa_bits - 1;
    if mantissa == <T as Float>::Unsigned::ZERO {
        sci_exp = 0;
    }

    let count = unsafe {
        binary::write_float_positive_exponent::<_, FORMAT>(
            mantissa,
            exp,
            sci_exp,
            &mut buffer,
            options,
        )
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_positive_exponent_test() {
    // Positive exponent

    // Check no formatting, binary.
    let options = Options::builder().build().unwrap();
    write_float_positive_exponent::<_, BINARY>(0.0f64, &options, "0.0");
    write_float_positive_exponent::<_, BINARY>(1.0f64, &options, "1.0");
    write_float_positive_exponent::<_, BINARY>(2.0f64, &options, "10.0");
    write_float_positive_exponent::<_, BINARY>(
        0.2345678901234567890e2f64,
        &options,
        "10111.0111010011110000000111111110110100110010011001",
    );
    write_float_positive_exponent::<_, BINARY>(
        0.1172839450617284e2f64,
        &options,
        "1011.10111010011110000000111111110110100110010011001",
    );
    write_float_positive_exponent::<_, BINARY>(
        0.0586419725308642e2f64,
        &options,
        "101.110111010011110000000111111110110100110010011001",
    );
    write_float_positive_exponent::<_, BINARY>(
        0.0293209862654321e2f64,
        &options,
        "10.1110111010011110000000111111110110100110010011001",
    );
    write_float_positive_exponent::<_, BINARY>(
        0.01466049313271605e2f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001",
    );

    // Check no formatting, base 4.
    write_float_positive_exponent::<_, BASE4>(0.0f64, &options, "0.0");
    write_float_positive_exponent::<_, BASE4>(1.0f64, &options, "1.0");
    write_float_positive_exponent::<_, BASE4>(2.0f64, &options, "2.0");
    write_float_positive_exponent::<_, BASE4>(
        0.2345678901234567890e2f64,
        &options,
        "113.13103300013332310302121",
    );
    write_float_positive_exponent::<_, BASE4>(
        0.1172839450617284e2f64,
        &options,
        "23.232213200033331221210302",
    );
    write_float_positive_exponent::<_, BASE4>(
        0.0586419725308642e2f64,
        &options,
        "11.313103300013332310302121",
    );
    write_float_positive_exponent::<_, BASE4>(
        0.0293209862654321e2f64,
        &options,
        "2.3232213200033331221210302",
    );
    write_float_positive_exponent::<_, BASE4>(
        0.01466049313271605e2f64,
        &options,
        "1.1313103300013332310302121",
    );

    // Check no formatting, octal.
    write_float_positive_exponent::<_, OCTAL>(0.0f64, &options, "0.0");
    write_float_positive_exponent::<_, OCTAL>(1.0f64, &options, "1.0");
    write_float_positive_exponent::<_, OCTAL>(2.0f64, &options, "2.0");
    write_float_positive_exponent::<_, OCTAL>(
        0.2345678901234567890e2f64,
        &options,
        "27.3517003773231144",
    );
    write_float_positive_exponent::<_, OCTAL>(
        0.1172839450617284e2f64,
        &options,
        "13.5647401775514462",
    );
    write_float_positive_exponent::<_, OCTAL>(
        0.0586419725308642e2f64,
        &options,
        "5.6723600776646231",
    );
    write_float_positive_exponent::<_, OCTAL>(
        0.0293209862654321e2f64,
        &options,
        "2.73517003773231144",
    );
    write_float_positive_exponent::<_, OCTAL>(
        0.01466049313271605e2f64,
        &options,
        "1.35647401775514462",
    );

    // Check no formatting, hexadecimal.
    write_float_positive_exponent::<_, HEX>(0.0f64, &HEX_OPTIONS, "0.0");
    write_float_positive_exponent::<_, HEX>(1.0f64, &HEX_OPTIONS, "1.0");
    write_float_positive_exponent::<_, HEX>(2.0f64, &HEX_OPTIONS, "2.0");
    write_float_positive_exponent::<_, HEX>(
        0.2345678901234567890e2f64,
        &HEX_OPTIONS,
        "17.74F01FED3264",
    );
    write_float_positive_exponent::<_, HEX>(
        0.1172839450617284e2f64,
        &HEX_OPTIONS,
        "B.BA780FF69932",
    );
    write_float_positive_exponent::<_, HEX>(
        0.0586419725308642e2f64,
        &HEX_OPTIONS,
        "5.DD3C07FB4C99",
    );
    write_float_positive_exponent::<_, HEX>(
        0.0293209862654321e2f64,
        &HEX_OPTIONS,
        "2.EE9E03FDA64C8",
    );
    write_float_positive_exponent::<_, HEX>(
        0.01466049313271605e2f64,
        &HEX_OPTIONS,
        "1.774F01FED3264",
    );

    // Check no formatting, base 32.
    write_float_positive_exponent::<_, BASE32>(0.0f64, &HEX_OPTIONS, "0.0");
    write_float_positive_exponent::<_, BASE32>(1.0f64, &HEX_OPTIONS, "1.0");
    write_float_positive_exponent::<_, BASE32>(2.0f64, &HEX_OPTIONS, "2.0");
    write_float_positive_exponent::<_, BASE32>(
        0.2345678901234567890e2f64,
        &HEX_OPTIONS,
        "N.EJO1VR9ICG",
    );
    write_float_positive_exponent::<_, BASE32>(
        0.1172839450617284e2f64,
        &HEX_OPTIONS,
        "B.N9S0VTKP68",
    );
    write_float_positive_exponent::<_, BASE32>(
        0.0586419725308642e2f64,
        &HEX_OPTIONS,
        "5.RKU0FUQCJ4",
    );
    write_float_positive_exponent::<_, BASE32>(
        0.0293209862654321e2f64,
        &HEX_OPTIONS,
        "2.TQF07VD69I",
    );
    write_float_positive_exponent::<_, BASE32>(
        0.01466049313271605e2f64,
        &HEX_OPTIONS,
        "1.ET7G3VMJ4P",
    );

    // Different exponent radix.
    write_float_positive_exponent::<_, BASE2_2_4>(
        0.2345678901234567890e2f64,
        &options,
        "10111.0111010011110000000111111110110100110010011001",
    );
    write_float_positive_exponent::<_, BASE4_2_32>(
        0.2345678901234567890e2f64,
        &options,
        "113.13103300013332310302121",
    );
    write_float_positive_exponent::<_, BASE4_4_8>(
        0.2345678901234567890e2f64,
        &options,
        "113.13103300013332310302121",
    );
    write_float_positive_exponent::<_, BASE4_8_4>(
        0.2345678901234567890e2f64,
        &options,
        "113.13103300013332310302121",
    );
    write_float_positive_exponent::<_, BASE32_2_32>(
        0.2345678901234567890e2f64,
        &HEX_OPTIONS,
        "N.EJO1VR9ICG",
    );

    // Check no formatting, f32, binary.
    write_float_positive_exponent::<_, BINARY>(
        0.2345678901234567890e2f32,
        &options,
        "10111.0111010011110000001",
    );
    write_float_positive_exponent::<_, BINARY>(
        0.7345678901234567890e2f32,
        &options,
        "1001001.011101001111",
    );
    write_float_positive_exponent::<_, BINARY>(3.4028234664e38f32, &options, "11111111111111111111111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0");

    // Check with a minimum number of digits.
    let options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(5)).build().unwrap();
    write_float_positive_exponent::<_, BINARY>(1.0f64, &options, "1.0000");
    write_float_positive_exponent::<_, BINARY>(
        0.2345678901234567890e2f32,
        &options,
        "10111.0111010011110000001",
    );

    let options = Options::builder()
        .min_significant_digits(num::NonZeroUsize::new(5))
        .trim_floats(true)
        .build()
        .unwrap();
    write_float_positive_exponent::<_, BINARY>(1.0f64, &options, "1");
    write_float_positive_exponent::<_, BINARY>(
        0.2345678901234567890e2f32,
        &options,
        "10111.0111010011110000001",
    );

    // Check trimming floats works.
    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float_positive_exponent::<_, BINARY>(1.0f64, &options, "1");
    write_float_positive_exponent::<_, BINARY>(
        0.2345678901234567890e2f32,
        &options,
        "10111.0111010011110000001",
    );
}

fn write_float<T: Float, const FORMAT: u128>(f: T, options: &Options, expected: &str)
where
    <T as Float>::Unsigned: WriteInteger + FormattedSize,
{
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let count = unsafe { binary::write_float::<_, FORMAT>(f, &mut buffer, options) };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_test() {
    // Check no formatting, binary, and when exponent notation is used.
    let options = Options::builder().build().unwrap();
    write_float::<_, BINARY>(0.0f64, &options, "0.0");
    write_float::<_, BINARY>(1.0f64, &options, "1.0");
    write_float::<_, BINARY>(2.0f64, &options, "10.0");
    write_float::<_, BINARY>(0.5f64, &options, "0.1");
    write_float::<_, BINARY>(
        23.45678901234567890f64,
        &options,
        "10111.0111010011110000000111111110110100110010011001",
    );
    write_float::<_, BINARY>(
        93.82715604938272f64,
        &options,
        "1011101.11010011110000000111111110110100110010011001",
    );
    write_float::<_, BINARY>(
        375.3086241975309f64,
        &options,
        "101110111.010011110000000111111110110100110010011001",
    );
    write_float::<_, BINARY>(
        750.6172483950618f64,
        &options,
        "1011101110.10011110000000111111110110100110010011001",
    );
    write_float::<_, BINARY>(
        1501.2344967901236f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001e1010",
    );
    write_float::<_, BINARY>(
        0.09162808207947531f64,
        &options,
        "0.000101110111010011110000000111111110110100110010011001",
    );
    write_float::<_, BINARY>(
        0.04581404103973766f64,
        &options,
        "0.0000101110111010011110000000111111110110100110010011001",
    );
    write_float::<_, BINARY>(
        0.02290702051986883f64,
        &options,
        "1.01110111010011110000000111111110110100110010011001e-110",
    );

    // Try changing the exponent limits.
    let options = Options::builder()
        .negative_exponent_break(num::NonZeroI32::new(-6))
        .positive_exponent_break(num::NonZeroI32::new(10))
        .build()
        .unwrap();
    write_float::<_, BINARY>(
        1501.2344967901236f64,
        &options,
        "10111011101.0011110000000111111110110100110010011001",
    );
    write_float::<_, BINARY>(
        0.02290702051986883f64,
        &options,
        "0.00000101110111010011110000000111111110110100110010011001",
    );

    // Check max digits.
    let options =
        Options::builder().max_significant_digits(num::NonZeroUsize::new(5)).build().unwrap();
    write_float::<_, BINARY>(0.0f64, &options, "0.0");
    write_float::<_, BINARY>(1.0f64, &options, "1.0");
    write_float::<_, BINARY>(2.0f64, &options, "10.0");
    write_float::<_, BINARY>(0.5f64, &options, "0.1");
    write_float::<_, BINARY>(0.2345678901234567890f64, &options, "0.001111");
    write_float::<_, BINARY>(23.45678901234567890f64, &options, "10111.0");
    write_float::<_, BINARY>(93.82715604938272f64, &options, "1011100.0");
    write_float::<_, BINARY>(375.3086241975309f64, &options, "101110000.0");

    // Check max digits and trim floats.
    let options = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(5))
        .trim_floats(true)
        .build()
        .unwrap();
    write_float::<_, BINARY>(0.2345678901234567890f64, &options, "0.001111");
    write_float::<_, BINARY>(23.45678901234567890f64, &options, "10111");
    write_float::<_, BINARY>(93.82715604938272f64, &options, "1011100");
    write_float::<_, BINARY>(375.3086241975309f64, &options, "101110000");

    // Test the round mode.
    let truncate = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Truncate)
        .build()
        .unwrap();
    let round = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(4))
        .round_mode(RoundMode::Round)
        .build()
        .unwrap();
    write_float::<_, BINARY>(23.45678901234567890f64, &round, "11000.0");
    write_float::<_, BINARY>(23.45678901234567890f64, &truncate, "10110.0");

    let truncate = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(8))
        .round_mode(RoundMode::Truncate)
        .build()
        .unwrap();
    let round = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(8))
        .round_mode(RoundMode::Round)
        .build()
        .unwrap();

    write_float::<_, BINARY>(1.2345678901234567890e0f64, &truncate, "1.001111");
    write_float::<_, BINARY>(1.2345678901234567890e0f64, &round, "1.001111");
    write_float::<_, BINARY>(1.2345678901234567890e1f64, &truncate, "1100.0101");
    write_float::<_, BINARY>(1.2345678901234567890e1f64, &round, "1100.011");
    write_float::<_, BINARY>(1.2345678901234567890e2f64, &truncate, "1111011.0");
    write_float::<_, BINARY>(1.2345678901234567890e2f64, &round, "1111011.1");
    write_float::<_, BINARY>(1.2345678901234567890e3f64, &truncate, "1.001101e1010");
    write_float::<_, BINARY>(1.2345678901234567890e3f64, &round, "1.001101e1010");

    let truncate = Options::builder()
        .max_significant_digits(num::NonZeroUsize::new(8))
        .round_mode(RoundMode::Truncate)
        .trim_floats(true)
        .build()
        .unwrap();
    write_float::<_, BINARY>(1.2345678901234567890e2f64, &truncate, "1111011");
    write_float::<_, BINARY>(1.2345678901234567890e2f64, &round, "1111011.1");
}

quickcheck! {
    #[cfg_attr(miri, ignore)]
    fn f32_binary_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() {
            true
        } else {
            let f = f.abs();
            let count = unsafe { binary::write_float::<_, BINARY>(f, &mut buffer, &options) };
            let roundtrip = parse_f32(&buffer[..count], 2, b'e');
            roundtrip == f
        }
    }

    #[cfg_attr(miri, ignore)]
    fn f32_octal_quickcheck(f: f32) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() {
            true
        } else {
            let f = f.abs();
            let count = unsafe { binary::write_float::<_, OCTAL>(f, &mut buffer, &options) };
            let roundtrip = parse_f32(&buffer[..count], 8, b'e');
            roundtrip == f
        }
    }

    #[cfg_attr(miri, ignore)]
    fn f64_binary_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() {
            true
        } else {
            let f = f.abs();
            let count = unsafe { binary::write_float::<_, BINARY>(f, &mut buffer, &options) };
            let roundtrip = parse_f64(&buffer[..count], 2, b'e');
            roundtrip == f
        }
    }

    #[cfg_attr(miri, ignore)]
    fn f64_octal_quickcheck(f: f64) -> bool {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if f.is_special() {
            true
        } else {
            let f = f.abs();
            let count = unsafe { binary::write_float::<_, OCTAL>(f, &mut buffer, &options) };
            let roundtrip = parse_f64(&buffer[..count], 8, b'e');
            roundtrip == f
        }
    }
}

proptest! {
    #[test]
    #[cfg_attr(miri, ignore)]
    fn f32_binary_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !f.is_special() {
            let f = f.abs();
            let count = unsafe { binary::write_float::<_, BINARY>(f, &mut buffer, &options) };
            let roundtrip = parse_f32(&buffer[..count], 2, b'e');
            prop_assert_eq!(roundtrip, f)
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn f32_octal_proptest(f in f32::MIN..f32::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !f.is_special() {
            let f = f.abs();
            let count = unsafe { binary::write_float::<_, OCTAL>(f, &mut buffer, &options) };
            let roundtrip = parse_f32(&buffer[..count], 8, b'e');
            prop_assert_eq!(roundtrip, f)
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn f64_binary_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !f.is_special() {
            let f = f.abs();
            let count = unsafe { binary::write_float::<_, BINARY>(f, &mut buffer, &options) };
            let roundtrip = parse_f64(&buffer[..count], 2, b'e');
            prop_assert_eq!(roundtrip, f)
        }
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn f64_octal_proptest(f in f64::MIN..f64::MAX) {
        let mut buffer = [b'\x00'; BUFFER_SIZE];
        let options = Options::builder().build().unwrap();
        if !f.is_special() {
            let f = f.abs();
            let count = unsafe { binary::write_float::<_, OCTAL>(f, &mut buffer, &options) };
            let roundtrip = parse_f64(&buffer[..count], 8, b'e');
            prop_assert_eq!(roundtrip, f)
        }
    }
}
