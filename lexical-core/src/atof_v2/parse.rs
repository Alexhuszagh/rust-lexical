//! Utilities to parse a number.

use crate::util::*;

use super::binary::*;
use super::common::*;
use super::float::*;
use super::number::*;
use super::options::*;
use super::simple::*;

#[inline]
fn parse_iter<'a, IterDigits, IterSpec, F>(bytes: &'a [u8], options: &Options)
    -> ParseResult<(F, usize)>
where
    F: FloatType,
    IterDigits: ContiguousIterator<'a, u8>,
    IterSpec: ContiguousIterator<'a, u8>,
{
    // Helpers to convert the bytearray to digits.
    let to_iter = |b| IterDigits::new(b, options.digit_separator);
    let to_iter_spec = |b| IterSpec::new(b, options.digit_separator);

    if to_iter(bytes).empty() {
        return Err((ParseErrorCode::Empty, 0).into());
    }
    let mut num = NumberState::<F>::default();
    let rest = match parse_number::<F, _>(to_iter(bytes), &mut num, options) {
        Some(r) => r,
        None => return parse_inf_nan(to_iter_spec(bytes), options),
    };

    //  Validation
    //      Validate the float format is valid for the given input.
    if cfg!(not(feature = "format")) || options.required_exponent_digits {
        if num.exponential.is_some() && to_iter(num.exponential.unwrap()).empty() {
            let index = distance(bytes.as_ptr(), num.exponential.unwrap().as_ptr());
            let has_sign = num.has_exponent_sign as usize;
            return Err((ParseErrorCode::EmptyExponent, index + has_sign).into());
        }
    }
    if cfg!(feature = "format") && options.required_integer_digits {
        let integral = num.integral.unwrap();
        if to_iter(integral).empty() {
            let index = distance(bytes.as_ptr(), integral.as_ptr());
            return Err((ParseErrorCode::EmptyInteger, index).into());
        }
    }
    if cfg!(feature = "format") && options.required_fraction_digits {
        if num.fractional.is_some() && to_iter(num.fractional.unwrap()).empty() {
            let index = distance(bytes.as_ptr(), num.fractional.unwrap().as_ptr());
            return Err((ParseErrorCode::EmptyFraction, index).into());
        }
    }
    if cfg!(feature = "format") && options.no_positive_mantissa_sign {
        if num.has_mantissa_sign && !num.negative {
            return Err((ParseErrorCode::InvalidPositiveMantissaSign, 0).into());
        }
    }
    if cfg!(feature = "format") && options.required_mantissa_sign {
        if !num.has_mantissa_sign {
            return Err((ParseErrorCode::MissingMantissaSign, 0).into());
        }
    }
    if cfg!(feature = "format") && options.no_exponent_notation {
        if num.exponential.is_some() {
            let index = distance(bytes.as_ptr(), num.exponential.unwrap().as_ptr());
            return Err((ParseErrorCode::InvalidExponent, index - 1).into());
        }
    }
    if cfg!(feature = "format") && options.no_positive_exponent_sign {
        if num.has_exponent_sign && num.exponent >= 0 {
            let index = distance(bytes.as_ptr(), num.exponential.unwrap().as_ptr());
            return Err((ParseErrorCode::InvalidPositiveExponentSign, index).into());
        }
    }
    if cfg!(feature = "format") && options.required_exponent_sign {
        if num.exponential.is_some() && !num.has_exponent_sign {
            let index = distance(bytes.as_ptr(), num.exponential.unwrap().as_ptr());
            return Err((ParseErrorCode::MissingExponentSign, index).into());
        }
    }
    if cfg!(feature = "format") && options.no_exponent_without_fraction {
        if num.fractional.is_none() && num.exponential.is_some() {
            let index = distance(bytes.as_ptr(), num.exponential.unwrap().as_ptr());
            return Err((ParseErrorCode::ExponentWithoutFraction, index - 1).into());
        }
    }
    if cfg!(feature = "format") && options.no_float_leading_zeros {
        let integral = num.integral.unwrap();
        let mut iter = to_iter(integral);
        if iter.peek() == Some(&b'0') && iter.count() >= 2 {
            let index = distance(bytes.as_ptr(), integral.as_ptr());
            return Err((ParseErrorCode::InvalidLeadingZeros, index).into());
        }
    }
    if cfg!(feature = "format") && options.required_exponent_notation {
        if num.exponential.is_none() {
            return Err((ParseErrorCode::MissingExponent, rest).into());
        }
    }

    // Try the fast path.
    if let Some(value) = num.try_fast_path(options) {
        return Ok((value, rest));
    }

    // Try the moderate path.
    // This should also only work if F::BITS <= 64
    // TODO(ahuszagh) Should just take num??
    let mut am: AdjustedMantissa<F>;
    if cfg!(feature = "lemire") && F::Mantissa::BITS <= 64 {
        let mant_1 = num.mantissa + F::Mantissa::ONE;
        am = compute_float::<F>(num.exponent, num.mantissa, options);
        if num.many_digits && am != compute_float::<F>(num.exponent, mant_1, options) {
            am.power2 = -1;
        }
    } else {
        // TODO(ahuszagh) Need to implement properly using Bellepheron.
        am = AdjustedMantissa::default();
        todo!();
    }
    // Use the slow path.
    if am.power2 < 0 {
        // TODO(ahuszagh) Should use Number on this with the iter type.
        am = parse_long_mantissa::<F, _>(to_iter(bytes), options);
    }

    let mut word = am.mantissa;
    word |= as_cast::<F::Mantissa, _>(am.power2) << F::MANTISSA_SIZE;
    if num.negative {
        word |= as_cast(F::SIGN_MASK);
    }

    Ok((F::from_mantissa_bits(word), rest))
}

