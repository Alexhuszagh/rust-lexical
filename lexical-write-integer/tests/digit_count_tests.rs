#![cfg(not(feature = "compact"))]

mod util;

use lexical_write_integer::digit_count;

#[test]
fn fast_log2_test() {
    // Check the first, even if illogical case works.
    assert_eq!(digit_count::fast_log2(0u32), 0);
    assert_eq!(digit_count::fast_log2(1u32), 0);
    assert_eq!(digit_count::fast_log2(2u32), 1);
    assert_eq!(digit_count::fast_log2(3u32), 1);

    assert_eq!(digit_count::fast_log2((1u32 << 16) - 1), 15);
    assert_eq!(digit_count::fast_log2(1u32 << 16), 16);
    assert_eq!(digit_count::fast_log2((1u32 << 16) + 1), 16);

    assert_eq!(digit_count::fast_log2(u32::MAX), 31);
}

fn slow_log2(x: u32) -> usize {
    // Slow approach to calculating a log2, using floats.
    if x == 0 {
        0
    } else {
        (x as f64).log2().floor() as usize
    }
}

default_quickcheck! {
    fn fast_log2_quickcheck(x: u32) -> bool {
        slow_log2(x) == digit_count::fast_log2(x)
    }
}
