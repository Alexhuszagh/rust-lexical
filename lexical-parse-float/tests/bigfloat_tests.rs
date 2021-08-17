#![cfg(feature = "radix")]

use lexical_parse_float::bigint::{self, Bigfloat, Limb, LIMB_BITS};

#[test]
fn leading_zeros_test() {
    assert_eq!(Bigfloat::new().leading_zeros(), 0);

    assert_eq!(Bigfloat::from_u32(0xFF).leading_zeros(), LIMB_BITS as u32 - 8);
    assert_eq!(Bigfloat::from_u64(0xFF00000000).leading_zeros(), 24);

    assert_eq!(Bigfloat::from_u32(0xF).leading_zeros(), LIMB_BITS as u32 - 4);
    assert_eq!(Bigfloat::from_u64(0xF00000000).leading_zeros(), 28);

    assert_eq!(Bigfloat::from_u32(0xF0).leading_zeros(), LIMB_BITS as u32 - 8);
    assert_eq!(Bigfloat::from_u64(0xF000000000).leading_zeros(), 24);
}
