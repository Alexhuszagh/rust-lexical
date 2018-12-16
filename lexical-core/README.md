lexical
=======

[![Build Status](https://api.travis-ci.org/Alexhuszagh/rust-lexical.svg?branch=master)](https://travis-ci.org/Alexhuszagh/rust-lexical)
[![Latest Version](https://img.shields.io/crates/v/lexical-core.svg)](https://crates.io/crates/lexical-core)

Low-level, FFI-compatible, lexical conversion routines for use in a `no_std` context. This crate by default does not use the Rust standard library.

- [Getting Started](#getting-started)
- [Features](#features)
- [Documentation](#documentation)
- [Validation](#validation)
- [Caveats](#caveats)
- [Details](#details)
  - [Float to String](#float-to-string)
  - [String to Float](#string-to-float)
  - [Arbitrary-Precision Arithmetic](#arbitrary-precision-arithmetic)
  - [Algorithm Background and Comparison](#algorithm-background-and-comparison)

# Getting Started

lexical-core is a low-level, partially FFI-compatible API for number-to-string and string-to-number conversions, without requiring a system allocator. If you would like to use a convenient, high-level API, please look at [lexical](../lexical) instead.

Add lexical-core to your `Cargo.toml`:

```yaml
[dependencies]
lexical-core = "0.1"
```

```rust
extern crate lexical_core;

// String to number using Rust slices.
// The first argument is the radix, which should be 10 for decimal strings,
// and the second argument is the byte string parsed.
let f = lexical_core::atof::atof64_slice(10, b"3.5");   // 3.5
let i = lexical_core::atoi::atoi32(10, b"15");          // 15

// String to number using pointer ranges, for FFI-compatible code.
// The first argument is the radix, which should be 10 for decimal strings,
// the second argument is a pointer to the start of the parsed byte array,
// and the third argument is a pointer to 1-past-the-end. It will process
// bytes in the range [first, last).
unsafe {
    let bytes = b"3.5";
    let first = bytes.as_ptr();
    let last = first.add(bytes.len());
    let f = lexical_core::atof::atof64_range(10, first, last);
}

// The ato*_slice and ato*_range parsers are not checked, they do not
// validate that the input data is entirely correct, and stop parsing
// when invalid data is founding, returning whatever was parsed up until
// that point. The explicit behavior is to wrap on overflow, and 
// to discard invalid digits.
let i = lexical_core::atoi::atoi8(10, b"256");          // 0, wraps from 256
let i = lexical_core::atoi::atoi8(10, b"1a5");          // 1, discards "a5"

// You should prefer the checked parsers, whenever possible. These detect 
// numeric overflow, and no invalid trailing digits are present.
// The error code for success is 0, all errors are less than 0.

// Ideally, everything works great.
let res = lexical_core::atoi::try_atoi8(10, b"15");
assert_eq!(res.error.code, lexical_core::ErrorCode::Success);
assert_eq!(res.value, 15);

// However, it detects numeric overflow, setting `res.error.code`
// to the appropriate value.
let res = lexical_core::atoi::try_atoi8(10, b"256");
assert_eq!(res.error.code == lexical_core::ErrorCode::Overflow);

// Errors occurring prematurely terminating the parser due to invalid 
// digits return the index in the buffer where the invalid digit was 
// seen. This may useful in contexts like serde, which require numerical
// parsers from complex data without having to extract a substring 
// containing only numeric data ahead of time. If the error is set
// to a `InvalidDigit`, the value is guaranteed to be accurate up until
// that point. For example, if the trailing data is whitespace,
// the value from an invalid digit may be perfectly valid in some contexts.
let res = lexical_core::atoi::try_atoi8(10, b"15 45");
assert_eq!(res.error.code == lexical_core::ErrorCode::InvalidDigit);
assert_eq!(res.error.index == 2);
assert_eq!(res.value == 15);

// Number to string using slices.
// The first argument is the value, the second argument is the radix,
// and the third argument is the buffer to write to.
// The function returns a subslice of the original buffer, and will
// always start at the same position (`buf.as_ptr() == slc.as_ptr()`).
let mut buf = [b'0'; lexical_core::MAX_I64_SIZE];
let slc = lexical_core::itoa::i64toa_slice(15, 10, &mut buf);
assert_eq!(slc, "15");

// If an insufficiently long buffer is passed, the serializer will panic.
let mut buf = b['0'; 1];
let slc = lexical_core::itoa::i64toa_slice(15, 10, &mut buf); // PANICS

// In order to guarantee the buffer is long enough, always ensure there
// are at least `MAX_XX_SIZE`, where XX is the type name in upperase,
// IE, for `isize`, `MAX_ISIZE_SIZE`.
let mut buf = [b'0'; lexical_core::MAX_F64_SIZE];
let slc = lexical_core::ftoa::f64toa_slice(15.1, 10, &mut buf);
assert_eq!(slc, "15.1");
```

# Features

- `algorithm_m` Use Algorithm M rather than bigcomp for the string-to-float parser. Algorithm M is ~4x faster than bigcomp, however, it may use significantly more memory. If and only if Algorithm M and radix are both active, lexical-core requires a system allocator.
- `trim_floats` Export floats without a fraction as an integer, for example, `0.0f64` will be serialized to "0" and not "0.0", and `-0.0` as "0" and not "-0.0".
- `radix` Enable lexical conversions to and from non-base10 representations. With radix enabled, any radix from 2 to 36 (inclusive) is valid, otherwise, only 10 is valid.
- `ryu` Use dtolnay's [ryu](https://github.com/dtolnay/ryu/) library for fast and accurate float-to-string conversions.

# Documentation

Lexical-core's documentation can be found on [docs.rs](https://docs.rs/lexical-core).

# Validation

Float parsing is difficult to do correctly, and major bugs have been found in implementations from [libstdc++'s strtod](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) to [Python](https://bugs.python.org/issue7632). In order to validate the accuracy of the lexical, we employ the following external tests:

1. Hrvoje Abraham's [strtod](https://github.com/ahrvoje/numerics/tree/master/strtod) test cases.
2. Rust's [test-float-parse](https://github.com/rust-lang/rust/tree/64185f205dcbd8db255ad6674e43c63423f2369a/src/etc/test-float-parse) unittests.
3. Testbase's [stress tests](https://www.icir.org/vern/papers/testbase-report.pdf) for converting from decimal to binary.
4. [Various](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) [difficult](https://www.exploringbinary.com/how-glibc-strtod-works/) [cases](https://www.exploringbinary.com/how-strtod-works-and-sometimes-doesnt/) reported on blogs.

Although lexical may contain bugs leading to rounding error, it is tested against a comprehensive suite of random-data and near-halfway representations, and should be fast and correct for the vast majority of use-cases.

Finally, due to the heavy use of unsafe code, lexical-core is fuzzed using cargo-fuzz, to avoid memory errors, and the unittests are periodically run under Valgrind.

# Caveats

Lexical uses unsafe code in the back-end for performance, and therefore may introduce memory-safety issues. Although the code is fuzzed and tested under Valgrind, no guarantees are made and you should use lexical-core at your own risk.

Finally, for non-decimal (base 10) floats, lexical's float-to-string implementation is lossy, resulting in rounding for a small subset of inputs (up to 0.1% of the total value).

# Implementation Details

## Float to String

For more information on the Grisu2 and Grisu3 algorithms, see [Printing Floating-Point Numbers Quickly and Accurately with Integers](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf).

For more information on the Ryu algorithm, see [Ryū: fast float-to-string conversion](https://dl.acm.org/citation.cfm?id=3192369).

## String to Float

In order to implement an efficient parser in Rust, lexical uses the following steps:

1. We ignore the sign until the end, and merely toggle the sign bit after creating a correct representation of the positive float.
2. We handle special floats, such as "NaN", "inf", "Infinity". If we do not have a special float, we continue to the next step.
3. We parse up to 64-bits from the string for the mantissa, ignoring any trailing digits, and parse the exponent (if present) as a signed 32-bit integer. If the exponent overflows or underflows, we set the value to i32::max_value() or i32::min_value(), respectively.
4. **Fast Path** We then try to create an exact representation of a native binary float from parsed mantissa and exponent. If both can be exactly represented, we multiply the two to create an exact representation, since IEEE754 floats mandate the use of guard digits to minimizing rounding error. If either component cannot be exactly represented as the native float, we continue to the next step.
5. **Moderate Path** We create an approximate, extended, 80-bit float type (64-bits for the mantissa, 16-bits for the exponent) from both components, and multiplies them together. This minimizes the rounding error, through guard digits. We then estimate the error from the parsing and multiplication steps, and if the float +/- the error differs significantly from b+h, we return the correct representation (b or b+u). If we cannot unambiguously determine the correct floating-point representation, we continue to the next step.
6. **Fallback Moderate Path** Next, we create a 128-bit representation of the numerator and denominator for b+h, to disambiguate b from b+u by comparing the actual digits in the input to theoretical digits generated from b+h. This is accurate for ~36 significant digits from a 128-bit approximation with decimal float strings. If the input is less than or equal to 36 digits, we return the value from this step. Otherwise, we continue to the next step.
7. **Slow Path** We use arbitrary-precision arithmetic to disambiguate the correct representation without any rounding error.
    - **Default** We create an exact representation of the numerator and denominator for b+h, using arbitrary-precision integers, and determine which representation is accurate by comparing the actual digits in the input to the theoretical digits generated from b+h. This is accurate for any number of digits, and the required amount of memory does not depend on the number of digits.
    - **Algorithm M** We create an exact representation of the input digits as a big integer, to determine how to round the top 53 bits for the mantissa. If there is a fraction or a negative exponent, we create a big ratio of both the numerator and the denominator, and generate the significant digits from the exact quotient and remainder.

Since arbitrary-precision arithmetic is slow and scales poorly for decimal strings with many digits or exponents of high magnitude, lexical also supports a lossy algorithm, which returns the result from the moderate path. The result from the lossy parser should be accurate to within 1 ULP.

To use Algorithm M, use the feature `algorithm_m` when compiling lexical.

## Arbitrary-Precision Arithmetic

Lexical uses arbitrary-precision arithmetic to exactly represent strings between two floating-point representations with more than 36 digits, with various optimizations for multiplication and division relative to Rust's current implementation. The arbitrary-precision arithmetic logic is not dependent on memory allocation: the default slow-path algorithm only uses the stack, and Algorithm M only uses the heap when the `radix` feature is enabled.

## Algorithm Background and Comparison

For close-to-halfway representations of a decimal string `s`, where `s` is close between two representations, `b` and the next float `b+u`, arbitrary-precision arithmetic is used to determine the correct representation. This means `s` is close to `b+h`, where `h` is the halfway point between `b` and `b+u`.

For the following example, we will use the following values for our test case: 

* `s = 2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001e-324`
* `b = 0.0`
* `b+h = 2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125e-324`
* `b+u = 5e-324`

**Algorithm M**

Algorithm M represents the significant digits of a float as a fraction of arbitrary-precision integers (a more in-depth description can be found [here](https://www.exploringbinary.com/correct-decimal-to-floating-point-using-big-integers/)). For example, 1.23 would be 123/100, while 314.159 would be 314159/1000. We then scale the numerator and denominator by powers of 2 until the quotient is in the range `[2^52, 2^53)`, generating the correct significant digits of the mantissa. The use of Algorithm M may be enabled through the algorithm_m feature-gate, and tends to be more performant than bigcomp.

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

**bigcomp**

Bigcomp is a re-implementation of the canonical string-to-float parser, which creates an exact representation b+h as big integers, and compares the theoretical digits from `b+h` scaled into the range `[1, 10)` by a power of 10 to the actual digits in the input string (a more in-depth description can be found [here](https://www.exploringbinary.com/bigcomp-deciding-truncated-near-halfway-conversions/)). A maximum of 767 digits need to be compared to determine the correct representation, and the size of the big integers in the ratio does not depend on the number of digits in the input string.

**Comparison to dec2flt**

Rust's dec2flt also uses Algorithm M internally, however, numerous optimizations led to >100x performance improvements in lexical relative to dec2flt.
1. We scale the ratio using only 1-2 "iterations", without using a loop, by scaling the numerator to have 52 more bits than the numerator, and multiply the numerator by 2 if we underestimated the result.
2. We use an algorithm for basecase division that is optimized for arbitrary-precision integers of similar size (an implementation of Knuth's Algorithm D from "The Art of Computer Programming"), with a time complexity of `O(m)`, where m is the size of the denominator. In comparison, dec2flt uses restoring division, which is `O(n^2)`, where n is the size of the numerator. Furthermore, the restoring division algorithm iterates bit-by-bit and requires an `O(n)` comparison at each iteration. To put this into perspective, to calculate the quotient of a value of b+h close to 1e307, dec2flt requires ~140,000 native subtraction and comparison operations, while lexical requires ~96 multiplication and subtraction operations.
3. We limit the number of parsed digits to 767, the theoretical max number of digits produced by b+h, and merely compare any trailing digits to '0'. This provides an upper-bound on the computation cost.
4. The individual "limbs" of the big integers are comprised of integers the size of the architecture we compile on, for example, u32 on x86 and u64 on x86-64, minimizing the number of native operations required.

# License

Lexical-core is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

Lexical-core also ports some code from [V8](https://github.com/v8/v8), [libgo](https://golang.org/src) and [fpconv](https://github.com/night-shift/fpconv), and therefore might be subject to the terms of a 3-clause BSD license or BSD-like license.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
