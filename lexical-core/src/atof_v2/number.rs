
use crate::util::*;
use super::float::*;
use super::options::*;
use super::starts::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NumberState<'a, F: FloatType> {
    pub exponent: i64,
    pub mantissa: F::Mantissa,
    pub negative: bool,
    pub many_digits: bool,
    pub integral: Option<&'a[u8]>,
    pub fractional: Option<&'a[u8]>,
    pub exponential: Option<&'a[u8]>,
    pub has_mantissa_sign: bool,
    pub has_exponent_sign: bool,
}

impl<'a,F: FloatType> NumberState<'a, F> {
    #[inline]
    fn is_fast_path(&self, options: &Options) -> bool {
        F::exp_limit_min(options) <= self.exponent
            && self.exponent <= F::exp_limit_max(options) + F::mantissa_limit(options)
            && self.mantissa <= F::max_mantissa_fast(options)
            && !self.many_digits
    }

    #[inline]
    pub fn try_fast_path(&self, options: &Options) -> Option<F>
    {
        let is_fast = self.is_fast_path(options);
        if is_fast && self.exponent <= F::exp_limit_max(options) {
            let mut value: F = as_cast(self.mantissa);
            if self.exponent < 0 {
                value = value / F::fast_pow((-self.exponent) as _, options);
            } else {
                value = value * F::fast_pow(self.exponent as _, options);
            }
            if self.negative {
                value = -value;
            }
            Some(value)
        } else if is_fast {
            let max_exp = F::exp_limit_max(options);
            let shift = self.exponent - max_exp;
            let mant = self.mantissa.checked_mul(F::int_pow(shift as usize, options))?;
            if mant > F::max_mantissa_fast(options) {
                return None;
            }
            let mut value: F = as_cast(mant);
            value = value * F::fast_pow(max_exp as _, options);
            if self.negative {
                value = -value;
            }
            Some(value)
        } else {
            None
        }
    }
}

#[inline]
fn parse_digits<'a, T, Iter>(iter: &mut Iter, mant: &mut T, radix: u32, cond: impl Fn(T) -> bool)
    -> usize
where
    T: Integer,
    Iter: ContiguousIterator<'a, u8>,
{
    let base: T = as_cast(radix);
    let mut count = 0;
    while let Some(&c) = iter.peek() {
        if let Some(digit) = to_digit(c, radix) {
            iter.next();
            count += 1;
            if cond(*mant) {
                *mant = mant.wrapping_mul(base).wrapping_add(as_cast(digit));
            }
        } else {
            break;
        }
    }

    count
}

#[inline]
fn parse_xdigits<'a, T, Iter>(iter: &mut Iter, mant: &mut T, max: T, radix: u32) -> usize
where
    T: Integer,
    Iter: ContiguousIterator<'a, u8>,
{
    let mut count = 0;
    while *mant < max {
        // Can't have this panic, since we parsed too many digits before.
        let c = iter.peek().unwrap();
        if let Some(digit) = to_digit(*c, radix) {
            iter.next();
            count += 1;
            *mant = *mant * as_cast(radix) + as_cast(digit);
        } else {
            break;
        }
    }

    count
}

#[inline]
fn parse_scientific<'a, F, Iter>(iter: &mut Iter, num: &mut NumberState<'a, F>, radix: u32)
where
    F: FloatType,
    Iter: ContiguousIterator<'a, u8>,
{
    let mut negative = false;
    let peek = iter.peek();
    if let Some(b'+') = peek {
        iter.next();
        num.has_exponent_sign = true;
    } else if let Some(b'-') = peek {
        iter.next();
        negative = true;
        num.has_exponent_sign = true;
    }

    // Can't overflow, limit to 2^16.
    let mut exponent = 0i64;
    parse_digits(iter, &mut exponent, radix, |x| x < 0x10000);
    if negative {
        num.exponent -= exponent;
    } else {
        num.exponent += exponent;
    }
}

// TODO(ahuszagh) Restore?? Hey?
//  AOA
//  ALso have parse4digits_le
//#[inline]
//fn parse_8digits_le(mut v: u64) -> u64 {
//    const MASK: u64 = 0x0000_00FF_0000_00FF;
//    const MUL1: u64 = 0x000F_4240_0000_0064;
//    const MUL2: u64 = 0x0000_2710_0000_0001;
//    v -= 0x3030_3030_3030_3030;
//    v = (v * 10) + (v >> 8); // will not overflow, fits in 63 bits
//    let v1 = (v & MASK).wrapping_mul(MUL1);
//    let v2 = ((v >> 16) & MASK).wrapping_mul(MUL2);
//    ((v1.wrapping_add(v2) >> 32) as u32) as u64
//}

// TODO(ahuszagh) Is this actually??? Dependent on little-endian?
//  I think the masks are but...
//#[inline]
//fn try_parse_8digits_le(s: &mut AsciiStr<'_>, x: &mut u64) {
//    // may cause overflows, to be handled later
//    if cfg!(target_endian = "little") {
//        if let Some(v) = s.try_read_u64() {
//            if is_8digits_le(v) {
//                *x = x
//                    .wrapping_mul(1_0000_0000)
//                    .wrapping_add(parse_8digits_le(v));
//                s.step_by(8);
//                if let Some(v) = s.try_read_u64() {
//                    if is_8digits_le(v) {
//                        *x = x
//                            .wrapping_mul(1_0000_0000)
//                            .wrapping_add(parse_8digits_le(v));
//                        s.step_by(8);
//                    }
//                }
//            }
//        }
//    }
//}

