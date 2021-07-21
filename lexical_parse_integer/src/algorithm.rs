//! The algorithm definitions for the the string-to-integer conversions.

// /// Iterate over the digits and iteratively process them.
// TODO(ahuszagh) Should return the count, TBH...
//macro_rules! parse_digits {
//    ($value:ident, $iter:ident, $radix:ident, $op:ident, $code:ident) => {
//        while let Some(c) = $iter.next() {
//            let digit = match to_digit(*c, $radix) {
//                Some(v) => v,
//                None => return Ok(($value, c)),
//            };
//            $value = match $value.checked_mul(as_cast($radix)) {
//                Some(v) => v,
//                None => return Err((ErrorCode::$code, c)),
//            };
//            $value = match $value.$op(as_cast(digit)) {
//                Some(v) => v,
//                None => return Err((ErrorCode::$code, c)),
//            };
//        }
//    };
//}
