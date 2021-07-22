use lexical_util::algorithm;

#[test]
fn copy_to_dest_test() {
    let src = b"12345";
    let mut dst = [b'0'; 16];

    assert_eq!(5, unsafe { algorithm::copy_to_dst(&mut dst, src) });
    assert_eq!(&dst[..5], src);
}
