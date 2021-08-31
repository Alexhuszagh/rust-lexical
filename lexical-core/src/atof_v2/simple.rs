use crate::util::*;

use super::common::*;
use super::decimal::*;
use super::float::*;
use super::options::*;

#[inline]
pub(crate) fn parse_long_mantissa<'a, F, Iter>(mut iter: Iter, options: &Options)
    -> AdjustedMantissa<F>
where
    F: FloatType,
    Iter: ContiguousIterator<'a, u8>,
{
    let am_zero = AdjustedMantissa::zero_pow2(0);
    let am_inf = AdjustedMantissa::zero_pow2(F::MAX_EXPONENT + F::EXPONENT_BIAS);

    // TODO(ahuszagh) Can simplify this a lot...
    // Since we can use ::new() on the iterator.
    let mut d = parse_decimal(iter);
    // TODO(ahuszagh) This needs to be the minimal exponent
    if d.num_digits == 0 || d.decimal_point < F::min_exp(options) {
        return am_zero;
    } else if d.decimal_point >= 310 {
        return am_inf;
    }
    // TODO(ahuszagh) Add more...
    todo!()
}
