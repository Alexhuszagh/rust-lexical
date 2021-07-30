// Bare-bones implementation algorithm to avoid any abstractions.

use lexical_util::digit::char_to_digit;
use lexical_util::num::{as_cast, Integer};
//use lexical_util::result::Result;
use lexical_util::{from_lexical, lexical_partial_to_complete};

macro_rules! into_error_v2 {
    ($code:ident) => {
        Err(lexical_util::error::ErrorCode::$code)
    };
}

/// Parse digits for a positive or negative value.
/// Optimized for operations with machine integers.
macro_rules! parse_digits {
    ($iter:ident, $radix:expr, $addsub:ident, $overflow:ident, $t:ident) => {{
        let mut value = <$t>::ZERO;

        // Do our slow parsing algorithm: 1 digit at a time.
        while let Some(&c) = $iter.next() {
            let digit = match (c as char).to_digit($radix) {
                Some(v) => v,
                None => return Ok(value),
            };
            value = match value.checked_mul(as_cast($radix)) {
                Some(v) => v,
                None => return into_error_v2!($overflow),
            };
            value = match value.$addsub(as_cast(digit)) {
                Some(v) => v,
                None => return into_error_v2!($overflow),
            };
        }

        Ok(value)
    }};
}

//#[inline]
//pub fn parse_digits<'a, T>(
//    bytes: &[u8],
//    radix: u32,
//    is_negative: bool,
//) -> Result<(T, usize)>
//where
//    T: Integer,
//{
//    let mut iter = bytes.iter();
//    if T::IS_SIGNED && is_negative {
//        parse_digits!(iter, radix, checked_sub, Underflow, T)
//    } else {
//        parse_digits!(iter, radix, checked_add, Overflow, T)
//    }
//}

#[inline]
pub fn algorithm<'a, T>(mut bytes: &[u8]) -> Result<T, lexical_util::error::ErrorCode>
where
    T: Integer,
{
    if bytes.is_empty() {
        return into_error_v2!(Empty);
    }
    let is_negative = match bytes.get(0) {
        Some(&b'+') => {
            bytes = unsafe { bytes.get_unchecked(1..) };
            false
        },
        Some(&b'-') if T::IS_SIGNED => {
            bytes = unsafe { bytes.get_unchecked(1..) };
            true
        },
        _ => false,
    };
    if bytes.is_empty() {
        return into_error_v2!(Empty);
    }
    let mut value = T::ZERO;
//    if is_positive {
//        // The number is positive
//        for &c in digits {
//            let x = match (c as char).to_digit(radix) {
//                Some(x) => x,
//                None => return Err(PIE { kind: InvalidDigit }),
//            };
//            result = match result.checked_mul(radix) {
//                Some(result) => result,
//                None => return Err(PIE { kind: PosOverflow }),
//            };
//            result = match result.checked_add(x) {
//                Some(result) => result,
//                None => return Err(PIE { kind: PosOverflow }),
//            };
//        }
//    } else {
//        // The number is negative
//        for &c in digits {
//            let x = match (c as char).to_digit(radix) {
//                Some(x) => x,
//                None => return Err(PIE { kind: InvalidDigit }),
//            };
//            result = match result.checked_mul(radix) {
//                Some(result) => result,
//                None => return Err(PIE { kind: NegOverflow }),
//            };
//            result = match result.checked_sub(x) {
//                Some(result) => result,
//                None => return Err(PIE { kind: NegOverflow }),
//            };
//        }
//    }
//    Ok(result)
    if T::IS_SIGNED && is_negative {
        for &c in bytes {
            let x = match (c as char).to_digit(10) {
                Some(x) => x,
                None => return into_error_v2!(Empty),
            };
            value = match value.checked_mul(as_cast(10)) {
                Some(value) => value,
                None => return into_error_v2!(Empty),
            };
            value = match value.checked_sub(as_cast(x)) {
                Some(value) => value,
                None => return into_error_v2!(Empty),
            };
        }
    } else {
        for &c in bytes {
            let x = match (c as char).to_digit(10) {
                Some(x) => x,
                None => return into_error_v2!(Empty),
            };
            value = match value.checked_mul(as_cast(10)) {
                Some(value) => value,
                None => return into_error_v2!(Empty),
            };
            value = match value.checked_add(as_cast(x)) {
                Some(value) => value,
                None => return into_error_v2!(Empty),
            };
        }
    }
    Ok(value)
}

// Implement FromLexical for numeric type.
macro_rules! integer_from_lexical {
    ($($t:tt $(, #[$meta:meta])? ; )*) => ($(
        impl FromLexical for $t {
            $(#[$meta:meta])?
            #[inline]
            fn from_lexical(bytes: &[u8]) -> Result<Self, lexical_util::error::ErrorCode>
            {
                algorithm(bytes)
                //lexical_partial_to_complete!(Self::from_lexical_partial, bytes)
            }

//            #[inline]
//            fn from_lexical_partial(
//                bytes: &[u8],
//            ) -> lexical_util::result::Result<(Self, usize)>
//            {
//                todo!();
//                //algorithm(bytes)
//            }
        }
    )*)
}

pub trait FromLexical: Sized {
    fn from_lexical(bytes: &[u8]) -> Result<Self, lexical_util::error::ErrorCode>;
}

//from_lexical! {}
integer_from_lexical! {
    u8 ;
    u16 ;
    u32 ;
    u64 ;
    u128 ;
    usize ;
    i8 ;
    i16 ;
    i32 ;
    i64 ;
    i128 ;
    isize ;
}
