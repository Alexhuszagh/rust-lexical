use lexical_util::num;

fn as_primitive<T: num::AsPrimitive>(t: T) {
    let _: u8 = t.as_u8();
    let _: u16 = t.as_u16();
    let _: u32 = t.as_u32();
    let _: u64 = t.as_u64();
    let _: u128 = t.as_u128();
    let _: usize = t.as_usize();
    let _: i8 = t.as_i8();
    let _: i16 = t.as_i16();
    let _: i32 = t.as_i32();
    let _: i64 = t.as_i64();
    let _: i128 = t.as_i128();
    let _: isize = t.as_isize();
    let _: f32 = t.as_f32();
    let _: f64 = t.as_f64();
}

#[test]
fn as_primitive_test() {
    as_primitive(1u8);
    as_primitive(1u16);
    as_primitive(1u32);
    as_primitive(1u64);
    as_primitive(1u128);
    as_primitive(1usize);
    as_primitive(1i8);
    as_primitive(1i16);
    as_primitive(1i32);
    as_primitive(1i64);
    as_primitive(1i128);
    as_primitive(1isize);
    as_primitive(1f32);
    as_primitive(1f64);
}

fn as_cast<T: num::AsCast>(t: T) {
    let _: i8 = num::as_cast(t);
    let _: i16 = num::as_cast(t);
    let _: i32 = num::as_cast(t);
    let _: i64 = num::as_cast(t);
    let _: i128 = num::as_cast(t);
    let _: isize = num::as_cast(t);
    let _: u8 = num::as_cast(t);
    let _: u16 = num::as_cast(t);
    let _: u32 = num::as_cast(t);
    let _: u64 = num::as_cast(t);
    let _: u128 = num::as_cast(t);
    let _: usize = num::as_cast(t);
    let _: f32 = num::as_cast(t);
    let _: f64 = num::as_cast(t);
}

#[test]
fn as_cast_test() {
    as_cast(1u8);
    as_cast(1u16);
    as_cast(1u32);
    as_cast(1u64);
    as_cast(1u128);
    as_cast(1usize);
    as_cast(1i8);
    as_cast(1i16);
    as_cast(1i32);
    as_cast(1i64);
    as_cast(1i128);
    as_cast(1isize);
    as_cast(1f32);
    as_cast(1f64);
}

fn check_number<T: num::Number>(x: T, mut y: T) {
    // Copy, partialeq, partialord
    let _ = x;
    assert!(x < y);
    assert!(x != y);

    // Operations
    let _ = y + x;
    let _ = y - x;
    let _ = y * x;
    let _ = y / x;
    let _ = y % x;
    y += x;
    y -= x;
    y *= x;
    y /= x;
    y %= x;

    // Conversions already tested.
}

#[test]
fn number_test() {
    check_number(1u8, 5);
    check_number(1u16, 5);
    check_number(1u32, 5);
    check_number(1u64, 5);
    check_number(1u128, 5);
    check_number(1usize, 5);
    check_number(1i8, 5);
    check_number(1i16, 5);
    check_number(1i32, 5);
    check_number(1i64, 5);
    check_number(1i128, 5);
    check_number(1isize, 5);
    check_number(1f32, 5.0);
    check_number(1f64, 5.0);
}

fn check_integer<T: num::Integer>(mut x: T) {
    // Copy, partialeq, partialord, ord, eq
    let _ = x;
    assert!(x > T::ONE);
    assert!(x != T::ONE);
    assert_eq!(x.min(T::ONE), T::ONE);
    assert_eq!(x.max(T::ONE), x);

    // Operations
    let _ = x + T::ONE;
    let _ = x - T::ONE;
    let _ = x * T::ONE;
    let _ = x / T::ONE;
    let _ = x % T::ONE;
    x += T::ONE;
    x -= T::ONE;
    x *= T::ONE;
    x /= T::ONE;
    x %= T::ONE;

    // Bitwise operations
    let _ = x & T::ONE;
    let _ = x | T::ONE;
    let _ = x ^ T::ONE;
    x &= T::ONE;
    x |= T::ONE;
    x ^= T::ONE;

    // Bit shifts
    let _ = x << 1i32;
    let _ = x >> 1i32;
    x <<= 1i32;
    x >>= 1i32;

    // Conversions already tested.
}

