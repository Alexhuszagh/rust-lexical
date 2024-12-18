#![allow(dead_code, unused_imports)]

#[cfg(feature = "power-of-two")]
use lexical_util::format::NumberFormatBuilder;
use proptest::prelude::*;
pub(crate) use quickcheck::QuickCheck;

#[cfg(feature = "power-of-two")]
pub const fn from_radix(radix: u8) -> u128 {
    NumberFormatBuilder::from_radix(radix)
}

pub fn default_proptest_config() -> ProptestConfig {
    ProptestConfig {
        cases: if cfg!(miri) {
            10
        } else {
            ProptestConfig::default().cases
        },
        max_shrink_iters: if cfg!(miri) {
            10
        } else {
            ProptestConfig::default().max_shrink_iters
        },
        failure_persistence: if cfg!(miri) {
            None
        } else {
            ProptestConfig::default().failure_persistence
        },
        ..ProptestConfig::default()
    }
}

// This is almost identical to quickcheck's itself, just to add default
// arguments  https://docs.rs/quickcheck/1.0.3/src/quickcheck/lib.rs.html#43-67
// The code is unlicensed.
#[macro_export]
macro_rules! default_quickcheck {
    (@as_items $($i:item)*) => ($($i)*);
    {
        $(
            $(#[$m:meta])*
            fn $fn_name:ident($($arg_name:ident : $arg_ty:ty),*) -> $ret:ty {
                $($code:tt)*
            }
        )*
    } => (
        $crate::default_quickcheck! {
            @as_items
            $(
                #[test]
                $(#[$m])*
                fn $fn_name() {
                    fn prop($($arg_name: $arg_ty),*) -> $ret {
                        $($code)*
                    }
                    $crate::util::QuickCheck::new()
                        .max_tests(if cfg!(miri) { 10 } else { 10_000 })
                        .quickcheck(prop as fn($($arg_ty),*) -> $ret);
                }
            )*
        }
    )
}
