lexical
=======

[![Build Status](https://api.travis-ci.org/Alexhuszagh/rust-lexical.svg?branch=master)](https://travis-ci.org/Alexhuszagh/rust-lexical)
[![Latest Version](https://img.shields.io/crates/v/lexical.svg)](https://crates.io/crates/lexical)

Fast lexical conversion routines for both std and no_std environments. Lexical provides routines to convert numbers to and from decimal strings. Lexical also supports non-base 10 numbers, for both integers and floats.  Lexical is simple to use, and exports only 6 functions in the high-level API.

**Table of Contents**

- [Getting Started](#getting-started)
- [Benchmarks](#benchmarks)
- [Documentation](#documentation)
- [Caveats](#caveats)
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

# Benchmarks

The following benchmarks measure the time it takes to convert 10,000 random values, for different types. The values were randomly generated using NumPy, and run in both std (rustc 1.29.2) and no_std (rustc 1.31.0) contexts (only std is shown) on an x86-64 Intel processor. More information on these benchmarks can be found in the [benches](benches) folder and in the source code for the respective algorithms. Adding the flags "target-cpu=native" and "link-args=-s" were also used, however, they minimally affected the relative performance difference between different lexical conversion implementations.

For all the following benchmarks, lower is better.

**Float to String**

![ftoa benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/ftoa.png)

**Integer To String**

![itoa benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/itoa.png)

**String to Float**

![atof benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atof.png)

**String to Integer**

![atoi benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atoi.png)

# Backends

For Float-To-String conversions, lexical uses one of three backends: an internal, Grisu2 algorithm (~99.5% correct), an external, Grisu3 algorithm (100% correct), and an external, Ryu algorithm (100% correct, ~2x as fast).

# Documentation

Lexical's documentation can be found on [docs.rs](https://docs.rs/lexical).

# Caveats

Lexical heavily uses unsafe code for performance, and therefore may introduce memory-safety issues. Although the code is tested with wide variety of inputs to minimize the risk of memory-safety bugs, no guarantees are made and you should use it at your own risk.

For float-parsing, lexical uses native floats for intermediate values, rather than arbitrary-precision integers, leading to fairly minor rounding (up to 1e-16), meaning that the float parser is not completely correct (for example, 1.2345e-308 is parsed as 1.2344999999999994e-308).

Finally, for non-base10 floats, lexical's float-to-string implementations may lead to fairly lossy rounding for a small subset of inputs (up to 0.1% of the total value).

# Details

For more information on the Grisu2 and Grisu3 algorithms, see [Printing Floating-Point Numbers Quickly and Accurately with Integers](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf).

For more information on the Ryu algorithm, see [Ryū: fast float-to-string conversion](https://dl.acm.org/citation.cfm?id=3192369).

# License

Lexical is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

Lexical also ports some code from [V8](https://github.com/v8/v8) and [fpconv](https://github.com/night-shift/fpconv), and therefore might be subject to the terms of a 3-clause BSD license.

# Roadmap

- Add a correct, string-to-float conversion for base10 floats.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
