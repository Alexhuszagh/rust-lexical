mod util;

use core::fmt::Debug;
use core::str::{from_utf8_unchecked, FromStr};

#[cfg(feature = "radix")]
use lexical_util::constants::BUFFER_SIZE;
use lexical_write_integer::{ToLexical, ToLexicalWithOptions};
use proptest::prelude::*;
#[cfg(feature = "radix")]
use util::from_radix;

use crate::util::default_proptest_config;

trait Roundtrip: ToLexical + ToLexicalWithOptions + FromStr {
    #[allow(dead_code)]
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, std::num::ParseIntError>;
}

macro_rules! roundtrip_impl {
    ($($t:ty)*) => ($(
        impl Roundtrip for $t {
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, std::num::ParseIntError> {
                <$t>::from_str_radix(src, radix)
            }
        }
    )*);
}

roundtrip_impl! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

fn roundtrip<T>(x: T) -> T
where
    T: Roundtrip,
    <T as FromStr>::Err: Debug,
{
    let mut buffer = [b'\x00'; 48];
    let bytes = x.to_lexical(&mut buffer);
    let string = unsafe { from_utf8_unchecked(bytes) };
    string.parse::<T>().unwrap()
}

#[cfg(feature = "radix")]
fn roundtrip_radix<T>(x: T, radix: u32) -> T
where
    T: Roundtrip,
    <T as FromStr>::Err: Debug,
{
    let mut buffer = [b'\x00'; BUFFER_SIZE];
    let options = T::Options::default();
    // Trick it into assuming we have a valid radix.
    let bytes = match radix {
        2 => x.to_lexical_with_options::<{ from_radix(2) }>(&mut buffer, &options),
        3 => x.to_lexical_with_options::<{ from_radix(3) }>(&mut buffer, &options),
        4 => x.to_lexical_with_options::<{ from_radix(4) }>(&mut buffer, &options),
        5 => x.to_lexical_with_options::<{ from_radix(5) }>(&mut buffer, &options),
        6 => x.to_lexical_with_options::<{ from_radix(6) }>(&mut buffer, &options),
        7 => x.to_lexical_with_options::<{ from_radix(7) }>(&mut buffer, &options),
        8 => x.to_lexical_with_options::<{ from_radix(8) }>(&mut buffer, &options),
        9 => x.to_lexical_with_options::<{ from_radix(9) }>(&mut buffer, &options),
        10 => x.to_lexical_with_options::<{ from_radix(10) }>(&mut buffer, &options),
        11 => x.to_lexical_with_options::<{ from_radix(11) }>(&mut buffer, &options),
        12 => x.to_lexical_with_options::<{ from_radix(12) }>(&mut buffer, &options),
        13 => x.to_lexical_with_options::<{ from_radix(13) }>(&mut buffer, &options),
        14 => x.to_lexical_with_options::<{ from_radix(14) }>(&mut buffer, &options),
        15 => x.to_lexical_with_options::<{ from_radix(15) }>(&mut buffer, &options),
        16 => x.to_lexical_with_options::<{ from_radix(16) }>(&mut buffer, &options),
        17 => x.to_lexical_with_options::<{ from_radix(17) }>(&mut buffer, &options),
        18 => x.to_lexical_with_options::<{ from_radix(18) }>(&mut buffer, &options),
        19 => x.to_lexical_with_options::<{ from_radix(19) }>(&mut buffer, &options),
        20 => x.to_lexical_with_options::<{ from_radix(20) }>(&mut buffer, &options),
        21 => x.to_lexical_with_options::<{ from_radix(21) }>(&mut buffer, &options),
        22 => x.to_lexical_with_options::<{ from_radix(22) }>(&mut buffer, &options),
        23 => x.to_lexical_with_options::<{ from_radix(23) }>(&mut buffer, &options),
        24 => x.to_lexical_with_options::<{ from_radix(24) }>(&mut buffer, &options),
        25 => x.to_lexical_with_options::<{ from_radix(25) }>(&mut buffer, &options),
        26 => x.to_lexical_with_options::<{ from_radix(26) }>(&mut buffer, &options),
        27 => x.to_lexical_with_options::<{ from_radix(27) }>(&mut buffer, &options),
        28 => x.to_lexical_with_options::<{ from_radix(28) }>(&mut buffer, &options),
        29 => x.to_lexical_with_options::<{ from_radix(29) }>(&mut buffer, &options),
        30 => x.to_lexical_with_options::<{ from_radix(30) }>(&mut buffer, &options),
        31 => x.to_lexical_with_options::<{ from_radix(31) }>(&mut buffer, &options),
        32 => x.to_lexical_with_options::<{ from_radix(32) }>(&mut buffer, &options),
        33 => x.to_lexical_with_options::<{ from_radix(33) }>(&mut buffer, &options),
        34 => x.to_lexical_with_options::<{ from_radix(34) }>(&mut buffer, &options),
        35 => x.to_lexical_with_options::<{ from_radix(35) }>(&mut buffer, &options),
        36 => x.to_lexical_with_options::<{ from_radix(36) }>(&mut buffer, &options),
        _ => unreachable!(),
    };
    let string = unsafe { from_utf8_unchecked(bytes) };
    T::from_str_radix(string, radix).unwrap()
}

