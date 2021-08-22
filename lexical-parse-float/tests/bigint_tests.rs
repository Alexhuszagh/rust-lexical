mod stackvec;

use lexical_parse_float::bigint::Bigint;
use stackvec::vec_from_u32;

#[test]
fn simple_test() {
    let x = Bigint::new();
    assert_eq!(x.hi64(), (0, false));

    let x = Bigint::from_u32(1);
    assert_eq!(&*x.data, &[1]);

    let mut x = Bigint::from_u64(1);
    assert_eq!(&*x.data, &[1]);

    x.pow(10, 10);
    let expected = vec_from_u32(&[1410065408, 2]);
    assert!(x.data == expected, "failed");
    assert_eq!(x.bit_length(), 34);

    let y = Bigint::from_u64(5);
    x *= &y;
    let expected = vec_from_u32(&[2755359744, 11]);
    assert!(x.data == expected, "failed");
}
