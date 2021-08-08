// CORE
// Carbon copy of the implementation from Rust core.

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[non_exhaustive]
pub enum IntErrorKind {
    Empty,
    InvalidDigit,
    PosOverflow,
    NegOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ParseIntError {
    pub kind: IntErrorKind,
}

pub trait FromStrRadixHelper: PartialOrd + Copy {
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

pub fn parse_int<T: FromStrRadixHelper>(src: &str, radix: u32) -> Result<T, ParseIntError> {
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
