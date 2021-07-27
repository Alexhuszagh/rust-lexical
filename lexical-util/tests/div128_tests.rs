#![cfg(feature = "write")]

use lexical_util::div128::u128_divrem;
use lexical_util::step::u64_step;
use proptest::{prop_assert_eq, proptest};

proptest! {
    #[test]
    fn u128_divrem_proptest(i in u128::min_value()..u128::max_value()) {
        let (hi, lo) = u128_divrem::<10>(i);
        let step = u64_step::<10>();
        let d = 10u128.pow(step as u32);
        let expected = (i / d, (i % d) as u64);
        prop_assert_eq!((hi, lo), expected);
    }

    #[test]
    #[cfg(feature = "radix")]
    fn u128_divrem_2_proptest(i in u128::min_value()..u128::max_value(), radix in 2u32..=36) {
        // Simulate a const expr.
        let ((hi, lo), step) = match radix {
            2 => (u128_divrem::<2>(i), u64_step::<2>()),
            3 => (u128_divrem::<3>(i), u64_step::<3>()),
            4 => (u128_divrem::<4>(i), u64_step::<4>()),
            5 => (u128_divrem::<5>(i), u64_step::<5>()),
            6 => (u128_divrem::<6>(i), u64_step::<6>()),
            7 => (u128_divrem::<7>(i), u64_step::<7>()),
            8 => (u128_divrem::<8>(i), u64_step::<8>()),
            9 => (u128_divrem::<9>(i), u64_step::<9>()),
            10 => (u128_divrem::<10>(i), u64_step::<10>()),
            11 => (u128_divrem::<11>(i), u64_step::<11>()),
            12 => (u128_divrem::<12>(i), u64_step::<12>()),
            13 => (u128_divrem::<13>(i), u64_step::<13>()),
            14 => (u128_divrem::<14>(i), u64_step::<14>()),
            15 => (u128_divrem::<15>(i), u64_step::<15>()),
            16 => (u128_divrem::<16>(i), u64_step::<16>()),
            17 => (u128_divrem::<17>(i), u64_step::<17>()),
            18 => (u128_divrem::<18>(i), u64_step::<18>()),
            19 => (u128_divrem::<19>(i), u64_step::<19>()),
            20 => (u128_divrem::<20>(i), u64_step::<20>()),
            21 => (u128_divrem::<21>(i), u64_step::<21>()),
            22 => (u128_divrem::<22>(i), u64_step::<22>()),
            23 => (u128_divrem::<23>(i), u64_step::<23>()),
            24 => (u128_divrem::<24>(i), u64_step::<24>()),
            25 => (u128_divrem::<25>(i), u64_step::<25>()),
            26 => (u128_divrem::<26>(i), u64_step::<26>()),
            27 => (u128_divrem::<27>(i), u64_step::<27>()),
            28 => (u128_divrem::<28>(i), u64_step::<28>()),
            29 => (u128_divrem::<29>(i), u64_step::<29>()),
            30 => (u128_divrem::<30>(i), u64_step::<30>()),
            31 => (u128_divrem::<31>(i), u64_step::<31>()),
            32 => (u128_divrem::<32>(i), u64_step::<32>()),
            33 => (u128_divrem::<33>(i), u64_step::<33>()),
            34 => (u128_divrem::<34>(i), u64_step::<34>()),
            35 => (u128_divrem::<35>(i), u64_step::<35>()),
            36 => (u128_divrem::<36>(i), u64_step::<36>()),
            _ => unreachable!(),
        };
        let d = (radix as u128).pow(step as u32);
        let expected = (i / d, (i % d) as u64);
        prop_assert_eq!((hi, lo), expected);
    }
}
