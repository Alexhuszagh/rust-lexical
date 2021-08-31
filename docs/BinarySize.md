# Binary Size

The binary size comparisons were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.12.5/Fedora 34, and run against commit [c858a0e](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/c858a0ee9ed841a1d95f55eaf746f8c87e25f7bc). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`.

Each binary is generated using all optimization levels, and includes the result before and after stripping, with the core functionality black-boxed, to ensure optimization does not optimize-out the result.

All these binaries sizes are *relative* to the size of an empty Rust binary: that is, the size of the empty executable is subtracted from the total binary's size. For some cases, this leads to results of 0 bytes, which isn't real, but in practice leads to no additional size in the resulting executable.

# Default

**Optimization Level "0"**

![Parse Unstripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opt0.svg)
![Write Unstripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opt0.svg)
![Parse Stripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opt0.svg)
![Write Stripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opt0.svg)

**Optimization Level "1"**

![Parse Unstripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opt1.svg)
![Write Unstripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opt1.svg)
![Parse Stripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opt1.svg)
![Write Stripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opt1.svg)

**Optimization Level "2"**

![Parse Unstripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opt2.svg)
![Write Unstripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opt2.svg)
![Parse Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opt2.svg)
![Write Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opt2.svg)

**Optimization Level "3"**

![Parse Unstripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opt3.svg)
![Write Unstripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opt3.svg)
![Parse Stripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opt3.svg)
![Write Stripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opt3.svg)

**Optimization Level "s"**

![Parse Unstripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opts.svg)
![Write Unstripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opts.svg)
![Parse Stripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opts.svg)
![Write Stripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opts.svg)

**Optimization Level "z"**

![Parse Unstripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_optz.svg)
![Write Unstripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_optz.svg)
![Parse Stripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_optz.svg)
![Write Stripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_optz.svg)

# Compact

**Optimization Level "0"**

![Parse Unstripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opt0_features=compact.svg)
![Write Unstripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opt0_features=compact.svg)
![Parse Stripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opt0_features=compact.svg)
![Write Stripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opt0_features=compact.svg)

**Optimization Level "1"**

![Parse Unstripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opt1_features=compact.svg)
![Write Unstripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opt1_features=compact.svg)
![Parse Stripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opt1_features=compact.svg)
![Write Stripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opt1_features=compact.svg)

**Optimization Level "2"**

![Parse Unstripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opt2_features=compact.svg)
![Write Unstripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opt2_features=compact.svg)
![Parse Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opt2_features=compact.svg)
![Write Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opt2_features=compact.svg)

**Optimization Level "3"**

![Parse Unstripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opt3_features=compact.svg)
![Write Unstripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opt3_features=compact.svg)
![Parse Stripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opt3_features=compact.svg)
![Write Stripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opt3_features=compact.svg)

**Optimization Level "s"**

![Parse Unstripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_opts_features=compact.svg)
![Write Unstripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_opts_features=compact.svg)
![Parse Stripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_opts_features=compact.svg)
![Write Stripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_opts_features=compact.svg)

**Optimization Level "z"**

![Parse Unstripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_unstripped_optz_features=compact.svg)
![Write Unstripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_unstripped_optz_features=compact.svg)
![Parse Stripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/parse_stripped_optz_features=compact.svg)
![Write Stripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/write_stripped_optz_features=compact.svg)
