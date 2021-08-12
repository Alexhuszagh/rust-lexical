#![cfg(any(feature = "compact", feature = "radix"))]

use lexical_parse_float::bellerophon::bellerophon;
use lexical_parse_float::float::{extended_to_float, ExtendedFloat80, RawFloat};
use lexical_parse_float::number::Number;
use lexical_util::format::STANDARD;

fn bellerophon_test<F: RawFloat, const FORMAT: u128>(
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
    };
    let xfp = bellerophon::<F, FORMAT>(&num);
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

#[cfg(feature = "compact")]
fn compute_float32(q: i64, w: u64) -> (i32, u64) {
    let num = Number {
        exponent: q,
        mantissa: w,
        is_negative: false,
        many_digits: false,
    };
    let fp = bellerophon::<f32, { STANDARD }>(&num);
    (fp.exp, fp.mant)
}

#[cfg(feature = "compact")]
fn compute_float64(q: i64, w: u64) -> (i32, u64) {
    let num = Number {
        exponent: q,
        mantissa: w,
        is_negative: false,
        many_digits: false,
    };
    let fp = bellerophon::<f64, { STANDARD }>(&num);
    (fp.exp, fp.mant)
}

#[test]
#[cfg(feature = "compact")]
fn halfway_round_down_test() {
    // Halfway, round-down tests
    bellerophon_test::<f64, { STANDARD }>(9007199254740992, 0, false, 0, 1076);
    bellerophon_test::<f64, { STANDARD }>(9007199254740993, 0, false, 9223372036854776832, -1);
    bellerophon_test::<f64, { STANDARD }>(9007199254740994, 0, false, 1, 1076);

    bellerophon_test::<f64, { STANDARD }>(18014398509481984, 0, false, 0, 1077);
    bellerophon_test::<f64, { STANDARD }>(18014398509481986, 0, false, 9223372036854776832, -1);
    bellerophon_test::<f64, { STANDARD }>(18014398509481988, 0, false, 1, 1077);

    bellerophon_test::<f64, { STANDARD }>(9223372036854775808, 0, false, 0, 1086);
    bellerophon_test::<f64, { STANDARD }>(9223372036854776832, 0, false, 9223372036854776832, -1);
    bellerophon_test::<f64, { STANDARD }>(9223372036854777856, 0, false, 1, 1086);

    // Add a 0 but say we're truncated.
    bellerophon_test::<f64, { STANDARD }>(9007199254740992000, -3, true, 0, 1076);
    bellerophon_test::<f64, { STANDARD }>(9007199254740993000, -3, true, 9223372036854776832, -1);
    bellerophon_test::<f64, { STANDARD }>(9007199254740994000, -3, true, 1, 1076);
}

#[test]
#[cfg(feature = "compact")]
fn test_halfway_round_up() {
    // Halfway, round-down tests
    bellerophon_test::<f64, { STANDARD }>(9007199254740994, 0, false, 1, 1076);
    bellerophon_test::<f64, { STANDARD }>(9007199254740995, 0, false, 9223372036854778880, -1);
    bellerophon_test::<f64, { STANDARD }>(9007199254740996, 0, false, 2, 1076);

    bellerophon_test::<f64, { STANDARD }>(18014398509481988, 0, false, 1, 1077);
    bellerophon_test::<f64, { STANDARD }>(18014398509481990, 0, false, 9223372036854778880, -1);
    bellerophon_test::<f64, { STANDARD }>(18014398509481992, 0, false, 2, 1077);

    bellerophon_test::<f64, { STANDARD }>(9223372036854777856, 0, false, 1, 1086);
    bellerophon_test::<f64, { STANDARD }>(9223372036854778880, 0, false, 9223372036854778880, -1);
    bellerophon_test::<f64, { STANDARD }>(9223372036854779904, 0, false, 2, 1086);

    // Add a 0 but say we're truncated.
    bellerophon_test::<f64, { STANDARD }>(9007199254740994000, -3, true, 1, 1076);
    bellerophon_test::<f64, { STANDARD }>(9007199254740994990, -3, true, 9223372036854778869, -1);
    bellerophon_test::<f64, { STANDARD }>(9007199254740995000, -3, true, 9223372036854778879, -1);
    bellerophon_test::<f64, { STANDARD }>(9007199254740995010, -3, true, 9223372036854778890, -1);
    bellerophon_test::<f64, { STANDARD }>(9007199254740995050, -3, true, 2, 1076);
    bellerophon_test::<f64, { STANDARD }>(9007199254740996000, -3, true, 2, 1076);
}

