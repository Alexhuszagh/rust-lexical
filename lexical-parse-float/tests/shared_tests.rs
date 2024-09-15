use lexical_parse_float::float::ExtendedFloat80;
use lexical_parse_float::shared;
#[cfg(feature = "power-of-two")]
use lexical_util::format::NumberFormatBuilder;

#[test]
fn calculate_shift_test() {
    assert_eq!(shared::calculate_shift::<f64>(-63), 64);
    assert_eq!(shared::calculate_shift::<f64>(-15), 16);
    assert_eq!(shared::calculate_shift::<f64>(-8), 11);
    assert_eq!(shared::calculate_shift::<f64>(0), 11);
    assert_eq!(shared::calculate_shift::<f64>(50), 11);
}

#[test]
#[cfg(feature = "power-of-two")]
fn calculate_power2_test() {
    const BASE4: u128 = NumberFormatBuilder::from_radix(4);
    assert_eq!(shared::calculate_power2::<f64, BASE4>(-63, 5), 944);
    assert_eq!(shared::calculate_power2::<f64, BASE4>(-15, 5), 1040);
    assert_eq!(shared::calculate_power2::<f64, BASE4>(-8, 0), 1059);
    assert_eq!(shared::calculate_power2::<f64, BASE4>(-8, 5), 1054);
    assert_eq!(shared::calculate_power2::<f64, BASE4>(0, 5), 1070);
    assert_eq!(shared::calculate_power2::<f64, BASE4>(50, 5), 1170);
}

#[test]
fn log2_test() {
    assert_eq!(shared::log2(2), 1);
    assert_eq!(shared::log2(4), 2);
    assert_eq!(shared::log2(10), 1);
}

#[test]
fn starts_with_test() {
    assert_eq!(shared::starts_with(b"NaN".iter(), b"nAN".iter()), false);
    assert_eq!(shared::starts_with(b"nAN".iter(), b"nAN".iter()), true);
    assert_eq!(shared::starts_with(b"nAN1".iter(), b"nAN".iter()), true);
    assert_eq!(shared::starts_with(b"nAN1".iter(), b"nAN12".iter()), false);
}

#[test]
fn starts_with_uncased_test() {
    assert_eq!(shared::starts_with_uncased(b"NaN".iter(), b"nAN".iter()), true);
    assert_eq!(shared::starts_with_uncased(b"nAN".iter(), b"nAN".iter()), true);
    assert_eq!(shared::starts_with_uncased(b"nAN1".iter(), b"nAN".iter()), true);
    assert_eq!(shared::starts_with_uncased(b"nAN1".iter(), b"nAN12".iter()), false);
}

#[test]
fn round_test() {
    let mut fp = ExtendedFloat80 {
        mant: 9223372036854776832,
        exp: -10,
    };
    shared::round::<f64, _>(&mut fp, |f, s| {
        f.mant >>= s;
        f.exp += s;
    });
    assert_eq!(fp.mant, 0);
    assert_eq!(fp.exp, 1);

    let mut fp = ExtendedFloat80 {
        mant: 9223372036854776832,
        exp: -10,
    };
    shared::round::<f64, _>(&mut fp, |f, s| {
        f.mant >>= s;
        f.exp += s;
        // Round-up.
        f.mant += 1;
    });
    assert_eq!(fp.mant, 1);
    assert_eq!(fp.exp, 1);

    // Round-down
    let mut fp = ExtendedFloat80 {
        mant: 9223372036854776832,
        exp: -10,
    };
    shared::round::<f64, _>(&mut fp, |f, s| {
        shared::round_nearest_tie_even(f, s, |is_odd, is_halfway, is_above| {
            is_above || (is_odd && is_halfway)
        });
    });
    assert_eq!(fp.mant, 0);
    assert_eq!(fp.exp, 1);

    // Round up
    let mut fp = ExtendedFloat80 {
        mant: 9223372036854778880,
        exp: -10,
    };
    shared::round::<f64, _>(&mut fp, |f, s| {
        shared::round_nearest_tie_even(f, s, |is_odd, is_halfway, is_above| {
            is_above || (is_odd && is_halfway)
        });
    });
    assert_eq!(fp.mant, 2);
    assert_eq!(fp.exp, 1);

    // Round down
    let mut fp = ExtendedFloat80 {
        mant: 9223372036854778880,
        exp: -10,
    };
    shared::round::<f64, _>(&mut fp, shared::round_down);
    assert_eq!(fp.mant, 1);
    assert_eq!(fp.exp, 1);
}
