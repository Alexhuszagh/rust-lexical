use core::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastrand::Rng;
use lexical_parse_float::bigint;
use lexical_parse_float::float::RawFloat;
use lexical_parse_float::limits::{u32_power_limit, u64_power_limit};

// ALGORITHMS

fn standard_pow(big: &mut bigint::Bigint, exp: u32) {
    big.pow(10, exp).unwrap();
}

fn small_pow(big: &mut bigint::Bigint, mut exp: u32) {
    let shift = exp as usize;
    // Mul pow5
    let small_step = if bigint::LIMB_BITS == 32 {
        u32_power_limit(5)
    } else {
        u64_power_limit(5)
    };
    let max_native = (5 as bigint::Limb).pow(small_step);
    while exp >= small_step {
        big.data.mul_small(max_native).unwrap();
        exp -= small_step;
    }
    if exp != 0 {
        // SAFETY: safe, since `exp < small_step`.
        let small_power = unsafe { f64::int_pow_fast_path(exp as usize, 5) };
        big.data.mul_small(small_power as bigint::Limb).unwrap();
    }

    // Mul pow2
    bigint::shl(&mut big.data, shift).unwrap();
}

fn standard_mul(big: &mut bigint::Bigint, y: &[bigint::Limb]) {
    big.data *= y;
}

macro_rules! index_unchecked {
    ($x:ident[$i:expr]) => {
        *$x.get_unchecked($i)
    };
}

macro_rules! index_unchecked_mut {
    ($x:ident[$i:expr]) => {
        *$x.get_unchecked_mut($i)
    };
}

pub fn scalar_sub(x: bigint::Limb, y: bigint::Limb) -> (bigint::Limb, bool) {
    x.overflowing_sub(y)
}

#[inline]
pub fn small_sub_from<const SIZE: usize>(
    x: &mut bigint::StackVec<SIZE>,
    y: bigint::Limb,
    start: usize,
) {
    let mut index = start;
    let mut carry = y;
    while carry != 0 && index < x.len() {
        let result = scalar_sub(unsafe { index_unchecked!(x[index]) }, carry);
        unsafe { index_unchecked_mut!(x[index]) = result.0 };
        carry = result.1 as bigint::Limb;
        index += 1;
    }
    x.normalize();
}

pub fn large_sub<const SIZE: usize>(x: &mut bigint::StackVec<SIZE>, y: &[bigint::Limb]) {
    if x.len() < y.len() {
        unsafe { x.set_len(0) };
        return;
    }

    let mut carry = false;
    for index in 0..y.len() {
        let xi = unsafe { &mut index_unchecked_mut!(x[index]) };
        let yi = unsafe { index_unchecked!(y[index]) };

        let result = scalar_sub(*xi, yi);
        *xi = result.0;
        let mut tmp = result.1;
        if carry {
            let result = scalar_sub(*xi, 1);
            *xi = result.0;
            tmp |= result.1;
        }
        carry = tmp;
    }

    if carry && x.len() > y.len() {
        small_sub_from(x, 1, y.len());
    } else if carry {
        unsafe { x.set_len(0) };
    } else {
        x.normalize();
    }
}

