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

/// Calculate the exact exponent without overflow.
///
/// Remove the number of digits that contributed to the mantissa past
/// the dot.
#[inline]
pub(super) extern "C" fn normalize_exponent(exponent: i32, dot_shift: i32)
    -> i32
{
    match exponent.checked_sub(dot_shift) {
        Some(v) => v,
        None    => if dot_shift < 0 { i32::max_value() } else { i32::min_value() },
    }
}

// TESTS
// -----

#[cfg(test)]
mod test {
    use super::*;

    unsafe fn check_parse_exponent(base: u32, s: &str, tup: (i32, usize)) {
        let first = s.as_ptr();
        let last = first.add(s.len());
        let (v, p) = parse_exponent(base, first, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, p), tup.1);
    }

    #[test]
    fn parse_exponent_test() {
        let data = [
            (2, "e101101"),
            (3, "e1200"),
            (4, "e231"),
            (5, "e140"),
            (6, "e113"),
            (7, "e63"),
            (8, "e55"),
            (9, "e50"),
            (10, "e45"),
            (11, "e41"),
            (12, "e39"),
            (13, "e36"),
            (14, "e33"),
            (15, "^30"),
            (16, "^2d"),
            (17, "^2b"),
            (18, "^29"),
            (19, "^27"),
            (20, "^25"),
            (21, "^23"),
            (22, "^21"),
            (23, "^1m"),
            (24, "^1l"),
            (25, "^1k"),
            (26, "^1j"),
            (27, "^1i"),
            (28, "^1h"),
            (29, "^1g"),
            (30, "^1f"),
            (31, "^1e"),
            (32, "^1d"),
            (33, "^1c"),
            (34, "^1b"),
            (35, "^1a"),
            (36, "^19")
        ];
        unsafe {
            // basen
            for item in data.iter() {
                check_parse_exponent(item.0, item.1, (45, item.1.len()));
            }

            // invalid exponent character
            check_parse_exponent(28, "e1h", (0, 0));
            check_parse_exponent(10, "^45", (0, 0));

            // trailing characters
            check_parse_exponent(10, "e45 ", (45, 3));
            check_parse_exponent(10, "e45-", (45, 3));
            check_parse_exponent(10, "e45+", (45, 3));
            check_parse_exponent(10, "e45a", (45, 3));

            // positive
            check_parse_exponent(10, "e+45", (45, 4));

            // negative
            check_parse_exponent(10, "e-45", (-45, 4));

            // overflow
            check_parse_exponent(10, "e3000000000", (i32::max_value(), 11));
            check_parse_exponent(10, "e+3000000000", (i32::max_value(), 12));
            check_parse_exponent(10, "e-3000000000", (-i32::max_value(), 12));
        }
    }

    #[test]
    fn normalize_exponent_test() {
        assert_eq!(normalize_exponent(10, 5), 5);
        assert_eq!(normalize_exponent(0, 5), -5);
        assert_eq!(normalize_exponent(i32::max_value(), 5), i32::max_value()-5);
        assert_eq!(normalize_exponent(i32::max_value(), -5), i32::max_value());
        assert_eq!(normalize_exponent(i32::min_value(), 5), i32::min_value());
        assert_eq!(normalize_exponent(i32::min_value(), -5), i32::min_value()+5);
    }
}
