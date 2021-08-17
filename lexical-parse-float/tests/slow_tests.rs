use lexical_parse_float::float::RawFloat;
use lexical_parse_float::slow;

fn b<F: RawFloat>(float: F) -> (u64, i32) {
    let fp = slow::b(float);
    (fp.mant, fp.exp)
}

fn bh<F: RawFloat>(float: F) -> (u64, i32) {
    let fp = slow::bh(float);
    (fp.mant, fp.exp)
}

#[test]
fn b_test() {
    assert_eq!(b(1e-45_f32), (1, -149));
    assert_eq!(b(5e-324_f64), (1, -1074));
    assert_eq!(b(1e-323_f64), (2, -1074));
    assert_eq!(b(2e-323_f64), (4, -1074));
    assert_eq!(b(3e-323_f64), (6, -1074));
    assert_eq!(b(4e-323_f64), (8, -1074));
    assert_eq!(b(5e-323_f64), (10, -1074));
    assert_eq!(b(6e-323_f64), (12, -1074));
    assert_eq!(b(7e-323_f64), (14, -1074));
    assert_eq!(b(8e-323_f64), (16, -1074));
    assert_eq!(b(9e-323_f64), (18, -1074));
    assert_eq!(b(1_f32), (8388608, -23));
    assert_eq!(b(1_f64), (4503599627370496, -52));
    assert_eq!(b(1e38_f32), (9860761, 103));
    assert_eq!(b(1e308_f64), (5010420900022432, 971));
}

#[test]
fn bh_test() {
    assert_eq!(bh(1e-45_f32), (3, -150));
    assert_eq!(bh(5e-324_f64), (3, -1075));
    assert_eq!(bh(1_f32), (16777217, -24));
    assert_eq!(bh(1_f64), (9007199254740993, -53));
    assert_eq!(bh(1e38_f32), (19721523, 102));
    assert_eq!(bh(1e308_f64), (10020841800044865, 970));
}
