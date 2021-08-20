# Big Integer Implementation

Our slow-path algorithms depend on efficient big-integer arithmetic. Fortunately, due to the relatively small size of big integers required to represent 64-bit and smaller floats, simple algorithms outperform asymptotically faster algorithms, leading to a simple implementation.

**Storage**

To avoid re-allocations and for use in a `no_std` environment, we use a limit stack-allocated vector implementation, with only methods to append, insert, and pop elements from the vector.

**Exponentiation**

The most computationally expensive part of big-integer arithmetic is exponentiation, which requires multiplication between big-integers and between scalar values and big-integers for efficiency. In order to minimize the static storage required, we use a single, pre-computed large power when the `compact` feature is not enabled, and all smaller powers iteratively multiply by scalar values.

First, we remove powers-of-two from the radix, since `10^30 == 5^30 * 2^30`. This allows us to use bitshifts, rather than multiplication, to multiply by powers-of-two, greatly improving performance. For the remaining, odd, powers, we use a simple exponentiation algorithm that is very efficient:

```rust,ignore
/// Step for large power-of-5 for 32-bit limbs.
pub const LARGE_POW5_STEP: u32 = 135;

/// 5^135, using 64-bit limbs.
pub const LARGE_POW5: [u64; 5] = [
    1414648277510068013,
    9180637584431281687,
    4539964771860779200,
    10482974169319127550,
    198276706040285095,
];

/// Pre-computed, small powers-of-5.
pub const SMALL_INT_POW5: [u64; 28] = [
    1,
    5,
    ...,
];

// This assumes a 64-bit limb.
pub fn pow(x: &mut StackVec, mut exp: u32) {
    let step = LARGE_POW5_STEP;
    let large = &LARGE_POW5;
    while exp >= step {
        large_mul(x, large);
        exp -= step;
    }

    // Now use our pre-computed small powers iteratively.
    let small_step = 27;
    let max_native = (5u64).pow(small_step);
    while exp >= small_step {
        small_mul(x, max_native);
        exp -= small_step;
    }
    if exp != 0 {
        let small_power = SMALL_INT_POW5[exp as usize] as u64;
        small_mul(x, small_power);
    }
}
```

This uses power-reduction, and multiplication by iterative powers for efficient exponentiation. Asymptotically faster algorithms, such as exponentiation by squaring, are much less efficient for our big integer sizes.

**Multiplication**

For big-integer multiplication, required for exponentiation, two main algorithms exist: Karatsuba multiplication, and grade school multiplication. Karatsuba is asymptotically better, and reduces the number of multiplications from `N^2` to `N^1.58`, where `N` is the number of digits. Grade school multiplication, however, is dead simple, and much faster for our small big integers. Asymptotically faster algorithms, such as Toom-Cook, require significant more digits to be efficient, and therefore never are efficient.

In practice, grade school multiplication is as follows:

```rust,ignore
/// Grade-school multiplication algorithm.
pub fn long_mul(x: &[u64], y: &[u64]) -> StackVec {
    let mut z = StackVec::try_from(x).unwrap();
    if !y.is_empty() {
        let y0 = y[0];
        small_mul(&mut z, y0);

        for index in 1..y.len() {
            let yi = y[index];
            if yi != 0 {
                let mut zi = StackVec::try_from(x).unwrap();
                small_mul(&mut zi, yi);
                large_add_from(&mut z, &zi, index);
            }
        }
    }
    z
}
```

In short, all iterations multiply by a scalar value in the array `y`, and these two big-integers are added from the current offset, or analogous to `43 x 12` being equal to `2 x 43 + 10 x 43`.


**Single-Digit Division**

For our `byte_comp` algorithm, in order to iteratively shave digits off the big-integer, we adapt David Gay's `quorem` algorithm. First, we calculate the quotient from the two, largest limb in the big integers, and iteratively calculate the remainder for the limbs in the numerator. In the rare case that we underestimated the quotient, we add `1` to the quotient and iteratively adjust the numerator's limbs. In practice, this generally requires only 1 division and `N` multiplication operations per digit generated.

A simple, yet efficient, implementation of the algorithm is as follows:

```rust,ignore
/// Emit a single digit for the quotient and store the remainder in-place.
pub fn large_quorem(x: &mut StackVec, y: &[u64]) -> u64 {
    // Numerator is smaller the denominator, quotient always 0.
    let m = x.len();
    let n = y.len();
    if m < n {
        return 0;
    }

    // Calculate our initial estimate for q.
    let xm_1 = x[m - 1];
    let yn_1 = y[n - 1];
    let mut q = xm_1 / (yn_1 + 1);

    // Need to calculate the remainder if we don't have a 0 quotient.
    if q != 0 {
        let mut borrow: u128 = 0;
        let mut carry: u128 = 0;
        for j in 0..m {
            let yj = y[j] as u128;
            let p = yj * q as u128 + carry;
            carry = p >> 64;
            let xj = x[j] as u128;
            let t = xj.wrapping_sub(p & mask).wrapping_sub(borrow);
            borrow = (t >> 64) & 1;
            x[j] = t as u64;
        }
        x.normalize();
    }

    // Check if we under-estimated x.
    if compare(x, y) != cmp::Ordering::Less {
        q += 1;
        let mut borrow: u128 = 0;
        let mut carry: u128 = 0;
        for j in 0..m {
            let yj = y[j] as u128;
            let p = yj + carry;
            carry = p >> 64;
            let xj = x[j] as u128;
            let t = xj.wrapping_sub(p & mask).wrapping_sub(borrow);
            borrow = (t >> 64) & 1;
            x[j] = t as u64;
        }
        x.normalize();
    }

    q
}
```
