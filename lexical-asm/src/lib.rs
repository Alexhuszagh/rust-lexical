#![allow(unused)]

use lexical_parse_integer::FromLexical;
use lexical_util::error::Error;

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
}

macro_rules! parse_core {
    ($($name:ident $t:ty ;)*) => ($(
        pub fn $name(s: &str) -> Result<$t, ParseIntError> {
            from_str_radix::<$t>(s, 10)
        }
    )*);
}

parse_core! {
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

// CORE
// Carbon copy of the implementation from Rust core.

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
enum IntErrorKind {
    /// Value being parsed is empty.
    ///
    /// This variant will be constructed when parsing an empty string.
    Empty,
    /// Contains an invalid digit in its context.
    ///
    /// Among other causes, this variant will be constructed when parsing a string that
    /// contains a non-ASCII char.
    ///
    /// This variant is also constructed when a `+` or `-` is misplaced within a string
    /// either on its own or in the middle of a number.
    InvalidDigit,
    /// Integer is too large to store in target integer type.
    PosOverflow,
    /// Integer is too small to store in target integer type.
    NegOverflow,
    /// Value was Zero
    ///
    /// This variant will be emitted when the parsing string has a value of zero, which
    /// would be illegal for non-zero types.
    Zero,
}

pub struct ParseIntError {
    kind: IntErrorKind,
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
        radix >= 2 && radix <= 36,
        "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
        radix
    );

    if src.is_empty() {
        return Err(PIE {
            kind: Empty,
        });
    }

    let is_signed_ty = T::from_u32(0) > T::min_value();

    // all valid digits are ascii, so we will just iterate over the utf8 bytes
    // and cast them to chars. .to_digit() will safely return None for anything
    // other than a valid ascii digit for the given radix, including the first-byte
    // of multi-byte sequences
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
        // The number is positive
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
        // The number is negative
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