#[inline]
pub(crate) fn parse_number<'a, F, Iter>(mut iter: Iter, num: &mut NumberState<'a, F>, options: &Options)
    -> Option<usize>
where
    F: FloatType,
    Iter: ContiguousIterator<'a, u8>,
{
    // Check our preconditions.
    debug_assert!(
        options.mantissa_radix == options.exponent_base,
        "Only support different radixes and exponent bases with powers-of-two"
    );

    // Parse the sign.
    let peek = iter.peek();
    if let Some(b'+') = peek {
        iter.next();
        num.has_mantissa_sign = true;
    } else if let Some(b'-') = peek {
        iter.next();
        num.negative = true;
        num.has_mantissa_sign = true;
    }

    // Parse the significant digits before the decimal point.
    let start = iter.clone();
    let mut digits = iter.clone();
    let radix = options.mantissa_radix as u32;
    let mut mantissa = F::Mantissa::ZERO;
    let integral_count = parse_digits(&mut digits, &mut mantissa, radix, |_| true);
    let len = start.slice_length() - digits.slice_length();
    num.integral = Some(&start.as_slice()[..len]);
    let mut n_digits = integral_count;

    // Parse the significant digits after the decimal point.
    let peek = digits.peek();
    if peek == Some(&options.decimal_point) {
        digits.next();
        let start = digits.clone();
        // TODO(ahuszagh) Should have try_read_u64 and try_read_u32 here.
        // It should honestly have an option to store the pointer if it is read.
        let fractional_count = parse_digits(&mut digits, &mut mantissa, radix, |_| true);
        let len = start.slice_length() - digits.slice_length();
        n_digits += fractional_count;
        num.fractional = Some(&start.as_slice()[..len]);
        num.exponent = -(fractional_count as i64);
    }

    // Parse exponent notation.
    let peek = digits.peek();
    let has_exponent = if cfg!(feature = "format") && options.case_sensitive_exponent {
        peek == Some(&options.exponent)
    } else {
        let peek = peek.map(|c| c.to_ascii_lowercase());
        let exp = options.exponent.to_ascii_lowercase();
        peek == Some(exp)
    };
    if has_exponent {
        digits.next();
        let start = digits.clone();
        let radix = options.exponent_radix as u32;
        parse_scientific(&mut digits, num, radix);
        let len = start.slice_length() - digits.slice_length();
        num.exponential = Some(&start.as_slice()[..len]);
    }
    let len = start.slice_length() - digits.slice_length();

    // Return early if we have not many digits.
    if n_digits <= options.max_digits_mantissa {
        num.mantissa = mantissa;
        return Some(len);
    }

    // Try to handle leading zeros.
    n_digits -= options.max_digits_mantissa;
    let mut digits = iter.clone();
    while let Some(&c) = digits.next() {
        if c == b'0' {
            n_digits -= 1;
        } else if c != options.decimal_point {
            break;
        }
    }

    // Have more than max digits digits, need to re-parse.
    if n_digits > 0 {
        let mut digits = iter.clone();
        num.many_digits = true;
        mantissa = F::Mantissa::ZERO;
        let count = parse_xdigits(&mut digits, &mut mantissa, F::min_digit_int(options), radix);
        if mantissa >= F::min_digit_int(options) {
            // Too many digits to parse, need to adjust
            num.exponent += (integral_count - count) as i64;
        } else {
            digits.next();
            let count = parse_xdigits(&mut digits, &mut mantissa, F::min_digit_int(options), radix);
            num.exponent -= count as i64;
        }
    }

    num.mantissa = mantissa;
    Some(len)
}

#[inline]
fn parse_inf_nan_starts_with<'a, StartsWith, F, Iter>(mut iter: Iter, options: &Options)
    -> ParseResult<(F, usize)>
where
    F: Float,
    Iter: ContiguousIterator<'a, u8>,
    StartsWith: Starts,
{
    let count = iter.clone().count();
    let mut negative = false;
    let peek = iter.peek();
    if let Some(b'+') = peek {
        iter.next();
    } else if let Some(b'-') = peek {
        iter.next();
        negative = true;
    }

    let (mut float, mut iter) = if let (true, iter) = StartsWith::with(iter.clone(), to_iter_n(options.nan_string, b'\x00')) {
        (F::NAN, iter)
    } else if let (true, iter) = StartsWith::with(iter.clone(), to_iter_n(options.infinity_string, b'\x00')) {
        (F::INFINITY, iter)
    } else if let (true, iter) = StartsWith::with(iter.clone(), to_iter_n(options.inf_string, b'\x00')) {
        (F::INFINITY, iter)
    } else {
        // No significant digits found
        return Err((ParseErrorCode::EmptyMantissa, 0).into());
    };
    iter.trim();
    if negative {
        float = -float;
    }
    Ok((float, count - iter.count()))
}

#[inline]
pub(crate) fn parse_inf_nan<'a, F, Iter>(iter: Iter, options: &Options)
    -> ParseResult<(F, usize)>
where
    F: Float,
    Iter: ContiguousIterator<'a, u8>,
    StartsWith: Starts,
{
    if cfg!(feature = "format") && options.case_sensitive_special {
        parse_inf_nan_starts_with::<LowercaseStartsWith, _, _>(iter, options)
    } else {
        parse_inf_nan_starts_with::<StartsWith, _, _>(iter, options)
    }
}
