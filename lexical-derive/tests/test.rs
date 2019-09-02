//! Test the proc-macro.

extern crate lexical_core;
extern crate lexical_derive;

use lexical_derive::Lexical;

trait Lexical: Sized {
    fn to_lexical<'a>(self, bytes: &'a mut [u8]) -> &'a mut [u8];
    fn from_lexical(bytes: &[u8]) -> lexical_core::Result<Self>;
}

#[derive(Lexical)]
struct Wrapper {
    pub value: i32,
}

#[test]
fn to_lexical_test() {
    let wrapper = Wrapper { value: 15 };
    let mut bytes = [b'0'; 256];
    assert_eq!(wrapper.to_lexical(&mut bytes), b"15");
}

#[test]
fn from_lexical_test() {
    let res = Wrapper::from_lexical(b"15").unwrap();
    assert_eq!(res.value, 15);
}
