#![cfg(feature = "write")]

use lexical_util::div128;
use proptest::{prop_assert_eq, proptest};

proptest! {
    #[test]
    fn u128_divrem_proptest(i in u128::min_value()..u128::max_value()) {
        let (hi, lo) = div128::u128_divrem::<10>(i);
        let step = div128::u64_step::<10>();
        let d = 10u128.pow(step as u32);
        let expected = (i / d, (i % d) as u64);
        prop_assert_eq!((hi, lo), expected);
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u128_divrem_2_proptest(i in u128::min_value()..u128::max_value(), radix in 2u32..=36) {
        // Simulate a const expr.
        let ((hi, lo), step) = match radix {
            2 => (div128::u128_divrem::<2>(i), div128::u64_step::<2>()),
            3 => (div128::u128_divrem::<3>(i), div128::u64_step::<3>()),
            4 => (div128::u128_divrem::<4>(i), div128::u64_step::<4>()),
            5 => (div128::u128_divrem::<5>(i), div128::u64_step::<5>()),
            6 => (div128::u128_divrem::<6>(i), div128::u64_step::<6>()),
            7 => (div128::u128_divrem::<7>(i), div128::u64_step::<7>()),
            8 => (div128::u128_divrem::<8>(i), div128::u64_step::<8>()),
            9 => (div128::u128_divrem::<9>(i), div128::u64_step::<9>()),
            10 => (div128::u128_divrem::<10>(i), div128::u64_step::<10>()),
            11 => (div128::u128_divrem::<11>(i), div128::u64_step::<11>()),
            12 => (div128::u128_divrem::<12>(i), div128::u64_step::<12>()),
            13 => (div128::u128_divrem::<13>(i), div128::u64_step::<13>()),
            14 => (div128::u128_divrem::<14>(i), div128::u64_step::<14>()),
            15 => (div128::u128_divrem::<15>(i), div128::u64_step::<15>()),
            16 => (div128::u128_divrem::<16>(i), div128::u64_step::<16>()),
            17 => (div128::u128_divrem::<17>(i), div128::u64_step::<17>()),
            18 => (div128::u128_divrem::<18>(i), div128::u64_step::<18>()),
            19 => (div128::u128_divrem::<19>(i), div128::u64_step::<19>()),
            20 => (div128::u128_divrem::<20>(i), div128::u64_step::<20>()),
            21 => (div128::u128_divrem::<21>(i), div128::u64_step::<21>()),
            22 => (div128::u128_divrem::<22>(i), div128::u64_step::<22>()),
            23 => (div128::u128_divrem::<23>(i), div128::u64_step::<23>()),
            24 => (div128::u128_divrem::<24>(i), div128::u64_step::<24>()),
            25 => (div128::u128_divrem::<25>(i), div128::u64_step::<25>()),
            26 => (div128::u128_divrem::<26>(i), div128::u64_step::<26>()),
            27 => (div128::u128_divrem::<27>(i), div128::u64_step::<27>()),
            28 => (div128::u128_divrem::<28>(i), div128::u64_step::<28>()),
            29 => (div128::u128_divrem::<29>(i), div128::u64_step::<29>()),
            30 => (div128::u128_divrem::<30>(i), div128::u64_step::<30>()),
            31 => (div128::u128_divrem::<31>(i), div128::u64_step::<31>()),
            32 => (div128::u128_divrem::<32>(i), div128::u64_step::<32>()),
            33 => (div128::u128_divrem::<33>(i), div128::u64_step::<33>()),
            34 => (div128::u128_divrem::<34>(i), div128::u64_step::<34>()),
            35 => (div128::u128_divrem::<35>(i), div128::u64_step::<35>()),
            36 => (div128::u128_divrem::<36>(i), div128::u64_step::<36>()),
            _ => unreachable!(),
        };
        let d = (radix as u128).pow(step as u32);
        let expected = (i / d, (i % d) as u64);
        prop_assert_eq!((hi, lo), expected);
    }
}
