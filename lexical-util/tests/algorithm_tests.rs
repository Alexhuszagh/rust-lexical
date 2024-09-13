#[cfg(feature = "write")]
use lexical_util::algorithm;

#[test]
#[cfg(feature = "write")]
fn copy_to_dest_test() {
    let src = b"12345";
    let mut dst = [b'0'; 16];

    assert_eq!(5, algorithm::copy_to_dst(&mut dst, src));
    assert_eq!(&dst[..5], src);
}

#[test]
#[cfg(feature = "write")]
fn ltrim_char_test() {
    let w = "0001";
    let x = "1010";
    let y = "1.00";
    let z = "1e05";

    assert_eq!(algorithm::ltrim_char_count(w.as_bytes(), b'0'), 3);
    assert_eq!(algorithm::ltrim_char_count(x.as_bytes(), b'0'), 0);
    assert_eq!(algorithm::ltrim_char_count(x.as_bytes(), b'1'), 1);
    assert_eq!(algorithm::ltrim_char_count(y.as_bytes(), b'0'), 0);
    assert_eq!(algorithm::ltrim_char_count(y.as_bytes(), b'1'), 1);
    assert_eq!(algorithm::ltrim_char_count(z.as_bytes(), b'0'), 0);
    assert_eq!(algorithm::ltrim_char_count(z.as_bytes(), b'1'), 1);
    assert_eq!(algorithm::ltrim_char_count(z.as_bytes(), b'5'), 0);
}
