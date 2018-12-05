//! Algorithm to parse an exponent from a float string.

use lib::{mem, ptr};
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
pub(super) unsafe extern "C" fn parse_exponent(state: &mut ParseState, base: u32, last: *const u8)
    -> i32
{
    if state.curr != last && (*state.curr).to_ascii_lowercase() == exponent_notation_char(base).to_ascii_lowercase() {
        state.increment();

        // Turn off truncation before we parse the exponent, since we want to
        // determine if the truncation currently overflows. We also want to
        // ensure we don't lose the current truncation status.
        let mut trunc_tmp = ptr::null();
        mem::swap(&mut state.trunc, &mut trunc_tmp);

        // Use atoi_sign so we can handle overflow differently for +/- numbers.
        // We care whether the value is positive.
        // Use i32::max_value() since it's valid in 2s complement for
        // positive or negative numbers, and will trigger a short-circuit.
        let cb = atoi::unchecked::<i32>;
        let (exponent, sign) = atoi::filter_sign::<i32, _>(state, base, last, cb);
        let exponent = if state.is_truncated() { i32::max_value() } else { exponent };
        let exponent = if sign == -1 { -exponent } else { exponent };

        mem::swap(&mut state.trunc, &mut trunc_tmp);
        exponent
    } else {
        0
    }
}

/// Calculate the exact exponent without overflow.
///
/// Remove the number of digits that contributed to the mantissa past
/// the dot, and add the number of truncated digits from the mantissa.
#[inline]
#[cfg(any(test, not(feature = "imprecise")))]
pub(super) extern "C" fn normalize_exponent(exponent: i32, dot_shift: usize, truncated: usize)
    -> i32
{
    if dot_shift > truncated {
        unwrap_or_min(exponent.checked_sub((dot_shift - truncated).try_i32_or_max()))
    } else {
        unwrap_or_max(exponent.checked_add((truncated - dot_shift).try_i32_or_max()))
    }
}

/// Calculate the binary factor from a basen number.
#[inline]
#[cfg(any(test, not(feature = "imprecise")))]
pub(super) extern "C" fn binary_factor(base: u32)
    -> f64
{
    // logic error, disable in release builds
    debug_assert!(base >= 2 && base <= 36, "Numerical base must be from 2-36");

    #[cfg(feature = "table")] {
        const TABLE: [f64; 35] = [1.0, 1.584962500721156, 2.0, 2.321928094887362, 2.584962500721156, 2.807354922057604, 3.0, 3.169925001442312, 3.321928094887362, 3.4594316186372973, 3.584962500721156, 3.700439718141092, 3.807354922057604, 3.9068905956085187, 4.0, 4.087462841250339, 4.169925001442312, 4.247927513443585, 4.321928094887363, 4.392317422778761, 4.459431618637297, 4.523561956057013, 4.584962500721156, 4.643856189774724, 4.700439718141092, 4.754887502163468, 4.807354922057604, 4.857980995127572, 4.906890595608519, 4.954196310386875, 5.0, 5.044394119358453, 5.087462841250339, 5.129283016944966, 5.169925001442312];
        let idx: usize = as_cast(base - 2);
        unsafe { *TABLE.get_unchecked(idx) }
    }

    #[cfg(not(feature = "table"))] {
        (base as f64).log2()
    }
}

/// Calculate the binary exponent from a basen exponent.
/// Assume no possible overflow.
#[inline]
#[cfg(any(test, not(feature = "imprecise")))]
pub(super) extern "C" fn binary_exponent(base: u32, exponent: i32)
    -> i32
{
    if exponent == 0 {
        0
    } else if exponent > 0 {
        as_cast((exponent as f64 * binary_factor(base)).ceil())
    } else {
        as_cast((exponent as f64 * binary_factor(base)).floor())
    }
}

/// Calculate the basen factor from a binary number.
#[inline]
#[cfg(any(test, not(feature = "imprecise")))]
pub(super) extern "C" fn basen_factor(base: u32)
    -> f64
{
    // logic error, disable in release builds
    debug_assert!(base >= 2 && base <= 36, "Numerical base must be from 2-36");

    #[cfg(feature = "table")] {
        // Calculated as 1
        const TABLE: [f64; 35] = [1.0, 0.6309297535714575, 0.5, 0.43067655807339306, 0.38685280723454163, 0.3562071871080222, 0.3333333333333333, 0.31546487678572877, 0.3010299956639812, 0.2890648263178879, 0.27894294565112987, 0.27023815442731974, 0.26264953503719357, 0.2559580248098155, 0.25, 0.24465054211822604, 0.23981246656813146, 0.23540891336663824, 0.23137821315975915, 0.227670248696953, 0.22424382421757544, 0.22106472945750374, 0.21810429198553155, 0.21533827903669653, 0.21274605355336318, 0.2103099178571525, 0.20801459767650948, 0.20584683246043448, 0.2037950470905062, 0.20184908658209985, 0.2, 0.19823986317056053, 0.1965616322328226, 0.1949590218937863, 0.19342640361727081];
        let idx: usize = as_cast(base - 2);
        unsafe { *TABLE.get_unchecked(idx) }
    }

    #[cfg(not(feature = "table"))] {
        1.0 / binary_exponent(base)
    }
}

/// Calculate the basen exponent from a binary exponent.
/// Assume no possible overflow.
#[cfg(any(test, not(feature = "imprecise")))]
pub(super) extern "C" fn basen_exponent(base: u32, exponent: i32)
    -> i32
{
    if exponent == 0 {
        0
    } else if exponent > 0 {
        as_cast((exponent as f64 * basen_factor(base)).ceil())
    } else {
        as_cast((exponent as f64 * basen_factor(base)).floor())
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
        let mut state = ParseState::new(first);
        let v = parse_exponent(&mut state, base, last);
        assert_eq!(v, tup.0);
        assert_eq!(distance(first, state.curr), tup.1);
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

            // empty
            check_parse_exponent(10, "", (0, 0));

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

            // >= base15
            check_parse_exponent(15, "^20", (30, 3));
            check_parse_exponent(15, "^+20", (30, 4));
            check_parse_exponent(15, "^-20", (-30, 4));

            // overflow
            check_parse_exponent(10, "e10000000000", (i32::max_value(), 12));
            check_parse_exponent(10, "e+10000000000", (i32::max_value(), 13));
            check_parse_exponent(10, "e-10000000000", (-i32::max_value(), 13));
        }
    }

    #[test]
    fn normalize_exponent_test() {
        assert_eq!(normalize_exponent(10, 5, 0), 5);
        assert_eq!(normalize_exponent(0, 5, 0), -5);
        assert_eq!(normalize_exponent(i32::max_value(), 5, 0), i32::max_value()-5);
        assert_eq!(normalize_exponent(i32::max_value(), 0, 5), i32::max_value());
        assert_eq!(normalize_exponent(i32::min_value(), 5, 0), i32::min_value());
        assert_eq!(normalize_exponent(i32::min_value(), 0, 5), i32::min_value()+5);
    }

    // TODO(ahuszagh) Add tests for:
    //      binary_exponent
    //      basen_exponent
}