pub const KARATSUBA_CUTOFF: usize = 32;

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn karatsuba_split(
    x: &[bigint::Limb],
    index: usize,
) -> (&[bigint::Limb], &[bigint::Limb]) {
    let x0 = &index_unchecked!(x[..index]);
    let x1 = &index_unchecked!(x[index..]);
    (x0, x1)
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn karatsuba_mul<const SIZE: usize>(
    x: &[bigint::Limb],
    y: &[bigint::Limb],
) -> bigint::StackVec<SIZE> {
    if y.len() <= KARATSUBA_CUTOFF {
        bigint::long_mul(x, y).unwrap()
    } else if x.len() < y.len() / 2 {
        karatsuba_uneven_mul(x, y)
    } else {
        let m = y.len() / 2;
        let (xl, xh) = karatsuba_split(x, m);
        let (yl, yh) = karatsuba_split(y, m);
        let mut sumx = bigint::StackVec::<SIZE>::try_from(xl).unwrap();
        bigint::large_add(&mut sumx, xh);
        let mut sumy = bigint::StackVec::<SIZE>::try_from(yl).unwrap();
        bigint::large_add(&mut sumy, yh);
        let z0 = karatsuba_mul::<SIZE>(xl, yl);
        let mut z1 = karatsuba_mul::<SIZE>(&sumx, &sumy);
        let z2 = karatsuba_mul::<SIZE>(xh, yh);
        large_sub(&mut z1, &z2);
        large_sub(&mut z1, &z0);

        let mut result = bigint::StackVec::<SIZE>::new();
        result.try_extend(&z0).unwrap();
        bigint::large_add_from(&mut result, &z1, m);
        bigint::large_add_from(&mut result, &z2, 2 * m);

        result
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn karatsuba_uneven_mul<const SIZE: usize>(
    x: &[bigint::Limb],
    mut y: &[bigint::Limb],
) -> bigint::StackVec<SIZE> {
    let mut result = bigint::StackVec::new();
    result.try_resize(x.len() + y.len(), 0).unwrap();

    let mut start = 0;
    while !y.is_empty() {
        let m = x.len().min(y.len());
        let (yl, yh) = karatsuba_split(y, m);
        let prod = karatsuba_mul::<SIZE>(x, yl);
        bigint::large_add_from(&mut result, &prod, start);
        y = yh;
        start += m;
    }
    result.normalize();

    result
}

#[inline(always)]
pub fn large_mul<const SIZE: usize>(x: &mut bigint::StackVec<SIZE>, y: &[bigint::Limb]) {
    if y.len() == 1 {
        unsafe { x.mul_small(index_unchecked!(y[0])) };
    } else if x.len() < y.len() {
        *x = unsafe { karatsuba_mul(x, y) };
    } else {
        *x = unsafe { karatsuba_mul(y, x) };
    }
}

fn karatsuba_mul_algo(big: &mut bigint::Bigint, y: &[bigint::Limb]) {
    large_mul(&mut big.data, y);
}

// GENERATOR

#[inline(always)]
fn new_limb(rng: &Rng) -> bigint::Limb {
    if bigint::LIMB_BITS == 32 {
        rng.u32(..) as bigint::Limb
    } else {
        rng.u64(..) as bigint::Limb
    }
}

macro_rules! generator {
    (@pow $group:ident, $name:expr, $cb:ident) => {{
        $group.bench_function($name, |bench| {
            let mut big = bigint::Bigint::new();
            let seed = fastrand::u64(..);
            let rng = Rng::with_seed(seed);
            bench.iter(|| {
                unsafe { big.data.set_len(0) };
                big.data.try_push(new_limb(&rng)).unwrap();
                // Don't go any higher than 300.
                $cb(&mut big, rng.u32(1..300));
                black_box(&big);
            })
        });
    }};

    (@pow $group:ident) => {{
        generator!(@pow $group, "standard_pow", standard_pow);
        generator!(@pow $group, "small_pow", small_pow);
    }};

    (@mul $group:ident, $name:expr, $cb:ident) => {{
        $group.bench_function($name, |bench| {
            let mut big = bigint::Bigint::new();
            let seed = fastrand::u64(..);
            let rng = Rng::with_seed(seed);
            bench.iter(|| {
                unsafe { big.data.set_len(0) };
                // Don't go higher than 20, since we a minimum of 60 limbs.
                let count = rng.usize(1..20);
                for _ in 0..count {
                    big.data.try_push(new_limb(&rng)).unwrap();
                }
                let count = rng.usize(1..20);
                let mut vec: Vec<bigint::Limb> = Vec::new();
                for _ in 0..count {
                    vec.push(new_limb(&rng));
                }

                // Don't go any higher than 300.
                $cb(&mut big, &vec);
                black_box(&big);
            })
        });
    }};

    (@mul $group:ident) => {{
        generator!(@mul $group, "standard_mul", standard_mul);
        generator!(@mul $group, "karatsuba_mul", karatsuba_mul_algo);
    }};
}

fn mul(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("mul");
    group.measurement_time(Duration::from_secs(5));

    generator!(@mul group);
}

fn pow(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("pow");
    group.measurement_time(Duration::from_secs(5));

    generator!(@pow group);
}

criterion_group!(mul_benches, mul);
criterion_group!(pow_benches, pow);
criterion_main!(mul_benches, pow_benches);
