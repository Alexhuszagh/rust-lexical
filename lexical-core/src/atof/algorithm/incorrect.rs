//! Incorrect, fast algorithms for string-to-float conversions.

use crate::atoi;
use crate::util::*;
use super::format::*;
use crate::lib::result::Result as StdResult;

// FRACTION

type Wrapped<F> = WrappedFloat<F>;

// Process the integer component of the raw float.
perftools_inline!{
fn process_integer<'a, F, Data>(data: &Data, radix: u32)
    -> F
    where F: StablePower,
          Data: FastDataInterface<'a>
{
    match data.integer().len() {
        0 => F::ZERO,
        // Cannot overflow and cannot have invalid digits.
        _ => atoi::standalone_mantissa::<Wrapped<F>, _>(data.integer_iter(), radix)
                .into_inner()
    }
}}

// Process the fraction component of the raw float.
perftools_inline!{
fn process_fraction<'a, F, Data>(data: &Data, radix: u32)
    -> F
    where F: StablePower,
          Data: FastDataInterface<'a>
{
    // We don't really care about numerical precision, so just break
    // the fraction into 12-digit pieces.
    // 12 is the maximum number of digits we can use without
    // potentially overflowing  a 36-radix float string.
    let mut fraction = F::ZERO;
    let mut digits: i32 = 0;
    let mut iter = data.fraction_iter();
    while !iter.consumed() {
        let (value, length) = atoi::standalone_mantissa_n::<u64, _>(&mut iter, radix, 12);
        digits = digits.saturating_add(length.as_i32());
        if !value.is_zero() {
            fraction += F::iterative_pow(as_cast(value), radix, -digits);
        }
    }

    fraction
}}

// Convert the float string to a native floating-point number.
perftools_inline!{
fn to_native<F: StablePower>(bytes: &[u8], radix: u32)
    -> ParseResult<(F, *const u8)>
{
    let mut data = StandardFastDataInterface::new(0);
    let ptr = data.extract(bytes, radix)?;

    let integer: F = process_integer(&data, radix);
    let fraction: F = process_fraction(&data, radix);
    let mut value = integer + fraction;
    if !data.raw_exponent().is_zero() && !value.is_zero() {
        value = value.iterative_pow(radix, data.raw_exponent());
    }
    Ok((value, ptr))
}}

// ATOF/ATOD
// ---------

// Parse 32-bit float from string.
perftools_inline!{
pub(crate) fn atof<'a>(bytes: &'a [u8], radix: u32, _: Sign)
    -> StdResult<(f32, *const u8), (ErrorCode, *const u8)>
{
    to_native::<f32>(bytes, radix)
}}

// Parse 64-bit float from string.
perftools_inline!{
pub(crate) fn atod<'a>(bytes: &'a [u8], radix: u32, _: Sign)
    -> StdResult<(f64, *const u8), (ErrorCode, *const u8)>
{
    to_native::<f64>(bytes, radix)
}}

// Parse 32-bit float from string.
perftools_inline!{
pub(crate) fn atof_lossy<'a>(bytes: &'a [u8], radix: u32, _: Sign)
    -> StdResult<(f32, *const u8), (ErrorCode, *const u8)>
{
    to_native::<f32>(bytes, radix)
}}

// Parse 64-bit float from string.
perftools_inline!{
pub(crate) fn atod_lossy<'a>(bytes: &'a [u8], radix: u32, _: Sign)
    -> StdResult<(f64, *const u8), (ErrorCode, *const u8)>
{
    to_native::<f64>(bytes, radix)
}}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_integer_test() {
        type Data<'a> = StandardFastDataInterface<'a>;

        let data = (b!("1"), b!("2345"), b!(""), 0).into();
        assert_eq!(1.0, process_integer::<f64, Data>(&data, 10));

        let data = (b!("12"), b!("345"), b!(""), 0).into();
        assert_eq!(12.0, process_integer::<f64, Data>(&data, 10));

        let data = (b!("12345"), b!("6789"), b!(""), 0).into();
        assert_eq!(12345.0, process_integer::<f64, Data>(&data, 10));
    }

    #[test]
    fn process_fraction_test() {
        type Data<'a> = StandardFastDataInterface<'a>;

        let data = (b!("1"), b!("2345"), b!(""), 0).into();
        assert_eq!(0.2345, process_fraction::<f64, Data>(&data, 10));

        let data = (b!("12"), b!("345"), b!(""), 0).into();
        assert_eq!(0.345, process_fraction::<f64, Data>(&data, 10));

        let data = (b!("12345"), b!("6789"), b!(""), 0).into();
        assert_eq!(0.6789, process_fraction::<f64, Data>(&data, 10));
    }

    #[test]
    fn atof_test() {
        let atof10 = move |x| match atof(x, 10, Sign::Positive) {
            Ok((v, p))  => Ok((v, distance(x.as_ptr(), p))),
            Err((v, p)) => Err((v, distance(x.as_ptr(), p))),
        };

        assert_eq!(Ok((1.2345, 6)), atof10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atof10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atof10(b"12345.6789"));
        assert_f32_eq!(1.2345e10, atof10(b"1.2345e10").unwrap().0);
    }

    #[test]
    fn atod_test() {
        let atod10 = move |x| match atod(x, 10, Sign::Positive) {
            Ok((v, p))  => Ok((v, distance(x.as_ptr(), p))),
            Err((v, p)) => Err((v, distance(x.as_ptr(), p))),
        };

        assert_eq!(Ok((1.2345, 6)), atod10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atod10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atod10(b"12345.6789"));
        assert_f64_eq!(1.2345e10, atod10(b"1.2345e10").unwrap().0);
    }

    // Lossy
    // Just a synonym for the regular overloads, since we're not using the
    // correct feature. Use the same tests.

    #[test]
    fn atof_lossy_test() {
        let atof10 = move |x| match atof_lossy(x, 10, Sign::Positive) {
            Ok((v, p))  => Ok((v, distance(x.as_ptr(), p))),
            Err((v, p)) => Err((v, distance(x.as_ptr(), p))),
        };

        assert_eq!(Ok((1.2345, 6)), atof10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atof10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atof10(b"12345.6789"));
        assert_f32_eq!(1.2345e10, atof10(b"1.2345e10").unwrap().0);
    }

    #[test]
    fn atod_lossy_test() {
        let atod10 = move |x| match atod_lossy(x, 10, Sign::Positive) {
            Ok((v, p))  => Ok((v, distance(x.as_ptr(), p))),
            Err((v, p)) => Err((v, distance(x.as_ptr(), p))),
        };

        assert_eq!(Ok((1.2345, 6)), atod10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atod10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atod10(b"12345.6789"));
        assert_f64_eq!(1.2345e10, atod10(b"1.2345e10").unwrap().0);
    }
}
