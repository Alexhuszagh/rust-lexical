mod util;

use lexical_util::mul::{mul, mulhi};

default_quickcheck! {
    fn mul_u16_quickcheck(x: u16, y: u16) -> bool {
        let (hi, lo) = mul::<u16, u8>(x, y);
        let hi = hi as u32;
        let lo = lo as u32;
        let expected = x as u32 * y as u32;
        ((hi << 16) | lo) == expected
    }

    fn mul_u32_quickcheck(x: u32, y: u32) -> bool {
        let (hi, lo) = mul::<u32, u16>(x, y);
        let hi = hi as u64;
        let lo = lo as u64;
        let expected = x as u64 * y as u64;
        ((hi << 32) | lo) == expected
    }

    fn mul_u64_quickcheck(x: u64, y: u64) -> bool {
        let (hi, lo) = mul::<u64, u32>(x, y);
        let hi = hi as u128;
        let lo = lo as u128;
        let expected = x as u128 * y as u128;
        ((hi << 64) | lo) == expected
    }

    fn mulhi_u16_quickcheck(x: u16, y: u16) -> bool {
        let actual = mulhi::<u16, u8>(x, y);
        let expected = (x as u32 * y as u32) >> 16;
        actual == expected as u16
    }

    fn mulhi_u32_quickcheck(x: u32, y: u32) -> bool {
        let actual = mulhi::<u32, u16>(x, y);
        let expected = (x as u64 * y as u64) >> 32;
        actual == expected as u32
    }

    fn mulhi_u64_quickcheck(x: u64, y: u64) -> bool {
        let actual = mulhi::<u64, u32>(x, y);
        let expected = (x as u128 * y as u128) >> 64;
        actual == expected as u64
    }
}
