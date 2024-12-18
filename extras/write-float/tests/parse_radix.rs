#![cfg(feature = "power-of-two")]

use fraction::{BigFraction, ToPrimitive};
use lexical_util::step::u64_step;

// NOTE:
//  These are extremely naive, inefficient binary parsers based off
//  of the documentation in `binary.rs`. See the Python code there
//  for a more legible example.

macro_rules! parse_float {
    ($name:ident, $t:ident, $cb:ident) => {
        pub fn $name(string: &[u8], radix: u32, exp: u8) -> $t {
            let index = string.iter().position(|&x| x == b'.').unwrap();
            let integer = &string[..index];
            let rest = &string[index + 1..];
            let fraction: &[u8];
            let exponent: i32;
            if let Some(index) = rest.iter().position(|&x| x == exp) {
                fraction = &rest[..index];
                let exp_digits = unsafe { std::str::from_utf8_unchecked(&rest[index + 1..]) };
                exponent = i32::from_str_radix(exp_digits, radix).unwrap();
            } else {
                fraction = rest;
                exponent = 0;
            }

            // Now need to reconstruct our integer.
            let step = u64_step(radix);
            let pow = BigFraction::new((radix as u128).pow(step as u32), 1u64);
            let mut fint = BigFraction::new(0u64, 1u64);
            let mut index = 0;
            while index < integer.len() {
                let count = step.min(integer.len() - index);
                let end = index + count;
                let digits = unsafe { std::str::from_utf8_unchecked(&integer[index..end]) };
                let tmp = u64::from_str_radix(digits, radix).unwrap();
                fint *= pow.clone();
                fint += BigFraction::new(tmp, 1u64);
                index = end;
            }

            // Scale it to the exponent.
            // Note that these should always be exact, since we can hold
            // all powers-of-two exactly.
            if exponent >= 0 {
                fint *= BigFraction::from((radix as f64).powi(exponent));
            } else {
                fint /= BigFraction::from((radix as f64).powi(-exponent));
            }

            // Now need to reconstruct our fraction.
            let mut ffrac = BigFraction::new(0u64, 1u64);
            let mut index = 0;
            while index < fraction.len() {
                let count = step.min(fraction.len() - index);
                let end = index + count;
                let digits = unsafe { std::str::from_utf8_unchecked(&fraction[index..end]) };
                let tmp = u64::from_str_radix(digits, radix).unwrap();
                ffrac *= pow.clone();
                ffrac += BigFraction::new(tmp, 1u64);
                index = end;
            }

            let exp_shift = fraction.len() as i32 - exponent;
            let ffrac_exp_num;
            let ffrac_exp_den;
            if exp_shift > 0 {
                ffrac_exp_num = 0;
                ffrac_exp_den = exp_shift;
            } else {
                ffrac_exp_num = -exp_shift;
                ffrac_exp_den = 0;
            }

            ffrac *= BigFraction::from((radix as f64).powi(ffrac_exp_num));
            ffrac /= BigFraction::from((radix as f64).powi(ffrac_exp_den));

            (fint + ffrac).$cb().unwrap()
        }
    };
}

parse_float!(parse_f32, f32, to_f32);
parse_float!(parse_f64, f64, to_f64);
