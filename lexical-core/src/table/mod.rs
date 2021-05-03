//! Cached tables for precalculated values.
//!
//! NOTE:
//! -----
//! In total, all the pre-computed tables take up the following amount
//! of space, based on the source files here:
//!     src/atof/algorithm/cached/float80_decimal.rs:   ~1 KB
//!     src/atof/algorithm/cached/float80_radix.rs:     ~29 KB
//!     src/atof/algorithm/cached/float160_decimal.rs:  ~1.6 KB
//!     src/atof/algorithm/cached/float160_radix.rs:    ~50 KB
//!     src/atof/algorithm/powers/large32_decimal.rs:   ~5 KB
//!     src/atof/algorithm/powers/large32_radix.rs:     ~50 KB
//!     src/atof/algorithm/powers/large64_decimal.rs:   ~4.8 KB
//!     src/atof/algorithm/powers/large64_radix.rs:     ~50 KB
//!     src/atof/algorithm/powers/small32_binary.rs:    ~296 B
//!     src/atof/algorithm/powers/small32_decimal.rs:   ~96 B
//!     src/atof/algorithm/powers/small32_radix.rs:     ~1 KB
//!     src/atof/algorithm/powers/small64_binary.rs:    ~1.3 KB
//!     src/atof/algorithm/powers/small64_decimal.rs:   ~384 B
//!     src/atof/algorithm/powers/small64_radix.rs:     ~3.7 KB
//!     src/table/binary.rs:                            ~2.7 KB
//!     src/table/decimal.rs:                           ~430 B
//!     src/table/radix.rs:                             ~35 KB
//!
//! Therefore, the total storage with the radix feature is ~127 KB,
//! while the total storage with the binary feature is ~11 KB,
//! while the total storage without the radix feature is ~6 KB.
//! There's no real way around this extra storage, since in order
//! to do fast, accurate computations with arbitrary-precision
//! arithmetic, we need pre-computed arrays, which is very expensive.
//! In the grand scheme of things, 127 KB is fairly small.
//!
//! Note: these figures assume that 32-bit and 64-bit powers
//! are mutually independent, and cached/float160 is not being compiled
//! in (which it currently is not).

// Hide modules.
mod decimal;
mod pow;

// Re-export all tables and traits.
pub use self::decimal::*;
pub use self::pow::*;

cfg_if! {
if #[cfg(feature = "power_of_two")] {
    mod binary;
    pub(crate) use self::binary::*;
}} // cfg_if

cfg_if! {
if #[cfg(feature = "radix")] {
    mod radix;
    pub(crate) use self::radix::*;
}} // cfg_if
