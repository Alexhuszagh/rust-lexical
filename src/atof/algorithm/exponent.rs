//! Algorithm to parse an exponent from a float string.

use atoi;
use util::*;

/// Parse the exponential portion from a float-string, if we have an `(e|^)[+-]?\d+`.
///
/// On overflow, just return a comically large exponent, since we don't
/// care. It will lead to infinity regardless, and doesn't affect whether
/// the type is representable.
///
/// Returns the exponent and a pointer to the current buffer position.
///
/// The float string must be non-special, non-zero, and positive.
#[inline]
pub(super) unsafe extern "C" fn parse_exponent(base: u32, first: *const u8, last: *const u8)
    -> (i32, *const u8)
{
    let mut p = first;
    let dist = distance(first, last);
    if dist > 1 && (*p).to_ascii_lowercase() == exponent_notation_char(base) {
        p = p.add(1);
        // Use atoi_sign so we can handle overflow differently for +/- numbers.
        // We care whether the value is positive.
        // Use is32::max_value() since it's valid in 2s complement for
        // positive or negative numbers, and will trigger a short-circuit.
        let cb = atoi::unchecked::<i32>;
        let (exponent, p, overflow, sign) = atoi::filter_sign::<i32, _>(base, p, last, cb);
        let exponent = if overflow { i32::max_value() } else { exponent };
        let exponent = if sign == -1 { -exponent } else { exponent };
        (exponent, p)
    } else {
        (0, p)
    }
}
