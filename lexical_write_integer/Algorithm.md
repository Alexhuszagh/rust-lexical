# Algorithm Approach

**Digit Counting**

Fast digit counting can remove the requirement to use intermediate buffers when writing digits, since the digits are written in reverse order, which can lead to dramatic performance improvements.

For values <= `u32`, we can use a fast digit counting algorithm described [here](https://lemire.me/blog/2021/06/03/computing-the-number-of-digits-of-an-integer-even-faster/). 

This depends on a fast log2 algorithm, which can then be used to compare to a pre-computed table to determine to round-up or down. Note that although all the algorithms are done for `u32`, in the actual implementation they use generic values.

```rust
#[inline]
pub fn fast_log2(x: u32) -> usize {
    32 - 1 - (x | 1).leading_zeros() as usize
}

#[inline]
pub fn fast_digit_count(x: u32) -> usize {
    const TABLE: [u64; 32] = [...];
    let shift = TABLE[fast_log2(x)];
    let count = (x as u64 + shift) >> 32;
    count as usize
}
```

However, this scales poorly for larger values, just due to the static storage required. Therefore, for `u64` and larger integers, we use the slightly slower, but still fast approximation via an integral log10.

```rust
#[inline]
pub fn fast_log10(x: u32) -> usize {
    let log2 = fast_log2(x);
    (log2 * 1233) >> 12
}

pub fn fallback_digit_count(x: u32) -> usize {
    const TABLE: [u128; 38] = [...];

    let log10 = fast_log10(x);
    let shift_up = log10 < TABLE.len() && x as u128 >= TABLE[log10];
    log10 + shift_up as usize + 1
}
```

The second algorithm is trivial to explain: we calculate a fast, integral log 10 of the value, which can be off by as much as 1, rounded down. We then therefore have a pre-computed table of all values, as 128-bit integers, and then determine if the value is smaller than the desired value.

**Power-of-4 Reduction**

The fastest algorithm by far seems to be a power-of-4 reduction, using a loop. This reduces the number of operations to 2 division/remainder operations per loop.

The algorithm writes backwards, to the end of a buffer, and then copies everything to start of the buffer, as expected.

```rust
let radix2 = radix * radix;
let radix4 = radix2 * radix2;
while value >= radix4 {
    let r = value % radix4;
    value /= radix4;
    let r1 = (2 * (r / radix2));
    let r2 = (2 * (r % radix2));

    index -= 1;
    buffer[index] = table[r2 + 1];
    index -= 1;
    buffer[index] = table[r2];
    index -= 1;
    buffer[index] = table[r1 + 1];
    index -= 1;
    buffer[index] = table[r1];
}

while value >= radix2 {
    let r = 2 * (value % radix2);
    value /= radix2;
    index -= 1;
    buffer[index] = table[r + 1];
    index -= 1;
    buffer[index] = table[r];
}

if value < radix {
    index -= 1;
    buffer[index] = value - b'0';
} else {
    let r = 2 * value;
    index -= 1;
    buffer[index] = table[r + 1];
    index -= 1;
    buffer[index] = table[r];
}
```

The major performance bottleneck, however, is the intermediate copy, which can slow down the algorithm by ~3x. The solution, of course, is to pre-compute the number of digits and therefore use no intermediate buffer.

**Manually Unrolling**

Another approach is to calculate the number of digits, and then to manually unroll the loops for a range of values at the cost of code size. Unfortunately, this isn't very fast in practice, even if it seems good on paper.

For example, we can unroll loops as follows:

```rust
/// Convert a value from `[100, 1000)` into a table offset.
#[inline]
fn sequential_index(v0: u32, v1: u32) -> usize {
    (2 * v0 - 200 * v1) as usize
}

/// Convert a value from `[10, 100)` into a table offset.
#[inline]
fn last_index(value: u32) -> usize {
    2 * value as usize
}

#[inline]
fn write_5(value: u32, buffer: &mut [u8]) {
    let v_0 = value;
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = last_index(v_2);
    buffer[4] = table[i_0 + 1];
    buffer[3] = table[i_0];
    buffer[2] = table[i_1 + 1];
    buffer[1] = table[i_1];
    buffer[0] = table[i_2 + 1];
}
```

This uses the fact that we know we have **exactly** 5 digits to remove any nested branching. However, this scales poorly for larger values, since we cannot enumerate every single possibility without prohibitive code bloat. For example, we must provide all values from 6-10 digits as if by 10 digits:

```rust
#[inline]
fn write_10(value: u64, buffer: &mut [u8]) {
    let t0 = (value / 100000000) as u32;
    let v_0 = (value as u32).wrapping_sub(t0.wrapping_mul(100000000));
    let v_1 = v_0 / 100;
    let v_2 = v_1 / 100;
    let v_3 = v_2 / 100;
    let v_4 = t0;
    let i_0 = sequential_index(v_0, v_1);
    let i_1 = sequential_index(v_1, v_2);
    let i_2 = sequential_index(v_2, v_3);
    let i_3 = last_index(v_3);
    let i_4 = last_index(v_4);

    buffer[9] = table[i_0 + 1];
    buffer[8] = table[i_0];
    buffer[7] = table[i_1 + 1];
    buffer[6] = table[i_1];
    buffer[5] = table[i_2 + 1];
    buffer[4] = table[i_2];
    buffer[3] = table[i_3 + 1];
    buffer[2] = table[i_3];
    buffer[1] = table[i_4 + 1];
    buffer[0] = table[i_4];
}

fn write_5_10(value: u64, buffer: &mut [u8]) {
    let mut digits: [u8; 16] = [b'0'; 16];
    write_10(value as u64, &mut digits);
    let count = fallback_digit_count(value);
    copy_to_dst(buffer, digits.get_unchecked(10 - count..10));
}
```

This removes any performance benefits of the removed branching, and makes it considerably slower than the simpler approach.
