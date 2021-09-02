# Binary Size

The binary size comparisons were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.12.5/Fedora 34, and run against commit [8406144](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/84061444c4dd42930fcd74c18e5066db1063536d). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`.

Each binary is generated using all optimization levels, and includes the result before and after stripping, with the core functionality black-boxed, to ensure optimization does not optimize-out the result.

All these binaries sizes are *relative* to the size of an empty Rust binary: that is, the size of the empty executable is subtracted from the total binary's size. For some cases, this leads to results of 0 bytes, which isn't real, but in practice leads to no additional size in the resulting executable.

# Default

**Optimization Level "0"**

![Parse Unstripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opt0_posix.svg)
![Write Unstripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opt0_posix.svg)
![Parse Stripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt0_posix.svg)
![Write Stripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt0_posix.svg)

**Optimization Level "1"**

![Parse Unstripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opt1_posix.svg)
![Write Unstripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opt1_posix.svg)
![Parse Stripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt1_posix.svg)
![Write Stripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt1_posix.svg)

**Optimization Level "2"**

![Parse Unstripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opt2_posix.svg)
![Write Unstripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opt2_posix.svg)
![Parse Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt2_posix.svg)
![Write Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt2_posix.svg)

**Optimization Level "3"**

![Parse Unstripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opt3_posix.svg)
![Write Unstripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opt3_posix.svg)
![Parse Stripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt3_posix.svg)
![Write Stripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt3_posix.svg)

**Optimization Level "s"**

![Parse Unstripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opts_posix.svg)
![Write Unstripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opts_posix.svg)
![Parse Stripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opts_posix.svg)
![Write Stripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opts_posix.svg)

**Optimization Level "z"**

![Parse Unstripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_optz_posix.svg)
![Write Unstripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_optz_posix.svg)
![Parse Stripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_optz_posix.svg)
![Write Stripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_optz_posix.svg)

# Compact

**Optimization Level "0"**

![Parse Unstripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opt0_features=compact_posix.svg)
![Write Unstripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opt0_features=compact_posix.svg)
![Parse Stripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt0_features=compact_posix.svg)
![Write Stripped - Optimization Level "0"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt0_features=compact_posix.svg)

**Optimization Level "1"**

![Parse Unstripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opt1_features=compact_posix.svg)
![Write Unstripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opt1_features=compact_posix.svg)
![Parse Stripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt1_features=compact_posix.svg)
![Write Stripped - Optimization Level "1"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt1_features=compact_posix.svg)

**Optimization Level "2"**

![Parse Unstripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opt2_features=compact_posix.svg)
![Write Unstripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opt2_features=compact_posix.svg)
![Parse Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt2_features=compact_posix.svg)
![Write Stripped - Optimization Level "2"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt2_features=compact_posix.svg)

**Optimization Level "3"**

![Parse Unstripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opt3_features=compact_posix.svg)
![Write Unstripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opt3_features=compact_posix.svg)
![Parse Stripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opt3_features=compact_posix.svg)
![Write Stripped - Optimization Level "3"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opt3_features=compact_posix.svg)

**Optimization Level "s"**

![Parse Unstripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_opts_features=compact_posix.svg)
![Write Unstripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_opts_features=compact_posix.svg)
![Parse Stripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_opts_features=compact_posix.svg)
![Write Stripped - Optimization Level "s"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_opts_features=compact_posix.svg)

**Optimization Level "z"**

![Parse Unstripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_unstripped_optz_features=compact_posix.svg)
![Write Unstripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_unstripped_optz_features=compact_posix.svg)
![Parse Stripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_parse_stripped_optz_features=compact_posix.svg)
![Write Stripped - Optimization Level "z"](https://raw.githubusercontent.com/Alexhuszagh/rust-lexical/main/assets/size_write_stripped_optz_features=compact_posix.svg)