default_quickcheck! {
    fn u8_quickcheck(i: u8) -> bool {
        i == roundtrip(i)
    }

    fn u16_quickcheck(i: u16) -> bool {
        i == roundtrip(i)
    }

    fn u32_quickcheck(i: u32) -> bool {
        i == roundtrip(i)
    }

    fn u64_quickcheck(i: u64) -> bool {
        i == roundtrip(i)
    }

    fn u128_quickcheck(i: u128) -> bool {
        i == roundtrip(i)
    }

    fn usize_quickcheck(i: usize) -> bool {
        i == roundtrip(i)
    }

    fn i8_quickcheck(i: i8) -> bool {
        i == roundtrip(i)
    }

    fn i16_quickcheck(i: i16) -> bool {
        i == roundtrip(i)
    }

    fn i32_quickcheck(i: i32) -> bool {
        i == roundtrip(i)
    }

    fn i64_quickcheck(i: i64) -> bool {
        i == roundtrip(i)
    }

    fn i128_quickcheck(i: i128) -> bool {
        i == roundtrip(i)
    }

    fn isize_quickcheck(i: isize) -> bool {
        i == roundtrip(i)
    }
}

proptest! {
    #![proptest_config(default_proptest_config())]

    #[test]
    fn u8_proptest(i in u8::MIN..u8::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn i8_proptest(i in i8::MIN..i8::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn u16_proptest(i in u16::MIN..u16::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn i16_proptest(i in i16::MIN..i16::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn u32_proptest(i in u32::MIN..u32::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn i32_proptest(i in i32::MIN..i32::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn u64_proptest(i in u64::MIN..u64::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn i64_proptest(i in i64::MIN..i64::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn u128_proptest(i in u128::MIN..u128::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn i128_proptest(i in i128::MIN..i128::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn usize_proptest(i in usize::MIN..usize::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn isize_proptest(i in isize::MIN..isize::MAX) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    fn jeaiii_magic_10u64_proptest(i in 10_0000_0000..100_0000_0000u64) {
        prop_assert_eq!(i, roundtrip(i));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u8_proptest_radix(i in u8::MIN..u8::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn i8_proptest_radix(i in i8::MIN..i8::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u16_proptest_radix(i in u16::MIN..u16::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn i16_proptest_radix(i in i16::MIN..i16::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u32_proptest_radix(i in u32::MIN..u32::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn i32_proptest_radix(i in i32::MIN..i32::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u64_proptest_radix(i in u64::MIN..u64::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn i64_proptest_radix(i in i64::MIN..i64::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u128_proptest_radix(i in u128::MIN..u128::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn i128_proptest_radix(i in i128::MIN..i128::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn usize_proptest_radix(i in usize::MIN..usize::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn isize_proptest_radix(i in isize::MIN..isize::MAX, radix in 2u32..=36) {
        prop_assert_eq!(i, roundtrip_radix(i, radix));
    }
}
