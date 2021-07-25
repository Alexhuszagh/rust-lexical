lexical-experimental
====================

This is an experimental branch of [lexical](https://github.com/Alexhuszagh/rust-lexical), with a focus on fast compile times, optimal performance, and modularization. Each subcomponent, such as integer parsing, float parsing, will be broken in workspaces.

Currently, the various algorithm designs and benchmarks can be seen markdown files in each respective crate. The current crates are:

- lexical-util (shared utilities for all crates).
- lexical-parse-integer (parse integers from string).
- lexical-parse-float (parse floats from string).
- lexical-write-integer (write integers to string).
- lexical-write-float (write floats to string).
- lexical-core (meta-crate for all specific parsers).
- lexical (high-level wrappers when a system allocator exists).

# License

Lexical is dual licensed under the Apache 2.0 license as well as the MIT license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

# Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in lexical by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
