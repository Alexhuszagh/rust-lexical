//! Large-to-scalar exponentiation operations.
//!
//! Modifies a big integer from a native scalar.

use crate::lib::mem;

use crate::util::config::*;
use crate::util::traits::*;
use crate::util::powers::*;

use super::small;
use super::large;

// POWER
// -----

/// Calculate x^n, using exponentiation by squaring.
///
/// This algorithm is slow, using `mul_power` should generally be preferred,
/// as although it's not asymptotically faster, it precalculates a lot
/// of results.
#[inline]
pub fn ipow<T>(x: &mut T, mut n: Limb)
where
    T: CloneableVecLike<Limb>,
{
    // Store `x` as 1, and switch `base` to `x`.
    let mut base = T::default();
    base.push(1);
    mem::swap(x, &mut base);

    // Do main algorithm.
    loop {
        if n.is_odd() {
            large::imul(x, &base);
        }
        n /= 2;

        // We need to break as a post-condition, since the real work
        // is in the `imul` and `mul` algorithms.
        if n.is_zero() {
            break;
        } else {
            base = large::mul(&base, &base);
        }
    }
}

/// Calculate x^n, using exponentiation by squaring.
#[inline]
#[allow(dead_code)]
pub fn pow<T>(x: &[Limb], n: Limb) -> T
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    ipow(&mut z, n);
    z
}

/// MulAssign by a power.
///
/// Theoretically...
///
/// Use an exponentiation by squaring method, since it reduces the time
/// complexity of the multiplication to ~`O(log(n))` for the squaring,
/// and `O(n*m)` for the result. Since `m` is typically a lower-order
/// factor, this significantly reduces the number of multiplications
/// we need to do. Iteratively multiplying by small powers follows
/// the nth triangular number series, which scales as `O(p^2)`, but
/// where `p` is `n+m`. In short, it scales very poorly.
///
/// Practically....
///
/// Exponentiation by Squaring:
///     running 2 tests
///     test bigcomp_f32_lexical ... bench:       1,018 ns/iter (+/- 78)
///     test bigcomp_f64_lexical ... bench:       3,639 ns/iter (+/- 1,007)
///
/// Exponentiation by Iterative Small Powers:
///     running 2 tests
///     test bigcomp_f32_lexical ... bench:         518 ns/iter (+/- 31)
///     test bigcomp_f64_lexical ... bench:         583 ns/iter (+/- 47)
///
/// Exponentiation by Iterative Large Powers (of 2):
///     running 2 tests
///     test bigcomp_f32_lexical ... bench:         671 ns/iter (+/- 31)
///     test bigcomp_f64_lexical ... bench:       1,394 ns/iter (+/- 47)
///
/// Even using worst-case scenarios, exponentiation by squaring is
/// significantly slower for our workloads. Just multiply by small powers,
/// in simple cases, and use precalculated large powers in other cases.
pub fn imul_power<T>(x: &mut T, radix: u32, n: u32)
where
    T: CloneableVecLike<Limb>,
{
    let small_powers = get_small_powers(radix);
    let large_powers = get_large_powers(radix);

    if n == 0 {
        // No exponent, just return.
        // The 0-index of the large powers is `2^0`, which is 1, so we want
        // to make sure we don't take that path with a literal 0.
        return;
    }

    // We want to use the asymptotically faster algorithm if we're going
    // to be using Karabatsu multiplication sometime during the result,
    // otherwise, just use exponentiation by squaring.
    let bit_length = 32 - n.leading_zeros().as_usize();
    debug_assert!(bit_length != 0 && bit_length <= large_powers.len());
    if x.len() + large_powers[bit_length - 1].len() < 2 * large::KARATSUBA_CUTOFF {
        // We can use iterative small powers to make this faster for the
        // easy cases.

        // Multiply by the largest small power until n < step.
        let step = small_powers.len() - 1;
        let power = small_powers[step];
        let mut n = n.as_usize();
        while n >= step {
            small::imul(x, power);
            n -= step;
        }

        // Multiply by the remainder.
        small::imul(x, small_powers[n]);
    } else {
        // In theory, this code should be asymptotically a lot faster,
        // in practice, our small::imul seems to be the limiting step,
        // and large imul is slow as well.

        // Multiply by higher order powers.
        let mut idx: usize = 0;
        let mut bit: usize = 1;
        let mut n = n.as_usize();
        while n != 0 {
            if n & bit != 0 {
                debug_assert!(idx < large_powers.len());
                large::imul(x, large_powers[idx]);
                n ^= bit;
            }
            idx += 1;
            bit <<= 1;
        }
    }
}

/// Mul by a power.
#[inline]
#[allow(dead_code)]
pub fn mul_power<T>(x: &[Limb], radix: u32, n: u32) -> T
where
    T: CloneableVecLike<Limb>,
{
    let mut z = T::default();
    z.extend_from_slice(x);
    imul_power(&mut z, radix, n);
    z
}