#[test]
fn integer_test() {
    check_integer(65u8);
    check_integer(65u16);
    check_integer(65u32);
    check_integer(65u64);
    check_integer(65u128);
    check_integer(65usize);
    check_integer(65i8);
    check_integer(65i16);
    check_integer(65i32);
    check_integer(65i64);
    check_integer(65i128);
    check_integer(65isize);
}

#[test]
fn ceil_divmod_test() {
    use lexical_util::num::Integer;

    assert_eq!(5usize.ceil_divmod(7), (1, -2));
    assert_eq!(0usize.ceil_divmod(7), (0, 0));
    assert_eq!(35usize.ceil_divmod(7), (5, 0));
    assert_eq!(36usize.ceil_divmod(7), (6, -6));
}

#[cfg(feature = "floats")]
fn check_float<T: num::Float>(mut x: T) {
    // Copy, partialeq, partialord
    let _ = x;
    assert!(x > T::ONE);
    assert!(x != T::ONE);

    // Operations
    let _ = x + T::ONE;
    let _ = x - T::ONE;
    let _ = x * T::ONE;
    let _ = x / T::ONE;
    let _ = x % T::ONE;
    let _ = -x;
    x += T::ONE;
    x -= T::ONE;
    x *= T::ONE;
    x /= T::ONE;
    x %= T::ONE;

    // Check functions
    let _ = x.to_bits();
    assert_eq!(T::from_bits(x.to_bits()), x);
    let _ = x.is_sign_positive();
    let _ = x.is_sign_negative();
    let _ = x.ln();
    let _ = x.floor();

    // Check properties
    let _ = x.to_bits() & T::SIGN_MASK;
    let _ = x.to_bits() & T::EXPONENT_MASK;
    let _ = x.to_bits() & T::HIDDEN_BIT_MASK;
    let _ = x.to_bits() & T::MANTISSA_MASK;
    assert!(T::from_bits(T::INFINITY_BITS).is_special());
}

#[test]
#[cfg(feature = "floats")]
fn float_test() {
    use lexical_util::num::Float;

    check_float(123f32);
    check_float(123f64);

    // b00000000000000000000000000000001
    let f: f32 = 1e-45;
    assert!(f.is_odd());
    assert!(f.next().is_even());
    assert!(f.next_positive().is_even());
    assert!(f.prev().is_even());
    assert!(f.prev_positive().is_even());
    assert!(f.round_positive_even().is_even());
    assert_eq!(f.prev().next(), f);
    assert_eq!(f.prev_positive().next_positive(), f);
    assert_eq!(f.round_positive_even(), f.next());

    // b00111101110011001100110011001101
    let f: f32 = 0.1;
    assert!(f.is_odd());
    assert!(f.next().is_even());
    assert!(f.next_positive().is_even());
    assert!(f.prev().is_even());
    assert!(f.prev_positive().is_even());
    assert!(f.round_positive_even().is_even());
    assert_eq!(f.prev().next(), f);
    assert_eq!(f.prev_positive().next_positive(), f);
    assert_eq!(f.round_positive_even(), f.next());

    // b01000000000000000000000000000000
    let f: f32 = 1.0;
    assert!(f.is_even());
    assert!(f.next().is_odd());
    assert!(f.next_positive().is_odd());
    assert!(f.prev().is_odd());
    assert!(f.prev_positive().is_odd());
    assert!(f.round_positive_even().is_even());
    assert_eq!(f.prev().next(), f);
    assert_eq!(f.prev_positive().next_positive(), f);
    assert_ne!(f.round_positive_even(), f.next());
}
