//! Fast, correct parser for the mantissa digits.

use crate::atoi;

use super::format::*;
use crate::float::Mantissa;

// Parse the raw float state into a mantissa, calculating the number
// of truncated digits and the offset.
#[inline(always)]
pub(crate) fn process_mantissa<'a, M, Data>(data: &Data, radix: u32) -> (M, usize)
where
    M: Mantissa,
    Data: FastDataInterface<'a>,
{
    atoi::standalone_mantissa_correct(data.integer_iter(), data.fraction_iter(), radix)
}

// TEST
// ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_mantissa_test() {
        type Data<'a> = StandardFastDataInterface<'a>;
        // 64-bits
        let data = (b!("1"), Some(b!("2345")), None, 0).into();
        assert_eq!((12345, 0), process_mantissa::<u64, Data>(&data, 10));

        let data = (b!("12"), Some(b!("345")), None, 0).into();
        assert_eq!((12345, 0), process_mantissa::<u64, Data>(&data, 10));

        let data = (b!("12345"), Some(b!("6789")), None, 0).into();
        assert_eq!((123456789, 0), process_mantissa::<u64, Data>(&data, 10));

        let data = (b!("1"), Some(b!("2345")), Some(b!("10")), 10).into();
        assert_eq!((12345, 0), process_mantissa::<u64, Data>(&data, 10));

        let data = (b!("100000000000000000000"), None, None, 0).into();
        assert_eq!((10000000000000000000, 1), process_mantissa::<u64, Data>(&data, 10));

        let data = (b!("100000000000000000001"), None, None, 0).into();
        assert_eq!((10000000000000000000, 1), process_mantissa::<u64, Data>(&data, 10));

        let data = (b!("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791"), Some(b!("9999999999999999999999999999999999999999999999999999999999999999999999")), None, 0).into();
        assert_eq!((17976931348623158079, 359), process_mantissa::<u64, Data>(&data, 10));

        let data = (b!("1009"), None, Some(b!("-31")), -31).into();
        assert_eq!((1009, 0), process_mantissa::<u64, Data>(&data, 10));

        // 128-bit
        let data = (b!("1"), Some(b!("2345")), None, 0).into();
        assert_eq!((12345, 0), process_mantissa::<u128, Data>(&data, 10));

        let data = (b!("12"), Some(b!("345")), None, 0).into();
        assert_eq!((12345, 0), process_mantissa::<u128, Data>(&data, 10));

        let data = (b!("12345"), Some(b!("6789")), None, 0).into();
        assert_eq!((123456789, 0), process_mantissa::<u128, Data>(&data, 10));

        let data = (b!("1"), Some(b!("2345")), Some(b!("10")), 10).into();
        assert_eq!((12345, 0), process_mantissa::<u128, Data>(&data, 10));

        let data = (b!("100000000000000000000"), None, None, 0).into();
        assert_eq!((100000000000000000000, 0), process_mantissa::<u128, Data>(&data, 10));

        let data = (b!("100000000000000000001"), None, None, 0).into();
        assert_eq!((100000000000000000001, 0), process_mantissa::<u128, Data>(&data, 10));
    }
}
