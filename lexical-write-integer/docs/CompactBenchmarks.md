# Compact Benchmarks

These benchmarks were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.12.5/Fedora 34, and run against commit [961aefc](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/961aefc5d7c1f4eb8b10c043a585644bc891c832). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`. The exact code and data used to run the benchmark can be seen [here](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-benchmark/write-integer).

# Random

**Uniform**

A benchmark on randomly-generated numbers uniformly distributed over the entire range.

![Uniform Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/random_uniform_features=compact.svg)

**Simple**

A benchmark on randomly-generated, simple numbers to test writing numbers with few digits, Each number is in the range `[0, 1000]` (or `[0, 50]` for `u8`).

![Simple Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/random_simple_features=compact.svg)

**Large**

A benchmark on randomly-generated, large numbers to test writing numbers with many digits.

![Large Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/random_large_features=compact.svg)

**Simple Negative**

A benchmark on randomly-generated, simple, positive and negative numbers to test writing numbers with few digits. Each number is in the range `[-1000, 1000]` (or `[-50, 50]` for `u8`).

![Simple Negative Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/random_simple_signed_features=compact.svg)

**Large Negative**

A benchmark on randomly-generated, large, negative numbers to test writing negative numbers with many digits.

![Large Negative Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/random_large_signed_features=compact.svg)

# JSON

**Simple**

A benchmark on pre-computed, simple values generated via NumPy. The data was generated as follows:

```python
np.random.randint(0, 100, size=10000, dtype=np.uint8)
np.random.randint(0, 500, size=10000, dtype=np.uint16)
np.random.randint(0, 500, size=10000, dtype=np.uint32)
np.random.randint(0, 500, size=10000, dtype=np.uint64)
[random.randrange(0, 500) for _ in range(10000)]
```

![JSON Simple Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/json_simple_features=compact.svg)

**Random**

A benchmark on pre-computed, uniformly random values generated via NumPy. The data was generated as follows:

```python
np.random.randint(0, 255, size=10000, dtype=np.uint8)
np.random.randint(0, 65535, size=10000, dtype=np.uint16)
np.random.randint(0, 4294967295, size=10000, dtype=np.uint32)
np.random.randint(0, 18446744073709551615, size=10000, dtype=np.uint64)
[random.randrange(0, 2**128-1) for _ in range(10000)]

np.random.randint(-128, 127, size=10000, dtype=np.int8)
np.random.randint(-32768, 32767, size=10000, dtype=np.int16)
np.random.randint(-2147483648, 2147483647, size=10000, dtype=np.int32)
np.random.randint(-9223372036854775808, 9223372036854775807, size=10000, dtype=np.int64)
[random.randrange(-2**127, 2**127-1) for _ in range(10000)]
```

![JSON Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/json_random_features=compact.svg)

**Chained Random**

A benchmark on randomly writing simple or random data, using a PRNG to ensure the writing algorithm does not know if simple or random data is being parsed. This is mostly a precaution, in case an algorithm branches on the number of digits to avoid branch prediction from skewing the results.

![JSON Chained Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/json_chain_random_features=compact.svg)
