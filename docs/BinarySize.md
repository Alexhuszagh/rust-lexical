# Binary Size

The binary size comparisons were run on an `Intel(R) Core(TM) i7-6560U CPU @ 2.20GHz` processor, on Linux 5.12.5/Fedora 34, and run against commit [c858a0e](https://github.com/Alexhuszagh/rust-lexical-experimental/commit/c858a0ee9ed841a1d95f55eaf746f8c87e25f7bc). The Rust compiler version was `rustc 1.55.0-nightly (b41936b92 2021-07-20)`.

Each binary is generated using all optimization levels, and includes the result before and after stripping, with the core functionality black-boxed, to ensure optimization does not optimize-out the result.

All these binaries sizes are *relative* to the size of an empty Rust binary: that is, the size of the empty executable is subtracted from the total binary's size. For some cases, this leads to results of 0 bytes, which isn't real, but in practice leads to no additional size in the resulting executable.

# Default

**Optimization Level "0"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|437.4KB|116.0KB|67.0KB|28.0KB|
|parse-float-f64|437.5KB|116.0KB|67.0KB|28.0KB|
|parse-integer-i8|121.7KB|28.0KB|30.3KB|8.0KB|
|parse-integer-i16|121.9KB|28.0KB|30.4KB|8.0KB|
|parse-integer-i32|120.9KB|28.0KB|30.3KB|8.0KB|
|parse-integer-i64|121.4KB|28.0KB|30.4KB|8.0KB|
|parse-integer-i128|121.9KB|28.0KB|37.1KB|12.0KB|
|parse-integer-u8|120.4KB|28.0KB|30.3KB|8.0KB|
|parse-integer-u16|120.6KB|28.0KB|30.4KB|8.0KB|
|parse-integer-u32|119.0KB|28.0KB|30.3KB|8.0KB|
|parse-integer-u64|119.8KB|28.0KB|30.3KB|8.0KB|
|parse-integer-u128|120.6KB|28.0KB|34.8KB|12.0KB|
|write-float-f32|385.1KB|72.0KB|118.9KB|40.0KB|
|write-float-f64|407.4KB|92.0KB|118.8KB|40.0KB|
|write-integer-i8|187.7KB|20.0KB|92.8KB|16.0KB|
|write-integer-i16|187.7KB|20.0KB|92.8KB|16.0KB|
|write-integer-i32|187.6KB|20.0KB|92.7KB|16.0KB|
|write-integer-i64|188.2KB|20.0KB|93.0KB|16.0KB|
|write-integer-i128|199.0KB|28.0KB|97.0KB|20.0KB|
|write-integer-u8|186.7KB|20.0KB|92.6KB|16.0KB|
|write-integer-u16|186.8KB|20.0KB|92.8KB|16.0KB|
|write-integer-u32|186.7KB|20.0KB|92.2KB|16.0KB|
|write-integer-u64|187.2KB|20.0KB|92.7KB|16.0KB|
|write-integer-u128|194.1KB|24.0KB|97.0KB|20.0KB|

**Optimization Level "1"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|390.8KB|64.0KB|66.9KB|20.0KB|
|parse-float-f64|390.6KB|64.0KB|66.9KB|20.0KB|
|parse-integer-i8|60.2KB|4.0KB|35.7KB|4.0KB|
|parse-integer-i16|60.4KB|4.0KB|35.7KB|4.0KB|
|parse-integer-i32|59.5KB|4.0KB|35.8KB|4.0KB|
|parse-integer-i64|60.2KB|4.0KB|35.9KB|4.0KB|
|parse-integer-i128|73.2KB|8.0KB|39.0KB|4.0KB|
|parse-integer-u8|58.3KB|4.0KB|35.8KB|4.0KB|
|parse-integer-u16|58.4KB|4.0KB|35.8KB|4.0KB|
|parse-integer-u32|63.8KB|4.0KB|35.9KB|4.0KB|
|parse-integer-u64|65.5KB|4.0KB|35.9KB|4.0KB|
|parse-integer-u128|71.2KB|8.0KB|36.9KB|4.0KB|
|write-float-f32|290.0KB|24.0KB|119.4KB|36.0KB|
|write-float-f64|305.2KB|36.0KB|119.4KB|36.0KB|
|write-integer-i8|152.4KB|4.0KB|89.3KB|8.0KB|
|write-integer-i16|152.4KB|4.0KB|89.4KB|8.0KB|
|write-integer-i32|152.2KB|4.0KB|89.3KB|8.0KB|
|write-integer-i64|156.6KB|8.0KB|89.5KB|8.0KB|
|write-integer-i128|162.5KB|12.0KB|89.5KB|8.0KB|
|write-integer-u8|151.5KB|4.0KB|89.1KB|8.0KB|
|write-integer-u16|151.5KB|4.0KB|89.3KB|8.0KB|
|write-integer-u32|151.5KB|4.0KB|88.6KB|8.0KB|
|write-integer-u64|151.9KB|4.0KB|89.2KB|8.0KB|
|write-integer-u128|161.8KB|12.0KB|89.5KB|8.0KB|

**Optimization Level "2"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|30.6KB|32.0KB|25.0KB|20.0KB|
|parse-float-f64|31.2KB|32.0KB|24.4KB|20.0KB|
|parse-integer-i8|2.1KB|4.0KB|4.4KB|4.0KB|
|parse-integer-i16|2.9KB|4.0KB|5.1KB|4.0KB|
|parse-integer-i32|2.7KB|4.0KB|4.9KB|4.0KB|
|parse-integer-i64|2.2KB|4.0KB|4.4KB|4.0KB|
|parse-integer-i128|2.9KB|4.0KB|7.9KB|4.0KB|
|parse-integer-u8|2.2KB|4.0KB|4.2KB|4.0KB|
|parse-integer-u16|2.1KB|4.0KB|4.9KB|4.0KB|
|parse-integer-u32|2.2KB|4.0KB|4.9KB|4.0KB|
|parse-integer-u64|2.6KB|4.0KB|5.1KB|4.0KB|
|parse-integer-u128|2.9KB|4.0KB|5.1KB|4.0KB|
|write-float-f32|3.4KB|4.0KB|95.2KB|28.0KB|
|write-float-f64|15.6KB|16.0KB|95.7KB|28.0KB|
|write-integer-i8|4.2KB|4.0KB|6.4KB|4.0KB|
|write-integer-i16|3.5KB|4.0KB|5.8KB|4.0KB|
|write-integer-i32|4.0KB|4.0KB|5.1KB|4.0KB|
|write-integer-i64|3.4KB|4.0KB|5.6KB|4.0KB|
|write-integer-i128|3.5KB|4.0KB|11.0KB|8.0KB|
|write-integer-u8|3.5KB|4.0KB|6.0KB|4.0KB|
|write-integer-u16|4.0KB|4.0KB|5.8KB|4.0KB|
|write-integer-u32|4.2KB|4.0KB|5.1KB|4.0KB|
|write-integer-u64|4.2KB|4.0KB|4.9KB|4.0KB|
|write-integer-u128|3.4KB|4.0KB|10.8KB|8.0KB|

**Optimization Level "3"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|27.4KB|28.0KB|24.1KB|20.0KB|
|parse-float-f64|27.4KB|28.0KB|24.9KB|20.0KB|
|parse-integer-i8|2.4KB|4.0KB|936B|0B|
|parse-integer-i16|2.4KB|4.0KB|88B|0B|
|parse-integer-i32|2.2KB|4.0KB|936B|0B|
|parse-integer-i64|2.4KB|4.0KB|256B|0B|
|parse-integer-i128|3.1KB|4.0KB|2.9KB|0B|
|parse-integer-u8|2.4KB|4.0KB|936B|0B|
|parse-integer-u16|2.4KB|4.0KB|256B|0B|
|parse-integer-u32|2.4KB|4.0KB|256B|0B|
|parse-integer-u64|2.4KB|4.0KB|88B|0B|
|parse-integer-u128|3.1KB|4.0KB|256B|0B|
|write-float-f32|4.9KB|4.0KB|91.4KB|24.0KB|
|write-float-f64|12.2KB|12.0KB|91.2KB|24.0KB|
|write-integer-i8|24B|0B|5.9KB|4.0KB|
|write-integer-i16|192B|0B|6.6KB|4.0KB|
|write-integer-i32|872B|0B|5.0KB|4.0KB|
|write-integer-i64|704B|0B|5.9KB|4.0KB|
|write-integer-i128|24B|0B|7.7KB|4.0KB|
|write-integer-u8|704B|0B|5.5KB|4.0KB|
|write-integer-u16|192B|0B|6.6KB|4.0KB|
|write-integer-u32|192B|0B|5.9KB|4.0KB|
|write-integer-u64|24B|0B|5.2KB|4.0KB|
|write-integer-u128|192B|0B|7.2KB|4.0KB|

**Optimization Level "s"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|24.8KB|24.0KB|25.0KB|20.0KB|
|parse-float-f64|24.8KB|24.0KB|25.0KB|20.0KB|
|parse-integer-i8|4.5KB|4.0KB|504B|0B|
|parse-integer-i16|4.5KB|4.0KB|504B|0B|
|parse-integer-i32|4.5KB|4.0KB|504B|0B|
|parse-integer-i64|4.5KB|4.0KB|504B|0B|
|parse-integer-i128|4.5KB|4.0KB|7.4KB|4.0KB|
|parse-integer-u8|4.5KB|4.0KB|504B|0B|
|parse-integer-u16|4.5KB|4.0KB|504B|0B|
|parse-integer-u32|4.5KB|4.0KB|504B|0B|
|parse-integer-u64|4.5KB|4.0KB|504B|0B|
|parse-integer-u128|4.5KB|4.0KB|512B|0B|
|write-float-f32|4.1KB|4.0KB|91.8KB|24.0KB|
|write-float-f64|12.3KB|12.0KB|91.6KB|24.0KB|
|write-integer-i8|120B|0B|6.3KB|4.0KB|
|write-integer-i16|120B|0B|6.3KB|4.0KB|
|write-integer-i32|120B|0B|5.6KB|4.0KB|
|write-integer-i64|120B|0B|6.3KB|4.0KB|
|write-integer-i128|4.1KB|4.0KB|7.6KB|4.0KB|
|write-integer-u8|24B|0B|5.7KB|4.0KB|
|write-integer-u16|24B|0B|6.4KB|4.0KB|
|write-integer-u32|24B|0B|5.6KB|4.0KB|
|write-integer-u64|24B|0B|5.6KB|4.0KB|
|write-integer-u128|4.1KB|4.0KB|7.6KB|4.0KB|

**Optimization Level "z"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|22.5KB|20.0KB|21.0KB|16.0KB|
|parse-float-f64|22.5KB|20.0KB|21.0KB|16.0KB|
|parse-integer-i8|4.5KB|4.0KB|824B|0B|
|parse-integer-i16|4.5KB|4.0KB|832B|0B|
|parse-integer-i32|4.5KB|4.0KB|832B|0B|
|parse-integer-i64|4.5KB|4.0KB|832B|0B|
|parse-integer-i128|4.5KB|4.0KB|7.7KB|4.0KB|
|parse-integer-u8|4.5KB|4.0KB|728B|0B|
|parse-integer-u16|4.5KB|4.0KB|736B|0B|
|parse-integer-u32|4.5KB|4.0KB|736B|0B|
|parse-integer-u64|4.5KB|4.0KB|736B|0B|
|parse-integer-u128|4.5KB|4.0KB|736B|0B|
|write-float-f32|4.4KB|4.0KB|91.8KB|24.0KB|
|write-float-f64|16.4KB|16.0KB|91.6KB|24.0KB|
|write-integer-i8|272B|0B|6.3KB|4.0KB|
|write-integer-i16|272B|0B|6.3KB|4.0KB|
|write-integer-i32|272B|0B|5.6KB|4.0KB|
|write-integer-i64|272B|0B|6.3KB|4.0KB|
|write-integer-i128|4.4KB|4.0KB|7.6KB|4.0KB|
|write-integer-u8|24B|0B|5.7KB|4.0KB|
|write-integer-u16|24B|0B|6.4KB|4.0KB|
|write-integer-u32|24B|0B|5.6KB|4.0KB|
|write-integer-u64|24B|0B|5.6KB|4.0KB|
|write-integer-u128|224B|0B|7.6KB|4.0KB|

# Compact

**Optimization Level "0"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|451.0KB|100.0KB|67.0KB|28.0KB|
|parse-float-f64|451.2KB|100.0KB|67.0KB|28.0KB|
|parse-integer-i8|229.4KB|20.0KB|30.2KB|8.0KB|
|parse-integer-i16|229.4KB|20.0KB|30.4KB|8.0KB|
|parse-integer-i32|229.4KB|20.0KB|30.3KB|8.0KB|
|parse-integer-i64|229.4KB|20.0KB|30.4KB|8.0KB|
|parse-integer-i128|229.5KB|20.0KB|37.1KB|12.0KB|
|parse-integer-u8|225.0KB|16.0KB|30.3KB|8.0KB|
|parse-integer-u16|229.1KB|20.0KB|30.4KB|8.0KB|
|parse-integer-u32|224.9KB|16.0KB|30.3KB|8.0KB|
|parse-integer-u64|229.1KB|20.0KB|30.3KB|8.0KB|
|parse-integer-u128|229.1KB|20.0KB|34.8KB|12.0KB|
|write-float-f32|296.1KB|56.0KB|118.9KB|40.0KB|
|write-float-f64|296.0KB|56.0KB|118.8KB|40.0KB|
|write-integer-i8|118.1KB|12.0KB|92.8KB|16.0KB|
|write-integer-i16|118.1KB|12.0KB|92.8KB|16.0KB|
|write-integer-i32|118.0KB|12.0KB|92.7KB|16.0KB|
|write-integer-i64|118.1KB|12.0KB|92.9KB|16.0KB|
|write-integer-i128|122.2KB|16.0KB|97.0KB|20.0KB|
|write-integer-u8|117.4KB|12.0KB|92.6KB|16.0KB|
|write-integer-u16|117.5KB|12.0KB|92.8KB|16.0KB|
|write-integer-u32|117.4KB|12.0KB|92.2KB|16.0KB|
|write-integer-u64|117.5KB|12.0KB|92.7KB|16.0KB|
|write-integer-u128|117.5KB|12.0KB|97.0KB|20.0KB|

**Optimization Level "1"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|392.9KB|44.0KB|66.9KB|20.0KB|
|parse-float-f64|393.1KB|44.0KB|66.9KB|20.0KB|
|parse-integer-i8|150.0KB|4.0KB|35.7KB|4.0KB|
|parse-integer-i16|150.1KB|4.0KB|35.8KB|4.0KB|
|parse-integer-i32|149.9KB|4.0KB|35.8KB|4.0KB|
|parse-integer-i64|150.1KB|4.0KB|35.9KB|4.0KB|
|parse-integer-i128|154.1KB|8.0KB|39.0KB|4.0KB|
|parse-integer-u8|149.5KB|4.0KB|35.8KB|4.0KB|
|parse-integer-u16|149.6KB|4.0KB|35.9KB|4.0KB|
|parse-integer-u32|149.3KB|4.0KB|35.9KB|4.0KB|
|parse-integer-u64|149.5KB|4.0KB|35.9KB|4.0KB|
|parse-integer-u128|149.6KB|4.0KB|36.8KB|4.0KB|
|write-float-f32|248.3KB|32.0KB|119.4KB|36.0KB|
|write-float-f64|248.4KB|32.0KB|119.5KB|36.0KB|
|write-integer-i8|101.8KB|4.0KB|89.3KB|8.0KB|
|write-integer-i16|101.8KB|4.0KB|89.4KB|8.0KB|
|write-integer-i32|101.5KB|4.0KB|89.2KB|8.0KB|
|write-integer-i64|101.7KB|4.0KB|89.5KB|8.0KB|
|write-integer-i128|101.7KB|4.0KB|89.6KB|8.0KB|
|write-integer-u8|100.6KB|4.0KB|89.2KB|8.0KB|
|write-integer-u16|100.8KB|4.0KB|89.3KB|8.0KB|
|write-integer-u32|100.5KB|4.0KB|88.6KB|8.0KB|
|write-integer-u64|100.7KB|4.0KB|89.2KB|8.0KB|
|write-integer-u128|100.6KB|4.0KB|89.5KB|8.0KB|

**Optimization Level "2"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|15.4KB|16.0KB|24.2KB|20.0KB|
|parse-float-f64|15.6KB|16.0KB|24.2KB|20.0KB|
|parse-integer-i8|4.0KB|4.0KB|4.2KB|4.0KB|
|parse-integer-i16|3.5KB|4.0KB|3.4KB|4.0KB|
|parse-integer-i32|3.5KB|4.0KB|3.6KB|4.0KB|
|parse-integer-i64|3.3KB|4.0KB|3.6KB|4.0KB|
|parse-integer-i128|3.5KB|4.0KB|7.1KB|4.0KB|
|parse-integer-u8|3.3KB|4.0KB|3.6KB|4.0KB|
|parse-integer-u16|3.5KB|4.0KB|4.2KB|4.0KB|
|parse-integer-u32|4.1KB|4.0KB|4.2KB|4.0KB|
|parse-integer-u64|3.5KB|4.0KB|4.2KB|4.0KB|
|parse-integer-u128|4.1KB|4.0KB|3.4KB|4.0KB|
|write-float-f32|3.2KB|4.0KB|94.4KB|28.0KB|
|write-float-f64|3.4KB|4.0KB|94.0KB|28.0KB|
|write-integer-i8|24B|0B|5.6KB|4.0KB|
|write-integer-i16|-824B|0B|5.6KB|4.0KB|
|write-integer-i32|-656B|0B|4.5KB|4.0KB|
|write-integer-i64|-656B|0B|4.8KB|4.0KB|
|write-integer-i128|2.4KB|0B|10.2KB|8.0KB|
|write-integer-u8|-656B|0B|5.2KB|4.0KB|
|write-integer-u16|24B|0B|5.0KB|4.0KB|
|write-integer-u32|-656B|0B|4.7KB|4.0KB|
|write-integer-u64|-824B|0B|4.2KB|4.0KB|
|write-integer-u128|1.9KB|0B|10.8KB|8.0KB|

**Optimization Level "3"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|10.3KB|12.0KB|24.7KB|20.0KB|
|parse-float-f64|14.2KB|16.0KB|24.9KB|20.0KB|
|parse-integer-i8|2.2KB|4.0KB|64B|0B|
|parse-integer-i16|2.2KB|4.0KB|256B|0B|
|parse-integer-i32|2.9KB|4.0KB|88B|0B|
|parse-integer-i64|2.7KB|4.0KB|88B|0B|
|parse-integer-i128|2.2KB|4.0KB|2.9KB|0B|
|parse-integer-u8|2.2KB|4.0KB|248B|0B|
|parse-integer-u16|2.1KB|4.0KB|256B|0B|
|parse-integer-u32|2.1KB|4.0KB|936B|0B|
|parse-integer-u64|2.1KB|4.0KB|256B|0B|
|parse-integer-u128|2.9KB|4.0KB|936B|0B|
|write-float-f32|4.6KB|4.0KB|91.2KB|24.0KB|
|write-float-f64|3.9KB|4.0KB|91.7KB|24.0KB|
|write-integer-i8|24B|0B|5.9KB|4.0KB|
|write-integer-i16|704B|0B|6.6KB|4.0KB|
|write-integer-i32|24B|0B|5.7KB|4.0KB|
|write-integer-i64|536B|0B|5.9KB|4.0KB|
|write-integer-i128|3.2KB|0B|7.0KB|4.0KB|
|write-integer-u8|704B|0B|6.3KB|4.0KB|
|write-integer-u16|-144B|0B|6.6KB|4.0KB|
|write-integer-u32|-144B|0B|5.0KB|4.0KB|
|write-integer-u64|24B|0B|5.2KB|4.0KB|
|write-integer-u128|2.6KB|0B|7.2KB|4.0KB|

**Optimization Level "s"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|12.7KB|12.0KB|25.0KB|20.0KB|
|parse-float-f64|12.7KB|12.0KB|25.0KB|20.0KB|
|parse-integer-i8|4.5KB|4.0KB|504B|0B|
|parse-integer-i16|4.5KB|4.0KB|504B|0B|
|parse-integer-i32|4.5KB|4.0KB|504B|0B|
|parse-integer-i64|4.5KB|4.0KB|504B|0B|
|parse-integer-i128|4.5KB|4.0KB|7.4KB|4.0KB|
|parse-integer-u8|4.5KB|4.0KB|504B|0B|
|parse-integer-u16|4.5KB|4.0KB|504B|0B|
|parse-integer-u32|4.5KB|4.0KB|504B|0B|
|parse-integer-u64|4.5KB|4.0KB|504B|0B|
|parse-integer-u128|4.5KB|4.0KB|512B|0B|
|write-float-f32|4.1KB|4.0KB|91.8KB|24.0KB|
|write-float-f64|4.1KB|4.0KB|91.6KB|24.0KB|
|write-integer-i8|128B|0B|6.3KB|4.0KB|
|write-integer-i16|128B|0B|6.3KB|4.0KB|
|write-integer-i32|128B|0B|5.6KB|4.0KB|
|write-integer-i64|128B|0B|6.3KB|4.0KB|
|write-integer-i128|4.5KB|0B|7.6KB|4.0KB|
|write-integer-u8|24B|0B|5.7KB|4.0KB|
|write-integer-u16|24B|0B|6.4KB|4.0KB|
|write-integer-u32|24B|0B|5.6KB|4.0KB|
|write-integer-u64|24B|0B|5.6KB|4.0KB|
|write-integer-u128|2.6KB|0B|7.6KB|4.0KB|

**Optimization Level "z"**

|Function|Size Lexical|Size Lexical (stripped)|Size Core|Size Core (stripped)|
|:-:|:-:|:-:|:-:|:-:|
|parse-float-f32|14.0KB|12.0KB|21.0KB|16.0KB|
|parse-float-f64|14.0KB|12.0KB|21.0KB|16.0KB|
|parse-integer-i8|4.5KB|4.0KB|824B|0B|
|parse-integer-i16|4.5KB|4.0KB|832B|0B|
|parse-integer-i32|4.5KB|4.0KB|832B|0B|
|parse-integer-i64|4.5KB|4.0KB|832B|0B|
|parse-integer-i128|4.5KB|4.0KB|7.7KB|4.0KB|
|parse-integer-u8|4.5KB|4.0KB|728B|0B|
|parse-integer-u16|4.5KB|4.0KB|736B|0B|
|parse-integer-u32|4.5KB|4.0KB|736B|0B|
|parse-integer-u64|4.5KB|4.0KB|736B|0B|
|parse-integer-u128|4.5KB|4.0KB|736B|0B|
|write-float-f32|4.2KB|4.0KB|91.8KB|24.0KB|
|write-float-f64|4.2KB|4.0KB|91.6KB|24.0KB|
|write-integer-i8|128B|0B|6.3KB|4.0KB|
|write-integer-i16|128B|0B|6.3KB|4.0KB|
|write-integer-i32|128B|0B|5.6KB|4.0KB|
|write-integer-i64|128B|0B|6.3KB|4.0KB|
|write-integer-i128|4.5KB|0B|7.6KB|4.0KB|
|write-integer-u8|24B|0B|5.7KB|4.0KB|
|write-integer-u16|24B|0B|6.4KB|4.0KB|
|write-integer-u32|24B|0B|5.6KB|4.0KB|
|write-integer-u64|24B|0B|5.6KB|4.0KB|
|write-integer-u128|2.6KB|0B|7.6KB|4.0KB|
