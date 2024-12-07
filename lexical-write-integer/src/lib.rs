//! Fast lexical integer-to-string conversion routines.
//!
//! The default implementations use power reduction to unroll
//! 4 loops at a time to minimize the number of required divisions,
//! leading to massive performance gains. In addition, decimal
//! strings pre-calculate the number of digits, avoiding temporary buffers.
//!
//! A compact, fallback algorithm uses a naive, simple algorithm,
//! where each loop generates a single digit. This comes at a performance
//! penalty, but produces smaller binaries.
//!
//! # Features
//!
//! * `std` - Use the standard library.
//! * `power-of-two` - Add support for writing power-of-two integer strings.
//! * `radix` - Add support for strings of any radix.
//! * `compact` - Reduce code size at the cost of performance.
//! * `safe` - Ensure only memory-safe indexing is used.
//!
//! # Note
//!
//! Only documented functionality is considered part of the public API:
//! any of the modules, internal functions, or structs may change
//! release-to-release without major or minor version changes. Use
//! internal implementation details at your own risk.
//!
//! lexical-write-integer mainly exists as an implementation detail for
//! lexical-core, although its API is stable. If you would like to use
//! a high-level API that writes to and parses from `String` and `&str`,
//! respectively, please look at [lexical](https://crates.io/crates/lexical)
//! instead. If you would like an API that supports multiple numeric
//! conversions, please look at [lexical-core](https://crates.io/crates/lexical-core)
//! instead.
//!
//! # Version Support
//!
//! The minimum, standard, required version is 1.63.0, for const generic
//! support. Older versions of lexical support older Rust versions.
//!
//! # Design
//!
//! - [Algorithm Approach](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Algorithm.md)
//! - [Benchmarks](https://github.com/Alexhuszagh/rust-lexical/blob/main/lexical-write-integer/docs/Benchmarks.md)
//! - [Comprehensive Benchmarks](https://github.com/Alexhuszagh/lexical-benchmarks)
//!
//! # Safety
//!
//! This module uses a some more unsafe code for moderately acceptable
//! performance. The compact decimal serializer has no non-local safety
//! invariants, which since it's focused on code size rather than performance,
//! this trade-off is acceptable and it uses a temporary, over-allocated buffer
//! as an intermediate.
//!
//! The decimal writer relies on pre-computed tables and an exact calculation
//! of the digit count ([`digit_count`]) to avoid any overhead. Avoid
//! intermediary copies is **CRITICAL** for fast performance so the entire
//! buffer must be known but assigned to use algorithms the compiler cannot
//! easily verify. This is because we use multi-digit optimizations with our
//! pre-computed tables, so we cannot just iterate over the slice and assign
//! iteratively. Using checked indexing can lead to 30%+ decreases in
//! performance. However, with careful analysis and factoring of the code, it's
//! fairly easy to demonstrate the safety as long as the caller ensures at least
//! the required number of digits are provided.
//!
//! Our algorithms work like this, carving off the lower digits and writing them
//! to the back of the buffer.
//!
//! ```rust,ignore
//! let mut value = 12345u32;
//! let buffer = [0u8; 32];
//! let digits = value.digit_count();
//! let bytes = buffer[..digits];
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
//! // oontinue with radix^2 and then a single digit.
//! ```
//!
//! We can efficiently determine at compile time if the pre-computed
//! tables are large enough so there are no non-local safety considerations
//! there. The current logic call stack is:
//! 1. [`to_lexical`]
//! 2. [decimal][dec], compact, or radix (gets the correct tables and calls
//!    algorithm)
//! 3. [algorithm]
//!
//! [decimal][dec], compact, and radix therefore **MUST** be safe and do type
//! check of the bounds to avoid too much exposure to unsafety. Only
//! [`algorithm`] should have any unsafety associated with it. That is, as long
//! as the direct caller has ensure the proper buffer is allocated, there are
//! non-local safety invariants.
//!
//! [`digit_count`]: crate::decimal::DigitCount
//! [`to_lexical`]: crate::ToLexical::to_lexical
//! [dec]: crate::decimal::Decimal::decimal
//! [`algorithm`]: crate::algorithm::algorithm

// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    clippy::doc_markdown,
    clippy::unnecessary_safety_comment,
    clippy::semicolon_if_nothing_returned,
    clippy::unwrap_used,
    clippy::as_underscore,
    clippy::doc_markdown
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
pub use lexical_util::format::{self, NumberFormatBuilder};
pub use lexical_util::options::WriteOptions;

pub use self::api::{ToLexical, ToLexicalWithOptions};
#[doc(inline)]
pub use self::options::{Options, OptionsBuilder};
