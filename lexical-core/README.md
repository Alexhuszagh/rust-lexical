lexical-core
============

[![Build Status](https://api.travis-ci.org/Alexhuszagh/rust-lexical.svg?branch=master)](https://travis-ci.org/Alexhuszagh/rust-lexical)
[![Latest Version](https://img.shields.io/crates/v/lexical-core.svg)](https://crates.io/crates/lexical-core)

Low-level, FFI-compatible, lexical conversion routines for use in a `no_std` context. This crate by default does not use the Rust standard library. And, as of version 0.3, lexical-core uses minimal unsafe features, limiting the chance of memory-unsafe code.

- [Getting Started](#getting-started)
- [Features](#features)
- [Configuration](#configuration)
- [Constants](#constants)
- [FFI Example](#ffi-example)
- [Documentation](#documentation)
- [Validation](#validation)
- [Implementation Details](#implementation-details)
  - [Float to String](#float-to-string)
  - [String to Float](#string-to-float)
  - [Arbitrary-Precision Arithmetic](#arbitrary-precision-arithmetic)
  - [Algorithm Background and Comparison](#algorithm-background-and-comparison)
- [Known Issues](#known-issues)
- [Version Support](#version-support)
- [Changelog](#changelog)
- [License](#license)
- [Contributing](#contributing)

# Getting Started

lexical-core is a low-level, partially FFI-compatible API for number-to-string and string-to-number conversions, without requiring a system allocator. If you would like to use a convenient, high-level API, please look at [lexical](../lexical) instead.

Add lexical-core to your `Cargo.toml`:

```toml
[dependencies]
lexical-core = "^0.5"
```

And an introduction through use:

```rust
extern crate lexical_core;

// String to number using Rust slices.
// The argument is the byte string parsed.
let f = lexical_core::atof64(b"3.5").unwrap();   // 3.5
let i = lexical_core::atoi32(b"15").unwrap();    // 15

// String to number using pointer ranges, for FFI-compatible code.
// The first argument is a pointer to the start of the parsed byte array,
// and the second argument is a pointer to 1-past-the-end. It will process
// bytes in the range [first, last).
unsafe {
    // Get an FFI-compatible range.
    let bytes = b"3.5";
    let first = bytes.as_ptr();
    let last = first.add(bytes.len());
    // Get our result and extract our value using C-compatible functions.
    let res = lexical_core::ffi::atof64(first, last);
    let f = lexical_core::ffi::f64_result_ok(res); // Aborts if res is not ok.
}

// If and only if the `radix` feature is enabled, you may use the radix
// overloads to parse non-decimal floats and strings.
let f = lexical_core::atof32_radix(2, b"11.1").unwrap();   // 3.5
let i = lexical_core::atoi32_radix(2, b"1111").unwrap();   // 15

// The ato* and ffi::ato* parsers are checked, they validate the
// input data is entirely correct, and stop parsing when invalid data
// is found, or upon numerical overflow.
let r = lexical_core::atoi8(b"256"); // Err(ErrorCode::Overflow.into())
let r = lexical_core::atoi8(b"1a5"); // Err(ErrorCode::InvalidDigit.into())

// In order to extract and parse a number from a substring of the input
// data, use the ato*_partial and ffi::ato*_partial parsers.
// These functions return the parsed value and the number of processed
// digits, allowing you to extract and parse the number in a single pass.
let r = lexical_core::atoi8(b"3a5"); // Ok((3, 1))

// Lexical-core includes FFI functions to properly extract data and handle
// errors during routines. All the following functions may be used in
// external libraries, include from C.

unsafe {
    unsafe fn to_range(bytes: &'static [u8]) -> (*const u8, *const u8) {
        let first = bytes.as_ptr();
        let last = first.add(bytes.len());
        (first, last)
    }

    // Ideally, everything works great.
    let (first, last) = to_range(b"15");
    let res = lexical_core::ffi::atoi8(first, last);
    if lexical_core::ffi::i8_result_is_ok(res) {
        let i = lexical_core::ffi::i8_result_ok(res);
        assert_eq!(i, 15);
    }

    // However, it detects numeric overflow, returning an error with
    // an error code equal to `ErrorCode::Overflow`.
    let (first, last) = to_range(b"256");
    let res = lexical_core::ffi::atoi8(first, last);
    if lexical_core::ffi::i8_result_is_err(res) {
        let err = lexical_core::ffi::i8_result_err(res);
        assert_eq!(err.code, lexical_core::ffi::ErrorCode::Overflow);
    }

    // Errors occurring prematurely terminating the parser due to invalid
    // digits return the index in the buffer where the invalid digit was
    // seen. This may useful in contexts like serde, which require numerical
    // parsers from complex data without having to extract a substring
    // containing only numeric data ahead of time.
    let (first, last) = to_range(b"15 45");
    let res = lexical_core::ffi::atoi8(first, last);
    if lexical_core::ffi::i8_result_is_err(res) {
        let err = lexical_core::ffi::i8_result_err(res);
        assert_eq!(err.code, lexical_core::ffi::ErrorCode::InvalidDigit);
        assert_eq!(err.index, 2);
    }
}

// Number to string using slices.
// The first argument is the value, the second argument is the radix,
// and the third argument is the buffer to write to.
// The function returns a subslice of the original buffer, and will
// always start at the same position (`buf.as_ptr() == slc.as_ptr()`).
let mut buf = [b'0'; lexical_core::MAX_I64_SIZE];
let slc = lexical_core::i64toa(15, &mut buf);
assert_eq!(slc, b"15");

// If an insufficiently long buffer is passed, the serializer will panic.
// PANICS
let mut buf = [b'0'; 1];
//let slc = lexical_core::i64toa(15, &mut buf);

// In order to guarantee the buffer is long enough, always ensure there
// are at least `MAX_*_SIZE`, where * is the type name in upperase,
// IE, for `isize`, `MAX_ISIZE_SIZE`.
let mut buf = [b'0'; lexical_core::MAX_F64_SIZE];
let slc = lexical_core::f64toa(15.1, &mut buf);
assert_eq!(slc, b"15.1");

// When the `radix` feature is enabled, for base10 floats, using `MAX_*_SIZE`
// may significantly overestimate the space required to format the number.
// Therefore, the `MAX_*_SIZE_BASE10` constants allow you to get a much
// tighter bound on the space required.
let mut buf = [b'0'; lexical_core::MAX_F64_SIZE_BASE10];
let slc = lexical_core::f64toa(15.1, &mut buf);
assert_eq!(slc, b"15.1");
```

# Features

- `correct` Use a correct string-to-float parser. Enabled by default, and may be turned off by setting `default-features = false`. If neither `algorithm_m` nor `bhcomp` is enabled while `correct` is enabled, lexical uses the bigcomp algorithm.
- `trim_floats` Export floats without a fraction as an integer, for example, `0.0f64` will be serialized to "0" and not "0.0", and `-0.0` as "0" and not "-0.0".
- `radix` Enable lexical conversions to and from non-base10 representations. With radix enabled, any radix from 2 to 36 (inclusive) is valid, otherwise, only 10 is valid.
- `rounding` Enable the `FLOAT_ROUNDING` config variable to dictate how to round IEEE754 floats.
- `ryu` Use dtolnay's [ryu](https://github.com/dtolnay/ryu/) library for fast and accurate float-to-string conversions.

# Configuration

Lexical-core also includes configuration options that allow you to configure float processing and formatting:

- `NAN_STRING` The representation of Not a Number (NaN) as a string (default `b"NaN"`). For float parsing, lexical-core uses case-insensitive comparisons. This string **must** start with an `'N'` or `'n'`.
- `INF_STRING` The short, default representation of infinity as a string (default `b"inf"`). For float parsing, lexical-core uses case-insensitive comparisons. This string **must** start with an `'I'` or `'i'`.
- `INFINITY_STRING` The long, backup representation of infinity as a string (default `b"infinity"`). `INFINITY_STRING` must be at least as long as `INF_STRING`, and will only be used during float parsing. This string **must** start with an `'I'` or `'i'`.
- `EXPONENT_DEFAULT_CHAR` - The default character designating the exponent component of a float (default `b'e'`) for strings with a radix less than 15 (including decimal strings). For float parsing, lexical-core uses case-insensitive comparisons. This value should be not be in character set `[0-9a-eA-E.+\-]`.
- `EXPONENT_BACKUP_CHAR` - (radix only) The backup character designating the exponent component of a float (default `b'^'`) for strings with a radix greater than or equal to 15. This value should be not be in character set `[0-9a-zA-Z.+\-]`.
- `FLOAT_ROUNDING` - The IEEE754 float-rounding scheme to be used during float parsing. In almost every case, this should be set to `NearestTieEven`.

# Constants

Lexical-core also includes a few constants to simplify interfacing with number-to-string code. These are named `MAX_*_SIZE`, and indicate the maximum number of characters a number-to-string function may write. For example, `atoi32_range` may write up to `MAX_I32_SIZE` characters. When the radix feature is enabled, these constants may significantly overestimate the number of bytes required, so another set of constants named `MAX_*_SIZE_BASE10` are exported, signifying the maximum number of characters a number-to-string function may write in base 10. These are provided as Rust constants so they may be used as the size element in arrays. For FFI-code, lexical-core exports unmangled constants named `MAX_*_SIZE_FFI` and `MAX_*_SIZE_BASE10_FFI`, to allow their use in non-Rust code.

# FFI Example

First, build lexical-core in release mode from the project home:

```bash
cargo build --release
```

Next, add the shared library to the search path, or load it exactly. For example, to use lexical-core from Python, from the project home directory:

```python
from ctypes import *
import os

# This is the path on Unix, on Windows use *.dll and on MacOS X, use *.dylib.
path = os.path.join(os.getcwd(), "target", "release", "liblexical_core.so")
lib = CDLL(path)

# To access global variables, use $type.in_dll($lib, "$variable")
i8_size = c_size_t.in_dll(lib, "MAX_I8_SIZE_FFI")
print(i8_size)          # c_ulong(4)

exponent_char = c_char.in_dll(lib, "EXPONENT_DEFAULT_CHAR")
print(exponent_char)    # c_char(b'e')

# Define our result types for the error-checked parsers.
class error(Structure):
    _fields_ = [("code", c_int),
                ("index", c_size_t)]

class union_f32(Union):
    _fields_ = [("value", c_float),
                ("error", error)]

class result_f32(Structure):
    _fields_ = [("tag", c_uint),
                ("data", union_f32)]

# Need to set the appropriate restypes for our functions, Python assumes
# they're all `c_int`.
lib.f32_result_ffi_ok.restype = c_float
lib.atof32_range.restype = result_f32
lib.f32toa_range.restype = POINTER(c_char)

# Call string-to-number parsers. This isn't elegant, because we want
# a valid range of values, but it works.
def to_address(ptr):
    '''Get address from pointer.'''
    return cast(ptr, c_voidp).value

def to_ucharp(address):
    '''Get unsigned char* pointer from address or another pointer.'''
    return cast(address, POINTER(c_ubyte))

def distance(first, last):
    '''Calculate the distance between two ranges'''
    return to_address(last) - to_address(first)

data = b"1.2345"
first = to_ucharp(data)
last = to_ucharp(to_address(first) + len(data))
result = lib.f32_result_ffi_ok(lib.atof32_range(first, last))
print(result)               # 1.2345000505447388

# Call the number-to-string serializers.
# First, create a buffer type of sufficient length.
f32_size = c_size_t.in_dll(lib, "MAX_F32_SIZE_FFI")
F32BufferType = c_char * f32_size.value
buf = F32BufferType()

# Next, initialize our arguments for the call.
value = c_float(1.2345)
first = to_ucharp(buf)
last = to_ucharp(to_address(first) + len(buf))

# Call the serializer and create a Python string from our result.
ptr = lib.f32toa_range(value, first, last)
length = distance(first, ptr)
result = string_at(buf, 6)
print(result)               # "1.2345"
```

Further examples, and libraries to use lexical-core from FFI may be found in the [ffi](ffi) directory.

# Documentation

Lexical-core's documentation can be found on [docs.rs](https://docs.rs/lexical-core).

# Validation

Float parsing is difficult to do correctly, and major bugs have been found in implementations from [libstdc++'s strtod](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) to [Python](https://bugs.python.org/issue7632). In order to validate the accuracy of the lexical, we employ the following external tests:

1. Hrvoje Abraham's [strtod](https://github.com/ahrvoje/numerics/tree/master/strtod) test cases.
2. Rust's [test-float-parse](https://github.com/rust-lang/rust/tree/64185f205dcbd8db255ad6674e43c63423f2369a/src/etc/test-float-parse) unittests.
3. Testbase's [stress tests](https://www.icir.org/vern/papers/testbase-report.pdf) for converting from decimal to binary.
4. [Various](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) [difficult](https://www.exploringbinary.com/how-glibc-strtod-works/) [cases](https://www.exploringbinary.com/how-strtod-works-and-sometimes-doesnt/) reported on blogs.

Although lexical may contain bugs leading to rounding error, it is tested against a comprehensive suite of random-data and near-halfway representations, and should be fast and correct for the vast majority of use-cases.

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
7. **Slow Path** We use arbitrary-precision arithmetic to disambiguate the correct representation without any rounding error. We create an exact representation of the input digits as a big integer, to determine how to round the top 53 bits for the mantissa. If there is a fraction or a negative exponent, we create a representation of the significant digits for `b+h` and scale the input digits by the binary exponent in `b+h`, and scale the significant digits in `b+h` by the decimal exponent, and compare the two to determine if we need to round up or down.

Since arbitrary-precision arithmetic is slow and scales poorly for decimal strings with many digits or exponents of high magnitude, lexical also supports a lossy algorithm, which returns the result from the moderate path. The result from the lossy parser should be accurate to within 1 ULP.

## Arbitrary-Precision Arithmetic

Lexical uses arbitrary-precision arithmetic to exactly represent strings between two floating-point representations, and is highly optimized for performance. The following section is a comparison of different algorithms to determine the correct float representation. The arbitrary-precision arithmetic logic is not dependent on memory allocation: it only uses the heap when the `radix` feature is enabled.

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

**bigcomp**

Bigcomp is the canonical string-to-float parser, which creates an exact representation of `b+h` as a big integer, and compares the theoretical digits from `b+h` scaled into the range `[1, 10)` by a power of 10 to the actual digits in the input string (a more in-depth description can be found [here](https://www.exploringbinary.com/bigcomp-deciding-truncated-near-halfway-conversions/)). A maximum of 768 digits need to be compared to determine the correct representation, and the size of the big integers in the ratio does not depend on the number of digits in the input string.

Bigcomp is used as a fallback algorithm for lexical-core when the radix feature is enabled, since the radix-representation of a binary float may never terminate if the radix is not divisible by 2. Since bigcomp uses constant memory, it is used as the default algorithm if more than `2^15` digits are passed and the representation is potentially non-terminating.

**bhcomp**

Bhcomp is a simple, performant algorithm that compared the significant digits to the theoretical significant digits for `b+h`. Simply, the significant digits from the string are parsed, creating a ratio. A ratio is generated for `b+h`, and these two ratios are scaled using the binary and radix exponents.

For example, "2.470328e-324" produces a ratio of `2470328/10^329`, while `b+h` produces a binary ratio of `1/2^1075`. We're looking to compare these ratios, so we need to scale them using common factors. Here, we convert this to `(2470328*5^329*2^1075)/10^329` and `(1*5^329*2^1075)/2^1075`, which converts to `2470328*2^746` and `1*5^329`.

Our significant digits (real_digits) and `b+h` (bh_digits) therefore start like:
```
real_digits = 91438982...
bh_digits   = 91438991...
```

Since our real digits are below the theoretical halfway point, we know we need to round-down, meaning our literal value is `b`, or `0.0`. This approach allows us to calculate whether we need to round-up or down with a single comparison step, without any native divisions required. This is the default algorithm lexical-core uses.

**Other Optimizations**

1. We remove powers of 2 during exponentiation in bhcomp.
2. We limit the number of parsed digits to the theoretical max number of digits produced by `b+h` (768 for decimal strings), and merely compare any trailing digits to '0'. This provides an upper-bound on the computation cost.
3. We use fast exponentiation and multiplication algorithms to scale the significant digits for comparison.
4. For the fallback bigcomp algorithm, we use a division algorithm optimized for the generation of a single digit from a given radix, by setting the leading bit in the denominator 4 below the most-significant bit (in decimal strings). This requires only 1 native division per digit generated.
4. The individual "limbs" of the big integers are optimized to the architecture we compile on, for example, u32 on x86 and u64 on x86-64, minimizing the number of native operations required. Currently, 64-bit limbs are used on target architectures `aarch64`, `powerpc64`, `mips64`, and `x86_64`.

# Known Issues

On the ARMVv6 architecture, the stable exponentiation for the fast, incorrect float parser is not fully stable. For example, `1e-300` is correct, while `5e-324` rounds to `0`, leading to "5e-324" being incorrectly parsed as `0`. This does not affect the default, correct float parser, nor ARMVv7 or ARMVv8 (aarch64) architectures. This bug can compound errors in the incorrect parser (feature-gated by disabling the `correct` feature`). It is not known if this bug is an artifact of Qemu emulation of ARMv6, or is actually representative the hardware.

Versions of lexical-core prior to 0.4.3 could round parsed floating-point numbers with an error of up to 1 ULP. This occurred for strings with 16 or more digits and a trailing 0 in the fraction, the `b+h` comparison in the slow-path algorithm incorrectly scales the the theoretical digits due to an over-calculated real exponent. This affects a very small percentage of inputs, however, it is recommended to update immediately.

# Version Support

Lexical-core is tested to work from Rustc versions of 1.31.0-1.51.0, and should work on newer versions as well. Please report any errors compiling lexical-core for any Rust compiler 1.31.0 or later. Please note the test suite require a Rustc version of 1.34 or later.

# Changelog

All changes since 2.2.0 are documented in [CHANGELOG](CHANGELOG).

# License

Lexical-core is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

Lexical-core also ports some code from [rust](https://github.com/rust-lang/rust) (for backwards compatibility), [V8](https://github.com/v8/v8), [libgo](https://golang.org/src) and [fpconv](https://github.com/night-shift/fpconv), and therefore might be subject to the terms of a 3-clause BSD license or BSD-like license.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
