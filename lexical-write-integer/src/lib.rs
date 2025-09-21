//! Fast lexical integer-to-string conversion routines.
//!
//! This contains high-performance methods to write integers
//! directly to bytes, can be converted to [`str`] using
//! [`str::from_utf8`]. Using [`to_lexical`] is analogous to [`to_string`],
//! just writing to an existing buffer.
//!
//! [`str::from_utf8`]: core::str::from_utf8
//! [`to_lexical`]: ToLexical::to_lexical
//!
//! # Getting Started
//!
//! To write a number to bytes, use [`to_lexical`]:
//!
//! ```rust
//! # #[no_std]
//! # use core::str;
//! use lexical_write_integer::{FormattedSize, ToLexical};
//!
//! let mut buffer = [0u8; u64::FORMATTED_SIZE_DECIMAL];
//! let digits = 1234u64.to_lexical(&mut buffer);
//! assert_eq!(str::from_utf8(digits), Ok("1234"));
//! ```
//!
//! Using [`FormattedSize::FORMATTED_SIZE_DECIMAL`] guarantees the buffer
//! will be large enough to write the digits for all numbers of that
//! type.
//!
//! # Features
//!
//! * `format` - Add support for custom integer formatting (currently
//!   unsupported).
//! * `power-of-two` - Add support for writing power-of-two integer strings.
//! * `radix` - Add support for strings of any radix.
//! * `compact` - Reduce code size at the cost of performance.
//! * `std` (Default) - Disable to allow use in a [`no_std`] environment.
//!
//! [`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
//!
//! A complete description of supported features includes:
//!
//! #### format
//!
//! Add support for custom integer formatting. Currently no custom styles are
//! supported but this could include digit [`separator`] support in the future.
//!
//! [`separator`]: NumberFormatBuilder::digit_separator
//!
//! <!--
//! For a list of all supported fields, see [Write Integer
//! Fields][NumberFormatBuilder#write-integer-fields].
//! -->
//!
//! #### power-of-two
//!
//! Enable writing numbers using radixes that are powers of two, that is, `2`,
//! `4`, `8`, `16`, and `32`. In these cases, you should use [`FORMATTED_SIZE`]
//! to create a sufficiently large buffer.
//!
//! [`FORMATTED_SIZE`]: FormattedSize::FORMATTED_SIZE
//!
//! ```rust
//! # #[cfg(feature = "power-of-two")] {
//! # use core::str;
//! use lexical_write_integer::{FormattedSize, NumberFormatBuilder, Options, ToLexicalWithOptions};
//!
//! let mut buffer = [0u8; u64::FORMATTED_SIZE];
//! const BINARY: u128 = NumberFormatBuilder::binary();
//! const OPTIONS: Options = Options::new();
//! let digits = 1234u64.to_lexical_with_options::<BINARY>(&mut buffer, &OPTIONS);
//! assert_eq!(str::from_utf8(digits), Ok("10011010010"));
//! # }
//! ```
//!
//! #### radix
//!
//! Enable writing numbers using all radixes from `2` to `36`. This requires
//! more static storage than [`power-of-two`][crate#power-of-two], and increases
//! compile times, but can be quite useful for esoteric programming languages
//! which use duodecimal integers, for example.
//!
//! ```rust
//! # #[cfg(feature = "radix")] {
//! # use core::str;
//! use lexical_write_integer::{FormattedSize, NumberFormatBuilder, Options, ToLexicalWithOptions};
//!
//! let mut buffer = [0u8; u64::FORMATTED_SIZE];
//! const FORMAT: u128 = NumberFormatBuilder::from_radix(12);
//! const OPTIONS: Options = Options::new();
//! let digits = 1234u64.to_lexical_with_options::<FORMAT>(&mut buffer, &OPTIONS);
//! assert_eq!(str::from_utf8(digits), Ok("86A"));
//! # }
//! ```
//!
//! #### compact
//!
//! Reduce the generated code size at the cost of performance. This minimizes
//! the number of static tables, inlining, and generics used, drastically
//! reducing the size of the generated binaries.
//!
//! #### std
//!
//! Enable use of the standard library. Currently, the standard library
//! is not used, and may be disabled without any change in functionality
//! on stable.
//!
//! # Higher-Level APIs
//!
//! If you would like support for writing to [`String`] directly, use
//! [`lexical`] instead. If you would like an API that supports multiple numeric
//! conversions rather than just writing integers, use [`lexical-core`] instead.
//!
//! [`lexical`]: https://crates.io/crates/lexical
//! [`lexical-core`]: https://crates.io/crates/lexical-core
//!
//! # Version Support
//!
//! The minimum, standard, required version is [`1.63.0`][`rust-1.63.0`], for
//! const generic support. Older versions of lexical support older Rust
//! versions.
//!
//! # Algorithm
//!
//! We use 3 algorithms for serializing numbers:
//! 1. [`Jeaiii Algorithm`] (decimal only)
//! 2. Power reduction to write 4 digits at a time (radix only)
//! 3. Compact, single-digit serialization
//!
//! ## Decimal
//!
//! Our decimal-based digit writers are based on the [`Jeaiii Algorithm`], which
//! branches based on the number of digits and writes digits to minimize the
//! number of additions and multiplications. This avoids the need to calculate
//! the number of digits ahead of time, by just branching on the value.
//!
//! James Anhalt's itoa algorithm along with Junekey Jeon's performance tweaks
//! have excellent performance, however, this can be further optimized. Both
//! James Anhalt's and Junekey Jeon's use a binary search for determining the
//! correct number of digits to print (for 32-bit integers).
//!
//! ```text
//!      /\____________
//!     /  \______     \______
//!    /\   \     \     \     \
//!   0  1  /\    /\    /\    /\
//!        2  3  4  5  6  7  8  9
//! ```
//!
//! This leads to a max tree depth of 4, and the major performance bottleneck
//! with larger type sizes is the branching. A minor modification can optimize
//! this, leadingg to a max tree depth of 3 while only required 1 extra
//! comparison at the top level. Also, we invert the comparisons: oddly enough,
//! our benchmarks show doing larger comparisons then smaller improves
//! performance for numbers with both large and small numbers of digits.
//!
//! ```text
//!           ____________________
//!       ___/_       __|__       \
//!      /  |  \     /     \      /\
//!     /\  1   0   /\     /\    8  9
//!    3  2        6  7   4  5
//! ```
//!
//! For larger integers, we can apply the a similar algorithm with minor
//! modifications to minimize branching while keeping excellent performance.
//!
//! ## Radix
//!
//! Our radix-based algorithms work like this, carving off the lower digits and
//! writing them to the back of the buffer.
//!
//! ```rust,ignore
//! let mut value = 12345u32;
//! let buffer = [0u8; 32];
//! let digits = value.digit_count();
//! let bytes = buffer[..digits];
//!
//! let table = ...;  // some pre-computed table of 2 * radix^2 length
//!
//! let radix = 10;
//! let radix2 = radix * radix;
//! let radix4 = radix2 * radix2
//! let mut index = bytes.len();
//! while value >= 10000 {
//!     let r = value % radix4;
//!     value /= radix4;
//!     let r1 = 2 * (r / radix2) as usize;
//!     let r2 = 2 * (r % radix2) as usize;
//!
//!     // write 5, then 4
//!     index -= 1;
//!     bytes[index] = table[r2 + 1];
//!     index -= 1;
//!     bytes[index] = table[r2];
//!
//!     // write 3 then 2
//!     index -= 1;
//!     bytes[index] = table[r1 + 1];
//!     index -= 1;
//!     bytes[index] = table[r1];
//! }
//!
//! // continue with radix^2 and then a single digit.
//! ```
//!
//! We can efficiently determine at compile time if the pre-computed
//! tables are large enough so there are no non-local safety considerations
//! there. The current logic call stack is:
//! 1. [`to_lexical`]
//! 2. [`decimal`][`dec`], [`compact`][`cmp`], or [`radix`][`rdx`] (gets the
//!    correct tables and calls algorithm)
//! 3. [`jeaiii`]
//!
//! # Compact
//!
//! A compact, fallback algorithm uses a naive, simple algorithm,
//! where each loop generates a single digit. This comes at a performance
//! penalty, but produces smaller binaries. It is analogous to the below
//! code.
//!
//! ```rust,ignore
//! const fn digit_to_char(digit: u32) -> u8 {
//!     match r {
//!        b'0'..=b'9' => c - b'0',
//!        b'A'..=b'Z' => c - b'A' + 10,
//!        b'a'..=b'z' => c - b'a' + 10,
//!        _ => 0xFF,  // unreachable
//!     }
//! }
//!
//! let mut value = 12345u32;
//! let buffer = [0u8; 32];
//! let digits = value.digit_count();
//! let bytes = buffer[..digits];
//!
//! let radix = 10;
//! let mut index = bytes.len();
//! while value >= radix {
//!     let r = value % radix;
//!     value /= radix;
//!     index -= 1;
//!     bytes[index] = digit_to_char(r);
//! }
//!
//! index -= 1;
//! bytes[index] = digit_to_char(value);
//! ```
//!
//! # Design
//!
//! - [Algorithm Approach](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Algorithm.md)
//! - [Benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Benchmarks.md)
//! - [Comprehensive Benchmarks](https://github.com/Alexhuszagh/lexical-benchmarks)
//!
//! # Safety Guarantees
//!
//! <div class="warning info-warning">
//! <style>
//! .info-warning::before {
//!   color: #87CEFAb0 !important;
//! }
//! .info-warning {
//!   border-left: 2px solid #87CEFAb0 !important;
//! }
//! </style>
//!
//! This module uses some unsafe code to achieve accept acceptable performance.
//! Providing a buffer of insufficient size will cause the code to panic and
//! cannot lead to out-of-bounds access. The safety guarantees and logic are
//! described below.
//!
//! </div>
//!
//! ### Decimal
//!
//! Our decimal writer uses a branched algorithm and therefore the indexing for
//! each element in the buffer is known ahead of time. The digit
//! [`generation`][`digit-gen`] is [well-established][`Jeaiii Algorithm`] to
//! ensure the the lookup value is less than the size of the pre-computed table
//! (`2 * 10^2`, or 200), and as long as this invariant holds true, then no
//! undefined behavior can occur.
//!
//! [`digit-gen`]: https://github.com/jk-jeon/idiv/blob/main/subproject/example/jeaiii_analysis.cpp
//!
//! ### Radix
//!
//! The non-decimal writers rely on pre-computed tables and an exact calculation
//! of the digit count ([`digit_count`]) to avoid any overhead. Avoiding
//! intermediary copies is **CRITICAL** for fast performance so the entire
//! buffer must be known but assigned to use algorithms the compiler cannot
//! easily verify. This is because we use multi-digit optimizations with our
//! pre-computed tables, so we cannot just iterate over the slice and assign
//! iteratively. Using checked indexing for the pre-compuited table can lead to
//! 30%+ decreases in performance. However, with careful analysis and factoring
//! of the code, it's trivial to demonstrate both the table lookups and buffer
//! indexing are safe.
//!
//! For radixes that are 2^N, we use the `ceil(log(value | 1, radix))` which can
//! always be calculated through the number of leading [`zeros`][`log2_lz`]. For
//! other radixes, we calculate the number of digits exactly the same way as if
//! we were writing digits in an initial pass.
//!
//! ### Compact
//!
//! The compact decimal writer uses no unsafe indexing.
//!
//! [`digit_count`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-write-integer/src/digit_count.rs#L180
//! [`to_lexical`]: crate::ToLexical::to_lexical
//! [`dec`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-write-integer/src/decimal.rs#L278
//! [`jeaiii`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-write-integer/src/jeaiii.rs
//! [`cmp`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-write-integer/src/compact.rs
//! [`rdx`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-write-integer/src/radix.rs
//! [`Jeaiii Algorithm`]: https://jk-jeon.github.io/posts/2022/02/jeaiii-algorithm/
//! [`rust-1.63.0`]: https://blog.rust-lang.org/2022/08/11/Rust-1.63.0.html
//! [`String`]: https://doc.rust-lang.org/alloc/string/struct.String.html
//! [`to_string`]: https://doc.rust-lang.org/alloc/string/trait.ToString.html#tymethod.to_string
//! [`log2_lz`]: https://github.com/Alexhuszagh/rust-lexical/blob/c6c5052/lexical-write-integer/src/digit_count.rs#L119

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(
    clippy::doc_markdown,
    clippy::unnecessary_safety_comment,
    clippy::semicolon_if_nothing_returned,
    clippy::unwrap_used,
    clippy::as_underscore
)]
#![allow(
    // used when concepts are logically separate
    clippy::match_same_arms,
    // loss of precision is intentional
    clippy::integer_division,
    // mathematical names use 1-character identifiers
    clippy::min_ident_chars,
    // these are not cryptographically secure contexts
    clippy::integer_division_remainder_used,
    // this can be intentional
    clippy::module_name_repetitions,
    // this is intentional: already passing a pointer and need performance
    clippy::needless_pass_by_value,
    // we use this for inline formatting for unsafe blocks
    clippy::semicolon_inside_block,
)]

pub mod algorithm;
pub mod compact;
pub mod decimal;
pub mod digit_count;
pub mod jeaiii;
pub mod options;
pub mod radix;
pub mod table;
pub mod write;

mod api;
mod table_binary;
mod table_decimal;
mod table_radix;

// Re-exports
pub use lexical_util::constants::{FormattedSize, BUFFER_SIZE};
pub use lexical_util::error::Error;
pub use lexical_util::format::{self, NumberFormat, NumberFormatBuilder};
pub use lexical_util::options::WriteOptions;
pub use lexical_util::result::Result;

pub use self::api::{ToLexical, ToLexicalWithOptions};
#[doc(inline)]
pub use self::options::{Options, OptionsBuilder};
