//! Parse sign and validate integer format.

use lexical_util::format::NumberFormat;
use lexical_util::iterator::ByteIter;
use lexical_util::num::Integer;
use lexical_util::result::Result;

/// Determines if the integer is negative and validates the input data.
///
/// This routine does the following:
///
/// 1. Parses the sign digit.
/// 2. Handles if positive signs before integers are not allowed.
/// 3. Handles negative signs if the type is unsigned.
/// 4. Handles if the sign is required, but missing.
/// 5. Handles if the iterator is empty, before or after parsing the sign.
/// 6. Handles if the iterator has invalid, leading zeros.
///
/// Returns if the value is negative, or any values detected when
/// validating the input.
#[inline]
pub fn parse_sign_and_validate<'a, T, Iter, const FORMAT: u128>(iter: &mut Iter) -> Result<bool>
where
    T: Integer,
    Iter: ByteIter<'a>,
{
    let format = NumberFormat::<FORMAT> {};
    let is_negative = match iter.peek() {
        Some(&b'+') if !format.no_positive_mantissa_sign() => {
            iter.next();
            false
        },
        Some(&b'-') if T::IS_SIGNED => {
            iter.next();
            true
        },
        Some(&b'+') => return into_error!(InvalidPositiveSign, iter),
        Some(&b'-') => return into_error!(InvalidNegativeSign, iter),
        Some(_) if !format.required_mantissa_sign() => false,
        Some(_) => return into_error!(MissingSign, iter),
        None => return into_error!(Empty, iter),
    };
    // Note: need to call as a trait function.
    //  The standard library may add an `is_empty` function for iterators.
    if ByteIter::is_empty(iter) {
        return into_error!(Empty, iter);
    }
    if format.no_integer_leading_zeros() && iter.peek() == Some(&b'0') {
        return into_error!(InvalidLeadingZeros, iter);
    }
    Ok(is_negative)
}