#[test]
#[cfg(feature = "compact")]
fn test_extremes() {
    // Need to check we get proper results with rounding for near-infinity
    // and near-zero and/or denormal floats.
    bellerophon_test::<f64, { STANDARD }>(5, -324, false, 1, 0);
    bellerophon_test::<f64, { STANDARD }>(10, -324, false, 2, 0);
    // This is very close to 2.4703282292062327206e-342.
    bellerophon_test::<f64, { STANDARD }>(2470328229206232720, -342, false, 0, 0);
    bellerophon_test::<f64, { STANDARD }>(
        2470328229206232721,
        -342,
        false,
        9223372036854775808,
        -1,
    );
    bellerophon_test::<f64, { STANDARD }>(
        2470328229206232725,
        -342,
        false,
        9223372036854775824,
        -1,
    );
    bellerophon_test::<f64, { STANDARD }>(2470328229206232726, -342, false, 1, 0);
    bellerophon_test::<f64, { STANDARD }>(2470328229206232730, -342, false, 1, 0);
    // Check very close to literal infinity.
    // 17.976931348623155
    // 1.797693134862315508561243283845062402343434371574593359244049e+308
    // 1.797693134862315708145274237317043567980705675258449965989175e+308
    bellerophon_test::<f64, { STANDARD }>(17976931348623155, 292, false, 4503599627370494, 2046);
    bellerophon_test::<f64, { STANDARD }>(17976931348623156, 292, false, 4503599627370494, 2046);
    bellerophon_test::<f64, { STANDARD }>(1797693134862315605, 290, false, 4503599627370494, 2046);
    bellerophon_test::<f64, { STANDARD }>(1797693134862315607, 290, false, 4503599627370494, 2046);
    bellerophon_test::<f64, { STANDARD }>(
        1797693134862315608,
        290,
        false,
        18446744073709548540,
        -1,
    );
    bellerophon_test::<f64, { STANDARD }>(
        1797693134862315609,
        290,
        false,
        18446744073709548550,
        -1,
    );
    bellerophon_test::<f64, { STANDARD }>(179769313486231561, 291, false, 4503599627370495, 2046);
    bellerophon_test::<f64, { STANDARD }>(17976931348623157, 292, false, 4503599627370495, 2046);

    // Check existing issues and underflow.
    bellerophon_test::<f64, { STANDARD }>(2470328229206232726, -343, false, 0, 0);
    bellerophon_test::<f64, { STANDARD }>(2470328229206232726, -342, false, 1, 0);
    bellerophon_test::<f64, { STANDARD }>(1, -250, false, 1945308223406668, 192);
    bellerophon_test::<f64, { STANDARD }>(1, -150, false, 2867420733609077, 524);
    bellerophon_test::<f64, { STANDARD }>(1, -45, false, 1924152549665465, 873);
    bellerophon_test::<f64, { STANDARD }>(1, -40, false, 400386103400348, 890);
    bellerophon_test::<f64, { STANDARD }>(1, -20, false, 2142540351554083, 956);
    bellerophon_test::<f64, { STANDARD }>(1, 0, false, 0, 1023);
    bellerophon_test::<f64, { STANDARD }>(1, 20, false, 1599915997629504, 1089);
    bellerophon_test::<f64, { STANDARD }>(1, 40, false, 3768206498159781, 1155);
    bellerophon_test::<f64, { STANDARD }>(1, 150, false, 999684479948463, 1521);
    bellerophon_test::<f64, { STANDARD }>(1, 250, false, 1786584717939204, 1853);
    // Minimum positive normal float.
    bellerophon_test::<f64, { STANDARD }>(22250738585072014, -324, false, 0, 1);
    // Maximum positive subnormal float.
    bellerophon_test::<f64, { STANDARD }>(2225073858507201, -323, false, 4503599627370495, 0);
    // Next highest subnormal float.
    bellerophon_test::<f64, { STANDARD }>(22250738585072004, -324, false, 4503599627370494, 0);
    bellerophon_test::<f64, { STANDARD }>(22250738585072006, -324, false, 4503599627370494, 0);
    bellerophon_test::<f64, { STANDARD }>(22250738585072007, -324, false, 4503599627370495, 0);
    bellerophon_test::<f64, { STANDARD }>(222507385850720062, -325, false, 4503599627370494, 0);
    bellerophon_test::<f64, { STANDARD }>(222507385850720063, -325, false, 4503599627370494, 0);
    bellerophon_test::<f64, { STANDARD }>(222507385850720064, -325, false, 4503599627370494, 0);
    bellerophon_test::<f64, { STANDARD }>(
        2225073858507200641,
        -326,
        false,
        18446744073709545462,
        -1,
    );
    bellerophon_test::<f64, { STANDARD }>(
        2225073858507200642,
        -326,
        false,
        18446744073709545472,
        -1,
    );
    bellerophon_test::<f64, { STANDARD }>(222507385850720065, -325, false, 4503599627370495, 0);
}

