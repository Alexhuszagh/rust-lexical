# Binary Size

The binary size comparisons were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.12.5/Fedora 34, and run against commit [c858a0e](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/c858a0ee9ed841a1d95f55eaf746f8c87e25f7bc). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`.

Each binary is generated using all optimization levels, and includes the result before and after stripping, with the core functionality black-boxed, to ensure optimization does not optimize-out the result.

All these binaries sizes are *relative* to the size of an empty Rust binary: that is, the size of the empty executable is subtracted from the total binary's size. For some cases, this leads to results of 0 bytes, which isn't real, but in practice leads to no additional size in the resulting executable.

# Default

**Optimization Level "0"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-integer-i8|161.0KB|36.0KB|30.2KB|8.0KB|
|parse-integer-i16|161.1KB|36.0KB|30.3KB|8.0KB|
|parse-integer-i32|160.8KB|36.0KB|30.3KB|8.0KB|
|parse-integer-i64|158.2KB|36.0KB|30.4KB|8.0KB|
|parse-integer-i128|167.6KB|40.0KB|37.1KB|12.0KB|
|parse-integer-u8|160.9KB|36.0KB|30.3KB|8.0KB|
|parse-integer-u16|161.1KB|36.0KB|30.4KB|8.0KB|
|parse-integer-u32|160.2KB|36.0KB|30.3KB|8.0KB|
|parse-integer-u64|158.0KB|36.0KB|30.3KB|8.0KB|
|parse-integer-u128|165.1KB|40.0KB|34.7KB|12.0KB|
|write-integer-i8|190.5KB|20.0KB|92.8KB|16.0KB|
|write-integer-i16|190.5KB|20.0KB|92.8KB|16.0KB|
|write-integer-i32|190.3KB|20.0KB|92.7KB|16.0KB|
|write-integer-i64|190.9KB|20.0KB|92.9KB|16.0KB|
|write-integer-i128|202.8KB|28.0KB|97.0KB|20.0KB|
|write-integer-u8|189.5KB|20.0KB|92.6KB|16.0KB|
|write-integer-u16|189.5KB|20.0KB|92.8KB|16.0KB|
|write-integer-u32|189.4KB|20.0KB|92.2KB|16.0KB|
|write-integer-u64|190.0KB|20.0KB|92.7KB|16.0KB|
|write-integer-u128|201.9KB|28.0KB|97.0KB|20.0KB

**Optimization Level "1"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-integer-i8|52.6KB|4.0KB|35.7KB|4.0KB|
|parse-integer-i16|52.5KB|4.0KB|35.7KB|4.0KB|
|parse-integer-i32|59.9KB|4.0KB|35.8KB|4.0KB|
|parse-integer-i64|61.3KB|4.0KB|35.9KB|4.0KB|
|parse-integer-i128|72.4KB|8.0KB|39.0KB|4.0KB|
|parse-integer-u8|51.6KB|4.0KB|35.8KB|4.0KB|
|parse-integer-u16|51.7KB|4.0KB|35.9KB|4.0KB|
|parse-integer-u32|58.4KB|4.0KB|35.8KB|4.0KB|
|parse-integer-u64|59.5KB|4.0KB|35.9KB|4.0KB|
|parse-integer-u128|63.0KB|4.0KB|36.9KB|4.0KB|
|write-integer-i8|154.5KB|4.0KB|89.4KB|8.0KB|
|write-integer-i16|154.4KB|4.0KB|89.3KB|8.0KB|
|write-integer-i32|154.2KB|4.0KB|89.2KB|8.0KB|
|write-integer-i64|158.7KB|8.0KB|89.4KB|8.0KB|
|write-integer-i128|165.5KB|12.0KB|89.5KB|8.0KB|
|write-integer-u8|153.5KB|4.0KB|89.1KB|8.0KB|
|write-integer-u16|153.6KB|4.0KB|89.3KB|8.0KB|
|write-integer-u32|153.5KB|4.0KB|88.6KB|8.0KB|
|write-integer-u64|154.0KB|4.0KB|89.1KB|8.0KB|
|write-integer-u128|164.7KB|12.0KB|89.5KB|8.0KB|

# Compact
