lexical
=======

[![Build Status](https://api.travis-ci.org/Alexhuszagh/rust-lexical.svg?branch=master)](https://travis-ci.org/Alexhuszagh/rust-lexical)
[![Latest Version](https://img.shields.io/crates/v/lexical.svg)](https://crates.io/crates/lexical)

Fast lexical conversion routines for both std and no_std environments. Lexical provides routines to convert numbers to and from decimal strings. Lexical also supports non-base 10 numbers, for both integers and floats. Lexical is simple to use, focuses on performance and correctness, and exports only 10 functions in the high-level API.

**Table of Contents**

- [Getting Started](#getting-started)
- [Benchmarks](#benchmarks)
- [Documentation](#documentation)
- [Validation](#validation)
- [Caveats](#caveats)
- [Details](#details)
  - [Float to String](#float-to-string)
  - [String to Float](#string-to-float)
  - [Arbitrary-Precision Arithmetic](#arbitrary-precision-arithmetic)
  - [Comparison to Algorithm M and dtoa](#comparison-to-algorithm-m-and-dtoa)
- [License](#license)
- [Contributing](#contributing)

# Getting Started

Add lexical to your `Cargo.toml`:

```yaml
[dependencies]
lexical = "1"
```

And get started using lexical:

```rust
extern crate lexical;

// Number to string
lexical::to_string(3.0);            // "3.0", always has a fraction suffix, 
lexical::to_string(3);              // "3"
lexical::to_string_radix(3.0, 2);   // "11.0", non-base 10 representation.
lexical::to_string_radix(1.5, 2);   // "1.1"
lexical::to_string_radix(1.25, 2);  // "1.01"

// String to number.
let i: i32 = lexical::parse("3");            // 3, auto-type deduction.
let f: f32 = lexical::parse("3.5");          // 3.5
let d = lexical::parse::<f64, _>("3.5");     // 3.5, explicit type hints.
let d = lexical::try_parse::<f64, _>("3.5"); // Ok(3.5), error checking parse.
let d = lexical::try_parse::<f64, _>("3a");  // Err(Error(_)), failed to parse.
```

Lexical's parsers can be either error-checked and unchecked. The unchecked parsers continue to parse until they encounter invalid data or overflow, returning a number was successfully parsed up until that point. This is analogous to C's `strtod`, which may not be desirable for many applications. Therefore, lexical also includes checked parsers, which ensure the entire buffer is used while parsing, without discarding characters, and that the resulting number did not overflow. Upon erroring, the checked parsers will return the an enum indicating overflow or the index where the first invalid digit  was found.

```rust
// This will return Err(Error(ErrorKind::InvalidDigit(3))), indicating 
// the first invalid character occurred at the index 3 in the input 
// string (the space character).
let x: i32 = lexical::try_parse("123 456");

// This will return Ok(123), since that is the value found before invalid
// character was encountered.
let x: i32 = lexical::parse("123 456");
```

For floating-points, Lexical also includes `parse_lossy` and `try_parse_lossy`, which may lead to minor rounding error (relative error of ~1e-16) in rare cases (see [details](#details) for more information), without using slow algorithms that lead to serious performance degradation.

```rust
let x: f32 = lexical::parse_lossy("3.5");       // 3.5
let x: f32 = lexical::try_parse_lossy("3.5");   // Ok(3.5)
```

# Benchmarks

The following benchmarks measure the time it takes to convert 10,000 random values, for different types. The values were randomly generated using NumPy, and run in both std (rustc 1.29.2) and no_std (rustc 1.31.0) contexts (only std is shown) on an x86-64 Intel processor. More information on these benchmarks can be found in the [benches](benches) folder and in the source code for the respective algorithms. Adding the flags "target-cpu=native" and "link-args=-s" were also used, however, they minimally affected the relative performance difference between different lexical conversion implementations.

For cross-language benchmarks, the C++ benchmark was done using GCC 8.2.1 with libstdc++ using Google Benchmark and the `-O3` flag. The Python benchmark was done using IPython on Python 3.6.6. The Go benchmark was done using go1.10.4. All benchmarks used the same data.

For all the following benchmarks, lower is better.

**Float to String**

![ftoa benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/ftoa.png)

**Integer To String**

![itoa benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/itoa.png)

**String to Float**

![atof benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atof.png)

**String to Integer**

![atoi benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atoi.png)

**String to f32 Comprehensive**

![atof32 benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atof_digits_f32.png)

**String to f64 Comprehensive**

![atof64 benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atof_digits_f64.png)

**String to f64 Cross-Language Comparison**

Note: Rust was unable to parse the "stress" benchmark, producing an error result of `ParseFloatError { kind: Invalid }`.

![atof64 language benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atof_language_comparison_f64.png)

# Backends

For Float-To-String conversions, lexical uses one of three backends: an internal, Grisu2 algorithm, an external, Grisu3 algorithm, and an external, Ryu algorithm (~2x as fast).

# Documentation

Lexical's documentation can be found on [docs.rs](https://docs.rs/lexical).

# Validation

Float parsing is difficult to do correctly, and major bugs have been found in implementations from [libstdc++'s strtod](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) to [Python](https://bugs.python.org/issue7632). In order to validate the accuracy of the lexical, we employ the following external tests:

1. Hrvoje Abraham's [strtod](https://github.com/ahrvoje/numerics/tree/master/strtod) test cases.
2. Rust's [test-float-parse](https://github.com/rust-lang/rust/tree/64185f205dcbd8db255ad6674e43c63423f2369a/src/etc/test-float-parse) unittests.
3. Testbase's [stress tests](https://www.icir.org/vern/papers/testbase-report.pdf) for converting from decimal to binary.
4. [Various](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) [difficult](https://www.exploringbinary.com/how-glibc-strtod-works/) [cases](https://www.exploringbinary.com/how-strtod-works-and-sometimes-doesnt/) reported on blogs.

Although Lexical is likely to contain bugs leading to rounding error, it is tested against a comprehensive suite of random-data and near-halfway representations, and should be fast and correct for the vast majority of use-cases.

# Caveats

Lexical uses unsafe code in the back-end for performance, and therefore may introduce memory-safety issues. Although the code is tested with wide variety of inputs to minimize the risk of memory-safety bugs, no guarantees are made and you should use it at your own risk.

Finally, for non-decimal (base 10) floats, lexical's float-to-string implementation is lossy, resulting in rounding for a small subset of inputs (up to 0.1% of the total value).

# Details

## Float to String

For more information on the Grisu2 and Grisu3 algorithms, see [Printing Floating-Point Numbers Quickly and Accurately with Integers](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf).

For more information on the Ryu algorithm, see [Ryū: fast float-to-string conversion](https://dl.acm.org/citation.cfm?id=3192369).

## String to Float

For float parsing, lexical tries the following algorithms, using the first algorithm that produces a correct result:

1. Create an exact representation using a native floating-point type from the significant digits and exponent.
2. Create an approximate representation, exact within rounding error, using a custom floating-point type with 80-bits of precision, preventing rounding-error in the significant digits.
3. Create an exact representation of the significant digits using arbitrary-precision arithmetic, and create the closest native float from this representation.

Although using the 3<sup>rd</sup> algorithm only occurs when the estimated rounding error is greater than the difference between two floating-point representations (a rare occurrence), maliciously constructed input could force use of the 3<sup>rd</sup> algorithm. This has seriously implications for performance, since arbitrary-precision arithmetic scales poorly with large numbers, and can lead to performance degradation in excess of 100-1000 fold. The lossy float-parsing algorithms therefore avoid using arbitrary-precision arithmetic, and use a modified 2<sup>nd</sup> step with an extended floating-point type with 160-bits of precision, creating a nearly accurate native float (within 1 bit in the mantissa) without major performance regressions.

## Arbitrary-Precision Arithmetic

# TODO(ahuszagh) Change to the actual dtoa implementation...

Lexical uses custom arbitrary-precision arithmetic to exactly represent complex floats, with various optimizations for multiplication and division relative to Rust's current implementation. Lexical uses a small vector of `u32`s for internal storage (storing up to 1024 bits on the stack), and an `i32` to store the binary exponent. Given `i^X` is largest power that can be stored in a `u32`, we use the following optimizations:

1. During the parsing of digits, we use a simple optimization to minimize the number of arbitrary-precision operations required. Since `z+b(y+b(x+b(w))) == z+b(y) + b^2(x+b(w))`, we can parse `X-1` length segments of digits using native integers, multiply the big float by `i^(X-1)`, and then add the parsed digits to the big float. This never overflows, and results in `S/(X-1)` arbitrary-precision multiplications and additions, as opposed to `S` arbitrary-precision multiplications and additions, where `S` is the number of digits in the fraction component.
2. Multiplication or division by a power of 2 can be represented by an increment or decrement of the binary exponent.
3. Multiplication by `i^n` iteratively multiplies by the largest power that fits in a u32 (`i^X`) until the remaining exponent is less than or equal to `X`, and then multiplies by `i^r`, where `r` is the remainder, using precalculated powers.
4. Division by `i^n` first pads the underlying storage with 0s (to avoid intermediate rounding, which is described below), and then iteratively divides by the largest power that fits in a u32 (`i^X`) until the remaining exponent is less than or equal to `X`, and then divides by `i^r`, where `r` is the remainder, using precalculated powers.

Since rounding error may be introduced during division, lexical pads the big float with 0s to avoid any significant rounding error being introduced. Since we manually store the exponent, we can avoid denormal and subnormal results easily, without requiring both a numerator and a denominator. The number of bits required to avoid rounding for a given `i` was calculated using numerical simulations for `x/i^n ∀ n [1, 150], ∀ x {2, ..., 179424673}`, where `x` is in a set 39 primes meant to induce rounding error. The number of padding bits required to avoid introducing significant rounding error while dividing by `i^n` was found to be linear with `n`, and the change in the slope of the number of padding bits required at a given floating-point precision was also found to be linear. However, the number of padding bits required was not correlated to the number of bits in our numerator, signifying the number of bits required to pad our big float was well approximated solely by a linear function of the exponent at a given number of bits of precision. We therefore estimated the required number of bits to pad our big float at ~70 bits of precision in the resulting value (greater than the precision of single- and double-precision IEEE754 floats), to avoid introducing any significant rounding error in our big float representation.

These optimizations led to significant performance wins, with performance in the worst case rivaling libstdc++'s `strtod`, and significantly outperforming any other implementation.

## Comparison to Algorithm M and dtoa

For close-to-halfway representations of a decimal string `s`, where `s` is close between two representations, `b` and the next float `b+u`, arbitrary-precision arithmetic is used to determine the correct representation. This means `s` is close to `b+h`, where `h` is the halfway point between `b` and `b+u`.

For the following example, we will use the following values for our test case: 

* `s = 2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001e-324`
* `b = 0.0`
* `b+h = 2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125e-324`
* `b+u = 5e-324`

**Algorithm M**

Algorithm M is described [here](https://www.exploringbinary.com/correct-decimal-to-floating-point-using-big-integers/) in depth, and Rust's libcore uses it internally. First, Algorithm M converts both the fraction and the exponent to a fraction of big integers. For example, `3.14159e2` is the same as `314159e-3`, which means a numerator of `314159` and denominator of `1000`. Next, Algorithm M calculates the quotient and remainder of the numerator and denominator until the quotient is the same representation as the mantissa in the native float, for example, in range range `[2^52, 2^53)` for f64, or in the range `[2^23, 2^24)` for f32. If the quotient is below the range, multiply the numerator by 2, and decrease the binary exponent.  If the quotient is above the range, multiply the denominator by 2, and increase the binary exponent. Therefore, Algorithm M requires `N` bitshifts and ~`N^2`<sup>1</sup> division/modulus operations per iteration, where `N` is the number of native ints in the big integer.

A naive implementation, in Python, is as follows:

```python
def algorithm_m(num, b):
    # Ensure numerator >= 2**52
    bits = int(math.ceil(math.log2(num)))
    if bits <= 53:
        num <<= 53
        b -= 53

    # Track number of steps required (optional).
    steps = 0
    while True:
        steps += 1
        c = num//b
        if c < 2**52:
            b //= 2
        elif c >= 2**53:
            b *= 2
        else:
            break

    return (num, b, steps-1)
```

For example, to `s` into the range `[2^52, 2^53)`, we need a a numerator of `247....`, and a denominator of `10^1078`, requiring 1127 iterations to scale the value. Similarly, scaling `2.4703282292062327e-324` requires 1127 iterations to scale, showing how Algorithm M scales poorly, even for relatively simple inputs.

In practice, Algorithm M is too slow for production code, and well-established algorithms like [dtoa](https://www.ampl.com/netlib/fp/dtoa.c) use another approach.

<sup>1</sup> Faster multiplication algorithms are available, however, these algorithms are too slow for small values of `N`, such as in Rust's case.

**dtoa**

David M. Gay's `dtoa` implementation is the canonical string-to-float parser, and uses another approach for performance, described in depth [here](https://www.exploringbinary.com/bigcomp-deciding-truncated-near-halfway-conversions/). dtoa represents `b` as an N-bit integer (24 for f32, 53 for f64) and a binary exponent and calculates `b+h` from `b`. Finally, dtoa scales `b+h` by a power of 10 such that the calculated value would be from [0, 10), and creates a fraction of big integers. It then calculates the generated digits from `b+h` by iteratively using calculating the quotient (the digit) and remainder of the fraction, setting the numerator to `10*remainder`. When the digits present in `s` or `b+h` differ from each other, the correct representation is determined. Therefore, dtoa requires `N^2` division/modulus operations and `N` multiplication operations, where `N` is the number of native ints in the big integer, similar to Algorithm M. However, dtoa converges in the average case much faster than Algorithm M.

For example, differentiating `s` from `b+h` requires 756 iterations (close to the worst case of 768 iterations), a slight improvement over Algorithm M. However, differentiating `2.4703282292062327e-324` from `b+h` only requires 18 steps, dramatically faster than Algorithm M.

**Lexical**

Lexical departs from all of these approaches by using numerical simulations to calc


By contrast, lexical potentially requires more memory usage for padding bytes to avoid significant rounding error during division. 

# License

Lexical is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

Lexical also ports some code from [V8](https://github.com/v8/v8), [libgo](https://golang.org/src) and [fpconv](https://github.com/night-shift/fpconv), and therefore might be subject to the terms of a 3-clause BSD license or BSD-like license.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