#[test]
#[cfg(feature = "compact")]
fn compute_float_f32_rounding() {
    // These test near-halfway cases for single-precision floats.
    assert_eq!(compute_float32(0, 16777216), (151, 0));
    assert_eq!(compute_float32(0, 16777217), (-1, 9223372586610589696));
    assert_eq!(compute_float32(0, 16777218), (151, 1));
    assert_eq!(compute_float32(0, 16777219), (-1, 9223373686122217472));
    assert_eq!(compute_float32(0, 16777220), (151, 2));

    // These are examples of the above tests, with
    // digits from the exponent shifted to the mantissa.
    assert_eq!(compute_float32(-10, 167772160000000000), (151, 0));
    assert_eq!(compute_float32(-10, 167772170000000000), (-1, 9223372586610589696));
    assert_eq!(compute_float32(-10, 167772180000000000), (151, 1));
    // Let's check the lines to see if anything is different in table...
    assert_eq!(compute_float32(-10, 167772190000000000), (-1, 9223373686122217472));
    assert_eq!(compute_float32(-10, 167772200000000000), (151, 2));
}

#[test]
#[cfg(feature = "compact")]
fn compute_float_f64_rounding() {
    // These test near-halfway cases for double-precision floats.
    assert_eq!(compute_float64(0, 9007199254740992), (1076, 0));
    assert_eq!(compute_float64(0, 9007199254740993), (-1, 9223372036854776832));
    assert_eq!(compute_float64(0, 9007199254740994), (1076, 1));
    assert_eq!(compute_float64(0, 9007199254740995), (-1, 9223372036854778880));
    assert_eq!(compute_float64(0, 9007199254740996), (1076, 2));
    assert_eq!(compute_float64(0, 18014398509481984), (1077, 0));
    assert_eq!(compute_float64(0, 18014398509481986), (-1, 9223372036854776832));
    assert_eq!(compute_float64(0, 18014398509481988), (1077, 1));
    assert_eq!(compute_float64(0, 18014398509481990), (-1, 9223372036854778880));
    assert_eq!(compute_float64(0, 18014398509481992), (1077, 2));

    // These are examples of the above tests, with
    // digits from the exponent shifted to the mantissa.
    assert_eq!(compute_float64(-3, 9007199254740992000), (1076, 0));
    assert_eq!(compute_float64(-3, 9007199254740993000), (-1, 9223372036854776832));
    assert_eq!(compute_float64(-3, 9007199254740994000), (1076, 1));
    assert_eq!(compute_float64(-3, 9007199254740995000), (-1, 9223372036854778879));
    assert_eq!(compute_float64(-3, 9007199254740996000), (1076, 2));
}
