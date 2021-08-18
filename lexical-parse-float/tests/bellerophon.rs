#![cfg(any(feature = "compact", feature = "radix"))]
#![allow(dead_code)]

use lexical_parse_float::bellerophon::bellerophon;
use lexical_parse_float::float::{extended_to_float, ExtendedFloat80, RawFloat};
use lexical_parse_float::number::Number;
use lexical_util::format::STANDARD;

pub fn bellerophon_test<F: RawFloat, const FORMAT: u128>(
    xmant: u64,
    xexp: i32,
    many_digits: bool,
    ymant: u64,
    yexp: i32,
) {
    let num = Number {
        exponent: xexp as i64,
        mantissa: xmant,
        is_negative: false,
        many_digits,
        integer: &[],
        fraction: None,
    };
    let xfp = bellerophon::<F, FORMAT>(&num, false);
    let yfp = ExtendedFloat80 {
        mant: ymant,
        exp: yexp,
    };
    // Given us useful error messages if the floats are valid.
    if xfp.exp >= 0 && yfp.exp >= 0 {
        assert!(
            xfp == yfp,
            "x != y, xfp={:?}, yfp={:?}, x={:?}, y={:?}",
            xfp,
            yfp,
            extended_to_float::<F>(xfp),
            extended_to_float::<F>(yfp)
        );
    } else {
        assert_eq!(xfp, yfp);
    }
}

pub fn compute_float32(q: i64, w: u64) -> (i32, u64) {
    let num = Number {
        exponent: q,
        mantissa: w,
        is_negative: false,
        many_digits: false,
        integer: &[],
        fraction: None,
    };
    let fp = bellerophon::<f32, { STANDARD }>(&num, false);
    (fp.exp, fp.mant)
}

pub fn compute_float64(q: i64, w: u64) -> (i32, u64) {
    let num = Number {
        exponent: q,
        mantissa: w,
        is_negative: false,
        many_digits: false,
        integer: &[],
        fraction: None,
    };
    let fp = bellerophon::<f64, { STANDARD }>(&num, false);
    (fp.exp, fp.mant)
}
