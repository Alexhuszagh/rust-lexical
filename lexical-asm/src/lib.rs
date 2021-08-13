use core::num::ParseFloatError;
use lexical_parse_float::FromLexical as FloatFromLexical;
use lexical_parse_integer::FromLexical as IntFromLexical;
use lexical_util::error::Error;
use lexical_write_float::ToLexical as FloatToLexical;
use lexical_write_integer::ToLexical as IntToLexical;
use std::io::Write;

// PARSE INTEGER
// -------------

macro_rules! parse_lexical {
    ($($name:ident $t:ty ;)*) => ($(
        pub fn $name(s: &str) -> Result<$t, Error> {
            <$t>::from_lexical(s.as_bytes())
        }
    )*);
}

parse_lexical! {
    u8_parse_lexical u8 ;
    u16_parse_lexical u16 ;
    u32_parse_lexical u32 ;
    u64_parse_lexical u64 ;
    u128_parse_lexical u128 ;
    i8_parse_lexical i8 ;
    i16_parse_lexical i16 ;
    i32_parse_lexical i32 ;
    i64_parse_lexical i64 ;
    i128_parse_lexical i128 ;
    f32_parse_lexical f32 ;
    f64_parse_lexical f64 ;
}

macro_rules! parse_from_str_radix {
    ($($name:ident $t:ty ;)*) => ($(
        pub fn $name(s: &str) -> Result<$t, ParseIntError> {
            from_str_radix::<$t>(s, 10)
        }
    )*);
}

macro_rules! parse_core {
    ($($name:ident $t:ty ;)*) => ($(
        pub fn $name(s: &str) -> Result<$t, ParseFloatError> {
            s.parse::<$t>()
        }
    )*);
}

parse_from_str_radix! {
    u8_parse_core u8 ;
    u16_parse_core u16 ;
    u32_parse_core u32 ;
    u64_parse_core u64 ;
    u128_parse_core u128 ;
    i8_parse_core i8 ;
    i16_parse_core i16 ;
    i32_parse_core i32 ;
    i64_parse_core i64 ;
    i128_parse_core i128 ;
}

parse_core! {
    f32_parse_core f32 ;
    f64_parse_core f64 ;
}

// CORE
// Carbon copy of the implementation from Rust core.

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IntErrorKind {
    Empty,
    InvalidDigit,
    PosOverflow,
    NegOverflow,
    Zero,
}

pub struct ParseIntError {
    pub kind: IntErrorKind,
}

trait FromStrRadixHelper: PartialOrd + Copy {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn from_u32(u: u32) -> Self;
    fn checked_mul(&self, other: u32) -> Option<Self>;
    fn checked_sub(&self, other: u32) -> Option<Self>;
    fn checked_add(&self, other: u32) -> Option<Self>;
}

macro_rules! doit {
    ($($t:ty)*) => ($(impl FromStrRadixHelper for $t {
        #[inline]
        fn min_value() -> Self { Self::MIN }
        #[inline]
        fn max_value() -> Self { Self::MAX }
        #[inline]
        fn from_u32(u: u32) -> Self { u as Self }
        #[inline]
        fn checked_mul(&self, other: u32) -> Option<Self> {
            Self::checked_mul(*self, other as Self)
        }
        #[inline]
        fn checked_sub(&self, other: u32) -> Option<Self> {
            Self::checked_sub(*self, other as Self)
        }
        #[inline]
        fn checked_add(&self, other: u32) -> Option<Self> {
            Self::checked_add(*self, other as Self)
        }
    })*)
}
doit! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

fn from_str_radix<T: FromStrRadixHelper>(src: &str, radix: u32) -> Result<T, ParseIntError> {
    use self::IntErrorKind::*;
    use self::ParseIntError as PIE;

    assert!(
        (2..=36).contains(&radix),
        "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
        radix
    );

    if src.is_empty() {
        return Err(PIE {
            kind: Empty,
        });
    }

    let is_signed_ty = T::from_u32(0) > T::min_value();
    let src = src.as_bytes();

    let (is_positive, digits) = match src[0] {
        b'+' | b'-' if src[1..].is_empty() => {
            return Err(PIE {
                kind: InvalidDigit,
            });
        },
        b'+' => (true, &src[1..]),
        b'-' if is_signed_ty => (false, &src[1..]),
        _ => (true, src),
    };

    let mut result = T::from_u32(0);
    if is_positive {
        for &c in digits {
            let x = match (c as char).to_digit(radix) {
                Some(x) => x,
                None => {
                    return Err(PIE {
                        kind: InvalidDigit,
                    })
                },
            };
            result = match result.checked_mul(radix) {
                Some(result) => result,
                None => {
                    return Err(PIE {
                        kind: PosOverflow,
                    })
                },
            };
            result = match result.checked_add(x) {
                Some(result) => result,
                None => {
                    return Err(PIE {
                        kind: PosOverflow,
                    })
                },
            };
        }
    } else {
        for &c in digits {
            let x = match (c as char).to_digit(radix) {
                Some(x) => x,
                None => {
                    return Err(PIE {
                        kind: InvalidDigit,
                    })
                },
            };
            result = match result.checked_mul(radix) {
                Some(result) => result,
                None => {
                    return Err(PIE {
                        kind: NegOverflow,
                    })
                },
            };
            result = match result.checked_sub(x) {
                Some(result) => result,
                None => {
                    return Err(PIE {
                        kind: NegOverflow,
                    })
                },
            };
        }
    }
    Ok(result)
}

// WRITE INTEGER
// -------------

macro_rules! write_lexical {
    ($($name:ident $t:ty ;)*) => ($(
        pub fn $name(value: $t, buffer: &mut [u8]) -> &mut [u8] {
            value.to_lexical(buffer)
        }
    )*);
}

write_lexical! {
    u8_write_lexical u8 ;
    u16_write_lexical u16 ;
    u32_write_lexical u32 ;
    u64_write_lexical u64 ;
    u128_write_lexical u128 ;
    i8_write_lexical i8 ;
    i16_write_lexical i16 ;
    i32_write_lexical i32 ;
    i64_write_lexical i64 ;
    i128_write_lexical i128 ;
    f32_write_lexical f32 ;
    f64_write_lexical f64 ;
}

macro_rules! write_std {
    ($($name:ident $t:ty ;)*) => ($(
        pub fn $name(value: $t, buffer: &mut Vec<u8>) -> &mut [u8] {
            write!(buffer, "{}", value).unwrap();
            buffer
        }
    )*);
}

write_std! {
    u8_write_std u8 ;
    u16_write_std u16 ;
    u32_write_std u32 ;
    u64_write_std u64 ;
    u128_write_std u128 ;
    i8_write_std i8 ;
    i16_write_std i16 ;
    i32_write_std i32 ;
    i64_write_std i64 ;
    i128_write_std i128 ;
    f32_write_std f32 ;
    f64_write_std f64 ;
}
