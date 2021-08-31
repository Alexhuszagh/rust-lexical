# Algorithm Approach

## Digit Parsing Optimizations

**Parsing Multiple Digits**

Normally, to parse 8 digits, we need at **least** 7 multiplies. However, for numeric strings with a radix <= 10, all digits are adjacent in the range `[0x30, 0x39]`, meaning we can validate all digits are valid using bitmasks or addition/subtraction operations, and can normalize the digits by subtracting `0x30`. For a detailed explanation of the algorithm, see ["Fast numeric string to int"](https://johnnylee-sde.github.io/Fast-numeric-string-to-int/) by Johnny Lee.

Once these digits are validated and normalized, we can scale the numbers from the range `0 <= N <= 9` to the range `0 <= Nn <= 99` with a single multiply. Likewise, we can go to `0 <= Nnnn <= 9999` and `0 <= Nnnnnnnn <= 99999999` in 2 and 3 multiplies, respectively.

This means we can parse 8 digits in 3 (rather than 7) multiplies, which scales well for input strings of any length, since we represent the significant digits using a 64-bit, unsigned integer.

**Overflow Checking**

Rather than do checked multiplication and additions in each loop, which increases the amount of branching, we can check after if numeric overflow has occurred by checking the number of parsed digits, and the resulting value. In order to check for potential overflow, we use the **maximum** number of digits, or `step` digits, that can always be parsed without overflow, and detect afterwards if we parsed more significant digits than that. In the case of potential overflow, we then re-parse the input string, up until `step` digits, and return the significant digits. By avoiding overflow checking while parsing on the first pass, we get significant performance improvements for common floats, while less common inputs have a trivial amount of additional overhead.

Our algorithm for parsing digits therefore is as follows, where the in-depth discussion of parsing multiple digits can be found in [lexical-parse-integer](https://github.com/Alexhuszagh/rust-lexical/tree/main/lexical-parse-integer).

```rust,ignore
use lexical_parse_integer::algorithm::try_parse_8digits;

/// Iteratively parse and consume digits in intervals of 8.
#[inline]
pub fn parse_8digits<'a>(iter: &mut slice::Iter<'a, u8>, mantissa: &mut u64)
{
    // Can do up to 2 iterations without overflowing, however, for large
    // inputs, this is much faster than any other alternative.
    while let Some(v) = try_parse_8digits::<u64, _, FORMAT>(&mut iter) {
        *mantissa = mantissa.wrapping_mul(100000000).wrapping_add(v);
    }
}

/// Iteratively parse and consume digits from bytes.
#[inline]
pub fn parse_digits<'a, Cb>(iter: &mut slice::Iter<'a, u8>, mut cb: Cb)
    Cb: FnMut(u32),
{
    while let Some(&c) = iter.as_slice().get(0) {
        match char_to_digit_const(c, radix) {
            Some(v) => cb(v),
            None => break,
        }
        iter.next();
    }
}

/// Parse a partial, non-special floating point number.
///
/// This creates a representation of the float as the
/// significant digits and the decimal exponent.
#[inline]
pub fn parse_partial_number(
    input: &[u8],
    is_negative: bool,
    options: &Options,
) -> Result<(Number, usize)> {
    ...

    let mut mantissa = 0_u64;
    let start = input.iter();
    let mut iter = start.clone();
    parse_8digits(&mut iter, &mut mantissa);
    parse_digits(&mut iter, |digit| {
        mantissa = mantissa.wrapping_mul(10).wrapping_add(digit as _);
    });

    ...
}
```

## String to Float

In order to implement an efficient parser in Rust, lexical uses the following steps:

1. We ignore the sign until the end, and merely toggle the sign bit after creating a correct representation of the positive float.
2. We parse up to 64-bits from the string for the mantissa, ignoring any trailing digits, and parse the exponent (if present) as a signed 32-bit integer. If the exponent overflows or underflows, we set the value to i32::max_value() or i32::min_value(), respectively. If we have no valid digits, we try to parse a special float, returning an error if both fail.
3. **Fast Path** We then try to create an exact representation of a native binary float from parsed mantissa and exponent. If both can be exactly represented, we multiply the two to create an exact representation, since IEEE754 floats mandate the use of guard digits to minimizing rounding error. If either component cannot be exactly represented as the native float, we continue to the next step.
4. **Moderate Path** These algorithms used an extended representation of a float as a 64-bit or larger representation, to scale the significant digits to the exponent with minimal intermediate rounding. If we can differentiate our representation from halfway points, we return the correct representation. If we cannot unambiguously determine the correct floating-point representation, we round to the `b` representation and go to the slow path algorithm.
5. **Slow Path** We use arbitrary-precision arithmetic to disambiguate the correct representation without any rounding error. We create an exact representation of the input digits as a big integer, to determine how to round the significant digits. If there is a fraction or a negative exponent, we create a representation of the significant digits for `b+h` and scale the input digits by the binary exponent in `b+h`, and scale the significant digits in `b+h` by the decimal exponent, and compare the two to determine if we need to round up or down.

Since arbitrary-precision arithmetic is slow and scales poorly for decimal strings with many digits or exponents of high magnitude, lexical also provides an option for a lossy algorithm, which will always return the parsed float within 1 ULP, rounded-down if it cannot be differentiated from a halfway representation. This amortizes the time required to parse a float, at the expense of accuracy.

# Halfway Cases

When parsing floats, the most significant problem is determining how to round the resulting value. The IEEE-754 standard specifies rounding to nearest, then tie even.

For example, using this rounding scheme to decimal numbers:
- `8.9` would round to `9.0`.
- `9.1` would round to `9.0`.
- `9.5` would round to `10.0`.
- `10.5` would round to `10.0`.

With parsing from decimal strings to binary, fixed-width floating point numbers, we must round to the nearest float. This becomes tricky when values are near their halfway point. For example, with a single-precision float `f32`, we would round as follows:
- `16777216.9` rounds to `16777216.0`
- `16777217.0` rounds to `16777216.0`
- `16777217.1` rounds to `16777218.0`

This is easier illustrated if we represent the float in binary. First, here's the layout of an IEEE-754 single-precision float as bits:

🟦🟩🟩🟩🟩🟩🟩🟩🟩🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪🟪

Where:
- 🟦 is the sign bit.
- 🟩 are the exponent bits.
- 🟪 are the mantissa, or significant digit, bits.

We'll ignore the exponent and sign bits right now, and only consider the mantissa, or significant digits. The lowest exponent bit, also called the hidden bit, is used as an implicit, extra bit of precision for normal floats, meaning we have 24-bits of precision. For 3 numbers, we would therefore have the following representations, where the last bit is truncated off:

- `16777216.0` => `100000000000000000000000 0`
- `16777217.0` => `100000000000000000000000 1`
- `16777218.0` => `100000000000000000000001 0`

Therefore, `16777217.0` is exactly halfway between `16777216.0` and `16777218.0`. Although solving these halfway cases can superficially seem easy, simple algorithms will fail even when parsing the shortest, accurate decimal representation.

## Moderate Path Algorithms

Due to the speed of the Eisel-Lemire algorithm, it is the preferred algorithm when performance is desired. However, due to the increased static storage required for the pre-computed powers-of-5, we use the Bellerophon algorithm when the `compact` feature is enabled, or for non-decimal floats.

**Eisel-Lemire Algorithm**

The Eisel-Lemire [algorithm](https://arxiv.org/abs/2101.11408) is a fast, moderate path algorithm that creates an extended, 128-bit approximation of the mantissa scaled to the decimal exponent. The algorithm is not correct for all cases, but is accurate and fast for the vast majority of cases, and does not report false positives. The scaling uses a pre-computed table of 128-bit powers of 10, computed for the entire range of valid `f64` values.

First, we create a 128-bit estimate by multiplying the 64-bit, normalized mantissa by the 64 high bits of the power of 10. We're currently within 2-units of the accurate representation, and if we cannot narrow this to a 1-unit range, then we expand to a 192-bit estimate, and proceed in a similar fashion to narrow to a 1-unit range. If the bits truncated after shifting would be a halfway point, the lower bits are all 0, then we round down to `b` and use the slow-path algorithm. This is very fast, because the major bottleneck is 1-2 64-bit multiply instructions.

**Bellerophon Algorithm**

The Bellerophon algorithm, described in "How to read floating point numbers accurately" by Clinger (available [here](https://dl.acm.org/doi/10.1145/989393.989430)) creates an 80-bit extended-precision representation of the float (64-bits for the mantissa, 16-bits for the binary exponent), and then multiplies this extended-precision float by two pre-computed powers of 10 (a small power, and a large power, to minimize storage requirements). It then attempts to calculate the rounded bits, or "slop", from the intermediate multiplications, and calculates if this inaccuracy is enough to overlap with a halfway representation.

Although quite fast, this requires 2-3 64-bit multiply instructions, and is ~20% slower for algorithms that can create correct representations from both algorithms. Although the two algorithms do not have identical coverage (in some cases, the Bellerophon algorithm is slightly more comprehensive), in practice, the increased performance of the Eisel-Lemire algorithm makes it superior for real-world data.

**Power-of-Two Algorithms**

When our radix is a power-of-two, we do not have to worry about any intermediate rounding while parsing our significant digits, nor of rounding when scaling the significant digits by the exponent, since both can be exactly represented by the binary float. The only case where our truncated representation could be a halfway representation is when the float is even and our truncated significant digits are **exactly** at a halfway representation. In order to disambiguate the two, we simply need to round-up if any truncated digits are non-zero.

## Arbitrary-Precision Arithmetic

Lexical uses arbitrary-precision arithmetic to exactly represent strings between two floating-point representations, and is highly optimized for performance. The following section is a comparison of different algorithms to determine the correct float representation. The arbitrary-precision arithmetic logic is not dependent on memory allocation, and therefore may always be used in a `no_std` environment.

## Algorithm Background and Comparison

For close-to-halfway representations of a decimal string `s`, where `s` is close between two representations, `b` and the next float `b+u`, arbitrary-precision arithmetic is used to determine the correct representation. This means `s` is close to `b+h`, where `h` is the halfway point between `b` and `b+u`.

For the following example, we will use the following values for our test case:

* `s = 2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125001e-324`
* `b = 0.0`
* `b+h = 2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125e-324`
* `b+u = 5e-324`

**Algorithm M**

Algorithm M represents the significant digits of a float as a fraction of arbitrary-precision integers (a more in-depth description can be found [here](https://www.exploringbinary.com/correct-decimal-to-floating-point-using-big-integers/)). For example, 1.23 would be 123/100, while 314.159 would be 314159/1000. We then scale the numerator and denominator by powers of 2 until the quotient is in the range `[2^52, 2^53)`, generating the correct significant digits of the mantissa.

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

**byte_comp**

`byte_comp` (or bigcomp) is the canonical string-to-float parser, which creates an exact representation of `b+h` as a big integer, and compares the theoretical digits from `b+h` scaled into the range `[1, 10)` by a power of 10 to the actual digits in the input string (a more in-depth description can be found [here](https://www.exploringbinary.com/bigcomp-deciding-truncated-near-halfway-conversions/)). A maximum of 768 digits need to be compared to determine the correct representation, and the size of the big integers in the ratio does not depend on the number of digits in the input string.

byte_comp is used as a fallback algorithm for lexical-core when the `radix` feature is enabled, since the representation of a binary float may never terminate if the radix is not divisible by 2. Since `byte_comp` incrementally generates digits from the theoretical digits of `b`, it is used as the default algorithm if the radix is odd.

**digit_comp**

`digit_comp` is a simple, performant algorithm using big-integer arithmetic for fast conversions from a big-integer representation to a native float.

For floats with positive exponents relative to the significant digits, this algorithm is trivial: scale the significant digits to the exponent, extract the high 64-bits, and round to the nearest float, using the presence of any non-zero, truncated digits to direct rounding.

For floats with negative exponents relative to the significant digits, we compare the significant digits to the theoretical significant digits for `b+h`. Simply, the significant digits from the string are parsed, creating a ratio. A ratio is generated for `b+h`, and these two ratios are scaled using the binary and radix exponents.

For example, "2.470328e-324" produces a ratio of `2470328/10^329`, while `b+h` produces a binary ratio of `1/2^1075`. We're looking to compare these ratios, so we need to scale them using common factors. Here, we convert this to `(2470328*5^329*2^1075)/10^329` and `(1*5^329*2^1075)/2^1075`, which converts to `2470328*2^746` and `1*5^329`.

Our significant digits (real digits) and `b+h` (theoretical digits) therefore start like:
```
real_digits = 91438982...
bh_digits   = 91438991...
```

Since our real digits are below the theoretical halfway point, we know we need to round-down, meaning our literal value is `b`, or `0.0`. This approach allows us to calculate whether we need to round-up or down with a single comparison step, without any native divisions required. This is the default algorithm lexical-core uses.

**decimal**

Other float-parsing algorithms, such as [go](https://github.com/golang/go/blob/master/src/strconv/decimal.go) use a decimal representation, which uses a fixed number of bytes to represent the significant digits of a float, with each byte corresponding to a single significant digit. This representation is then scaled to the range `(1/2 ... 1]`, and then the significant digits are generated, using iterative shifts. In practice, this is a fair bit faster than Algorithm M, however, it is much slower for inputs of all sizes than `byte_comp` or `digit_comp`.

**Other Optimizations**

1. We remove powers of 2 during exponentiation in `byte_comp`. For `digit_comp`, we use bitshifts rather than big-integer multiplication when multiplying by powers-of-two.
2. We limit the number of parsed digits to the theoretical max number of digits produced by `b+h` (768 for decimal strings), and merely compare any trailing digits to '0'. This provides an upper-bound on the computation cost.
3. We use fast exponentiation and multiplication algorithms to scale the significant digits for comparison.
4. For the fallback `byte_comp` algorithm, we use a division algorithm optimized for the generation of a single digit from a given radix, by setting the leading bit in the denominator 4 below the most-significant bit (in decimal strings). This requires only 1 native division per digit generated.
4. The individual "limbs" of the big integers are optimized to the architecture we compile on, for example, u32 on x86 and u64 on x86-64, minimizing the number of native operations required. Currently, 64-bit limbs are used on all 64-bit architectures except SPARCv8 and SPARCv9, where 128-bit multiplication is emulated rather than implemented in hardware.

## Limb Sizes

An important consideration in big-integer arithmetic is the size of the limbs we use, to ensure optimal performance for the native architecture. Currently, all tested 64-bit architectures besides SPARCv8 and SPARCv9 have support for 128-bit multiplication, or the ability to extract the high and low bits of a 64-bit multiplication in 2 steps. This is considerably more efficient than using 32-bit limbs, and so on those architectures we default to 64-bit limbs, and on all else we fallback to 32-bit limbs.

Therefore, the largest performance gains relative to the `decimal` implementation, which has similar relative performance on all architectures, is when using 64-bit limbs. Regardless, `digit_comp` is faster than the `decimal` algorithm in all cases: ~5-9x faster for 64-bit limbs, and ~3x-5x faster for 32-bit limbs. This difference makes quite a lot of sense when emulating 128-bit multiplication with native 64-bit multiplication takes 3-4 multiplication instructions, which almost entirely explains the performance gap.

## Validation

Float parsing is difficult to do correctly, and major bugs have been found in implementations from [libstdc++'s strtod](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) to [Python](https://bugs.python.org/issue7632). In order to validate the accuracy of the lexical, we employ the following external tests:

1. Hrvoje Abraham's [strtod](https://github.com/ahrvoje/numerics/tree/master/strtod) test cases.
2. Rust's [test-float-parse](https://github.com/rust-lang/rust/tree/64185f205dcbd8db255ad6674e43c63423f2369a/src/etc/test-float-parse) unittests.
3. Testbase's [stress tests](https://www.icir.org/vern/papers/testbase-report.pdf) for converting from decimal to binary.
4. Nigel Tao's [tests](https://github.com/nigeltao/parse-number-fxx-test-data) extracted from test suites for Freetype, Google's double-conversion library, IBM's IEEE-754R compliance test, as well as numerous other curated examples.
5. [Various](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) [difficult](https://www.exploringbinary.com/how-glibc-strtod-works/) [cases](https://www.exploringbinary.com/how-strtod-works-and-sometimes-doesnt/) reported on blogs.

Although lexical may contain bugs leading to rounding error, it is tested against a comprehensive suite of random-data and near-halfway representations, and should be fast and correct for the vast majority of use-cases.
