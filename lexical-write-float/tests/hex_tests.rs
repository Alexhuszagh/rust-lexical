#![cfg(feature = "power-of-two")]

use core::num;
use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
use lexical_util::format::NumberFormatBuilder;
use lexical_util::num::{Float, Integer};
use lexical_write_float::{binary, hex, Options};
use lexical_write_integer::write::WriteInteger;

const BASE4_2_10: u128 = NumberFormatBuilder::new()
    .mantissa_radix(4)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build();
const BASE8_2_10: u128 = NumberFormatBuilder::new()
    .mantissa_radix(8)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build();
const BASE16_2_10: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build();
const BASE32_2_10: u128 = NumberFormatBuilder::new()
    .mantissa_radix(32)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build();
const BASE16_4_10: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(4))
    .exponent_radix(num::NonZeroU8::new(10))
    .build();
const HEX_OPTIONS: Options = unsafe { Options::builder().exponent(b'^').build_unchecked() };

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
        hex::write_float_scientific::<_, FORMAT>(mantissa, exp, sci_exp, &mut buffer, options)
    };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_scientific_test() {
    // Positive exponent

    // Check no formatting, base4/2.
    let options = Options::builder().build().unwrap();
    write_float_scientific::<_, BASE4_2_10>(0.0f64, &options, "0.0e0");
    write_float_scientific::<_, BASE4_2_10>(1.0f64, &options, "1.0e0");
    write_float_scientific::<_, BASE4_2_10>(2.0f64, &options, "2.0e0");
    write_float_scientific::<_, BASE4_2_10>(0.5f64, &options, "2.0e-2");
    write_float_scientific::<_, BASE4_2_10>(
        0.2345678901234567890e20f64,
        &options,
        "1.10112013100111033030021213e64",
    );
    write_float_scientific::<_, BASE4_2_10>(
        0.1172839450617284e20f64,
        &options,
        "2.20230032200222132120103032e62",
    );
    write_float_scientific::<_, BASE4_2_10>(
        0.0586419725308642e20f64,
        &options,
        "1.10112013100111033030021213e62",
    );
    write_float_scientific::<_, BASE4_2_10>(
        0.0293209862654321e20f64,
        &options,
        "2.20230032200222132120103032e60",
    );
    write_float_scientific::<_, BASE4_2_10>(
        0.01466049313271605e20f64,
        &options,
        "1.10112013100111033030021213e60",
    );

    write_float_scientific::<_, BASE4_2_10>(
        0.2345678901234567890e-20f64,
        &options,
        "2.30103300013110301132322302e-70",
    );
    write_float_scientific::<_, BASE4_2_10>(
        0.1172839450617284e-20f64,
        &options,
        "1.12021320003222120233131121e-70",
    );
    write_float_scientific::<_, BASE4_2_10>(
        0.0586419725308642e-20f64,
        &options,
        "2.30103300013110301132322302e-72",
    );
    write_float_scientific::<_, BASE4_2_10>(
        0.0293209862654321e-20f64,
        &options,
        "1.12021320003222120233131121e-72",
    );
    write_float_scientific::<_, BASE4_2_10>(
        0.01466049313271605e-20f64,
        &options,
        "2.30103300013110301132322302e-74",
    );

    // Check no formatting, base8/2.
    write_float_scientific::<_, BASE8_2_10>(0.0f64, &options, "0.0e0");
    write_float_scientific::<_, BASE8_2_10>(1.0f64, &options, "1.0e0");
    write_float_scientific::<_, BASE8_2_10>(2.0f64, &options, "2.0e0");
    write_float_scientific::<_, BASE8_2_10>(0.5f64, &options, "4.0e-3");
    write_float_scientific::<_, BASE8_2_10>(
        0.2345678901234567890e20f64,
        &options,
        "2.42607202517141147e63",
    );
    write_float_scientific::<_, BASE8_2_10>(
        0.1172839450617284e20f64,
        &options,
        "1.213035012474604634e63",
    );
    write_float_scientific::<_, BASE8_2_10>(
        0.0586419725308642e20f64,
        &options,
        "5.05416405236302316e60",
    );
    write_float_scientific::<_, BASE8_2_10>(
        0.0293209862654321e20f64,
        &options,
        "2.42607202517141147e60",
    );
    write_float_scientific::<_, BASE8_2_10>(
        0.01466049313271605e20f64,
        &options,
        "1.213035012474604634e60",
    );

    write_float_scientific::<_, BASE8_2_10>(
        0.2345678901234567890e-20f64,
        &options,
        "1.304740165142756544e-69",
    );
    write_float_scientific::<_, BASE8_2_10>(
        0.1172839450617284e-20f64,
        &options,
        "5.42360072461367262e-72",
    );
    write_float_scientific::<_, BASE8_2_10>(
        0.0586419725308642e-20f64,
        &options,
        "2.61170035230573531e-72",
    );
    write_float_scientific::<_, BASE8_2_10>(
        0.0293209862654321e-20f64,
        &options,
        "1.304740165142756544e-72",
    );
    write_float_scientific::<_, BASE8_2_10>(
        0.01466049313271605e-20f64,
        &options,
        "5.42360072461367262e-75",
    );

    // Check no formatting, base16/2.
    write_float_scientific::<_, BASE16_2_10>(0.0f64, &HEX_OPTIONS, "0.0^0");
    write_float_scientific::<_, BASE16_2_10>(1.0f64, &HEX_OPTIONS, "1.0^0");
    write_float_scientific::<_, BASE16_2_10>(2.0f64, &HEX_OPTIONS, "2.0^0");
    write_float_scientific::<_, BASE16_2_10>(0.5f64, &HEX_OPTIONS, "8.0^-4");
    write_float_scientific::<_, BASE16_2_10>(
        0.2345678901234567890e20f64,
        &HEX_OPTIONS,
        "1.45874153CC267^64",
    );
    write_float_scientific::<_, BASE16_2_10>(
        0.1172839450617284e20f64,
        &HEX_OPTIONS,
        "A.2C3A0A9E61338^60",
    );
    write_float_scientific::<_, BASE16_2_10>(
        0.0586419725308642e20f64,
        &HEX_OPTIONS,
        "5.161D054F3099C^60",
    );
    write_float_scientific::<_, BASE16_2_10>(
        0.0293209862654321e20f64,
        &HEX_OPTIONS,
        "2.8B0E82A7984CE^60",
    );
    write_float_scientific::<_, BASE16_2_10>(
        0.01466049313271605e20f64,
        &HEX_OPTIONS,
        "1.45874153CC267^60",
    );

    write_float_scientific::<_, BASE16_2_10>(
        0.2345678901234567890e-20f64,
        &HEX_OPTIONS,
        "B.13C075317BAC8^-72",
    );
    write_float_scientific::<_, BASE16_2_10>(
        0.1172839450617284e-20f64,
        &HEX_OPTIONS,
        "5.89E03A98BDD64^-72",
    );
    write_float_scientific::<_, BASE16_2_10>(
        0.0586419725308642e-20f64,
        &HEX_OPTIONS,
        "2.C4F01D4C5EEB2^-72",
    );
    write_float_scientific::<_, BASE16_2_10>(
        0.0293209862654321e-20f64,
        &HEX_OPTIONS,
        "1.62780EA62F759^-72",
    );
    write_float_scientific::<_, BASE16_2_10>(
        0.01466049313271605e-20f64,
        &HEX_OPTIONS,
        "B.13C075317BAC8^-76",
    );

    // Check no formatting, base32/2.
    write_float_scientific::<_, BASE32_2_10>(0.0f64, &HEX_OPTIONS, "0.0^0");
    write_float_scientific::<_, BASE32_2_10>(1.0f64, &HEX_OPTIONS, "1.0^0");
    write_float_scientific::<_, BASE32_2_10>(2.0f64, &HEX_OPTIONS, "2.0^0");
    write_float_scientific::<_, BASE32_2_10>(0.5f64, &HEX_OPTIONS, "G.0^-5");
    write_float_scientific::<_, BASE32_2_10>(
        0.2345678901234567890e20f64,
        &HEX_OPTIONS,
        "K.B1Q1AF62CS^60",
    );
    write_float_scientific::<_, BASE32_2_10>(
        0.1172839450617284e20f64,
        &HEX_OPTIONS,
        "A.5GT0L7J16E^60",
    );
    write_float_scientific::<_, BASE32_2_10>(
        0.0586419725308642e20f64,
        &HEX_OPTIONS,
        "5.2OEGAJPGJ7^60",
    );
    write_float_scientific::<_, BASE32_2_10>(
        0.0293209862654321e20f64,
        &HEX_OPTIONS,
        "2.HC7859SO9JG^60",
    );
    write_float_scientific::<_, BASE32_2_10>(
        0.01466049313271605e20f64,
        &HEX_OPTIONS,
        "1.8M3K2KUC4PO^60",
    );

    write_float_scientific::<_, BASE32_2_10>(
        0.2345678901234567890e-20f64,
        &HEX_OPTIONS,
        "2.OJO1QJ2UTCG^-70",
    );
    write_float_scientific::<_, BASE32_2_10>(
        0.1172839450617284e-20f64,
        &HEX_OPTIONS,
        "1.C9S0T9HFEM8^-70",
    );
    write_float_scientific::<_, BASE32_2_10>(
        0.0586419725308642e-20f64,
        &HEX_OPTIONS,
        "M.4U0EKONNB4^-75",
    );
    write_float_scientific::<_, BASE32_2_10>(
        0.0293209862654321e-20f64,
        &HEX_OPTIONS,
        "B.2F07ACBRLI^-75",
    );
    write_float_scientific::<_, BASE32_2_10>(
        0.01466049313271605e-20f64,
        &HEX_OPTIONS,
        "5.H7G3L65TQP^-75",
    );

    // Check no formatting, base16/4.
    write_float_scientific::<_, BASE16_4_10>(0.0f64, &HEX_OPTIONS, "0.0^0");
    write_float_scientific::<_, BASE16_4_10>(1.0f64, &HEX_OPTIONS, "1.0^0");
    write_float_scientific::<_, BASE16_4_10>(2.0f64, &HEX_OPTIONS, "2.0^0");
    write_float_scientific::<_, BASE16_4_10>(0.5f64, &HEX_OPTIONS, "8.0^-2");
    write_float_scientific::<_, BASE16_4_10>(
        0.2345678901234567890e20f64,
        &HEX_OPTIONS,
        "1.45874153CC267^32",
    );
    write_float_scientific::<_, BASE16_4_10>(
        0.1172839450617284e20f64,
        &HEX_OPTIONS,
        "A.2C3A0A9E61338^30",
    );
    write_float_scientific::<_, BASE16_4_10>(
        0.0586419725308642e20f64,
        &HEX_OPTIONS,
        "5.161D054F3099C^30",
    );
    write_float_scientific::<_, BASE16_4_10>(
        0.0293209862654321e20f64,
        &HEX_OPTIONS,
        "2.8B0E82A7984CE^30",
    );
    write_float_scientific::<_, BASE16_4_10>(
        0.01466049313271605e20f64,
        &HEX_OPTIONS,
        "1.45874153CC267^30",
    );

    write_float_scientific::<_, BASE16_4_10>(
        0.2345678901234567890e-20f64,
        &HEX_OPTIONS,
        "B.13C075317BAC8^-36",
    );
    write_float_scientific::<_, BASE16_4_10>(
        0.1172839450617284e-20f64,
        &HEX_OPTIONS,
        "5.89E03A98BDD64^-36",
    );
    write_float_scientific::<_, BASE16_4_10>(
        0.0586419725308642e-20f64,
        &HEX_OPTIONS,
        "2.C4F01D4C5EEB2^-36",
    );
    write_float_scientific::<_, BASE16_4_10>(
        0.0293209862654321e-20f64,
        &HEX_OPTIONS,
        "1.62780EA62F759^-36",
    );
    write_float_scientific::<_, BASE16_4_10>(
        0.01466049313271605e-20f64,
        &HEX_OPTIONS,
        "B.13C075317BAC8^-38",
    );

    // Check with a minimum number of digits.
    let options =
        Options::builder().min_significant_digits(num::NonZeroUsize::new(5)).build().unwrap();
    write_float_scientific::<_, BASE16_4_10>(0.0f64, &options, "0.0000e0");
    write_float_scientific::<_, BASE16_4_10>(1.0f64, &options, "1.0000e0");
    write_float_scientific::<_, BASE16_4_10>(2.0f64, &options, "2.0000e0");
    write_float_scientific::<_, BASE16_4_10>(0.5f64, &options, "8.0000e-2");
    write_float_scientific::<_, BASE16_4_10>(
        0.2345678901234567890e2f64,
        &options,
        "1.774F01FED3264e2",
    );

    let options = Options::builder()
        .min_significant_digits(num::NonZeroUsize::new(5))
        .trim_floats(true)
        .build()
        .unwrap();
    write_float_scientific::<_, BASE16_4_10>(0.0f64, &options, "0e0");
    write_float_scientific::<_, BASE16_4_10>(1.0f64, &options, "1e0");
    write_float_scientific::<_, BASE16_4_10>(2.0f64, &options, "2e0");
    write_float_scientific::<_, BASE16_4_10>(0.5f64, &options, "8e-2");
    write_float_scientific::<_, BASE16_4_10>(
        0.2345678901234567890e2f64,
        &options,
        "1.774F01FED3264e2",
    );

    // Check trimming floats
    let options = Options::builder().trim_floats(true).build().unwrap();
    write_float_scientific::<_, BASE16_4_10>(1f32, &options, "1e0");
    write_float_scientific::<_, BASE16_4_10>(1.4e-45f32, &options, "8e-76");
    write_float_scientific::<_, BASE16_4_10>(1.2345678901234567890f32, &options, "1.3C0CA4e0");
}

// NOTE: This doesn't handle float rounding or truncation.
// It assumes this has already been done.
fn write_float<T: Float, const FORMAT: u128>(f: T, options: &Options, expected: &str)
where
    <T as Float>::Unsigned: WriteInteger + FormattedSize,
{
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let count = unsafe { hex::write_float::<_, FORMAT>(f, &mut buffer, options) };
    let actual = unsafe { std::str::from_utf8_unchecked(&buffer[..count]) };
    assert_eq!(actual, expected);
}

#[test]
fn write_float_test() {
    let options = Options::builder().build().unwrap();

    write_float::<_, BASE4_2_10>(
        0.2345678901234567890f64,
        &options,
        "0.033000302210022030112133232",
    );
    write_float::<_, BASE4_2_10>(0.1172839450617284f64, &options, "0.013200121102011012023033313");
    write_float::<_, BASE4_2_10>(0.0586419725308642f64, &options, "0.0033000302210022030112133232");
    write_float::<_, BASE4_2_10>(0.0293209862654321f64, &options, "1.3200121102011012023033313e-6");
    write_float::<_, BASE4_2_10>(
        0.01466049313271605f64,
        &options,
        "3.3000302210022030112133232e-8",
    );
}
