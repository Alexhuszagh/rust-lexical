lexical
=======

[![Build Status](https://api.travis-ci.org/Alexhuszagh/lexical.svg?branch=master)](https://travis-ci.org/Alexhuszagh/lexical)
[![Latest Version](https://img.shields.io/crates/v/lexical.svg)](https://crates.io/crates/lexical)

Fast lexical conversion routines for both std and no_std environments. Lexical provides routines to convert numbers to and from decimal strings. Lexical also supports non-base 10 numbers, for both integers and floats.  Lexical is simple to use, and exports only 6 functions in the high-level API.

Lexical heavily uses unsafe code for performance, and therefore may introduce memory-safety issues. Although the code is tested with wide variety of inputs to minimize the risk of memory-safety bugs, no guarantees are made and you should use it at your own risk.

**Table of Contents**

- [Getting Started](#getting-started)
- [Benchmarks](#benchmarks)
- [Documentation](#documentation)
- [License](#license)
- [Contributing](#contributing)

# Getting Started

Add lexical to your `Cargo.toml`:

```yaml
[dependencies]
lexical = "1.0"
```

And get started using lexical:

```rust
extern crate lexical;

// Number to string
lexical::to_string(3.0);            // "3.0", always has a fraction suffix, 
lexical::to_string(3);              // "3"
lexical::to_string_digits(3.0, 2);  // "11.0", non-base 10 representation.
lexical::to_string_digits(1.5, 2);  // "1.1"
lexical::to_string_digits(1.25, 2); // "1.01"

// String to number.
let i: i32 = lexical::parse("3");            // 3, auto-type deduction.
let f: f32 = lexical::parse("3.5");          // 3.5
let d = lexical::parse::<f64, _>("3.5");     // 3.5, explicit type hints.
let d = lexical::try_parse::<f64, _>("3.5"); // Ok(3.5), error checking parse.
let d = lexical::try_parse::<f64, _>("3a");  // Err(1), failed to parse.
```

Lexical's parsers can be either error-checked and unchecked. The unchecked parsers continue to parse until they encounter invalid data, returning a number was successfully parsed up until that point. This is analogous to C's `strtod`, which may not be desirable for many applications. Therefore, lexical also includes checked parsers, which ensure the entire buffer is used while parsing and do not discard any characters. Upon erroring, the checked parsers will return the index where the invalid data was found.

```rust
// This will return Err(3), indicating the first invalid character
// occurred at the index 3 in the input string (the space character).
let x: i32 = lexical::try_parse("123 456");

// This will return 123, since that is the value found before invalid
// character was encountered.
let x: i32 = lexical::parse("123 456");
```

# Benchmarks

The following benchmarks measure the time it takes to convert 10,000 random values, for different types. The values were randomly generated using NumPy, and run in both std (rustc 1.29.2) and no_std (rustc 1.31.0) contexts (only std is shown) on an x86-64 Intel processor. More information on these benchmarks can be found in the [benches](benches) folder and in the source code for the respective algorithms.

For all the following benchmarks, lower is better.

**Float to String**

![ftoa benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/ftoa.png)

**Integer To String**

![itoa benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/itoa.png)

**String to Float**

![atof benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atof.png)

**String to Integer**

![atoi benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/atoi.png)

Furthermore, for number-to-string conversions, lexical's performance is comparable to (if not slightly faster) than [dtoa](https://github.com/dtolnay/dtoa) and [itoa](https://github.com/dtolnay/itoa), despite supporting more functionality than either (including non-base 10 representations).

**Float to String**

![lexical-dtoa benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/lexical_dtoa.png)

**Integer To String**

![lexical-itoa benchmark](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/master/assets/lexical_itoa.png)

# Documentation

Lexical's documentation can be found on [docs.rs](https://docs.rs/lexicals).

# Details

The float-to-string implementation uses the Grisu2 algorithm for speed, which for ~0.5% of inputs may return the non-shortest representation.

For more information on the Grisu2 algorithm and other floating-point representation algorithms, see [Printing Floating-Point Numbers Quickly and Accurately with Integers](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf).

# License

Lexical is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

Lexical also ports some code from [V8](https://github.com/v8/v8) and [fpconv](https://github.com/night-shift/fpconv), and therefore might be subject to the terms of a 3-clause BSD license.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