#[inline]
pub fn parse_float<F>(bytes: &[u8], options: &Options)
    -> ParseResult<(F, usize)>
where
    F: FloatType,
{
    #[cfg(not(feature = "format"))]
    {
        return parse_iter::<IterN, IterN, _>(bytes, options);
    }

    #[cfg(feature = "format")]
    {
        let digits_s = options.digit_digit_separator;
        let special_s = options.special_digit_separator;
        return match (digits_s, special_s) {
            (false, false) => parse_iter::<IterN, IterN, _>(bytes, options),
            (true, false) => parse_iter::<IterS, IterN, _>(bytes, options),
            (false, true) => parse_iter::<IterN, IterS, _>(bytes, options),
            (true, true) => parse_iter::<IterS, IterS, _>(bytes, options),
        };
    }
}


#[cfg(test)]
mod tests {
    // TODO(ahuszagh) Need to fix this shit.
    use super::*;
    use super::super::options::*;
    use super::super::table::*;
    // TODO(ahuszagh) Add initial tests

    // TODO(ahuszagh) Remove and refactor.
    // TODO(ahuszagh) Need to calculate these values for other bases.
    const OPTIONS: Options = Options {
        digit_separator: b'\x00',
        decimal_point: b'.',
        exponent: b'e',
        mantissa_radix: 10,
        exponent_base: 10,
        exponent_radix: 10,
        nan_string: b"NaN",
        inf_string: b"inf",
        infinity_string: b"infinity",
        required_integer_digits: false,
        required_fraction_digits: false,
        required_exponent_digits: false,
        no_positive_mantissa_sign: false,
        required_mantissa_sign: false,
        no_exponent_notation: false,
        no_positive_exponent_sign: false,
        required_exponent_sign: false,
        no_exponent_without_fraction: false,
        no_special: false,
        case_sensitive_special: false,
        case_sensitive_exponent: false,
        no_integer_leading_zeros: false,
        no_float_leading_zeros: false,
        required_exponent_notation: false,
        integer_internal_digit_separator: false,
        fraction_internal_digit_separator: false,
        exponent_internal_digit_separator: false,
        internal_digit_separator: false,
        integer_leading_digit_separator: false,
        fraction_leading_digit_separator: false,
        exponent_leading_digit_separator: false,
        leading_digit_separator: false,
        integer_trailing_digit_separator: false,
        fraction_trailing_digit_separator: false,
        exponent_trailing_digit_separator: false,
        trailing_digit_separator: false,
        integer_consecutive_digit_separator: false,
        fraction_consecutive_digit_separator: false,
        exponent_consecutive_digit_separator: false,
        consecutive_digit_separator: false,
        digit_digit_separator: false,
        special_digit_separator: false,
        max_digits_mantissa: 19,
        f32_min_digit_int: 10000000000000000000,
        f64_min_digit_int: 10000000000000000000,
        f32_max_mantissa_fast: 2u64<<23,
        f64_max_mantissa_fast: 2u64<<52,
        f32_fast_pow: &BASE10_F32_POWERS,
        f64_fast_pow: &BASE10_F64_POWERS,
        f32_int_pow: &BASE10_INT_POWERS,
        f64_int_pow: &BASE10_INT_POWERS,
        log2_power: 152_170,
        log2_power_shift: 16,
        f32_exp_limit_min: -10,
        f64_exp_limit_min: -22,
        f32_exp_limit_max: 10,
        f64_exp_limit_max: 22,
        f32_mantissa_limit: 7,
        f64_mantissa_limit: 15,
        smallest_power: BASE10_MIN_EXP,
        largest_power: BASE10_MAX_EXP,
        bias: BASE10_BIAS,
        f32_min_exponent_round_to_even: -17,
        f64_min_exponent_round_to_even: -4,
        f32_max_exponent_round_to_even: 10,
        f64_max_exponent_round_to_even: 23,
        f32_min_exp: -342,
        f64_min_exp: 308,
        f32_max_exp: -342,
        f64_max_exp: 308,
        power_of_x_128: &BASE10_POWERS,
        min_safe_exponent: -27,
        max_safe_exponent: 55,
        // TODO(ahuszagh) Here...
    };

    #[test]
    fn test_fast_path() {
        assert_eq!(parse_float::<f64>(b"1.2345", &OPTIONS), Ok((1.2345, 6)));
        assert_eq!(parse_float::<f64>(b"1.2345e32", &OPTIONS), Ok((1.2345e32, 9)));
        // TODO(ahuszagh) Restore later...
        //assert_eq!(parse_float::<f64>(b"1.2345e", &OPTIONS), Ok((1.2345, 7)));
        //assert_eq!(parse_float::<f64>(b"1.2345e+", &OPTIONS), Ok((1.2345, 8)));
        //assert_eq!(parse_float::<f64>(b".e", &OPTIONS), Ok((0.0, 2)));
        //assert_eq!(parse_float::<f64>(b"e", &OPTIONS), Ok((0.0, 1)));
    }

    // TODO(ahuszagh) Need to add cases for moderate paths, etc....
}
