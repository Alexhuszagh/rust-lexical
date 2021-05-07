//! Cached tables for pre-calculated exponent powers.
//!
//! NOTE:
//! -----
//!
//! This is annotated here, since it's the largest collection of
//! pre-computed tables.
//!
//! In total, all the pre-computed tables take up the following amount
//! of space, based on the source files here:
//!     src/util/cached/float80_decimal.rs:             ~1 KB
//!     src/util/cached/float80_radix.rs:               ~29 KB
//!     src/util/cached/float160_decimal.rs:            ~7 KB
//!     src/util/cached/float160_radix.rs:              ~200 KB
//!     src/util/powers/large/decimal32.rs:             ~5 KB
//!     src/util/powers/large/radix32.rs:               ~50 KB
//!     src/util/powers/large/decimal64.rs:             ~4.8 KB
//!     src/util/powers/large/radix64.rs:               ~50 KB
//!     src/util/powers/small/binary32.rs:              ~296 B
//!     src/util/powers/small/decimal32.rs:             ~96 B
//!     src/util/powers/small/radix32.rs:               ~1 KB
//!     src/util/powers/small/binary64.rs:              ~1.3 KB
//!     src/util/powers/small/decimal64.rs:             ~384 B
//!     src/util/powers/small/radix64.rs:               ~3.7 KB
//!     src/util/float/decimal.rs:                      ~230 B
//!     src/util/float/radix.rs:                        ~6 KB
//!     src/util/digit/binary.rs:                       ~2.7 KB
//!     src/util/digit/decimal.rs:                      ~200 B
//!     src/util/digit/radix.rs:                        ~29 KB
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
//! in (which it currently is not). Storage requirements increase
//! dramatically with support for `f128`.

mod float;
pub(crate) use self::float::*;

cfg_if! {
if #[cfg(feature = "parse_floats")] {
    mod large;
    mod small;

    pub(crate) use self::large::*;
    pub(crate) use self::small::*;
}} // cfg_if
