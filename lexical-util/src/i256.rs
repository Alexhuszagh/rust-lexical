//! An signed 256-bit integer type.
//!
//! This aims to have feature parity with Rust's signed
//! integer types, such as [i32][core::i32]. The documentation
//! is based off of [i32][core::i32] for each method/member.

// TODO: Document
// TODO: Feature gate this...

// FIXME: Add support for [Saturating][core::num::Saturating] and
// [Wrapping][core::num::Wrapping] when we drop support for <1.74.0.

/// The 256-bit signed integer type.
///
/// This has the same binary representation as Apache Arrow's types,
/// and therefore can safely be transmuted from one to the other.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct i256 {
    pub(crate) lo: u128,
    pub(crate) hi: i128,
}

impl i256 {
    // TODO: Here
}
