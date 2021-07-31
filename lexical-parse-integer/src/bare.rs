// Bare-bones implementation algorithm to avoid any abstractions.

use lexical_util::digit::char_to_digit_const;
use lexical_util::format::{NumberFormat, STANDARD};
use lexical_util::num::{as_cast, Integer};
use lexical_util::error::{Error, ErrorCode};
use lexical_util::iterator::{Byte, ByteIter};
use lexical_util::noskip::AsNoSkip;
//use lexical_util::result::Result;
use lexical_util::from_lexical;

macro_rules! into_error {
    ($code:ident, $index:expr) => {
        Err((ErrorCode::$code, $index).into())
    };
}

// Complete parsers
macro_rules! into_ok_value {
    ($value:ident, $index:expr) => {
        Ok($value)
    };
}

// Partial parsers.
macro_rules! into_ok_index {
    ($value:ident, $index:expr) => {
        Ok(($value, $index))
    };
}

// Complete parsers.
macro_rules! invalid_digit_err {
    ($value:ident, $index:expr) => {
        into_error!(InvalidDigit, $index)
    };
}

// Partial parsers
macro_rules! invalid_digit_ok {
    ($value:ident, $index:expr) => {
        into_ok_index!($value, $index)
    };
}

macro_rules! parse_sign {
    ($iter:ident) => (
        // TODO(ahuszagb) This should work in **all** cases.
        // See asm_v14 for reasons why. Now we need to add in the
        // optimizations for fast digit parsing.
        match $iter.peek() {
            Some(&b'+') => {
                (false, 1)
            },
            Some(&b'-') if T::IS_SIGNED => {
                (true, 1)
            },
            _ => (false, 0),
        }
    );
}

/// Parse digits for a positive or negative value.
/// Optimized for operations with machine integers.
macro_rules! parse_digits {
    (
        $value:ident,
        $iter:ident,
        $radix:expr,
        $addsub:ident,
        $overflow:ident,
        $invalid_digit:ident
    ) => {{
        // Do our slow parsing algorithm: 1 digit at a time.
        while let Some(&c) = $iter.next() {
            let digit = match char_to_digit_const(c, $radix) {
                Some(v) => v,
                None => return $invalid_digit!($value, $iter.cursor() - 1),
            };
            $value = match $value.checked_mul(as_cast($radix)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter.cursor() - 1),
            };
            $value = match $value.$addsub(as_cast(digit)) {
                Some(v) => v,
                None => return into_error!($overflow, $iter.cursor() - 1),
            };
        }
    }};
}

macro_rules! algorithm {
    (
        $bytes:ident,
        $format:ident,
        $invalid_digit:ident,
        $into_ok:ident
    ) => {{
        let format = NumberFormat::<{ $format }> {};
        // THIS...
        // ACTUALLY! WORKS
        //  DOCUMENT IT! NOW!!!!
        //      OMG!!!!! WE'RE GONNA LIVE!!!
        let mut byte = $bytes.noskip();
        let mut iter = byte.integer_iter();
        let (is_negative, shift) = parse_sign!(iter);
        unsafe { iter.step_by_unchecked(shift); }
        if ByteIter::is_empty(&iter) {
            return into_error!(Empty, shift);
        }

        let mut value = T::ZERO;
        if !T::IS_SIGNED || !is_negative {
            parse_digits!(value, iter, format.radix(), checked_add, Overflow, $invalid_digit);
        } else {
            parse_digits!(value, iter, format.radix(), checked_sub, Underflow, $invalid_digit);
        }
        $into_ok!(value, iter.length())
    }};
}

#[inline]
fn algorithm<'a, T, const FORMAT: u128>(bytes: &[u8]) -> Result<T, Error>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, invalid_digit_err, into_ok_value)
}

#[inline]
fn algorithm_partial<'a, T, const FORMAT: u128>(bytes: &[u8]) -> Result<(T, usize), Error>
where
    T: Integer,
{
    algorithm!(bytes, FORMAT, invalid_digit_ok, into_ok_index)
}

// Implement FromLexical for numeric type.
macro_rules! integer_from_lexical {
    ($($t:tt $(, #[$meta:meta])? ; )*) => ($(
        impl FromLexical for $t {
            $(#[$meta:meta])?
            #[inline]
            fn from_lexical(bytes: &[u8]) -> Result<Self, Error>
            {
                algorithm::<_, STANDARD>(bytes)
            }

            #[inline]
            fn from_lexical_partial(
                bytes: &[u8],
            ) -> Result<(Self, usize), Error>
            {
                algorithm_partial::<_, STANDARD>(bytes)
            }
        }
    )*)
}

//pub trait FromLexical: Sized {
//    fn from_lexical(bytes: &[u8]) -> Result<Self, Error>;
//    fn from_lexical_partial(bytes: &[u8]) -> Result<(Self, usize), Error>;
//}

from_lexical! {}
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
