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
pub(super) fn parse_exponent<'a>(radix: u32, bytes: &'a [u8])
    -> (i32, &'a [u8])
{
    // Force a check that the distance is >= 2, so we ensure there's something
    // after the exponent. This fixes a regression discovered via proptest.
    // Safety: bytes.len() >= 2.
    if bytes.len() >= 2 && case_insensitive_equal(index!(bytes[0]), exponent_notation_char(radix)) {
        // Use atoi_sign so we can handle overflow differently for +/- numbers.
        // We care whether the value is positive.
        // Use i32::max_value() since it's valid in 2s complement for
        // positive or negative numbers, and will trigger a short-circuit.
        // Safety: bytes.len() >= 2.
        let bytes = &index!(bytes[1..]);
        let cb = atoi::unchecked::<i32>;
        let (exponent, sign, len, truncated) = atoi::filter_sign::<i32, _>(radix, bytes, cb);
        let exponent = match truncated.is_some() {
            true  => match sign {
                Sign::Negative => -i32::max_value(),
                Sign::Positive => i32::max_value(),
            },
            false => exponent,
        };

        // Safety: atoi always returns a value <= bytes.len().
        (exponent, &index!(bytes[len..]))
    } else {
        (0, bytes)
    }
}

/// Calculate the scientific notation exponent without overflow.
///
/// For example, 0.1 would be -1, and 10 would be 1 in base 10.
#[inline]
#[cfg(feature = "correct")]
pub(super) fn scientific_exponent(exponent: i32, integer_digits: usize, fraction_start: usize)
    -> i32
{
    if integer_digits == 0 {
        let fraction_start = fraction_start.try_i32_or_max();
        exponent.saturating_sub(fraction_start).saturating_sub(1)
    } else {
        let integer_shift = (integer_digits - 1).try_i32_or_max();
        exponent.saturating_add(integer_shift)
    }
}

/// Calculate the mantissa exponent without overflow.
///
/// Remove the number of digits that contributed to the mantissa past
/// the dot, and add the number of truncated digits from the mantissa,
/// to calculate the scaling factor for the mantissa from a raw exponent.
#[inline]
#[cfg(feature = "correct")]
pub(super) fn mantissa_exponent(raw_exponent: i32, fraction_digits: usize, truncated: usize)
    -> i32
{
    if fraction_digits > truncated {
        raw_exponent.saturating_sub((fraction_digits - truncated).try_i32_or_max())
    } else {
        raw_exponent.saturating_add((truncated - fraction_digits).try_i32_or_max())
    }
}

/// Calculate the integral ceiling of the binary factor from a basen number.
#[inline]
#[cfg(feature = "correct")]
pub(super) fn integral_binary_factor(radix: u32)
    -> u32
{
    debug_assert_radix!(radix);

    #[cfg(not(feature = "radix"))] {
        4
    }

    #[cfg(feature = "radix")] {
        match radix.as_i32() {
            2  => 1,
            3  => 2,
            4  => 2,
            5  => 3,
            6  => 3,
            7  => 3,
            8  => 3,
            9  => 4,
            10 => 4,
            11 => 4,
            12 => 4,
            13 => 4,
            14 => 4,
            15 => 4,
            16 => 4,
            17 => 5,
            18 => 5,
            19 => 5,
            20 => 5,
            21 => 5,
            22 => 5,
            23 => 5,
            24 => 5,
            25 => 5,
            26 => 5,
            27 => 5,
            28 => 5,
            29 => 5,
            30 => 5,
            31 => 5,
            32 => 5,
            33 => 6,
            34 => 6,
            35 => 6,
            36 => 6,
            // Invalid radix
            _  => unreachable!(),
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod test {
    use super::*;

    fn check_parse_exponent(radix: u32, s: &str, tup: (i32, usize)) {
        let (value, slc) = parse_exponent(radix, s.as_bytes());
        assert_eq!(value, tup.0);
        assert_eq!(distance(s.as_ptr(), slc.as_ptr()), tup.1);
    }

    #[test]
    fn parse_exponent_test() {
        // empty
        check_parse_exponent(10, "", (0, 0));

        // invalid exponent character
        #[cfg(feature = "radix")]
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

        // lowercase
        check_parse_exponent(10, "e45", (45, 3));
        check_parse_exponent(10, "e+45", (45, 4));
        check_parse_exponent(10, "e-45", (-45, 4));
        check_parse_exponent(10, "e20", (20, 3));
        check_parse_exponent(10, "e+20", (20, 4));
        check_parse_exponent(10, "e-20", (-20, 4));

        // uppercase
        check_parse_exponent(10, "E45", (45, 3));
        check_parse_exponent(10, "E+45", (45, 4));
        check_parse_exponent(10, "E-45", (-45, 4));
        check_parse_exponent(10, "E20", (20, 3));
        check_parse_exponent(10, "E+20", (20, 4));
        check_parse_exponent(10, "E-20", (-20, 4));

        // overflow
        check_parse_exponent(10, "e10000000000", (i32::max_value(), 12));
        check_parse_exponent(10, "e+10000000000", (i32::max_value(), 13));
        check_parse_exponent(10, "e-10000000000", (-i32::max_value(), 13));

        #[cfg(feature = "radix")] {
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
            // basen
            for item in data.iter() {
                check_parse_exponent(item.0, item.1, (45, item.1.len()));
            }

            // >= base15
            check_parse_exponent(15, "^20", (30, 3));
            check_parse_exponent(15, "^+20", (30, 4));
            check_parse_exponent(15, "^-20", (-30, 4));
        }
    }

    #[cfg(feature = "correct")]
    #[test]
    fn scientific_exponent_test() {
        // 0 digits in the integer
        assert_eq!(scientific_exponent(0, 0, 5), -6);
        assert_eq!(scientific_exponent(10, 0, 5), 4);
        assert_eq!(scientific_exponent(-10, 0, 5), -16);

        // >0 digits in the integer
        assert_eq!(scientific_exponent(0, 1, 5), 0);
        assert_eq!(scientific_exponent(0, 2, 5), 1);
        assert_eq!(scientific_exponent(0, 2, 20), 1);
        assert_eq!(scientific_exponent(10, 2, 20), 11);
        assert_eq!(scientific_exponent(-10, 2, 20), -9);

        // Underflow
        assert_eq!(scientific_exponent(i32::min_value(), 0, 0), i32::min_value());
        assert_eq!(scientific_exponent(i32::min_value(), 0, 5), i32::min_value());

        // Overflow
        assert_eq!(scientific_exponent(i32::max_value(), 0, 0), i32::max_value()-1);
        assert_eq!(scientific_exponent(i32::max_value(), 5, 0), i32::max_value());
    }

    #[cfg(feature = "correct")]
    #[test]
    fn mantissa_exponent_test() {
        assert_eq!(mantissa_exponent(10, 5, 0), 5);
        assert_eq!(mantissa_exponent(0, 5, 0), -5);
        assert_eq!(mantissa_exponent(i32::max_value(), 5, 0), i32::max_value()-5);
        assert_eq!(mantissa_exponent(i32::max_value(), 0, 5), i32::max_value());
        assert_eq!(mantissa_exponent(i32::min_value(), 5, 0), i32::min_value());
        assert_eq!(mantissa_exponent(i32::min_value(), 0, 5), i32::min_value()+5);
    }

    #[cfg(all(feature = "correct", feature = "radix"))]
    #[test]
    fn integral_binary_factor_test() {
        const TABLE: [u32; 35] = [1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6];
        for (idx, base) in (2..37).enumerate() {
            assert_eq!(integral_binary_factor(base), TABLE[idx]);
        }
    }
}
