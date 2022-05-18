lexical
=======

High-performance numeric conversion routines for use in a `no_std` environment. This does not depend on any standard library features, nor a system allocator.

**Similar Projects**

If you want a minimal, stable, and compile-time friendly version of lexical's float-parsing algorithm, see [minimal-lexical](https://github.com/Alexhuszagh/minimal-lexical). If you want a minimal, performant float parser, recent versions of the Rust standard library should be [sufficient](https://github.com/rust-lang/rust/pull/86761).

**Table of Contents**

- [Getting Started](#getting-started)
- [Partial/Complete Parsers](#partialcomplete-parsers)
- [no_std](#no_std)
- [Features](#features)
- [Customization](#customization)
  - [Number Format API](#number-format-api)
  - [Options API](#options-api)
- [Documentation](#documentation)
- [Validation](#validation)
- [Metrics](#metrics)
- [Safety](#safety)
- [Platform Support](#platform-support)
- [Versioning and Version Support](#versioning-and-version-support)
- [Changelog](#changelog)
- [License](#license)
- [Contributing](#contributing)

# Getting Started

Add lexical to your `Cargo.toml`:

```toml
[dependencies]
lexical = "^6.0"
```

And get started using lexical:

```rust
// Number to string
use lexical_core::BUFFER_SIZE;
let mut buffer = [b'0'; BUFFER_SIZE];
lexical_core::write(3.0, &mut buffer);   // "3.0", always has a fraction suffix,
lexical_core::write(3, &mut buffer);     // "3"

// String to number.
let i: i32 = lexical_core::parse("3")?;      // Ok(3), auto-type deduction.
let f: f32 = lexical_core::parse("3.5")?;    // Ok(3.5)
let d: f64 = lexical_core::parse("3.5")?;    // Ok(3.5), error checking parse.
let d: f64 = lexical_core::parse("3a")?;     // Err(Error(_)), failed to parse.
```

In order to use lexical in generic code, the trait bounds `FromLexical` (for `parse`) and `ToLexical` (for `to_string`) are provided.

```rust
/// Multiply a value in a string by multiplier, and serialize to string.
fn mul_2<T>(value: &str, multiplier: T)
    -> Result<String, lexical_core::Error>
where 
    T: lexical_core::ToLexical + lexical_core::FromLexical,
{
    let value: T = lexical_core::parse(value.as_bytes())?;
    let mut buffer = [b'0'; lexical_core::BUFFER_SIZE];
    let bytes = lexical_core::write(value * multiplier, &mut buffer);
    Ok(std::str::from_utf8(bytes).unwrap())
}
```

# Partial/Complete Parsers

Lexical has both partial and complete parsers: the complete parsers ensure the entire buffer is used while parsing, without ignoring trailing characters, while the partial parsers parse as many characters as possible, returning both the parsed value and the number of parsed digits. Upon encountering an error, lexical will return an error indicating both the error type and the index at which the error occurred inside the buffer.

**Complete Parsers**

```rust
// This will return Err(Error::InvalidDigit(3)), indicating
// the first invalid character occurred at the index 3 in the input
// string (the space character).
let x: i32 = lexical_core::parse(b"123 456")?;
```

**Partial Parsers**

```rust
// This will return Ok((123, 3)), indicating that 3 digits were successfully
// parsed, and that the returned value is `123`.
let (x, count): (i32, usize) = lexical_core::parse_partial(b"123 456")?;
```

# no_std

`lexical-core` does not depend on a standard library, nor a system allocator. To use `lexical-core` in a `no_std` environment, add the following to `Cargo.toml`:

```toml
[dependencies.lexical-core]
version = "0.8.5"
default-features = false
# Can select only desired parsing/writing features.
features = ["write-integers", "write-floats", "parse-integers", "parse-floats"]
```

And get started using lexical:

```rust
// A constant for the maximum number of bytes a formatter will write.
use lexical_core::BUFFER_SIZE;
let mut buffer = [b'0'; BUFFER_SIZE];

// Number to string. The underlying buffer must be a slice of bytes.
let count = lexical_core::write(3.0, &mut buffer);
assert_eq!(buffer[..count], b"3.0");
let count = lexical_core::write(3i32, &mut buffer);
assert_eq!(buffer[..count], b"3");

// String to number. The input must be a slice of bytes.
let i: i32 = lexical_core::parse(b"3")?;      // Ok(3), auto-type deduction.
let f: f32 = lexical_core::parse(b"3.5")?;    // Ok(3.5)
let d: f64 = lexical_core::parse(b"3.5")?;    // Ok(3.5), error checking parse.
let d: f64 = lexical_core::parse(b"3a")?;     // Err(Error(_)), failed to parse.
```

# Features

Lexical feature-gates each numeric conversion routine, resulting in faster compile times if certain numeric conversions. These features can be enabled/disabled for both `lexical-core` (which does not require a system allocator) and `lexical`. By default, all conversions are enabled.

- **parse-floats**: &ensp; Enable string-to-float conversions.
- **parse-integers**: &ensp; Enable string-to-integer conversions.
- **write-floats**: &ensp; Enable float-to-string conversions.
- **write-integers**: &ensp; Enable integer-to-string conversions.

Lexical is highly customizable, and contains numerous other optional features:

- **std**: &ensp; Enable use of the Rust standard library (enabled by default).
- **power-of-two**: &ensp; Enable conversions to and from non-decimal strings.
    <blockquote>With power_of_two enabled, the radixes <code>{2, 4, 8, 10, 16, and 32}</code> are valid, otherwise, only 10 is valid. This enables common conversions to/from hexadecimal integers/floats, without requiring large pre-computed tables for other radixes.</blockquote>
- **radix**: &ensp; Allow conversions to and from non-decimal strings.
    <blockquote>With radix enabled, any radix from 2 to 36 (inclusive) is valid, otherwise, only 10 is valid.</blockquote>
- **format**: &ensp; Customize acceptable number formats for number parsing and writing.
    <blockquote>With format enabled, the number format is dictated through bitflags and masks packed into a <code>u128</code>. These dictate the valid syntax of parsed and written numbers, including enabling digit separators, requiring integer or fraction digits, and toggling case-sensitive exponent characters.</blockquote>
- **compact**: &ensp; Optimize for binary size at the expense of performance. 
    <blockquote>This minimizes the use of pre-computed tables, producing significantly smaller binaries.</blockquote>
- **safe**: &ensp; Require all array indexing to be bounds-checked. 
    <blockquote>This is effectively a no-op for number parsers, since they use safe indexing except where indexing without bounds checking can be trivially shown to be correct. The number writers frequently use unsafe indexing, since we can easily over-estimate the number of digits in the output due to the fixed-length input.</blockquote>
- **f16**: &ensp; Add support for numeric conversions to-and-from 16-bit floats.
    <blockquote>Adds <code>f16</code>, a half-precision IEEE-754 floating-point type, and <code>bf16</code>, the Brain Float 16 type, and numeric conversions to-and-from these floats. Note that since these are storage formats, and therefore do not have native arithmetic operations, all conversions are done using an intermediate <code>f32</code>.</blockquote>

To ensure the safety when bounds checking is disabled, we extensively fuzz the all numeric conversion routines. See the [Safety](#safety) section below for more information.

Lexical also places a heavy focus on code bloat: with algorithms both optimized for performance and size. By default, this focuses on performance, however, using the `compact` feature, you can also opt-in to reduced code size at the cost of performance. The compact algorithms minimize the use of pre-computed tables and other optimizations at the cost of performance.

# Customization

> ⚠ **WARNING:** If changing the number of significant digits written, disabling the use of exponent notation, or changing exponent notation thresholds, `BUFFER_SIZE` may be insufficient to hold the resulting output. `WriteOptions::buffer_size` will provide a correct upper bound on the number of bytes written. If a buffer of insufficient length is provided, lexical-core will panic.

Every language has competing specifications for valid numerical input, meaning a number parser for Rust will incorrectly accept or reject input for different programming or data languages. For example:

```rust
// Valid in Rust strings.
// Not valid in JSON.
let f: f64 = lexical_core::parse(b"3.e7")?;  // 3e7

// Let's only accept JSON floats.
const JSON: u128 = lexical_core::format::JSON;
let options = ParseFloatOptions::new();
let f: f64 = lexical_core::parse_with_options::<JSON>(b"3.0e7", &options)?; // 3e7
let f: f64 = lexical_core::parse_with_options::<JSON>(b"3.e7", &options)?;  // Errors!
```

Due the high variability in the syntax of numbers in different programming and data languages, we provide 2 different APIs to simplify converting numbers with different syntax requirements.

- Number Format API (feature-gated via `format` or `power-of-two`). 
    <blockquote>This is a packed struct contained flags to specify compile-time syntax rules for number parsing or writing. This includes features such as the radix of the numeric string, digit separators, case-sensitive exponent characters, optional base prefixes/suffixes, and more.</blockquote>
- Options API.
    <blockquote>This contains run-time rules for parsing and writing numbers. This includes exponent break points, rounding modes, the exponent and decimal point characters, and the string representation of NaN and Infinity.</blockquote>

A limited subset of functionality is documented in examples below, however, the complete specification can be found in the API reference documentation.

## Number Format API

The number format class provides numerous flags to specify number syntax when parsing or writing. When the `power-of-two` feature is enabled, additional flags are added:

- The radix for the significant digits (default `10`).
- The radix for the exponent base (default `10`).
- The radix for the exponent digits (default `10`).

When the `format` feature is enabled, numerous other syntax and digit separator flags are enabled, including:

- A digit separator character, to group digits for increased legibility.
- Whether leading, trailing, internal, and consecutive digit separators are allowed.
- Toggling required float components, such as digits before the decimal point.
- Toggling whether special floats are allowed or are case-sensitive.

Many pre-defined constants therefore exist to simplify common use-cases,
including:

- JSON, XML, TOML, YAML, SQLite, and many more.
- Rust, Python, C#, FORTRAN, COBOL literals and strings, and many more.

An example of building a custom number format is as follows:

```rust
const FORMAT: u128 = lexical_core::NumberFormatBuilder::new()
    // Disable exponent notation.
    .no_exponent_notation(true)
    // Disable all special numbers, such as Nan and Inf.
    .no_special(true)
    .build();

// Due to use in a `const fn`, we can't panic or expect users to unwrap invalid
// formats, so it's up to the caller to verify the format. If an invalid format
// is provided to a parser or writer, the function will error or panic, respectively.
debug_assert!(lexical_core::format_is_valid::<FORMAT>());
```

## Options API

The options API allows customizing number parsing and writing at run-time, such as specifying the maximum number of significant digits, exponent characters, and more.

An example of building a custom options struct is as follows:

```rust
use std::num;

let options = lexical_core::WriteFloatOptions::builder()
    // Only write up to 5 significant digits, IE, `1.23456` becomes `1.2345`.
    .max_significant_digits(num::NonZeroUsize::new(5))
    // Never write less than 5 significant digits, `1.1` becomes `1.1000`.
    .min_significant_digits(num::NonZeroUsize::new(5))
    // Trim the trailing `.0` from integral float strings.
    .trim_floats(true)
    // Use a European-style decimal point.
    .decimal_point(b',')
    // Panic if we try to write NaN as a string.
    .nan_string(None)
    // Write infinity as "Infinity".
    .inf_string(Some(b"Infinity"))
    .build()
    .unwrap();
```

# Documentation

Lexical's API reference can be found on [docs.rs](https://docs.rs/lexical), as can [lexical-core's](lexical-core). Detailed descriptions of the algorithms used can be found here:

- [Parsing Integers](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-integer/docs/Algorithm.md)
- [Parsing Floats](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-float/docs/Algorithm.md)
- [Writing Integers](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Algorithm.md)
- [Writing Floats](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-float/docs/Algorithm.md)

In addition, descriptions of how lexical handles [digit separators](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/DigitSeparators.md) and implements [big-integer arithmetic](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-float/docs/BigInteger.md) are also documented.

# Validation

**Float-Parsing**

Float parsing is difficult to do correctly, and major bugs have been found in implementations from [libstdc++'s strtod](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) to [Python](https://bugs.python.org/issue7632). In order to validate the accuracy of the lexical, we employ the following external tests:

1. Hrvoje Abraham's [strtod](https://github.com/ahrvoje/numerics/tree/master/strtod) test cases.
2. Rust's [test-float-parse](https://github.com/rust-lang/rust/tree/64185f205dcbd8db255ad6674e43c63423f2369a/src/etc/test-float-parse) unittests.
3. Testbase's [stress tests](https://www.icir.org/vern/papers/testbase-report.pdf) for converting from decimal to binary.
4. Nigel Tao's [tests](https://github.com/nigeltao/parse-number-fxx-test-data) extracted from test suites for Freetype, Google's double-conversion library, IBM's IEEE-754R compliance test, as well as numerous other curated examples.
5. [Various](https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/) [difficult](https://www.exploringbinary.com/how-glibc-strtod-works/) [cases](https://www.exploringbinary.com/how-strtod-works-and-sometimes-doesnt/) reported on blogs.

Although lexical may contain bugs leading to rounding error, it is tested against a comprehensive suite of random-data and near-halfway representations, and should be fast and correct for the vast majority of use-cases.

# Metrics

Various benchmarks, binary sizes, and compile times are shown here:

**Build Timings**

The compile-times when building with all numeric conversions enabled. For a more fine-tuned breakdown, see [build timings](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/BuildTimings.md).

![Build Timings](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/timings_all_posix.svg)

**Binary Size**

The binary sizes of stripped binaries compiled at optimization level "2". For a more fine-tuned breakdown, see [binary sizes](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/BinarySize.md).

![Parse Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt2_posix.svg)
![Write Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt2_posix.svg)

**Benchmarks -- Parse Integer**

A benchmark on randomly-generated integers uniformly distributed over the entire range. For a more fine-tuned breakdown, see [benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-integer/docs/Benchmarks.md).

![Uniform Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-integer/assets/random_uniform.svg)

**Benchmarks -- Parse Float**

A benchmark on parsing floats from various real-world data sets. For a more fine-tuned breakdown, see [benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-parse-float/docs/Benchmarks.md).

![Real Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-parse-float/assets/real.svg)

**Benchmarks -- Write Integer**

A benchmark on writing random integers uniformly distributed over the entire range. For a more fine-tuned breakdown, see [benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Benchmarks.md).

![Uniform Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-integer/assets/random_uniform.svg)

**Benchmarks -- Write Float**

A benchmark on writing floats generated via a random-number generator and parsed from a JSON document. For a more fine-tuned breakdown, see [benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-float/docs/Benchmarks.md).

![Random Data](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/lexical-write-float/assets/json.svg)

# Safety

Due to the use of memory unsafe code in the integer and float writers, we extensively fuzz our float writers and parsers. The fuzz harnesses may be found under [fuzz](https://github.com/Alexhuszagh/rust-lexical/tree/main/fuzz), and are run continuously. So far, we've parsed and written over 72 billion floats.

Due to the simple logic of the integer writers, and the lack of memory safety in the integer parsers, we minimally fuzz both, and test it with edge-cases, which has shown no memory safety issues to date.

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

- v0.8.x supports 1.51+, including stable, beta, and nightly.
- v0.7.x supports 1.37+, including stable, beta, and nightly.
- v0.6.x supports Rustc 1.24+, including stable, beta, and nightly.

Please report any errors compiling a supported lexical-core version on a compatible Rustc version.

**Versioning**

lexical uses [semantic versioning](https://semver.org/). Removing support for Rustc versions newer than the latest stable Debian or Ubuntu version is considered an incompatible API change, requiring a major version change.

# Changelog

All changes are documented in [CHANGELOG](https://github.com/Alexhuszagh/rust-lexical/blob/main/CHANGELOG).

# License

Lexical is dual licensed under the Apache 2.0 license as well as the MIT license. See the [LICENSE.md](LICENSE.md) file for full license details.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. Contributing to the repository means abiding by the [code of conduct](https://github.com/Alexhuszagh/rust-lexical/blob/main/CODE_OF_CONDUCT.md).

For the process on how to contribute to lexical, see the [development](https://github.com/Alexhuszagh/rust-lexical/blob/main/docs/Development.md) quick-start guide.
