// Copyright 2020 Junekey Jeon
// Modified 2021 Alex Huszagh
// Distributed under an Apache2.0/Boost license.

use lexical_util::num::as_cast;
use lexical_write_float::float::RawFloat;
use lexical_write_float::{BUFFER_SIZE, ToLexical};

fn shorter_interval_test<F>() -> Result<(), String>
where
    F: RawFloat + ToLexical + std::str::FromStr + std::string::ToString,
{
    let mut buffer = [b'0'; BUFFER_SIZE];
    for exponent in F::DENORMAL_EXPONENT..=F::MAX_EXPONENT {
        let biased_exp: F::Unsigned = as_cast(exponent - F::EXPONENT_BIAS);
        let float = F::from_bits(biased_exp << F::MANTISSA_SIZE);
        let string = unsafe { std::str::from_utf8_unchecked(float.to_lexical(&mut buffer)) };
        let roundtrip = string.parse::<F>().map_err(|_| float.to_string())?;
        if roundtrip != float {
            return Err(float.to_string())
        }
    }
    Ok(())
}

pub fn main() -> Result<(), String> {
    shorter_interval_test::<f32>()?;
    shorter_interval_test::<f64>()?;
    Ok(())
}
