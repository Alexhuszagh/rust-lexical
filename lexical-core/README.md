lexical-core
============

[![Build Status](https://api.travis-ci.org/Alexhuszagh/rust-lexical.svg?branch=master)](https://travis-ci.org/Alexhuszagh/rust-lexical)
[![Latest Version](https://img.shields.io/crates/v/lexical-core.svg)](https://crates.io/crates/lexical-core)
[![Rustc Version 1.37+](https://img.shields.io/badge/rustc-1.37+-lightgray.svg)](https://blog.rust-lang.org/2019/08/15/Rust-1.37.0.html)

Low-level, lexical conversion routines for use in a `no_std` context. This crate by default does not use the Rust standard library, and does not require a system allocator.

**Similar Projects**

If you want a minimal, stable, and compile-time friendly version of lexical-core's float-parsing algorithm, see [minimal-lexical](https://github.com/Alexhuszagh/minimal-lexical). For language bindings to lexical, see [lexical-capi](https://github.com/Alexhuszagh/rust-lexical/tree/master/lexical-capi).

**Table of Contents**

- [Getting Started](#getting-started)
- [Features](#features)
  - [Format](#format)
- [Options](#options)
- [Constants](#constants)
- [Documentation](#documentation)
- [Validation](#validation)
- [Implementation Details](#implementation-details)
  - [Float to String](#float-to-string)
  - [String to Float](#string-to-float)
  - [Halfway Cases](#halfway-cases)
  - [Eisel-Lemire vs. Extended Float](#eisel-lemire-vs-extended-float)
  - [Arbitrary-Precision Arithmetic](#arbitrary-precision-arithmetic)
  - [Algorithm Background and Comparison](#algorithm-background-and-comparison)
- [Known Issues](#known-issues)
- [Platform Support](#platform-support)
- [Versioning and Version Support](#versioning-and-version-support)
- [Changelog](#changelog)
- [License](#license)
- [Contributing](#contributing)

# Getting Started

lexical-core is a low-level API for number-to-string and string-to-number conversions, without requiring a system allocator. If you would like to use a convenient, high-level API, please look at [lexical](../lexical) instead.

Add lexical-core to your `Cargo.toml`:

```toml
[dependencies]
lexical-core = "^0.8.0"
```

And an introduction through use:

```rust
extern crate lexical_core;

// String to number using Rust slices.
// The argument is the byte string parsed.
let f: f32 = lexical_core::parse(b"3.5").unwrap();   // 3.5
let i: i32 = lexical_core::parse(b"15").unwrap();    // 15

// All lexical_core parsers are checked, they validate the
// input data is entirely correct, and stop parsing when invalid data
// is found, or upon numerical overflow.
let r = lexical_core::parse::<u8>(b"256"); // Err(ErrorCode::Overflow.into())
let r = lexical_core::parse::<u8>(b"1a5"); // Err(ErrorCode::InvalidDigit.into())

// In order to extract and parse a number from a substring of the input
// data, use `parse_partial`. These functions return the parsed value and
// the number of processed digits, allowing you to extract and parse the
// number in a single pass.
let r = lexical_core::parse_partial::<i8>(b"3a5"); // Ok((3, 1))

// If an insufficiently long buffer is passed, the serializer will panic.
// PANICS
let mut buf = [b'0'; 1];
//let slc = lexical_core::write::<i64>(15, &mut buf);

// In order to guarantee the buffer is long enough, always ensure there
// are at least `T::FORMATTED_SIZE` bytes, which requires the
// `lexical_core::Number` trait to be in scope.
use lexical_core::Number;
let mut buf = [b'0'; f64::FORMATTED_SIZE];
let slc = lexical_core::write::<f64>(15.1, &mut buf);
assert_eq!(slc, b"15.1");

// When the `radix` feature is enabled, for decimal floats, using
// `T::FORMATTED_SIZE` may significantly overestimate the space
// required to format the number. Therefore, the
// `T::FORMATTED_SIZE_DECIMAL` constants allow you to get a much
// tighter bound on the space required.
let mut buf = [b'0'; f64::FORMATTED_SIZE_DECIMAL];
let slc = lexical_core::write::<f64>(15.1, &mut buf);
assert_eq!(slc, b"15.1");
```

# Features

- **power_of_two** Allow conversions to and from non-decimal strings.
    <blockquote>With power_of_two enabled, the radixes <code>{2, 4, 8, 10, 16, and 32}</code> are valid, otherwise, only 10 is valid.</blockquote>
- **radix** Allow conversions to and from non-decimal strings.
    <blockquote>With radix enabled, any radix from 2 to 36 (inclusive) is valid, otherwise, only 10 is valid.</blockquote>
- **format** Customize accepted inputs for number parsing.
    <blockquote>With format enabled, the number format is dictated through the <code>NumberFormat</code> bitflags, which allow you to toggle how to parse a string into a number. Various flags including enabling digit separators, requiring integer or fraction digits, and toggling special values.</blockquote>
- **rounding** Enable custom rounding for IEEE754 floats.
    <blockquote>By default, lexical uses round-nearest, tie-even for float rounding (recommended by IEE754).</blockquote>
- **ryu** Use dtolnay's [ryu](https://github.com/dtolnay/ryu/) library for float-to-string conversions.
    <blockquote>Enabled by default, and may be turned off by setting <code>default-features = false</code>. Ryu is ~2x as fast as other float formatters. Note that enabling this feature disables most float-formatting options.</blockquote>
- **no_alloc** Do not use a system allocator.
    <blockquote>Enabled by default, and may be turned off by setting <code>default-features = false</code>. If the feature is turned off, storage for arbitrary-precision arithmetic will use dynamically-allocated memory rather than the stack.</blockquote>
- **lemire** Use the lemire float-parsing algorithm.
    <blockquote>Enabled by default, and may be turned off by setting <code>default-features = false</code>. The Eisel-Lemire is a fast float-parsing algorithm for most use-cases. See the <a href="https://github.com/Alexhuszagh/rust-lexical/tree/master/lexical-core#string-to-float">String to Float</a> section below for a more detailed explanation</blockquote>

In terms of the static array storage for pre-computed values (required for accuracy and performance), 6KB are required if neither `radix` nor `binary` is enabled, 11KB are required if `binary` is enabled, and 127KB are required if `radix` is enabled. This is due to pre-computed powers being required for accurate calculations, and cannot be avoided.

## Format

Every language has competing specifications for valid numerical input, meaning a number parser for Rust will incorrectly accept or reject input for different programming or data languages. For example:

```rust
extern crate lexical_core;

use lexical_core::*;

// Valid in Rust strings.
// Not valid in JSON.
let f: f64 = parse(b"3.e7").unwrap();                       // 3e7

// Let's only accept JSON floats.
let format = NumberFormat::JSON;
let options = ParseFloatOptions::builder()
    .format(Some(format))
    .build()
    .unwrap();
let f: f64 = parse_with_options(b"3.0e7", &options).unwrap();       // 3e7
let f: f64 = parse_with_options(b"3.e7", &options).unwrap();        // Panics!

// We can also allow digit separators, for example.
// OCaml, a programming language that inspired Rust,
// accepts digit separators pretty much anywhere.
let format = NumberFormat::OCAML_STRING;
let options = ParseFloatOptions::builder()
    .format(Some(format))
    .build()
    .unwrap();
let f: f64 = parse(b"3_4.__0_1").unwrap();                          // Panics!
let f: f64 = parse_with_options(b"3_4.__0_1", &options).unwrap();   // 34.01
```

The parsing specification is defined by `NumberFormat`, which provides pre-defined constants for over 40 programming and data languages. However, it also allows you to create your own specification, to dictate parsing.

```rust
extern crate lexical_core;

use lexical_core::*;

// Let's use the standard, Rust grammar.
let format = NumberFormat::STANDARD;

// Let's use a permissive grammar, one that allows anything besides
// digit separators.
let format = NumberFormat::PERMISSIVE;

// Let's ignore digit separators and have an otherwise permissive grammar.
let format = NumberFormat::IGNORE.rebuild()
    .digit_separator(b'_')
    .build()
    .unwrap();

// Create our own grammar.
// A NumberFormat is compiled from options into binary flags, each
// taking 1-bit, allowing high-performance, customizable parsing
// once they're compiled. Each flag will be explained while defining it.

// The '_' character will be used as a digit separator.
let digit_separator = b'_';

// Require digits in the integer component of a float.
// `0.1` is valid, but `.1` is not.
let required_integer_digits = false;

// Require digits in the fraction component of a float.
// `1.0` is valid, but `1.` and `1` are not.
let required_fraction_digits = false;

// Require digits in the exponent component of a float.
// `1.0` and `1.0e7` is valid, but `1.0e` is not.
let required_exponent_digits = false;

// Do not allow a positive sign before the mantissa.
// `1.0` and `-1.0` are valid, but `+1.0` is not.
let no_positive_mantissa_sign = false;

// Require a sign before the mantissa.
// `+1.0` and `-1.0` are valid, but `1.0` is not.
let required_mantissa_sign = false;

// Do not allow the use of exponents.
// `300.0` is valid, but `3.0e2` is not.
let no_exponent_notation = false;

// Do not allow a positive sign before the exponent.
// `3.0e2` and 3.0e-2` are valid, but `3.0e+2` is not.
let no_positive_exponent_sign = false;

// Require a sign before the exponent.
// `3.0e+2` and `3.0e-2` are valid, but `3.0e2` is not.
let required_exponent_sign = false;

// Do not allow an exponent without fraction digits.
// `3.0e7` is valid, but `3e7` and `3.e7` are not.
let no_exponent_without_fraction = false;

// Do not allow special values.
// `1.0` is valid, but `NaN` and `inf` are not.
let no_special = false;

// Use case-sensitive matching when parsing special values.
// `NaN` is valid, but `nan` and `NAN` are not.
let case_sensitive_special = false;

// Allow digit separators between digits in the integer component.
// `3_4.01` is valid, but `_34.01`, `34_.01` and `34.0_1` are not.
let integer_internal_digit_separator = false;

// Allow digit separators between digits in the fraction component.
// `34.0_1` is valid, but `34._01`, `34.01_` and `3_4.01` are not.
let fraction_internal_digit_separator = false;

// Allow digit separators between digits in the exponent component.
// `1.0e6_7` is valid, but `1.0e_67`, `1.0e67_` and `1_2.0e67` are not.
let exponent_internal_digit_separator = false;

// Allow digit separators before any digits in the integer component.
// These digit separators may occur before or after the sign, as long
// as they occur before any digits.
// `_34.01` is valid, but `3_4.01`, `34_.01` and `34._01` are not.
let integer_leading_digit_separator = false;

// Allow digit separators before any digits in the fraction component.
// `34._01` is valid, but `34.0_1`, `34.01_` and `_34.01` are not.
let fraction_leading_digit_separator = false;

// Allow digit separators before any digits in the exponent component.
// These digit separators may occur before or after the sign, as long
// as they occur before any digits.
// `1.0e_67` is valid, but `1.0e6_7`, `1.0e67_` and `_1.0e67` are not.
let exponent_leading_digit_separator = false;

// Allow digit separators after any digits in the integer component.
// If `required_integer_digits` is not set, `_.01` is valid.
// `34_.01` is valid, but `3_4.01`, `_34.01` and `34.01_` are not.
let integer_trailing_digit_separator = false;

// Allow digit separators after any digits in the fraction component.
// If `required_fraction_digits` is not set, `1._` is valid.
// `34.01_` is valid, but `34.0_1`, `34._01` and `34_.01` are not.
let fraction_trailing_digit_separator = false;

// Allow digit separators after any digits in the exponent component.
// If `required_exponent_digits` is not set, `1.0e_` is valid.
// `1.0e67_` is valid, but `1.0e6_7`, `1.0e_67` and `1.0_e67` are not.
let exponent_trailing_digit_separator = false;

// Allow consecutive separators in the integer component.
// This requires another integer digit separator flag to be set.
// For example, if `integer_internal_digit_separator` and this flag are set,
// `3__4.01` is valid, but `__34.01`, `34__.01` and `34.0__1` are not.
let integer_consecutive_digit_separator = false;

// Allow consecutive separators in the fraction component.
// This requires another fraction digit separator flag to be set.
// For example, if `fraction_internal_digit_separator` and this flag are set,
// `34.0__1` is valid, but `34.__01`, `34.01__` and `3__4.01` are not.
let fraction_consecutive_digit_separator = false;

// Allow consecutive separators in the exponent component.
// This requires another exponent digit separator flag to be set.
// For example, if `exponent_internal_digit_separator` and this flag are set,
// `1.0e6__7` is valid, but `1.0e__67`, `1.0e67__` and `1__2.0e67` are not.
let exponent_consecutive_digit_separator = false;

// Allow digit separators in special values.
// If set, allow digit separators in special values will be ignored.
// `N_a_N__` is valid, but `i_n_f_e` is not.
let special_digit_separator = false;

// Compile the grammar.
// We can add as many or as few specifiers as we want.
let format = NumberFormat::builder()
    .digit_separator(digit_separator)
    .required_integer_digits(required_integer_digits)
    .required_fraction_digits(required_fraction_digits)
    .required_exponent_digits(required_exponent_digits)
    .no_positive_mantissa_sign(no_positive_mantissa_sign)
    .required_mantissa_sign(required_mantissa_sign)
    .no_exponent_notation(no_exponent_notation)
    .no_positive_exponent_sign(no_positive_exponent_sign)
    .required_exponent_sign(required_exponent_sign)
    .no_exponent_without_fraction(no_exponent_without_fraction)
    .no_special(no_special)
    .case_sensitive_special(case_sensitive_special)
    .integer_internal_digit_separator(integer_internal_digit_separator)
    .fraction_internal_digit_separator(fraction_internal_digit_separator)
    .exponent_internal_digit_separator(exponent_internal_digit_separator)
    .integer_leading_digit_separator(integer_leading_digit_separator)
    .fraction_leading_digit_separator(fraction_leading_digit_separator)
    .exponent_leading_digit_separator(exponent_leading_digit_separator)
    .integer_trailing_digit_separator(integer_trailing_digit_separator)
    .fraction_trailing_digit_separator(fraction_trailing_digit_separator)
    .exponent_trailing_digit_separator(exponent_trailing_digit_separator)
    .integer_consecutive_digit_separator(integer_consecutive_digit_separator)
    .fraction_consecutive_digit_separator(fraction_consecutive_digit_separator)
    .exponent_consecutive_digit_separator(exponent_consecutive_digit_separator)
    .special_digit_separator(special_digit_separator)
    .build()
    .unwrap();
```

# Options

Lexical-core also includes number parse and write options for additional customizability.

- **NaN**
    - `ParseFloatOptions::nan_string`
    - `WriteFloatOptions::nan_string`
    <blockquote>The representation of Not a Number (NaN) as a string (default <code>b"NaN"</code>). By default, lexical-core uses case-insensitive comparisons, although this may be turned off in <code>NumberFormat</code>. This string <b>must</b> start with an <code>'N'</code> or <code>'n'</code>.</blockquote>
- **Short Infinity**
    - `ParseFloatOptions::inf_string`
    - `WriteFloatOptions::inf_string`
    <blockquote>The short, default representation of infinity as a string (default <code>b"inf"</code>). By default, lexical-core uses case-insensitive comparisons, although this may be turned off in <code>NumberFormat</code>. This string **must** start with an <code>'I'</code> or <code>'i'</code>.</blockquote>
- **Long Infinity**
    - `ParseFloatOptions::infinity_string`
    <blockquote>The long, backup representation of infinity as a string (default <code>b"infinity"</code>). The long infinity must be at least as long as the short infinity, and will only be used during float parsing (and is by default case-insensitive). This string **must** start with an <code>'I'</code> or <code>'i'</code>.</blockquote>
- **Exponent Decimal Character**
    - `ParseFloatOptions::exponent_decimal`
    - `WriteFloatOptions::exponent_decimal`
    <blockquote>The default character designating the exponent component of a float (default <code>b'e'</code>) for strings with radix 10 (decimal). For float parsing, lexical-core uses case-insensitive comparisons. This value should be not be in character set <code>[0-9+\-]</code>, and should not be equal to any digit separators or decimal point characters.</blockquote>
- **Exponent Backup Character** (radix only)
    - `ParseFloatOptions::exponent_backup`
    - `WriteFloatOptions::exponent_backup`
    <blockquote>The backup character designating the exponent component of a float (default <code>b'^'</code>) for non-decimal strings (<code>radix != 10</code>). This value should be not be in character set <code>[0-9a-zA-Z+\-]</code>, and should not be equal to any digit separators or decimal point characters.</blockquote>
- **Float Rounding** (rounding only)
    - `ParseFloatOptions::rounding`
    <blockquote>The IEEE754 float-rounding scheme to be used during float parsing. In almost every case, this should be set to <code>RoundingKind::NearestTieEven</code>.</blockquote>

# Constants

Lexical-core also includes a few constants to simplify interfacing with number-to-string code, and are implemented for the `lexical_core::Number` trait, which is required by `ToLexical`.

- **FORMATTED_SIZE** The maximum number of bytes a formatter may write.
    <blockquote>For example, <code>lexical_core::write_radix::&lt;i32&gt;</code> may write up to <code>i32::FORMATTED_SIZE</code> characters. This constant may significantly overestimate the number of characters required for decimal strings when the radix feature is enabled.</blockquote>
- **FORMATTED_SIZE_DECIMAL** The maximum number of bytes a formatter may write in decimal (base 10).
    <blockquote>For example, <code>lexical_core::write::&lt;i32&gt;</code> may write up to <code>i32::FORMATTED_SIZE_DECIMAL</code> characters.</blockquote>

These are provided as Rust constants so they may be used as the size element in arrays.

# Documentation

Lexical-core's documentation can be found on [docs.rs](https://docs.rs/lexical-core).

# Validation

Float parsing is difficult to do correctly, and major bugs have been found in implementations from [libstdc++'s strtod](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) to [Python](https://bugs.python.org/issue7632). In order to validate the accuracy of the lexical, we employ the following external tests:

1. Hrvoje Abraham's [strtod](https://github.com/ahrvoje/numerics/tree/master/strtod) test cases.
2. Rust's [test-float-parse](https://github.com/rust-lang/rust/tree/64185f205dcbd8db255ad6674e43c63423f2369a/src/etc/test-float-parse) unittests.
3. Testbase's [stress tests](https://www.icir.org/vern/papers/testbase-report.pdf) for converting from decimal to binary.
4. Nigel Tao's [tests](https://github.com/nigeltao/parse-number-fxx-test-data) extracted from test suites for Freetype, Google's double-conversion library, IBM's IEEE-754R compliance test, as well as numerous other curated examples.
5. [Various](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) [difficult](https://www.exploringbinary.com/how-glibc-strtod-works/) [cases](https://www.exploringbinary.com/how-strtod-works-and-sometimes-doesnt/) reported on blogs.

Although lexical may contain bugs leading to rounding error, it is tested against a comprehensive suite of random-data and near-halfway representations, and should be fast and correct for the vast majority of use-cases.

# Implementation Details

## Float to String

For float-to-decimal conversions, we use either the Grisu algorithms or the Ryu algorithm. Both are optimized, high-performance algorithms that print the shortest, correct version of a float in the vast majority of cases.

For more information on the Grisu2 and Grisu3 algorithms, see [Printing Floating-Point Numbers Quickly and Accurately with Integers](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf).

For more information on the Ryu algorithm, see [Ryū: fast float-to-string conversion](https://dl.acm.org/citation.cfm?id=3192369).

For non-decimal numbers, we use 2 different algorithms: 
- An [optimized](https://github.com/Alexhuszagh/rust-lexical/blob/master/lexical-core/src/ftoa/binary.rs) algorithm for radixes that are powers of two.
- A [generic](https://github.com/Alexhuszagh/rust-lexical/blob/master/lexical-core/src/ftoa/radix.rs) radix algorithm all other radixes.

Both algorithms are correct, although their performance differs dramatically. The optimized algorithm knows that the significant digits of a float can always be exactly represented in a fixed size output, and can use optimized integer-to-string algorithms to generate significant digits.

The generic algorithm, adapted from the V8 codebase, attempts to write a single character from significant digits of the fraction. However, due to digit carry-over, it needs to frequently back-trace, re-writing digits and preventing loop unrolling. However, as non-decimal, non-power-of-two float-to-string conversions are very uncommon, this performance penalty is unlikely to be an issue.

## String to Float

In order to implement an efficient parser in Rust, lexical uses the following steps:

1. We ignore the sign until the end, and merely toggle the sign bit after creating a correct representation of the positive float.
2. We handle special floats, such as "NaN", "inf", "Infinity". If we do not have a special float, we continue to the next step.
3. We parse up to 64-bits from the string for the mantissa, ignoring any trailing digits, and parse the exponent (if present) as a signed 32-bit integer. If the exponent overflows or underflows, we set the value to i32::max_value() or i32::min_value(), respectively.
4. **Fast Path** We then try to create an exact representation of a native binary float from parsed mantissa and exponent. If both can be exactly represented, we multiply the two to create an exact representation, since IEEE754 floats mandate the use of guard digits to minimizing rounding error. If either component cannot be exactly represented as the native float, we continue to the next step.
5. **Moderate Path** If the `lemire` feature is enabled, we use the [Eisel-Lemire](https://nigeltao.github.io/blog/2020/eisel-lemire.html) algorithm. We first we create a representation of the float as a mantissa and the decimal exponent for that mantissa, create a 128-bit representation of the mantissa multiplied by the decimal exponent, and check for halfway representations. If we can differentiate our representation from halfway points, we return the correct representation. If we cannot unambiguously determine the correct floating-point representation, we round to the `b` representation and go to the slow path algorithm.
6. **Fallback Moderate Path** If `lemire` feature is not enabled, we create an approximate, extended, 80-bit float type (64-bits for the mantissa, 16-bits for the exponent) from both components, and multiply them together. This minimizes the rounding error, through guard digits. We then estimate the error from the parsing and multiplication steps, and if the float +/- the error differs significantly from `b+h`, we return the correct representation (`b` or `b+u`). If we cannot unambiguously determine the correct floating-point representation, we round downwards to `b` and continue to the next step.
7. **Slow Path** We use arbitrary-precision arithmetic to disambiguate the correct representation without any rounding error. We create an exact representation of the input digits as a big integer, to determine how to round the top 53 bits for the mantissa. If there is a fraction or a negative exponent, we create a representation of the significant digits for `b+h` and scale the input digits by the binary exponent in `b+h`, and scale the significant digits in `b+h` by the decimal exponent, and compare the two to determine if we need to round up or down.

Since arbitrary-precision arithmetic is slow and scales poorly for decimal strings with many digits or exponents of high magnitude, lexical also supports a two faster algorithms:
- A lossy algorithm, which returns the value from the fast or moderate path. This will be accurate to within 1 ULP.
- An incorrect algorithm, which will return the exact value from the fast path, or an incorrect value to within a few ULP. This is only meant for applications where accuracy does not matter: only performance.

## Halfway Cases

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

## Eisel-Lemire vs. Extended Float

The Eisel-Lemire [algorithm](https://nigeltao.github.io/blog/2020/eisel-lemire.html) is a fast, moderate path algorithm that creates an extended, 128-bit approximation of the mantissa scaled to the decimal exponent. The algorithm is not correct for all cases, but is accurate and fast for the vast majority of cases, and does not report false positives. The scaling uses a pre-computed table of 128-bit powers of 10, computed for the entire range of valid `f64` values.

First, we create a 128-bit estimate by multiplying the 64-bit, normalized mantissa by the 64 high bits of the power of 10. We're currently within 2-units of the accurate representation, and if we cannot narrow this to a 1-unit range, then we expand to a 192-bit estimate, and proceed in a similar fashion to narrow to a 1-unit range. If the bits truncated after shifting would be a halfway point, the lower bits are all 0, then we round down to `b` and use the slow-path algorithm. This is very fast, because the major bottleneck is 1-2 64-bit multiply instructions.

The extended-float algorithm creates an 80-bit extended-precision representation of the float (64-bits for the mantissa, 16-bits for the binary exponent), and then multiplies this extended-precision float by two pre-computed powers of 10 (a small power, and a large power, to minimize storage requirements). Although quite fast, this requires 2-3 64-bit multiply instructions, and is ~20% slower for algorithms that can create correct representations from both algorithms.

Although the extended-float algorithm is more comprehensive (it can correctly the determine the accurate representation of more floats than the Eisel-Lemire algorithm), in practice, this reduced accuracy tends not to matter. Since neither algorithm produces false positives, and the slow-path algorithm is reasonably performant for the remaining cases, for almost all forms of data, Eisel-Lemire is faster than the extended-float algorithm.

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

On the ARMv6 architecture, the stable exponentiation for the fast, incorrect float parser is not fully stable. For example, `1e-300` is correct, while `5e-324` rounds to `0`, leading to "5e-324" being incorrectly parsed as `0`. This does not affect the default, correct float parser, nor ARMVv7 or ARMVv8 (aarch64) architectures. This bug can compound errors in the incorrect parser. It is not known if this bug is an artifact of Qemu emulation of ARMv6, or is actually representative the hardware.

Versions of lexical-core prior to 0.4.3 could round parsed floating-point numbers with an error of up to 1 ULP. This occurred for strings with 16 or more digits and a trailing 0 in the fraction, the `b+h` comparison in the slow-path algorithm incorrectly scales the the theoretical digits due to an over-calculated real exponent. This affects a very small percentage of inputs, however, it is recommended to update immediately.

# Platform Support

lexical-core is tested on a wide variety of platforms, including big and small-endian systems, to ensure portable code. Supported architectures include:
- x86_64 Linux, Windows, macOS, Android, iOS, FreeBSD, and NetBSD.
- x86 Linux, macOS, Android, iOS, and FreeBSD.
- aarch64 (ARM8v8-A) Linux, Android, and iOS.
- armv7 (ARMv7-A) Linux, Android, and iOS.
- arm (ARMv6) Linux, and Android.
- mips (MIPS) Linux.
- mipsel (MIPS LE) Linux.
- mips64 (MIPS64 BE) Linux.
- mips64el (MIPS64 LE) Linux.
- powerpc (PowerPC) Linux.
- powerpc64 (PPC64) Linux.
- powerpc64le (PPC64LE) Linux.
- s390x (IBM Z) Linux.

lexical-core should also work on a wide variety of other architectures and ISAs. If you have any issue compiling lexical-core on any architecture, please file a bug report.

# Versioning and Version Support

**Version Support**

The currently supported versions are:
- v0.8.x 
- v0.7.x (Maintenance)
- v0.6.x (Maintenance)

**Rustc Compatibility**

v0.8.x supports 1.37+, including stable, beta, and nightly.

v0.7.x supports 1.37+, including stable, beta, and nightly.

v0.6.x supports Rustc 1.24+, including stable, beta, and nightly.

Please report any errors compiling a supported lexical-core version on a compatible Rustc version.

**Versioning**

Lexical-core uses [semantic versioning](https://semver.org/). Removing support for Rustc versions newer than the latest stable Debian or Ubuntu version is considered an incompatible API change, requiring a major version change.

# Changelog

All changes since 0.4.1 are documented in [CHANGELOG](CHANGELOG).

# License

Lexical-core is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

Lexical-core also ports some code from [rust](https://github.com/rust-lang/rust) (for backwards compatibility), [V8](https://github.com/v8/v8), [libgo](https://golang.org/src) and [fpconv](https://github.com/night-shift/fpconv), and therefore might be subject to the terms of a 3-clause BSD license or BSD-like license.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
