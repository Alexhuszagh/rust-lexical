//! Incorrect, fast algorithms for string-to-float conversions.

use crate::atoi;
use crate::util::*;
use super::alias::*;
use super::format::*;

// FRACTION

type Wrapped<F> = WrappedFloat<F>;

/// Process the integer component of the raw float.
#[inline(always)]
fn process_integer<'a, F, Data>(data: &Data, radix: u32)
    -> F
    where F: FloatType,
          Data: FastDataInterface<'a>
{
    match data.integer().len() {
        0 => F::ZERO,
        // Cannot overflow and cannot have invalid digits.
        _ => atoi::standalone_mantissa_incorrect::<Wrapped<F>, _>(data.integer_iter(), radix)
                .into_inner()
    }
}

/// Process the fraction component of the raw float.
#[inline(always)]
fn process_fraction<'a, F, Data>(data: &Data, radix: u32)
    -> F
    where F: FloatType,
          Data: FastDataInterface<'a>
{
    // We don't really care about numerical precision, so just break
    // the fraction into 12-digit pieces.
    // 12 is the maximum number of digits we can use without
    // potentially overflowing  a 36-radix float string.
    // We also have a fast, short-circuiting algorithm here:
    // After we've seen the number of digits required to guaranteed
    // we've done a full significand (excluding rounding), we can
    // then short-circuit. We need to do `2*max_digits - 1`,
    // since we might have lead with `max_digits - 1` 0 digits,
    // and only the last one was non-zero.
    let mut fraction = F::ZERO;
    let mut digits: i32 = 0;
    let mut nonzero_digits: i32 = 0;
    let max_digits: i32 = 2 * F::max_incorrect_digits(radix) - 1;
    let mut iter = data.fraction_iter();
    while !iter.consumed() && nonzero_digits <= max_digits {
        let (value, length) = atoi::standalone_mantissa_incorrect_n::<u64, _>(&mut iter, radix, 12);
        digits = digits.saturating_add(length.as_i32());
        if !value.is_zero() {
            nonzero_digits = nonzero_digits.saturating_add(length.as_i32());
            fraction += F::iterative_pow(as_cast(value), radix, -digits);
        }
    }

    fraction
}

pub(crate) fn to_native<'a, F, Data>(data: Data, radix: u32) -> F
    where F: FloatType,
          Data: FastDataInterface<'a>
{
    let integer: F = process_integer(&data, radix);
    let fraction: F = process_fraction(&data, radix);
    let mut value = integer + fraction;
    if !data.raw_exponent().is_zero() && !value.is_zero() {
        value = value.iterative_pow(radix, data.raw_exponent());
    }
    value
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use crate::util::*;
    use super::*;

    #[test]
    fn process_integer_test() {
        type Data<'a> = StandardFastDataInterface<'a>;

        let data = (b!("1"), Some(b!("2345")), None, 0).into();
        assert_eq!(1.0, process_integer::<f64, Data>(&data, 10));

        let data = (b!("12"), Some(b!("345")), None, 0).into();
        assert_eq!(12.0, process_integer::<f64, Data>(&data, 10));

        let data = (b!("12345"), Some(b!("6789")), None, 0).into();
        assert_eq!(12345.0, process_integer::<f64, Data>(&data, 10));
    }

    #[test]
    fn process_fraction_test() {
        type Data<'a> = StandardFastDataInterface<'a>;

        let data = (b!("1"), Some(b!("2345")), None, 0).into();
        assert_eq!(0.2345, process_fraction::<f64, Data>(&data, 10));

        let data = (b!("12"), Some(b!("345")), None, 0).into();
        assert_eq!(0.345, process_fraction::<f64, Data>(&data, 10));

        let data = (b!("12345"), Some(b!("6789")), None, 0).into();
        assert_eq!(0.6789, process_fraction::<f64, Data>(&data, 10));
    }

    #[test]
    fn atof_test() {
        let options = ParseFloatOptions::builder()
            .incorrect(true)
            .build()
            .unwrap();
        let atof10 = move |x| f32::from_lexical_partial_with_options(x, &options);

        assert_eq!(Ok((1.2345, 6)), atof10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atof10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atof10(b"12345.6789"));
        assert_f32_near_eq!(1.2345e10, atof10(b"1.2345e10").unwrap().0);
    }

    #[test]
    fn atod_test() {
        let options = ParseFloatOptions::builder()
            .incorrect(true)
            .build()
            .unwrap();
        let atod10 = move |x| f64::from_lexical_partial_with_options(x, &options);

        assert_eq!(Ok((1.2345, 6)), atod10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atod10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atod10(b"12345.6789"));
        assert_f64_near_eq!(1.2345e10, atod10(b"1.2345e10").unwrap().0);
    }
}
